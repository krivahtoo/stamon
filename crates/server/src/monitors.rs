use std::{str::FromStr, time::Duration};

use apalis::{
    layers::{
        WorkerBuilderExt,
        retry::{RetryLayer, RetryPolicy},
        tracing::TraceLayer,
    },
    prelude::{Event, Monitor, WorkerBuilder, WorkerFactoryFn},
};
use apalis_cron::{CronStream, Schedule};
use apalis_sql::sqlite::SqliteStorage;
use tower::load_shed::LoadShedLayer;
use tracing::{debug, error, info};

use crate::{
    AppState,
    job::{self, Notification},
    models::service::Service,
    service, utils,
};

pub async fn monitors(state: &AppState) -> Result<(), Box<dyn std::error::Error>> {
    let notification_storage: SqliteStorage<Notification> = SqliteStorage::new(state.pool.clone());

    let monitor_storage: SqliteStorage<Service> = SqliteStorage::new(state.pool.clone());

    let schedule = Schedule::from_str("* * * * * *")?;
    let cron_timer = WorkerBuilder::new("uptime-timer")
        .enable_tracing()
        .layer(LoadShedLayer::new())
        .rate_limit(2, Duration::from_secs(1))
        .catch_panic()
        .data(service::TimerService::new(state.pool.clone()))
        .backend(CronStream::new(schedule))
        .build_fn(service::run_timer_cron_service);

    let notify_worker = WorkerBuilder::new("notification-worker")
        .layer(TraceLayer::new())
        .backend(notification_storage)
        .build_fn(job::notify);

    let monitor_worker = WorkerBuilder::new("monitor-worker")
        .data(state.clone())
        .layer(RetryLayer::new(RetryPolicy::retries(3)))
        .layer(TraceLayer::new())
        .backend(monitor_storage)
        .build_fn(job::job_monitor);

    Monitor::new()
        .register(cron_timer)
        .register(notify_worker)
        .register(monitor_worker)
        .shutdown_timeout(Duration::from_secs(10))
        .on_event(|e| {
            let worker_id = e.id();
            match e.inner() {
                Event::Start => {
                    info!(target: "worker", worker = %worker_id, "started");
                }
                Event::Engage(task_id) => {
                    if worker_id.name() != "uptime-timer" {
                        debug!(target: "worker", worker = %worker_id, task = %task_id, "engaged");
                    }
                }
                Event::Idle => {
                    debug!(target: "worker", worker = %worker_id, "idle");
                }
                Event::Error(e) => {
                    error!(target: "worker", worker = %worker_id, "error: {e:?}");
                }
                Event::Exit => {
                    info!(target: "worker", worker = %worker_id, "exited");
                }
                Event::Stop => {
                    info!(target: "worker", worker = %worker_id, "stopped");
                }
                _ => {}
            }
        })
        .run_with_signal(async {
            utils::shutdown_signal().await?;
            info!(target: "worker","Ctrl+C Received, Shutting down");
            Ok(())
        })
        .await?;
    Ok(())
}
