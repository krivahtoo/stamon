use alerts::ChannelConfig;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, QueryBuilder, Sqlite, SqlitePool, types::Json};

use crate::{build_query_bind, build_update_query};

/// A place alerts are sent. Its config holds secrets, so only admins see it.
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Channel {
    pub id: u32,
    pub user_id: u32,
    pub name: String,
    pub active: bool,
    pub apply_to_all: bool,
    #[sqlx(json)]
    pub config: ChannelConfig,
    /// The config's `type`, derived by the database.
    pub channel_type: String,
}

/// What everyone may see about a channel.
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct ChannelSummary {
    pub id: u32,
    pub name: String,
    pub active: bool,
    pub apply_to_all: bool,
    pub channel_type: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChannelForCreate {
    #[serde(skip)]
    pub user_id: Option<u32>,
    pub name: String,
    pub active: Option<bool>,
    pub apply_to_all: Option<bool>,
    pub config: ChannelConfig,
}

#[derive(Debug, Default, Deserialize)]
pub struct ChannelForUpdate {
    pub name: Option<String>,
    pub active: Option<bool>,
    pub apply_to_all: Option<bool>,
    /// Replaces the whole channel config.
    pub config: Option<Json<ChannelConfig>>,
}

fn validate_name(name: Option<&str>) -> Result<(), String> {
    if name.is_some_and(|n| n.trim().is_empty()) {
        return Err("name must not be empty".into());
    }
    Ok(())
}

impl ChannelForCreate {
    pub fn validate(&self) -> Result<(), String> {
        validate_name(Some(&self.name))?;
        self.config.validate()
    }
}

impl ChannelForUpdate {
    pub fn validate(&self) -> Result<(), String> {
        validate_name(self.name.as_deref())?;
        match &self.config {
            Some(Json(config)) => config.validate(),
            None => Ok(()),
        }
    }
}

impl Channel {
    pub async fn insert(pool: &SqlitePool, channel: ChannelForCreate) -> sqlx::Result<u32> {
        let result = sqlx::query(
            r#"INSERT INTO NotificationChannels (user_id, name, active, apply_to_all, config)
               VALUES (?, ?, ?, ?, ?)"#,
        )
        .bind(channel.user_id)
        .bind(channel.name)
        .bind(channel.active.unwrap_or(true))
        .bind(channel.apply_to_all.unwrap_or(false))
        .bind(Json(channel.config))
        .execute(pool)
        .await?;
        Ok(result.last_insert_rowid() as u32)
    }

    pub async fn get(pool: &SqlitePool, channel_id: u32) -> sqlx::Result<Option<Channel>> {
        sqlx::query_as("SELECT * FROM NotificationChannels WHERE id = ?")
            .bind(channel_id)
            .fetch_optional(pool)
            .await
    }

    pub async fn summaries(pool: &SqlitePool) -> sqlx::Result<Vec<ChannelSummary>> {
        sqlx::query_as(
            r#"SELECT id, name, active, apply_to_all, channel_type
               FROM NotificationChannels
               ORDER BY name"#,
        )
        .fetch_all(pool)
        .await
    }

    pub async fn update(
        pool: &SqlitePool,
        channel_id: u32,
        update_data: ChannelForUpdate,
    ) -> sqlx::Result<u64> {
        let mut query = String::from("UPDATE NotificationChannels SET ");
        let mut has_updates = false;

        build_update_query!(query, has_updates, update_data, {
            name,
            active,
            apply_to_all,
            config
        });

        if !has_updates {
            return Ok(0);
        }
        query.truncate(query.len() - 2);
        query.push_str(" WHERE id = ?");

        let mut query_builder = sqlx::query(&query);
        build_query_bind!(query_builder, update_data, {
            name,
            active,
            apply_to_all,
            config
        });
        let result = query_builder.bind(channel_id).execute(pool).await?;
        Ok(result.rows_affected())
    }

    pub async fn delete(pool: &SqlitePool, channel_id: u32) -> sqlx::Result<u64> {
        let result = sqlx::query("DELETE FROM NotificationChannels WHERE id = ?")
            .bind(channel_id)
            .execute(pool)
            .await?;
        Ok(result.rows_affected())
    }

    /// Active channels that receive a service's alerts.
    pub async fn for_service(pool: &SqlitePool, service_id: u32) -> sqlx::Result<Vec<Channel>> {
        sqlx::query_as(
            r#"SELECT *
               FROM NotificationChannels
               WHERE active = true
                 AND (apply_to_all = true
                      OR id IN (SELECT channel_id FROM ServiceChannels WHERE service_id = ?))"#,
        )
        .bind(service_id)
        .fetch_all(pool)
        .await
    }

    /// Channels linked to a service, not counting ones that apply to all.
    pub async fn linked_ids(pool: &SqlitePool, service_id: u32) -> sqlx::Result<Vec<u32>> {
        sqlx::query_scalar(
            "SELECT channel_id FROM ServiceChannels WHERE service_id = ? ORDER BY channel_id",
        )
        .bind(service_id)
        .fetch_all(pool)
        .await
    }

    /// The ids in `channel_ids` that don't belong to a channel.
    pub async fn missing_ids(pool: &SqlitePool, channel_ids: &[u32]) -> sqlx::Result<Vec<u32>> {
        if channel_ids.is_empty() {
            return Ok(vec![]);
        }
        let mut query =
            QueryBuilder::<Sqlite>::new("SELECT id FROM NotificationChannels WHERE id IN (");
        let mut ids = query.separated(", ");
        for id in channel_ids {
            ids.push_bind(id);
        }
        query.push(")");
        let found: Vec<u32> = query.build_query_scalar().fetch_all(pool).await?;
        Ok(channel_ids
            .iter()
            .copied()
            .filter(|id| !found.contains(id))
            .collect())
    }

    /// Replace the channels linked to a service.
    pub async fn set_for_service(
        pool: &SqlitePool,
        service_id: u32,
        channel_ids: &[u32],
    ) -> sqlx::Result<()> {
        let mut tx = pool.begin().await?;
        sqlx::query("DELETE FROM ServiceChannels WHERE service_id = ?")
            .bind(service_id)
            .execute(&mut *tx)
            .await?;
        for channel_id in channel_ids {
            sqlx::query(
                "INSERT OR IGNORE INTO ServiceChannels (service_id, channel_id) VALUES (?, ?)",
            )
            .bind(service_id)
            .bind(channel_id)
            .execute(&mut *tx)
            .await?;
        }
        tx.commit().await
    }
}

#[cfg(test)]
mod tests {
    use alerts::{ChatWebhookConfig, NtfyConfig};

    use super::*;

    fn ntfy(topic: &str) -> ChannelConfig {
        ChannelConfig::Ntfy(NtfyConfig {
            server_url: "https://ntfy.sh".into(),
            topic: topic.into(),
            access_token: None,
        })
    }

    fn new_channel(name: &str, apply_to_all: bool) -> ChannelForCreate {
        ChannelForCreate {
            user_id: Some(1),
            name: name.into(),
            active: None,
            apply_to_all: Some(apply_to_all),
            config: ntfy("stamon"),
        }
    }

    #[sqlx::test(fixtures(path = "fixtures", scripts("users")))]
    async fn insert_get_update_delete(pool: SqlitePool) -> sqlx::Result<()> {
        let id = Channel::insert(&pool, new_channel("Ops ntfy", false)).await?;
        let channel = Channel::get(&pool, id).await?.unwrap();
        assert_eq!(channel.name, "Ops ntfy");
        assert!(channel.active);
        assert_eq!(channel.channel_type, "ntfy");
        assert_eq!(channel.config, ntfy("stamon"));

        let slack = ChannelConfig::Slack(ChatWebhookConfig {
            webhook_url: "https://hooks.slack.com/services/x".into(),
        });
        let updated = Channel::update(
            &pool,
            id,
            ChannelForUpdate {
                active: Some(false),
                config: Some(Json(slack.clone())),
                ..Default::default()
            },
        )
        .await?;
        assert_eq!(updated, 1);
        let channel = Channel::get(&pool, id).await?.unwrap();
        assert!(!channel.active);
        assert_eq!(channel.channel_type, "slack");
        assert_eq!(channel.config, slack);
        assert_eq!(channel.name, "Ops ntfy");

        let summaries = Channel::summaries(&pool).await?;
        assert_eq!(summaries.len(), 1);
        assert_eq!(summaries[0].channel_type, "slack");

        assert_eq!(Channel::delete(&pool, id).await?, 1);
        assert!(Channel::get(&pool, id).await?.is_none());
        Ok(())
    }

    #[sqlx::test(fixtures(path = "fixtures", scripts("users", "services")))]
    async fn service_channels(pool: SqlitePool) -> sqlx::Result<()> {
        let linked = Channel::insert(&pool, new_channel("linked", false)).await?;
        let other = Channel::insert(&pool, new_channel("other", false)).await?;
        let everywhere = Channel::insert(&pool, new_channel("everywhere", true)).await?;
        let paused = Channel::insert(
            &pool,
            ChannelForCreate {
                active: Some(false),
                ..new_channel("paused", true)
            },
        )
        .await?;

        Channel::set_for_service(&pool, 1, &[linked, linked]).await?;
        assert_eq!(Channel::linked_ids(&pool, 1).await?, [linked]);

        let ids = |channels: Vec<Channel>| {
            let mut ids: Vec<u32> = channels.iter().map(|c| c.id).collect();
            ids.sort();
            ids
        };
        assert_eq!(
            ids(Channel::for_service(&pool, 1).await?),
            [linked, everywhere]
        );
        assert_eq!(ids(Channel::for_service(&pool, 2).await?), [everywhere]);
        assert!(!ids(Channel::for_service(&pool, 1).await?).contains(&paused));

        // Replacing the links drops the old ones.
        Channel::set_for_service(&pool, 1, &[other]).await?;
        assert_eq!(Channel::linked_ids(&pool, 1).await?, [other]);

        // Deleting a channel removes its links.
        Channel::delete(&pool, other).await?;
        assert!(Channel::linked_ids(&pool, 1).await?.is_empty());

        assert_eq!(Channel::missing_ids(&pool, &[linked, 999]).await?, [999]);
        assert!(Channel::missing_ids(&pool, &[]).await?.is_empty());
        Ok(())
    }

    #[test]
    fn validation() {
        assert!(new_channel("ok", false).validate().is_ok());
        assert!(
            ChannelForCreate {
                name: " ".into(),
                ..new_channel("x", false)
            }
            .validate()
            .is_err()
        );
        assert!(
            ChannelForCreate {
                config: ntfy("bad/topic"),
                ..new_channel("x", false)
            }
            .validate()
            .is_err()
        );
        assert!(
            ChannelForUpdate {
                config: Some(Json(ntfy(""))),
                ..Default::default()
            }
            .validate()
            .is_err()
        );
    }
}
