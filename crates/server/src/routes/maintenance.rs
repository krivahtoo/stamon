use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, put},
};
use axum_macros::debug_handler;
use serde_json::json;
use tracing::error;

use crate::{
    AppState,
    auth::{Claims, EditorClaims},
    models::{
        maintenance::{Maintenance, MaintenanceForSave},
        missing_ids,
    },
};

fn message(status: StatusCode, message: &str) -> Response {
    (status, Json(json!({ "message": message }))).into_response()
}

fn internal_error(e: sqlx::Error) -> Response {
    error!("{e}");
    message(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
}

/// The response rejecting an invalid window, if it is one.
async fn rejection(state: &AppState, window: &MaintenanceForSave) -> Option<Response> {
    if let Err(e) = window.validate() {
        return Some(message(StatusCode::BAD_REQUEST, &e));
    }
    match missing_ids(&state.pool, "Services", &window.service_ids).await {
        Ok(missing) if missing.is_empty() => None,
        Ok(missing) => Some(message(
            StatusCode::BAD_REQUEST,
            &format!("Unknown service ids: {missing:?}"),
        )),
        Err(e) => Some(internal_error(e)),
    }
}

#[debug_handler]
async fn list_windows(_: Claims, State(state): State<AppState>) -> Response {
    match Maintenance::list(&state.pool).await {
        Ok(windows) => Json(json!({ "maintenance": windows })).into_response(),
        Err(e) => internal_error(e),
    }
}

#[debug_handler(state = AppState)]
async fn add_window(
    EditorClaims(claims): EditorClaims,
    State(state): State<AppState>,
    Json(mut window): Json<MaintenanceForSave>,
) -> Response {
    if let Some(response) = rejection(&state, &window).await {
        return response;
    }
    window.user_id = Some(claims.user_id);
    match Maintenance::insert(&state.pool, window).await {
        Ok(id) => (
            StatusCode::CREATED,
            Json(json!({ "message": "Maintenance scheduled", "id": id })),
        )
            .into_response(),
        Err(e) => internal_error(e),
    }
}

#[debug_handler(state = AppState)]
async fn replace_window(
    _: EditorClaims,
    State(state): State<AppState>,
    Path(window_id): Path<u32>,
    Json(window): Json<MaintenanceForSave>,
) -> Response {
    if let Some(response) = rejection(&state, &window).await {
        return response;
    }
    match Maintenance::replace(&state.pool, window_id, window).await {
        Ok(true) => message(StatusCode::OK, "Maintenance updated"),
        Ok(false) => message(StatusCode::NOT_FOUND, "Maintenance not found"),
        Err(e) => internal_error(e),
    }
}

#[debug_handler(state = AppState)]
async fn delete_window(
    _: EditorClaims,
    State(state): State<AppState>,
    Path(window_id): Path<u32>,
) -> Response {
    match Maintenance::delete(&state.pool, window_id).await {
        Ok(0) => message(StatusCode::NOT_FOUND, "Maintenance not found"),
        Ok(_) => message(StatusCode::OK, "Maintenance deleted"),
        Err(e) => internal_error(e),
    }
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/maintenance", get(list_windows).post(add_window))
        .route(
            "/maintenance/{id}",
            put(replace_window).delete(delete_window),
        )
}
