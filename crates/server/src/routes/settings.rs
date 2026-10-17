use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
};
use serde_json::json;
use tracing::error;

use super::demo_forbidden;
use crate::{
    AppState, VERSION,
    auth::{AdminClaims, Claims, TOKEN_MINUTES, ViewerClaims},
    config::env_config,
    models::{settings::Settings, user::UserRole},
};

fn message(status: StatusCode, message: &str) -> Response {
    (status, Json(json!({ "message": message }))).into_response()
}

async fn get_settings(
    ViewerClaims(Claims { role, .. }): ViewerClaims,
    State(state): State<AppState>,
) -> Response {
    let env = env_config();
    match Settings::load(&state.pool, env.log_retention_days).await {
        Ok(settings) => {
            // Server paths are only for admins.
            let data_path = (role == UserRole::Admin).then(|| env.data_path.display().to_string());
            Json(json!({
                "settings": settings,
                "instance": {
                    "version": VERSION,
                    "demo": env.demo,
                    "data_path": data_path,
                    "token_minutes": TOKEN_MINUTES,
                },
            }))
            .into_response()
        }
        Err(e) => {
            error!("{e}");
            message(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
        }
    }
}

async fn update_settings(
    _: AdminClaims,
    State(state): State<AppState>,
    Json(settings): Json<Settings>,
) -> Response {
    if let Some(response) = demo_forbidden() {
        return response;
    }
    let valid = settings.validate().and_then(|()| {
        if env_config().demo {
            settings.validate_demo()
        } else {
            Ok(())
        }
    });
    if let Err(e) = valid {
        return message(StatusCode::BAD_REQUEST, &e);
    }
    match settings.save(&state.pool).await {
        Ok(()) => message(StatusCode::OK, "Settings saved"),
        Err(e) => {
            error!("{e}");
            message(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
        }
    }
}

pub fn routes() -> Router<AppState> {
    Router::new().route("/settings", get(get_settings).put(update_settings))
}
