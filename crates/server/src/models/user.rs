use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

use crate::auth::hash;

#[derive(sqlx::Type, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[sqlx(rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    /// Manages users and settings, and everything an editor can do
    Admin,
    /// Creates, edits and deletes monitors
    Editor,
    /// Read-only access
    Viewer,
}

impl UserRole {
    fn rank(self) -> u8 {
        match self {
            UserRole::Viewer => 0,
            UserRole::Editor => 1,
            UserRole::Admin => 2,
        }
    }

    /// Whether this role grants at least the permissions of `required`.
    pub fn allows(self, required: UserRole) -> bool {
        self.rank() >= required.rank()
    }
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

#[derive(Debug, Default, Deserialize)]
pub struct UserForUpdate {
    pub role: Option<UserRole>,
    pub active: Option<bool>,
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

    pub async fn find_by_username(pool: &SqlitePool, username: &str) -> sqlx::Result<Option<User>> {
        sqlx::query_as("SELECT * FROM Users WHERE username = ?")
            .bind(username)
            .fetch_optional(pool)
            .await
    }

    pub async fn update(
        pool: &SqlitePool,
        user_id: u32,
        update: UserForUpdate,
    ) -> sqlx::Result<u64> {
        let result = sqlx::query(
            "UPDATE Users SET role = COALESCE(?, role), active = COALESCE(?, active) WHERE id = ?",
        )
        .bind(update.role)
        .bind(update.active)
        .bind(user_id)
        .execute(pool)
        .await?;
        Ok(result.rows_affected())
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

    #[test]
    fn role_ordering() {
        assert!(UserRole::Admin.allows(UserRole::Editor));
        assert!(UserRole::Editor.allows(UserRole::Editor));
        assert!(UserRole::Editor.allows(UserRole::Viewer));
        assert!(!UserRole::Editor.allows(UserRole::Admin));
        assert!(!UserRole::Viewer.allows(UserRole::Editor));
    }

    #[sqlx::test(fixtures("users"))]
    async fn update_role_and_active(pool: SqlitePool) -> sqlx::Result<()> {
        let updated = User::update(
            &pool,
            3,
            UserForUpdate {
                role: Some(UserRole::Editor),
                active: None,
            },
        )
        .await?;
        assert_eq!(updated, 1);

        let user = User::get(&pool, 3).await?;
        assert_eq!(user.role, UserRole::Editor);
        assert!(user.active);

        User::update(
            &pool,
            3,
            UserForUpdate {
                role: None,
                active: Some(false),
            },
        )
        .await?;
        let user = User::get(&pool, 3).await?;
        assert_eq!(user.role, UserRole::Editor);
        assert!(!user.active);

        assert_eq!(User::update(&pool, 999, UserForUpdate::default()).await?, 0);
        Ok(())
    }

    #[sqlx::test(fixtures("users"))]
    async fn find_user_by_username(pool: SqlitePool) -> sqlx::Result<()> {
        assert_eq!(User::find_by_username(&pool, "user3").await?.unwrap().id, 3);
        assert!(User::find_by_username(&pool, "nobody").await?.is_none());
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

    /// Run every migration before `version`, then the rest, with `setup`
    /// executed in between, on one connection.
    async fn migrate_around(
        pool: &SqlitePool,
        version: i64,
        setup: &str,
    ) -> sqlx::Result<sqlx::pool::PoolConnection<sqlx::Sqlite>> {
        let mut conn = pool.acquire().await?;
        let migrations = sqlx::migrate!();
        for migration in migrations.iter().filter(|m| m.version < version) {
            sqlx::raw_sql(&migration.sql).execute(&mut *conn).await?;
        }
        sqlx::raw_sql(setup).execute(&mut *conn).await?;
        for migration in migrations.iter().filter(|m| m.version >= version) {
            sqlx::raw_sql(&migration.sql).execute(&mut *conn).await?;
        }
        Ok(conn)
    }

    const ENSURE_ADMIN: i64 = 20251009190000;

    async fn roles(conn: &mut sqlx::SqliteConnection) -> sqlx::Result<Vec<(String, UserRole)>> {
        sqlx::query_as("SELECT username, role FROM Users ORDER BY id")
            .fetch_all(conn)
            .await
    }

    #[sqlx::test(migrations = false)]
    async fn migration_promotes_first_user_without_admin(pool: SqlitePool) -> sqlx::Result<()> {
        let mut conn = migrate_around(
            &pool,
            ENSURE_ADMIN,
            r#"INSERT INTO Users (username, password, role, active) VALUES
               ('gone', 'x', 'viewer', 0),
               ('first', 'x', 'viewer', 1),
               ('second', 'x', 'viewer', 1);"#,
        )
        .await?;
        assert_eq!(
            roles(&mut conn).await?,
            [
                ("gone".to_owned(), UserRole::Viewer),
                ("first".to_owned(), UserRole::Admin),
                ("second".to_owned(), UserRole::Viewer),
            ]
        );
        Ok(())
    }

    #[sqlx::test(migrations = false)]
    async fn migration_keeps_roles_when_an_admin_exists(pool: SqlitePool) -> sqlx::Result<()> {
        let mut conn = migrate_around(
            &pool,
            ENSURE_ADMIN,
            r#"INSERT INTO Users (username, password, role, active) VALUES
               ('first', 'x', 'viewer', 1),
               ('boss', 'x', 'admin', 1);"#,
        )
        .await?;
        assert_eq!(
            roles(&mut conn).await?,
            [
                ("first".to_owned(), UserRole::Viewer),
                ("boss".to_owned(), UserRole::Admin),
            ]
        );
        Ok(())
    }
}
