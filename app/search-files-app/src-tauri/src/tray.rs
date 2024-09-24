use tauri::{
    menu::{CheckMenuItem, Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager, Runtime,
};
use tauri_plugin_autostart::MacosLauncher;
use tauri_plugin_autostart::ManagerExt;

pub fn create_tray<R: Runtime>(app: &tauri::AppHandle<R>) -> tauri::Result<()> {
    app.plugin(tauri_plugin_autostart::init(
        MacosLauncher::LaunchAgent,
        Some(vec!["--split"]),
    ))?;
    // Get the autostart manager

    let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let show_i = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
    let auto_i = CheckMenuItem::new(
        app,
        "auto start",
        true,
        app.autolaunch().is_enabled().unwrap(),
        None::<&str>,
    )
    .unwrap();
    let auto_start_id = auto_i.id().as_ref().to_string();
    println!("auto start id: {}", auto_start_id);

    let menu = Menu::with_items(app, &[&show_i, &auto_i, &quit_i])?;

    let _ = TrayIconBuilder::with_id("tray")
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .menu_on_left_click(false)
        .on_menu_event(move |app, event| match event.id.as_ref() {
            "quit" => {
                println!("menu item id: quit");
                app.cleanup_before_exit();
                app.exit(0);
            }
            "show" => {
                println!("menu item id: show");
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            id if id == &auto_start_id => {
                println!("menu item id: auto start");

                if app.autolaunch().is_enabled().unwrap() {
                    // 重设为取消自动重启
                    let _ = app.autolaunch().disable();
                } else {
                    // 重设为自动重启
                    let _ = app.autolaunch().enable();
                }
                println!(
                    "registered for autostart? {}",
                    app.autolaunch().is_enabled().unwrap()
                );
            }
            u => {
                println!("unknown menu item id: {}", u);
            }
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        })
        .build(app);

    Ok(())
}
