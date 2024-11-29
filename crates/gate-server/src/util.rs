use tracing::error;

use mavuika_database::{sql_op, DbConnection, DbError};
use mavuika_encryption::xor::MhyXorpad;

pub fn xor_packet(
    session_xorpad: Option<&MhyXorpad>,
    initial_xorpad: Option<&MhyXorpad>,
    buf: &mut [u8],
) {
    match (session_xorpad, initial_xorpad) {
        (Some(xorpad), _) => xorpad.xor(buf),
        (None, Some(xorpad)) => xorpad.xor(buf),
        (_, _) => (),
    }
}

pub async fn fetch_user_uid(db_conn: &DbConnection, account_uid: &str) -> Result<u32, DbError> {
    match sql_op::select_user_uid_by_account_uid(db_conn, account_uid)
        .await
        .inspect_err(|err| error!("failed to select user uid: {err}"))?
    {
        Some(uid) => Ok(uid.uid as u32),
        None => Ok(sql_op::insert_user_uid(db_conn, account_uid)
            .await
            .inspect_err(|err| error!("failed to insert user uid: {err}"))?
            .uid as u32),
    }
}
