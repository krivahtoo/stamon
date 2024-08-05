use std::{net::IpAddr, str::FromStr, sync::Arc, time::Duration};

use apalis::prelude::{Data, Job, WorkerId};
use tracing::{debug, error};

use crate::{
    models::{
        log::{Log, LogForCreate, Status},
        service::{Service, ServiceType},
    },
    AppState,
};

impl Job for Service {
    const NAME: &'static str = "stamon::Monitor";
}

#[tracing::instrument]
async fn ping(svc: Service) -> LogForCreate {
    let addr = IpAddr::from_str(&svc.url).unwrap();
    let data = [1, 2, 3, 4]; // ping data
    let data_arc = Arc::new(&data[..]);
    let options = ping_rs::PingOptions {
        ttl: 128,
        dont_fragment: true,
    };
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
                ..Default::default()
            }
        }
        //Err(PingError::IoPending) => Status::Pending(None),
        Err(e) => {
            error!("{:?}", e);
            LogForCreate {
                status: Status::Down,
                message: Some(format!("{:?}", e)),
                service_id: svc.id,
                ..Default::default()
            }
        }
    }
}

pub async fn job_monitor(job: Service, wid: Data<WorkerId>, state: Data<AppState>) {
    let status = match job.service_type {
        ServiceType::Ping => {
            state
                .tx
                .send(format!("running ping on {}", job.url))
                .unwrap();
            ping(job).await
        }
        _ => todo!(),
    };
    debug!(worker = wid.to_string(), "Monitor status {:?}", status);

    // save status
    if let Err(e) = Log::insert(&state.pool, status).await {
        error!("error {e}");
    };
}
