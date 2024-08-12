use tauri::{Manager, State};

use crate::{
    config::config::Config,
    entity::structs::DirAndFileInfo,
    response::{
        response::Response,
        tauri_result::{TauriError, TauriResult},
    },
    utils::fileUtil::get_document_notebooks_data,
};
use std::{path::Path, result::Result::Ok, sync::Arc};

// 获取本地笔记本列表
#[tauri::command]
pub async fn get_document_notebooks<'a>(
    config: State<'a, Config>,
) -> TauriResult<Response<DirAndFileInfo>> {
    let document_dir_lock = Arc::clone(&config.document_dir);
    let document_dir_opt = document_dir_lock.lock()?;
    match *document_dir_opt {
        Some(ref dir) => {
            println!("document_dir: {}", dir);
            let local_data_dir = Path::new(&dir);
            if !local_data_dir.exists() {
                std::fs::create_dir_all(local_data_dir)?;
            }
            return get_document_notebooks_data(&dir);
        }
        None => Err(TauriError::default_not_found()),
    }
}

#[tauri::command]
pub async fn set_data_dir<'a>(
    app_handle: tauri::AppHandle,
    config: State<'a, Config>,
    data_dir: String,
) -> Result<(), String> {
    {
        let document_dir_lock = Arc::clone(&config.document_dir);
        let mut document_dir_opt = document_dir_lock.lock().map_err(|e| e.to_string())?;
        *document_dir_opt = Some(data_dir);
    }
    Config::save_config(&app_handle.path().app_config_dir().unwrap(), &config);
    Ok(())
}

#[tauri::command]
pub async fn exist_file(cur_dir: String, file_name: String) -> Result<bool, String> {
    let file_path = cur_dir + "/" + &file_name;
    println!("file_path: {:?}", file_path);
    let file_path = Path::new(&file_path);
    Ok(file_path.exists())
}
