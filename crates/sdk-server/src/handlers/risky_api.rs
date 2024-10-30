use axum::{routing::post, Router};

use crate::AppState;

pub fn routes() -> Router<AppState> {
    Router::new().route("/account/risky/api/check", post(risky_api_check))
}

async fn risky_api_check(_: String) -> &'static str {
    r#"{"data":{},message:"OK",retcode:0}"#
}
