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

#[macro_export]
macro_rules! build_query_bind {
    ($query_builder:ident, $update_data:ident, {
        $($field:ident),*
    }) => {
        $(
            if let Some(value) = $update_data.$field {
                $query_builder = $query_builder.bind(value);
            }
        )*
    };
}

#[macro_export]
macro_rules! build_update_query {
    ($query:ident, $has_updates:ident, $update_data:ident, {
        $($field:ident),*
    }) => {
        $(
            if $update_data.$field.is_some() {
                $query.push_str(concat!(stringify!($field), " = ?, "));
                $has_updates = true
            }
        )*
    };
}
