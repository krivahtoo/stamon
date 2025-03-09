use argon2::{Argon2, PasswordHasher, password_hash::SaltString};
use axum::{
    RequestPartsExt, async_trait,
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
    response::{IntoResponse, Json, Response},
};
use axum_extra::{
    TypedHeader,
    extract::CookieJar,
    headers::{Authorization, authorization::Bearer},
};
use jsonwebtoken::{DecodingKey, Validation, decode};
use rand_core::OsRng;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{config::env_config, models::user::UserRole};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    #[serde(rename = "sub")]
    pub user_id: u32,
    pub role: UserRole,
    #[serde(rename = "iat")]
    pub issued_at: usize,
    #[serde(rename = "exp")]
    pub expiry: usize,
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let token = match parts.extract::<TypedHeader<Authorization<Bearer>>>().await {
            Ok(TypedHeader(Authorization(v))) => v.token().to_owned(),
            _ => {
                let cookies = parts
                    .extract::<CookieJar>()
                    .await
                    .map_err(|_| AuthError::MissingCredentials)?;
                cookies
                    .get("token")
                    .map(|c| c.value().to_owned())
                    .ok_or(AuthError::MissingCredentials)?
            }
        };
        // Decode the user data
        let token_data = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(env_config().jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data.claims)
    }
}

#[derive(Debug)]
pub enum AuthError {
    MissingCredentials,
    InvalidToken,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::MissingCredentials => (StatusCode::FORBIDDEN, "Missing credentials"),
            AuthError::InvalidToken => (StatusCode::FORBIDDEN, "Invalid token"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

pub fn hash(val: String) -> String {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(val.as_bytes(), &salt)
        .expect("Error while hashing key")
        .to_string()
}
