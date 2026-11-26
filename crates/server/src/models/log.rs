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
    /// Not checked: the service is in a maintenance window
    Maintenance = 4,
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
    service_target: Option<String>,
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
                s.target AS service_target,
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
                service_target: i.service_target.clone(),
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

    /// A service's logs, newest first. With `before`, only logs older than
    /// that log id, for paging.
    pub async fn list(
        pool: &SqlitePool,
        service_id: u32,
        limit: Option<u32>,
        before: Option<i64>,
    ) -> sqlx::Result<Vec<Log>> {
        let logs = sqlx::query_as::<_, Log>(
            r#"SELECT * FROM Logs
               WHERE service_id = ? AND id < COALESCE(?, 9223372036854775807)
               ORDER BY id DESC LIMIT ?"#,
        )
        .bind(service_id)
        .bind(before)
        .bind(limit.unwrap_or(100))
        .fetch_all(pool)
        .await?;

        Ok(logs)
    }
}

/// Availability of a service over a period.
#[derive(Debug, Default, Serialize, PartialEq)]
pub struct Uptime {
    /// Percentage of up checks among up and down ones; `None` without any.
    pub percent: Option<f64>,
    /// Mean response time of up checks, in milliseconds.
    pub avg_latency: Option<f64>,
    pub checks: u32,
}

/// Up and down results in one slice of time. Latency only counts up checks,
/// so it is `None` when every check in the bucket was down.
#[derive(Debug, Serialize, PartialEq)]
pub struct LatencyBucket {
    pub start: DateTime<Utc>,
    pub avg: Option<f64>,
    pub min: Option<u32>,
    pub max: Option<u32>,
    pub up: u32,
    pub down: u32,
}

/// Response times of a service over a period, in buckets, oldest first.
/// Buckets without up or down results are left out.
#[derive(Debug, Serialize, PartialEq)]
pub struct Latency {
    pub bucket_secs: i64,
    pub buckets: Vec<LatencyBucket>,
    /// Mean response time of all the up checks.
    pub avg: Option<f64>,
    pub min: Option<u32>,
    pub max: Option<u32>,
}

impl Log {
    /// Uptime since `since`. Pending, failed and maintenance results say
    /// nothing about the service, so they don't count.
    pub async fn uptime(
        pool: &SqlitePool,
        service_id: u32,
        since: DateTime<Utc>,
    ) -> sqlx::Result<Uptime> {
        let (up, down, avg_latency): (u32, u32, Option<f64>) = sqlx::query_as(
            r#"SELECT COUNT(*) FILTER (WHERE status = 1),
                      COUNT(*) FILTER (WHERE status = 2),
                      AVG(duration) FILTER (WHERE status = 1)
               FROM Logs
               WHERE service_id = ? AND datetime(time) >= datetime(?)"#,
        )
        .bind(service_id)
        .bind(since)
        .fetch_one(pool)
        .await?;

        let checks = up + down;
        Ok(Uptime {
            percent: (checks > 0).then(|| f64::from(up) * 100.0 / f64::from(checks)),
            avg_latency,
            checks,
        })
    }

