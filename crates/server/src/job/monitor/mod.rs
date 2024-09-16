use apalis::prelude::{Data, Job, WorkerId};
use tracing::{debug, error};

use crate::{
    models::{
        log::{Log, Status},
        service::{Service, ServiceForUpdate, ServiceType},
    },
    ws::{Event, Level, Notification},
    AppState,
};

mod http;
mod ping;

impl Job for Service {
    const NAME: &'static str = "stamon::Monitor";
}

pub async fn job_monitor(job: Service, wid: Data<WorkerId>, state: Data<AppState>) {
    let status_log = match job.service_type {
        ServiceType::Ping => ping::ping(job.clone(), state.tx.clone()).await,
        ServiceType::Http => http::get(job.clone(), state.tx.clone()).await,
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