use std::path::PathBuf;

use crate::db::meta::EntryMeta;

use super::trie::TrieCache;

pub struct Cacher {
    tree: TrieCache,
}

impl Cacher {
    pub fn new() -> Self {
        Cacher {
            tree: TrieCache::new(),
        }
    }

    pub fn add_path(&mut self, path: &PathBuf, meta: Option<EntryMeta>, update_count: bool) {
        _ = self.tree.insert_path(path, meta, update_count);
    }

    pub fn remove_path(&mut self, path: &PathBuf) {
        _ = self.tree.delete(path);
    }

    pub fn contains_path(&self, path: &PathBuf) -> bool {
        self.tree.contains_full_path(path)
    }

    pub fn search_entry(&self, entry: &str) -> Vec<PathBuf> {
        self.tree.search_entry(entry)
    }
}
