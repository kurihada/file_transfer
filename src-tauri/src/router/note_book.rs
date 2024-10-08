use std::fs;
use std::path::Path;
use std::sync::Arc;

use tauri::{Manager, State};

use crate::config::config::Config;
use crate::response::resp_data::{ResData, ResDataNoData};
use crate::response::response::Response;
use crate::response::tauri_result::{TauriError, TauriResult};

// 获取本地笔记列表
#[tauri::command]
pub async fn get_note_list<'a>(
    config: State<'a, Config>,
    notebook: String,
) -> Result<ResData<Vec<String>>, String> {
    let document_dir_lock = Arc::clone(&config.document_dir);
    let document_dir_opt = document_dir_lock.lock().map_err(|e| e.to_string())?;
    match *document_dir_opt {
        Some(ref dir) => {
            // 本地文件目录
            let local_data_dir = Path::new(dir.as_str());
            // 文件夹是否存在
            if local_data_dir.exists() {
                let mut notes: Vec<String> = vec![];
                println!("get_note_list - {:?}", local_data_dir);
                let paths = std::fs::read_dir(local_data_dir.join(notebook));
                match paths {
                    Ok(ps) => {
                        // 遍历文件
                        for path in ps {
                            let entry = path.unwrap();
                            let path = entry.path();
                            if path.is_file() {
                                let name = path.file_stem().unwrap();
                                let n = name.to_str().unwrap().to_string();
                                println!("get_note_list - {:?}", n);
                                notes.push(n);
                            }
                        }
                        return Ok(ResData {
                            code: 200,
                            msg: "success".to_string(),
                            data: notes,
                        });
                    }
                    Err(_) => return Err("读取目录失败".to_string()),
                }
            }
            Ok(ResData {
                code: 500,
                msg: "文件不存在".to_string(),
                data: Vec::new(),
            })
        }
        None => Ok(ResData {
            code: 500,
            msg: "文件不存在".to_string(),
            data: Vec::new(),
        }),
    }
}

// 读取本地笔记文件
#[tauri::command]
pub async fn read_note_file(cur_path: String) -> Result<ResData<String>, String> {
    // 本地文件目录
    let path = Path::new(cur_path.as_str());
    // 文件夹是否存在
    if path.exists() {
        println!("{:?}", path);
        let content = std::fs::read_to_string(path);
        match content {
            Ok(s) => {
                return Ok(ResData {
                    code: 200,
                    msg: "读取成功".to_string(),
                    data: s,
                })
            }
            Err(err) => {
                return Ok(ResData {
                    code: 500,
                    msg: "笔记读取失败".to_string(),
                    data: err.to_string(),
                })
            }
        }
    }
    return Ok(ResData {
        code: 500,
        msg: format!("笔记文件不存在{:?}", path),
        data: String::new(),
    });
}

// 创建本地 笔记本/文件夹
#[tauri::command]
pub async fn create_folder<'a>(
    config: State<'a, Config>,
    folder_path: String,
) -> TauriResult<Response<String>> {
    let document_dir_lock = Arc::clone(&config.document_dir);
    let document_dir_opt = document_dir_lock.lock()?;
    match *document_dir_opt {
        Some(ref dir) => {
            if !folder_path.starts_with(dir) {
                return Err(TauriError::param_error(Some(format!(
                    "文件夹({})必须在当前文档目录下({})",
                    folder_path, dir
                ))));
            }
            let folder_path = Path::new(&folder_path);
            if folder_path.exists() {
                return Err(TauriError::param_error(Some("文件夹已存在".to_string())));
            }
            if let Err(_) = fs::create_dir_all(folder_path) {
                return Err(TauriError::common_error(Some("创建文件夹失败".to_string())));
            }
            Ok(Response::success("创建成功".to_string()))
        }
        None => Err(TauriError::default_not_found(None)),
    }
}

/**
 * 在指定的目录中创建一个markdown文件。
 */
#[tauri::command]
pub async fn create_note_file<'a>(
    config: State<'a, Config>,
    folder_path: String,
    file_name: String,
) -> TauriResult<Response<String>> {
    let document_dir_lock = Arc::clone(&config.document_dir);
    let document_dir_opt = document_dir_lock.lock()?;
    match *document_dir_opt {
        Some(ref dir) => {
            if !folder_path.starts_with(dir) {
                return Err(TauriError::param_error(Some(format!(
                    "文件夹({})必须在当前文档目录下({})",
                    folder_path, dir
                ))));
            }
            let folder_path = Path::new(&folder_path);
            if !folder_path.exists() {
                return Err(TauriError::param_error(Some("文件夹不存在".to_string())));
            }
            if let Err(_) = fs::File::create(folder_path.join(file_name + ".md")) {
                return Err(TauriError::common_error(Some("创建文件失败".to_string())));
            }
            Ok(Response::success("创建成功".to_string()))
        }
        None => Err(TauriError::default_not_found(None)),
    }
}

