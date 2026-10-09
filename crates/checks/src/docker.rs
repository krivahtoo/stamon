use std::time::Instant;

use serde::{Deserialize, Serialize};
use tokio::{
    io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt},
    net::TcpStream,
};

use crate::CheckOutcome;

const DEFAULT_HOST: &str = "unix:///var/run/docker.sock";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DockerConfig {
    /// Container name or id.
    pub container: String,
    /// `unix:///path/to/docker.sock` or `tcp://host:port`. Defaults to the
    /// local socket. TLS-protected daemons aren't supported.
    #[serde(default)]
    pub docker_host: Option<String>,
}

enum Endpoint<'a> {
    Unix(&'a str),
    Tcp(&'a str),
}

fn endpoint(host: &str) -> Result<Endpoint<'_>, String> {
    if let Some(path) = host.strip_prefix("unix://") {
        Ok(Endpoint::Unix(path))
    } else if let Some(addr) = host.strip_prefix("tcp://") {
        Ok(Endpoint::Tcp(addr.trim_end_matches('/')))
    } else {
        Err("docker_host must start with unix:// or tcp://".into())
    }
}

impl DockerConfig {
    pub fn validate(&self) -> Result<(), String> {
        // Names and ids only use these, and they go into the request path.
        let valid = !self.container.is_empty()
            && self
                .container
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '.' | '-'));
        if !valid {
            return Err("container must be a container name or id".into());
        }
        endpoint(self.host())?;
        Ok(())
    }

    fn host(&self) -> &str {
        self.docker_host
            .as_deref()
            .map(str::trim)
            .filter(|h| !h.is_empty())
            .unwrap_or(DEFAULT_HOST)
    }
}

#[derive(Deserialize)]
struct Inspect {
    #[serde(rename = "State")]
    state: State,
}

#[derive(Deserialize)]
struct State {
    #[serde(rename = "Status")]
    status: String,
    #[serde(rename = "Running")]
    running: bool,
    #[serde(rename = "Health")]
    health: Option<Health>,
}

#[derive(Deserialize)]
struct Health {
    #[serde(rename = "Status")]
    status: String,
}

/// Send one HTTP/1.0 request and return the status code and body. HTTP/1.0
/// keeps the response unchunked and closes the connection after it.
async fn request<S: AsyncRead + AsyncWrite + Unpin>(
    mut stream: S,
    path: &str,
) -> std::io::Result<(u16, Vec<u8>)> {
    stream
        .write_all(format!("GET {path} HTTP/1.0\r\nHost: docker\r\n\r\n").as_bytes())
        .await?;
    let mut response = Vec::new();
    stream.read_to_end(&mut response).await?;

    let invalid = || std::io::Error::new(std::io::ErrorKind::InvalidData, "invalid HTTP response");
    let split = response
        .windows(4)
        .position(|w| w == b"\r\n\r\n")
        .ok_or_else(invalid)?;
    let head = std::str::from_utf8(&response[..split]).map_err(|_| invalid())?;
    let status = head
        .split_whitespace()
        .nth(1)
        .and_then(|code| code.parse().ok())
        .ok_or_else(invalid)?;
    Ok((status, response[split + 4..].to_vec()))
}

