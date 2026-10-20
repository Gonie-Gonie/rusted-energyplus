//! Actual retained filesystem streams for OpenOutputFiles and Schedule input.
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

/// Explicit selected output paths; no Native path is inferred from a fixture.
#[derive(Debug)]
pub struct SourceInitializationFilePaths {
    /// Actual weather STAT path produced by CommandLineInterface583-584.
    pub stat_input: PathBuf,
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
    /// Actual cost-information path; a fresh local IOFilePath at CostInfoOut.
    pub sci: PathBuf,
    /// Actual report-variable dictionary path.
    pub rdd: PathBuf,
    /// Actual meter dictionary path.
    pub mdd: PathBuf,
    /// Actual Audit path, opened only at ProcessScheduleInput718.
    pub audit: PathBuf,
    /// Actual ManageSimulation185 debug path, opened before SQL/PostIP.
    pub debug: PathBuf,
    /// Actual local END, native-style ERR, and retained DFS paths from this frontend.
    pub end: PathBuf,
    /// Actual native-style ERR path owned by the frontend error backend.
    pub error: PathBuf,
    /// Actual DFS diagnostic output path selected by the frontend.
    pub dfs: PathBuf,
    /// Actual caller-selected paths for Native DataStringGlobals five table formats.
    pub tabular_csv: PathBuf,
    /// Tab-separated tabular output path selected by the real output caller.
    pub tabular_tab: PathBuf,
    /// Fixed-width tabular output path selected by the real output caller.
    pub tabular_fixed: PathBuf,
    /// HTML tabular output path selected by the real output caller.
    pub tabular_html: PathBuf,
    /// XML tabular output path selected by the real output caller.
    pub tabular_xml: PathBuf,
}

/// Windows text-mode transformation shared by the actual physical streams.
/// Native IOFiles290 omits binary; every LF expands while prior CR is retained.
pub(crate) fn native_text_bytes(bytes: &[u8]) -> std::borrow::Cow<'_, [u8]> {
    #[cfg(windows)]
    if bytes.contains(&b'\n') {
        let mut text = Vec::with_capacity(bytes.len());
        for &byte in bytes {
            if byte == b'\n' {
                text.push(b'\r');
            }
            text.push(byte);
        }
        return std::borrow::Cow::Owned(text);
    }
    std::borrow::Cow::Borrowed(bytes)
}

