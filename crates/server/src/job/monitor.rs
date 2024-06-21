use std::{fmt::Debug, net::IpAddr, sync::Arc, time::Duration};

use apalis::prelude::{Data, Job, WorkerId};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tracing::{debug, error};

use crate::AppState;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Status {
    Up,
    Down(Option<String>),
    Pending(Option<String>),
}

#[non_exhaustive]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum MonitorType {
    Ping(IpAddr, Duration),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MonitorJob {
    pub id: i32,
    pub job_type: MonitorType,
}

impl Job for MonitorJob {
    const NAME: &'static str = "stamon::Monitor";
}

#[tracing::instrument]
async fn ping(addr: IpAddr, timeout: Duration) -> Status {
    let data = [1, 2, 3, 4]; // ping data
    let data_arc = Arc::new(&data[..]);
    let options = ping_rs::PingOptions {
        ttl: 128,
        dont_fragment: true,
    };
    match ping_rs::send_ping_async(&addr, timeout, data_arc, Some(&options)).await {
        Ok(reply) => {
            debug!(
                bytes = data.len(),
                time = reply.rtt,
                ttl = options.ttl,
                "reply from {}:",
                reply.address,
            );
            Status::Up
        }
        //Err(PingError::IoPending) => Status::Pending(None),
        Err(e) => {
            error!("{:?}", e);
            Status::Down(Some(format!("{:?}", e)))
        }
    }
}

pub async fn job_monitor(job: MonitorJob, wid: Data<WorkerId>, state: Data<AppState>) {
    let status = match job.job_type {
        MonitorType::Ping(addr, timeout) => {
            state.tx.send(format!("running ping on {}", addr)).unwrap();
            ping(addr, timeout).await
        }
    };

    // save status

    debug!(worker = wid.to_string(), "Monitor status {:?}", status);
}