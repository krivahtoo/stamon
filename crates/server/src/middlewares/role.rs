use axum::{body::Body, extract::Request, http::StatusCode, middleware::Next, response::Response};
use serde_json::{Value, json};
use tracing::debug;

use crate::{auth::Claims, extractors::json::Json, models::user::UserRole};

// Middleware for filtering admin users
pub async fn require_admin_role(
    claims: Claims,
    req: Request<Body>,
    next: Next,
) -> Result<Response, (StatusCode, Json<Value>)> {
    // Check if claims exist in the request extensions
    if let UserRole::Admin = claims.role {
        Ok(next.run(req).await)
    } else {
        debug!("User not admin");
        Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({ "error": "Unauthorized" })),
        ))
    }
}
