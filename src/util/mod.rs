pub mod errors;

use std::path::PathBuf;

use errors::CustomError;
use regex::Regex;
use strsim::levenshtein;

use crate::config::CONF;

/// 模糊匹配
pub fn pattern_match(entry: &str, pattern: &str, is_fuzzy: bool) -> bool {
    if is_fuzzy {
        // 设定一个模糊匹配的阈值，比如距离小于等于2
        let threshold = 2;
        levenshtein(entry, pattern) <= threshold
    } else {
        entry == pattern
    }
}

/// 正则表达式匹配
pub fn regex_match(path: &PathBuf, pattern: &str) -> bool {
    // 将 PathBuf 转换为 &str 类型
    let path_str = path.to_str().unwrap_or("");

    // 编译正则表达式
    let re = Regex::new(pattern).unwrap();

    // 使用正则表达式匹配路径
    re.is_match(path_str)
}

/// 不存在或处于黑名单的文件
pub fn is_excluded(path: &PathBuf) -> bool {
    // 获取文件名
    if !path.exists() {
        return true;
    }

    // 检查文件名是否在黑名单中
    return is_blacklisted(path);
}

/// 将相对路径转换为绝对路径的辅助函数
pub fn to_absolute_path(path: &PathBuf) -> Result<PathBuf, CustomError> {
    if path.is_absolute() {
        Ok(path.to_path_buf())
    } else {
        // 转换为绝对路径
        let absolute_path = std::env::current_dir()?.join(path);
        Ok(absolute_path)
    }
}

// 检查文件是否在黑名单中的辅助函数
pub fn is_blacklisted(file_path: &PathBuf) -> bool {
    for pattern in &CONF.database.blacklist {
        if regex_match(file_path, pattern) {
            return true;
        }
    }

    false
}

mod test {
    #[allow(unused)]
    use super::*;
    #[test]
    fn test_is_blacklisted() {
        // 设置windows和非windows条件编译
        #[cfg(target_os = "windows")]
        let mut path = PathBuf::from("F://CSLearn//proj//file_elf//target//debug//file_elf.exe");

        #[cfg(not(target_os = "windows"))]
        let mut path = PathBuf::from("/home/toni/proj/file_elf/target/debug/file_elf");

        let res = is_blacklisted(&path);
        assert!(res);

        path = PathBuf::from("/home/toni/proj/file_elf/Cargo.toml");

        let res = is_blacklisted(&path);
        assert!(!res);

        let res = is_excluded(&path);
        assert!(!res);

        path = PathBuf::from("/home/toni/proj/file_elf/virtual.toml");

        let res = is_blacklisted(&path);
        assert!(!res);

        let res = is_excluded(&path);
        assert!(res);
    }
}
