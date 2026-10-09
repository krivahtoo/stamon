use axum::{
    Json, Router,
    extract::{Path, Query, State},
    response::{IntoResponse, Response},
    routing::get,
};
use axum_macros::debug_handler;
use serde::Deserialize;
use serde_json::json;
use tracing::error;

use checks::CheckConfig;
use sqlx::types::Json as SqlJson;

use crate::{
    AppState,
    auth::{Claims, EditorClaims},
    models::{
        log::Log,
        service::{Service, ServiceForCreate, ServiceForUpdate, fill_push_token},
    },
};

#[derive(Deserialize)]
struct Pagination {
    limit: Option<u32>,
}

fn bad_request(message: String) -> Response {
    json_response(400, &message)
}

fn json_response(status: u16, message: &str) -> Response {
    Response::builder()
        .status(status)
        .header("Content-Type", "application/json")
        .body(json!({ "message": message }).to_string())
        .unwrap()
        .into_response()
}

/// The response rejecting a push token that another service already uses.
async fn push_token_rejection(
    state: &AppState,
    config: &CheckConfig,
    service_id: Option<u32>,
) -> Option<Response> {
    let CheckConfig::Push(push) = config else {
        return None;
    };
    match Service::find_by_push_token(&state.pool, &push.token).await {
        Ok(Some(other)) if Some(other.id) != service_id => {
            Some(json_response(409, "Push token is already in use"))
        }
        Ok(_) => None,
        Err(e) => {
            error!("Error checking push token: {e}");
            Some(json_response(500, "Internal server error"))
        }
    }
}

#[debug_handler(state = AppState)]
async fn add_service(
    EditorClaims(Claims { user_id, .. }): EditorClaims,
    State(state): State<AppState>,
    Json(mut service): Json<ServiceForCreate>,
) -> Response {
    fill_push_token(&mut service.config);
    if let Err(message) = service.validate() {
        return bad_request(message);
    }
    if let Some(response) = push_token_rejection(&state, &service.config, None).await {
        return response;
    }
    service.user_id = Some(user_id);
    if let Err(e) = Service::insert(&state.pool, service).await {
        error!("Error adding service: {e}");
        return Response::builder()
            .status(500)
            .header("Content-Type", "application/json")
            .body(json!({ "message": "Internal server error" }).to_string())
            .unwrap()
            .into_response();
    };
    Response::builder()
        .status(201)
        .header("Content-Type", "application/json")
        .body(json!({ "message": "Services created" }).to_string())
        .unwrap()
        .into_response()
}

#[debug_handler]
async fn get_service(
    _: Claims,
    State(state): State<AppState>,
    Path(service_id): Path<u32>,
) -> Response {
    match Service::get(&state.pool, service_id).await {
        Err(e) => {
            error!("Error updating service({service_id}): {e}");
            Response::builder()
                .status(500)
                .header("Content-Type", "application/json")
                .body(json!({ "message": "Internal server error" }).to_string())
                .unwrap()
                .into_response()
        }
        Ok(Some(s)) => Response::builder()
            .status(200)
            .header("Content-Type", "application/json")
            .body(json!({ "service": s }).to_string())
            .unwrap()
            .into_response(),
        _ => Response::builder()
            .status(404)
            .header("Content-Type", "application/json")
            .body(json!({ "message": "Service not found" }).to_string())
            .unwrap()
            .into_response(),
    }
}

#[debug_handler(state = AppState)]
async fn update_service(
    _: EditorClaims,
    State(state): State<AppState>,
    Path(service_id): Path<u32>,
    Json(mut service): Json<ServiceForUpdate>,
) -> Response {
    if let Some(SqlJson(config)) = &mut service.config {
        fill_push_token(config);
    }
    if let Err(message) = service.validate() {
        return bad_request(message);
    }
    if let Some(SqlJson(config)) = &service.config
        && let Some(response) = push_token_rejection(&state, config, Some(service_id)).await
    {
        return response;
    }
    if let Err(e) = Service::update(&state.pool, service_id, service).await {
        error!("Error updating service({service_id}): {e}");
        return Response::builder()
            .status(500)
            .header("Content-Type", "application/json")
            .body(json!({ "message": "Internal server error" }).to_string())
            .unwrap()
            .into_response();
    };
    Response::builder()
        .status(201)
        .header("Content-Type", "application/json")
        .body(json!({ "message": "Services updated" }).to_string())
        .unwrap()
        .into_response()
}

#[debug_handler(state = AppState)]
async fn delete_service(
    _: EditorClaims,
    State(state): State<AppState>,
    Path(service_id): Path<u32>,
) -> Response {
    let (status, message) = match Service::delete(&state.pool, service_id).await {
        Ok(0) => (404, "Service not found"),
        Ok(_) => (200, "Service deleted"),
        Err(e) => {
            error!("Error deleting service({service_id}): {e}");
            (500, "Internal server error")
        }
    };
    Response::builder()
        .status(status)
        .header("Content-Type", "application/json")
        .body(json!({ "message": message }).to_string())
        .unwrap()
        .into_response()
}

#[debug_handler]
async fn list_services(_: Claims, State(state): State<AppState>) -> Response {
    let Ok(services) = Service::all(&state.pool).await else {
        return Response::builder()
            .header("Content-Type", "application/json")
            .status(500)
            .body(json!({ "message": "Internal server error" }).to_string())
            .unwrap()
            .into_response();
    };
    Response::builder()
        .header("Content-Type", "application/json")
        .body(json!({ "services": services }).to_string())
        .unwrap()
        .into_response()
}

#[debug_handler]
async fn list_service_logs(
    _: Claims,
    State(state): State<AppState>,
    Path(service_id): Path<u32>,
    pagination: Query<Pagination>,
) -> Response {
    match Log::list(&state.pool, service_id, pagination.limit).await {
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

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/services", get(list_services).post(add_service))
        .route(
            "/services/{id}",
            get(get_service).put(update_service).delete(delete_service),
        )
        .route("/services/{id}/logs", get(list_service_logs))
}
