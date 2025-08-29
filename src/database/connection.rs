use anyhow::Result;
use std::path::Path;

pub fn get_database_path() -> Result<String> {
    let app_data_dir = dirs::data_local_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("alohopass");
    
    std::fs::create_dir_all(&app_data_dir)?;
    Ok(app_data_dir.join("alohopass.db").to_string_lossy().to_string())
}

pub fn open_database<P: AsRef<Path>>(path: P) -> rusqlite::Result<rusqlite::Connection> {
    rusqlite::Connection::open(path)
} 