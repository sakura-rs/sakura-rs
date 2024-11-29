#[derive(thiserror::Error, Debug)]
pub enum DbError {
    #[error("SQL error: {0}")]
    SqlxError(#[from] sqlx::Error),
    #[error("entry not found")]
    NotFound,
}
