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
    #[serde(skip_serializing)]
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
        let mut query = "INSERT INTO Users (username, password, role, active".to_string();
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
        sqlx::query_as("SELECT * FROM Users WHERE id = ?")
            .bind(user_id)
            .fetch_one(pool)
            .await
    }

    pub async fn list(pool: &SqlitePool) -> sqlx::Result<Vec<User>> {
        sqlx::query_as("SELECT * FROM Users").fetch_all(pool).await
    }
}

#[cfg(test)]
mod tests {
    use sqlx::SqlitePool;

    use super::*;

    #[sqlx::test]
    async fn insert_user_with_defaults(pool: SqlitePool) -> sqlx::Result<()> {
        let count = User::insert(
            &pool,
            UserForRegister {
                username: "testuser".to_string(),
                password: "testpass".to_string(),
                role: None, // Should default to Viewer
                timezone: None,
            },
        )
        .await?;

        assert_eq!(count, 1);

        // Verify the user was inserted with correct defaults
        let user = sqlx::query_as::<_, User>("SELECT * FROM Users WHERE username = ?")
            .bind("testuser")
            .fetch_one(&pool)
            .await?;

        assert_eq!(user.username, "testuser");
        assert!(matches!(user.role, UserRole::Viewer));
        assert!(user.active);
        assert!(user.timezone.is_none());

        Ok(())
    }

    #[sqlx::test]
    async fn insert_user_with_admin_role(pool: SqlitePool) -> sqlx::Result<()> {
        let count = User::insert(
            &pool,
            UserForRegister {
                username: "adminuser".to_string(),
                password: "adminpass".to_string(),
                role: Some(UserRole::Admin),
                timezone: Some("UTC".to_string()),
            },
        )
        .await?;

        assert_eq!(count, 1);

        // Verify the user was inserted with admin role
        let user = sqlx::query_as::<_, User>("SELECT * FROM Users WHERE username = ?")
            .bind("adminuser")
            .fetch_one(&pool)
            .await?;

        assert_eq!(user.username, "adminuser");
        assert!(matches!(user.role, UserRole::Admin));
        assert!(user.active);
        assert_eq!(user.timezone, Some("UTC".to_string()));

        Ok(())
    }

    #[sqlx::test(fixtures("users"))]
    async fn get_existing_user(pool: SqlitePool) -> sqlx::Result<()> {
        let user = User::get(&pool, 1).await?;

        assert_eq!(user.id, 1);
        assert_eq!(user.username, "user1");
        assert!(matches!(user.role, UserRole::Admin));
        assert!(user.active);

        Ok(())
    }

    #[sqlx::test(fixtures("users"))]
    async fn get_nonexistent_user_fails(pool: SqlitePool) -> sqlx::Result<()> {
        let result = User::get(&pool, 999).await;
        assert!(result.is_err());

        Ok(())
    }

    #[sqlx::test(fixtures("users"))]
    async fn list_all_users(pool: SqlitePool) -> sqlx::Result<()> {
        let users = User::list(&pool).await?;

        assert_eq!(users.len(), 4);
        assert_eq!(users[0].username, "user1");
        assert_eq!(users[1].username, "user2");

        Ok(())
    }
}
