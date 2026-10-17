use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

use super::{config::Config, service::DEMO_MIN_INTERVAL};

const CATEGORY: &str = "general";

/// Instance-wide settings admins change from the dashboard, stored in `Configs`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Settings {
    /// Days of logs and alert history to keep; 0 keeps them forever.
    pub log_retention_days: u32,
    /// Defaults the new service form starts with.
    pub default_interval: u16,
    pub default_timeout: u16,
    pub default_retry: u16,
    pub default_retry_interval: u16,
}

impl Settings {
    /// The settings used when none are saved; retention comes from `LOG_RETENTION_DAYS`.
    pub fn defaults(log_retention_days: u32) -> Self {
        Self {
            log_retention_days,
            default_interval: 60,
            default_timeout: 20,
            default_retry: 1,
            default_retry_interval: 30,
        }
    }

    /// The saved settings, with `defaults(log_retention_days)` for any not saved yet.
    pub async fn load(pool: &SqlitePool, log_retention_days: u32) -> sqlx::Result<Self> {
        let mut settings = Self::defaults(log_retention_days);
        for config in Config::list_by_category(pool, CATEGORY).await? {
            // A value that no longer parses keeps the default instead of failing every read.
            let value = config.value.as_str();
            match config.name.as_str() {
                "log_retention_days" => set(&mut settings.log_retention_days, value),
                "default_interval" => set(&mut settings.default_interval, value),
                "default_timeout" => set(&mut settings.default_timeout, value),
                "default_retry" => set(&mut settings.default_retry, value),
                "default_retry_interval" => set(&mut settings.default_retry_interval, value),
                _ => {}
            }
        }
        Ok(settings)
    }

    pub async fn save(&self, pool: &SqlitePool) -> sqlx::Result<()> {
        let values = [
            ("log_retention_days", self.log_retention_days.to_string()),
            ("default_interval", self.default_interval.to_string()),
            ("default_timeout", self.default_timeout.to_string()),
            ("default_retry", self.default_retry.to_string()),
            (
                "default_retry_interval",
                self.default_retry_interval.to_string(),
            ),
        ];
        let mut tx = pool.begin().await?;
        for (name, value) in values {
            Config::upsert(&mut *tx, name, &value, Some(CATEGORY)).await?;
        }
        tx.commit().await
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.default_interval == 0 {
            return Err("default interval must be at least 1 second".into());
        }
        if self.default_timeout == 0 {
            return Err("default timeout must be at least 1 second".into());
        }
        Ok(())
    }

    /// The public demo's limits, on top of [`Self::validate`].
    pub fn validate_demo(&self) -> Result<(), String> {
        if self.default_interval < DEMO_MIN_INTERVAL {
            return Err(format!(
                "default interval must be at least {DEMO_MIN_INTERVAL} seconds in the demo"
            ));
        }
        Ok(())
    }
}

fn set<T: std::str::FromStr>(field: &mut T, value: &str) {
    if let Ok(parsed) = value.parse() {
        *field = parsed;
    }
}

#[cfg(test)]
mod tests {
    use sqlx::SqlitePool;

    use super::*;

    #[sqlx::test]
    async fn load_falls_back_to_defaults(pool: SqlitePool) -> sqlx::Result<()> {
        let settings = Settings::load(&pool, 14).await?;
        assert_eq!(settings, Settings::defaults(14));
        Ok(())
    }

    #[sqlx::test]
    async fn saved_settings_override_defaults(pool: SqlitePool) -> sqlx::Result<()> {
        let saved = Settings {
            log_retention_days: 0,
            default_interval: 300,
            default_timeout: 5,
            default_retry: 0,
            default_retry_interval: 60,
        };
        saved.save(&pool).await?;
        saved.save(&pool).await?;

        assert_eq!(Settings::load(&pool, 90).await?, saved);
        assert_eq!(Config::list_by_category(&pool, CATEGORY).await?.len(), 5);
        Ok(())
    }

    #[sqlx::test]
    async fn unparsable_value_keeps_default(pool: SqlitePool) -> sqlx::Result<()> {
        Config::upsert(&pool, "default_interval", "soon", Some(CATEGORY)).await?;
        let settings = Settings::load(&pool, 90).await?;
        assert_eq!(settings.default_interval, 60);
        Ok(())
    }

    #[test]
    fn validation() {
        let mut settings = Settings::defaults(90);
        assert!(settings.validate().is_ok());

        settings.default_timeout = 0;
        assert!(settings.validate().is_err());

        settings = Settings::defaults(90);
        settings.default_interval = 30;
        assert!(settings.validate().is_ok());
        assert!(settings.validate_demo().is_err());
    }
}
