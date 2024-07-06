use std::{io, net::SocketAddr, str::FromStr, sync::Arc, time::Duration};

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
use axum::{routing::get, Router};
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use tokio::{signal, sync::broadcast};
use tower_http::{
    services::{ServeDir, ServeFile},
    timeout::TimeoutLayer as HttpTimeoutLayer,
    trace::TraceLayer as HttpTraceLayer,
};
use tracing::{debug, error, info};
use ws::ws_handler;

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
mod ws;

type AppState = Arc<AppStateInner>;

#[derive(Clone, Debug)]
struct AppStateInner {
    pool: SqlitePool,
    tx: broadcast::Sender<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let env = env_config();
    info!(
        "Running stamon version: {}",
        option_env!("STAMON_VERSION").unwrap_or(env!("CARGO_PKG_VERSION"))
    );

    if !Sqlite::database_exists(&env.db_file).await.unwrap_or(false) {
        info!("Creating database {}", env.db_file);
        match Sqlite::create_database(&env.db_file).await {
            Ok(_) => info!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        info!("Database already exists");
    }

    let pool = SqlitePool::connect(&env.db_file)
        .await
        .expect("Should open sqlite db");

    // Run database migrations
    models::setup(&pool).await?;

    let serve_dir = ServeDir::new(&env.assets_path)
        .not_found_service(ServeFile::new(env.assets_path.join("404.html")));
    info!("Serving assets at: {}", env.assets_path.to_string_lossy());
    let (tx, _rx) = broadcast::channel(100);

    // build our application with a route
    let state = Arc::new(AppStateInner { pool, tx });
    let ws_route = Router::new()
        .route("/ws", get(ws_handler))
        .with_state(state.clone());
    let app = Router::new()
        .nest("/api", routes(state.clone()))
        .merge(ws_route)
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
        info!("listening on port {}", env.port);
        axum::serve(
            listener,
            app.into_make_service_with_connect_info::<SocketAddr>(),
        )
        .with_graceful_shutdown(async {
            shutdown_signal()
                .await
                .expect("failed to install Ctrl+C handler");
            info!("Ctrl+C Received, Shutting down");
        })
        .await?;
        Ok::<(), Box<dyn std::error::Error>>(())
    };

    // run background workers
    let monitors = async {
        let notification_storage: SqliteStorage<Notification> =
            SqliteStorage::new(state.pool.clone());

        let monitor_storage: SqliteStorage<MonitorJob> = SqliteStorage::new(state.pool.clone());

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
                shutdown_signal().await?;
                info!(target: "worker","Ctrl+C Received, Shutting down");
                Ok(())
            })
            .await?;
        info!("workers stopped");
        Ok::<(), Box<dyn std::error::Error>>(())
    };

    // Start axum server and workers
    match tokio::join!(http, monitors) {
        (Err(e), _) => error!("Server exited with error: {e}"),
        (_, Err(e)) => error!("Worker exited with error: {e}"),
        _ => ()
    }
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