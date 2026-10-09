use argon2::password_hash::rand_core::{OsRng, RngCore};
use checks::CheckConfig;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool, types::Json};

use crate::{build_query_bind, build_update_query};

use super::log::Status;

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Service {
    pub id: u32,
    pub user_id: u32,
    pub active: bool,
    pub name: String,
    pub interval: u32,
    pub timeout: u32,
    pub last_status: Status,
    pub retry: u32,
    pub retry_interval: u32,
    pub invert: bool,
    pub consecutive_failures: u32,
    pub next_run_at: i64,
    /// When a push monitor last received a heartbeat (unix seconds).
    pub last_push_at: Option<i64>,
    #[sqlx(json)]
    pub config: CheckConfig,
    /// The config's `type`, derived by the database.
    pub service_type: String,
    /// The config's URL or host, derived by the database. Push monitors have none.
    pub target: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServiceForCreate {
    #[serde(skip)]
    pub user_id: Option<u32>,
    pub active: Option<bool>,
    pub name: String,
    pub interval: u16,
    pub timeout: Option<u16>,
    pub retry: u16,
    pub retry_interval: u16,
    pub invert: Option<bool>,
    pub config: CheckConfig,
}

#[derive(Debug, Default, Deserialize)]
pub struct ServiceForUpdate {
    pub active: Option<bool>,
    pub name: Option<String>,
    pub interval: Option<u16>,
    pub timeout: Option<u16>,
    #[serde(skip)]
    pub last_status: Option<Status>,
    pub retry: Option<u16>,
    pub retry_interval: Option<u16>,
    pub invert: Option<bool>,
    /// Replaces the whole check config.
    pub config: Option<Json<CheckConfig>>,
    #[serde(skip)]
    pub last_push_at: Option<i64>,
}

/// Give a push config without a token a random one.
pub fn fill_push_token(config: &mut CheckConfig) {
    if let CheckConfig::Push(push) = config
        && push.token.trim().is_empty()
    {
        let mut bytes = [0u8; 16];
        OsRng.fill_bytes(&mut bytes);
        push.token = bytes.iter().map(|b| format!("{b:02x}")).collect();
    }
}

/// A new push monitor counts from when it's set up, so it isn't reported as
/// missing a heartbeat before it could have sent one.
fn initial_push_at(config: &CheckConfig) -> Option<i64> {
    config.is_passive().then(|| Utc::now().timestamp())
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
        validate_fields(Some(&self.name), Some(self.interval), self.timeout)?;
        self.config.validate()
    }
}

impl ServiceForUpdate {
    pub fn validate(&self) -> Result<(), String> {
        validate_fields(self.name.as_deref(), self.interval, self.timeout)?;
        match &self.config {
            Some(Json(config)) => config.validate(),
            None => Ok(()),
        }
    }
}

