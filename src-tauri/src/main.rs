#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::atomic::Ordering;

use resp::R;
use tauri::Manager;

mod clip;
mod common;
mod config;
mod lang;
mod manager;
mod resp;
mod setup;
mod shortcut;
mod tray;
mod util;
mod window;

/// 写入剪贴板
///
/// Write to clipboard
#[tauri::command]
fn copy(content: String) -> R<()> {
    clip::set(content).into()
}

/// 打开指定链接
///
/// Open the specified link
#[tauri::command]
async fn open(url: String) -> R<()> {
    open::that(url).map_err(anyhow::Error::msg).into()
}

/// 固定窗口标识
///
/// Pin the window
#[tauri::command]
async fn pin() -> R<bool> {
    R::success(common::PIN.load(Ordering::SeqCst))
}

/// 取消固定窗口标识
///
/// Unpin the window
#[tauri::command]
async fn unpin() {
    common::PIN.store(false, Ordering::SeqCst);
}

/// 取消临时固定窗口标识
///
/// Unpin the temporary window
#[tauri::command]
async fn untmp() {
    common::TMP_PIN.store(false, Ordering::SeqCst);
}

/// 取消临时固定窗口标识
///
/// Unpin the temporary window
#[tauri::command]
async fn theme() -> R<String> {
    R::success(config::theme())
}

/// 获取当前翻译服务地址
///
/// Get current translation host
#[tauri::command]
async fn host() -> R<String> {
    R::success(config::host())
}

/// 获取当前代理地址
///
/// Get current proxy URL
#[tauri::command]
async fn proxy() -> R<String> {
    R::success(config::proxy())
}

/// 获取当前模式
///
/// Get current mode
#[tauri::command]
async fn mode() -> R<bool> {
    R::success(config::mode())
}

/// 设置翻译服务地址
///
/// Set translation host
#[tauri::command]
async fn set_host(host: String) -> R<()> {
    config::set_host(&host);
    R::success(())
}

/// 设置代理地址
///
/// Set proxy URL
#[tauri::command]
async fn set_proxy(proxy: String) -> R<()> {
    config::set_proxy(&proxy);
    R::success(())
}

/// 设置模式
///
/// Set mode
#[tauri::command]
async fn set_mode(mode: bool) -> R<()> {
    config::set_mode(mode);
    R::success(())
}

/// 关闭设置窗口
///
/// Close settings window
#[tauri::command]
async fn close_settings(app: tauri::AppHandle) -> R<()> {
    if let Some(window) = app.get_webview_window("settings") {
        let _ = window.close();
    }
    R::success(())
}

#[tokio::main]
async fn main() {
    // 全局初始化
    // Global initialization
    common::init().await;

    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|_, _, _| {}))
        .setup(setup::handler)
        .invoke_handler(tauri::generate_handler![
            copy,
            open,
            pin,
            unpin,
            untmp,
            theme,
            host,
            proxy,
            set_host,
            set_proxy,
            mode,
            set_mode,
            close_settings
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
