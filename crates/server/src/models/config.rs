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
        let mut query = "INSERT INTO config (name, value, last_updated".to_string();
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
}
