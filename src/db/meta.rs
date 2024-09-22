use std::cmp::Ordering;
use std::time::SystemTime;
use std::{path::PathBuf, str::FromStr};

use serde::{Deserialize, Serialize};

use crate::util::errors::CustomError;

#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq)]
pub enum EntryType {
    Dir,
    File,
    Unknown,
}

impl ToString for EntryType {
    fn to_string(&self) -> String {
        match self {
            EntryType::Dir => "Dir".to_string(),
            EntryType::File => "File".to_string(),
            EntryType::Unknown => "Unknown".to_string(),
        }
    }
}

impl FromStr for EntryType {
    type Err = CustomError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "Dir" {
            Ok(EntryType::Dir)
        } else if s == "File" {
            Ok(EntryType::File)
        } else {
            Ok(EntryType::Unknown)
        }
    }
}

/// 文件项的元数据，包括从 PathBuf 中获取的基础属性，以及本项目维持的一个访问计数
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EntryMeta {
    /// 文件的路径
    pub path: PathBuf,
    /// 文件大小
    pub size: u64,
    /// 最后修改时间
    pub modified: SystemTime,
    /// 文件的访问计数
    pub access_count: u32,
    /// 文件类型
    pub entry_type: EntryType,
}

impl PartialEq for EntryMeta {
    fn eq(&self, other: &Self) -> bool {
        self.access_count == other.access_count && self.entry_type == other.entry_type
    }
}

impl PartialOrd for EntryMeta {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // 1. 比较 entry_type
        match (self.entry_type, other.entry_type) {
            (EntryType::File, EntryType::File) => {
                // 2. entry_type 相同时，比较 access_count
                other.access_count.partial_cmp(&self.access_count)
            }
            (EntryType::File, _) => Some(Ordering::Less), // `File` 优先级高
            (_, EntryType::File) => Some(Ordering::Greater),
            (_, _) => {
                // 2. 比较 access_count
                other.access_count.partial_cmp(&self.access_count)
            }
        }
    }
}

impl Eq for EntryMeta {}

impl Ord for EntryMeta {
    fn cmp(&self, other: &Self) -> Ordering {
        // 1. 比较 entry_type
        match (self.entry_type, other.entry_type) {
            (EntryType::File, EntryType::File) => Ordering::Equal,
            (EntryType::File, _) => Ordering::Less, // `File` 优先级高
            (_, EntryType::File) => Ordering::Greater,
            (_, _) => {
                // 2. 比较 access_count
                other.access_count.cmp(&self.access_count)
            }
        }
    }
}

impl EntryMeta {
    /// 从路径创建一个新的 EntryMeta
    pub fn new(path: &PathBuf) -> Result<EntryMeta, CustomError> {
        // 如何判断path是空字符串?
        if path.to_str().unwrap().is_empty() {
            return EntryMeta::new_empty();
        }
        let metadata = path.metadata()?;

        let entry_type = if metadata.is_dir() {
            EntryType::Dir
        } else {
            EntryType::File
        };

        Ok(EntryMeta {
            path: path.clone(),
            size: metadata.len(),
            modified: metadata.modified()?,
            access_count: 0,
            entry_type,
        })
    }

    pub fn new_empty() -> Result<EntryMeta, CustomError> {
        Ok(EntryMeta {
            path: PathBuf::new(),
            size: 0,
            modified: SystemTime::now(),
            access_count: 0,
            entry_type: EntryType::Unknown,
        })
    }

    /// 增加访问计数
    pub fn increment_access_count(&mut self) {
        self.access_count += 1;
    }
}

#[cfg(test)]
mod tests {
    use log::debug;

    use super::*;
    use std::{
        collections::BinaryHeap,
        time::{Duration, SystemTime},
    };

