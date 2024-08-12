use serde::Serialize;

#[derive(Serialize)]
pub struct Response<T> {
    pub code: String,
    pub message: String,
    pub data: T,
}

#[allow(dead_code)]
impl<T: Default> Response<T> {
    pub fn success(data: T) -> Self {
        Self {
            code: "200".to_string(),
            message: "success".to_string(),
            data,
        }
    }
    pub fn success_with_message(data: T, message: &str) -> Self {
        Self {
            code: "200".to_string(),
            message: message.to_string(),
            data,
        }
    }
    pub fn success_with_code(data: T, message: &str, code: &str) -> Self {
        Self {
            code: code.to_string(),
            message: message.to_string(),
            data,
        }
    }
    pub fn failure_with_message(message: &str, code: &str) -> Self {
        Self {
            code: code.to_string(),
            message: message.to_string(),
            data: T::default(),
        }
    }
}
