use std::path::Path;
use std::sync::Arc;

use tauri::{Manager, State};

use crate::config::config::Config;
use crate::response::resp_data::{ResData, ResDataNoData};

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
pub async fn create_notebook(
    app_handle: tauri::AppHandle,
    notebook_name: String,
) -> Result<ResDataNoData, String> {
    let app_dir = app_handle.path().document_dir();
    if let Ok(dir) = app_dir {
        // 本地文件目录
        let local_data_dir = dir.as_path().join("notebooks");
        // 文件夹是否存在
        if local_data_dir.exists() {
            let dir = local_data_dir.join(notebook_name);
            // 创建文件夹
            let res = std::fs::create_dir_all(dir);
            match res {
                Ok(_) => {
                    return Ok(ResDataNoData {
                        code: 200,
                        msg: "创建成功".to_string(),
                    })
                }
                Err(err) => {
                    return Ok(ResDataNoData {
                        code: 500,
                        msg: format!("创建失败：{0}", err.to_string()),
                    })
                }
            }
        }
        return Ok(ResDataNoData {
            code: 500,
            msg: "文件夹不存在".to_string(),
        });
    }
    Ok(ResDataNoData {
        code: 500,
        msg: "文件夹不存在".to_string(),
    })
}

/**
 * @param notebook 文件夹/笔记本
 * @param newNoteMdFileName 笔记名 文件名
 */
#[tauri::command]
pub async fn create_note_file(
    app_handle: tauri::AppHandle,
    notebook: String,
    newNoteMdFileName: String,
) -> Result<ResData<String>, String> {
    let app_dir = app_handle.path().document_dir();
    if let Ok(dir) = app_dir {
        let local_data_dir = dir.as_path().join("notebooks");
        if local_data_dir.exists() {
            let newNoteMdFileName_: String = newNoteMdFileName.clone();
            let name = chrono::prelude::Local::now().timestamp_millis().to_string();
            let temp_file = local_data_dir
                .join(notebook)
                .join(newNoteMdFileName + ".md");
            // 创建文件
            let res = std::fs::File::create(temp_file);
            match res {
                Ok(_) => {
                    return Ok(ResData {
                        code: 200,
                        msg: "创建成功".to_string(),
                        data: newNoteMdFileName_,
                    })
                }
                Err(err) => {
                    return Ok(ResData {
                        code: 500,
                        msg: format!("创建失败：{0}", err.to_string()),
                        data: String::new(),
                    })
                }
            }
        }
        return Ok(ResData {
            code: 500,
            msg: "文件夹不存在".to_string(),
            data: String::new(),
        });
    }
    Ok(ResData {
        code: 500,
        msg: "文件夹不存在".to_string(),
        data: String::new(),
    })
}

/**
 * @param current_notebook 文件夹
 * @param current_note
 * @param md_title 标题/文件名
 * @param md_text 内容
 */
#[tauri::command]
pub async fn save_note(
    app_handle: tauri::AppHandle,
    current_notebook: String,
    current_note: String,
    md_title: String,
    md_text: String,
) -> Result<ResDataNoData, String> {
    let app_dir = app_handle.path().document_dir();
    if let Ok(dir) = app_dir {
        // 本地文件目录
        let local_data_dir = dir.as_path().join("notebooks");
        // 文件夹是否存在
        if local_data_dir.exists() {
            let path = local_data_dir
                .join(current_notebook.clone())
                .join(current_note + ".md");
            let new_path = local_data_dir.join(current_notebook).join(md_title + ".md");
            let wres = std::fs::write(path.clone(), md_text);
            match wres {
                Ok(_) => {
                    //将文件或目录重命名为新名称，如果 to 已存在，则替换原始文件。如果新名称位于不同的挂载点，这将不起作用。
                    let new_name_res = std::fs::rename(path, new_path);
                    match new_name_res {
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
