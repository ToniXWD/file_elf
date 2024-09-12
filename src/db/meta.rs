use std::time::SystemTime;
use std::{path::PathBuf, str::FromStr};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
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
    type Err = String;

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

impl EntryMeta {
    /// 从路径创建一个新的 EntryMeta
    pub fn new(path: &PathBuf) -> Result<EntryMeta, std::io::Error> {
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

    pub fn new_empty() -> Result<EntryMeta, std::io::Error> {
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
