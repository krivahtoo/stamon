use std::{net::IpAddr, str::FromStr, sync::Arc, time::Duration};

use apalis::prelude::{Data, Job, WorkerId};
use ping_rs::PingError;
use tracing::{debug, error};

use crate::{
    models::{
        log::{Log, LogForCreate, Status},
        service::{Service, ServiceForUpdate, ServiceType},
    },
    AppState,
};

impl Job for Service {
    const NAME: &'static str = "stamon::Monitor";
}

#[tracing::instrument(skip(svc), fields(name = svc.name, url = svc.url))]
async fn ping(svc: Service) -> LogForCreate {
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
            error!("Ping failed {}", msg);
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
    let status = match job.service_type {
        ServiceType::Ping => ping(job).await,
        _ => todo!(),
    };
    state
        .tx
        .send(serde_json::to_string(&status).unwrap())
        .unwrap();
    debug!(worker = wid.to_string(), "Service status {}", status);

    // save status
    if let Err(e) = Service::update(
        &state.pool,
        status.service_id,
        ServiceForUpdate {
            last_status: Some(status.status),
            ..Default::default()
        },
    )
    .await
    {
        error!("error {e}");
    };
    if let Err(e) = Log::insert(&state.pool, status).await {
        error!("error {e}");
    };
}
