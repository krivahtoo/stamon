use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::{FromRow, SqlitePool, Type};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Type, Serialize)]
#[sqlx(rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum DeliveryStatus {
    Sent,
    Failed,
}

/// An alert that was sent, or that couldn't be.
#[derive(Debug, FromRow, Serialize)]
pub struct Notification {
    pub id: u32,
    /// `None` for test alerts.
    pub service_id: Option<u32>,
    pub service_name: Option<String>,
    /// `None` once the channel is deleted.
    pub channel_id: Option<u32>,
    pub channel_name: Option<String>,
    pub channel_type: Option<String>,
    pub title: String,
    pub message: Option<String>,
    pub status: DeliveryStatus,
    pub error: Option<String>,
    pub sent_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct NotificationForCreate {
    pub service_id: Option<u32>,
    pub channel_id: Option<u32>,
    pub title: String,
    pub message: Option<String>,
    pub status: DeliveryStatus,
    pub error: Option<String>,
}

impl Notification {
    pub async fn insert(
        pool: &SqlitePool,
        notification: NotificationForCreate,
    ) -> sqlx::Result<()> {
        sqlx::query(
            r#"INSERT INTO Notifications (service_id, channel_id, title, message, status, error)
               VALUES (?, ?, ?, ?, ?, ?)"#,
        )
        .bind(notification.service_id)
        .bind(notification.channel_id)
        .bind(notification.title)
        .bind(notification.message)
        .bind(notification.status)
        .bind(notification.error)
        .execute(pool)
        .await?;
        Ok(())
    }

    /// Delete deliveries older than `before`, returning how many were removed.
    pub async fn prune(pool: &SqlitePool, before: DateTime<Utc>) -> sqlx::Result<u64> {
        let result = sqlx::query("DELETE FROM Notifications WHERE datetime(sent_at) < datetime(?)")
            .bind(before)
            .execute(pool)
            .await?;
        Ok(result.rows_affected())
    }

    /// The most recent deliveries first.
    pub async fn list(pool: &SqlitePool, limit: Option<u32>) -> sqlx::Result<Vec<Notification>> {
        sqlx::query_as(
            r#"SELECT n.id, n.service_id, s.name AS service_name,
                      n.channel_id, c.name AS channel_name, c.channel_type,
                      n.title, n.message, n.status, n.error, n.sent_at
               FROM Notifications n
               LEFT JOIN Services s ON s.id = n.service_id
               LEFT JOIN NotificationChannels c ON c.id = n.channel_id
               ORDER BY n.id DESC
               LIMIT ?"#,
        )
        .bind(limit.unwrap_or(50))
        .fetch_all(pool)
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test(fixtures(path = "fixtures", scripts("users", "services")))]
    async fn records_and_lists_deliveries(pool: SqlitePool) -> sqlx::Result<()> {
        sqlx::query(
            r#"INSERT INTO NotificationChannels (user_id, name, config)
               VALUES (1, 'Ops', '{"type":"ntfy","topic":"ops"}')"#,
        )
        .execute(&pool)
        .await?;

        Notification::insert(
            &pool,
            NotificationForCreate {
                service_id: Some(1),
                channel_id: Some(1),
                title: "Service One is down".into(),
                message: Some("Unexpected status code: 500".into()),
                status: DeliveryStatus::Sent,
                error: None,
            },
        )
        .await?;
        Notification::insert(
            &pool,
            NotificationForCreate {
                service_id: None,
                channel_id: Some(1),
                title: "Test alert".into(),
                message: None,
                status: DeliveryStatus::Failed,
                error: Some("ntfy returned 403".into()),
            },
        )
        .await?;

        let history = Notification::list(&pool, None).await?;
        assert_eq!(history.len(), 2);
        assert_eq!(history[0].title, "Test alert", "newest first");
        assert_eq!(history[0].status, DeliveryStatus::Failed);
        assert_eq!(history[0].service_name, None);
        assert_eq!(history[1].service_name.as_deref(), Some("Service One"));
        assert_eq!(history[1].channel_name.as_deref(), Some("Ops"));
        assert_eq!(history[1].channel_type.as_deref(), Some("ntfy"));

        // Deleting the channel keeps the history.
        sqlx::query("DELETE FROM NotificationChannels WHERE id = 1")
            .execute(&pool)
            .await?;
        let history = Notification::list(&pool, Some(1)).await?;
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].channel_id, None);
        Ok(())
    }

    #[sqlx::test]
    async fn prune_removes_old_deliveries(pool: SqlitePool) -> sqlx::Result<()> {
        sqlx::query(
            r#"INSERT INTO Notifications (title, status, sent_at) VALUES
               ('old', 'sent', datetime('now', '-100 days')),
               ('new', 'sent', datetime('now'))"#,
        )
        .execute(&pool)
        .await?;
        let removed = Notification::prune(&pool, Utc::now() - chrono::TimeDelta::days(90)).await?;
        assert_eq!(removed, 1);
        let left = Notification::list(&pool, None).await?;
        assert_eq!(left.len(), 1);
        assert_eq!(left[0].title, "new");
        Ok(())
    }
}
