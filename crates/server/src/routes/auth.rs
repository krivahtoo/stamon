use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{
    debug_handler,
    extract::State,
    response::{IntoResponse, Redirect, Response},
    routing::{get, post},
    Json, Router,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use reqwest::StatusCode;
use serde_json::json;
use sqlx::Row;
use tracing::{debug, error};

use crate::{
    auth::Claims,
    config::env_config,
    models::{
        user::{User, UserRole},
        UserForLogin, UserForRegister,
    },
    AppState,
};

#[debug_handler]
async fn login(State(state): State<AppState>, Json(user_login): Json<UserForLogin>) -> Response {
    let users_exists: bool = match sqlx::query("SELECT EXISTS(SELECT 1 FROM users)")
        .fetch_one(&state.pool)
        .await
    {
        Ok(row) => row.get(0),
        Err(e) => {
            error!("{e}");
            false
        }
    };

    if !users_exists {
        debug!("No user registered");
        return Redirect::temporary("/register").into_response();
    }

    let Some(user) = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
        .bind(user_login.username)
        .fetch_optional(&state.pool)
        .await
        .unwrap()
    else {
        return Redirect::to("/register").into_response();
    };

    let is_valid = {
        let parsed_hash = PasswordHash::new(&user.password).unwrap();
        Argon2::default()
            .verify_password(user_login.password.as_bytes(), &parsed_hash)
            .map_or(false, |_| true)
    };

    if !is_valid {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Invalid email or password"})),
        )
            .into_response();
    }

    let now = Utc::now();
    let issued_at = now.timestamp() as usize;
    let expiry = (now + Duration::minutes(60)).timestamp() as usize;
    let claims: Claims = Claims {
        user_id: user.id,
        expiry,
        issued_at,
        role: UserRole::Admin,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(env_config().jwt_secret.as_bytes()),
    )
    .unwrap();

    Response::builder()
        .header("Content-Type", "application/json")
        .header(
            "Set-Cookie",
            format!("token={}; Path=/; SameSite=Lax; Secure; HttpOnly", token),
        )
        .body(json!({ "token": token }).to_string())
        .unwrap()
        .into_response()
}

#[debug_handler]
async fn register(State(state): State<AppState>, Json(user): Json<UserForRegister>) -> Response {
    let users_exists: bool = match sqlx::query("SELECT EXISTS(SELECT 1 FROM users)")
        .fetch_one(&state.pool)
        .await
    {
        Ok(row) => row.get(0),
        Err(e) => {
            error!("{e}");
            false
        }
    };

    if users_exists {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({"error": "Cannot register"})),
        )
            .into_response();
    }

    let count = match User::insert(&state.pool, user).await {
        Ok(c) => c,
        Err(e) => {
            error!("{e}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Registration failed"})),
            )
                .into_response();
        }
    };
    debug!("Register {:?}", count);

    Json(json!({"message": "user registered"})).into_response()
}

async fn logout() -> Response {
    Response::builder()
        .header("Content-Type", "application/json")
        .header(
            "Set-Cookie",
            "token=; Path=/; SameSite=Lax; Secure; HttpOnly; Expires=Thu, 01 Jan 1970 00:00:00 GMT",
        )
        .body(json!({ "message": "Logged out successfully" }).to_string())
        .unwrap()
        .into_response()
}

async fn check(State(state): State<AppState>) -> Response {
    let users_exists: bool = match sqlx::query("SELECT EXISTS(SELECT 1 FROM users)")
        .fetch_one(&state.pool)
        .await
    {
        Ok(row) => row.get(0),
        Err(e) => {
            error!("Failed to check for existing users: {e}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Service temporarily unavailable"})),
            )
                .into_response();
        }
    };

    if !users_exists {
        debug!("No user registered");
        return (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "No user registered"})),
        )
            .into_response();
    }
    Json(json!({"message": "Already setup"})).into_response()
}

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/check", get(check))
        .route("/logout", get(logout))
        .route("/login", post(login))
        .route("/register", post(register))
        .with_state(state)
}
