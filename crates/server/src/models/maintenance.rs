use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

/// Planned downtime. Covered services aren't checked and don't alert.
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Maintenance {
    pub id: u32,
    pub user_id: u32,
    pub title: String,
    pub description: Option<String>,
    pub starts_at: DateTime<Utc>,
    pub ends_at: DateTime<Utc>,
    pub apply_to_all: bool,
    /// Services covered besides all of them when `apply_to_all` is set.
    #[sqlx(skip)]
    pub service_ids: Vec<u32>,
}

/// A whole window, used to create one or replace an existing one.
#[derive(Debug, Clone, Deserialize)]
pub struct MaintenanceForSave {
    #[serde(skip)]
    pub user_id: Option<u32>,
    pub title: String,
    pub description: Option<String>,
    pub starts_at: DateTime<Utc>,
    pub ends_at: DateTime<Utc>,
    #[serde(default)]
    pub apply_to_all: bool,
    #[serde(default)]
    pub service_ids: Vec<u32>,
}

impl MaintenanceForSave {
    pub fn validate(&self) -> Result<(), String> {
        if self.title.trim().is_empty() {
            return Err("title must not be empty".into());
        }
        if self.ends_at <= self.starts_at {
            return Err("ends_at must be after starts_at".into());
        }
        if !self.apply_to_all && self.service_ids.is_empty() {
            return Err("pick at least one service or apply to all".into());
        }
        Ok(())
    }
}

impl Maintenance {
    async fn with_service_ids(
        pool: &SqlitePool,
        mut windows: Vec<Maintenance>,
    ) -> sqlx::Result<Vec<Maintenance>> {
        let links: Vec<(u32, u32)> = sqlx::query_as(
            "SELECT window_id, service_id FROM MaintenanceServices ORDER BY service_id",
        )
        .fetch_all(pool)
        .await?;
        for window in &mut windows {
            window.service_ids = links
                .iter()
                .filter(|(window_id, _)| *window_id == window.id)
                .map(|(_, service_id)| *service_id)
                .collect();
        }
        Ok(windows)
    }

    /// Every window, latest start first.
    pub async fn list(pool: &SqlitePool) -> sqlx::Result<Vec<Maintenance>> {
        let windows =
            sqlx::query_as("SELECT * FROM MaintenanceWindows ORDER BY datetime(starts_at) DESC")
                .fetch_all(pool)
                .await?;
        Self::with_service_ids(pool, windows).await
    }

    /// Windows covering the service that haven't ended by `now`, soonest first.
    pub async fn upcoming_for_service(
        pool: &SqlitePool,
        service_id: u32,
        now: DateTime<Utc>,
    ) -> sqlx::Result<Vec<Maintenance>> {
        let windows = sqlx::query_as(
            r#"SELECT *
               FROM MaintenanceWindows
               WHERE datetime(ends_at) > datetime(?1)
                 AND (apply_to_all = true
                      OR id IN (SELECT window_id FROM MaintenanceServices WHERE service_id = ?2))
               ORDER BY datetime(starts_at)"#,
        )
        .bind(now)
        .bind(service_id)
        .fetch_all(pool)
        .await?;
        Self::with_service_ids(pool, windows).await
    }

    /// The window covering the service at `now`, if any.
    pub async fn active_for_service(
        pool: &SqlitePool,
        service_id: u32,
        now: DateTime<Utc>,
    ) -> sqlx::Result<Option<Maintenance>> {
        let upcoming = Self::upcoming_for_service(pool, service_id, now).await?;
        Ok(upcoming.into_iter().find(|w| w.starts_at <= now))
    }

    async fn save_services(
        tx: &mut sqlx::SqliteConnection,
        window_id: u32,
        service_ids: &[u32],
    ) -> sqlx::Result<()> {
        sqlx::query("DELETE FROM MaintenanceServices WHERE window_id = ?")
            .bind(window_id)
            .execute(&mut *tx)
            .await?;
        for service_id in service_ids {
            sqlx::query(
                "INSERT OR IGNORE INTO MaintenanceServices (window_id, service_id) VALUES (?, ?)",
            )
            .bind(window_id)
            .bind(service_id)
            .execute(&mut *tx)
            .await?;
        }
        Ok(())
    }

    /// Returns the new window's id.
    pub async fn insert(pool: &SqlitePool, window: MaintenanceForSave) -> sqlx::Result<u32> {
        let mut tx = pool.begin().await?;
        let result = sqlx::query(
            r#"INSERT INTO MaintenanceWindows
                   (user_id, title, description, starts_at, ends_at, apply_to_all)
               VALUES (?, ?, ?, ?, ?, ?)"#,
        )
        .bind(window.user_id)
        .bind(window.title.trim())
        .bind(window.description.filter(|d| !d.trim().is_empty()))
        .bind(window.starts_at)
        .bind(window.ends_at)
        .bind(window.apply_to_all)
        .execute(&mut *tx)
        .await?;
        let id = result.last_insert_rowid() as u32;
        Self::save_services(&mut tx, id, &window.service_ids).await?;
        tx.commit().await?;
        Ok(id)
    }

