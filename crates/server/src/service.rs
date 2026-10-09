use std::ops::Deref;

use apalis::prelude::*;
use apalis_sql::sqlite::SqliteStorage;
use chrono::{DateTime, Timelike, Utc};
use sqlx::sqlite::SqlitePool;
use tracing::{debug, error};

use crate::{job::CheckJob, models::service::Service};

#[derive(Clone)]
pub struct TimerService {
    pub pool: SqlitePool,
}

impl TimerService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn execute(&self, job: Timer) -> Result<(), Box<dyn std::error::Error>> {
        let mut storage: SqliteStorage<CheckJob> = SqliteStorage::new(self.pool.clone());
        let now = Utc::now().timestamp();

        for service in Service::due(&self.pool, now).await? {
            // Reschedule before queueing so a failed push skips one run instead of
            // queueing the service again on every tick.
            let next_run_at = now + i64::from(service.interval.max(1));
            Service::set_next_run(&self.pool, service.id, next_run_at).await?;
            storage
                .push(CheckJob {
                    service_id: service.id,
                })
                .await?;
        }

        if job.second() == 0 {
            let removed = storage.vacuum().await?;
            if removed > 0 {
                debug!("Removed {removed} finished jobs");
            }
        }
        Ok(())
    }
}

#[derive(Default, Debug, Clone)]
pub struct Timer(DateTime<Utc>);

impl From<DateTime<Utc>> for Timer {
    fn from(t: DateTime<Utc>) -> Self {
        Timer(t)
    }
}

impl Deref for Timer {
    type Target = DateTime<Utc>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub async fn run_timer_cron_service(
    job: Timer,
    svc: Data<TimerService>,
    worker: Worker<Context>,
    id: TaskId,
) -> bool {
    let timer = job.to_rfc3339();
    let wid = worker.id();
    match svc.execute(job).await {
        Ok(_) => true,
        Err(e) => {
            error!(
                worker = wid.to_string(),
                id = id.to_string(),
                timer = timer,
                "{e}"
            );
            false
        }
    }
}
