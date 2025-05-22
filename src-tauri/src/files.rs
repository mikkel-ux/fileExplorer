use dirs::{desktop_dir, document_dir, download_dir, home_dir, picture_dir, video_dir};
use serde::Serialize;
use std::fs::{self, DirEntry, Permissions};
use std::os::windows::fs::MetadataExt;
use std::path::Path;
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

//============
//test
//============
/* #[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct FileData {
    number: i32,
}

impl FileData {
    fn boo() -> String {
        "boo".to_string()
    }

    fn foo(&mut self) {
        self.number += 42;
    }

    fn get_number(&self) -> i32 {
        self.number
    }
}

#[test]
fn test_impl() {
    let mut file_data = FileData { number: 0 };
    file_data.foo();
    assert_eq!(file_data.get_number(), 42);
    assert_eq!(FileData::boo(), "boo");
} */
#[derive(Debug, Clone, Serialize)]
pub struct FileData {
    name: String,
    path: String,
    size: u64,
    extension: String,
    created: std::time::SystemTime,
    modified: std::time::SystemTime,
    accessed: std::time::SystemTime,
    is_dir: bool,
    permissions: u32,
    is_hidden: bool,
    is_read_only: bool,
}

impl FileData {
    pub fn from_path(path: &Path) -> Option<Self> {
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
        let is_dir = metadata.is_dir();
        let attrs: u32 = metadata.file_attributes();
        let permissions = attrs;
        let is_hidden = attrs & 0x2 != 0; // FILE_ATTRIBUTE_HIDDEN
        let is_read_only = attrs & 0x1 != 0; // FILE_ATTRIBUTE_READONLY
        let size = metadata.len();

        Some(Self {
            name,
            path: path.to_string_lossy().to_string(),
            size,
            extension,
            created,
            modified,
            accessed,
            is_dir,
            permissions,
            is_hidden,
            is_read_only,
        })
    }
}

#[test]
fn idk() {
    let path = "C:/Users/rumbo/.testFoulderForFE";
    let test = FileData::from_path(Path::new(path));
    println!("test: {:?}", test);
    println!(" ");

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
pub fn get_files_in_dirs(path: String) -> Result<Vec<FileData>, String> {
    let mut file_data_list = Vec::new();
    let files = fs::read_dir(path).map_err(|e| format!("Error reading directory: {}", e))?;
    for file in files {
        let entry = file.map_err(|e| format!("Error reading entry: {}", e))?;
        let path = entry.path();
        if path.is_dir() == false {
            let file_data = FileData::from_path(&path).ok_or("Failed to get file data")?;
            file_data_list.push(file_data);
        }
    }
    Ok(file_data_list)
}