    /// Response times since `since`, in buckets of `bucket_secs` aligned to
    /// the Unix epoch.
    pub async fn latency(
        pool: &SqlitePool,
        service_id: u32,
        since: DateTime<Utc>,
        bucket_secs: i64,
    ) -> sqlx::Result<Latency> {
        let rows: Vec<(i64, Option<f64>, Option<u32>, Option<u32>, u32, u32)> = sqlx::query_as(
            r#"SELECT CAST(strftime('%s', time) AS INTEGER) / ?2 * ?2 AS bucket,
                      AVG(duration) FILTER (WHERE status = 1),
                      MIN(duration) FILTER (WHERE status = 1),
                      MAX(duration) FILTER (WHERE status = 1),
                      COUNT(*) FILTER (WHERE status = 1),
                      COUNT(*) FILTER (WHERE status = 2)
               FROM Logs
               WHERE service_id = ?1 AND datetime(time) >= datetime(?3)
               GROUP BY bucket
               HAVING COUNT(*) FILTER (WHERE status IN (1, 2)) > 0
               ORDER BY bucket"#,
        )
        .bind(service_id)
        .bind(bucket_secs)
        .bind(since)
        .fetch_all(pool)
        .await?;

        let buckets: Vec<_> = rows
            .into_iter()
            .map(|(start, avg, min, max, up, down)| LatencyBucket {
                start: DateTime::from_timestamp(start, 0).unwrap_or_default(),
                avg,
                min,
                max,
                up,
                down,
            })
            .collect();

        let up: u32 = buckets.iter().map(|b| b.up).sum();
        let total: f64 = buckets
            .iter()
            .filter_map(|b| b.avg.map(|avg| avg * f64::from(b.up)))
            .sum();
        Ok(Latency {
            bucket_secs,
            avg: (up > 0).then(|| total / f64::from(up)),
            min: buckets.iter().filter_map(|b| b.min).min(),
            max: buckets.iter().filter_map(|b| b.max).max(),
            buckets,
        })
    }

    /// Up and down results per UTC day since `since`, oldest first; days
    /// without any are left out.
    pub async fn daily_counts(
        pool: &SqlitePool,
        service_id: u32,
        since: DateTime<Utc>,
    ) -> sqlx::Result<Vec<(NaiveDate, u32, u32)>> {
        sqlx::query_as(
            r#"SELECT DATE(time) AS day,
                      COUNT(*) FILTER (WHERE status = 1),
                      COUNT(*) FILTER (WHERE status = 2)
               FROM Logs
               WHERE service_id = ? AND datetime(time) >= datetime(?)
               GROUP BY day
               HAVING COUNT(*) FILTER (WHERE status IN (1, 2)) > 0
               ORDER BY day"#,
        )
        .bind(service_id)
        .bind(since)
        .fetch_all(pool)
        .await
    }

    /// When the service's current status began: the first log with that
    /// status after the last one with a different status. Pending, failed
    /// and maintenance results say nothing about the service, so they do
    /// not end a run.
    pub async fn status_since(
        pool: &SqlitePool,
        service_id: u32,
        status: Status,
    ) -> sqlx::Result<Option<DateTime<Utc>>> {
        sqlx::query_scalar(
            r#"SELECT MIN(time)
               FROM Logs
               WHERE service_id = ?1
                 AND status = ?2
                 AND id > COALESCE(
                     (SELECT MAX(id) FROM Logs
                      WHERE service_id = ?1 AND status != ?2 AND status NOT IN (0, 3, 4)), 0)"#,
        )
        .bind(service_id)
        .bind(status)
        .fetch_one(pool)
        .await
    }

    /// Delete logs older than `before`, returning how many were removed.
    pub async fn prune(pool: &SqlitePool, before: DateTime<Utc>) -> sqlx::Result<u64> {
        let result = sqlx::query("DELETE FROM Logs WHERE datetime(time) < datetime(?)")
            .bind(before)
            .execute(pool)
            .await?;
        Ok(result.rows_affected())
    }
}

