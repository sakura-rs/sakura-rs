use serde::Serialize;

pub mod combo_granter;
pub mod mdk_shield_api;
pub mod register;
pub mod risky_api;

#[derive(Serialize)]
pub struct Response<T> {
    data: Option<T>,
    message: String,
    retcode: i32,
}

impl<T> Response<T> {
    pub fn new(data: T) -> Self {
        Self {
            data: Some(data),
            message: String::from("OK"),
            retcode: 0,
        }
    }

    pub fn error(retcode: i32, message: &str) -> Self {
        Self {
            data: None,
            message: message.to_string(),
            retcode,
        }
    }
}
