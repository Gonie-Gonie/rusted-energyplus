//! Local binary InputFile owner for IOFiles65–103/134–147.
use super::Error;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::Path;

pub(super) fn file_exists(path: &Path) -> Result<bool, Error> {
    // FileSystem289–292: exists && !is_directory, including short circuit.
    Ok(path.try_exists()? && !fs::metadata(path)?.is_dir())
}

pub(super) struct StatInput {
    // The actual local fstream allocation is distinct from its physical open.
    stream: Option<BufReader<File>>,
    _file_size: u64,
    eof: bool,
    failed: bool,
}
impl StatInput {
    pub(super) fn open(path: &Path) -> Result<Self, Error> {
        // InputFile::open first obtains filesystem::file_size, then constructs
        // the binary input fstream. A failed open has an allocated failed stream;
        // Native next calls ShowFatalError71, an explicit unrepresented frontier.
        let mut owner = Self {
            stream: None,
            _file_size: 0,
            eof: false,
            failed: false,
        };
        owner._file_size = fs::metadata(path)?.len();
        match File::open(path) {
            Ok(file) => owner.stream = Some(BufReader::new(file)),
            Err(error) => {
                owner.failed = true;
                // Retain the real OS cause; do not invoke legacy fatal flags.
                return Err(Error::Io(error));
            }
        }
        Ok(owner)
    }
    pub(super) fn good(&self) -> bool {
        self.stream.is_some() && !self.eof && !self.failed
    }
    pub(super) fn read_line(&mut self) -> Result<Vec<u8>, Error> {
        let mut line = Vec::new();
        let stream = self.stream.as_mut().ok_or(Error::Unavailable(
            "actual STAT InputFile fstream missing before getline",
        ))?;
        let read = stream.read_until(b'\n', &mut line);
        let count = match read {
            Ok(count) => count,
            Err(error) => {
                self.failed = true;
                return Err(Error::Io(error));
            }
        };
        if count == 0 {
            self.eof = true;
            self.failed = true;
            return Ok(Vec::new());
        }
        if line.last() == Some(&b'\n') {
            line.pop();
        } else {
            // Successful final unterminated line sets eofbit without failbit.
            self.eof = true;
        }
        if line.last() == Some(&b'\r') {
            line.pop();
        }
        Ok(line)
    }
}
