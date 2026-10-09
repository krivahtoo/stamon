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
use chrono::{TimeDelta, Utc};
use sqlx::types::Json as SqlJson;

use crate::{
    AppState,
    auth::{Claims, EditorClaims},
    models::{
        channel::Channel,
        log::Log,
        service::{Service, ServiceForCreate, ServiceForUpdate, clean_tags, fill_push_token},
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

/// The response rejecting channel ids that don't exist.
async fn unknown_channels_rejection(state: &AppState, channel_ids: &[u32]) -> Option<Response> {
    match Channel::missing_ids(&state.pool, channel_ids).await {
        Ok(missing) if missing.is_empty() => None,
        Ok(missing) => Some(bad_request(format!("Unknown channel ids: {missing:?}"))),
        Err(e) => {
            error!("Error checking channels: {e}");
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
    service.tags = clean_tags(std::mem::take(&mut service.tags));
    if let Err(message) = service.validate() {
        return bad_request(message);
    }
    if let Some(response) = push_token_rejection(&state, &service.config, None).await {
        return response;
    }
    if let Some(response) = unknown_channels_rejection(&state, &service.channel_ids).await {
        return response;
    }
    service.user_id = Some(user_id);
    let channel_ids = std::mem::take(&mut service.channel_ids);
    let id = match Service::insert(&state.pool, service).await {
        Ok(id) => id,
        Err(e) => {
            error!("Error adding service: {e}");
            return json_response(500, "Internal server error");
        }
    };
    if let Err(e) = Channel::set_for_service(&state.pool, id, &channel_ids).await {
        error!("Error linking channels to service({id}): {e}");
        return json_response(500, "Service created, but its channels could not be saved");
    }
    Response::builder()
        .status(201)
        .header("Content-Type", "application/json")
        .body(json!({ "message": "Services created", "id": id }).to_string())
        .unwrap()
        .into_response()
}

#[derive(Deserialize)]
struct ServiceChannels {
    channel_ids: Vec<u32>,
}

/// Channels linked to the service; channels that apply to all aren't listed.
#[debug_handler]
async fn get_service_channels(
    _: Claims,
    State(state): State<AppState>,
    Path(service_id): Path<u32>,
) -> Response {
    match Channel::linked_ids(&state.pool, service_id).await {
        Ok(channel_ids) => Json(json!({ "channel_ids": channel_ids })).into_response(),
        Err(e) => {
            error!("Error listing channels of service({service_id}): {e}");
            json_response(500, "Internal server error")
        }
    }
}

#[debug_handler(state = AppState)]
async fn set_service_channels(
    _: EditorClaims,
    State(state): State<AppState>,
    Path(service_id): Path<u32>,
    Json(body): Json<ServiceChannels>,
) -> Response {
    match Service::get(&state.pool, service_id).await {
        Ok(Some(_)) => (),
        Ok(None) => return json_response(404, "Service not found"),
        Err(e) => {
            error!("Error loading service({service_id}): {e}");
            return json_response(500, "Internal server error");
        }
    }
    if let Some(response) = unknown_channels_rejection(&state, &body.channel_ids).await {
        return response;
    }
    match Channel::set_for_service(&state.pool, service_id, &body.channel_ids).await {
        Ok(()) => json_response(200, "Channels updated"),
        Err(e) => {
            error!("Error linking channels to service({service_id}): {e}");
            json_response(500, "Internal server error")
        }
    }
}

/// Uptime over the last day, week and month, and how long the service has
/// had its current status.
#[debug_handler]
async fn service_stats(
    _: Claims,
    State(state): State<AppState>,
    Path(service_id): Path<u32>,
) -> Response {
    let service = match Service::get(&state.pool, service_id).await {
        Ok(Some(service)) => service,
        Ok(None) => return json_response(404, "Service not found"),
        Err(e) => {
            error!("Error loading service({service_id}): {e}");
            return json_response(500, "Internal server error");
        }
    };
    let now = Utc::now();
    let stats = async {
        Ok::<_, sqlx::Error>(json!({
            "uptime_24h": Log::uptime(&state.pool, service_id, now - TimeDelta::days(1)).await?,
            "uptime_7d": Log::uptime(&state.pool, service_id, now - TimeDelta::days(7)).await?,
            "uptime_30d": Log::uptime(&state.pool, service_id, now - TimeDelta::days(30)).await?,
            "status_since": Log::status_since(&state.pool, service_id, service.last_status).await?,
        }))
    };
    match stats.await {
        Ok(stats) => Json(json!({ "stats": stats })).into_response(),
        Err(e) => {
            error!("Error computing stats for service({service_id}): {e}");
            json_response(500, "Internal server error")
        }
    }
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
    if let Some(SqlJson(tags)) = &mut service.tags {
        *tags = clean_tags(std::mem::take(tags));
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
        .route("/services/{id}/stats", get(service_stats))
        .route(
            "/services/{id}/channels",
            get(get_service_channels).put(set_service_channels),
        )
}
