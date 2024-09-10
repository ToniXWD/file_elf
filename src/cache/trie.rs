use std::{collections::HashMap, io::Error, path::PathBuf};

use crate::db::meta::EntryMeta;

pub struct TrieCache {
    root: TrieNode,
}

impl TrieCache {
    pub fn new() -> TrieCache {
        TrieCache {
            root: TrieNode::new(),
        }
    }

    pub fn search_full_path(&self, path: &PathBuf) -> Option<PathBuf> {
        let paths = path
            .components()
            .map(|elem| elem.as_os_str().to_str().unwrap())
            .collect();
        self.root.search_full_path(paths)
    }

    pub fn search_entry(&self, entry: &str) -> Vec<PathBuf> {
        self.root.search_entry(entry)
    }

    pub fn insert_path(
        &mut self,
        path: &PathBuf,
        meta: Option<EntryMeta>,
        update_count: bool,
    ) -> Result<(), Error> {
        let paths = path
            .components()
            .map(|elem| elem.as_os_str().to_str().unwrap())
            .collect();
        self.root.insert(paths, meta, update_count)
    }

    pub fn contains_full_path(&self, path: &PathBuf) -> bool {
        let paths = path
            .components()
            .map(|elem| elem.as_os_str().to_str().unwrap())
            .collect();
        self.root.search_full_path(paths).is_some()
    }

    pub fn delete(&mut self, path: &PathBuf) -> Result<(), String> {
        let paths = path
            .components()
            .map(|elem| elem.as_os_str().to_str().unwrap())
            .collect();
        self.root.delete(paths)
    }
}

#[derive(Clone)]
pub struct TrieNode {
    pub(crate) entry_name: String,
    pub(crate) full_path: PathBuf,
    pub(crate) meta: EntryMeta,
    pub(crate) children: HashMap<String, Box<TrieNode>>,
}

impl TrieNode {
    fn new() -> TrieNode {
        TrieNode {
            entry_name: String::new(),
            full_path: PathBuf::new(),
            children: HashMap::new(),
            meta: EntryMeta::new(&PathBuf::new()).unwrap(),
        }
    }

    fn new_with_path(path: &PathBuf) -> Result<TrieNode, std::io::Error> {
        // 尝试获取文件名
        let file_name = path.file_name();
        let entry_name = match file_name {
            Some(name) => name.to_str().unwrap().to_string(),
            None => "/".to_string(), // 如果没有文件名，默认为 "/"
        }
        .to_string();

        let meta = EntryMeta::new(path)?;

        Ok(TrieNode {
            entry_name,
            full_path: meta.path.clone(),
            children: std::collections::HashMap::new(),
            meta,
        })
    }

    pub fn contains_full_path(&self, path: &PathBuf) -> bool {
        let paths = path
            .components()
            .map(|elem| elem.as_os_str().to_str().unwrap())
            .collect();
        self.search_full_path(paths).is_some()
    }

    /// 根据文件名或文件夹名查找所有匹配的路径
    pub fn search_entry(&self, entry_name: &str) -> Vec<PathBuf> {
        let mut results = Vec::new();

        // 如果当前节点的名称与目标名称相匹配，将当前节点的路径加入结果
        if self.entry_name == entry_name {
            results.push(self.full_path.clone());
        }

        // 对于每个子节点，递归搜索并合并结果
        for (_, child) in &self.children {
            results.extend(child.search_entry(entry_name));
        }

        results
    }

    fn search_full_path(&self, path: Vec<&str>) -> Option<PathBuf> {
        let mut cur_node = self;

        for p in path {
            if let Some(node) = cur_node.children.get(p) {
                cur_node = &**node;
            } else {
                return None;
            }
        }
        Some(cur_node.full_path.clone())
    }

