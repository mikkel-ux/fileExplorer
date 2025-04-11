use dirs::{desktop_dir, document_dir, download_dir, home_dir, picture_dir, video_dir};
use std::fs;
use std::path::PathBuf;

#[tauri::command]
pub fn get_files() {
    let path = desktop_dir().unwrap();
    let files = fs::read_dir(path).unwrap();
    for file in files {
        let entry = file.unwrap();
        let metadata = entry.metadata().unwrap();
        println!("Name: {}", entry.file_name().to_str().unwrap());
        println!("Path: {}", entry.path().to_str().unwrap());
        println!("Size: {}", metadata.len());
        println!("Created: {:?}", metadata.created().unwrap());
        println!("Modified: {:?}", metadata.modified().unwrap());
        println!("Accessed: {:?}", metadata.is_dir());
        println!(" ");
    }
}

#[tauri::command]
pub fn get_dirs(path: String) -> Result<Vec<String>, String> {
    let dir_path = get_path(path)?;

    let entries = fs::read_dir(&dir_path).map_err(|e| format!("Error reading directory: {}", e))?;

    let mut paths = Vec::new();
    for entry in entries {
        if let Ok(entry) = entry {
            if entry.path().is_dir() {
                paths.push(entry.path().display().to_string().replace("\\", "/"));
            }
        }
    }
    Ok(paths)
}

#[tauri::command]
pub fn get_path(path: String) -> Result<String, String> {
    let dir_path = match path.to_lowercase().as_str() {
        "home" => home_dir().ok_or("Failed to get home dir".to_string()),
        "desktop" => desktop_dir().ok_or("Failed to get desktop dir".to_string()),
        "documents" => document_dir().ok_or("Failed to get documents dir".to_string()),
        "downloads" => download_dir().ok_or("Failed to get downloads dir".to_string()),
        "pictures" => picture_dir().ok_or("Failed to get pictures dir".to_string()),
        "videos" => video_dir().ok_or("Failed to get videos dir".to_string()),
        _ => Ok(PathBuf::from(path)),
    }?;

    Ok(dir_path.display().to_string().replace("\\", "/"))
}
