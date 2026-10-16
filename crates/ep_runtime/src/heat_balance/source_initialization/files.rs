//! Actual retained filesystem streams for OpenOutputFiles and Schedule input.
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

/// Explicit selected output paths; no Native path is inferred from a fixture.
#[derive(Debug)]
pub struct SourceInitializationFilePaths {
    /// Actual ESO output path.
    pub eso: PathBuf,
    /// Actual EIO output path, reused by later HB initialization.
    pub eio: PathBuf,
    /// Actual meter output path.
    pub mtr: PathBuf,
    /// Actual branch/node output path.
    pub bnd: PathBuf,
    /// Actual DXF drawing path; opened freshly at each reached DXF call.
    pub dxf: PathBuf,
    /// Actual report-variable dictionary path.
    pub rdd: PathBuf,
    /// Actual meter dictionary path.
    pub mdd: PathBuf,
    /// Actual Audit path, opened only at ProcessScheduleInput718.
    pub audit: PathBuf,
    /// Actual ManageSimulation185 debug path, opened before SQL/PostIP.
    pub debug: PathBuf,
    /// Actual caller-selected paths for Native DataStringGlobals five table formats.
    pub tabular_csv: PathBuf,
    pub tabular_tab: PathBuf,
    pub tabular_fixed: PathBuf,
    pub tabular_html: PathBuf,
    pub tabular_xml: PathBuf,
}

#[derive(Debug)]
struct SourceOutputStream {
    path: PathBuf,
    file: Option<File>,
    failed: bool,
}
impl SourceOutputStream {
    fn new(path: PathBuf) -> Self {
        Self {
            path,
            file: None,
            failed: false,
        }
    }
    fn ensure_open(&mut self) -> io::Result<&mut Self> {
        if self.file.is_none() || self.failed {
            // IOFiles.cc278-294: read/write/create/truncate; default controls
            // are true because the sole ordinary admission excludes Output:Files.
            match OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .truncate(true)
                .open(&self.path)
            {
                Ok(file) => {
                    self.file = Some(file);
                    self.failed = false;
                }
                Err(error) => {
                    self.file = None;
                    self.failed = true;
                    return Err(error);
                }
            }
        }
        Ok(self)
    }
}
impl Write for SourceOutputStream {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        let result = self
            .file
            .as_mut()
            .ok_or_else(|| {
                io::Error::new(io::ErrorKind::NotConnected, "actual output is not open")
            })?
            .write(bytes);
        if result.is_err() || matches!(result,Ok(0) if !bytes.is_empty()) {
            self.failed = true;
        }
        result
    }
    fn write_all(&mut self, bytes: &[u8]) -> io::Result<()> {
        let result = self
            .file
            .as_mut()
            .ok_or_else(|| {
                io::Error::new(io::ErrorKind::NotConnected, "actual output is not open")
            })?
            .write_all(bytes);
        if result.is_err() {
            self.failed = true;
        }
        result
    }
    fn flush(&mut self) -> io::Result<()> {
        let result = self
            .file
            .as_mut()
            .ok_or_else(|| {
                io::Error::new(io::ErrorKind::NotConnected, "actual output is not open")
            })?
            .flush();
        if result.is_err() {
            self.failed = true;
        }
        result
    }
}

/// Actual audit capability; a concrete real stream must outlive returned errors.
pub(crate) trait ScheduleAuditFileHost {
    fn audit_path(&self) -> &Path;
    fn ensure_audit_open(&mut self) -> io::Result<&mut dyn Write>;
}

