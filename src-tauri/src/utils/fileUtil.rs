use std::{
    fs::{metadata, read_dir},
    path::Path,
};

use crate::{
    entity::structs::DirAndFileInfo,
    response::{
        response::Response,
        tauri_result::{TauriError, TauriResult},
    },
};

/**
 * 获取目录下的所有文件
 * @param root_path_str 根目录
 * @returns 给定目录下的所有文件信息
 */
pub fn get_document_notebooks_data(root_path_str: &String) -> TauriResult<Response<String>> {
    let root_path = Path::new(root_path_str);
    let mut root_node = DirAndFileInfo::create(
        root_path.file_name().unwrap().to_str().unwrap().to_string(),
        root_path.to_str().unwrap().to_string(),
        root_path.is_file(),
        root_path.is_dir(),
        root_path.parent().unwrap().to_str().unwrap().to_string(),
    );
    get_dir_and_file(&mut root_node)?;
    let result = serde_json::to_string(&root_node)?;
    println!("get_document_notebooks_data {}", &result);
    Ok(Response::success(result))
}

/**
 * 获取目录下的所有文件
 * @param parent_info 父级目录信息
 * @returns void
 */
pub fn get_dir_and_file(parent_info: &mut DirAndFileInfo) -> TauriResult<()> {
    let path = &parent_info.path;
    println!("get_dir_and_file {}", &path);
    if !metadata(&path)?.is_dir() {
        return Err(TauriError::default_file_not_exist());
    }
    for child_dir in read_dir(&path)? {
        let child_path = child_dir?.path();
        let mut child_info = DirAndFileInfo::create(
            child_path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
            child_path.to_str().unwrap().to_string(),
            child_path.is_file(),
            child_path.is_dir(),
            child_path.parent().unwrap().to_str().unwrap().to_string(),
        );
        if child_path.is_dir() {
            get_dir_and_file(&mut child_info)?;
        }
        parent_info.children.push(child_info);
    }
    Ok(())
}
