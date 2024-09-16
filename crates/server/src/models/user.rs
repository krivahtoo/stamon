use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

use crate::auth::hash;

#[derive(sqlx::Type, Debug, Serialize, Deserialize)]
#[sqlx(rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    Admin,
    Viewer,
}

#[derive(sqlx::FromRow, Serialize)]
pub struct User {
    pub id: u32,
    pub username: String,
    #[serde(skip)]
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
    pub timezone: Option<String>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct UserForLogin {
    pub username: String,
    pub password: String,
}

impl User {
    pub async fn insert(pool: &SqlitePool, user: UserForRegister) -> sqlx::Result<u64> {
        // Default role to Viewer if not provided
        let role = user.role.unwrap_or(UserRole::Viewer);

        // Construct the base query
        let mut query = "INSERT INTO users (username, password, role, active".to_string();
        if user.timezone.is_some() {
            query.push_str(", timezone");
        }
        query.push_str(") VALUES (?, ?, ?, ?");
        if user.timezone.is_some() {
            query.push_str(", ?");
        }
        query.push(')');

        // Create a query builder and bind parameters
        let mut query_builder = sqlx::query(&query)
            .bind(user.username)
            .bind(hash(user.password)) // use hashed password
            .bind(role)
            .bind(true); // Assume new users are active by default

        if let Some(timezone) = user.timezone {
            query_builder = query_builder.bind(timezone);
        }

        // Execute the query
        let result = query_builder.execute(pool).await?;
        Ok(result.rows_affected())
    }

    pub async fn get(pool: &SqlitePool, user_id: u32) -> sqlx::Result<User> {
        sqlx::query_as("SELECT * FROM users WHERE id = ?")
            .bind(user_id)
            .fetch_one(pool)
            .await
    }

    pub async fn list(pool: &SqlitePool) -> sqlx::Result<Vec<User>> {
        sqlx::query_as("SELECT * FROM users").fetch_all(pool).await
    }
}
