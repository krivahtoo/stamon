use std::{net::SocketAddr, sync::Arc, time::Duration};

use axum::{Router, http::HeaderValue, middleware, routing::get};
use middlewares::log::request_logger;
use sqlx::{Sqlite, SqlitePool, migrate::MigrateDatabase};
use tokio::{net::TcpListener, sync::broadcast};
use tower_http::{
    LatencyUnit,
    cors::{Any, CorsLayer},
    services::{ServeDir, ServeFile},
    timeout::TimeoutLayer as HttpTimeoutLayer,
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer as HttpTraceLayer},
};
use tracing::{Level, error, info};
use ws::ws_handler;

use crate::{config::env_config, routes::routes, ws::Event as WsEvent};

mod auth;
mod config;
mod extractors;
mod job;
mod middlewares;
mod models;
mod monitors;
mod routes;
mod service;
mod utils;
mod ws;

type AppState = Arc<AppStateInner>;

#[derive(Clone, Debug)]
struct AppStateInner {
    pool: SqlitePool,
    tx: broadcast::Sender<WsEvent>,
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
        .nest("/api", routes())
        .with_state(state.clone());
    let app = Router::new()
        .merge(ws_route)
        .fallback_service(serve_dir)
        .layer(middleware::from_fn(request_logger))
        .layer((
            /*
            HttpTraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().include_headers(true))
                .on_request(DefaultOnRequest::new().level(Level::DEBUG))
                .on_response(
                    DefaultOnResponse::new()
                        .level(Level::INFO)
                        .latency_unit(LatencyUnit::Millis),
                ),
            */
            // Graceful shutdown will wait for outstanding requests to complete. Add a timeout so
            // requests don't hang forever.
            HttpTimeoutLayer::new(Duration::from_secs(10)),
            #[cfg(debug_assertions)]
            CorsLayer::new()
                .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
                .allow_methods(Any)
                .allow_headers(Any),
        ));

    // run our app with hyper, listening globally on env.port
    let http = async {
        let mut listenfd = listenfd::ListenFd::from_env();
        let listener = match listenfd.take_tcp_listener(0)? {
            Some(listener) => {
                info!("Starting server in developer mode");
                TcpListener::from_std(listener)
            }
            None => TcpListener::bind(("0.0.0.0", env.port)).await,
        }?;
        info!(
            "listening on port {}",
            listener.local_addr().unwrap().port()
        );
        axum::serve(
            listener,
            // this is required for ws to work, I don't know why.
            app.into_make_service_with_connect_info::<SocketAddr>(),
        )
        .with_graceful_shutdown(async {
            utils::shutdown_signal()
                .await
                .expect("failed to install Ctrl+C handler");
            info!("Ctrl+C Received, Shutting down");
        })
        .await?;
        Ok::<(), Box<dyn std::error::Error>>(())
    };

    // run background workers
    let workers = async {
        monitors::monitors(&state).await?;
        info!("workers stopped");
        Ok::<(), Box<dyn std::error::Error>>(())
    };

    // Start axum server and workers
    match tokio::join!(http, workers) {
        (Err(e), _) => error!("Server exited with error: {e}"),
        (_, Err(e)) => error!("Worker exited with error: {e}"),
        _ => (),
    }
    info!("closing db connection");
    state.pool.close().await;
    Ok(())
}
