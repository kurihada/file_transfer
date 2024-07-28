use std::error::Error;

#[derive(serde::Deserialize, serde::Serialize)]
pub enum DeviceKind {
    Window,
    Linux,
    Android,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct DeviceInfo {
    name: String,
    ip: String,
    kind: DeviceKind,
}

impl DeviceInfo {
    pub fn new(name: String, ip: String, kind: DeviceKind) -> DeviceInfo {
        DeviceInfo { name, ip, kind }
    }
}

pub fn scan_devices() -> Result<Vec<DeviceInfo>, Box<dyn Error>> {
    let device = DeviceInfo::new(
        "test".to_string(),
        "0.0.0.0".to_string(),
        DeviceKind::Window,
    );
    Ok(vec![device])
}
