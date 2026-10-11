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
    auth::{AdminClaims, Claims},
    models::{
        UserForRegister,
        user::{User, UserForUpdate},
    },
};

fn message(status: StatusCode, message: &str) -> Response {
    (status, Json(json!({ "message": message }))).into_response()
}

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

#[debug_handler(state = AppState)]
async fn create_user(
    _: AdminClaims,
    State(state): State<AppState>,
    Json(user): Json<UserForRegister>,
) -> Response {
    if user.username.trim().is_empty() || user.password.is_empty() {
        return message(
            StatusCode::BAD_REQUEST,
            "Username and password are required",
        );
    }
    match User::find_by_username(&state.pool, &user.username).await {
        Ok(Some(_)) => return message(StatusCode::CONFLICT, "Username already taken"),
        Ok(None) => (),
        Err(e) => {
            error!("{e}");
            return message(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error");
        }
    }
    match User::insert(&state.pool, user).await {
        Ok(_) => message(StatusCode::CREATED, "User created"),
        Err(e) => {
            error!("{e}");
            message(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
        }
    }
}

#[debug_handler(state = AppState)]
async fn update_user(
    AdminClaims(claims): AdminClaims,
    State(state): State<AppState>,
    Path(user_id): Path<u32>,
    Json(update): Json<UserForUpdate>,
) -> Response {
    // Prevents an admin from locking everyone out by demoting or disabling themselves.
    if user_id == claims.user_id {
        return message(
            StatusCode::BAD_REQUEST,
            "You cannot change your own role or status",
        );
    }
    match User::update(&state.pool, user_id, update).await {
        Ok(0) => message(StatusCode::NOT_FOUND, "User not found"),
        Ok(_) => message(StatusCode::OK, "User updated"),
        Err(e) => {
            error!("{e}");
            message(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
        }
    }
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/users", get(list_users).post(create_user))
        .route("/users/{id}", put(update_user))
        .route("/user", get(get_user))
}
