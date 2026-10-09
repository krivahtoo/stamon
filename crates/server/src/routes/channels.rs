use alerts::{Alert, AlertStatus};
use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use axum_macros::debug_handler;
use chrono::Utc;
use serde::Deserialize;
use serde_json::json;
use tracing::error;

use crate::{
    AppState,
    auth::{AdminClaims, Claims},
    models::{
        channel::{Channel, ChannelForCreate, ChannelForUpdate},
        notification::{DeliveryStatus, Notification, NotificationForCreate},
    },
};

fn message(status: StatusCode, message: &str) -> Response {
    (status, Json(json!({ "message": message }))).into_response()
}

fn internal_error(e: sqlx::Error) -> Response {
    error!("{e}");
    message(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
}

/// Everyone can see which channels exist, to pick them for a service; the
/// configs hold secrets, so they're left out.
#[debug_handler]
async fn list_channels(_: Claims, State(state): State<AppState>) -> Response {
    match Channel::summaries(&state.pool).await {
        Ok(channels) => Json(json!({ "channels": channels })).into_response(),
        Err(e) => internal_error(e),
    }
}

#[debug_handler(state = AppState)]
async fn get_channel(
    _: AdminClaims,
    State(state): State<AppState>,
    Path(channel_id): Path<u32>,
) -> Response {
    match Channel::get(&state.pool, channel_id).await {
        Ok(Some(channel)) => Json(json!({ "channel": channel })).into_response(),
        Ok(None) => message(StatusCode::NOT_FOUND, "Channel not found"),
        Err(e) => internal_error(e),
    }
}

#[debug_handler(state = AppState)]
async fn add_channel(
    AdminClaims(claims): AdminClaims,
    State(state): State<AppState>,
    Json(mut channel): Json<ChannelForCreate>,
) -> Response {
    if let Err(e) = channel.validate() {
        return message(StatusCode::BAD_REQUEST, &e);
    }
    channel.user_id = Some(claims.user_id);
    match Channel::insert(&state.pool, channel).await {
        Ok(id) => (
            StatusCode::CREATED,
            Json(json!({ "message": "Channel created", "id": id })),
        )
            .into_response(),
        Err(e) => internal_error(e),
    }
}

#[debug_handler(state = AppState)]
async fn update_channel(
    _: AdminClaims,
    State(state): State<AppState>,
    Path(channel_id): Path<u32>,
    Json(channel): Json<ChannelForUpdate>,
) -> Response {
    if let Err(e) = channel.validate() {
        return message(StatusCode::BAD_REQUEST, &e);
    }
    match Channel::update(&state.pool, channel_id, channel).await {
        Ok(_) => message(StatusCode::OK, "Channel updated"),
        Err(e) => internal_error(e),
    }
}

#[debug_handler(state = AppState)]
async fn delete_channel(
    _: AdminClaims,
    State(state): State<AppState>,
    Path(channel_id): Path<u32>,
) -> Response {
    match Channel::delete(&state.pool, channel_id).await {
        Ok(0) => message(StatusCode::NOT_FOUND, "Channel not found"),
        Ok(_) => message(StatusCode::OK, "Channel deleted"),
        Err(e) => internal_error(e),
    }
}

/// Send a sample alert right away and report whether it went through.
#[debug_handler(state = AppState)]
async fn test_channel(
    _: AdminClaims,
    State(state): State<AppState>,
    Path(channel_id): Path<u32>,
) -> Response {
    let channel = match Channel::get(&state.pool, channel_id).await {
        Ok(Some(channel)) => channel,
        Ok(None) => return message(StatusCode::NOT_FOUND, "Channel not found"),
        Err(e) => return internal_error(e),
    };
    let alert = Alert {
        service_id: 0,
        service_name: "Stamon".into(),
        target: None,
        status: AlertStatus::Up,
        message: Some(format!(
            "Test alert for the \"{}\" channel. If you can read this, it works.",
            channel.name
        )),
        time: Utc::now(),
    };
    let result = channel.config.send(&alert).await;

    let notification = NotificationForCreate {
        service_id: None,
        channel_id: Some(channel.id),
        title: "Test alert".into(),
        message: alert.message.clone(),
        status: if result.is_ok() {
            DeliveryStatus::Sent
        } else {
            DeliveryStatus::Failed
        },
        error: result.as_ref().err().cloned(),
    };
    if let Err(e) = Notification::insert(&state.pool, notification).await {
        error!("Failed to record test alert: {e}");
    }

    match result {
        Ok(()) => message(StatusCode::OK, "Test alert sent"),
        Err(e) => message(StatusCode::BAD_GATEWAY, &e),
    }
}

#[derive(Deserialize)]
struct Pagination {
    limit: Option<u32>,
}

#[debug_handler]
async fn list_notifications(
    _: Claims,
    State(state): State<AppState>,
    Query(pagination): Query<Pagination>,
) -> Response {
    match Notification::list(&state.pool, pagination.limit).await {
        Ok(notifications) => Json(json!({ "notifications": notifications })).into_response(),
        Err(e) => internal_error(e),
    }
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/channels", get(list_channels).post(add_channel))
        .route(
            "/channels/{id}",
            get(get_channel).put(update_channel).delete(delete_channel),
        )
        .route("/channels/{id}/test", post(test_channel))
        .route("/notifications", get(list_notifications))
}
