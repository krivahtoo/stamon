use std::{net::IpAddr, sync::Arc, time::Duration};

use ping_rs::PingError;
use serde::{Deserialize, Serialize};
use tracing::{debug, warn};

use crate::CheckOutcome;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PingConfig {
    /// IP address or hostname. A URL is accepted and its host is pinged.
    pub host: String,
}

impl PingConfig {
    pub fn validate(&self) -> Result<(), String> {
        if self.host.trim().is_empty() {
            return Err("host must not be empty".into());
        }
        Ok(())
    }
}

/// Resolve a ping target, which may be an IP address, a hostname or a URL.
async fn resolve(target: &str) -> Result<IpAddr, String> {
    let host = reqwest::Url::parse(target)
        .ok()
        .and_then(|url| url.host_str().map(str::to_owned))
        .unwrap_or_else(|| target.trim().to_owned());
    let host = host.trim_start_matches('[').trim_end_matches(']');

    if let Ok(ip) = host.parse::<IpAddr>() {
        return Ok(ip);
    }
    tokio::net::lookup_host((host, 0))
        .await
        .map_err(|e| format!("Failed to resolve {host}: {e}"))?
        .next()
        .map(|addr| addr.ip())
        .ok_or_else(|| format!("No address found for {host}"))
}

pub(crate) async fn check(config: &PingConfig, timeout: Duration) -> CheckOutcome {
    let addr = match resolve(&config.host).await {
        Ok(addr) => addr,
        Err(msg) => {
            warn!("{msg}");
            return CheckOutcome::down(Duration::ZERO, msg);
        }
    };
    let data = [1, 2, 3, 4]; // ping data
    let options = ping_rs::PingOptions {
        ttl: 128,
        dont_fragment: true,
    };
    match ping_rs::send_ping_async(&addr, timeout, Arc::new(&data[..]), Some(&options)).await {
        Ok(reply) => {
            debug!(
                bytes = data.len(),
                time = reply.rtt,
                ttl = options.ttl,
                "reply from {}:",
                reply.address,
            );
            CheckOutcome::up(Duration::from_millis(reply.rtt.into()))
        }
        Err(PingError::OsError(_, msg)) => {
            warn!("Ping failed {}", msg);
            CheckOutcome::error(msg)
        }
        Err(e) => CheckOutcome::down(Duration::ZERO, format!("{e:?}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn resolves_ips_hostnames_and_urls() {
        let localhost = |ip: IpAddr| ip.is_loopback();
        assert!(localhost(resolve("127.0.0.1").await.unwrap()));
        assert!(localhost(resolve("::1").await.unwrap()));
        assert!(localhost(resolve("localhost").await.unwrap()));
        assert!(localhost(
            resolve("http://localhost:8080/health").await.unwrap()
        ));
        assert!(localhost(resolve("http://[::1]/").await.unwrap()));
    }

    #[tokio::test]
    async fn unresolvable_host_is_an_error() {
        assert!(resolve("does-not-exist.invalid").await.is_err());
    }

    #[test]
    fn validation() {
        assert!(PingConfig { host: "  ".into() }.validate().is_err());
        assert!(
            PingConfig {
                host: "localhost".into()
            }
            .validate()
            .is_ok()
        );
    }
}
