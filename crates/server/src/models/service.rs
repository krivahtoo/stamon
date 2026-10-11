use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool, Type};

use crate::{build_query_bind, build_update_query};

use super::log::Status;

#[derive(Debug, Clone, Type, Default, Serialize, Deserialize)]
#[sqlx(rename_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum ServiceType {
    #[default]
    Ping,
    Http,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Service {
    pub id: u32,
    pub user_id: u32,
    pub active: bool,
    pub name: String,
    pub interval: u32,
    pub url: String,
    pub timeout: u32,
    pub payload: Option<String>,
    pub last_status: Status,
    pub service_type: ServiceType,
    pub retry: u32,
    pub retry_interval: u32,
    pub invert: bool,
    pub expected_code: Option<u16>,
    pub expected_payload: Option<String>,
    #[serde(default)]
    pub consecutive_failures: u32,
    #[serde(default)]
    pub next_run_at: i64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ServiceForCreate {
    #[serde(skip)]
    pub user_id: Option<u32>,
    pub active: Option<bool>,
    pub name: String,
    pub interval: u16,
    pub url: String,
    pub timeout: Option<u16>,
    pub payload: Option<String>,
    pub service_type: ServiceType,
    pub retry: u16,
    pub retry_interval: u16,
    pub invert: Option<bool>,
    pub expected_code: Option<u16>,
    pub expected_payload: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ServiceForUpdate {
    pub active: Option<bool>,
    pub name: Option<String>,
    pub interval: Option<u16>,
    pub url: Option<String>,
    pub timeout: Option<u16>,
    pub payload: Option<String>,
    #[serde(skip)]
    pub last_status: Option<Status>,
    pub service_type: Option<ServiceType>,
    pub retry: Option<u16>,
    pub retry_interval: Option<u16>,
    pub invert: Option<bool>,
    pub expected_code: Option<u16>,
    pub expected_payload: Option<String>,
}

#[derive(Debug, Default, Serialize)]
pub struct Stats {
    count: u32,
    active: u32,
    up: u32,
    down: u32,
    failed: u32,
}

impl ServiceForCreate {
    pub fn validate(&self) -> Result<(), String> {
        validate_fields(
            Some(&self.name),
            Some(&self.url),
            Some(self.interval),
            self.timeout,
            self.expected_code,
        )
    }
}

impl ServiceForUpdate {
    pub fn validate(&self) -> Result<(), String> {
        validate_fields(
            self.name.as_deref(),
            self.url.as_deref(),
            self.interval,
            self.timeout,
            self.expected_code,
        )
    }
}

fn validate_fields(
    name: Option<&str>,
    url: Option<&str>,
    interval: Option<u16>,
    timeout: Option<u16>,
    expected_code: Option<u16>,
) -> Result<(), String> {
    if name.is_some_and(|n| n.trim().is_empty()) {
        return Err("name must not be empty".into());
    }
    if url.is_some_and(|u| u.trim().is_empty()) {
        return Err("url must not be empty".into());
    }
    if interval == Some(0) {
        return Err("interval must be at least 1 second".into());
    }
    if timeout == Some(0) {
        return Err("timeout must be at least 1 second".into());
    }
    if let Some(code) = expected_code
        && !matches!(code, 0..=5 | 100..=599)
    {
        return Err("expected_code must be a status class (1-5) or a status code (100-599)".into());
    }
    Ok(())
}

impl Service {
    pub async fn get(pool: &SqlitePool, service_id: u32) -> sqlx::Result<Option<Service>> {
        let service = sqlx::query_as::<_, Service>(
            r#"SELECT *
               FROM Services
               WHERE id = ?"#,
        )
        .bind(service_id)
        .fetch_optional(pool)
        .await?;

        Ok(service)
    }

    pub async fn insert(pool: &SqlitePool, service: ServiceForCreate) -> sqlx::Result<u64> {
        let result = sqlx::query(
            r#"INSERT INTO Services (
                   user_id, active, name, interval, url, timeout, payload, service_type,
                   retry, retry_interval, invert, expected_code, expected_payload
               ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(service.user_id)
        .bind(service.active.unwrap_or(true))
        .bind(service.name)
        .bind(service.interval)
        .bind(service.url)
        .bind(service.timeout.unwrap_or(10))
        .bind(service.payload)
        .bind(service.service_type)
        .bind(service.retry)
        .bind(service.retry_interval)
        .bind(service.invert.unwrap_or(false))
        .bind(service.expected_code)
        .bind(service.expected_payload)
        .execute(pool)
        .await?;
        Ok(result.rows_affected())
    }

    pub async fn delete(pool: &SqlitePool, service_id: u32) -> sqlx::Result<u64> {
        let result = sqlx::query("DELETE FROM Services WHERE id = ?")
            .bind(service_id)
            .execute(pool)
            .await?;
        Ok(result.rows_affected())
    }

    /// Active services whose next check is due at `now` (unix seconds).
    pub async fn due(pool: &SqlitePool, now: i64) -> sqlx::Result<Vec<Service>> {
        sqlx::query_as::<_, Service>(
            r#"SELECT *
               FROM Services
               WHERE active = true AND next_run_at <= ?"#,
        )
        .bind(now)
        .fetch_all(pool)
        .await
    }

    pub async fn set_next_run(
        pool: &SqlitePool,
        service_id: u32,
        next_run_at: i64,
    ) -> sqlx::Result<()> {
        sqlx::query("UPDATE Services SET next_run_at = ? WHERE id = ?")
            .bind(next_run_at)
            .bind(service_id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn set_consecutive_failures(
        pool: &SqlitePool,
        service_id: u32,
        failures: u32,
    ) -> sqlx::Result<()> {
        sqlx::query("UPDATE Services SET consecutive_failures = ? WHERE id = ?")
            .bind(failures)
            .bind(service_id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn get_stats(pool: &SqlitePool) -> sqlx::Result<Stats> {
        let (count,) = sqlx::query_as::<_, (u32,)>(r#"SELECT COUNT(*) FROM Services"#)
            .fetch_one(pool)
            .await?;

        let (active,) = sqlx::query_as::<_, (u32,)>(
            r#"SELECT COUNT(*)
                FROM Services
                WHERE active = true
                "#,
        )
        .fetch_one(pool)
        .await?;

        let (up,) = sqlx::query_as::<_, (u32,)>(
            r#"SELECT COUNT(*)
                FROM Services
                WHERE active = true AND last_status = 1
                "#,
        )
        .fetch_one(pool)
        .await?;

        let (down,) = sqlx::query_as::<_, (u32,)>(
            r#"SELECT COUNT(*)
                FROM Services
                WHERE active = true AND last_status = 2
                "#,
        )
        .fetch_one(pool)
        .await?;

        let (failed,) = sqlx::query_as::<_, (u32,)>(
            r#"SELECT COUNT(*)
                FROM Services
                WHERE active = true AND last_status = 3
                "#,
        )
        .fetch_one(pool)
        .await?;

        Ok(Stats {
            count,
            active,
            up,
            failed,
            down,
        })
    }

    pub async fn all(pool: &SqlitePool) -> sqlx::Result<Vec<Service>> {
        let services = sqlx::query_as::<_, Service>(r#"SELECT * FROM Services"#)
            .fetch_all(pool)
            .await?;

        Ok(services)
    }

    pub async fn update(
        pool: &SqlitePool,
        service_id: u32,
        update_data: ServiceForUpdate,
    ) -> sqlx::Result<u64> {
        let mut query = String::from("UPDATE Services SET ");
        let mut has_updates = false;

        build_update_query!(query, has_updates, update_data, {
            active,
            name,
            interval,
            url,
            timeout,
            payload,
            last_status,
            service_type,
            retry,
            retry_interval,
            invert,
            expected_code,
            expected_payload
        });

        // Remove the trailing comma and space
        if has_updates {
            query.truncate(query.len() - 2);
            query.push_str(" WHERE id = ?");
        } else {
            // No updates were provided
            return Ok(0);
        }

        let mut query_builder = sqlx::query(&query);

        build_query_bind!(query_builder, update_data, {
            active,
            name,
            interval,
            url,
            timeout,
            payload,
            last_status,
            service_type,
            retry,
            retry_interval,
            invert,
            expected_code,
            expected_payload
        });

        // bind to service_id
        query_builder = query_builder.bind(service_id);

        // Execute the query
        let result = query_builder.execute(pool).await?;
        Ok(result.rows_affected())
    }
}

#[cfg(test)]
mod tests {
    use sqlx::SqlitePool;

    use super::*;

    #[sqlx::test(fixtures("users"))]
    async fn insert_service(pool: SqlitePool) -> sqlx::Result<()> {
        let count = Service::insert(
            &pool,
            ServiceForCreate {
                user_id: Some(1),
                active: Some(false),
                name: "foo".into(),
                interval: 0,
                url: "https://example.com".into(),
                payload: None,
                service_type: ServiceType::Ping,
                retry: 0,
                retry_interval: 0,
                ..Default::default()
            },
        )
        .await?;

        assert_eq!(count, 1);

        Ok(())
    }

    #[sqlx::test(fixtures("users", "services"))]
    async fn list_services(pool: SqlitePool) -> sqlx::Result<()> {
        let services = Service::due(&pool, i64::MAX).await?;

        dbg!(&services);

        assert_eq!(services.len(), 4);

        Ok(())
    }

    #[sqlx::test(fixtures("users", "services"))]
    async fn get_service(pool: SqlitePool) -> sqlx::Result<()> {
        let service = Service::get(&pool, 1).await?;

        dbg!(&service);

        assert!(service.is_some());

        Ok(())
    }

    #[sqlx::test(fixtures("users", "services"))]
    async fn get_stats(pool: SqlitePool) -> sqlx::Result<()> {
        let stats = Service::get_stats(&pool).await?;

        dbg!(&stats);

        assert_eq!(stats.active, 4);

        Ok(())
    }

    #[sqlx::test(fixtures("users"))]
    async fn insert_service_with_payload(pool: SqlitePool) -> sqlx::Result<()> {
        let count = Service::insert(
            &pool,
            ServiceForCreate {
                user_id: Some(1),
                active: Some(true),
                name: "Test Service".into(),
                interval: 60,
                url: "https://test.example.com".into(),
                payload: Some("{\"test\": \"data\"}".into()),
                service_type: ServiceType::Http,
                retry: 3,
                retry_interval: 30,
                ..Default::default()
            },
        )
        .await?;

        assert_eq!(count, 1);

        // Verify the service was created with payload
        let service = Service::get(&pool, 1).await?;
        assert!(service.is_some());
        let service = service.unwrap();
        assert_eq!(service.name, "Test Service");
        assert_eq!(service.payload, Some("{\"test\": \"data\"}".into()));

        Ok(())
    }

    #[sqlx::test(fixtures("users"))]
    async fn insert_service_without_payload(pool: SqlitePool) -> sqlx::Result<()> {
        let count = Service::insert(
            &pool,
            ServiceForCreate {
                user_id: Some(1),
                active: None, // Should default to true
                name: "Simple Service".into(),
                interval: 120,
                url: "https://simple.example.com".into(),
                payload: None,
                service_type: ServiceType::Ping,
                retry: 1,
                retry_interval: 60,
                ..Default::default()
            },
        )
        .await?;

        assert_eq!(count, 1);

        // Verify the service was created without payload and with default active=true
        let service = Service::get(&pool, 1).await?;
        assert!(service.is_some());
        let service = service.unwrap();
        assert_eq!(service.name, "Simple Service");
        assert!(service.payload.is_none());
        assert!(service.active);

        Ok(())
    }

    #[sqlx::test(fixtures("users", "services"))]
    async fn get_nonexistent_service(pool: SqlitePool) -> sqlx::Result<()> {
        let service = Service::get(&pool, 999).await?;
        assert!(service.is_none());

        Ok(())
    }

    #[sqlx::test(fixtures("users", "services"))]
    async fn list_services_returns_only_active(pool: SqlitePool) -> sqlx::Result<()> {
        let services = Service::due(&pool, i64::MAX).await?;

        // Should only return active services (4 out of 5 in fixtures)
        assert_eq!(services.len(), 4);

        // Verify all returned services are active
        for service in services {
            assert!(service.active);
        }

        Ok(())
    }

    #[sqlx::test(fixtures("users"))]
    async fn insert_persists_all_check_settings(pool: SqlitePool) -> sqlx::Result<()> {
        Service::insert(
            &pool,
            ServiceForCreate {
                user_id: Some(1),
                name: "API".into(),
                interval: 60,
                url: "https://api.example.com".into(),
                timeout: Some(7),
                service_type: ServiceType::Http,
                invert: Some(true),
                expected_code: Some(204),
                expected_payload: Some(r#"{"ok":true}"#.into()),
                ..Default::default()
            },
        )
        .await?;

        let service = Service::get(&pool, 1).await?.unwrap();
        assert_eq!(service.timeout, 7);
        assert!(service.invert);
        assert_eq!(service.expected_code, Some(204));
        assert_eq!(service.expected_payload.as_deref(), Some(r#"{"ok":true}"#));
        assert_eq!(service.consecutive_failures, 0);
        assert_eq!(service.next_run_at, 0);

        Ok(())
    }

    #[sqlx::test(fixtures("users", "services", "logs"))]
    async fn delete_service_cascades_logs(pool: SqlitePool) -> sqlx::Result<()> {
        assert_eq!(Service::delete(&pool, 1).await?, 1);
        assert!(Service::get(&pool, 1).await?.is_none());

        let (logs,) = sqlx::query_as::<_, (u32,)>("SELECT COUNT(*) FROM Logs WHERE service_id = 1")
            .fetch_one(&pool)
            .await?;
        assert_eq!(logs, 0);

        assert_eq!(Service::delete(&pool, 1).await?, 0);
        Ok(())
    }

    #[sqlx::test(fixtures("users", "services"))]
    async fn due_respects_next_run_at(pool: SqlitePool) -> sqlx::Result<()> {
        // Every active service starts out due
        assert_eq!(Service::due(&pool, 1_000).await?.len(), 4);

        Service::set_next_run(&pool, 1, 1_060).await?;
        let due = Service::due(&pool, 1_000).await?;
        assert_eq!(due.len(), 3);
        assert!(due.iter().all(|s| s.id != 1));

        assert_eq!(Service::due(&pool, 1_060).await?.len(), 4);
        Ok(())
    }

    #[sqlx::test(fixtures("users", "services"))]
    async fn update_consecutive_failures(pool: SqlitePool) -> sqlx::Result<()> {
        Service::set_consecutive_failures(&pool, 1, 2).await?;
        assert_eq!(
            Service::get(&pool, 1).await?.unwrap().consecutive_failures,
            2
        );
        Ok(())
    }

    #[test]
    fn validate_rejects_bad_settings() {
        let valid = ServiceForCreate {
            name: "svc".into(),
            url: "https://example.com".into(),
            interval: 60,
            ..Default::default()
        };
        assert!(valid.validate().is_ok());

        let zero_interval = ServiceForCreate {
            interval: 0,
            ..valid.clone()
        };
        assert!(zero_interval.validate().is_err());

        let zero_timeout = ServiceForCreate {
            timeout: Some(0),
            ..valid.clone()
        };
        assert!(zero_timeout.validate().is_err());

        let blank_name = ServiceForCreate {
            name: " ".into(),
            ..valid.clone()
        };
        assert!(blank_name.validate().is_err());

        for code in [Some(0), Some(2), Some(404)] {
            let ok = ServiceForCreate {
                expected_code: code,
                ..valid.clone()
            };
            assert!(ok.validate().is_ok(), "{code:?}");
        }
        let bad_code = ServiceForCreate {
            expected_code: Some(42),
            ..valid.clone()
        };
        assert!(bad_code.validate().is_err());

        let partial = ServiceForUpdate {
            interval: Some(0),
            ..Default::default()
        };
        assert!(partial.validate().is_err());
        assert!(ServiceForUpdate::default().validate().is_ok());
    }

    #[sqlx::test]
    async fn get_stats_empty_database(pool: SqlitePool) -> sqlx::Result<()> {
        let stats = Service::get_stats(&pool).await?;

        assert_eq!(stats.active, 0);
        assert_eq!(stats.count, 0);
        assert_eq!(stats.up, 0);
        assert_eq!(stats.down, 0);
        assert_eq!(stats.failed, 0);

        Ok(())
    }
}
