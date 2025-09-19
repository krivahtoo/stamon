use chrono::{DateTime, NaiveDate, Utc};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use sqlx::{
    SqlitePool,
    prelude::{FromRow, Type},
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
        let mut query = "INSERT INTO Logs (service_id, status, duration".to_string();
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
            FROM Logs l
            JOIN Services s ON l.service_id = s.id
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
        let logs = sqlx::query_as::<_, Log>(r#"SELECT * FROM Logs ORDER BY id DESC LIMIT ?"#)
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
            r#"SELECT * FROM Logs WHERE service_id = ? ORDER BY id DESC LIMIT ?"#,
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

        assert_eq!(incidents.len(), 1);

        Ok(())
    }

    #[sqlx::test(fixtures("users", "services"))]
    async fn insert_log_with_message(pool: SqlitePool) -> sqlx::Result<()> {
        let count = Log::insert(
            &pool,
            LogForCreate {
                service_id: 1,
                status: Status::Up,
                message: Some("Service is healthy".to_string()),
                time: None, // Should use current time
                duration: 150,
            },
        )
        .await?;

        assert_eq!(count, 1);

        // Verify the log was created
        let logs = Log::list(&pool, 1, Some(1)).await?;
        assert_eq!(logs.len(), 1);
        assert_eq!(logs[0].service_id, 1);
        assert!(matches!(logs[0].status, Status::Up));
        assert_eq!(logs[0].message, Some("Service is healthy".to_string()));
        assert_eq!(logs[0].duration, 150);

        Ok(())
    }

    #[sqlx::test(fixtures("users", "services"))]
    async fn insert_log_without_message(pool: SqlitePool) -> sqlx::Result<()> {
        let count = Log::insert(
            &pool,
            LogForCreate {
                service_id: 2,
                status: Status::Down,
                message: None,
                time: Some(Utc::now()),
                duration: 0,
            },
        )
        .await?;

        assert_eq!(count, 1);

        // Verify the log was created without message
        let logs = Log::list(&pool, 2, Some(1)).await?;
        assert_eq!(logs.len(), 1);
        assert_eq!(logs[0].service_id, 2);
        assert!(matches!(logs[0].status, Status::Down));
        assert!(logs[0].message.is_none());
        assert_eq!(logs[0].duration, 0);

        Ok(())
    }

    #[sqlx::test(fixtures("users", "services"))]
    async fn test_status_enum_values(pool: SqlitePool) -> sqlx::Result<()> {
        // Test all status variants
        let statuses = [Status::Pending, Status::Up, Status::Down, Status::Failed];
        
        for (i, status) in statuses.iter().enumerate() {
            let count = Log::insert(
                &pool,
                LogForCreate {
                    service_id: 1,
                    status: *status,
                    message: Some(format!("Test status {}", i)),
                    time: None,
                    duration: i as u32 * 10,
                },
            )
            .await?;

            assert_eq!(count, 1);
        }

        // Verify all logs were created
        let logs = Log::list(&pool, 1, Some(10)).await?;
        assert_eq!(logs.len(), 4);

        Ok(())
    }

    #[sqlx::test(fixtures("users", "services", "logs"))]
    async fn list_logs_with_limit(pool: SqlitePool) -> sqlx::Result<()> {
        let logs = Log::list(&pool, 2, Some(1)).await?;

        // Should return only 1 log due to limit
        assert_eq!(logs.len(), 1);

        Ok(())
    }

    #[sqlx::test(fixtures("users", "services", "logs"))]
    async fn list_logs_no_limit(pool: SqlitePool) -> sqlx::Result<()> {
        let logs = Log::list(&pool, 2, None).await?;

        // Should return all logs for service 2 (3 logs in fixtures)
        assert_eq!(logs.len(), 3);

        Ok(())
    }

    #[sqlx::test(fixtures("users", "services", "logs"))]
    async fn list_all_logs(pool: SqlitePool) -> sqlx::Result<()> {
        let logs = Log::list_all(&pool, None).await?;

        // Should return all logs in database (5 logs in fixtures)
        assert_eq!(logs.len(), 5);

        Ok(())
    }

    #[sqlx::test(fixtures("users", "services", "logs"))]
    async fn list_incidents_with_limit(pool: SqlitePool) -> sqlx::Result<()> {
        let incidents = Log::incidents(&pool, Some(1)).await?;

        // Should respect the limit
        assert!(incidents.len() <= 1);

        Ok(())
    }

    #[sqlx::test]
    async fn list_logs_empty_database(pool: SqlitePool) -> sqlx::Result<()> {
        let logs = Log::list_all(&pool, None).await?;
        assert_eq!(logs.len(), 0);

        let incidents = Log::incidents(&pool, None).await?;
        assert_eq!(incidents.len(), 0);

        Ok(())
    }
}