    // 更新 insert 方法，以便在插入时构建完整路径
    pub fn insert(
        &mut self,
        path: Vec<&str>,
        meta: Option<EntryMeta>,
        update_count: bool,
    ) -> Result<(), Error> {
        let mut full_path = self.full_path.clone(); // 初始化完整路径
        let mut cur_node = self;

        for (idx, component) in path.iter().enumerate() {
            let component_str = component.to_string();

            // 检查子节点是否存在，如果不存在则创建新的节点
            cur_node
                .children
                .entry(component_str.clone())
                .or_insert_with(|| {
                    let new_path = full_path.join(component_str.clone());
                    let mut new_node = TrieNode::new_with_path(&new_path).unwrap();

                    if idx == path.len() - 1 && meta.is_some() {
                        new_node.meta = meta.clone().unwrap();
                    }

                    Box::new(new_node)
                });

            // 移动到子节点并更新完整路径
            cur_node = &mut **cur_node.children.get_mut(&component_str).unwrap();
            full_path.push(component);

            // 更新完整路径
            cur_node.full_path = full_path.clone();
            if update_count {
                // 不一定将访问计数自增, 在初始化的时候从数据库构建trie时就不需要自增访问计数
                cur_node.meta.access_count += 1; // 访问计数加1
            }
            println!("{:?}", &cur_node.meta);
        }

        Ok(())
    }

    pub fn delete(&mut self, path: Vec<&str>) -> Result<(), String> {
        let mut cur_node = self;

        for (index, component) in path.iter().enumerate() {
            let component_str = component.to_string();
            let is_last = index == path.len() - 1;

            if cur_node.children.contains_key(&component_str) {
                if is_last {
                    cur_node.children.remove(&component_str);
                    return Ok(());
                }
                cur_node = &mut **cur_node.children.get_mut(&component_str).unwrap();
            } else {
                return Err("PathNotFound".to_string());
            }
        }

        Ok(())
    }
}

mod test {
    #[test]
    fn simple_test() {
        use super::*;

        let mut cache = TrieCache::new();

        let path = PathBuf::from("/tmp/tmp/documents/file.txt");
        let res = cache.insert_path(&path, None, false);

        assert!(res.is_ok());

        let found_path = cache.search_full_path(&PathBuf::from("/tmp"));
        assert_eq!(found_path, Some(PathBuf::from("/tmp")));

        let found_path = cache.search_full_path(&PathBuf::from("/tmp/tmp/documents"));
        assert_eq!(found_path, Some(PathBuf::from("/tmp/tmp/documents")));

        let found_path = cache.search_full_path(&PathBuf::from("/tmp/tmp/documents/file.txt"));
        assert_eq!(
            found_path,
            Some(PathBuf::from("/tmp/tmp/documents/file.txt"))
        );

        let path = PathBuf::from("/tmp/tmp2/documents/file.txt");
        let res = cache.insert_path(&path, None, false);

        assert!(res.is_ok());

        let found_path = cache.search_full_path(&PathBuf::from("/tmp"));
        assert_eq!(found_path, Some(PathBuf::from("/tmp")));

        let found_path = cache.search_full_path(&PathBuf::from("/tmp/tmp2/documents"));
        assert_eq!(found_path, Some(PathBuf::from("/tmp/tmp2/documents")));

        let found_path = cache.search_full_path(&PathBuf::from("/tmp/tmp2/documents/file.txt"));
        assert_eq!(
            found_path,
            Some(PathBuf::from("/tmp/tmp2/documents/file.txt"))
        );
    }

    #[test]
    fn test_search_entry() {
        use super::*;

        let mut cache = TrieCache::new();

        let path1 = PathBuf::from("/tmp/tmp/documents/file1.txt");
        let path2 = PathBuf::from("/tmp/tmp/documents/file2.txt");
        let path3 = PathBuf::from("/tmp/tmp/downloads/file1.txt");
        let path4 = PathBuf::from("/tmp/tmp/downloads/file2.txt");

        _ = cache.insert_path(&path1, None, false);
        _ = cache.insert_path(&path2, None, false);
        _ = cache.insert_path(&path3, None, false);
        _ = cache.insert_path(&path4, None, false);

        // 检查根据文件名搜索
        let mut results = cache.search_entry("file1.txt");
        results.sort();
        assert_eq!(results, vec![path1, path3]);

        // 检查根据目录名搜索
        let mut results = cache.search_entry("documents");
        results.sort();
        assert_eq!(results, vec![PathBuf::from("/tmp/tmp/documents")]);

        // 检查不存在的条目
        let mut results = cache.search_entry("nonexistent");
        results.sort();
        assert_eq!(results, Vec::<PathBuf>::new());
    }
}
