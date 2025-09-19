use chrono::{DateTime, Utc};
use sqlx::{FromRow, SqlitePool};

#[derive(Debug, FromRow)]
pub struct Config {
    pub id: u16,
    pub name: String,
    pub value: String,
    pub category: Option<String>,
    pub last_updated: DateTime<Utc>,
}

impl Config {
    pub async fn insert(
        pool: &SqlitePool,
        name: String,
        value: String,
        category: Option<String>,
    ) -> sqlx::Result<u64> {
        // Construct the base query
        let mut query = "INSERT INTO Configs (name, value, last_updated".to_string();
        if category.is_some() {
            query.push_str(", category");
        }
        query.push_str(") VALUES (?, ?, ?");
        if category.is_some() {
            query.push_str(", ?");
        }
        query.push(')');

        // Create a query builder and bind parameters
        let mut query_builder = sqlx::query(&query).bind(name).bind(value).bind(Utc::now());

        if let Some(category_value) = category {
            query_builder = query_builder.bind(category_value);
        }

        // Execute the query
        let result = query_builder.execute(pool).await?;
        Ok(result.rows_affected())
    }

    pub async fn get_by_name(pool: &SqlitePool, name: &str) -> sqlx::Result<Option<Config>> {
        sqlx::query_as("SELECT * FROM Configs WHERE name = ?")
            .bind(name)
            .fetch_optional(pool)
            .await
    }

    pub async fn list_by_category(pool: &SqlitePool, category: &str) -> sqlx::Result<Vec<Config>> {
        sqlx::query_as("SELECT * FROM Configs WHERE category = ?")
            .bind(category)
            .fetch_all(pool)
            .await
    }
}

#[cfg(test)]
mod tests {
    use sqlx::SqlitePool;

    use super::*;

    #[sqlx::test]
    async fn insert_config_without_category(pool: SqlitePool) -> sqlx::Result<()> {
        let count = Config::insert(
            &pool,
            "test_setting".to_string(),
            "test_value".to_string(),
            None,
        )
        .await?;

        assert_eq!(count, 1);

        // Verify the config was inserted
        let config = Config::get_by_name(&pool, "test_setting").await?;
        assert!(config.is_some());
        let config = config.unwrap();
        assert_eq!(config.name, "test_setting");
        assert_eq!(config.value, "test_value");
        assert!(config.category.is_none());

        Ok(())
    }

    #[sqlx::test]
    async fn insert_config_with_category(pool: SqlitePool) -> sqlx::Result<()> {
        let count = Config::insert(
            &pool,
            "notification_enabled".to_string(),
            "true".to_string(),
            Some("notifications".to_string()),
        )
        .await?;

        assert_eq!(count, 1);

        // Verify the config was inserted with category
        let config = Config::get_by_name(&pool, "notification_enabled").await?;
        assert!(config.is_some());
        let config = config.unwrap();
        assert_eq!(config.name, "notification_enabled");
        assert_eq!(config.value, "true");
        assert_eq!(config.category, Some("notifications".to_string()));

        Ok(())
    }

    #[sqlx::test]
    async fn get_nonexistent_config(pool: SqlitePool) -> sqlx::Result<()> {
        let config = Config::get_by_name(&pool, "nonexistent").await?;
        assert!(config.is_none());

        Ok(())
    }

    #[sqlx::test]
    async fn list_configs_by_category(pool: SqlitePool) -> sqlx::Result<()> {
        // Insert multiple configs in the same category
        Config::insert(
            &pool,
            "setting1".to_string(),
            "value1".to_string(),
            Some("category1".to_string()),
        )
        .await?;

        Config::insert(
            &pool,
            "setting2".to_string(),
            "value2".to_string(),
            Some("category1".to_string()),
        )
        .await?;

        Config::insert(
            &pool,
            "setting3".to_string(),
            "value3".to_string(),
            Some("category2".to_string()),
        )
        .await?;

        // Get configs for category1
        let configs = Config::list_by_category(&pool, "category1").await?;
        assert_eq!(configs.len(), 2);

        // Get configs for category2
        let configs = Config::list_by_category(&pool, "category2").await?;
        assert_eq!(configs.len(), 1);

        // Get configs for nonexistent category
        let configs = Config::list_by_category(&pool, "nonexistent").await?;
        assert_eq!(configs.len(), 0);

        Ok(())
    }
}
