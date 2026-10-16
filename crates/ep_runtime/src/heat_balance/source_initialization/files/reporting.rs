//! Concrete real output/dictionary/meter stream host over the sole opened files.
use super::{SourceInitializationFiles, SourceOutputStream};
use crate::heat_balance::source_initialization::SourceSqliteState;
use crate::heat_balance::source_output_registration::{
    SourceMeterReportingHost, SourceMtdFileHost, SourceOutputDictionaryHost,
    SourceOutputReportStreams,
};
use std::io::{self, Write};
use std::path::Path;
pub(crate) struct SourceRuntimeReportStreams<'a> {
    eso: &'a mut SourceOutputStream,
    mtr: &'a mut SourceOutputStream,
    sqlite: &'a SourceSqliteState,
    output_variables: &'a mut Vec<String>,
}
fn reached_write(stream: &mut SourceOutputStream, bytes: &[u8]) -> io::Result<()> {
    if stream.file.is_none() {
        return Err(io::Error::new(
            io::ErrorKind::NotConnected,
            "actual output has not opened",
        ));
    }
    // std::ostream already-failed state suppresses a further formatted write.
    if stream.failed {
        return Ok(());
    }
    stream.write_all(bytes)
}
impl SourceOutputReportStreams for SourceRuntimeReportStreams<'_> {
    fn eso_good(&self) -> bool {
        self.eso.file.is_some() && !self.eso.failed
    }
    fn mtr_good(&self) -> bool {
        self.mtr.file.is_some() && !self.mtr.failed
    }
    fn write_eso(&mut self, bytes: &[u8]) -> io::Result<()> {
        reached_write(self.eso, bytes)
    }
    fn write_mtr(&mut self, bytes: &[u8]) -> io::Result<()> {
        reached_write(self.mtr, bytes)
    }
    fn sqlite_present(&self) -> bool {
        self.sqlite.present()
    }
}
impl SourceOutputDictionaryHost for SourceRuntimeReportStreams<'_> {
    fn eso_good(&self) -> bool {
        SourceOutputReportStreams::eso_good(self)
    }
    fn write_eso_dictionary(&mut self, line: &str) -> io::Result<()> {
        SourceOutputReportStreams::write_eso(self, line.as_bytes())
    }
    fn sqlite_present(&self) -> bool {
        self.sqlite.present()
    }
    fn results_framework_output_variables(&mut self) -> &mut Vec<String> {
        self.output_variables
    }
}
pub(crate) struct SourceRuntimeMeterStreams<'a, 'files> {
    reports: &'a mut SourceRuntimeReportStreams<'files>,
    mtd: &'a mut SourceMtdFileHost,
}
impl SourceOutputReportStreams for SourceRuntimeMeterStreams<'_, '_> {
    fn eso_good(&self) -> bool {
        SourceOutputReportStreams::eso_good(&*self.reports)
    }
    fn mtr_good(&self) -> bool {
        SourceOutputReportStreams::mtr_good(&*self.reports)
    }
    fn write_eso(&mut self, bytes: &[u8]) -> io::Result<()> {
        SourceOutputReportStreams::write_eso(&mut *self.reports, bytes)
    }
    fn write_mtr(&mut self, bytes: &[u8]) -> io::Result<()> {
        SourceOutputReportStreams::write_mtr(&mut *self.reports, bytes)
    }
    fn sqlite_present(&self) -> bool {
        SourceOutputReportStreams::sqlite_present(&*self.reports)
    }
}
impl SourceMeterReportingHost for SourceRuntimeMeterStreams<'_, '_> {
    fn write_mtd(&mut self, bytes: &[u8]) -> io::Result<()> {
        self.mtd.write_reached(bytes)
    }
    fn results_framework_output_variables(&mut self) -> &mut Vec<String> {
        self.reports.output_variables
    }
    fn eso_file_path(&self) -> &Path {
        &self.reports.eso.path
    }
    fn mtr_file_path(&self) -> &Path {
        &self.reports.mtr.path
    }
}
impl<'files> SourceRuntimeReportStreams<'files> {
    pub(crate) fn meter_streams<'a>(
        &'a mut self,
        mtd: &'a mut SourceMtdFileHost,
    ) -> SourceRuntimeMeterStreams<'a, 'files> {
        SourceRuntimeMeterStreams { reports: self, mtd }
    }
}
impl SourceInitializationFiles {
    /// Reborrow actual ESO/MTR/EIO individually; no reopen, extra handle, sink,
    /// filename-derived SQL presence, or copied report data is introduced.
    pub(crate) fn source_report_streams_and_eio<'a>(
        &'a mut self,
        sqlite: &'a SourceSqliteState,
        output_variables: &'a mut Vec<String>,
    ) -> (SourceRuntimeReportStreams<'a>, &'a mut dyn Write) {
        let [eso, eio, mtr, _bnd] = &mut self.outputs;
        (
            SourceRuntimeReportStreams {
                eso,
                mtr,
                sqlite,
                output_variables,
            },
            eio,
        )
    }
}

impl SourceInitializationFiles {
    /// Same physical ESO/MTR plus typed same EIO goodbit; no open/reset/copy.
    pub(crate) fn source_weather_streams_and_eio<'a>(
        &'a mut self,
        sqlite: &'a SourceSqliteState,
        output_variables: &'a mut Vec<String>,
    ) -> (
        SourceRuntimeReportStreams<'a>,
        &'a mut dyn crate::weather::day::source_weather_initialization::SourceWeatherEio,
    ) {
        let [eso, eio, mtr, _bnd] = &mut self.outputs;
        (
            SourceRuntimeReportStreams {
                eso,
                mtr,
                sqlite,
                output_variables,
            },
            eio,
        )
    }
}
