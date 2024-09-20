use std::{
    collections::BinaryHeap,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use crate::{
    config::CONF,
    db::{
        meta::{EntryMeta, EntryType},
        Database,
    },
    util::{errors::CustomError, is_excluded},
};

use super::{hot_dir::HOTDIR, trie::TrieCache, CACHER};

pub struct Cacher {
    pub tree: TrieCache,
}

pub fn init_trie(db: Arc<Mutex<dyn Database>>) {
    let db_guard = db.lock().unwrap();

    let data = db_guard.find_all();
    drop(db_guard); // 任何时刻只持有一把锁来避免死锁

    let mut cache_guard = CACHER.lock().unwrap();
    let trie = &mut cache_guard.tree.root;

    let mut del_paths = Vec::new();

    // 创建一个 BinaryHeap 优先队列, 用于记录热点文件夹
    let mut dir_heap: BinaryHeap<EntryMeta> = BinaryHeap::new();

    for (entry, meta) in data {
        let path = meta.path.clone();
        // 判断是不是 不存在的文件 or 处于黑名单中的文件
        if is_excluded(&path) {
            println!(
                "path not exists or in blacklist: {:#?}, marked as deleted",
                &path
            );
            del_paths.push(path);
            continue;
        }

        if meta.entry_type == EntryType::Dir {
            // 记录热点文件夹
            dir_heap.push(meta.clone());
            if dir_heap.len() > CONF.database.hotdirnum {
                dir_heap.pop();
            }
        }

        let paths = path
            .components()
            .map(|elem| elem.as_os_str().to_str().unwrap())
            .collect();
        match trie.insert(paths, Some(meta), false) {
            Ok(meta) => {
                if meta.is_some() {
                    println!("init trie: entry({}), meta({:?})", entry, meta.unwrap());
                }
            }
            Err(e) => {
                println!("init trie error: {}", e);
            }
        }
    }
    drop(cache_guard); // 任何时刻只持有一把锁来避免死锁

    let db_guard = db.lock().unwrap();
    // 清理数据库中: 不存在的文件 + 处于黑名单中的文件
    del_paths
        .into_iter()
        .for_each(|path| match db_guard.delete_by_path(&path) {
            Ok(_) => {
                println!("delete path in DB: {:?}", path);
            }
            Err(e) => {
                println!("delete path error: {}", e);
            }
        });
    drop(db_guard); // 任何时刻只持有一把锁来避免死锁

    match HOTDIR.write() {
        Ok(mut hd_guard) => {
            dir_heap.into_iter().for_each(|meta| {
                hd_guard.push(meta); // 加入全局热点文件夹列表
            });
        }
        Err(e) => {
            eprintln!("lock HOTDIR failed: {}", e);
        }
    }
}

impl Cacher {
    pub fn new() -> Self {
        Cacher {
            tree: TrieCache::new(),
        }
    }

    pub fn add_path(
        &mut self,
        path: &PathBuf,
        meta: Option<EntryMeta>,
        update_count: bool,
    ) -> Result<Option<EntryMeta>, CustomError> {
        self.tree.insert_path(path, meta, update_count)
    }

    pub fn remove_path(&mut self, path: &PathBuf) {
        _ = self.tree.delete(path);
    }

    pub fn contains_path(&mut self, path: &PathBuf, update_count: bool) -> bool {
        self.tree.contains_full_path(path, update_count)
    }

    pub fn search_path(&mut self, path: &PathBuf, update_count: bool) -> Option<EntryMeta> {
        self.tree.search_full_path(path, update_count)
    }

    pub fn search_path_regex(&self, pattern_path: &str) -> Vec<PathBuf> {
        self.tree.search_path_regex(pattern_path)
    }

    pub fn search_entry(&self, entry: &str, is_fuzzy: bool) -> Vec<PathBuf> {
        self.tree.search_entry(entry, is_fuzzy)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_init() {
        use super::*;
        use crate::db::SqliteDatabase;

        let db =
            SqliteDatabase::new(&PathBuf::from("/home/toni/proj/file_elf/sqlite3.db")).unwrap();

        init_trie(Arc::new(Mutex::new(db)));
    }
}
