use std::fs::{self, File};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};

pub fn read_lines<P: AsRef<Path>>(filename: P) -> std::io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line?);
    }
    Ok(lines)
}

pub fn write_lines<P: AsRef<Path>>(filename: P, lines: &[String]) -> std::io::Result<()> {
    let file = File::create(filename)?;
    let mut writer = BufWriter::new(file);
    for line in lines {
        writeln!(writer, "{}", line)?;
    }
    Ok(())
}

pub fn remove_file_if_exists<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
    if path.as_ref().exists() {
        fs::remove_file(path)?;
    }
    Ok(())
}

pub fn create_dir_all<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
    fs::create_dir_all(path)?;
    Ok(())
}

pub fn copy_file<P: AsRef<Path>>(src: P, dst: P) -> std::io::Result<()> {
    fs::copy(src, dst)?;
    Ok(())
}

pub fn move_file<P: AsRef<Path>>(src: P, dst: P) -> std::io::Result<()> {
    fs::rename(src, dst)?;
    Ok(())
}

pub fn symlink_file<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> std::io::Result<()> {
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(src, dst)?;
    }
    #[cfg(windows)]
    {
        std::os::windows::fs::symlink_file(src, dst)?;
    }
    Ok(())
}
