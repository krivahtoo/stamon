use axum::{Json, Router, extract::State, http::StatusCode, routing::get};
use serde_json::{Value, json};
use tracing::error;

use crate::{
    AppState,
    models::service::{Service, Stats},
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

pub fn routes() -> Router<AppState> {
    let stats_route = Router::new().route("/stats", get(stats));

    Router::new()
        .merge(auth::routes())
        .merge(service::routes())
        .merge(logs::routes())
        .merge(users::routes())
        .merge(stats_route)
        .fallback(root)
}

// fallback handler that responds with a 404
async fn root() -> (StatusCode, Json<Value>) {
    (
        StatusCode::NOT_FOUND,
        Json(json!({ "message": "Not Found" })),
    )
}
