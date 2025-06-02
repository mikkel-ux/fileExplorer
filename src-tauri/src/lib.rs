/* use tauri::{WebviewUrl, WebviewWindowBuilder}; */

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
//use tauri_plugin_fs::FsExt;
mod files;
mod gif_to_img;
mod search;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            search::search_files,
            search::search_test,
            files::get_dirs,
            files::get_path,
            files::get_files_dirs_in_dir,
            gif_to_img::first_frame_from_gif,
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

//if dnd is not working try using this again
/* .setup(|app| {
    tauri::WebviewWindowBuilder::new(
        app,
        "main",
        tauri::WebviewUrl::App("index.html".into()),
    )
    .title("File Explorer")
    .inner_size(1000.0, 500.0)
    .resizable(true)
    .decorations(false)
    .transparent(false)
    .disable_drag_drop_handler()
    .build()
    .expect("Failed to create main window");

    Ok(())
}) */
/* }) */
