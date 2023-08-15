use std::fs::OpenOptions;
use std::io::{self, BufWriter, Write};
use std::path::Path;

pub struct Logger {
    file: Option<BufWriter<std::fs::File>>,
}

impl Logger {
    pub fn new<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .map(BufWriter::new)?;
        Ok(Self { file: Some(file) })
    }

    pub fn log(&mut self, message: &str) -> io::Result<()> {
        writeln!(io::stdout(), "{}", message)?;
        if let Some(file) = self.file.as_mut() {
            writeln!(file, "{}", message)?;
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
