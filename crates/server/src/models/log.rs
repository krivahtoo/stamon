use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{
    prelude::{FromRow, Type},
    SqlitePool,
};

#[derive(Debug, Clone, Type, Default, Deserialize, Serialize)]
#[repr(i32)]
pub enum Status {
    #[default]
    Pending = 0,
    Up = 1,
    Down = 2,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Log {
    pub id: i64,
    pub service_id: u32,
    pub status: Status,
    pub message: Option<String>,
    pub time: DateTime<Utc>,
    pub duration: u32,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LogForCreate {
    pub service_id: u32,
    pub status: Status,
    pub message: Option<String>,
    pub time: Option<DateTime<Utc>>,
    pub duration: u32,
}

impl Log {
    pub async fn insert(pool: &SqlitePool, log: LogForCreate) -> sqlx::Result<u64> {
        // Construct the base query
        let mut query = "INSERT INTO logs (service_id, status, duration".to_string();
        if log.message.is_some() {
            query.push_str(", message");
        }
        if log.time.is_some() {
            query.push_str(", time");
        }
        query.push_str(") VALUES (?, ?, ?");
        if log.message.is_some() {
            query.push_str(", ?");
        }
        if log.time.is_some() {
            query.push_str(", ?");
        }
        query.push(')');

        // Create a query builder and bind parameters
        let mut query_builder = sqlx::query(&query)
            .bind(log.service_id)
            .bind(log.status)
            .bind(log.duration);

        if let Some(message) = log.message {
            query_builder = query_builder.bind(message);
        }
        if let Some(time) = log.time {
            query_builder = query_builder.bind(time);
        }

        // Execute the query
        let result = query_builder.execute(pool).await?;
        Ok(result.rows_affected())
    }

    pub async fn all(pool: &SqlitePool, limit: Option<u32>) -> sqlx::Result<Vec<Log>> {
        let logs = sqlx::query_as::<_, Log>(r#"SELECT * FROM logs LIMIT ?"#)
            .bind(limit.unwrap_or(1000))
            .fetch_all(pool)
            .await?;

        Ok(logs)
    }
}

#[cfg(test)]
mod tests {
    use sqlx::SqlitePool;

    use super::*;

    #[sqlx::test(fixtures("users", "services"))]
    async fn insert_log(pool: SqlitePool) -> sqlx::Result<()> {
        let count = Log::insert(
            &pool,
            LogForCreate {
                service_id: 1,
                status: Status::Up,
                message: Some("message".to_string()),
                time: Some(Utc::now()),
                duration: 10,
            },
        )
        .await?;

        assert_eq!(count, 1);

        Ok(())
    }

    #[sqlx::test(fixtures("users", "services", "logs"))]
    async fn list_logs(pool: SqlitePool) -> sqlx::Result<()> {
        let logs = Log::all(&pool, None).await?;

        dbg!(&logs);

        assert_eq!(logs.len(), 5);

        Ok(())
    }
}