impl std::fmt::Display for LogForCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} time={}ms", self.status, self.duration)
    }
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;
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
        let logs = Log::list(&pool, 2, Some(2), None).await?;

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
        let logs = Log::list(&pool, 1, Some(1), None).await?;
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
        let logs = Log::list(&pool, 2, Some(1), None).await?;
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
        let statuses = [
            Status::Pending,
            Status::Up,
            Status::Down,
            Status::Failed,
            Status::Maintenance,
        ];

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
        let logs = Log::list(&pool, 1, Some(10), None).await?;
        assert_eq!(logs.len(), 5);
        assert!(matches!(logs[0].status, Status::Maintenance));

        Ok(())
    }

    #[sqlx::test(fixtures("users", "services", "logs"))]
    async fn list_logs_with_limit(pool: SqlitePool) -> sqlx::Result<()> {
        let logs = Log::list(&pool, 2, Some(1), None).await?;

        // Should return only 1 log due to limit
        assert_eq!(logs.len(), 1);

        Ok(())
    }

    #[sqlx::test(fixtures("users", "services", "logs"))]
    async fn list_logs_no_limit(pool: SqlitePool) -> sqlx::Result<()> {
        let logs = Log::list(&pool, 2, None, None).await?;

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

    async fn insert_at(pool: &SqlitePool, status: Status, minutes_ago: i64, duration: u32) {
        Log::insert(
            pool,
            LogForCreate {
                service_id: 1,
                status,
                message: None,
                time: Some(Utc::now() - chrono::TimeDelta::minutes(minutes_ago)),
                duration,
            },
        )
        .await
        .unwrap();
    }

    #[sqlx::test(fixtures("users", "services"))]
    async fn uptime_counts_up_and_down_in_window(pool: SqlitePool) -> sqlx::Result<()> {
        let hour_ago = Utc::now() - chrono::TimeDelta::hours(1);
        assert_eq!(Log::uptime(&pool, 1, hour_ago).await?, Uptime::default());

        insert_at(&pool, Status::Up, 120, 500).await; // outside the window
        insert_at(&pool, Status::Up, 50, 100).await;
        insert_at(&pool, Status::Up, 40, 300).await;
        insert_at(&pool, Status::Up, 30, 200).await;
        insert_at(&pool, Status::Down, 20, 9).await;
        insert_at(&pool, Status::Pending, 15, 9).await;
        insert_at(&pool, Status::Failed, 10, 0).await;

        let uptime = Log::uptime(&pool, 1, hour_ago).await?;
        assert_eq!(uptime.checks, 4);
        assert_eq!(uptime.percent, Some(75.0));
        assert_eq!(uptime.avg_latency, Some(200.0));
        Ok(())
    }

    #[sqlx::test(fixtures("users", "services"))]
    async fn status_since_finds_last_change(pool: SqlitePool) -> sqlx::Result<()> {
        assert_eq!(Log::status_since(&pool, 1, Status::Up).await?, None);

        insert_at(&pool, Status::Up, 50, 1).await;
        insert_at(&pool, Status::Down, 40, 1).await;
        insert_at(&pool, Status::Up, 30, 1).await;
        insert_at(&pool, Status::Up, 20, 1).await;

        let since = Log::status_since(&pool, 1, Status::Up).await?.unwrap();
        let minutes = (Utc::now() - since).num_minutes();
        assert_eq!(minutes, 30, "since the recovery 30 minutes ago");
        Ok(())
    }

    #[sqlx::test(fixtures("users", "services"))]
    async fn status_since_ignores_maintenance_and_retries(pool: SqlitePool) -> sqlx::Result<()> {
        insert_at(&pool, Status::Down, 60, 1).await;
        insert_at(&pool, Status::Up, 50, 1).await;
        insert_at(&pool, Status::Maintenance, 40, 0).await;
        insert_at(&pool, Status::Pending, 30, 1).await;
        insert_at(&pool, Status::Failed, 25, 0).await;
        insert_at(&pool, Status::Up, 20, 1).await;

        let since = Log::status_since(&pool, 1, Status::Up).await?.unwrap();
        let minutes = (Utc::now() - since).num_minutes();
        assert_eq!(minutes, 50, "maintenance and retries do not end the Up run");
        Ok(())
    }

    #[sqlx::test(fixtures("users", "services"))]
    async fn prune_removes_old_logs(pool: SqlitePool) -> sqlx::Result<()> {
        insert_at(&pool, Status::Up, 60 * 24 * 100, 1).await;
        insert_at(&pool, Status::Up, 10, 1).await;
        let removed = Log::prune(&pool, Utc::now() - chrono::TimeDelta::days(90)).await?;
        assert_eq!(removed, 1);
        assert_eq!(Log::list(&pool, 1, None, None).await?.len(), 1);
        Ok(())
    }

    #[sqlx::test(fixtures("users", "services"))]
    async fn daily_counts_group_by_day(pool: SqlitePool) -> sqlx::Result<()> {
        let day_of =
            |minutes_ago: i64| (Utc::now() - chrono::TimeDelta::minutes(minutes_ago)).date_naive();
        insert_at(&pool, Status::Up, 60 * 24 * 2, 1).await;
        insert_at(&pool, Status::Down, 60 * 24 * 2, 1).await;
        insert_at(&pool, Status::Maintenance, 60 * 24, 1).await; // a day with no checks
        insert_at(&pool, Status::Up, 1, 1).await;

        let counts = Log::daily_counts(&pool, 1, Utc::now() - chrono::TimeDelta::days(30)).await?;
        assert_eq!(counts, [(day_of(60 * 24 * 2), 1, 1), (day_of(1), 1, 0)]);
        Ok(())
    }

    #[sqlx::test(fixtures("users", "services", "logs"))]
    async fn list_service_logs_before(pool: SqlitePool) -> sqlx::Result<()> {
        let ids = |logs: Vec<Log>| logs.into_iter().map(|log| log.id).collect::<Vec<_>>();
        let all = ids(Log::list(&pool, 2, None, None).await?);
        assert_eq!(all.len(), 3);

        let older = ids(Log::list(&pool, 2, None, Some(all[0])).await?);
        assert_eq!(older, all[1..], "only logs older than the given id");
        assert!(Log::list(&pool, 2, None, Some(all[2])).await?.is_empty());
        Ok(())
    }

    #[sqlx::test(fixtures("users", "services"))]
    async fn latency_groups_checks_into_buckets(pool: SqlitePool) -> sqlx::Result<()> {
        let at = |minute: u32| Utc.with_ymd_and_hms(2024, 7, 27, 10, minute, 0).unwrap();
        let insert = |status, time, duration| {
            Log::insert(
                &pool,
                LogForCreate {
                    service_id: 1,
                    status,
                    message: None,
                    time: Some(time),
                    duration,
                },
            )
        };
        insert(Status::Up, at(0) - chrono::TimeDelta::minutes(5), 999).await?; // before `since`
        insert(Status::Up, at(1), 100).await?;
        insert(Status::Up, at(5), 200).await?;
        insert(Status::Down, at(7), 0).await?;
        insert(Status::Pending, at(8), 50).await?;
        insert(Status::Up, at(31), 400).await?;
        insert(Status::Maintenance, at(45), 0).await?; // a bucket with no checks

        let latency = Log::latency(&pool, 1, at(0), 600).await?;
        assert_eq!(
            latency.buckets,
            [
                LatencyBucket {
                    start: at(0),
                    avg: Some(150.0),
                    min: Some(100),
                    max: Some(200),
                    up: 2,
                    down: 1,
                },
                LatencyBucket {
                    start: at(30),
                    avg: Some(400.0),
                    min: Some(400),
                    max: Some(400),
                    up: 1,
                    down: 0,
                },
            ]
        );
        assert_eq!(latency.avg, Some(700.0 / 3.0), "weighted by up checks");
        assert_eq!((latency.min, latency.max), (Some(100), Some(400)));
        Ok(())
    }

    #[sqlx::test(fixtures("users", "services"))]
    async fn latency_of_only_down_checks(pool: SqlitePool) -> sqlx::Result<()> {
        insert_at(&pool, Status::Down, 5, 0).await;

        let latency = Log::latency(&pool, 1, Utc::now() - chrono::TimeDelta::hours(1), 60).await?;
        assert_eq!(latency.buckets.len(), 1);
        assert_eq!(latency.buckets[0].avg, None);
        assert_eq!(latency.buckets[0].down, 1);
        assert_eq!((latency.avg, latency.min, latency.max), (None, None, None));
        Ok(())
    }
}
