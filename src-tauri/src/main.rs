#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::fs::OpenOptions;
use std::io::Write;
use tauri::Manager;
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{ClickType, TrayIconBuilder},
};

use std::io;
use std::path::Path;
use std::path::PathBuf;

#[tauri::command]
async fn greet(
    request: tauri::ipc::Request<'_>,
) -> Result<(), String>  {
    if let tauri::ipc::InvokeBody::Raw(data) = request.body() {
        let path = PathBuf::from(request.headers().get("path").unwrap().to_str().unwrap());
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .map_err(|e| e.to_string())
            .unwrap();
        file.write_all(data).map_err(|e| e.to_string()).unwrap();
        Ok(())
    } else {
        return Err("No data was provided".to_string());
    }
}

fn main() {
    tauri::Builder::default()
		.plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
