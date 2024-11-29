mod sdk_sql_op;

pub use sdk_sql_op::{
    insert_combo_token, insert_sdk_account, select_combo_token_by_account, SelectSdkAccount,
};
use sqlx::{query, query_as};

use crate::{
    data::{PlayerDataRow, UserUidRow},
    DbConnection, DbError,
};

pub async fn insert_or_update_player_data(
    conn: &DbConnection,
    uid: i32,
    data: serde_json::Value,
) -> Result<(), DbError> {
    query("INSERT INTO t_player_data (uid, data) VALUES ($1, $2) ON CONFLICT (uid) DO UPDATE SET data = ($2)")
        .bind(uid)
        .bind(data)
        .execute(&conn.0)
        .await
        .map(|_| ())
        .map_err(DbError::from)
}

pub async fn select_player_data_by_uid(
    conn: &DbConnection,
    uid: i32,
) -> Result<Option<PlayerDataRow>, DbError> {
    query_as("SELECT * FROM t_player_data WHERE uid = ($1)")
        .bind(uid)
        .fetch_optional(&conn.0)
        .await
        .map_err(DbError::from)
}

pub async fn select_user_uid_by_account_uid(
    conn: &DbConnection,
    account_uid: &str,
) -> Result<Option<UserUidRow>, DbError> {
    query_as("SELECT * FROM t_user_uid WHERE account_uid = ($1)")
        .bind(account_uid)
        .fetch_optional(&conn.0)
        .await
        .map_err(DbError::from)
}

pub async fn insert_user_uid(
    conn: &DbConnection,
    account_uid: &str,
) -> Result<UserUidRow, DbError> {
    query_as("INSERT INTO t_user_uid (account_uid) VALUES ($1) RETURNING *")
        .bind(account_uid)
        .fetch_one(&conn.0)
        .await
        .map_err(DbError::from)
}
