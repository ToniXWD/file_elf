// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
use std::path::PathBuf;
use std::process::Command;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use std::time::Instant;
use tauri::GlobalShortcutManager;
use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

/// 打开文件
#[tauri::command]
fn open_file(name: String) {
    println!("Opening file: {}", name);
    let path = PathBuf::from(name);

    if path.exists() {
        open_directory(&path);
    } else {
        println!("The provided path does not exist.");
    }
}

/// 判断name是文件还是文件夹, 如果是文件夹则打开资源管理器, 否则打开文件所在目录的资源管理器
#[tauri::command]
fn open_dir(name: String) {
    println!("Opening file or directory: {}", name);
    let path = PathBuf::from(name);

    if path.exists() {
        if path.is_dir() {
            // 如果是目录，则直接打开该目录
            open_directory(&path);
        } else if path.is_file() {
            // 如果是文件，则尝试打开文件所在的目录
            let parent_dir = path
                .parent()
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from("."));
            open_directory(&parent_dir);
        }
    } else {
        println!("The provided path does not exist.");
    }
}

/// 打开指定的目录
fn open_directory(path: &PathBuf) {
    if let Err(err) = open::that(path) {
        println!("Failed to open directory: {}", err);
    } else {
        println!("Directory opened successfully.");
    }
}

fn launch_file_elf() {
    // 获取当前目录
    let current_dir = env::current_dir().expect("Failed to get current directory");

    // 判断操作系统，选择正确的文件名
    #[cfg(target_os = "windows")]
    let elf_file = "file_elf.exe";

    #[cfg(not(target_os = "windows"))]
    let elf_file = "file_elf";

    // 生成文件路径
    let elf_path = current_dir.join(elf_file);

    // 如果文件存在，则启动子进程
    if elf_path.exists() {
        println!("Found backend executable: {}", elf_file);

        #[cfg(target_os = "windows")]
        let result = Command::new(elf_path)
            .creation_flags(0x08000000) // CREATE_NO_WINDOW flag for Windows
            .spawn(); // 启动子进程

        #[cfg(not(target_os = "windows"))]
        let result = Command::new(elf_path).spawn(); // 启动子进程 (非Windows平台)

        match result {
            Ok(_) => println!("Backend process started successfully."),
            Err(e) => eprintln!("Failed to start backend process: {}", e),
        }
    } else {
        eprintln!("Backend executable not found: {}", elf_file);
    }
}

fn main() {
    // 启动后端程序(生产环境)
    launch_file_elf();

    // 创建托盘菜单项
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let show = CustomMenuItem::new("show".to_string(), "Show");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");

    // 定义托盘菜单
    let tray_menu = SystemTrayMenu::new()
        .add_item(hide)
        .add_item(show)
        .add_item(quit);

    // 初始化托盘
    let system_tray = SystemTray::new().with_menu(tray_menu);
    tauri::Builder::default()
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => {
                let window = app.get_window("main").unwrap();
                match id.as_str() {
                    "hide" => {
                        window.hide().unwrap();
                    }
                    "show" => {
                        window.show().unwrap();
                    }
                    "quit" => {
                        std::process::exit(0);
                    }
                    _ => {}
                }
            }
            _ => {}
        })
        .on_window_event(|event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event.event() {
                event.window().hide().unwrap();
                api.prevent_close(); // 阻止窗口关闭
            }
        })
        .setup(|app| {
            let window = app.get_window("main").unwrap();

            // 使用 Arc 和 Mutex 来包装 last_press_time 以便在多线程环境中安全地修改
            let last_press_time = Arc::new(Mutex::new(Instant::now()));
            let double_click_threshold = Duration::from_millis(500); // 设置双击阈值时间为 500 毫秒

            let mut shortcut_manager = app.global_shortcut_manager();
            let last_press_time_clone = Arc::clone(&last_press_time);

            match shortcut_manager.register("Esc", move || {
                let now = Instant::now();
                let mut last_time = last_press_time_clone.lock().unwrap();

                if now.duration_since(*last_time) < double_click_threshold {
                    // 如果两次 Alt 键按下的时间间隔小于阈值，显示或隐藏窗口
                    if window.is_visible().unwrap() {
                        window.hide().unwrap();
                    } else {
                        window.show().unwrap();
                        window.set_focus().unwrap(); // 让窗口获得焦点
                    }
                }
                *last_time = now; // 更新 last_press_time
            }) {
                Ok(_) => println!("Shortcut registered successfully."),
                Err(e) => println!("Failed to register shortcut: {}", e),
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![open_file, open_dir])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
