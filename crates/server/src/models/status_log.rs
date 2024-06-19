use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;

#[derive(Debug, FromRow)]
pub struct StatusLog {
    pub id: u64,
    pub entity_id: u32,
    pub status: u8,
    pub message: Option<String>,
    pub time: DateTime<Utc>,
    pub ping: Option<u32>,
    pub duration: u32,
}