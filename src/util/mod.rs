pub mod errors;

use std::path::PathBuf;

use regex::Regex;
use strsim::levenshtein;

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
