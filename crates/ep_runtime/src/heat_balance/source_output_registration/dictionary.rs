//! Reached OutVar::writeReportDictionaryItem, with actual borrowed file/RF owners.
use super::{
    OrdinaryRadiationError, OrdinaryRadiationUnavailable, SourceOutputRegistration,
    SourceReportFrequency,
};
use crate::error::WarmupOutputError;
use std::io::{self, Write};

/// Caller supplies the real ESO state, SQL-presence guard and RF metadata vector.
pub(crate) trait SourceOutputDictionaryHost {
    fn eso_good(&self) -> bool;
    fn write_eso_dictionary(&mut self, line: &str) -> io::Result<()>;
    fn sqlite_present(&self) -> bool;
    fn results_framework_output_variables(&mut self) -> &mut Vec<String>;
}

/// Borrowed stream and actual ResultsFramework outputVariables, with no values.
pub(crate) struct SourceOutputDictionaryStreams<'a> {
    /// Some only when the actual Source ESO stream is open and good.
    pub(crate) eso: Option<&'a mut dyn Write>,
    /// Read from the caller's sole SQLite owner, never inferred from filename.
    pub(crate) sqlite_present: bool,
    pub(crate) results_framework_output_variables: &'a mut Vec<String>,
}
impl SourceOutputDictionaryHost for SourceOutputDictionaryStreams<'_> {
    fn eso_good(&self) -> bool {
        self.eso.is_some()
    }
    fn write_eso_dictionary(&mut self, line: &str) -> io::Result<()> {
        self.eso
            .as_mut()
            .ok_or_else(|| io::Error::other("actual Source ESO not open"))?
            .write_all(line.as_bytes())
    }
    fn sqlite_present(&self) -> bool {
        self.sqlite_present
    }
    fn results_framework_output_variables(&mut self) -> &mut Vec<String> {
        self.results_framework_output_variables
    }
}

impl SourceReportFrequency {
    pub(super) fn dictionary_format(self) -> (usize, i32, &'static str, &'static str) {
        match self {
            Self::EachCall => (0, 1, " !Each Call", "Each Call"),
            Self::TimeStep => (1, 1, " !TimeStep", "TimeStep"),
            Self::Hour => (2, 1, " !Hourly", "Hourly"),
            Self::Day => (
                3,
                7,
                " !Daily [Value,Min,Hour,Minute,Max,Hour,Minute]",
                "Daily",
            ),
            Self::Month => (
                4,
                9,
                " !Monthly [Value,Min,Day,Hour,Minute,Max,Day,Hour,Minute]",
                "Monthly",
            ),
            Self::Simulation => (
                5,
                11,
                " !RunPeriod [Value,Min,Month,Day,Hour,Minute,Max,Month,Day,Hour,Minute]",
                "RunPeriod",
            ),
            Self::Year => (
                6,
                11,
                " !Annual [Value,Min,Month,Day,Hour,Minute,Max,Month,Day,Hour,Minute]",
                "Annual",
            ),
        }
    }
}

impl SourceOutputRegistration {
    pub(super) fn write_real_report_dictionary(
        &mut self,
        number: usize,
        host: &mut (impl SourceOutputDictionaryHost + ?Sized),
    ) -> Result<(), OrdinaryRadiationError> {
        let var = self
            .out_vars
            .get(number)
            .ok_or(OrdinaryRadiationUnavailable::StorageDomain)?;
        if var.schedule.is_some() {
            // Scheduled report input is already a getter boundary. Preserve
            // the actual keyed variable if a future caller reaches it here.
            return Err(OrdinaryRadiationUnavailable::InputBranch.into());
        }
        let (frequency_index, arbitrary_integer, notice, frequency_name) =
            var.frequency.dictionary_format();
        let units = var.units.symbol();
        if host.eso_good() {
            let line = format!(
                "{},{},{},{} [{}]{}\n",
                var.report_id, arbitrary_integer, var.key, var.name, units, notice
            );
            host.write_eso_dictionary(&line)
                .map_err(|error| OrdinaryRadiationError::Output(WarmupOutputError::Io(error)))?;
        }
        if matches!(
            var.frequency,
            SourceReportFrequency::Hour
                | SourceReportFrequency::Day
                | SourceReportFrequency::Month
                | SourceReportFrequency::Year
                | SourceReportFrequency::Simulation
        ) {
            self.frequency_tracking_variables[frequency_index] = true;
        }
        if host.sqlite_present() {
            return Err(OrdinaryRadiationUnavailable::InputBranch.into());
        }
        host.results_framework_output_variables().push(format!(
            "{}:{} [{}]({})",
            var.key, var.name, units, frequency_name
        ));
        Ok(())
    }
}
