use std::{str::FromStr, time::Duration};

use apalis::{
    cron::{CronStream, Schedule},
    layers::{
        TimeoutLayer,
        retry::{RetryLayer, RetryPolicy},
        tracing::TraceLayer,
    },
    prelude::{Event, Monitor, WorkerBuilder, WorkerFactoryFn},
    sqlite::SqliteStorage,
    utils::TokioExecutor,
};
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
        .data(service::TimerService::new(state.pool.clone()))
        .stream(CronStream::new(schedule).into_stream())
        // curtesy of https://github.com/geofmureithi/apalis/issues/297
        .chain(|s| {
            s.layer(TimeoutLayer::new(Duration::from_secs(60)))
                .layer(TraceLayer::new())
        })
        .build_fn(service::run_timer_cron_service);

    let notify_worker = WorkerBuilder::new("notification-worker")
        .layer(TraceLayer::new())
        .with_storage(notification_storage.clone())
        .build_fn(job::notify);

    let monitor_worker = WorkerBuilder::new("monitor-worker")
        .data(state.clone())
        .layer(RetryLayer::new(RetryPolicy::retries(3)))
        .layer(TraceLayer::new())
        .with_storage(monitor_storage)
        .build_fn(job::job_monitor);

    Monitor::<TokioExecutor>::new()
        .register_with_count(1, cron_timer)
        .register_with_count(1, notify_worker)
        .register_with_count(2, monitor_worker)
        .shutdown_timeout(Duration::from_secs(10))
        .on_event(|e| {
            let worker_id = e.id();
            match e.inner() {
                Event::Start => {
                    info!(target: "worker", worker = %worker_id, "started");
                }
                Event::Engage => {
                    debug!(target: "worker", worker = %worker_id, "engaged");
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
