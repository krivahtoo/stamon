use axum::{http::StatusCode, Json, Router};
use serde_json::{json, Value};
use tracing::debug;

use crate::AppState;

mod auth;
mod logs;
mod service;

pub fn routes(state: AppState) -> Router<()> {
    Router::new()
        .merge(auth::routes(state.clone()))
        .merge(service::routes(state.clone()))
        .merge(logs::routes(state.clone()))
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
