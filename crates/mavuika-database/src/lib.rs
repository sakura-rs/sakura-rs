mod config;
pub mod data;
mod error;
pub mod sql_op;
mod util;
pub use error::DbError;

pub use config::DatabaseSettings;

pub use sqlx::migrate::MigrateError;
pub use sqlx::Error as SqlError;

pub struct DbConnection(pub(crate) sqlx::PgPool);

pub async fn connect_to(settings: &DatabaseSettings) -> Result<DbConnection, SqlError> {
    let pool = sqlx::PgPool::connect(&settings.to_string()).await?;
    Ok(DbConnection(pool))
}

pub async fn run_migrations(pool: &DbConnection) -> Result<(), MigrateError> {
    sqlx::migrate!("./migrations").run(&pool.0).await
}
