use axum::{
    Router,
    extract::{Query, State},
    response::{IntoResponse, Response},
    routing::get,
};
use axum_macros::debug_handler;
use serde::Deserialize;
use serde_json::json;
use tracing::error;

use crate::{AppState, auth::Claims, models::log::Log};

#[derive(Deserialize)]
struct Pagination {
    limit: Option<u32>,
}

#[debug_handler]
async fn list_logs(
    _: Claims,
    State(state): State<AppState>,
    pagination: Query<Pagination>,
) -> Response {
    match Log::list_all(&state.pool, pagination.limit).await {
        Ok(logs) => Response::builder()
            .header("Content-Type", "application/json")
            .body(json!({ "logs": logs }).to_string())
            .unwrap()
            .into_response(),
        Err(e) => {
            error!("{e}");
            Response::builder()
                .header("Content-Type", "application/json")
                .status(500)
                .body(json!({ "message": "Internal server error" }).to_string())
                .unwrap()
                .into_response()
        }
    }
}

#[debug_handler]
async fn list_log_incidents(
    _: Claims,
    State(state): State<AppState>,
    pagination: Query<Pagination>,
) -> Response {
    match Log::incidents(&state.pool, pagination.limit).await {
        Ok(incidents) => Response::builder()
            .header("Content-Type", "application/json")
            .body(json!({ "incidents": incidents }).to_string())
            .unwrap()
            .into_response(),
        Err(e) => {
            error!("{e}");
            Response::builder()
                .header("Content-Type", "application/json")
                .status(500)
                .body(json!({ "message": "Internal server error" }).to_string())
                .unwrap()
                .into_response()
        }
    }
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/logs", get(list_logs))
        .route("/logs/incidents", get(list_log_incidents))
}
