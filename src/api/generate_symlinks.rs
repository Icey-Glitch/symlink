use crate::api::config::ApiConfig;
use crate::api::log::Logger;
use regex::Regex;
use std::collections::HashSet;
use std::fs::{create_dir_all, read_link, remove_file};
use std::os::windows::fs::symlink_file as symlink;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn generate_symlinks(
    api_config: &ApiConfig,
    logger: &mut Logger,
) -> Result<(), Box<dyn std::error::Error>> {
    let symlink_root = api_config.symlink_root.clone();
    let mut symlinks: HashSet<PathBuf> = HashSet::new();

    for folder in &api_config.folders {
        for entry in WalkDir::new(&folder.path)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file())
        {
            let entry_path = entry.path();
            let symlink_path =
                PathBuf::from(&symlink_root).join(entry_path.strip_prefix(&folder.path).unwrap());

            if symlinks.contains(&symlink_path) {
                continue;
            }

            let excluded = folder.exclude.iter().any(|exclusion| {
                let re = Regex::new(exclusion).unwrap();
                re.is_match(entry_path.to_str().unwrap())
            });

            if entry_path.exists() && !excluded {
                create_dir_all(&symlink_path.parent().unwrap())?;

                if symlink_path.exists()
                    && symlink_path.symlink_metadata()?.file_type().is_symlink()
                {
                    let existing_link = read_link(&symlink_path)?;
                    remove_file(&symlink_path)?;
                    symlink(entry_path, &symlink_path)?;
                    logger.log(&format!(
                        "Replaced existing symlink: {} -> {}",
                        symlink_path.display(),
                        existing_link.display()
                    ))?;
                } else if symlink_path.exists() {
                    let mut new_path = symlink_path.clone();
                    new_path.set_extension("conflict");
                    logger.log(&format!(
                        "Conflict: {} already exists, moving to {}",
                        symlink_path.display(),
                        new_path.display()
                    ))?;
                    std::fs::rename(&symlink_path, &new_path)?;
                    symlink(entry_path, &symlink_path)?;
                } else {
                    symlink(entry_path, &symlink_path)?;
                }

                symlinks.insert(symlink_path);
            }
        }
    }

    Ok(())
}
