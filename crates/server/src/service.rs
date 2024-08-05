use std::ops::Deref;

use apalis::{
    prelude::{Data, Storage, TaskId, WorkerId},
    sqlite::SqliteStorage,
};
use chrono::{DateTime, Timelike, Utc};
use sqlx::sqlite::SqlitePool;
use tracing::error;

use crate::models::service::Service;

#[derive(Clone)]
pub struct TimerService {
    pub pool: SqlitePool,
}

impl TimerService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn execute(&self, job: Timer) -> Result<(), Box<dyn std::error::Error>> {
        let mut storage: SqliteStorage<Service> = SqliteStorage::new(self.pool.clone());

        for service in Service::all_active(&self.pool).await? {
            if job.is_interval(service.interval) {
                storage.push(service).await?;
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

impl Timer {
    fn is_interval(&self, interval: u32) -> bool {
        self.num_seconds_from_midnight() % interval == 0
    }
}

pub async fn run_timer_cron_service(
    job: Timer,
    svc: Data<TimerService>,
    wid: WorkerId,
    id: Data<TaskId>,
) -> bool {
    let timer = job.to_rfc3339();
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