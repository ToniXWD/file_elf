use std::path::PathBuf;

use crate::db::meta::EntryType;

#[allow(unused)]
pub fn get_path_type(path: &PathBuf) -> EntryType {
    // 用match判断path的类型, 然后赋值给n_type
    match path.metadata() {
        Ok(metadata) => {
            if metadata.is_dir() {
                return EntryType::Dir;
            } else {
                return EntryType::File;
            }
        }
        Err(_) => {
            return EntryType::Unknown;
        }
    }
}
