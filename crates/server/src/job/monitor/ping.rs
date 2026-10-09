use std::{net::IpAddr, sync::Arc, time::Duration};

use ping_rs::PingError;
use tokio::sync::broadcast::Sender;
use tracing::{debug, error, warn};

use super::Service;
use crate::{
    models::log::{LogForCreate, Status},
    ws::{Event, Level, Notification},
};

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

#[tracing::instrument(skip(svc, tx), fields(name = svc.name, url = svc.url))]
pub async fn ping(svc: Service, tx: Sender<Event>) -> LogForCreate {
    let addr = match resolve(&svc.url).await {
        Ok(addr) => addr,
        Err(msg) => {
            warn!("{msg}");
            return LogForCreate {
                status: Status::Down,
                message: Some(msg),
                service_id: svc.id,
                time: Some(chrono::Utc::now()),
                ..Default::default()
            };
        }
    };
    let data = [1, 2, 3, 4]; // ping data
    let data_arc = Arc::new(&data[..]);
    let options = ping_rs::PingOptions {
        ttl: 128,
        dont_fragment: true,
    };
    let time = chrono::Utc::now();
    match ping_rs::send_ping_async(
        &addr,
        Duration::from_secs(svc.timeout as u64),
        data_arc,
        Some(&options),
    )
    .await
    {
        Ok(reply) => {
            debug!(
                bytes = data.len(),
                time = reply.rtt,
                ttl = options.ttl,
                "reply from {}:",
                reply.address,
            );
            LogForCreate {
                status: Status::Up,
                duration: reply.rtt,
                service_id: svc.id,
                time: Some(time),
                ..Default::default()
            }
        }
        Err(PingError::OsError(_, msg)) => {
            if let Err(e) = tx.send(Event::Notification(Notification {
                message: format!("Error: {}", msg),
                title: "Network Error".to_string(),
                level: Level::Error,
            })) {
                error!("Failed to send notification: {:?}", e);
            };
            warn!("Ping failed {}", msg);
            LogForCreate {
                status: Status::Failed,
                message: Some(msg),
                service_id: svc.id,
                time: Some(time),
                ..Default::default()
            }
        }
        Err(e) => {
            error!("{:?}", e);
            LogForCreate {
                status: Status::Down,
                message: Some(format!("{:?}", e)),
                service_id: svc.id,
                time: Some(time),
                ..Default::default()
            }
        }
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
}
