use tauri::{Manager, State};

use crate::{
    config::config::Config,
    response::resp_data::ResData,
    utils::fileUtil::{all_path, get_document_notebooks_data, read_dir_info},
};
use std::{fs, path::Path, ptr::null, result::Result::Ok, sync::Arc};

#[tauri::command]
pub async fn my_read_file(path: std::path::PathBuf) -> String {
    // 读取文件内容，以文本字符串形式返回
    std::fs::read_to_string(path).unwrap()
}

// 获取本地笔记本列表
#[tauri::command]
pub async fn get_home_notebooks(
    app_handle: tauri::AppHandle,
) -> Result<ResData<Vec<String>>, String> {
    let app_dir = app_handle.path().document_dir();
    if let Ok(dir) = app_dir {
        // 本地文件目录
        let local_data_dir = dir.as_path().join("notebooks");
        println!("local_data_dir： {:?}", local_data_dir);
        let local_data_dir_2 = local_data_dir.clone();
        all_path(&local_data_dir_2.as_os_str().to_str().unwrap().to_string());
        read_dir_info(&local_data_dir);
        // 文件夹是否存在
        if local_data_dir.exists() {
            let mut notebooks: Vec<String> = vec![];
            let paths = std::fs::read_dir(local_data_dir);
            match paths {
                Ok(ps) => {
                    // 遍历目录
                    for path in ps {
                        let entry = path.unwrap();
                        let path = entry.path();
                        if path.is_dir() {
                            let name = path.file_stem().unwrap();
                            let n = name.to_str().unwrap().to_string();
                            notebooks.push(n);
                        }
                    }
                    return Ok(ResData {
                        code: 200,
                        msg: "success".to_string(),
                        data: notebooks,
                    });
                }
                Err(_) => return Err("读取目录失败".to_string()),
            }
        }
        // 目录不存在则创建
        let a = std::fs::create_dir_all(local_data_dir);
        match a {
            Ok(_) => println!("创建目录成功"),
            Err(_) => return Err("创建目录失败".to_string()),
        }
    }
    Ok(ResData {
        code: 500,
        msg: "读取目录失败".to_string(),
        data: Vec::new(),
    })
}

// 获取本地笔记本列表
#[tauri::command]
pub async fn get_document_notebooks<'a>(
    config: State<'a, Config>,
) -> Result<ResData<String>, String> {
    let document_dir_lock = Arc::clone(&config.document_dir);
    let document_dir_opt = document_dir_lock.lock().map_err(|e| e.to_string())?;
    match *document_dir_opt {
        Some(ref dir) => {
            println!("document_dir: {:?}", dir);
            let local_data_dir = Path::new(&dir);
            if !local_data_dir.exists() {
                std::fs::create_dir_all(local_data_dir).map_err(|_| "创建目录失败")?;
            }
            return get_document_notebooks_data(&dir);
        }
        None => Err("读取文档目录失败".into()),
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
