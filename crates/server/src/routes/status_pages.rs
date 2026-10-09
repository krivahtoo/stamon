use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, put},
};
use axum_macros::debug_handler;
use chrono::Utc;
use serde_json::json;
use tracing::error;

use crate::{
    AppState,
    auth::{Claims, EditorClaims},
    models::{
        missing_ids,
        status_page::{StatusPage, StatusPageForSave},
    },
};

fn message(status: StatusCode, message: &str) -> Response {
    (status, Json(json!({ "message": message }))).into_response()
}

fn internal_error(e: sqlx::Error) -> Response {
    error!("{e}");
    message(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
}

/// The response rejecting an invalid page, if it is one.
async fn rejection(
    state: &AppState,
    page: &StatusPageForSave,
    page_id: Option<u32>,
) -> Option<Response> {
    if let Err(e) = page.validate() {
        return Some(message(StatusCode::BAD_REQUEST, &e));
    }
    match StatusPage::get_by_slug(&state.pool, &page.slug).await {
        Ok(Some(other)) if Some(other.id) != page_id => {
            return Some(message(StatusCode::CONFLICT, "Slug is already in use"));
        }
        Ok(_) => (),
        Err(e) => return Some(internal_error(e)),
    }
    match missing_ids(&state.pool, "Services", &page.service_ids).await {
        Ok(missing) if missing.is_empty() => None,
        Ok(missing) => Some(message(
            StatusCode::BAD_REQUEST,
            &format!("Unknown service ids: {missing:?}"),
        )),
        Err(e) => Some(internal_error(e)),
    }
}

#[debug_handler]
async fn list_pages(_: Claims, State(state): State<AppState>) -> Response {
    match StatusPage::list(&state.pool).await {
        Ok(pages) => Json(json!({ "status_pages": pages })).into_response(),
        Err(e) => internal_error(e),
    }
}

#[debug_handler(state = AppState)]
async fn add_page(
    EditorClaims(claims): EditorClaims,
    State(state): State<AppState>,
    Json(mut page): Json<StatusPageForSave>,
) -> Response {
    if let Some(response) = rejection(&state, &page, None).await {
        return response;
    }
    page.user_id = Some(claims.user_id);
    match StatusPage::insert(&state.pool, page).await {
        Ok(id) => (
            StatusCode::CREATED,
            Json(json!({ "message": "Status page created", "id": id })),
        )
            .into_response(),
        Err(e) => internal_error(e),
    }
}

#[debug_handler(state = AppState)]
async fn replace_page(
    _: EditorClaims,
    State(state): State<AppState>,
    Path(page_id): Path<u32>,
    Json(page): Json<StatusPageForSave>,
) -> Response {
    if let Some(response) = rejection(&state, &page, Some(page_id)).await {
        return response;
    }
    match StatusPage::replace(&state.pool, page_id, page).await {
        Ok(true) => message(StatusCode::OK, "Status page updated"),
        Ok(false) => message(StatusCode::NOT_FOUND, "Status page not found"),
        Err(e) => internal_error(e),
    }
}

#[debug_handler(state = AppState)]
async fn delete_page(
    _: EditorClaims,
    State(state): State<AppState>,
    Path(page_id): Path<u32>,
) -> Response {
    match StatusPage::delete(&state.pool, page_id).await {
        Ok(0) => message(StatusCode::NOT_FOUND, "Status page not found"),
        Ok(_) => message(StatusCode::OK, "Status page deleted"),
        Err(e) => internal_error(e),
    }
}

/// A published status page. Public: no login needed.
#[debug_handler]
async fn public_page(State(state): State<AppState>, Path(slug): Path<String>) -> Response {
    let page = match StatusPage::get_by_slug(&state.pool, &slug).await {
        Ok(Some(page)) if page.published => page,
        Ok(_) => return message(StatusCode::NOT_FOUND, "Status page not found"),
        Err(e) => return internal_error(e),
    };
    match page.public_view(&state.pool, Utc::now()).await {
        Ok(view) => Json(view).into_response(),
        Err(e) => internal_error(e),
    }
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/status-pages", get(list_pages).post(add_page))
        .route("/status-pages/{id}", put(replace_page).delete(delete_page))
        .route("/status/{slug}", get(public_page))
}
