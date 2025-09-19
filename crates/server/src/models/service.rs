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
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ServiceForCreate {
    #[serde(skip)]
    pub user_id: Option<u32>,
    pub active: Option<bool>,
    pub name: String,
    pub interval: u16,
    pub url: String,
    pub payload: Option<String>,
    pub service_type: ServiceType,
    pub retry: u16,
    pub retry_interval: u16,
    pub invert: Option<bool>,
    pub expected_code: Option<u32>,
    pub expected_payload: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ServiceForUpdate {
    pub active: Option<bool>,
    pub name: Option<String>,
    pub interval: Option<u16>,
    pub url: Option<String>,
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
        // Default active to true if not provided
        let active = service.active.unwrap_or(true);

        // Construct the base query
        let mut query = "INSERT INTO Services (user_id, active, name, interval, url, service_type, retry, retry_interval".to_string();
        if service.payload.is_some() {
            query.push_str(", payload");
        }
        query.push_str(") VALUES (?, ?, ?, ?, ?, ?, ?, ?");
        if service.payload.is_some() {
            query.push_str(", ?");
        }
        query.push(')');

        // Create a query builder and bind parameters
        let mut query_builder = sqlx::query(&query)
            .bind(service.user_id)
            .bind(active)
            .bind(service.name)
            .bind(service.interval)
            .bind(service.url)
            .bind(service.service_type)
            .bind(service.retry)
            .bind(service.retry_interval);

        if let Some(payload) = service.payload {
            query_builder = query_builder.bind(payload);
        }

        // Execute the query
        let result = query_builder.execute(pool).await?;
        Ok(result.rows_affected())
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

    pub async fn all_active(pool: &SqlitePool) -> sqlx::Result<Vec<Service>> {
        let services = sqlx::query_as::<_, Service>(
            r#"SELECT *
            FROM Services
            WHERE active = true
            "#,
        )
        .fetch_all(pool)
        .await?;

        Ok(services)
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
        let services = Service::all_active(&pool).await?;

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
        let services = Service::all_active(&pool).await?;

        // Should only return active services (4 out of 5 in fixtures)
        assert_eq!(services.len(), 4);

        // Verify all returned services are active
        for service in services {
            assert!(service.active);
        }

        Ok(())
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
