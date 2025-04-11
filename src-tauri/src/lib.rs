/* use tauri::{WebviewUrl, WebviewWindowBuilder}; */

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod files;
mod search;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            search::search_files,
            files::get_dirs,
            files::get_files,
            files::get_path,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/* .setup(|app| {
    WebviewWindowBuilder::new(app, "tab-1", WebviewUrl::App("index.html".into()))
        .title("Tab - home")
        .build()
        .expect("Failed to create tab-1 window");
    Ok(())
}) */
