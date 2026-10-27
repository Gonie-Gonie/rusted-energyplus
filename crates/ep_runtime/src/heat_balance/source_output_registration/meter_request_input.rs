//! Genuine UpdateMeterReporting3804-4026 on the existing meters and real raw requests.
use super::{SourceOutputRegistration, SourceReportFrequency};
use crate::heat_balance::error_backend::{SourceDiagnosticFlags, SourceErrorBackend};
use crate::heat_balance::selected_heat_balance_input::{
    HeatBalanceInputError as Error, SelectedHeatBalanceInput,
};
use crate::heat_balance::selected_input_processor::{
    SelectedInputProcessor, SourceInputProcessorShortCut,
};
use std::io;
mod custom;
mod details;
mod dictionary;
use super::frequency;

/// Actual per-call retained streams and sole existing RF dictionary vector.
pub(crate) trait SourceMeterReportingHost: super::SourceOutputReportStreams {
    fn write_mtd(&mut self, bytes: &[u8]) -> io::Result<()>;
    fn results_framework_output_variables(&mut self) -> &mut Vec<String>;
    fn eso_file_path(&self) -> &std::path::Path;
    fn mtr_file_path(&self) -> &std::path::Path;
}
pub(crate) struct SourceMeterReportingOwners<'a, 'backend> {
    pub(crate) raw: &'a SelectedHeatBalanceInput,
    pub(crate) input: &'a mut SelectedInputProcessor,
    pub(crate) shortcut: &'a mut SourceInputProcessorShortCut,
    pub(crate) backend: &'a mut SourceErrorBackend<'backend>,
    pub(crate) diagnostic_flags: SourceDiagnosticFlags<'a>,
}

impl SourceOutputRegistration {
    pub(crate) fn update_source_meter_reporting(
        &mut self,
        o: &mut SourceMeterReportingOwners<'_, '_>,
        host: &mut impl SourceMeterReportingHost,
    ) -> Result<(), Error> {
        let mut alphas: [String; 2] = Default::default();
        let mut numbers = [None; 1]; // Rust availability mask; no numeric value is read before getObjectItem1012.
        let mut errors_found = false;
        custom::get_custom_meter_input(o, &mut errors_found)?;
        if errors_found {
            self.errors_logged = true;
        }
        for family in [
            "Output:Meter",
            "Output:Meter:MeterFileOnly",
            "Output:Meter:Cumulative",
            "Output:Meter:Cumulative:MeterFileOnly",
        ] {
            o.shortcut.current_module_object = family.into();
            let count = o
                .input
                .get_num_objects_found(&o.shortcut.current_module_object);
            for number in 0..count {
                let _counts = o.raw.get_mixed_source_object_item(
                    family,
                    number,
                    o.input,
                    o.shortcut,
                    (&mut alphas, &mut numbers),
                )?;
                let meter_file_only = family == "Output:Meter:MeterFileOnly"
                    || family == "Output:Meter:Cumulative:MeterFileOnly";
                let cumulative = family == "Output:Meter:Cumulative"
                    || family == "Output:Meter:Cumulative:MeterFileOnly";
                let frequency = frequency::determine_frequency(
                    &alphas[1].to_ascii_uppercase(),
                    self.minimum_report_frequency,
                    o.backend,
                    Some(&o.diagnostic_flags),
                )?;
                if !self.setup_source_meter_from_name(
                    &mut alphas[0],
                    frequency,
                    meter_file_only,
                    cumulative,
                    host,
                    o,
                )? {
                    let field_name = o
                        .shortcut
                        .alpha_field_names
                        .first()
                        .ok_or(Error::Unavailable("actual shared meter Key Name field"))?;
                    o.backend.show_source_warning_error(
                        &format!(
                            "{}: invalid {}=\"{}\" - not found.",
                            o.shortcut.current_module_object, field_name, alphas[0],
                        ),
                        &o.diagnostic_flags,
                    )?;
                }
            }
        }
        details::report_meter_details(self, host)?;
        if self.errors_logged {
            return match o.backend.show_fatal_error(
                "UpdateMeterReporting: Previous Meter Specification errors cause program termination.",
            ) {
                Ok(never) => match never {},
                Err(error) => Err(Error::Fatal(error)),
            };
        }
        self.meter_values.resize(self.meters.meters.len(), 0.0);
        self.meter_values.fill(0.0);
        Ok(())
    }

