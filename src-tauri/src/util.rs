use std::{path::PathBuf, sync::atomic::Ordering, time::SystemTime};

use rdev::Key;
use selection::get_text;
use tauri::{AppHandle, Emitter, Manager};

use crate::{clip, common::SIMULATION, config};

fn should_fallback_to_clipboard(fallback: bool) -> bool {
    fallback || cfg!(windows)
}

/// 模拟获取复制文本
///
/// Simulate getting copy text
pub fn content(fallback: bool) -> String {
    // 更改模拟标识, 按键监听
    // Change the simulation flag, trigger the key
    SIMULATION.store(true, Ordering::SeqCst);
    // 模拟复制获取文本
    // Simulate copy and get text
    let content = get_text();
    // 重置模拟标识
    // Reset the simulation flag
    SIMULATION.store(false, Ordering::SeqCst);

    if content.is_empty() && should_fallback_to_clipboard(fallback) {
        // 当前环境下的 selection 处理可能为空，Windows 下需要兜底到剪贴板以保证划词翻译可用
        // The selection backend can be empty in some environments; on Windows we should fallback
        // to the system clipboard so word selection translations still work.
        match clip::get() {
            Ok(copy) => copy,
            Err(e) => {
                println!("error occurred while getting clipboard content: {:?}", e);
                String::default()
            }
        }
    } else {
        content
    }
}

#[cfg(test)]
mod tests {
    use super::should_fallback_to_clipboard;

    #[test]
    fn windows_uses_clipboard_fallback_for_empty_selection() {
        if cfg!(windows) {
            assert!(should_fallback_to_clipboard(false));
        }
    }

    #[test]
    fn native_fallback_stays_opt_in_on_other_platforms() {
        if !cfg!(windows) {
            assert!(!should_fallback_to_clipboard(false));
        }
    }
}

/// 获取当前时间
///
/// Get current time
pub fn now() -> u64 {
    let now = SystemTime::now();
    let timestamp = now
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Time went backwards");
    timestamp.as_millis() as u64
}

/// 获取可执行文件位置
///
/// Get executable file path
pub fn get_exe_dir() -> PathBuf {
    std::env::current_exe()
        .expect("Failed to get current executable path")
        .parent()
        .expect("Failed to get current executable parent directory")
        .to_path_buf()
}

/// 获取当前指定快捷键
///
/// Get current key
pub fn key() -> Key {
    let key = config::key();
    match key {
        1 => Key::ControlLeft,
        2 => Key::CapsLock,
        _ => Key::ShiftLeft,
    }
}

/// 修改主题
///
/// Change theme
pub fn theme(app: &AppHandle, theme: &str) {
    config::set_theme(theme);
    let panel = app.get_webview_window("panel").unwrap();
    let _ = panel.emit("theme", theme);
}
