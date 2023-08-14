use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs::{create_dir_all, remove_file, write};
use std::path::{Path, PathBuf};
use tempfile;
use walkdir::WalkDir;

// Define a struct to hold the configuration for each folder to symlink
#[derive(Debug, Serialize, Deserialize)]
struct FolderConfig {
    path: PathBuf,
    exclude: Vec<String>,
}

// Define a struct to hold the entire API configuration
#[derive(Debug, Serialize, Deserialize)]
struct ApiConfig {
    folders: Vec<FolderConfig>,
    symlink_root: PathBuf,
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

            // Check if the symlink already exists
            if symlinks.contains(&symlink_path.to_str().unwrap().to_owned()) {
                continue;
            }

            // Check if the file should be excluded based on the folder's exclusion patterns
            let excluded = folder.exclude.iter().any(|exclusion| {
                let re = Regex::new(exclusion).unwrap();
                re.is_match(entry_path.to_str().unwrap())
            });

            // Create a symlink for the file if it exists and is not excluded
            if entry_path.exists() && !excluded {
                create_dir_all(&symlink_path.parent().unwrap())?;
                #[cfg(windows)]
                std::os::windows::fs::symlink_file(&entry_path, &symlink_path)?;
                #[cfg(unix)]
                std::os::unix::fs::symlink(&entry_path, &symlink_path)?;
                symlinks.push(symlink_path.to_str().unwrap().to_owned());
            }
        }
    }

    // Remove duplicate symlinks and write them to a file
    symlinks.sort();
    symlinks.dedup();
    for symlink in &symlinks {
        symlinks_str.push_str(&format!("{}\n", symlink));
    }
    write("symlinks.txt", symlinks_str)?;

    Ok(())
}

// Remove duplicate symlinks and excluded symlinks from the list of symlinks
fn remove_duplicate_symlinks(
    symlinks: &mut HashSet<String>,
    symlinks_str: &mut String,
    exclude: &[PathBuf],
) {
    let mut new_symlinks_str = symlinks_str.to_owned();
    for subdir in exclude {
        let excluded_symlinks: Vec<&str> = new_symlinks_str
            .lines()
            .filter(|line| line.contains(subdir.to_str().unwrap()))
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
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_symlinks() {
        // Create a temporary directory for the test
        let temp_dir = tempfile::tempdir().unwrap();

        // Create some test files in the temporary directory
        let test_file1_path = temp_dir.path().join("test_file1.txt");
        let test_file2_path = temp_dir.path().join("test_file2.txt");
        let test_file3_path = temp_dir.path().join("test_file3.txt");
        std::fs::write(&test_file1_path, "test file 1").unwrap();
        std::fs::write(&test_file2_path, "test file 2").unwrap();
        std::fs::write(&test_file3_path, "test file 3").unwrap();

        // Create a test API configuration
        let api_config = ApiConfig {
            folders: vec![FolderConfig {
                path: temp_dir.path().to_owned(),
                exclude: vec![],
            }],
            symlink_root: PathBuf::from("tests/symlinks"),
        };

        // Generate symlinks for the test API configuration
        generate_symlinks(&api_config).unwrap();

        // Check that the symlinks were generated correctly
        let symlink_file1_path = PathBuf::from("tests/symlinks/test_file1.txt");
        let symlink_file2_path = PathBuf::from("tests/symlinks/test_file2.txt");
        let symlink_file3_path = PathBuf::from("tests/symlinks/test_file3.txt");
        assert!(symlink_file1_path.exists());
        assert!(symlink_file2_path.exists());
        assert!(symlink_file3_path.exists());
        assert_eq!(
            std::fs::read_to_string(&symlink_file1_path).unwrap(),
            "test file 1"
        );
        assert_eq!(
            std::fs::read_to_string(&symlink_file2_path).unwrap(),
            "test file 2"
        );
        assert_eq!(
            std::fs::read_to_string(&symlink_file3_path).unwrap(),
            "test file 3"
        );

        // Clean up the test files and symlinks
        std::fs::remove_file(&test_file1_path).unwrap();
        std::fs::remove_file(&test_file2_path).unwrap();
        std::fs::remove_file(&test_file3_path).unwrap();
        std::fs::remove_file(&symlink_file1_path).unwrap();
        std::fs::remove_file(&symlink_file2_path).unwrap();
        std::fs::remove_file(&symlink_file3_path).unwrap();
    }

    #[test]
    fn test_generate_symlinks_with_exclusions() {
        // Create a temporary directory for the test
        let temp_dir = tempfile::tempdir().unwrap();

        // Create some test files in the temporary directory
        let test_file1_path = temp_dir.path().join("test_file1.txt");
        let test_file2_path = temp_dir.path().join("test_file2.txt");
        let test_file3_path = temp_dir.path().join("test_file3.txt");
        let excluded_file_path = temp_dir.path().join("excluded_file.txt");
        std::fs::write(&test_file1_path, "test file 1").unwrap();
        std::fs::write(&test_file2_path, "test file 2").unwrap();
        std::fs::write(&test_file3_path, "test file 3").unwrap();
        std::fs::write(&excluded_file_path, "excluded file").unwrap();

        // Create a test API configuration with an exclusion pattern
        let api_config = ApiConfig {
            folders: vec![FolderConfig {
                path: temp_dir.path().to_owned(),
                exclude: vec!["excluded_file".to_owned()],
            }],
            symlink_root: PathBuf::from("tests/symlinks"),
        };

        // Generate symlinks for the test API configuration
        generate_symlinks(&api_config).unwrap();

        // Check that the symlinks were generated correctly
        let symlink_file1_path = PathBuf::from("tests/symlinks/test_file1.txt");
        let symlink_file2_path = PathBuf::from("tests/symlinks/test_file2.txt");
        let symlink_file3_path = PathBuf::from("tests/symlinks/test_file3.txt");
        let symlink_excluded_file_path = PathBuf::from("tests/symlinks/excluded_file.txt");
        assert!(symlink_file1_path.exists());
        assert!(symlink_file2_path.exists());
        assert!(symlink_file3_path.exists());
        assert!(!symlink_excluded_file_path.exists());
        assert_eq!(
            std::fs::read_to_string(&symlink_file1_path).unwrap(),
            "test file 1"
        );
        assert_eq!(
            std::fs::read_to_string(&symlink_file2_path).unwrap(),
            "test file 2"
        );
        assert_eq!(
            std::fs::read_to_string(&symlink_file3_path).unwrap(),
            "test file 3"
        );

        // Clean up the test files and symlinks
        std::fs::remove_file(&test_file1_path).unwrap();
        std::fs::remove_file(&test_file2_path).unwrap();
        std::fs::remove_file(&test_file3_path).unwrap();
        std::fs::remove_file(&excluded_file_path).unwrap();
        std::fs::remove_file(&symlink_file1_path).unwrap();
        std::fs::remove_file(&symlink_file2_path).unwrap();
        std::fs::remove_file(&symlink_file3_path).unwrap();
    }
}
