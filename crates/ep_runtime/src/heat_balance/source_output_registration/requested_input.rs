//! Actual OutputProcessor::GetReportVariableInput local getter, without shared scratch.
use super::{
    OrdinaryRadiationError, OrdinaryRadiationUnavailable, SourceOutputFileHost,
    SourceOutputRegistration, SourceRequestedVariable,
};
use crate::heat_balance::error_backend::{SourceDiagnosticFlags, SourceErrorBackend};
use crate::heat_balance::selected_input_processor::SelectedInputProcessor;
use crate::heat_balance::selected_schedule_manager::SelectedScheduleManager;
use crate::heat_balance::source_output_prescan::{REPORTING_RE2_LOG_ORIGIN, is_key_regex_like};
use ep_model::{ReportInputs, SourceOutputVariableInput};

/// One temporary borrow of the typed getter operands and the sole real input owner.
/// No input identity, unused set, argument storage or report request is cloned here.
#[derive(Clone, Copy)]
pub(crate) struct SourceReportVariableEnvironment<'a> {
    pub(crate) minimum_report_frequency: &'a str,
    pub(crate) diagnostic_flags: Option<SourceDiagnosticFlags<'a>>,
}

/// Short getter borrow of the original system string and phase cells.
impl<'a> SourceReportVariableEnvironment<'a> {
    pub(crate) fn new(
        minimum_report_frequency: &'a str,
        diagnostic_flags: Option<SourceDiagnosticFlags<'a>>,
    ) -> Self {
        Self {
            minimum_report_frequency,
            diagnostic_flags,
        }
    }

    pub(crate) fn from_owners(
        minimum_report_frequency: &'a str,
        globals: &'a crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals,
        developer: &'a bool,
        clock: Option<&'a crate::weather::day::WeatherGlobalState>,
    ) -> Self {
        Self::new(
            minimum_report_frequency,
            clock.map(|clock| SourceDiagnosticFlags {
                kickoff: &globals.kickoff_simulation,
                developer,
                warmup: &clock.warmup_flag,
                doing_sizing: &globals.doing_sizing,
            }),
        )
    }
}

pub(crate) struct SourceReportVariableInput<'a, 'backend> {
    reports: &'a ReportInputs,
    processor: &'a mut SelectedInputProcessor,
    environment: SourceReportVariableEnvironment<'a>,
    pub(super) backend: &'a mut SourceErrorBackend<'backend>,
    schedules: Option<&'a mut SelectedScheduleManager>,
}
impl<'a, 'backend> SourceReportVariableInput<'a, 'backend> {
    /// Reborrow the same backend only at the reached Source effect.
    pub(crate) fn backend(&mut self) -> &mut SourceErrorBackend<'backend> {
        self.backend
    }

    pub(crate) fn new(
        reports: &'a ReportInputs,
        processor: &'a mut SelectedInputProcessor,
        environment: SourceReportVariableEnvironment<'a>,
        backend: &'a mut SourceErrorBackend<'backend>,
    ) -> Self {
        Self {
            reports,
            processor,
            environment,
            backend,
            schedules: None,
        }
    }

    /// Borrow the same already initialized ScheduleManager on the first real
    /// GetReportVariableInput call. Only native schedule identities are retained.
    pub(crate) fn with_schedules(
        reports: &'a ReportInputs,
        processor: &'a mut SelectedInputProcessor,
        environment: SourceReportVariableEnvironment<'a>,
        schedules: &'a mut SelectedScheduleManager,
        backend: &'a mut SourceErrorBackend<'backend>,
    ) -> Self {
        Self {
            reports,
            processor,
            environment,
            backend,
            schedules: Some(schedules),
        }
    }

    fn validate_count_and_identities(&self, family: &str) -> Result<usize, OrdinaryRadiationError> {
        let count = self.processor.get_num_objects_found(family);
        if count != self.reports.output_variables.len()
            || self
                .reports
                .output_variables
                .iter()
                .enumerate()
                .any(|(index, object)| {
                    !self.processor.has_object(family, &object.object_name)
                        || object.num_alphas > 4
                        || self.reports.output_variables[..index]
                            .iter()
                            .any(|prior| prior.object_name == object.object_name)
                })
        {
            return Err(OrdinaryRadiationUnavailable::InputBranch.into());
        }
        Ok(count)
    }
}

