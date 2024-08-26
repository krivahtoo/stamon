use axum::{
    extract::{Path, State},
    response::{IntoResponse, Response},
    routing::{get, put},
    Json, Router,
};
use axum_macros::debug_handler;
use serde_json::json;
use tracing::error;

use crate::{
    auth::Claims,
    models::service::{Service, ServiceForCreate, ServiceForUpdate},
    AppState,
};

#[debug_handler]
async fn add_service(
    _: Claims,
    State(state): State<AppState>,
    Json(service): Json<ServiceForCreate>,
) -> Response {
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
async fn update_service(
    //_: Claims,
    State(state): State<AppState>,
    Path(service_id): Path<u32>,
    Json(service): Json<ServiceForUpdate>,
) -> Response {
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

#[debug_handler]
async fn list_services(State(state): State<AppState>) -> Response {
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

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/service", get(list_services).post(add_service))
        .route("/service/:id", put(update_service))
        .with_state(state)
}
