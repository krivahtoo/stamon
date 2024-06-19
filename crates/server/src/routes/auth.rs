use axum::{
    debug_handler,
    extract::State,
    routing::{get, post},
    Json, Router,
};
use tracing::debug;

use crate::{
    models::{UserForLogin, UserForRegister},
    AppState,
};

#[debug_handler]
async fn login(State(state): State<AppState>, Json(user): Json<UserForLogin>) -> &'static str {
    debug!("Login");
    "Login"
}

#[debug_handler]
async fn register(
    State(state): State<AppState>,
    Json(user): Json<UserForRegister>,
) -> &'static str {
    debug!("Register");
    "Register"
}

// basic handler that responds with a static string
async fn logout() -> &'static str {
    debug!("Root");
    "Hello, World!"
}

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/logout", get(logout))
        .route("/login", post(login))
        .route("/register", post(register))
        .with_state(state)
}