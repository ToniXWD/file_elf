use std::{
    io::Error,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use crate::db::{meta::EntryMeta, Database};

use super::{trie::TrieCache, CACHER};

pub struct Cacher {
    pub tree: TrieCache,
}

pub fn init_trie(db: Arc<Mutex<dyn Database>>) {
    let guard = db.lock().unwrap();

    let data = guard.find_all();
    let mut cache_guard = CACHER.lock().unwrap();
    let trie = &mut cache_guard.tree.root;

    for (entry, meta) in data {
        let path = meta.path.clone();
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
    ) -> Result<Option<EntryMeta>, Error> {
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

    pub fn search_entry(&self, entry: &str) -> Vec<PathBuf> {
        self.tree.search_entry(entry)
    }
}

mod test {

    #[test]
    fn test_init() {
        use super::*;
        use crate::db::SqliteDatabase;

        let db = SqliteDatabase::new("/home/toni/proj/file_elf/sqlite3.db").unwrap();

        init_trie(Arc::new(Mutex::new(db)));
    }
}
