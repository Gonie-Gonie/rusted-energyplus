//! Actual ResultsFramework meter frames, including unconditional disabled-JSON calls.
use super::meters::SourceMeter;
use super::*;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use std::collections::BTreeMap;
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceMeterVariable {
    pub(crate) name: String,
    pub(crate) frequency: SourceReportFrequency,
    pub(crate) time_step: TimeStepKind,
    pub(crate) report_id: i32,
    pub(crate) units: Option<SourceOutputUnits>,
    pub(crate) custom_units: String,
    pub(crate) values: Vec<f64>,
    pub(crate) accumulative: bool,
    pub(crate) meter_only: bool,
}
impl Default for SourceMeterVariable {
    fn default() -> Self {
        Self {
            name: String::new(),
            frequency: SourceReportFrequency::EachCall,
            time_step: TimeStepKind::Zone,
            report_id: -1,
            units: None,
            custom_units: String::new(),
            values: Vec::new(),
            accumulative: false,
            meter_only: true,
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceMeterDataFrame {
    pub(crate) iso8601: bool,
    pub(crate) beginning_of_interval: bool,
    pub(crate) enabled: bool,
    pub(crate) variables_scanned: bool,
    pub(crate) last_hour: i32,
    pub(crate) last_minute: i32,
    pub(crate) report_frequency: String,
    pub(crate) timestamps: Vec<String>,
    pub(crate) last_variable_id: i32,
    pub(crate) meter_map: BTreeMap<i32, SourceMeterVariable>,
}
impl SourceMeterDataFrame {
    pub(crate) fn new(frequency: &str) -> Self {
        Self {
            iso8601: false,
            beginning_of_interval: false,
            enabled: false,
            variables_scanned: false,
            last_hour: 0,
            last_minute: 0,
            report_frequency: frequency.into(),
            timestamps: Vec::new(),
            last_variable_id: -1,
            meter_map: BTreeMap::new(),
        }
    }
    fn add_variable(&mut self, variable: SourceMeterVariable) {
        self.last_variable_id = variable.report_id;
        self.meter_map
            .entry(self.last_variable_id)
            .or_insert(variable);
    }
    pub(super) fn initialize(&mut self, meters: &[SourceMeter], frequency: SourceReportFrequency) {
        if frequency != SourceReportFrequency::EachCall {
            for meter in meters {
                let period = &meter.periods[frequency as usize];
                if period.report || period.report_file_only {
                    self.add_variable(SourceMeterVariable {
                        name: meter.name.clone(),
                        frequency,
                        report_id: period.report_number,
                        units: Some(meter.units),
                        meter_only: period.report_file_only,
                        ..SourceMeterVariable::default()
                    });
                    self.enabled = true;
                }
                if period.accumulated_report || period.accumulated_file_only {
                    // Native does not pass Accumulative=true here; its default remains false.
                    self.add_variable(SourceMeterVariable {
                        name: meter.name.clone(),
                        frequency,
                        report_id: period.accumulated_report_number,
                        units: Some(meter.units),
                        meter_only: period.accumulated_file_only,
                        ..SourceMeterVariable::default()
                    });
                    self.enabled = true;
                }
            }
            self.variables_scanned = true;
        }
    }
    pub(super) fn new_row(
        &mut self,
        month: i32,
        day: i32,
        mut hour: i32,
        mut minute: i32,
        year: i32,
    ) -> Result<(), Error> {
        if minute > 0 {
            hour = hour
                .checked_sub(1)
                .ok_or(Error::Unavailable("actual RF hour domain"))?;
        }
        if minute == 60 {
            minute = 0;
            hour = hour
                .checked_add(1)
                .ok_or(Error::Unavailable("actual RF hour domain"))?;
        }
        if self.beginning_of_interval {
            if hour == 24 {
                hour = 0;
            }
            std::mem::swap(&mut hour, &mut self.last_hour);
            std::mem::swap(&mut minute, &mut self.last_minute);
        }
        let timestamp = if self.iso8601 {
            format!("{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:00")
        } else {
            format!("{month:02}/{day:02} {hour:02}:{minute:02}:00")
        };
        self.timestamps.push(timestamp);
        Ok(())
    }
    pub(super) fn push_value(&mut self, report_id: i32, value: f64) {
        self.meter_map
            .entry(report_id)
            .or_default()
            .values
            .push(value);
    }
}
/// Same caller-owned ResultsFramework outputVariables used by dictionary registration.
pub(crate) fn add_source_report_meter(
    output_variables: &mut Vec<String>,
    meter: &str,
    units: &str,
    frequency: SourceReportFrequency,
) {
    const NAMES: [&str; 7] = [
        "Each Call",
        "TimeStep",
        "Hourly",
        "Daily",
        "Monthly",
        "RunPeriod",
        "Annual",
    ];
    output_variables.push(format!("{meter} [{units}]({})", NAMES[frequency as usize]));
}
