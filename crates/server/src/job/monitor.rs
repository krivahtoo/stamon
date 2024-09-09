use std::{net::IpAddr, str::FromStr, sync::Arc, time::Duration};

use apalis::prelude::{Data, Job, WorkerId};
use ping_rs::PingError;
use tokio::sync::broadcast::Sender;
use tracing::{debug, error, warn};

use crate::{
    models::{
        log::{Log, LogForCreate, Status},
        service::{Service, ServiceForUpdate, ServiceType},
    },
    ws::{Event, Level, Notification},
    AppState,
};

impl Job for Service {
    const NAME: &'static str = "stamon::Monitor";
}

#[tracing::instrument(skip(svc, tx), fields(name = svc.name, url = svc.url))]
async fn ping(svc: Service, tx: Sender<Event>) -> LogForCreate {
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

pub async fn job_monitor(job: Service, wid: Data<WorkerId>, state: Data<AppState>) {
    let status_log = match job.service_type {
        ServiceType::Ping => ping(job.clone(), state.tx.clone()).await,
        _ => todo!(),
    };
    if let Err(e) = state.tx.send(Event::Log(status_log.clone())) {
        error!("Failed to send notification: {:?}", e);
    }
    debug!(worker = wid.to_string(), "Service status {}", status_log);

    // Check last status
    match (job.last_status, status_log.status) {
        (Status::Down, Status::Up) => {
            if let Err(e) = state.tx.send(Event::Notification(Notification {
                message: format!("Service {} back Up", job.name),
                title: "Back Up".to_string(),
                level: Level::Success,
            })) {
                error!("Failed to send notification: {:?}", e);
            }
        }
        (Status::Up, Status::Down) => {
            if let Err(e) = state.tx.send(Event::Notification(Notification {
                message: format!("Service {} is Down", job.name),
                title: "Service Down".to_string(),
                level: Level::Warning,
            })) {
                error!("Failed to send notification: {:?}", e);
            }
        }
        (Status::Failed, Status::Up | Status::Down) => {
            if let Err(e) = state.tx.send(Event::Notification(Notification {
                message: format!("Service {} check success", job.name),
                title: "Monitor Success".to_string(),
                level: Level::Info,
            })) {
                error!("Failed to send notification: {:?}", e);
            }
        }
        _ => (),
    };

    // save status
    if let Err(e) = Service::update(
        &state.pool,
        status_log.service_id,
        ServiceForUpdate {
            last_status: Some(status_log.status),
            ..Default::default()
        },
    )
    .await
    {
        error!("error {e}");
    };
    if let Err(e) = Log::insert(&state.pool, status_log).await {
        error!("error {e}");
    };
}