/**
 * 保存笔记文件
 * @param folder_path 文件夹路径
 * @param file_name 文件名称
 * @param file_content 文件内容
 */
#[tauri::command]
pub async fn save_note<'a>(
    config: State<'a, Config>,
    folder_path: String,
    file_name: String,
    file_content: String,
) -> TauriResult<Response<String>> {
    let document_dir_lock = Arc::clone(&config.document_dir);
    let document_dir_opt = document_dir_lock.lock()?;
    match *document_dir_opt {
        Some(ref dir) => {
            if !folder_path.starts_with(dir) {
                return Err(TauriError::param_error(Some(format!(
                    "文件夹({})必须在当前文档目录下({})",
                    folder_path, dir
                ))));
            }
            let file_path = Path::new(&folder_path).join(file_name);
            if !file_path.exists() {
                return Err(TauriError::param_error(Some("文件不存在".to_string())));
            }

            if let Err(_) = fs::write(file_path, file_content) {
                return Err(TauriError::common_error(Some("保存文件失败".to_string())));
            }
            Ok(Response::success("保存文件成功".to_string()))
        }
        None => Err(TauriError::default_not_found(None)),
    }
}

/**
 * 删除本地笔记本/文件夹
 * @param app_handle
 * @param current_notebook 文件夹
 */
#[tauri::command]
pub async fn remove_notebook(
    app_handle: tauri::AppHandle,
    current_notebook: String,
) -> Result<ResDataNoData, String> {
    let app_dir = app_handle.path().document_dir();
    if let Ok(dir) = app_dir {
        let local_data_dir = dir.as_path().join("notebooks");
        if local_data_dir.exists() {
            let dir = local_data_dir.join(current_notebook);
            let remove_res = std::fs::remove_dir_all(dir);
            match remove_res {
                Ok(_) => {
                    return Ok(ResDataNoData {
                        code: 200,
                        msg: "操作成功".to_string(),
                    });
                }
                Err(e) => {
                    return Ok(ResDataNoData {
                        code: 500,
                        msg: e.to_string(),
                    });
                }
            }
        }
    }
    Ok(ResDataNoData {
        code: 500,
        msg: "文件夹不存在".to_string(),
    })
}

/**
 * 删除本地笔记
 * @param current_note 笔记
 *
 */
#[tauri::command]
pub async fn remove_note(
    app_handle: tauri::AppHandle,
    current_notebook: String,
    current_note: String,
) -> Result<ResDataNoData, String> {
    let app_dir = app_handle.path().document_dir();
    if let Ok(dir) = app_dir {
        let local_data_dir = dir.as_path().join("notebooks");
        if local_data_dir.exists() {
            let file = local_data_dir.join(current_notebook).join(current_note);
            let remove_res = std::fs::remove_file(file);
            match remove_res {
                Ok(_) => {
                    return Ok(ResDataNoData {
                        code: 200,
                        msg: "操作成功".to_string(),
                    });
                }
                Err(e) => {
                    return Ok(ResDataNoData {
                        code: 500,
                        msg: e.to_string(),
                    });
                }
            }
        }
    }
    Ok(ResDataNoData {
        code: 500,
        msg: "文件夹不存在".to_string(),
    })
}

#[tauri::command]
pub async fn rename_path<'a>(
    config: State<'a, Config>,
    old_path: String,
    new_path: String,
) -> TauriResult<Response<String>> {
    let document_dir_lock = Arc::clone(&config.document_dir);
    let document_dir_opt = document_dir_lock.lock()?;
    match *document_dir_opt {
        Some(ref dir) => {
            if !old_path.starts_with(dir) {
                return Err(TauriError::param_error(Some(format!(
                    "旧路径({})必须在当前文档目录下({})",
                    old_path, dir
                ))));
            }
            if !new_path.starts_with(dir) {
                return Err(TauriError::param_error(Some(format!(
                    "新路径({})必须在当前文档目录下({})",
                    new_path, dir
                ))));
            }
            let old_path = Path::new(&old_path);
            if !old_path.exists() {
                return Err(TauriError::param_error(Some("旧路径不存在".to_string())));
            }
            let new_path = Path::new(&new_path);
            if new_path.exists() {
                return Err(TauriError::param_error(Some(
                    "新路径已经有文件/文件夹存在".to_string(),
                )));
            }
            fs::rename(&old_path, &new_path)?;
            Ok(Response::success("路径更新成功".to_string()))
        }
        None => Err(TauriError::default_not_found(None)),
    }
}
