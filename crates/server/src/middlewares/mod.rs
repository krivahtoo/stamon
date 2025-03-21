use axum::{body::Body, extract::Request, middleware::Next, response::Response};

use crate::auth::Claims;

pub mod log;
pub mod role;

// Middleware for filtering loggedin users
pub async fn require_login(_: Claims, req: Request<Body>, next: Next) -> Result<Response, ()> {
    Ok(next.run(req).await)
}
