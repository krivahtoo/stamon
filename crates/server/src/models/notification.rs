use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::prelude::FromRow;

#[derive(Debug, FromRow)]
pub struct Notification {
    pub id: u64,
    pub service_id: u32,
    pub log_id: u32,
    pub message: Option<String>,
    pub sent_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct NotificationForCreate {
    pub service_id: u32,
    pub log_id: u32,
    pub message: Option<String>,
    pub sent_at: DateTime<Utc>,
}
