use std::path::PathBuf;

use log::{debug, error};

use crate::{
    backend::{
        new_event_handler,
        writer::{DbAction, SENDER},
    },
    cache::{hot_dir::search_files_from_hot_dirs, CACHER},
    db::DB,
    util::is_excluded,
};

pub fn api_search(entry: String, is_fuzzy: bool) -> Vec<(String, bool)> {
    debug!("search: entry({}), is_fuzzy({})", entry, is_fuzzy);
    if entry.is_empty() {
        return Vec::new();
    }
    let guard = CACHER.lock().unwrap(); // 使用 mut 解锁后可以释放锁
    let res = guard.search_entry(&entry, is_fuzzy);

    drop(guard); // 显式释放锁

    if res.is_empty() {
        // 缓存没有查到, 从数据库中尽显查询(数据库查询暂不支持模糊查询)
        debug!("cache not found, DB search: entry({})", entry);
        match DB.lock().unwrap().find_by_entry(&entry) {
            Ok(recs) => {
                let res2 = recs
                    .into_iter()
                    .map(|elem| (elem.path.to_string_lossy().to_string(), true))
                    .collect();
                debug!("search: res2({:?})", res2);
                res2
            }
            Err(e) => {
                error!("DB error: {}", e);
                Vec::new()
            }
        }
    } else {
        let res2 = res
            .into_iter()
            .map(|elem| (elem.into_os_string().into_string().unwrap(), true))
            .collect();
        debug!("search: res2({:?})", res2);

        res2
    }
}

pub fn api_hot_search(entry: String, is_fuzzy: bool, is_regex: bool) -> Vec<(String, bool)> {
    debug!(
        "hot_search: entry({}), is_fuzzy({}), is_regex({})",
        entry, is_fuzzy, is_regex
    );
    if entry.is_empty() {
        return Vec::new();
    }
    let res = search_files_from_hot_dirs(&entry, is_fuzzy, is_regex);

    let mut cache_guard = CACHER.lock().unwrap();

    let res2 = res
        .into_iter()
        .map(|elem| {
            if cache_guard.contains_path(&PathBuf::from(&elem), false) {
                (elem, true)
            } else {
                (elem, false)
            }
        })
        .collect();

    debug!("hot_search: res2({:?})", res2);

    res2
}

pub fn api_regex_search(path: String) -> Vec<(String, bool)> {
    debug!("regex_search: entry({})", path);
    if path.is_empty() {
        return Vec::new();
    }
    let guard = CACHER.lock().unwrap(); // 使用 mut 解锁后可以释放锁
    let res = guard.search_path_regex(&path);

    let res2 = res
        .into_iter()
        .map(|elem| (elem.into_os_string().into_string().unwrap(), true))
        .collect();
    debug!("regex_search: res2({:?})", res2);
    res2
}

pub fn api_star_path(path_data: String) -> bool {
    let r_path = PathBuf::from(path_data);

    // 先插入缓存
    let mut guard = CACHER.lock().unwrap();
    _ = guard.add_path(&r_path, None, false);
    debug!("star_path: {:#?} insert to cache success", r_path);
    drop(guard);

    // 再插入数据库
    let sender = SENDER.clone();
    new_event_handler(&r_path, &sender);
    debug!("star_path: {:#?} insert to db success", r_path);

    true
}

pub fn api_unstar_path(path_data: String) -> bool {
    let r_path = PathBuf::from(path_data);
    if is_excluded(&r_path) {
        return true;
    }

    // 先删除缓存
    let mut guard = CACHER.lock().unwrap();
    _ = guard.remove_path(&r_path);
    drop(guard);

    // 再删除数据库
    let msg = DbAction::DELETE(r_path);

    let sender = SENDER.clone();
    match sender.send(msg) {
        Ok(_) => true,
        Err(_) => false,
    }
}

#[allow(unused)]
mod tests {
    use super::*;
    use crate::{cache::cache::init_trie, db::SqliteDatabase};
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_search() {
        let raw_db = SqliteDatabase::new(&PathBuf::from(
            "C:\\Users\\toni\\AppData\\Local\\search-files-app\\sqlite3.db",
        ))
        .unwrap();
        let db = Arc::new(Mutex::new(raw_db));
        init_trie((db));

        let res = api_search("小论文".to_string(), false);
        println!("{:?}", res)
    }
}
