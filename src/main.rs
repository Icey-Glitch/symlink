use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs::{create_dir_all, remove_file, write};
use std::path::Path;
use walkdir::WalkDir;

// Define a struct to hold the configuration for each folder to symlink
#[derive(Debug, Serialize, Deserialize)]
struct FolderConfig {
    path: String,
    exclude: Vec<String>,
}

// Define a struct to hold the entire API configuration
#[derive(Debug, Serialize, Deserialize)]
struct ApiConfig {
    folders: Vec<FolderConfig>,
    symlink_root: String,
}

// Load the API configuration from a JSON file
fn load_config(config_path: &str) -> Result<ApiConfig, Box<dyn std::error::Error>> {
    let config_str = std::fs::read_to_string(config_path)?;
    let config: ApiConfig = serde_json::from_str(&config_str)?;
    Ok(config)
}

// Generate symlinks for all files in the specified folders
fn generate_symlinks(api_config: &ApiConfig) -> Result<(), Box<dyn std::error::Error>> {
    let symlink_root = api_config.symlink_root.clone();
    let mut symlinks: Vec<String> = vec![];
    let mut symlinks_str = String::new();

    // Iterate over each folder in the API configuration
    for folder in &api_config.folders {
        // Iterate over each file in the folder
        for entry in WalkDir::new(&folder.path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| !e.file_type().is_dir())
        {
            let entry_path = entry.path();
            let _entry_name = entry_path.file_name().unwrap().to_str().unwrap();
            let symlink_path = Path::new(&symlink_root)
                .join(entry_path.strip_prefix(&folder.path).unwrap().to_owned());

            // Remove any existing symlink at the symlink path
            if symlink_path.exists() {
                remove_file(&symlink_path)?;
            }

            // Check if the file should be excluded based on the folder's exclusion patterns
            let excluded = folder.exclude.iter().any(|exclusion| {
                let re = Regex::new(exclusion).unwrap();
                re.is_match(entry_path.to_str().unwrap())
            });

            // Create a symlink for the file if it's not excluded
            if !excluded {
                create_dir_all(&symlink_path.parent().unwrap())?;
                std::os::windows::fs::symlink_file(&entry_path, &symlink_path)?;
                symlinks.push(symlink_path.to_str().unwrap().to_owned());
            }
        }
    }

    // Remove duplicate symlinks and write them to a file
    symlinks.sort();
    symlinks.dedup();
    remove_duplicate_symlinks(
        &mut symlinks,
        &mut symlinks_str,
        &api_config
            .folders
            .iter()
            .map(|f| f.path.clone())
            .collect::<Vec<String>>(),
    );
    write("symlinks.txt", symlinks_str)?;

    Ok(())
}

// Remove duplicate symlinks and excluded symlinks from the list of symlinks
fn remove_duplicate_symlinks(
    symlinks: &mut Vec<String>,
    symlinks_str: &mut String,
    exclude: &[String],
) {
    let mut new_symlinks_str = symlinks_str.to_owned();
    for subdir in exclude {
        let excluded_symlinks: Vec<&str> = new_symlinks_str
            .lines()
            .filter(|line| line.contains(subdir))
            .collect();
        let mut temp_str = new_symlinks_str.clone();
        for excluded_symlink in excluded_symlinks {
            temp_str = temp_str.replace(excluded_symlink, "");
        }
        new_symlinks_str = temp_str;
    }
    *symlinks_str = new_symlinks_str.clone();

    symlinks.retain(|symlink| {
        new_symlinks_str
            .clone()
            .lines()
            .any(|line| line.contains(symlink))
    });
}

// Entry point of the program
fn main() {
    let config_path = "config.json";

    // Load the API configuration from the JSON file
    let api_config = match load_config(config_path) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error loading config: {}", e);
            return;
        }
    };

    // Generate symlinks for all files in the specified folders
    match generate_symlinks(&api_config) {
        Ok(_) => println!("Symlinks generated successfully."),
        Err(e) => eprintln!("Error generating symlinks: {}", e),
    }
}
