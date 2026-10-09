//! Service checks run by the stamon monitor.
//!
//! Each kind of check has its own config type, held by [`CheckConfig`] and
//! stored as JSON. Adding a check type means adding a variant with its config,
//! a module that runs it, and its arms in [`CheckConfig`]'s methods.

use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};

mod database;
mod dns;
mod docker;
mod http;
mod ping;
mod push;
mod tcp;
mod tls;

pub use database::{DatabaseConfig, DatabaseEngine, REDACTED};
pub use dns::{DnsConfig, DnsRecordType};
pub use docker::DockerConfig;
pub use http::{HttpConfig, HttpMethod};
pub use ping::PingConfig;
pub use push::PushConfig;
pub use tcp::TcpConfig;
pub use tls::TlsConfig;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum CheckConfig {
    Http(HttpConfig),
    Ping(PingConfig),
    Tcp(TcpConfig),
    Dns(DnsConfig),
    Tls(TlsConfig),
    Docker(DockerConfig),
    Database(DatabaseConfig),
    /// Passive: the service reports in instead of being checked.
    Push(PushConfig),
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
            CheckConfig::Tcp(config) => config.validate(),
            CheckConfig::Dns(config) => config.validate(),
            CheckConfig::Tls(config) => config.validate(),
            CheckConfig::Docker(config) => config.validate(),
            CheckConfig::Database(config) => config.validate(),
            CheckConfig::Push(config) => config.validate(),
        }
    }

    /// Hide secrets before showing the config.
    pub fn redact(&mut self) {
        if let CheckConfig::Database(config) = self {
            config.redact();
        }
    }

    /// Restore secrets that are still redacted from the config being
    /// replaced, so a config read from the API can be saved back.
    pub fn keep_secrets_from(&mut self, previous: &CheckConfig) {
        if let (CheckConfig::Database(config), CheckConfig::Database(previous)) = (self, previous) {
            config.keep_password_from(previous);
        }
    }

    /// Whether the service reports in instead of being checked.
    pub fn is_passive(&self) -> bool {
        matches!(self, CheckConfig::Push(_))
    }

    /// Run the check, treating anything slower than `timeout` as down.
    pub async fn check(&self, timeout: Duration) -> CheckOutcome {
        let start = Instant::now();
        let check = async {
            match self {
                CheckConfig::Http(config) => http::check(config).await,
                CheckConfig::Ping(config) => ping::check(config, timeout).await,
                CheckConfig::Tcp(config) => tcp::check(config).await,
                CheckConfig::Dns(config) => dns::check(config).await,
                CheckConfig::Tls(config) => tls::check(config, timeout).await,
                CheckConfig::Docker(config) => docker::check(config).await,
                CheckConfig::Database(config) => database::check(config).await,
                CheckConfig::Push(_) => CheckOutcome::error(
                    "Push monitors wait for heartbeats instead of being checked",
                ),
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

        for config in [
            json!({ "type": "tcp", "host": "db", "port": 5432 }),
            json!({ "type": "dns", "host": "example.com", "record_type": "MX" }),
            json!({ "type": "tls", "host": "example.com" }),
            json!({ "type": "push", "token": "abcdefgh12345678" }),
            json!({ "type": "docker", "container": "web" }),
            json!({ "type": "database", "engine": "postgres", "host": "db.internal" }),
        ] {
            let parsed: CheckConfig = serde_json::from_value(config.clone()).unwrap();
            assert!(parsed.validate().is_ok(), "{config}");
            assert_eq!(parsed.is_passive(), config["type"] == "push");
        }
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
