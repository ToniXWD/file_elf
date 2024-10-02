use log::{error, info};
use tauri::{AppHandle, Manager};

pub mod server;
pub mod shortcut;
pub mod tray;

pub fn show_window(app: &AppHandle) {
    let windows = app.get_webview_window("main");

    match windows {
        Some(window) => match window.show() {
            Ok(_) => {
                info!("Show existed window successfully.")
            }
            Err(err) => {
                error!("Failed to show window: {}", err);
            }
        },
        None => {
            error!("Failed to get existed window at startup.");
        }
    }
}
