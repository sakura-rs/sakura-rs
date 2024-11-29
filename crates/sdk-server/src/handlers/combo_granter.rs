use axum::{extract::State, routing::post, Json, Router};
use mavuika_database::{sql_op, DbError};
use serde::{Deserialize, Serialize};

use crate::AppState;

use super::Response;

#[derive(Deserialize)]
struct RequestData {
    pub uid: String,
    pub token: String,
}

#[derive(Deserialize)]
struct GranterTokenRequest {
    pub data: String,
    pub device: String,
}

pub fn routes() -> Router<AppState> {
    Router::new().route(
        "/:product_name/combo/granter/login/v2/login",
        post(login_v2),
    )
}

#[derive(Serialize)]
struct ResponseData {
    pub account_type: u32,
    pub combo_id: String,
    pub combo_token: String,
    pub data: &'static str,
    pub heartbeat: bool,
    pub open_id: String,
}

async fn login_v2(
    state: State<AppState>,
    request: Json<GranterTokenRequest>,
) -> Json<Response<ResponseData>> {
    let Ok(data) = serde_json::from_str::<RequestData>(&request.data) else {
        return Json(Response::error(-101, "Account token error"));
    };

    let Ok(uid) = data.uid.parse::<i32>() else {
        return Json(Response::error(-101, "Account token error"));
    };

    match sql_op::SelectSdkAccount::ByUid(uid).fetch(state.db).await {
        Ok(account) if account.token == data.token => (),
        _ => return Json(Response::error(-101, "Account token error")),
    }

    match sql_op::select_combo_token_by_account(state.db, &data.uid).await {
        Ok(token) => success_rsp(token.account_uid, token.token),
        Err(DbError::NotFound) => {
            let Ok(token) = sql_op::insert_combo_token(state.db, &data.uid, &request.device).await
            else {
                return Json(Response::error(-1, "Internal server error"));
            };

            success_rsp(token.account_uid, token.token)
        }
        Err(DbError::SqlxError(err)) => {
            tracing::error!("SQL error: {err}");
            Json(Response::error(-1, "Internal server error"))
        }
    }
}

fn success_rsp(uid: String, token: String) -> Json<Response<ResponseData>> {
    Json(Response::new(ResponseData {
        account_type: 1,
        combo_id: uid.clone(),
        combo_token: token,
        data: r#"{"guest":false}"#,
        heartbeat: false,
        open_id: uid,
    }))
}
