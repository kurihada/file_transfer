use std::{
    fs::{self, File, OpenOptions},
    io::{ErrorKind, Read, Write},
    path::PathBuf,
    sync::{Arc, Mutex},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub document_dir: Arc<Mutex<Option<String>>>,
}

impl Config {
    fn new(document_dir: String) -> Self {
        Config {
            document_dir: Arc::new(Mutex::new(Some(document_dir))),
        }
    }

    pub fn save_config(config_dir: &PathBuf, config: &Config) -> () {
        let config_json = serde_json::to_string_pretty(&config).unwrap();
        fs::write(config_dir.join("config.json"), config_json).unwrap();
    }
}

pub fn init_config(config_dir: &PathBuf) -> std::io::Result<Config> {
    if !config_dir.exists() {
        if let Err(e) = fs::create_dir_all(&config_dir) {
            eprintln!("Failed to create config directory:{}", e);
            return Err(e.into());
        }
    }

    let config_path = &config_dir.join("config.json");
    // let mut file = OpenOptions::new()
    //     .create(true)
    //     .write(true)
    //     .open(config_path)?;
    // let mut config_json = String::new();
    // file.read_to_string(&mut config_json)?;
    // let config = serde_json::from_str::<Config>(&config_json)?;
    // Ok(config);
    // 尝试以读模式打开文件
    tauri_plugin_fs::
    let file = match File::open(config_path) {
        Ok(f) => f,
        Err(e) if e.kind() == ErrorKind::NotFound => {
            // 如果文件不存在，则创建它（注意：这会创建一个空文件）
            // 然后再次尝试以读模式打开它
            File::create(config_path)?
        }
        Err(e) => return Err(e), // 其他错误直接返回
    };

    // 使用BufReader来读取文件内容
    let mut reader: std::io::BufReader<File> = std::io::BufReader::new(file);
    let mut config_json = String::new();
    reader.read_to_string(&mut config_json)?; // 读取内容到String中
    let config = serde_json::from_str::<Config>(&config_json)?;
    Ok(config)
}
