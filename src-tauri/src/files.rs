use base64::{engine::general_purpose, Engine as _};
use chrono::{DateTime, Local};
use dirs::{desktop_dir, document_dir, download_dir, home_dir, picture_dir, video_dir};
use image::codecs::gif::GifDecoder;
use image::{AnimationDecoder, DynamicImage, ImageFormat};
use serde::Serialize;
use std::fs;
use std::fs::File;
use std::io::{BufReader, Cursor};
use std::path::Path;
use std::path::PathBuf;

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

pub fn format_size(bytes: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit = 0;

    while size >= 1024.0 && unit < UNITS.len() - 1 {
        size /= 1024.0;
        unit += 1;
    }

    format!("{:.2} {}", size, UNITS[unit])
}

pub fn first_frame_from_gif(path: String) -> Result<String, String> {
    let file = File::open(&path).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);

    let decoder = GifDecoder::new(reader).map_err(|e| e.to_string())?;

    let mut frames = decoder.into_frames();
    let frame_resoult = frames.next().ok_or("No frames found")?;
    let frame = frame_resoult.map_err(|e| e.to_string())?;

    let image = DynamicImage::ImageRgba8(frame.into_buffer());
    let mut buf = Vec::new();
    image
        .write_to(&mut Cursor::new(&mut buf), ImageFormat::Png)
        .map_err(|e| e.to_string())?;
    let encoded = general_purpose::STANDARD.encode(&buf);
    Ok(encoded)
}

#[derive(Debug, Clone, Serialize)]
pub struct FileData {
    name: String,
    path: String,
    size: String,
    extension: String,
    created: String,
    modified: String,
    accessed: String,
    #[serde(rename = "type")]
    file_type: String,
    permissions: u32,
    #[serde(rename = "isHidden")]
    is_hidden: bool,
    #[serde(rename = "isReadOnly")]
    is_read_only: bool,
    base64: Option<String>,
}

impl FileData {
    pub fn from_path(path: &Path) -> Option<Self> {
        let format = "%d-%m-%Y %H:%M:%S";
        let metadata = fs::metadata(path).ok()?;
        let name = path.file_name()?.to_str()?.to_string();
        let extension = path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_string();

        let created = metadata
            .created()
            .unwrap_or(std::time::SystemTime::UNIX_EPOCH);
        let modified = metadata
            .modified()
            .unwrap_or(std::time::SystemTime::UNIX_EPOCH);
        let accessed = metadata
            .accessed()
            .unwrap_or(std::time::SystemTime::UNIX_EPOCH);

        let created_date = DateTime::<Local>::from(created).format(format).to_string();
        let modified_date = DateTime::<Local>::from(modified).format(format).to_string();
        let accessed_date = DateTime::<Local>::from(accessed).format(format).to_string();

        let size = format_size(metadata.len());
        /* let is_dir = metadata.is_dir(); */
        let file_type = if metadata.is_dir() {
            "folder".to_string()
        } else {
            "file".to_string()
        };

        #[cfg(target_family = "windows")]
        let (permissions, is_hidden, is_read_only) = {
            use std::os::windows::fs::MetadataExt;
            let attrs = metadata.file_attributes();
            (attrs, attrs & 0x2 != 0, attrs & 0x1 != 0)
        };

        #[cfg(target_family = "unix")]
        let (permissions, is_hidden, is_read_only) = {
            use std::os::unix::fs::MetadataExt;
            let mode = metadata.mode();
            let is_hidden = path
                .file_name()
                .map_or(false, |f| f.to_string_lossy().starts_with('.'));
            let is_read_only = metadata.permissions().readonly();
            (mode, is_hidden, is_read_only)
        };

        let first_frame = if extension.eq_ignore_ascii_case("gif") {
            match first_frame_from_gif(path.to_string_lossy().to_string()) {
                Ok(frame) => Some(frame),
                Err(_) => None,
            }
        } else {
            None
        };

        Some(Self {
            name,
            path: path.to_string_lossy().to_string(),
            size,
            extension,
            created: created_date,
            modified: modified_date,
            accessed: accessed_date,
            file_type,
            permissions,
            is_hidden,
            is_read_only,
            base64: first_frame,
        })
    }
}

#[tauri::command]
pub fn get_files_dirs_in_dir(path: String) -> Result<Vec<FileData>, String> {
    let mut file_data_list = Vec::new();
    let files = fs::read_dir(path).map_err(|e| format!("Error reading directory: {}", e))?;
    for file in files {
        let entry = file.map_err(|e| format!("Error reading entry: {}", e))?;
        let path = entry.path();
        let file_data = FileData::from_path(&path).ok_or("Failed to get file data")?;
        file_data_list.push(file_data);
    }
    Ok(file_data_list)
}

#[test]
fn idk() {
    let path = "C:/Users/rumbo/.testFoulderForFE";
    let files = fs::read_dir(path).unwrap();
    for file in files {
        let entry = file.unwrap();
        let path = entry.path();
        let foo = FileData::from_path(&path);
        println!("foo: {:?}", foo);
        println!(" ");
    }
    println!(" ");
}

#[tauri::command]
pub fn open_in_default_app(path: String) -> Result<(), String> {
    open::that(path).map_err(|e| format!("Failed to open file: {}", e))
}
