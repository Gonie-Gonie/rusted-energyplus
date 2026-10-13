//! Sole actual MTD stream; process settings remain on their real shared owner.
use super::SourceOutputFileHost;
use std::fs::{File, OpenOptions};
use std::io;
use std::path::PathBuf;

/// Sole real MTD stream for the reached OutputProcessor initialization.
#[derive(Debug)]
pub struct SourceMtdFileHost {
    path: PathBuf,
    file: Option<File>,
}
impl SourceMtdFileHost {
    /// Retain the caller's chosen path without opening an unreached file.
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            path: path.into(),
            file: None,
        }
    }
    /// Flush an actually opened MTD without opening an unreached file.
    ///
    /// # Errors
    /// Returns the actual retained stream's OS error.
    pub fn flush_reached(&mut self) -> io::Result<()> {
        use std::io::Write;
        if let Some(file) = &mut self.file {
            file.flush()?;
        }
        Ok(())
    }
}
impl SourceOutputFileHost for SourceMtdFileHost {
    fn ensure_mtd_open(&mut self) -> io::Result<()> {
        if self.file.is_none() {
            self.file = Some(
                OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(&self.path)?,
            );
        }
        Ok(())
    }
}
