mod crypto;
mod error_handler;
mod commands;
mod models;
mod algo;
mod local_store;

use tauri::Manager;
use serde::Deserialize;
use std::sync::Mutex;
use tauri::EventTarget::App;
use crate::models::UnitModel;
use crate::commands::{get_open_filepath, load_file, get_current_model};

#[derive(Default)]
pub struct AppState {
    pub current_model: UnitModel
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(Mutex::new(AppState::default()));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_open_filepath, load_file, get_current_model])
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