    fn setup_source_meter_from_name(
        &mut self,
        name: &mut String,
        frequency: SourceReportFrequency,
        meter_file_only: bool,
        cumulative: bool,
        host: &mut impl SourceMeterReportingHost,
        diagnostics: &mut SourceMeterReportingOwners<'_, '_>,
    ) -> Result<bool, Error> {
        let mut result = false;
        if let Some(position) = name.find('[') {
            name.truncate(position);
        }
        if let Some(position) = name.find('*') {
            let name_substring = &name.as_bytes()[..position];
            let mut number = 0;
            while number < self.meters.meters.len() {
                let meter_name = self.meters.meters[number].name.as_bytes();
                if meter_name[..position.min(meter_name.len())].eq_ignore_ascii_case(name_substring)
                {
                    self.set_source_initial_meter_reporting(
                        number,
                        meter_file_only,
                        frequency,
                        cumulative,
                        host,
                        diagnostics,
                    )?;
                    result = true;
                }
                number += 1;
            }
        } else if let Some(&number) = self.meters.meter_map.get(name) {
            self.set_source_initial_meter_reporting(
                number,
                meter_file_only,
                frequency,
                cumulative,
                host,
                diagnostics,
            )?;
            result = true;
        }
        Ok(result)
    }

    fn set_source_initial_meter_reporting(
        &mut self,
        number: usize,
        meter_file_only: bool,
        frequency: SourceReportFrequency,
        cumulative: bool,
        host: &mut impl SourceMeterReportingHost,
        diagnostics: &mut SourceMeterReportingOwners<'_, '_>,
    ) -> Result<(), Error> {
        let meter = self
            .meters
            .meters
            .get_mut(number)
            .ok_or(Error::Unavailable("actual initial meter reporting ordinal"))?;
        let period_index = if frequency == SourceReportFrequency::EachCall {
            SourceReportFrequency::TimeStep as usize
        } else {
            frequency as usize
        };
        if !cumulative {
            if meter_file_only && meter.periods[period_index].report {
                let frequency_name = if frequency == SourceReportFrequency::EachCall {
                    SourceReportFrequency::TimeStep.dictionary_format().3
                } else {
                    frequency.dictionary_format().3
                };
                diagnostics.backend.show_source_warning_error(&format!(
                    "Output:Meter:MeterFileOnly requested for \"{}\" ({}), already on \"Output:Meter\". Will report to both {} and {}",
                    meter.name, frequency_name, source_filename(host.eso_file_path())?, source_filename(host.mtr_file_path())?,
                ), &diagnostics.diagnostic_flags)?;
            }
            if !meter.periods[period_index].report {
                meter.periods[period_index].report = true;
                if meter_file_only {
                    meter.periods[period_index].report_file_only = true;
                } else {
                    self.frequency_tracking_variables[frequency as usize] = true;
                }
                meter.index_group = details::determine_index_group(meter)?;
                let (mtr_written, eso_written) = dictionary::write_meter_dictionary(
                    frequency,
                    meter.periods[period_index].report_number,
                    meter,
                    false,
                    meter_file_only,
                    host,
                )?;
                self.meter_dictionary_observations
                    .push(super::SourceMeterDictionaryObservation {
                        meter_index: number,
                        frequency,
                        cumulative: false,
                        mtr_written,
                        eso_written,
                    });
            }
        } else {
            if meter_file_only && meter.periods[period_index].accumulated_report {
                diagnostics.backend.show_source_warning_error(&format!(
                    "Output:Meter:MeterFileOnly requested for \"Cumulative {}\" (TimeStep), already on \"Output:Meter\". Will report to both {} and {}",
                    meter.name, source_filename(host.eso_file_path())?, source_filename(host.mtr_file_path())?,
                ), &diagnostics.diagnostic_flags)?;
            }
            if !meter.periods[period_index].accumulated_report {
                meter.periods[period_index].accumulated_report = true;
                if meter_file_only {
                    meter.periods[period_index].accumulated_file_only = true;
                }
                meter.index_group = details::determine_index_group(meter)?;
                let (mtr_written, eso_written) = dictionary::write_meter_dictionary(
                    frequency,
                    meter.periods[period_index].accumulated_report_number,
                    meter,
                    true,
                    meter_file_only,
                    host,
                )?;
                self.meter_dictionary_observations
                    .push(super::SourceMeterDictionaryObservation {
                        meter_index: number,
                        frequency,
                        cumulative: true,
                        mtr_written,
                        eso_written,
                    });
            }
        }
        Ok(())
    }
}

// Original FileSystem.hh327–353 formats path via toString without quoting.
// The real output pathname owns filename selection; no Native file is borrowed.
fn source_filename(path: &std::path::Path) -> Result<&str, Error> {
    path.file_name()
        .unwrap_or_else(|| std::ffi::OsStr::new(""))
        .to_str()
        .ok_or(Error::Unavailable(
            "actual meter output filename UTF-8 formatting",
        ))
}

#[cfg(test)]
mod tests;
