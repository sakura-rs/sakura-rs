use rand::distributions::{Alphanumeric, DistString};
use sqlx::query_as;

use crate::{
    data::{self, ComboToken},
    DbConnection, DbError,
};

pub async fn insert_sdk_account(
    conn: &DbConnection,
    username: data::Username,
    password: data::Password,
) -> Result<data::SdkAccount, DbError> {
    let token = Alphanumeric.sample_string(&mut rand::thread_rng(), 64);

    Ok(query_as(
        "INSERT INTO t_sdk_account (token, username, password) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(token)
    .bind(username.as_str())
    .bind(password.hash_str())
    .fetch_one(&conn.0)
    .await?)
}

pub enum SelectSdkAccount<'s> {
    ByUsername(&'s str),
    ByUid(i32),
}

impl SelectSdkAccount<'_> {
    pub async fn fetch(self, conn: &DbConnection) -> Result<data::SdkAccount, DbError> {
        match self {
            Self::ByUsername(username) => {
                query_as("SELECT * from t_sdk_account where username = ($1)")
                    .bind(username)
                    .fetch_optional(&conn.0)
                    .await?
                    .ok_or(DbError::NotFound)
            }
            Self::ByUid(uid) => query_as("SELECT * from t_sdk_account where uid = ($1)")
                .bind(uid)
                .fetch_optional(&conn.0)
                .await?
                .ok_or(DbError::NotFound),
        }
    }
}

pub async fn insert_combo_token(
    conn: &DbConnection,
    account_uid: &str,
    device_id: &str,
) -> Result<ComboToken, DbError> {
    let token = Alphanumeric.sample_string(&mut rand::thread_rng(), 64);

    Ok(
        query_as("INSERT INTO t_combo_token VALUES ($1, $2, $3) RETURNING *")
            .bind(account_uid)
            .bind(token)
            .bind(device_id)
            .fetch_one(&conn.0)
            .await?,
    )
}

pub async fn select_combo_token_by_account(
    conn: &DbConnection,
    account_uid: &str,
) -> Result<ComboToken, DbError> {
    query_as("SELECT * from t_combo_token where account_uid = ($1)")
        .bind(account_uid)
        .fetch_optional(&conn.0)
        .await?
        .ok_or(DbError::NotFound)
}