fn validate_fields(
    name: Option<&str>,
    interval: Option<u16>,
    timeout: Option<u16>,
) -> Result<(), String> {
    if name.is_some_and(|n| n.trim().is_empty()) {
        return Err("name must not be empty".into());
    }
    if interval == Some(0) {
        return Err("interval must be at least 1 second".into());
    }
    if timeout == Some(0) {
        return Err("timeout must be at least 1 second".into());
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
                   user_id, active, name, interval, timeout, retry, retry_interval, invert,
                   last_push_at, config
               ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(service.user_id)
        .bind(service.active.unwrap_or(true))
        .bind(service.name)
        .bind(service.interval)
        .bind(service.timeout.unwrap_or(10))
        .bind(service.retry)
        .bind(service.retry_interval)
        .bind(service.invert.unwrap_or(false))
        .bind(initial_push_at(&service.config))
        .bind(Json(service.config))
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

    pub async fn find_by_push_token(
        pool: &SqlitePool,
        token: &str,
    ) -> sqlx::Result<Option<Service>> {
        sqlx::query_as::<_, Service>(
            r#"SELECT *
               FROM Services
               WHERE service_type = 'push' AND json_extract(config, '$.token') = ?"#,
        )
        .bind(token)
        .fetch_optional(pool)
        .await
    }

    /// Record a heartbeat and move the next check to the new deadline.
    pub async fn record_push(
        pool: &SqlitePool,
        service_id: u32,
        pushed_at: i64,
        next_run_at: i64,
    ) -> sqlx::Result<()> {
        sqlx::query("UPDATE Services SET last_push_at = ?, next_run_at = ? WHERE id = ?")
            .bind(pushed_at)
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
        mut update_data: ServiceForUpdate,
    ) -> sqlx::Result<u64> {
        if let Some(Json(config)) = &update_data.config {
            update_data.last_push_at = initial_push_at(config);
        }
        let mut query = String::from("UPDATE Services SET ");
        let mut has_updates = false;

        build_update_query!(query, has_updates, update_data, {
            active,
            name,
            interval,
            timeout,
            last_status,
            retry,
            retry_interval,
            invert,
            config,
            last_push_at
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
            timeout,
            last_status,
            retry,
            retry_interval,
            invert,
            config,
            last_push_at
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
    use checks::{HttpMethod, PingConfig, PushConfig};
    use serde_json::json;
    use sqlx::SqlitePool;

    use super::*;

    fn http(url: &str) -> CheckConfig {
        serde_json::from_value(json!({ "type": "http", "url": url })).unwrap()
    }

    fn new_service(name: &str, config: CheckConfig) -> ServiceForCreate {
        ServiceForCreate {
            user_id: Some(1),
            active: None,
            name: name.into(),
            interval: 60,
            timeout: None,
            retry: 1,
            retry_interval: 30,
            invert: None,
            config,
        }
    }

    #[sqlx::test(fixtures("users"))]
    async fn insert_service_with_defaults(pool: SqlitePool) -> sqlx::Result<()> {
        let count =
            Service::insert(&pool, new_service("Simple", http("https://example.com"))).await?;
        assert_eq!(count, 1);

        let service = Service::get(&pool, 1).await?.unwrap();
        assert_eq!(service.name, "Simple");
        assert!(service.active);
        assert!(!service.invert);
        assert_eq!(service.timeout, 10);
        assert_eq!(service.consecutive_failures, 0);
        assert_eq!(service.next_run_at, 0);
        assert_eq!(service.config, http("https://example.com"));

        Ok(())
    }

    #[sqlx::test(fixtures("users"))]
    async fn insert_persists_all_settings(pool: SqlitePool) -> sqlx::Result<()> {
        let config: CheckConfig = serde_json::from_value(json!({
            "type": "http",
            "url": "https://api.example.com",
            "method": "POST",
            "headers": { "Authorization": "Bearer x" },
            "body": "{}",
            "expected_code": 204,
            "expected_payload": "{\"ok\":true}",
        }))
        .unwrap();
        Service::insert(
            &pool,
            ServiceForCreate {
                active: Some(false),
                timeout: Some(7),
                invert: Some(true),
                ..new_service("API", config.clone())
            },
        )
        .await?;

        let service = Service::get(&pool, 1).await?.unwrap();
        assert!(!service.active);
        assert_eq!(service.timeout, 7);
        assert!(service.invert);
        assert_eq!(service.config, config);

        Ok(())
    }

    #[sqlx::test(fixtures("users"))]
    async fn type_and_target_follow_config(pool: SqlitePool) -> sqlx::Result<()> {
        Service::insert(
            &pool,
            new_service("web", http("https://example.com/health")),
        )
        .await?;
        Service::insert(
            &pool,
            new_service(
                "box",
                CheckConfig::Ping(PingConfig {
                    host: "10.0.0.5".into(),
                }),
            ),
        )
        .await?;

        let web = Service::get(&pool, 1).await?.unwrap();
        assert_eq!(web.service_type, "http");
        assert_eq!(web.target.as_deref(), Some("https://example.com/health"));
        let host = Service::get(&pool, 2).await?.unwrap();
        assert_eq!(host.service_type, "ping");
        assert_eq!(host.target.as_deref(), Some("10.0.0.5"));

        // Replacing the config updates the derived columns.
        Service::update(
            &pool,
            1,
            ServiceForUpdate {
                config: Some(Json(CheckConfig::Ping(PingConfig {
                    host: "example.com".into(),
                }))),
                ..Default::default()
            },
        )
        .await?;
        let web = Service::get(&pool, 1).await?.unwrap();
        assert_eq!(web.service_type, "ping");
        assert_eq!(web.target.as_deref(), Some("example.com"));

        Ok(())
    }

    fn push(token: &str) -> CheckConfig {
        CheckConfig::Push(PushConfig {
            token: token.into(),
            grace_secs: 10,
        })
    }

    #[test]
    fn fill_push_token_only_when_missing() {
        let mut config = push("");
        fill_push_token(&mut config);
        let CheckConfig::Push(generated) = &config else {
            unreachable!()
        };
        assert_eq!(generated.token.len(), 32);
        assert!(config.validate().is_ok());

        let mut other = push("");
        fill_push_token(&mut other);
        assert_ne!(config, other, "tokens are random");

        let mut chosen = push("my-own-token-1234");
        fill_push_token(&mut chosen);
        assert_eq!(chosen, push("my-own-token-1234"));
    }

    #[sqlx::test(fixtures("users"))]
    async fn push_monitors_track_heartbeats(pool: SqlitePool) -> sqlx::Result<()> {
        let before = Utc::now().timestamp();
        Service::insert(&pool, new_service("cron", push("cron-token-123456"))).await?;
        Service::insert(&pool, new_service("web", http("https://example.com"))).await?;

        let cron = Service::find_by_push_token(&pool, "cron-token-123456")
            .await?
            .unwrap();
        assert_eq!(cron.service_type, "push");
        assert_eq!(cron.target, None);
        assert!(
            cron.last_push_at.unwrap() >= before,
            "set up counts as a heartbeat"
        );
        assert!(
            Service::get(&pool, 2)
                .await?
                .unwrap()
                .last_push_at
                .is_none()
        );
        assert!(Service::find_by_push_token(&pool, "nope").await?.is_none());

        Service::record_push(&pool, cron.id, 5_000, 5_070).await?;
        let cron = Service::get(&pool, cron.id).await?.unwrap();
        assert_eq!(cron.last_push_at, Some(5_000));
        assert_eq!(cron.next_run_at, 5_070);

        // Switching an existing service to push starts its heartbeat clock.
        Service::update(
            &pool,
            2,
            ServiceForUpdate {
                config: Some(Json(push("web-token-12345678"))),
                ..Default::default()
            },
        )
        .await?;
        assert!(Service::get(&pool, 2).await?.unwrap().last_push_at.unwrap() >= before);

        Ok(())
    }

    #[sqlx::test(fixtures("users", "services"))]
    async fn update_settings_and_config(pool: SqlitePool) -> sqlx::Result<()> {
        let updated = Service::update(
            &pool,
            1,
            ServiceForUpdate {
                name: Some("Renamed".into()),
                timeout: Some(3),
                config: Some(Json(http("https://new.example.com"))),
                ..Default::default()
            },
        )
        .await?;
        assert_eq!(updated, 1);

        let service = Service::get(&pool, 1).await?.unwrap();
        assert_eq!(service.name, "Renamed");
        assert_eq!(service.timeout, 3);
        assert_eq!(service.config, http("https://new.example.com"));
        assert_eq!(service.interval, 5, "untouched fields are kept");

        assert_eq!(
            Service::update(&pool, 1, ServiceForUpdate::default()).await?,
            0
        );
        Ok(())
    }

    #[sqlx::test(fixtures("users", "services"))]
    async fn get_service(pool: SqlitePool) -> sqlx::Result<()> {
        let service = Service::get(&pool, 1).await?.unwrap();
        assert_eq!(service.name, "Service One");
        assert_eq!(service.service_type, "http");

        assert!(Service::get(&pool, 999).await?.is_none());
        Ok(())
    }

    #[sqlx::test(fixtures("users", "services"))]
    async fn get_stats(pool: SqlitePool) -> sqlx::Result<()> {
        let stats = Service::get_stats(&pool).await?;

        assert_eq!(stats.count, 5);
        assert_eq!(stats.active, 4);
        assert_eq!(stats.up, 1);
        assert_eq!(stats.down, 1);

        Ok(())
    }

    #[sqlx::test(fixtures("users", "services"))]
    async fn list_all_services(pool: SqlitePool) -> sqlx::Result<()> {
        assert_eq!(Service::all(&pool).await?.len(), 5);
        Ok(())
    }

    #[sqlx::test(fixtures("users", "services"))]
    async fn due_returns_only_active(pool: SqlitePool) -> sqlx::Result<()> {
        let services = Service::due(&pool, i64::MAX).await?;
        assert_eq!(services.len(), 4);
        assert!(services.iter().all(|s| s.active));
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

    #[sqlx::test(migrations = false)]
    async fn migrates_legacy_services_to_config(pool: SqlitePool) -> sqlx::Result<()> {
        const CONFIG_MIGRATION: i64 = 20251009130000;
        // One connection throughout, so no pooled connection holds the old schema.
        let mut conn = pool.acquire().await?;
        let migrations = sqlx::migrate!();
        for migration in migrations.iter().filter(|m| m.version < CONFIG_MIGRATION) {
            sqlx::raw_sql(&migration.sql).execute(&mut *conn).await?;
        }

        sqlx::raw_sql(
            r#"INSERT INTO Users (username, password) VALUES ('admin', 'x');
               INSERT INTO Services (user_id, name, interval, url, payload, service_type,
                                     retry_interval, expected_code, expected_payload) VALUES
               (1, 'get', 60, 'https://a.example.com', NULL, 'http', 30, 2, ''),
               (1, 'post', 60, 'https://b.example.com', '{"q":1}', 'http', 30, 404, '{"ok":true}'),
               (1, 'host', 60, '10.0.0.1', NULL, 'ping', 30, NULL, NULL);"#,
        )
        .execute(&mut *conn)
        .await?;

        for migration in migrations.iter().filter(|m| m.version >= CONFIG_MIGRATION) {
            sqlx::raw_sql(&migration.sql).execute(&mut *conn).await?;
        }

        let services = sqlx::query_as::<_, Service>("SELECT * FROM Services ORDER BY id")
            .fetch_all(&mut *conn)
            .await?;
        assert_eq!(services.len(), 3);
        assert!(services.iter().all(|s| s.retry_interval == 30));

        let CheckConfig::Http(get) = &services[0].config else {
            panic!("expected http config");
        };
        assert_eq!(get.url, "https://a.example.com");
        assert_eq!(get.method, HttpMethod::Get);
        assert_eq!(get.body, None);
        assert_eq!(get.expected_code, Some(2));
        assert_eq!(get.expected_payload, None, "empty template is dropped");

        let CheckConfig::Http(post) = &services[1].config else {
            panic!("expected http config");
        };
        assert_eq!(post.method, HttpMethod::Post);
        assert_eq!(post.body.as_deref(), Some(r#"{"q":1}"#));
        assert_eq!(post.expected_code, Some(404));
        assert_eq!(post.expected_payload.as_deref(), Some(r#"{"ok":true}"#));

        assert_eq!(
            services[2].config,
            CheckConfig::Ping(PingConfig {
                host: "10.0.0.1".into()
            })
        );
        assert_eq!(services[2].service_type, "ping");
        assert_eq!(services[2].target.as_deref(), Some("10.0.0.1"));

        Ok(())
    }

    #[test]
    fn validate_rejects_bad_settings() {
        let valid = new_service("svc", http("https://example.com"));
        assert!(valid.validate().is_ok());

        let invalid = [
            ServiceForCreate {
                interval: 0,
                ..valid.clone()
            },
            ServiceForCreate {
                timeout: Some(0),
                ..valid.clone()
            },
            ServiceForCreate {
                name: " ".into(),
                ..valid.clone()
            },
            ServiceForCreate {
                config: http("not a url"),
                ..valid.clone()
            },
        ];
        for service in invalid {
            assert!(service.validate().is_err(), "{service:?}");
        }

        let partial = ServiceForUpdate {
            interval: Some(0),
            ..Default::default()
        };
        assert!(partial.validate().is_err());
        let bad_config = ServiceForUpdate {
            config: Some(Json(CheckConfig::Ping(PingConfig { host: "".into() }))),
            ..Default::default()
        };
        assert!(bad_config.validate().is_err());
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
