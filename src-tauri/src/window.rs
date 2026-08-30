use std::path::PathBuf;

use tauri::AppHandle;

pub fn panel(app: &AppHandle) {
    tauri::WebviewWindowBuilder::new(
        app,
        "panel",
        tauri::WebviewUrl::App(PathBuf::from("index.html")),
    )
    .title("Tran")
    .inner_size(256.0, 100.0)
    .fullscreen(false)
    .resizable(false)
    .minimizable(false)
    .maximizable(false)
    .decorations(false)
    .always_on_top(true)
    .skip_taskbar(true)
    .visible(false)
    .shadow(true)
    .center()
    .build()
    .expect("Failed to create panel window");
}

pub fn settings(app: &AppHandle) {
    tauri::WebviewWindowBuilder::new(
        app,
        "settings",
        tauri::WebviewUrl::App(PathBuf::from("settings.html")),
    )
    .title("Tran Settings")
    .inner_size(420.0, 220.0)
    .fullscreen(false)
    .resizable(false)
    .minimizable(false)
    .maximizable(false)
    .decorations(true)
    .always_on_top(false)
    .skip_taskbar(false)
    .visible(false)
    .shadow(true)
    .center()
    .build()
    .expect("Failed to create settings window");
}
