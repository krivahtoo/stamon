use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use serde_json::{json, Value};
use tracing::{debug, error};

use crate::{
    models::service::{Service, Stats},
    AppState,
};

mod auth;
mod logs;
mod service;
mod users;

async fn stats(State(state): State<AppState>) -> (StatusCode, Json<Value>) {
    match Service::get_stats(&state.pool).await {
        Ok(stats) => (StatusCode::OK, Json(json!({"stats": stats}))),
        Err(e) => {
            error!("{e}");
            (StatusCode::OK, Json(json!({"stats": Stats::default()})))
        }
    }
}

pub fn routes(state: AppState) -> Router<()> {
    let stats_route = Router::new()
        .route("/stats", get(stats))
        .with_state(state.clone());

    Router::new()
        .merge(auth::routes(state.clone()))
        .merge(service::routes(state.clone()))
        .merge(logs::routes(state.clone()))
        .merge(users::routes(state.clone()))
        .merge(stats_route)
        .fallback(root)
}

// fallback handler that responds with a 404
async fn root() -> (StatusCode, Json<Value>) {
    debug!("Not found");
    (
        StatusCode::NOT_FOUND,
        Json(json!({ "message": "Not Found" })),
    )
}
