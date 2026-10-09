//! Service checks run by the stamon monitor.
//!
//! Each kind of check has its own config type, held by [`CheckConfig`] and
//! stored as JSON. Adding a check type means adding a variant with its config,
//! a module that runs it, and its arms in [`CheckConfig`]'s methods.

use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};

mod http;
mod ping;

pub use http::{HttpConfig, HttpMethod};
pub use ping::PingConfig;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum CheckConfig {
    Http(HttpConfig),
    Ping(PingConfig),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckStatus {
    Up,
    Down,
    /// The check couldn't run, so nothing is known about the service.
    Error,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CheckOutcome {
    pub status: CheckStatus,
    pub latency: Duration,
    pub message: Option<String>,
}

impl CheckOutcome {
    fn up(latency: Duration) -> Self {
        Self {
            status: CheckStatus::Up,
            latency,
            message: None,
        }
    }

    fn down(latency: Duration, message: impl Into<String>) -> Self {
        Self {
            status: CheckStatus::Down,
            latency,
            message: Some(message.into()),
        }
    }

    fn error(message: impl Into<String>) -> Self {
        Self {
            status: CheckStatus::Error,
            latency: Duration::ZERO,
            message: Some(message.into()),
        }
    }
}

impl CheckConfig {
    pub fn validate(&self) -> Result<(), String> {
        match self {
            CheckConfig::Http(config) => config.validate(),
            CheckConfig::Ping(config) => config.validate(),
        }
    }

    /// Run the check, treating anything slower than `timeout` as down.
    pub async fn check(&self, timeout: Duration) -> CheckOutcome {
        let start = Instant::now();
        let check = async {
            match self {
                CheckConfig::Http(config) => http::check(config).await,
                CheckConfig::Ping(config) => ping::check(config, timeout).await,
            }
        };
        match tokio::time::timeout(timeout, check).await {
            Ok(outcome) => outcome,
            Err(_) => CheckOutcome::down(
                start.elapsed(),
                format!("Timed out after {}s", timeout.as_secs_f32()),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn config_json_is_tagged_by_type() {
        let config: CheckConfig =
            serde_json::from_value(json!({ "type": "http", "url": "https://example.com" }))
                .unwrap();
        let CheckConfig::Http(http) = &config else {
            panic!("expected http config, got {config:?}");
        };
        assert_eq!(http.method, HttpMethod::Get);
        assert!(http.headers.is_empty());

        let ping: CheckConfig =
            serde_json::from_value(json!({ "type": "ping", "host": "10.0.0.1" })).unwrap();
        assert_eq!(
            ping,
            CheckConfig::Ping(PingConfig {
                host: "10.0.0.1".into()
            })
        );

        assert!(serde_json::from_value::<CheckConfig>(json!({ "type": "smtp" })).is_err());
        assert_eq!(
            serde_json::to_value(&ping).unwrap(),
            json!({ "type": "ping", "host": "10.0.0.1" })
        );
    }

    #[tokio::test]
    async fn slow_checks_time_out() {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let url = format!("http://{}/", listener.local_addr().unwrap());
        // Accept the connection but never answer.
        tokio::spawn(async move {
            let _conn = listener.accept().await;
            tokio::time::sleep(Duration::from_secs(10)).await;
        });

        let config = CheckConfig::Http(HttpConfig::get(&url));
        let outcome = config.check(Duration::from_millis(300)).await;
        assert_eq!(outcome.status, CheckStatus::Down);
        assert!(outcome.message.unwrap().contains("Timed out"));
    }
}
