use std::time::Instant;

use chrono::Utc;
use tokio::sync::broadcast::Sender;
use tracing::error;

use super::Service;
use crate::{
    models::log::{LogForCreate, Status},
    ws::{Event, Level, Notification},
};

#[tracing::instrument(skip(svc, tx), fields(name = svc.name, url = svc.url))]
pub async fn get(svc: Service, tx: Sender<Event>) -> LogForCreate {
    let now = Instant::now();
    let time = Some(Utc::now());

    match reqwest::get(&svc.url).await {
        Ok(_) => LogForCreate {
            status: Status::Up,
            service_id: svc.id,
            duration: now.elapsed().as_millis() as u32,
            time,
            ..Default::default()
        },
        Err(e) => {
            error!("Failed to get: {:?}", e);
            if e.is_connect() {
                if let Err(e) = tx.send(Event::Notification(Notification {
                    message: format!("Error: {}", e),
                    title: "Network Error".to_string(),
                    level: Level::Error,
                })) {
                    error!("Failed to send notification: {:?}", e);
                };
            }
            LogForCreate {
                status: Status::Down,
                service_id: svc.id,
                message: Some(format!("{e}")),
                duration: now.elapsed().as_millis() as u32,
                time,
            }
        }
    }
}
