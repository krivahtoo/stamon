use axum::{
    Router,
    extract::State,
    response::{IntoResponse, Response},
    routing::get,
};
use axum_macros::debug_handler;
use serde_json::json;
use tracing::error;

use crate::{AppState, auth::Claims, models::user::User};

#[debug_handler]
async fn get_user(Claims { user_id, .. }: Claims, State(state): State<AppState>) -> Response {
    match User::get(&state.pool, user_id).await {
        Ok(user) => Response::builder()
            .header("Content-Type", "application/json")
            .body(json!({ "user": user }).to_string())
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
async fn list_users(_: Claims, State(state): State<AppState>) -> Response {
    match User::list(&state.pool).await {
        Ok(users) => Response::builder()
            .header("Content-Type", "application/json")
            .body(json!({ "users": users }).to_string())
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
        .route("/users", get(list_users))
        .route("/user", get(get_user))
}
