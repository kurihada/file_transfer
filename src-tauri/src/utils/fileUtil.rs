use std::{
    error::Error,
    fs::{metadata, read_dir},
    path::Path,
};

use crate::{
    entity::structs::DirAndFileInfo,
    response::{resp_data::ResData, response::Response},
};

use super::regexUtil;

pub fn all_path(root_path_str: &String) -> Result<Vec<String>, Box<dyn Error>> {
    let root_path = Path::new(root_path_str);
    let mut root_node = DirAndFileInfo::create(
        root_path.file_name().unwrap().to_str().unwrap().to_string(),
        root_path.to_str().unwrap().to_string(),
        root_path.is_file(),
        root_path.is_dir(),
        root_path.parent().unwrap().to_str().unwrap().to_string(),
    );
    get_dir_and_file(&mut root_node);
    let sdata = serde_json::to_string(&root_node);
    if sdata.is_err() {
        println!(
            "Error, failed to serialize structure: {}",
            sdata.unwrap_err()
        );
    } else {
        let sdata2 = sdata.unwrap();
        println!("Serialized data:\n{}", sdata2);
    }

    let mut path_list = vec![String::from(root_path_str)];
    let mut start_index = 0;
    loop {
        let list_len = path_list.len();
        println!("all_path list_len  {}", list_len);
        for index in start_index..path_list.len() {
            let path = &path_list[index];
            println!("all_path path {} - {}", index, path);
            if metadata(path)?.is_dir() {
                for child_dir in read_dir(&path)? {
                    println!("all_path path child_dir - {:?}", child_dir);
                    path_list.push(String::from(
                        child_dir?.path().as_os_str().to_str().expect(""),
                    ));
                }
            }
        }
        if list_len == start_index {
            break;
        }
        start_index = list_len;
    }
    println!("all_path path path_list - {:?}", path_list);
    return Ok(path_list);
}

pub fn read_dir_info<P: AsRef<Path>>(local_data_dir: P) -> Result<ResData<Vec<String>>, String> {
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
    Ok(ResData {
        code: 500,
        msg: "读取文档目录失败".to_string(),
        data: Vec::new(),
    })
}

/**
 *
 * 获取目录下全部文件以及文件夹
 * num 编号
 * children_dirAndFileInfo 父文件夹信息
 */
pub fn get_dir_and_file(
    parent_info: &mut DirAndFileInfo,
) -> Result<ResData<String>, Box<dyn Error>> {
    let path = &parent_info.path;
    if metadata(&path)?.is_dir() {
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
            if (child_path.is_dir()) {
                get_dir_and_file(&mut child_info);
            }
            parent_info.children.push(child_info);
        }
    }
    Ok(ResData {
        code: 500,
        msg: "文件夹不存在".to_string(),
        data: String::new(),
    })
}

pub fn get_document_notebooks_data(root_path_str: &String) -> Result<String, String> {
    let root_path = Path::new(root_path_str);
    let mut root_node = DirAndFileInfo::create(
        root_path.file_name().unwrap().to_str().unwrap().to_string(),
        root_path.to_str().unwrap().to_string(),
        root_path.is_file(),
        root_path.is_dir(),
        root_path.parent().unwrap().to_str().unwrap().to_string(),
    );
    get_dir_and_file(&mut root_node);
    println!(
        "get_document_notebooks_data {}",
        serde_json::to_string(&root_node).unwrap()
    );
    Ok(serde_json::to_string(&root_node).unwrap())
}
