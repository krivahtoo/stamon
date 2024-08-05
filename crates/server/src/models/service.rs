use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool, Type};

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
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ServiceForCreate {
    pub user_id: u32,
    pub active: Option<bool>,
    pub name: String,
    pub interval: u16,
    pub url: String,
    pub payload: Option<String>,
    pub service_type: ServiceType,
    pub retry: u16,
    pub retry_interval: u16,
}

impl Service {
    pub async fn insert(pool: &SqlitePool, service: ServiceForCreate) -> sqlx::Result<u64> {
        // Default active to true if not provided
        let active = service.active.unwrap_or(true);

        // Construct the base query
        let mut query = "INSERT INTO services (user_id, active, name, interval, url, service_type, retry, retry_interval".to_string();
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

    pub async fn all_active(pool: &SqlitePool) -> sqlx::Result<Vec<Service>> {
        let services = sqlx::query_as::<_, Service>(
            r#"SELECT *
            FROM services
            WHERE active = true
            "#,
        )
        .fetch_all(pool)
        .await?;

        Ok(services)
    }

    pub async fn all(pool: &SqlitePool) -> sqlx::Result<Vec<Service>> {
        let services = sqlx::query_as::<_, Service>(r#"SELECT * FROM services"#)
            .fetch_all(pool)
            .await?;

        Ok(services)
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
                user_id: 1,
                active: Some(false),
                name: "foo".into(),
                interval: 0,
                url: "https://example.com".into(),
                payload: None,
                service_type: ServiceType::Ping,
                retry: 0,
                retry_interval: 0,
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
}
