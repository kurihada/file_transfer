use std::{
    fs::{self, File},
    io::{ErrorKind, Read},
    path::PathBuf,
    sync::{Arc, Mutex},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub document_dir: Arc<Mutex<Option<String>>>,
}

impl Config {
    pub fn new(document_dir_opt: Option<String>) -> Self {
        Config {
            document_dir: Arc::new(Mutex::new(document_dir_opt)),
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
    match File::open(config_path) {
        Ok(file) => {
            let mut reader: std::io::BufReader<File> = std::io::BufReader::new(file);
            let mut config_json = String::new();
            reader.read_to_string(&mut config_json)?;
            if config_json.is_empty() {
                return Ok(Config::new(None));
            }
            Ok(serde_json::from_str::<Config>(&config_json).unwrap())
        }
        Err(e) if e.kind() == ErrorKind::NotFound => {
            File::create(config_path).unwrap();
            Ok(Config::new(None))
        }
        Err(e) => return Err(e),
    }
}
