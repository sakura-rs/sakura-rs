use sqlx::{prelude::FromRow, types::Json};

pub mod password;
pub mod username;

pub use password::Password;
pub use username::Username;

#[derive(FromRow)]
pub struct SdkAccount {
    pub uid: i32,
    pub token: String,
    pub username: Username,
    pub password: Password,
}

#[derive(FromRow)]
pub struct ComboToken {
    pub account_uid: String,
    pub token: String,
    pub device_id: String,
}

#[derive(FromRow)]
pub struct UserUidRow {
    pub account_uid: String,
    pub uid: i32,
}

#[derive(FromRow)]
pub struct PlayerDataRow {
    pub uid: i32,
    pub data: Json<serde_json::Value>,
}
