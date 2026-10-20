use std::fs;
use std::path::PathBuf;
use crate::models::StoreData;

pub fn get_storage_dir() -> PathBuf {
    if let Some(mut dir) = dirs::data_dir() {
        dir.push("com.jasonvibe.envswitch");
        dir
    } else {
        let mut dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        dir.push(".envswitch");
        dir
    }
}

pub fn get_store_file_path() -> PathBuf {
    let mut path = get_storage_dir();
    path.push("store.json");
    path
}

pub fn load_store() -> Result<StoreData, String> {
    let file_path = get_store_file_path();
    if !file_path.exists() {
        let default_data = StoreData::default();
        save_store(&default_data)?;
        return Ok(default_data);
    }

    let content = fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read store file: {}", e))?;
    
    let data: StoreData = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse store data: {}", e))?;

    Ok(data)
}

pub fn save_store(data: &StoreData) -> Result<(), String> {
    let dir = get_storage_dir();
    if !dir.exists() {
        fs::create_dir_all(&dir)
            .map_err(|e| format!("Failed to create storage directory: {}", e))?;
    }

    let file_path = get_store_file_path();
    let json_str = serde_json::to_string_pretty(data)
        .map_err(|e| format!("Failed to serialize store data: {}", e))?;

    fs::write(&file_path, json_str)
        .map_err(|e| format!("Failed to write store file: {}", e))?;

    Ok(())
}