/// Non-Clone retained source output files, independent of Runtime Result.
#[derive(Debug)]
pub struct SourceInitializationFiles {
    outputs: [SourceOutputStream; 4],
    audit: SourceOutputStream,
    debug: SourceOutputStream,
    // Native IOFilePath owns only the path, not a permanently opened stream.
    dxf: PathBuf,
    tabular_paths: [PathBuf; 5],
    // Native IOFiles.hh626-627 retained InputOutputFile objects.
    dictionaries: [dictionaries::SourceDictionaryStream; 2],
}
impl SourceInitializationFiles {
    /// Retain genuine caller-selected paths without opening any stream early.
    #[must_use]
    pub fn new(paths: SourceInitializationFilePaths) -> Self {
        Self {
            outputs: [
                SourceOutputStream::new(paths.eso),
                SourceOutputStream::new(paths.eio),
                SourceOutputStream::new(paths.mtr),
                SourceOutputStream::new(paths.bnd),
            ],
            audit: SourceOutputStream::new(paths.audit),
            debug: SourceOutputStream::new(paths.debug),
            dxf: paths.dxf,
            tabular_paths: [paths.tabular_csv, paths.tabular_tab, paths.tabular_fixed, paths.tabular_html, paths.tabular_xml],
            dictionaries: [
                dictionaries::SourceDictionaryStream::new(paths.rdd),
                dictionaries::SourceDictionaryStream::new(paths.mdd),
            ],
        }
    }
    /// Native format chooses a path; the separate TAB stream is ordinal-indexed.
    pub(crate) fn source_tabular_output_path(&self, style: i32) -> &Path {
        &self.tabular_paths[match style { 0 => 0, 1 => 1, 3 => 3, 4 => 4, _ => 2 }]
    }
    pub(super) fn source_dxf_path(&self) -> &Path {
        &self.dxf
    }
    pub(super) fn open_source_dxf(&self) -> io::Result<File> {
        // IOFiles.hh498-505 creates a fresh InputOutputFile for each call.
        OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.dxf)
    }
    pub(super) fn output_path(&self, index: usize) -> &Path {
        &self.outputs[index].path
    }
    pub(super) fn ensure_output_open(&mut self, index: usize) -> io::Result<&mut dyn Write> {
        Ok(self.outputs[index].ensure_open()?)
    }
    pub(super) fn ensure_debug_open(&mut self) -> io::Result<()> {
        self.debug.ensure_open().map(|_| ())
    }
    /// Borrow the already-open actual EIO for subsequent reached producers.
    ///
    /// # Errors
    /// Fails if OpenOutputFiles has not opened EIO; this method never opens it.
    pub fn eio_writer(&mut self) -> io::Result<&mut dyn Write> {
        if self.outputs[1].file.is_none() {
            return Err(io::Error::new(
                io::ErrorKind::NotConnected,
                "actual EIO is not open",
            ));
        }
        Ok(&mut self.outputs[1])
    }
    /// Borrow the retained EIO stream without opening it or substituting a sink.
    /// Writes to an unreached stream return the real not-connected error.
    pub fn eio_stream(&mut self) -> &mut dyn Write {
        &mut self.outputs[1]
    }
    /// Same already-open physical BND, without opening or replacing the stream.
    pub(crate) fn bnd_stream(&mut self) -> &mut dyn Write {
        &mut self.outputs[3]
    }
    /// Flush every actually opened stream without opening an unreached output.
    ///
    /// # Errors
    /// Retains and returns the first actual OS failure; later streams are not flushed.
    pub fn flush_reached(&mut self) -> io::Result<()> {
        for stream in &mut self.outputs {
            if stream.file.is_some() {
                stream.flush()?;
            }
        }
        for stream in &mut self.dictionaries {
            if stream.is_open() {
                stream.flush()?;
            }
        }
        if self.debug.file.is_some() {
            self.debug.flush()?;
        }
        if self.audit.file.is_some() {
            self.audit.flush()?;
        }
        Ok(())
    }
}
impl ScheduleAuditFileHost for SourceInitializationFiles {
    fn audit_path(&self) -> &Path {
        &self.audit.path
    }
    fn ensure_audit_open(&mut self) -> io::Result<&mut dyn Write> {
        Ok(self.audit.ensure_open()?)
    }
}

mod dictionaries;
mod reporting;
pub(crate) use reporting::{SourceRuntimeMeterStreams, SourceRuntimeReportStreams};

impl crate::weather::day::source_weather_initialization::SourceWeatherEio for SourceOutputStream {
    fn good(&self) -> bool {
        self.file.is_some() && !self.failed
    }
}
