use axum::{
    extract::{Query, State},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use axum_macros::debug_handler;
use serde::Deserialize;
use serde_json::json;
use tracing::error;

use crate::{auth::Claims, models::log::Log, AppState};

#[derive(Deserialize)]
struct Pagination {
    limit: Option<u32>,
}

#[debug_handler]
async fn list_logs(
    //_: Claims,
    State(state): State<AppState>,
    pagination: Query<Pagination>,
) -> Response {
    match Log::list(&state.pool, pagination.limit).await {
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

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/logs", get(list_logs))
        .with_state(state)
}
