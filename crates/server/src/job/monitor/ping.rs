use std::{net::IpAddr, str::FromStr, sync::Arc, time::Duration};

use ping_rs::PingError;
use tokio::sync::broadcast::Sender;
use tracing::{debug, error, warn};

use super::Service;
use crate::{
    models::log::{LogForCreate, Status},
    ws::{Event, Level, Notification},
};

#[tracing::instrument(skip(svc, tx), fields(name = svc.name, url = svc.url))]
pub async fn ping(svc: Service, tx: Sender<Event>) -> LogForCreate {
    let addr = IpAddr::from_str(&svc.url).unwrap();
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
