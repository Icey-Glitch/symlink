use serde::{Deserialize, Serialize};
use std::fs::read_to_string;
use std::path::PathBuf;

// Define a struct to hold the configuration for each folder to symlink
#[derive(Debug, Serialize, Deserialize)]
pub struct FolderConfig {
    pub path: PathBuf,
    pub exclude: Vec<String>,
}

// Define a struct to hold the entire API configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiConfig {
    pub folders: Vec<FolderConfig>,
    pub symlink_root: PathBuf,
}

// Load the API configuration from a JSON file
pub fn load_config(config_path: &str) -> Result<ApiConfig, Box<dyn std::error::Error>> {
    let config_str = read_to_string(config_path)?;
    let config: ApiConfig = serde_json::from_str(&config_str)?;
    Ok(config)
}
