use std::time::Duration;

use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
};
use checks::{CheckOutcome, CheckStatus};
use chrono::Utc;
use serde::Deserialize;
use serde_json::json;
use tracing::error;

use crate::{
    AppState,
    job::monitor::{push_deadline, record},
    models::service::Service,
};

#[derive(Deserialize)]
struct Heartbeat {
    /// `up` (the default) or `down`.
    status: Option<String>,
    /// Message stored with the heartbeat.
    msg: Option<String>,
    /// Latency to record, in milliseconds.
    ping: Option<u64>,
}

fn message(status: StatusCode, message: &str) -> Response {
    (status, Json(json!({ "message": message }))).into_response()
}

/// Heartbeat from a push monitor. Public: the token in the URL is the secret.
async fn push(
    State(state): State<AppState>,
    Path(token): Path<String>,
    Query(heartbeat): Query<Heartbeat>,
) -> Response {
    let status = match heartbeat.status.as_deref() {
        None | Some("up") => CheckStatus::Up,
        Some("down") => CheckStatus::Down,
        Some(_) => return message(StatusCode::BAD_REQUEST, "status must be up or down"),
    };

    let mut svc = match Service::find_by_push_token(&state.pool, &token).await {
        Ok(Some(svc)) if svc.active => svc,
        Ok(_) => return message(StatusCode::NOT_FOUND, "Push monitor not found or paused"),
        Err(e) => {
            error!("Failed to look up push token: {e}");
            return message(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error");
        }
    };

    let now = Utc::now();
    svc.last_push_at = Some(now.timestamp());
    let deadline = push_deadline(&svc).unwrap_or(now.timestamp());
    if let Err(e) = Service::record_push(&state.pool, svc.id, now.timestamp(), deadline).await {
        error!("Failed to record heartbeat for service {}: {e}", svc.id);
        return message(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error");
    }

    let outcome = CheckOutcome {
        status,
        latency: Duration::from_millis(heartbeat.ping.unwrap_or(0)),
        message: heartbeat.msg.filter(|m| !m.trim().is_empty()),
    };
    record(&state, &svc, now, outcome).await;
    message(StatusCode::OK, "ok")
}

pub fn routes() -> Router<AppState> {
    Router::new().route("/push/{token}", get(push).post(push))
}
