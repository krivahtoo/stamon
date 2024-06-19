use std::time::Duration;

use apalis::{prelude::{Data, Storage, TaskId, WorkerId}, sqlite::SqliteStorage};
use chrono::{DateTime, Timelike, Utc};
use sqlx::sqlite::SqlitePool;
use tracing::{debug, error};

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
        let mut storage: SqliteStorage<Notification> = SqliteStorage::new(self.pool.clone());
        storage
            .push(Notification {
                to: format!("notify:{}@example.com", job.0.num_seconds_from_midnight()),
                text: "Test background job from apalis".to_string(),
            })
            .await?;
        let mut storage: SqliteStorage<MonitorJob> = SqliteStorage::new(self.pool.clone());
        storage
            .push(MonitorJob {
                id: 2,
                job_type: MonitorType::Ping("1.1.1.1".parse().unwrap(), Duration::from_secs(5)),
            })
            .await?;
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

pub async fn run_timer_cron_service(
    job: Timer,
    svc: Data<TimerService>,
    wid: WorkerId,
    id: Data<TaskId>,
) -> bool {
    debug!(
        worker = wid.to_string(),
        id = id.to_string(),
        "running monitor: {}",
        job.0
    );
    match svc.execute(job).await {
        Ok(_) => true,
        Err(e) => {
            error!("{e}");
            false
        }
    }
}