use sqlx::{QueryBuilder, Sqlite, SqlitePool};
use tracing::info;

pub use self::user::{UserForLogin, UserForRegister};

pub mod channel;
pub mod config;
pub mod log;
pub mod maintenance;
pub mod notification;
pub mod service;
pub mod user;

/// The ids in `ids` with no row in `table`.
pub async fn missing_ids(pool: &SqlitePool, table: &str, ids: &[u32]) -> sqlx::Result<Vec<u32>> {
    if ids.is_empty() {
        return Ok(vec![]);
    }
    let mut query = QueryBuilder::<Sqlite>::new(format!("SELECT id FROM {table} WHERE id IN ("));
    let mut separated = query.separated(", ");
    for id in ids {
        separated.push_bind(id);
    }
    query.push(")");
    let found: Vec<u32> = query.build_query_scalar().fetch_all(pool).await?;
    Ok(ids
        .iter()
        .copied()
        .filter(|id| !found.contains(id))
        .collect())
}

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
