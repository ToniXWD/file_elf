use chrono::{DateTime, Local};
use log::info;
use std::path::Path;
use std::time::SystemTime;

use crate::config::CONF;

pub fn setup_logger() -> Result<(), fern::InitError> {
    let log_level;
    info!("log_level: {}", CONF.database.log_level);
    match CONF.database.log_level.as_str() {
        "trace" => log_level = log::LevelFilter::Trace,
        "debug" => log_level = log::LevelFilter::Debug,
        "info" => log_level = log::LevelFilter::Info,
        "warn" => log_level = log::LevelFilter::Warn,
        "error" => log_level = log::LevelFilter::Error,
        _ => log_level = log::LevelFilter::Info, // 其余情况都使用info
    }

    info!("config log_level: {}", CONF.database.log_level);

    let log_path = Path::new("file_elf.log");
    if let Some(file_size) = check_file_size(log_path) {
        if file_size >= 10 * 1024 * 1024 {
            // 如果以前的文件超过10MB，则将其归档
            let archive_name = format!(
                "file_elf_{}_{}.log",
                log_path.display(),
                SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs()
            );
            let _ = std::fs::rename(log_path, archive_name);
        }
    }

    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}] {} {} {}",
                format_local_datetime(SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log_level)
        .chain(std::io::stdout())
        .chain(fern::log_file(log_path)?)
        .apply()?;
    Ok(())
}

/// 格式化时间并使用本地时区
fn format_local_datetime(now: SystemTime) -> String {
    // 将 SystemTime 转换为 DateTime<Utc>
    let datetime: DateTime<Local> = now.into();

    // 格式化时间为不含特殊字符的字符串，适合文件命名
    let formatted_time = datetime.format("%Y%m%d_%H%M%S").to_string();
    formatted_time
}

/// 检查文件大小
fn check_file_size<P: AsRef<Path>>(path: P) -> Option<u64> {
    if let Ok(metadata) = std::fs::metadata(path) {
        Some(metadata.len())
    } else {
        None
    }
}