    #[test]
    fn test_entry_meta_ordering() {
        // 创建几个测试用的路径
        let path1 = PathBuf::from("/file1");
        let path2 = PathBuf::from("/file2");
        let path3 = PathBuf::from("/dir1");

        // 创建不同类型的 EntryMeta
        let entry_file1 = EntryMeta {
            path: path1,
            size: 100,
            modified: SystemTime::now(),
            access_count: 10,
            entry_type: EntryType::File,
        };

        let entry_file2 = EntryMeta {
            path: path2,
            size: 100,
            modified: SystemTime::now(),
            access_count: 20,
            entry_type: EntryType::File,
        };

        let entry_dir = EntryMeta {
            path: path3,
            size: 100,
            modified: SystemTime::now(),
            access_count: 5,
            entry_type: EntryType::Dir,
        };

        // 文件应该优先于目录
        assert!(entry_file1 < entry_dir);
        assert!(entry_file2 < entry_dir);

        // 同为文件时，访问计数高的应该优先
        assert!(entry_file2 < entry_file1);
    }

    #[test]
    fn test_entry_meta_equality() {
        let path1 = PathBuf::from("/file1");

        let modified_time = SystemTime::now() - Duration::new(3600, 0); // 1小时前

        let entry_file1 = EntryMeta {
            path: path1.clone(),
            size: 100,
            modified: modified_time,
            access_count: 10,
            entry_type: EntryType::File,
        };

        let entry_file2 = EntryMeta {
            path: path1.clone(),
            size: 100,
            modified: modified_time,
            access_count: 10,
            entry_type: EntryType::File,
        };

        // 相同访问计数和类型的文件应该相等
        assert_eq!(entry_file1, entry_file2);

        // 修改访问计数应该不相等
        let mut entry_file2_different = entry_file2.clone();
        entry_file2_different.access_count = 15;
        assert_ne!(entry_file1, entry_file2_different);
    }

    #[test]
    fn test_increment_access_count() {
        let path = PathBuf::from("/file1");

        let mut entry = EntryMeta {
            path: path.clone(),
            size: 100,
            modified: SystemTime::now(),
            access_count: 10,
            entry_type: EntryType::File,
        };

        // 增加访问计数
        entry.increment_access_count();
        assert_eq!(entry.access_count, 11);
    }

    #[test]
    fn test_new_entry_meta_empty() {
        let empty_entry = EntryMeta::new_empty().unwrap();

        assert!(empty_entry.path.to_str().unwrap().is_empty());
        assert_eq!(empty_entry.size, 0);
        assert_eq!(empty_entry.entry_type, EntryType::Unknown);
    }

    #[test]
    fn test_new_entry_meta_from_path() {
        let path = PathBuf::from("/tmp");

        // 假设路径是一个目录
        let entry = EntryMeta::new(&path).unwrap();

        assert_eq!(entry.path, path);
        assert_eq!(entry.entry_type, EntryType::Dir); // 目录类型
    }

    #[test]
    fn test_heap() {
        // 创建一个 BinaryHeap 优先队列
        let mut heap: BinaryHeap<EntryMeta> = BinaryHeap::new();

        // 创建几个测试用的 EntryMeta 实例
        let entry_file1 = EntryMeta {
            path: PathBuf::from("/file1"),
            size: 100,
            modified: SystemTime::now(),
            access_count: 10,
            entry_type: EntryType::File,
        };

        let entry_file2 = EntryMeta {
            path: PathBuf::from("/file2"),
            size: 200,
            modified: SystemTime::now() - Duration::new(3600, 0), // 1小时前
            access_count: 20,
            entry_type: EntryType::File,
        };

        let entry_dir1 = EntryMeta {
            path: PathBuf::from("/dir1"),
            size: 500,
            modified: SystemTime::now(),
            access_count: 5,
            entry_type: EntryType::Dir,
        };

        let entry_dir2 = EntryMeta {
            path: PathBuf::from("/dir2"),
            size: 400,
            modified: SystemTime::now(),
            access_count: 15,
            entry_type: EntryType::Dir,
        };

        // 将 EntryMeta 实例加入到优先队列中
        heap.push(entry_file1.clone());
        heap.push(entry_file2.clone());
        heap.push(entry_dir1.clone());
        heap.push(entry_dir2.clone());

        // 按照优先队列的顺序弹出并验证比较规则
        debug!("Pop from BinaryHeap (highest priority first):");

        while let Some(entry) = heap.pop() {
            debug!(
                "{:?} - Access Count: {}, Type: {:?}",
                entry.path, entry.access_count, entry.entry_type
            );
        }
    }
}
