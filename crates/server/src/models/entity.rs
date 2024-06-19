use sqlx::{FromRow, Type};

#[derive(Debug, Type)]
pub enum MonitorType {
    Ping,
    Http,
}

#[derive(Debug, FromRow)]
pub struct Entity {
    pub id: u16,
    pub user_id: u16,
    pub active: bool,
    pub name: String,
    pub interval: u16,
    pub url: Option<String>,
    pub payload: Option<String>,
    pub monitor_type: MonitorType,
    pub retry: u8,
    pub retry_interval: Option<u16>,
}