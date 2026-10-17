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

use super::demo_forbidden;
use crate::{
    AppState,
    auth::{AdminClaims, Claims, ViewerClaims, verify_password},
    models::{
        UserForRegister,
        user::{MIN_PASSWORD_LEN, PasswordChange, ProfileUpdate, User, UserForUpdate},
    },
};

fn message(status: StatusCode, message: &str) -> Response {
    (status, Json(json!({ "message": message }))).into_response()
}

#[debug_handler]
async fn get_user(
    ViewerClaims(Claims { user_id, .. }): ViewerClaims,
    State(state): State<AppState>,
) -> Response {
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
async fn list_users(_: ViewerClaims, State(state): State<AppState>) -> Response {
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
    if let Some(response) = demo_forbidden() {
        return response;
    }
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
    if let Some(response) = demo_forbidden() {
        return response;
    }
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

/// Longest timezone name accepted; IANA names are well under this.
const MAX_TIMEZONE_LEN: usize = 64;

#[debug_handler(state = AppState)]
async fn update_profile(
    ViewerClaims(Claims { user_id, .. }): ViewerClaims,
    State(state): State<AppState>,
    Json(update): Json<ProfileUpdate>,
) -> Response {
    if let Some(response) = demo_forbidden() {
        return response;
    }
    let timezone = update
        .timezone
        .as_deref()
        .map(str::trim)
        .filter(|tz| !tz.is_empty());
    if timezone.is_some_and(|tz| tz.len() > MAX_TIMEZONE_LEN) {
        return message(StatusCode::BAD_REQUEST, "Timezone is too long");
    }
    match User::set_timezone(&state.pool, user_id, timezone).await {
        Ok(_) => message(StatusCode::OK, "Profile updated"),
        Err(e) => {
            error!("{e}");
            message(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
        }
    }
}

#[debug_handler(state = AppState)]
async fn change_password(
    ViewerClaims(Claims { user_id, .. }): ViewerClaims,
    State(state): State<AppState>,
    Json(change): Json<PasswordChange>,
) -> Response {
    if let Some(response) = demo_forbidden() {
        return response;
    }
    if change.new.chars().count() < MIN_PASSWORD_LEN {
        return message(
            StatusCode::BAD_REQUEST,
            &format!("New password must be at least {MIN_PASSWORD_LEN} characters"),
        );
    }
    let user = match User::get(&state.pool, user_id).await {
        Ok(user) => user,
        Err(e) => {
            error!("{e}");
            return message(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error");
        }
    };
    if !verify_password(&change.current, &user.password) {
        return message(StatusCode::BAD_REQUEST, "Current password is incorrect");
    }
    match User::set_password(&state.pool, user_id, change.new).await {
        Ok(_) => message(StatusCode::OK, "Password changed"),
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
        .route("/user", get(get_user).put(update_profile))
        .route("/user/password", put(change_password))
}
