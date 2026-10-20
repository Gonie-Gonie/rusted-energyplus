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
#[allow(
    dead_code,
    reason = "Retain the optional physical-stream dictionary host for direct registered-output consumers; current production supplies the same genuine host trait through its existing stream owner."
)]
pub(crate) struct SourceOutputDictionaryStreams<'a, 'stream> {
    /// Some only when the actual Source ESO stream is open and good.
    pub(crate) eso: Option<&'stream mut dyn Write>,
    /// Read from the caller's sole SQLite owner, never inferred from filename.
    pub(crate) sqlite_present: bool,
    pub(crate) results_framework_output_variables: &'a mut Vec<String>,
}
impl SourceOutputDictionaryHost for SourceOutputDictionaryStreams<'_, '_> {
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
        // Native2496 reads the resolved schedule's immutable Name, not the
        // spelling of the input reference and never its mutable current value.
        let schedule_name = match var.schedule {
            None => "",
            Some(schedule) => self
                .requested_variables
                .iter()
                .find(|request| request.reporting_schedule == Some(schedule))
                .map(|request| request.reporting_schedule_name.as_str())
                .filter(|name| !name.is_empty())
                .ok_or(OrdinaryRadiationUnavailable::StorageDomain)?,
        };
        let (frequency_index, arbitrary_integer, notice, frequency_name) =
            var.frequency.dictionary_format();
        let units = var.units.symbol();
        if host.eso_good() {
            let line = format!(
                "{},{},{},{} [{}]{}{}{}\n",
                var.report_id,
                arbitrary_integer,
                var.key,
                var.name,
                units,
                notice,
                if schedule_name.is_empty() { "" } else { "," },
                schedule_name
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
