use chrono::Utc;
use tokio::sync::broadcast::Sender;
use tracing::error;

use super::Service;
use crate::{
    models::log::{LogForCreate, Status},
    ws::Event,
};

#[tracing::instrument(skip(svc, tx), fields(name = svc.name, url = svc.url))]
pub async fn get(svc: Service, tx: Sender<Event>) -> LogForCreate {
    let time = Some(Utc::now());

    match reqwest::get(&svc.url).await {
        Ok(_) => LogForCreate {
            status: Status::Up,
            service_id: svc.id,
            time,
            ..Default::default()
        },
        Err(e) => {
            error!("Failed to get: {:?}", e);
            LogForCreate {
                status: Status::Down,
                service_id: svc.id,
                message: Some(format!("{e}")),
                time,
                ..Default::default()
            }
        }
    }
}
