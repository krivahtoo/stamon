use sqlx::SqlitePool;
use tracing::info;

pub use self::user::{UserForLogin, UserForRegister};

pub mod config;
pub mod log;
pub mod notification;
pub mod service;
pub mod user;

pub async fn setup(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query("PRAGMA journal_mode = 'WAL';")
        .execute(pool)
        .await?;
    sqlx::query("PRAGMA temp_store = 2;").execute(pool).await?;
    sqlx::query("PRAGMA synchronous = NORMAL;")
        .execute(pool)
        .await?;
    sqlx::query("PRAGMA cache_size = 64000;")
        .execute(pool)
        .await?;
    info!("Running database migrations");
    sqlx::migrate!().run(pool).await?;
    Ok(())
}
