use chrono::{DateTime, NaiveDate, Utc};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use sqlx::{
    prelude::{FromRow, Type},
    SqlitePool,
};

#[derive(Debug, Clone, Copy, Type, Default, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum Status {
    #[default]
    Pending = 0,
    Up = 1,
    Down = 2,
    /// There was an internal error
    Failed = 3,
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LogForCreate {
    pub service_id: u32,
    pub status: Status,
    pub message: Option<String>,
    pub time: Option<DateTime<Utc>>,
    pub duration: u32,
}

#[derive(Debug, FromRow, Serialize)]
pub struct Incident {
    service_id: u32,
    service_name: String,
    service_url: String,
    status: Status,
    date: NaiveDate,
    count: u32,
    messages: String,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
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

    pub async fn incidents(pool: &SqlitePool, limit: Option<u32>) -> sqlx::Result<Vec<Incident>> {
        let incidents = sqlx::query_as::<_, Incident>(
            r#"SELECT
                s.name AS service_name,
                s.url AS service_url,
                l.service_id,
                l.status,
                DATE(l.time) AS date,
                COUNT(*) AS count,
                GROUP_CONCAT(l.message, '; ') AS messages,
                MIN(l.time) AS start,
                MAX(l.time) AS end
            FROM logs l
            JOIN services s ON l.service_id = s.id
            WHERE l.status > 1
            GROUP BY l.service_id, l.status, date
            ORDER BY date DESC
            LIMIT ?;"#,
        )
        .bind(limit.unwrap_or(20))
        .fetch_all(pool)
        .await?
        .iter()
        .map(|i| {
            let messages = i
                .messages
                .split("; ")
                .unique()
                .collect::<Vec<&str>>()
                .join("; ");
            Incident {
                messages,
                service_id: i.service_id,
                service_name: i.service_name.clone(),
                service_url: i.service_url.clone(),
                status: i.status,
                date: i.date,
                count: i.count,
                start: i.start,
                end: i.end,
            }
        })
        .collect();

        Ok(incidents)
    }

    pub async fn list_all(pool: &SqlitePool, limit: Option<u32>) -> sqlx::Result<Vec<Log>> {
        let logs = sqlx::query_as::<_, Log>(r#"SELECT * FROM logs ORDER BY id DESC LIMIT ?"#)
            .bind(limit.unwrap_or(100))
            .fetch_all(pool)
            .await?;

        Ok(logs)
    }

    pub async fn list(
        pool: &SqlitePool,
        service_id: u32,
        limit: Option<u32>,
    ) -> sqlx::Result<Vec<Log>> {
        let logs = sqlx::query_as::<_, Log>(
            r#"SELECT * FROM logs WHERE service_id = ? ORDER BY id DESC LIMIT ?"#,
        )
        .bind(service_id)
        .bind(limit.unwrap_or(100))
        .fetch_all(pool)
        .await?;

        Ok(logs)
    }
}

impl std::fmt::Display for LogForCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} time={}ms", self.status, self.duration)
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
        let logs = Log::list_all(&pool, None).await?;

        dbg!(&logs);

        assert_eq!(logs.len(), 5);

        Ok(())
    }

    #[sqlx::test(fixtures("users", "services", "logs"))]
    async fn list_logs_limit(pool: SqlitePool) -> sqlx::Result<()> {
        let logs = Log::list_all(&pool, Some(2)).await?;

        dbg!(&logs);

        assert_eq!(logs.len(), 2);

        Ok(())
    }

    #[sqlx::test(fixtures("users", "services", "logs"))]
    async fn list_logs_order(pool: SqlitePool) -> sqlx::Result<()> {
        let logs = Log::list_all(&pool, Some(2)).await?;

        dbg!(&logs);

        assert!(logs.first().unwrap().id > logs.last().unwrap().id);

        assert_eq!(logs.len(), 2);

        Ok(())
    }

    #[sqlx::test(fixtures("users", "services", "logs"))]
    async fn list_service_logs(pool: SqlitePool) -> sqlx::Result<()> {
        let logs = Log::list(&pool, 2, Some(2)).await?;

        dbg!(&logs);

        assert!(logs.first().unwrap().id > logs.last().unwrap().id);

        assert_eq!(logs.len(), 2);

        Ok(())
    }

    #[sqlx::test(fixtures("users", "services", "logs"))]
    async fn list_incidents(pool: SqlitePool) -> sqlx::Result<()> {
        let incidents = Log::incidents(&pool, None).await?;

        dbg!(&incidents);

        //assert!(incidents.first().unwrap().id > incidents.last().unwrap().id);

        assert_eq!(incidents.len(), 1);

        Ok(())
    }
}