    /// Replace a window's details and services; returns whether it existed.
    pub async fn replace(
        pool: &SqlitePool,
        window_id: u32,
        window: MaintenanceForSave,
    ) -> sqlx::Result<bool> {
        let mut tx = pool.begin().await?;
        let result = sqlx::query(
            r#"UPDATE MaintenanceWindows
               SET title = ?, description = ?, starts_at = ?, ends_at = ?, apply_to_all = ?
               WHERE id = ?"#,
        )
        .bind(window.title.trim())
        .bind(window.description.filter(|d| !d.trim().is_empty()))
        .bind(window.starts_at)
        .bind(window.ends_at)
        .bind(window.apply_to_all)
        .bind(window_id)
        .execute(&mut *tx)
        .await?;
        if result.rows_affected() == 0 {
            return Ok(false);
        }
        Self::save_services(&mut tx, window_id, &window.service_ids).await?;
        tx.commit().await?;
        Ok(true)
    }

    pub async fn delete(pool: &SqlitePool, window_id: u32) -> sqlx::Result<u64> {
        let result = sqlx::query("DELETE FROM MaintenanceWindows WHERE id = ?")
            .bind(window_id)
            .execute(pool)
            .await?;
        Ok(result.rows_affected())
    }
}

#[cfg(test)]
mod tests {
    use chrono::TimeDelta;

    use super::*;

    fn window(
        hours_from_now: (i64, i64),
        apply_to_all: bool,
        service_ids: &[u32],
    ) -> MaintenanceForSave {
        let now = Utc::now();
        MaintenanceForSave {
            user_id: Some(1),
            title: "Database upgrade".into(),
            description: Some(" ".into()),
            starts_at: now + TimeDelta::hours(hours_from_now.0),
            ends_at: now + TimeDelta::hours(hours_from_now.1),
            apply_to_all,
            service_ids: service_ids.to_vec(),
        }
    }

    #[sqlx::test(fixtures(path = "fixtures", scripts("users", "services")))]
    async fn active_and_upcoming_windows(pool: SqlitePool) -> sqlx::Result<()> {
        let now = Utc::now();
        let active = Maintenance::insert(&pool, window((-1, 1), false, &[1, 2])).await?;
        let later = Maintenance::insert(&pool, window((5, 6), false, &[1])).await?;
        let _past = Maintenance::insert(&pool, window((-5, -4), false, &[1])).await?;

        let found = Maintenance::active_for_service(&pool, 1, now)
            .await?
            .unwrap();
        assert_eq!(found.id, active);
        assert_eq!(found.service_ids, [1, 2]);
        assert_eq!(found.description, None, "blank descriptions are dropped");
        assert!(
            Maintenance::active_for_service(&pool, 3, now)
                .await?
                .is_none()
        );

        let upcoming: Vec<u32> = Maintenance::upcoming_for_service(&pool, 1, now)
            .await?
            .iter()
            .map(|w| w.id)
            .collect();
        assert_eq!(upcoming, [active, later], "ended windows are left out");

        // A window for all services covers ones it doesn't list.
        Maintenance::insert(&pool, window((-1, 1), true, &[])).await?;
        assert!(
            Maintenance::active_for_service(&pool, 3, now)
                .await?
                .is_some()
        );

        assert_eq!(Maintenance::list(&pool).await?.len(), 4);
        Ok(())
    }

    #[sqlx::test(fixtures(path = "fixtures", scripts("users", "services")))]
    async fn replace_and_delete(pool: SqlitePool) -> sqlx::Result<()> {
        let id = Maintenance::insert(&pool, window((1, 2), false, &[1])).await?;
        let replaced = Maintenance::replace(
            &pool,
            id,
            MaintenanceForSave {
                title: "Network work".into(),
                ..window((3, 4), false, &[2, 3])
            },
        )
        .await?;
        assert!(replaced);
        let saved = Maintenance::list(&pool).await?.remove(0);
        assert_eq!(saved.title, "Network work");
        assert_eq!(saved.service_ids, [2, 3]);

        assert!(!Maintenance::replace(&pool, 999, window((1, 2), true, &[])).await?);

        assert_eq!(Maintenance::delete(&pool, id).await?, 1);
        assert!(Maintenance::list(&pool).await?.is_empty());
        Ok(())
    }

    #[test]
    fn validation() {
        assert!(window((1, 2), false, &[1]).validate().is_ok());
        assert!(window((1, 2), true, &[]).validate().is_ok());
        assert!(window((1, 2), false, &[]).validate().is_err());
        assert!(window((2, 1), true, &[]).validate().is_err());
        assert!(
            MaintenanceForSave {
                title: " ".into(),
                ..window((1, 2), true, &[])
            }
            .validate()
            .is_err()
        );
    }
}
