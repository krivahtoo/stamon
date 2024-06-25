use serde::{Serialize, Deserialize};
use chrono_tz::Tz;

#[derive(sqlx::Type, Debug, Serialize, Deserialize)]
#[sqlx(rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    Admin,
    Viewer,
}

#[derive(sqlx::FromRow, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub role: UserRole,
    pub active: bool,
    pub timezone: Option<String>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct UserForRegister {
    pub username: String,
    pub role: Option<UserRole>,
    pub password: String,
    pub timezone: Option<Tz>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct UserForLogin {
    pub username: String,
    pub password: String,
}