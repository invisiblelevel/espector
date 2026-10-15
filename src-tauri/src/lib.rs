//! Точка входа Tauri-приложения ESPector.

mod commands;
mod core;
mod models;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_os::init())
        .invoke_handler(tauri::generate_handler![
            commands::scan_folder,
            commands::save_report,
            commands::build_merged_ini,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}