pub(crate) async fn check(config: &DockerConfig) -> CheckOutcome {
    let start = Instant::now();
    let host = config.host();
    let path = format!("/containers/{}/json", config.container);

    let response = match endpoint(host) {
        Ok(Endpoint::Unix(socket)) => match tokio::net::UnixStream::connect(socket).await {
            Ok(stream) => request(stream, &path).await,
            Err(e) => Err(e),
        },
        Ok(Endpoint::Tcp(addr)) => match TcpStream::connect(addr).await {
            Ok(stream) => request(stream, &path).await,
            Err(e) => Err(e),
        },
        Err(e) => return CheckOutcome::error(e),
    };
    // Without the daemon nothing is known about the container.
    let (status, body) = match response {
        Ok(response) => response,
        Err(e) => return CheckOutcome::error(format!("Can't reach Docker at {host}: {e}")),
    };

    match status {
        200 => (),
        404 => {
            return CheckOutcome::down(
                start.elapsed(),
                format!("No such container: {}", config.container),
            );
        }
        status => {
            let body = String::from_utf8_lossy(&body);
            return CheckOutcome::error(format!("Docker returned {status}: {}", body.trim()));
        }
    }
    let inspect: Inspect = match serde_json::from_slice(&body) {
        Ok(inspect) => inspect,
        Err(e) => return CheckOutcome::error(format!("Unexpected response from Docker: {e}")),
    };

    let state = inspect.state;
    if !state.running {
        return CheckOutcome::down(start.elapsed(), format!("Container is {}", state.status));
    }
    // A container still starting up has no verdict yet, so only "unhealthy" counts.
    if let Some(health) = state.health
        && health.status == "unhealthy"
    {
        return CheckOutcome::down(start.elapsed(), "Container is unhealthy");
    }
    CheckOutcome::up(start.elapsed())
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use tokio::{
        io::AsyncBufReadExt,
        net::{TcpListener, UnixListener},
    };

    use super::*;
    use crate::CheckStatus;

    /// The response for a request path, like a tiny Docker daemon.
    fn respond(path: &str) -> String {
        let (status, body) = match path {
            "/containers/web/json" => {
                ("200 OK", r#"{"State":{"Status":"running","Running":true}}"#)
            }
            "/containers/sick/json" => (
                "200 OK",
                r#"{"State":{"Status":"running","Running":true,"Health":{"Status":"unhealthy"}}}"#,
            ),
            "/containers/booting/json" => (
                "200 OK",
                r#"{"State":{"Status":"running","Running":true,"Health":{"Status":"starting"}}}"#,
            ),
            "/containers/stopped/json" => {
                ("200 OK", r#"{"State":{"Status":"exited","Running":false}}"#)
            }
            _ => ("404 Not Found", r#"{"message":"No such container"}"#),
        };
        format!("HTTP/1.0 {status}\r\nContent-Type: application/json\r\n\r\n{body}")
    }

    async fn answer<S: AsyncRead + AsyncWrite + Unpin>(stream: S) {
        let mut stream = tokio::io::BufReader::new(stream);
        let mut line = String::new();
        stream.read_line(&mut line).await.unwrap();
        let path = line.split_whitespace().nth(1).unwrap().to_owned();
        // Skip the headers.
        loop {
            line.clear();
            stream.read_line(&mut line).await.unwrap();
            if line == "\r\n" {
                break;
            }
        }
        stream.write_all(respond(&path).as_bytes()).await.unwrap();
        stream.shutdown().await.unwrap();
    }

    fn socket_path() -> PathBuf {
        std::env::temp_dir().join(format!(
            "stamon-docker-{}-{:?}.sock",
            std::process::id(),
            std::thread::current().id()
        ))
    }

    async fn serve_unix() -> String {
        let path = socket_path();
        let _ = std::fs::remove_file(&path);
        let listener = UnixListener::bind(&path).unwrap();
        tokio::spawn(async move {
            loop {
                let (stream, _) = listener.accept().await.unwrap();
                tokio::spawn(answer(stream));
            }
        });
        format!("unix://{}", path.display())
    }

    fn config(container: &str, docker_host: &str) -> DockerConfig {
        DockerConfig {
            container: container.into(),
            docker_host: Some(docker_host.into()),
        }
    }

    #[tokio::test]
    async fn container_states() {
        let host = serve_unix().await;
        let status = |outcome: CheckOutcome| (outcome.status, outcome.message);

        assert_eq!(
            status(check(&config("web", &host)).await),
            (CheckStatus::Up, None)
        );
        assert_eq!(
            status(check(&config("booting", &host)).await),
            (CheckStatus::Up, None),
            "no health verdict yet"
        );
        assert_eq!(
            status(check(&config("sick", &host)).await),
            (CheckStatus::Down, Some("Container is unhealthy".into()))
        );
        assert_eq!(
            status(check(&config("stopped", &host)).await),
            (CheckStatus::Down, Some("Container is exited".into()))
        );
        assert_eq!(
            status(check(&config("gone", &host)).await),
            (CheckStatus::Down, Some("No such container: gone".into()))
        );
    }

    #[tokio::test]
    async fn tcp_daemon() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move {
            let (stream, _) = listener.accept().await.unwrap();
            answer(stream).await;
        });
        let outcome = check(&config("web", &format!("tcp://{addr}"))).await;
        assert_eq!(outcome.status, CheckStatus::Up, "{outcome:?}");
    }

    #[tokio::test]
    async fn unreachable_daemon_is_an_error() {
        let outcome = check(&config("web", "unix:///nonexistent/docker.sock")).await;
        assert_eq!(outcome.status, CheckStatus::Error);
        assert!(outcome.message.unwrap().starts_with("Can't reach Docker"));
    }

    #[test]
    fn validation() {
        assert!(
            DockerConfig {
                container: "my-app_1.web".into(),
                docker_host: None
            }
            .validate()
            .is_ok()
        );
        assert!(config("web", "tcp://10.0.0.5:2375").validate().is_ok());
        assert!(config("web", "http://10.0.0.5:2375").validate().is_err());
        assert!(
            config("../images", "unix:///var/run/docker.sock")
                .validate()
                .is_err()
        );
        assert!(
            config("", "unix:///var/run/docker.sock")
                .validate()
                .is_err()
        );
    }
}
