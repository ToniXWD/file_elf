pub mod errors;

use std::path::PathBuf;

use crate::config::CONF;
use errors::CustomError;
use log::{error, trace};
use regex::Regex;
use strsim::levenshtein;

#[cfg(target_os = "windows")]
use std::os::windows::ffi::OsStringExt;
#[cfg(target_os = "windows")]
use winapi::um::fileapi::GetLogicalDriveStringsW;

pub fn pattern_match(entry: &str, pattern: &str, is_fuzzy: bool) -> bool {
    let entry_l = entry.to_lowercase();
    let pattern_l = pattern.to_lowercase();

    // 只取前缀部分进行匹配
    let entry_prefix: String = entry_l.chars().take(pattern_l.chars().count()).collect();
    let pattern_prefix: String = pattern_l.chars().take(entry_l.chars().count()).collect();

    if is_fuzzy && pattern.len() > 3 && entry.len() > 3 {
        // 模糊匹配只在字符串长度>3时才启用
        // 设定一个模糊匹配的阈值，比如距离小于等于1
        let threshold = 1;
        let res = levenshtein(&entry_prefix, &pattern_prefix) <= threshold;
        trace!(
            "levenshtein: {} and {}: {}",
            entry_prefix,
            pattern_prefix,
            res
        );
        res
    } else {
        entry_prefix == pattern_prefix
    }
}

/// 正则表达式匹配
pub fn regex_match(path: &PathBuf, pattern: &str) -> bool {
    // 将 PathBuf 转换为 &str 类型
    let path_str = path.to_str().unwrap_or("");

    // 编译正则表达式
    if let Ok(re) = Regex::new(pattern) {
        // 使用正则表达式匹配路径
        re.is_match(path_str)
    } else {
        error!("Invalid regex pattern: {}", pattern);
        false
    }
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

// 条件编译: Windows
// 获取当前操作系统的所有盘符
#[cfg(target_os = "windows")]
pub fn get_drives() -> Vec<String> {
    use std::ffi::OsString;

    let mut buffer: [u16; 256] = [0; 256];
    let length = unsafe { GetLogicalDriveStringsW(buffer.len() as u32, buffer.as_mut_ptr()) };
    if length == 0 {
        return Vec::new();
    }
    let os_string = OsString::from_wide(&buffer[..length as usize]);
    os_string
        .to_string_lossy()
        .split('\u{0}')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
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

    #[test]
    #[cfg(target_os = "windows")]
    fn test_drives() {
        let drives = get_drives();
        println!("{:?}", drives);
    }
}
