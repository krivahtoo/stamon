// At the start
#![allow(dead_code)]

use std::{io, str::FromStr, sync::Arc, time::Duration};

use apalis::{
    cron::{CronStream, Schedule},
    layers::{
        retry::{RetryLayer, RetryPolicy},
        tracing::TraceLayer,
        TimeoutLayer,
    },
    prelude::*,
    sqlite::SqliteStorage,
};
use axum::Router;
use config::ConfigStore;
use sqlx::SqlitePool;
use tokio::signal;
use tower_http::{
    services::{ServeDir, ServeFile},
    timeout::TimeoutLayer as HttpTimeoutLayer,
    trace::TraceLayer as HttpTraceLayer,
};
use tracing::{debug, error, info};

use crate::{
    config::env_config,
    job::{MonitorJob, Notification},
    routes::routes,
};

mod auth;
mod config;
mod job;
mod models;
mod routes;
mod service;

type AppState = Arc<AppStateInner>;

#[derive(Clone)]
struct AppStateInner {
    store: ConfigStore,
    pool: SqlitePool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let env = env_config();

    let pool = SqlitePool::connect(&env.db_file)
        .await
        .expect("Should open sqlite db");

    // Run database migrations
    models::setup(&pool).await?;

    let store = ConfigStore::new(&env.config_file).expect("Should load config db");

    let serve_dir = ServeDir::new("assets").not_found_service(ServeFile::new("assets/index.html"));

    // build our application with a route
    let state = Arc::new(AppStateInner { store, pool });
    let app = Router::new()
        .nest("/api", routes(state.clone()))
        .fallback_service(serve_dir)
        .layer((
            HttpTraceLayer::new_for_http(),
            // Graceful shutdown will wait for outstanding requests to complete. Add a timeout so
            // requests don't hang forever.
            HttpTimeoutLayer::new(Duration::from_secs(10)),
        ));

    // run our app with hyper, listening globally on env.port
    let http = async {
        let listener = tokio::net::TcpListener::bind(("0.0.0.0", env.port))
            .await
            .unwrap_or_else(|e| panic!("Can't bind to port {}: {e}", env.port));
        info!("listening on port 3000");
        axum::serve(listener, app)
            .with_graceful_shutdown(async {
                shutdown_signal()
                    .await
                    .expect("failed to install Ctrl+C handler");
            })
            .await?;
        info!("axum server stopped");
        Ok::<(), Box<dyn std::error::Error>>(())
    };

    // run background workers
    let monitors = async {
        let notification_storage: SqliteStorage<Notification> =
            SqliteStorage::new(state.pool.clone());

        let monitor_storage: SqliteStorage<MonitorJob> = SqliteStorage::new(state.pool.clone());

        let schedule = Schedule::from_str("*/10 * * * * *")?;
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
            .data(state.pool.clone())
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
                        info!("Worker [{worker_id}] started");
                    }
                    Event::Engage => {
                        debug!("Worker [{worker_id}] engaged");
                    }
                    Event::Idle => {
                        debug!("Worker [{worker_id}] idle");
                    }
                    Event::Error(e) => {
                        error!("Worker [{worker_id}] encountered an error: {e}");
                    }
                    Event::Exit => {
                        info!("Worker [{worker_id}] exited");
                    }
                    Event::Stop => {
                        info!("Worker [{worker_id}] stopped");
                    }
                }
            })
            .run_with_signal(shutdown_signal())
            .await?;
        Ok::<(), Box<dyn std::error::Error>>(())
    };

    let _res = tokio::join!(http, monitors);
    info!("closing db connection");
    state.pool.close().await;
    Ok(())
}

async fn shutdown_signal() -> io::Result<()> {
    let ctrl_c = signal::ctrl_c();

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        v = ctrl_c => v,
        _ = terminate => Ok(()),
    }
}
