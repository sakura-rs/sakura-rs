use axum::{
    extract::State,
    response::Html,
    routing::{get, post},
    Form, Router,
};
use mavuika_database::{
    data::{password, Password, Username},
    sql_op, DbError, SqlError,
};
use serde::Deserialize;

use crate::AppState;

const REGISTER_PAGE: Html<&str> = Html(include_str!("../../html/register.html"));

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/account/register", get(|| async { REGISTER_PAGE }))
        .route("/account/register", post(process_register_request))
}

#[derive(Deserialize)]
struct RegisterForm {
    pub username: String,
    pub password: String,
    pub password_v2: String,
}

async fn process_register_request(
    State(state): State<AppState>,
    Form(form): Form<RegisterForm>,
) -> Html<String> {
    let Some(username) = Username::parse(form.username) else {
        return html_result(ResultStatus::Failure, "Invalid username format; should consists of characters [A-Za-z0-9_] and be at least 6 characters long.");
    };

    let password = match Password::new(form.password, form.password_v2) {
        Ok(password) => password,
        Err(password::Error::PairMismatch) => {
            return html_result(ResultStatus::Failure, "Passwords pair doesn't match")
        }
        Err(password::Error::LengthMismatch) => {
            return html_result(
                ResultStatus::Failure,
                "Password should contain at least 8 and not more than 30 characters",
            )
        }
        Err(password::Error::HashFailed(err)) => {
            tracing::error!("failed to hash password, err: {err}");
            return html_result(ResultStatus::Failure, "Internal server error");
        }
    };

    match sql_op::insert_sdk_account(state.db, username, password).await {
        Ok(_) => html_result(
            ResultStatus::Success,
            "Account successfully registered, now you can use in-game login",
        ),
        Err(DbError::SqlxError(SqlError::Database(e)))
            if e.constraint() == Some("t_sdk_account_username_key") =>
        {
            html_result(
                ResultStatus::Failure,
                "Account with specified username already exists",
            )
        }
        Err(err) => {
            tracing::error!("database operation error: {err}");
            html_result(ResultStatus::Failure, "Internal server error")
        }
    }
}

enum ResultStatus {
    Failure,
    Success,
}

impl ResultStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Failure => "error",
            Self::Success => "success",
        }
    }
}

fn html_result(result: ResultStatus, message: &str) -> Html<String> {
    static RESULT_HTML: &str = include_str!("../../html/result.html");

    Html(
        RESULT_HTML
            .replace("%RESULT%", result.as_str())
            .replace("%MESSAGE%", message),
    )
}
