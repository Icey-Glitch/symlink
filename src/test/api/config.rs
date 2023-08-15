#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_load_config() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("config.json");
        let mut file = File::create(&config_path).unwrap();
        file.write_all(
            r#"{
                "folders": [
                    {
                        "path": "/path/to/folder",
                        "exclude": ["\\.txt$"]
                    }
                ],
                "symlink_root": "/path/to/symlink/root"
            }"#
            .as_bytes(),
        )
        .unwrap();

        let config = load_config(&config_path.to_str().unwrap()).unwrap();

        assert_eq!(config.folders.len(), 1);
        assert_eq!(config.folders[0].path, PathBuf::from("/path/to/folder"));
        assert_eq!(config.folders[0].exclude.len(), 1);
        assert_eq!(config.folders[0].exclude[0], "\\.txt$");
        assert_eq!(config.symlink_root, PathBuf::from("/path/to/symlink/root"));
    }
}
