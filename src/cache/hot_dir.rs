use lazy_static::lazy_static;
use log::{debug, error};
use std::path::PathBuf;
use std::{collections::BinaryHeap, sync::RwLock};
use walkdir::WalkDir;

use crate::config::CONF;
use crate::util::{is_excluded, pattern_match, regex_match};
use crate::{db::EntryMeta, util::errors::CustomError};

lazy_static! {
    pub static ref HOTDIR: RwLock<BinaryHeap<EntryMeta>> = RwLock::new(BinaryHeap::new());
}

pub fn push_hot_dir(meta: EntryMeta) {
    match HOTDIR.write() {
        Ok(mut w_guard) => {
            w_guard.push(meta);
            if w_guard.len() > CONF.database.hotdirnum {
                w_guard.pop();
            }
        }
        Err(e) => {
            error!("lock HOTDIR failed: {}", e);
        }
    }
}

#[allow(unused)]
pub fn pop_hot_dir(meta: EntryMeta) -> Result<Option<EntryMeta>, CustomError> {
    match HOTDIR.write() {
        Ok(mut w_guard) => {
            w_guard.push(meta);
            Ok(w_guard.pop())
        }
        Err(e) => Err(CustomError::ErrStr(e.to_string())),
    }
}

#[allow(unused)]
pub fn get_hot_dir() -> Vec<EntryMeta> {
    let mut res = Vec::new();
    match HOTDIR.read() {
        Ok(mut r_guard) => {
            r_guard.iter().for_each(|meta| {
                res.push(meta.clone());
            });
        }
        Err(e) => return res,
    }
    res
}

pub fn search_files_from_hot_dirs(entry: &str, is_fuzzy: bool, is_regex: bool) -> Vec<String> {
    let mut results = Vec::new();

    match HOTDIR.read() {
        Ok(w_guard) => {
            w_guard.iter().for_each(|meta| {
                // 在每个热点文件夹中搜索
                let mut cur_dir_res = search_target_from_dir(&meta.path, entry, is_fuzzy, is_regex);
                results.append(&mut cur_dir_res);
            });
        }
        Err(e) => {
            error!("lock HOTDIR failed: {}", e);
        }
    }

    results
}

/// 搜索文件
pub fn search_target_from_dir(
    directory: &PathBuf,
    pattern: &str,
    is_fuzzy: bool,
    is_regex: bool,
) -> Vec<String> {
    let mut results = Vec::new();
    if directory.is_file() {
        return results;
    }

    for entry in WalkDir::new(directory).max_depth(1) {
        // 只在热点文件夹中查询1次
        match entry {
            Ok(dir_entry) => {
                debug!("searching path: {:#?}", dir_entry.path());
                let file_name = dir_entry.file_name().to_str().unwrap();
                if is_excluded(&dir_entry.path().to_path_buf()) {
                    continue;
                }

                if is_regex {
                    // 如果指定的是正则匹配
                    if regex_match(&PathBuf::from(file_name), pattern) {
                        results.push(dir_entry.path().to_string_lossy().to_string());
                    }
                } else {
                    // 不使用正则匹配
                    if pattern_match(file_name, pattern, is_fuzzy) {
                        results.push(dir_entry.path().to_string_lossy().to_string());
                    }
                }
            }
            Err(e) => {
                error!("Error: {}", e);
            }
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use crate::{cache::cache::init_trie, db::DB};

    #[allow(unused)]
    use super::*;

    #[test]
    fn test_search_files() {
        init_trie(DB.clone()); // 构建热点文件夹
                               // let dir = PathBuf::from("F:/betopgame/6BYe9bkf/bin/electron1.7.12");
                               // let res = search_target_from_dir(&dir, "ffmpeg.dll", false);
        let res = search_files_from_hot_dirs("example.psd", true, false);
        println!("{:?}", res);
        assert!(res.len() > 0);
    }

    #[test]
    fn test_get_hot_dir() {
        init_trie(DB.clone()); // 构建热点文件夹
        let hot_dirs = get_hot_dir();
        println!("{:?}", hot_dirs);
    }
}
