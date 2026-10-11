use std::time::Instant;

use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;

use crate::CheckOutcome;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TcpConfig {
    pub host: String,
    pub port: u16,
}

impl TcpConfig {
    pub fn validate(&self) -> Result<(), String> {
        if self.host.trim().is_empty() {
            return Err("host must not be empty".into());
        }
        if self.port == 0 {
            return Err("port must be between 1 and 65535".into());
        }
        Ok(())
    }
}

/// Up when a TCP connection to the port can be opened.
pub(crate) async fn check(config: &TcpConfig) -> CheckOutcome {
    let start = Instant::now();
    let host = config
        .host
        .trim()
        .trim_start_matches('[')
        .trim_end_matches(']');
    match TcpStream::connect((host, config.port)).await {
        Ok(_) => CheckOutcome::up(start.elapsed()),
        Err(e) => CheckOutcome::down(
            start.elapsed(),
            format!("Failed to connect to {host}:{}: {e}", config.port),
        ),
    }
}

#[cfg(test)]
mod tests {
    use tokio::net::TcpListener;

    use super::*;
    use crate::CheckStatus;

    #[tokio::test]
    async fn open_port_is_up_and_closed_port_is_down() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();
        let config = TcpConfig {
            host: "localhost".into(),
            port,
        };
        assert_eq!(check(&config).await.status, CheckStatus::Up);

        drop(listener);
        let outcome = check(&config).await;
        assert_eq!(outcome.status, CheckStatus::Down);
        assert!(outcome.message.unwrap().contains(&port.to_string()));
    }

    #[test]
    fn validation() {
        let config = |host: &str, port| TcpConfig {
            host: host.into(),
            port,
        };
        assert!(config("db.internal", 5432).validate().is_ok());
        assert!(config("", 5432).validate().is_err());
        assert!(config("db.internal", 0).validate().is_err());
    }
}
