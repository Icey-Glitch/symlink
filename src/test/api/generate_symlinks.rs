#[cfg(test)]

mod tests {
    use super::*;
    use std::fs::{create_dir, File};
    use std::io::Write;
    use tempfile::tempdir;

    #[test]

    fn test_generate_symlinks() {
        let dir = tempdir().unwrap();
        let symlink_root = dir.path().join("symlink_root");
        create_dir(&symlink_root).unwrap();
        let folder_path = dir.path().join("folder");
        create_dir(&folder_path).unwrap();
        let file_path = folder_path.join("file.txt");
        let mut file = File::create(&file_path).unwrap();
        file.write_all(b"test").unwrap();
        let config_path = dir.path().join("config.json");
        let mut file = File::create(&config_path).unwrap();
        file.write_all(
            format!(
                r#"{{
                    "folders": [
                        {{
                            "path": "{}",
                            "exclude": []
                        }}
                    ],
                    "symlink_root": "{}"
                }}"#,
                folder_path.to_str().unwrap(),
                symlink_root.to_str().unwrap()
            )
            .as_bytes(),
        )
        .unwrap();
        let config = load_config(&config_path.to_str().unwrap()).unwrap();

        generate_symlinks(&config).unwrap();

        let symlink_path = symlink_root.join("file.txt");
        assert!(symlink_path.exists());
        assert_eq!(symlink_path.read_link().unwrap(), file_path);
    }
}
