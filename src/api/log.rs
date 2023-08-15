use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::Path;

pub struct Logger {
    file: Option<std::fs::File>,
}

impl Logger {
    pub fn new<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file = OpenOptions::new().create(true).append(true).open(path)?;
        Ok(Self { file: Some(file) })
    }

    pub fn log(&mut self, message: &str) -> io::Result<()> {
        let formatted_message = format!("{}\n", message);
        print!("{}", formatted_message);
        if let Some(file) = &mut self.file {
            file.write_all(formatted_message.as_bytes())?;
            file.flush()?;
        }
        Ok(())
    }
}

impl Drop for Logger {
    fn drop(&mut self) {
        if let Some(mut file) = self.file.take() {
            let _ = file.flush();
        }
    }
}