#[derive(Debug)]
struct SourceOutputStream {
    path: PathBuf,
    file: Option<File>,
    failed: bool,
    // Metadata of the actual retained IO owner, set only by its successful del.
    deleted_empty: bool,
}
impl SourceOutputStream {
    fn new(path: PathBuf) -> Self {
        Self {
            path,
            file: None,
            failed: false,
            deleted_empty: false,
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
                    self.deleted_empty = false;
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
        // Native ostream.write consumes the requested logical byte sequence;
        // its Windows text-mode file buffer expands each LF at the same handle.
        self.write_all(bytes).map(|()| bytes.len())
    }
    fn write_all(&mut self, bytes: &[u8]) -> io::Result<()> {
        let bytes = native_text_bytes(bytes);
        let result = self
            .file
            .as_mut()
            .ok_or_else(|| {
                io::Error::new(io::ErrorKind::NotConnected, "actual output is not open")
            })?
            .write_all(bytes.as_ref());
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
    // Native InputFilePath owns this path; each FillWeather call opens a local stream.
    stat_input: PathBuf,
    outputs: [SourceOutputStream; 4],
    audit: SourceOutputStream,
    debug: SourceOutputStream,
    end: PathBuf,
    error: PathBuf,
    dfs: SourceOutputStream,
    // Native IOFilePath owns only the path, not a permanently opened stream.
    dxf: PathBuf,
    // Actual IOFiles.hh635 local report path, never inferred from another file.
    sci: PathBuf,
    tabular_paths: [PathBuf; 5],
    // Native IOFiles.hh626-627 retained InputOutputFile objects.
    dictionaries: [dictionaries::SourceDictionaryStream; 2],
}
impl SourceInitializationFiles {
    /// Retain genuine caller-selected paths without opening any stream early.
    #[must_use]
    pub fn new(paths: SourceInitializationFilePaths) -> Self {
        Self {
            stat_input: paths.stat_input,
            outputs: [
                SourceOutputStream::new(paths.eso),
                SourceOutputStream::new(paths.eio),
                SourceOutputStream::new(paths.mtr),
                SourceOutputStream::new(paths.bnd),
            ],
            audit: SourceOutputStream::new(paths.audit),
            debug: SourceOutputStream::new(paths.debug),
            end: paths.end,
            error: paths.error,
            dfs: SourceOutputStream::new(paths.dfs),
            dxf: paths.dxf,
            sci: paths.sci,
            tabular_paths: [
                paths.tabular_csv,
                paths.tabular_tab,
                paths.tabular_fixed,
                paths.tabular_html,
                paths.tabular_xml,
            ],
            dictionaries: [
                dictionaries::SourceDictionaryStream::new(paths.rdd),
                dictionaries::SourceDictionaryStream::new(paths.mdd),
            ],
        }
    }
    /// Native format chooses a path; the separate TAB stream is ordinal-indexed.
    pub(crate) fn source_tabular_output_path(&self, style: i32) -> &Path {
        &self.tabular_paths[match style {
            0 => 0,
            1 => 1,
            3 => 3,
            4 => 4,
            _ => 2,
        }]
    }
    pub(crate) fn source_stat_input_path(&self) -> &Path {
        &self.stat_input
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
    pub(super) fn source_cost_info_path(&self) -> &Path {
        &self.sci
    }
    pub(super) fn open_source_cost_info(
        &self,
        output_to_file: bool,
    ) -> io::Result<SourceLocalReportFile> {
        open_local_report(&self.sci, output_to_file)
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
    /// Exact retained ESO/MTR paths of this Source run, also after native close.
    /// Borrowing does not open, flush, substitute or reconstruct either stream.
    pub fn source_reported_paths(&self) -> (&Path, &Path) {
        (&self.outputs[0].path, &self.outputs[2].path)
    }

    /// Read only the real ESO/MTR handle and deletion lifecycle metadata.
    /// These facts neither establish nor assert simulation completion.
    pub fn source_reported_file_states(
        &self,
    ) -> (
        super::super::source_reported_catalog::SourceReportedFileState,
        super::super::source_reported_catalog::SourceReportedFileState,
    ) {
        let state = |index: usize| {
            let output = &self.outputs[index];
            super::super::source_reported_catalog::SourceReportedFileState {
                closed: output.file.is_none() && !output.failed,
                deleted_empty: output.deleted_empty,
            }
        };
        (state(0), state(2))
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
mod endrun;
mod endrun_audit;
mod reporting;
pub(crate) use endrun::SourceCloseOutputFilesOwners;
pub(crate) use reporting::{SourceRuntimeMeterStreams, SourceRuntimeReportStreams};

#[cfg(test)]
mod scheduled_filter_file_tests {
    use super::*;

    #[test]
    fn scheduled_filter_file_deletion_is_an_actual_owner_action() -> io::Result<()> {
        let path = std::env::temp_dir().join(format!("ep-filter-file-{}", std::process::id()));
        let mut stream = SourceOutputStream::new(path.clone());
        stream.delete_source_output()?;
        assert!(!stream.deleted_empty);
        stream.ensure_open()?.write_all(b"dictionary only\n")?;
        stream.close_source_output();
        assert!(!stream.deleted_empty && stream.file.is_none() && path.exists());
        stream.ensure_open()?.write_all(b"dictionary only\n")?;
        stream.delete_source_output()?;
        assert!(stream.deleted_empty && stream.file.is_none() && !path.exists());
        stream.ensure_open()?;
        assert!(!stream.deleted_empty && stream.file.is_some() && path.exists());
        stream.delete_source_output()?;
        Ok(())
    }
}

impl crate::weather::day::source_weather_initialization::SourceWeatherEio for SourceOutputStream {
    fn good(&self) -> bool {
        self.file.is_some() && !self.failed
    }
}

impl SourceInitializationFiles {
    /// Native InputOutputFile::getLines296-316 on the same physical EIO handle.
    /// Flushes, saves/restores its position, and never opens a second stream.
    pub(crate) fn source_eio_lines(
        &mut self,
    ) -> Result<Vec<Vec<u8>>, crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError>
    {
        use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
        use std::io::{Read, Seek, SeekFrom};
        let stream = &mut self.outputs[1];
        if stream.failed {
            return Err(Error::Unavailable(
                "actual EIO getLines300 failed-stream flush/tellg domain",
            ));
        }
        if stream.file.is_none() {
            // Actual os==nullptr returns an empty vector at Native315.
            return Ok(Vec::new());
        }
        stream.flush()?;
        let file = stream.file.as_mut().ok_or(Error::Unavailable(
            "actual EIO getLines same nonnull stream owner",
        ))?;
        let last_position = file.stream_position()?;
        let mut line = Vec::new();
        let mut lines = Vec::new();
        file.seek(SeekFrom::Start(0))?;
        let mut byte = [0u8; 1];
        loop {
            if file.read(&mut byte)? == 0 {
                if !line.is_empty() {
                    lines.push(line);
                }
                break;
            }
            if byte[0] == 0x1a {
                return Err(Error::Unavailable(
                    "actual EIO getLines306 Windows text CTRL-Z input domain",
                ));
            }
            if byte[0] == b'\n' {
                // Same Windows text-mode CRLF translation as Native fstream.
                if line.last() == Some(&b'\r') {
                    line.pop();
                }
                lines.push(std::mem::take(&mut line));
            } else {
                line.push(byte[0]);
            }
        }
        // Native clears getline eof/fail, then seekg(last_pos). Rust reads do
        // not retain those transient bits; same saved position is restored.
        file.seek(SeekFrom::Start(last_position))?;
        Ok(lines)
    }
}

mod program_end;

/// IOFilePath local stream with actual outputControl false dev-null semantics.
pub(super) struct SourceLocalReportFile {
    file: Option<File>,
}
fn open_local_report(path: &Path, output_to_file: bool) -> io::Result<SourceLocalReportFile> {
    let file = if output_to_file {
        Some(
            OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .truncate(true)
                .open(path)?,
        )
    } else {
        None
    };
    Ok(SourceLocalReportFile { file })
}
impl Write for SourceLocalReportFile {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if let Some(file) = &mut self.file {
            file.write_all(native_text_bytes(bytes).as_ref())?;
        }
        Ok(bytes.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        match &mut self.file {
            Some(file) => file.flush(),
            None => Ok(()),
        }
    }
}

#[cfg(test)]
mod local_report_tests {
    use super::*;
    #[test]
    fn source_cost_info_output_gate_discards_without_open_and_preserves_real_open_error() {
        // Empty is an invalid real filename. Native false opens its dev-null
        // stream without touching it; true reaches the real filesystem error.
        let invalid = Path::new("");
        let mut disabled = open_local_report(invalid, false).unwrap();
        disabled.write_all(b"unchanged logical report\n").unwrap();
        disabled.flush().unwrap();
        assert!(disabled.file.is_none());
        assert!(open_local_report(invalid, true).is_err());
    }
}
