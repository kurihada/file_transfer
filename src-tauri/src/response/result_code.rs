#[derive(Debug)]
pub enum ResultCode {
    NotFound,
    FileDirReadFailed,
}

impl ResultCode {
    pub fn code(&self) -> &'static str {
        match self {
            ResultCode::NotFound => "A0001",
            ResultCode::FileDirReadFailed => "A0002",
        }
    }

    pub fn message(&self) -> &'static str {
        match self {
            ResultCode::NotFound => "Not Found",
            ResultCode::FileDirReadFailed => "File directory read failed",
        }
    }
}
