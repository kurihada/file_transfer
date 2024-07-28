use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct DirAndFileInfo {
    pub name: String,
    pub path: String,
    pub is_file: bool,
    pub is_dir: bool,
    pub parent_id: String,
    pub children: Vec<DirAndFileInfo>,
}

impl DirAndFileInfo {
    pub fn create(
        name: String,
        path: String,
        is_file: bool,
        is_dir: bool,
        parent_id: String,
    ) -> DirAndFileInfo {
        DirAndFileInfo {
            name,
            path,
            is_file,
            is_dir,
            parent_id,
            children: Vec::new(),
        }
    }
}
