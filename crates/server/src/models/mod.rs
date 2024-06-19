use sqlx::SqlitePool;

pub use self::user::{UserForLogin, UserForRegister};

pub mod entity;
pub mod status_log;
pub mod user;
pub mod notification;

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
    sqlx::migrate!().run(pool).await?;
    Ok(())
}