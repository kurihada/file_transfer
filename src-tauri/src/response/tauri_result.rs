use serde::Serialize;
use std::{
    fmt::{self},
    result,
};

pub type TauriResult<T> = result::Result<T, TauriError>;

#[derive(Debug, Default, Serialize)]
pub struct ErrorInfo {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub enum TauriError {
    Base(ErrorInfo),
    NotFound(ErrorInfo),
    FileNotExist(ErrorInfo),
}

impl TauriError {
    pub fn common_error(message: Option<String>) -> Self {
        TauriError::Base(ErrorInfo {
            code: "A0000".to_string(),
            message: message.unwrap_or("Unknown error".to_string()),
        })
    }

    pub fn param_error(message: Option<String>) -> Self {
        TauriError::Base(ErrorInfo {
            code: "A0001".to_string(),
            message: message.unwrap_or("Param error".to_string()),
        })
    }

    pub fn default_not_found(message: Option<String>) -> Self {
        TauriError::NotFound(ErrorInfo {
            code: "A0002".to_string(),
            message: message.unwrap_or("Not found".to_string()),
        })
    }

    pub fn default_file_not_exist(message: Option<String>) -> Self {
        TauriError::FileNotExist(ErrorInfo {
            code: "A0003".to_string(),
            message: message.unwrap_or("File not exist".to_string()),
        })
    }
}

impl fmt::Display for TauriError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, " {}", self)
    }
}

impl std::error::Error for TauriError {}

impl From<serde_json::Error> for TauriError {
    fn from(err: serde_json::Error) -> Self {
        TauriError::Base(ErrorInfo {
            code: "B0001".to_string(),
            message: err.to_string(),
        })
    }
}

impl From<std::io::Error> for TauriError {
    fn from(err: std::io::Error) -> Self {
        TauriError::Base(ErrorInfo {
            code: "B0002".to_string(),
            message: err.to_string(),
        })
    }
}

impl<T> From<std::sync::PoisonError<T>> for TauriError {
    fn from(err: std::sync::PoisonError<T>) -> Self {
        TauriError::Base(ErrorInfo {
            code: "B0003".to_string(),
            message: err.to_string(),
        })
    }
}
