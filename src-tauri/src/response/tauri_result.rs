use std::{
    fmt::{self},
    result,
};

use serde::Serialize;

pub type TauriResult<T> = result::Result<T, TauriError>;

#[derive(Debug, Default, Serialize)]
pub struct ErrorInfo {
    code: String,
    message: String,
}

#[derive(Debug, Serialize)]
pub enum TauriError {
    Base(ErrorInfo),
    NotFound(ErrorInfo),
    FileNotExist(ErrorInfo),
}

impl TauriError {
    pub fn default_not_found() -> Self {
        TauriError::NotFound(ErrorInfo {
            code: "A0001".to_string(),
            message: "Not found".to_string(),
        })
    }

    pub fn default_file_not_exist() -> Self {
        TauriError::FileNotExist(ErrorInfo {
            code: "A0002".to_string(),
            message: "File not exist".to_string(),
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
