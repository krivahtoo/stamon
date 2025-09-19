use argon2::{
    Argon2, PasswordHasher,
    password_hash::{SaltString, rand_core::OsRng},
};
use axum::{
    RequestPartsExt,
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

pub fn verify_password(password: &str, hash: &str) -> bool {
    use argon2::{PasswordHash, PasswordVerifier};
    
    let parsed_hash = PasswordHash::new(hash).expect("Invalid hash format");
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_password() {
        let password = "test_password".to_string();
        let hashed = hash(password.clone());
        
        // Hash should not be empty
        assert!(!hashed.is_empty());
        
        // Hash should not equal the original password
        assert_ne!(hashed, password);
        
        // Hash should start with argon2 identifier
        assert!(hashed.starts_with("$argon2"));
    }

    #[test]
    fn test_hash_different_passwords() {
        let password1 = "password1".to_string();
        let password2 = "password2".to_string();
        
        let hash1 = hash(password1);
        let hash2 = hash(password2);
        
        // Different passwords should produce different hashes
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_hash_same_password_different_salt() {
        let password = "same_password".to_string();
        
        let hash1 = hash(password.clone());
        let hash2 = hash(password);
        
        // Same password should produce different hashes due to different salts
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_verify_password_correct() {
        let password = "test_password";
        let hashed = hash(password.to_string());
        
        // Correct password should verify successfully
        assert!(verify_password(password, &hashed));
    }

    #[test]
    fn test_verify_password_incorrect() {
        let password = "test_password";
        let wrong_password = "wrong_password";
        let hashed = hash(password.to_string());
        
        // Wrong password should fail verification
        assert!(!verify_password(wrong_password, &hashed));
    }

    #[test]
    fn test_verify_password_empty() {
        let password = "test_password";
        let empty_password = "";
        let hashed = hash(password.to_string());
        
        // Empty password should fail verification
        assert!(!verify_password(empty_password, &hashed));
    }

    #[test]
    fn test_auth_error_display() {
        let missing_creds = AuthError::MissingCredentials;
        let invalid_token = AuthError::InvalidToken;
        
        // Test that errors can be formatted (for debugging)
        assert!(!format!("{:?}", missing_creds).is_empty());
        assert!(!format!("{:?}", invalid_token).is_empty());
    }
}
