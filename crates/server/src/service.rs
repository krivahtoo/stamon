use std::time::Duration;

use apalis::{
    prelude::{Data, Storage, TaskId, WorkerId},
    sqlite::SqliteStorage,
};
use chrono::{DateTime, Timelike, Utc};
use sqlx::sqlite::SqlitePool;
use tracing::error;

use crate::job::{monitor::MonitorType, MonitorJob, Notification};

#[derive(Clone)]
pub struct TimerService {
    pub pool: SqlitePool,
}

impl TimerService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn execute(&self, job: Timer) -> Result<(), Box<dyn std::error::Error>> {
        if job.is_interval(16) {
            let mut storage: SqliteStorage<Notification> = SqliteStorage::new(self.pool.clone());
            storage
                .push(Notification {
                    to: format!("notify:{}@example.com", job.0.num_seconds_from_midnight()),
                    text: "Test background job from apalis".to_string(),
                })
                .await?;
        }
        if job.is_interval(10) {
            let mut storage: SqliteStorage<MonitorJob> = SqliteStorage::new(self.pool.clone());
            storage
                .push(MonitorJob {
                    id: 2,
                    job_type: MonitorType::Ping("1.1.1.1".parse().unwrap(), Duration::from_secs(5)),
                })
                .await?;
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

impl Timer {
    fn is_interval(&self, interval: u32) -> bool {
        self.0.num_seconds_from_midnight() % interval == 0
    }
}

pub async fn run_timer_cron_service(
    job: Timer,
    svc: Data<TimerService>,
    wid: WorkerId,
    id: Data<TaskId>,
) -> bool {
    let timer = job.0.to_rfc3339();
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
