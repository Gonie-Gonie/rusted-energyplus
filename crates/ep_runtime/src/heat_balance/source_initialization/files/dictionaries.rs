//! Actual dictionary-only InputOutputFile stream alternatives and lifecycle.
use super::SourceInitializationFiles;
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::source_initialization::SourceInitializationError as Error;
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::path::PathBuf;

#[derive(Debug)]
enum SourceDictionaryBuffer {
    PhysicalFile(File),
    // Native still owns the failed fstream object when physical open fails.
    FailedPhysicalOpen,
    // std::iostream(nullptr) retains a non-null object with badbit initially set.
    NativeNullBuffer { bad: bool },
}
#[derive(Debug)]
pub(super) struct SourceDictionaryStream {
    path: PathBuf,
    os: Option<SourceDictionaryBuffer>,
    print_to_dev_null: bool,
    failed: bool,
}
impl SourceDictionaryStream {
    pub(super) fn new(path: PathBuf) -> Self {
        Self {
            path,
            os: None,
            print_to_dev_null: false,
            failed: false,
        }
    }
    pub(super) fn is_open(&self) -> bool {
        self.os.is_some()
    }
    fn good(&self) -> bool {
        // Exact IOFiles223-231 special good() test on the actual null object.
        if self.print_to_dev_null
            && matches!(
                self.os,
                Some(SourceDictionaryBuffer::NativeNullBuffer { bad: true })
            )
        {
            return true;
        }
        match &self.os {
            Some(SourceDictionaryBuffer::PhysicalFile(_)) => !self.failed,
            Some(SourceDictionaryBuffer::NativeNullBuffer { bad }) => !bad && !self.failed,
            Some(SourceDictionaryBuffer::FailedPhysicalOpen) | None => false,
        }
    }
    fn open(&mut self, output_to_file: bool) -> io::Result<()> {
        if !output_to_file {
            self.os = Some(SourceDictionaryBuffer::NativeNullBuffer { bad: true });
            self.failed = true;
            self.print_to_dev_null = true;
            return Ok(());
        }
        let result = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.path);
        match result {
            Ok(file) => {
                self.os = Some(SourceDictionaryBuffer::PhysicalFile(file));
                self.failed = false;
                self.print_to_dev_null = false;
                Ok(())
            }
            Err(source) => {
                self.os = Some(SourceDictionaryBuffer::FailedPhysicalOpen);
                self.failed = true;
                self.print_to_dev_null = false;
                Err(source)
            }
        }
    }
    fn ensure(
        &mut self,
        output_to_file: bool,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error> {
        if !self.good() {
            if let Err(source) = self.open(output_to_file) {
                let message = format!(
                    "ProduceRDDMDD: Could not open file {} for output (write).",
                    self.path.display()
                );
                return match backend.show_fatal_error(&message) {
                    Ok(never) => match never {},
                    Err(stop) => Err(Error::Fatal { source, stop }),
                };
            }
        }
        Ok(())
    }
    fn write_formatted(&mut self, bytes: &[u8]) -> io::Result<()> {
        // IOFiles698-707 formats before os.write. All callers already formatted
        // these bytes; only the real ostream's bad/null buffer suppresses writing.
        let result = match self.os.as_mut() {
            Some(SourceDictionaryBuffer::NativeNullBuffer { bad: true }) => Ok(()),
            Some(SourceDictionaryBuffer::PhysicalFile(_)) if self.failed => Ok(()),
            Some(SourceDictionaryBuffer::PhysicalFile(file)) => {
                file.write_all(super::native_text_bytes(bytes).as_ref())
            }
            Some(SourceDictionaryBuffer::FailedPhysicalOpen) => Ok(()),
            Some(SourceDictionaryBuffer::NativeNullBuffer { bad: false }) => {
                return Err(io::Error::new(
                    io::ErrorKind::Unsupported,
                    "actual dictionary null-buffer badbit clear is not reached by these stream operations",
                ));
            }
            None => {
                return Err(io::Error::new(
                    io::ErrorKind::NotConnected,
                    "actual dictionary stream is not open",
                ));
            }
        };
        if result.is_err() {
            self.failed = true;
        }
        result
    }
    pub(super) fn flush(&mut self) -> io::Result<()> {
        let result = match self.os.as_mut() {
            Some(SourceDictionaryBuffer::PhysicalFile(file)) if !self.failed => file.flush(),
            _ => Ok(()),
        };
        if result.is_err() {
            self.failed = true;
        }
        result
    }
    fn close(&mut self) {
        self.os = None; // Native234-237 only resets the owned stream pointer.
        self.failed = false; // State belonged to that discarded stream object.
        // Native print_to_dev_null is retained until the next real open writes it.
    }
}
impl SourceInitializationFiles {
    pub(crate) fn ensure_source_rdd_open(
        &mut self,
        output_to_file: bool,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error> {
        self.dictionaries[0].ensure(output_to_file, backend)
    }
    pub(crate) fn ensure_source_mdd_open(
        &mut self,
        output_to_file: bool,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error> {
        self.dictionaries[1].ensure(output_to_file, backend)
    }
    pub(crate) fn write_source_rdd(&mut self, bytes: &[u8]) -> io::Result<()> {
        self.dictionaries[0].write_formatted(bytes)
    }
    pub(crate) fn write_source_mdd(&mut self, bytes: &[u8]) -> io::Result<()> {
        self.dictionaries[1].write_formatted(bytes)
    }
    pub(crate) fn close_source_rdd(&mut self) {
        self.dictionaries[0].close();
    }
    pub(crate) fn close_source_mdd(&mut self) {
        self.dictionaries[1].close();
    }
}
