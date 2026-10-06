// Learn more about Tauri commands at https://tauri.app/develop/calling-rus
mod algo;
mod crypto;
mod local;
mod tests;
mod types;

use types::UnitModel;
use tauri::Manager;
use std::sync::Mutex;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(Default, Debug)]
pub struct AppState {
    pub model: UnitModel
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(Mutex::new(AppState::default()))
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![local::enter_filepath])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