/// Literal procedure-local Array1D capacities; this is not IPShortCut.
struct LocalArguments {
    alphas: [String; 4],
    numerics: [f64; 1],
    alpha_blanks: [bool; 4],
    numeric_blanks: [bool; 1],
    alpha_field_names: [String; 4],
    numeric_field_names: [String; 1],
}
impl Default for LocalArguments {
    fn default() -> Self {
        Self {
            alphas: Default::default(),
            numerics: [0.0],
            alpha_blanks: [false; 4],
            numeric_blanks: [false],
            alpha_field_names: Default::default(),
            numeric_field_names: Default::default(),
        }
    }
}
impl LocalArguments {
    fn get_object_item(
        &mut self,
        family: &str,
        object: &SourceOutputVariableInput,
        processor: &mut SelectedInputProcessor,
    ) -> (u8, u8, i32) {
        // InputProcessor1008-1029: full six-array reset, then exact unused erase.
        self.alphas.fill(String::new());
        self.numerics.fill(0.0);
        self.numeric_blanks.fill(true);
        self.alpha_blanks.fill(true);
        self.alpha_field_names.fill(String::new());
        self.numeric_field_names.fill(String::new());
        processor.mark_object_as_used(family, &object.object_name);
        self.alphas.clone_from(&[
            object.key.clone(),
            object.variable_name.clone(),
            object.reporting_frequency.clone(),
            object.schedule_name.clone(),
        ]);
        self.alpha_blanks = object.alpha_blank;
        const JSON: [&str; 4] = [
            "key_value",
            "variable_name",
            "reporting_frequency",
            "schedule_name",
        ];
        const IDF: [&str; 4] = [
            "Key Value",
            "Variable Name",
            "Reporting Frequency",
            "Schedule Name",
        ];
        for (slot, name) in
            self.alpha_field_names
                .iter_mut()
                .zip(if object.is_epjson { JSON } else { IDF })
        {
            *slot = name.into();
        }
        // Output:Variable has four alpha fields and no numeric fields. Defaults
        // fill every alpha even beyond the actual physical NumAlphas boundary.
        (object.num_alphas, 0, 1)
    }
}

fn variable_name(name: &str) -> &str {
    let Some((without_units, _)) = name.split_once('[') else {
        return name;
    };
    without_units.trim_end_matches([' ', '\u{000c}', '\n', '\r', '\t', '\u{000b}'])
}

impl SourceOutputRegistration {
    pub(super) fn get_report_variable_input(
        &mut self,
        input: &mut SourceReportVariableInput<'_, '_>,
        _host: &impl SourceOutputFileHost,
        eio: &mut dyn std::io::Write,
    ) -> Result<(), OrdinaryRadiationError> {
        // Original allocates all six local arrays even on the already-read return.
        let mut arguments = LocalArguments::default();
        if !self.get_output_input {
            return Ok(());
        }
        self.get_output_input = false;
        if !input.environment.minimum_report_frequency.is_empty() {
            let minimum = super::frequency::determine_frequency(
                input.environment.minimum_report_frequency,
                self.minimum_report_frequency,
                input.backend,
                input.environment.diagnostic_flags.as_ref(),
            )
            .map_err(|error| OrdinaryRadiationError::Input(Box::new(error)))?;
            self.minimum_report_frequency = minimum;
            super::frequency::write_minimum_frequency_override(
                eio,
                minimum,
                input.environment.minimum_report_frequency,
            )
            .map_err(|error| {
                OrdinaryRadiationError::Output(crate::error::WarmupOutputError::Io(error))
            })?;
        }
        let current_module_object = String::from("Output:Variable");
        let count = input.validate_count_and_identities(&current_module_object)?;
        for index in 0..count {
            let object = &input.reports.output_variables[index];
            let (_num_alphas, _num_numbers, _io_status) =
                arguments.get_object_item(&current_module_object, object, input.processor);
            // ReqVar's constructor/push precedes every field-specific branch.
            self.requested_variables
                .push(SourceRequestedVariable::default());
            let request = self
                .requested_variables
                .last_mut()
                .ok_or(OrdinaryRadiationUnavailable::StorageDomain)?;
            request.key.clone_from(&arguments.alphas[0]);
            if request.key == "*" {
                request.key.clear();
            }
            request.is_simple_string = !is_key_regex_like(&request.key);
            if !request.is_simple_string {
                // Native constructs CI only, with NO ok() guard. An invalid
                // object is retained before name/frequency/schedule writes.
                request.case_insensitive_pattern =
                    Some(std::sync::Arc::new(ep_re2::Re2::new_logged(
                        &format!("(?i){}", request.key),
                        REPORTING_RE2_LOG_ORIGIN,
                    )));
            }
            request.name = variable_name(&arguments.alphas[1]).into();
            request.frequency = super::frequency::determine_frequency(
                &arguments.alphas[2].to_ascii_uppercase(),
                self.minimum_report_frequency,
                input.backend,
                input.environment.diagnostic_flags.as_ref(),
            )
            .map_err(|error| OrdinaryRadiationError::Input(Box::new(error)))?;
            if !arguments.alpha_blanks[3] {
                // Native OutputProcessor538-545 resolves the actual schedule and
                // propagates its used flag. Missing references remain rejected;
                // their unported severe/fatal path is never a null success.
                let schedules = input
                    .schedules
                    .as_deref_mut()
                    .ok_or(OrdinaryRadiationUnavailable::InputBranch)?;
                let schedule = schedules
                    .get_source_schedule(&arguments.alphas[3])
                    .map_err(|error| OrdinaryRadiationError::Input(Box::new(error)))?
                    .ok_or(OrdinaryRadiationUnavailable::InputBranch)?;
                request.reporting_schedule = Some(schedule);
                request.reporting_schedule_name.clone_from(
                    &schedules
                        .schedules
                        .get(schedule.0)
                        .ok_or(OrdinaryRadiationUnavailable::StorageDomain)?
                        .name,
                );
            }
            request.used = false;
        }
        Ok(())
    }
}

#[cfg(test)]
mod scheduled_filter_tests;
