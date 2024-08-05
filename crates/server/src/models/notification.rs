use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;

#[derive(Debug, FromRow)]
pub struct Notification {
    pub id: u64,
    pub entity_id: u32,
    pub status_log_id: u32,
    pub message: Option<String>,
    pub sent_at: DateTime<Utc>,
}
