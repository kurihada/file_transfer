use lazy_static::lazy_static;
use serde::Serialize;

use super::tauri_result::TauriError;

#[derive(Serialize)]
pub enum Result<T> {
    Success {
        code: String,
        message: String,
        data: T,
    },
    Failure {
        code: String,
        message: String,
    },
}

#[derive(Serialize, Deserialize)]
pub struct Response<T> {
    pub result: Result<T>,
}

#[allow(dead_code)]
impl<T> Response<T> {
    pub fn success(data: T) -> Self {
        Self {
            result: Result::Success {
                code: "200".to_string(),
                message: "success".to_string(),
                data,
            },
        }
    }
    pub fn success_with_message(data: T, message: &str) -> Self {
        Self {
            result: Result::Success {
                code: "200".to_string(),
                message: message.to_string(),
                data,
            },
        }
    }
    pub fn success_with_code(data: T, message: &str, code: &str) -> Self {
        Self {
            result: Result::Success {
                code: code.to_string(),
                message: message.to_string(),
                data,
            },
        }
    }
    pub fn failure_with_message(message: &str, code: &str) -> Self {
        Self {
            result: Result::Failure {
                code: code.to_string(),
                message: message.to_string(),
            },
        }
    }
}

lazy_static! {
    pub static ref FAILURE_NOT_FOUND: Response<()> = Response {
        result: Result::Failure {
            code: "404".to_string(),
            message: "not found".to_string(),
        },
    };
}
