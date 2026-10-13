//! Actual OutputProcessor::GetReportVariableInput local getter, without shared scratch.
use super::{
    OrdinaryRadiationError, OrdinaryRadiationUnavailable, SourceOutputFileHost,
    SourceOutputRegistration, SourceReportFrequency, SourceRequestedVariable,
};
use crate::heat_balance::selected_input_processor::SelectedInputProcessor;
use ep_model::{ReportInputs, SourceOutputVariableInput};

/// One temporary borrow of the typed getter operands and the sole real input owner.
/// No input identity, unused set, argument storage or report request is cloned here.
pub(crate) struct SourceReportVariableInput<'a> {
    reports: &'a ReportInputs,
    processor: &'a mut SelectedInputProcessor,
    minimum_report_frequency: &'a str,
}
impl<'a> SourceReportVariableInput<'a> {
    pub(crate) fn new(
        reports: &'a ReportInputs,
        processor: &'a mut SelectedInputProcessor,
        minimum_report_frequency: &'a str,
    ) -> Self {
        Self {
            reports,
            processor,
            minimum_report_frequency,
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

fn frequency(
    text: &str,
    minimum: SourceReportFrequency,
) -> Result<SourceReportFrequency, OrdinaryRadiationError> {
    use SourceReportFrequency as F;
    // determineFrequency returns the Hour default without applying minimum when
    // there are fewer than four characters, or no recognized four-byte prefix.
    if text.len() < 4 {
        return Ok(F::Hour);
    }
    let values = [
        ("DETA", "DETAILED", F::EachCall),
        ("TIME", "TIMESTEP", F::TimeStep),
        ("HOUR", "HOURLY", F::Hour),
        ("DAIL", "DAILY", F::Day),
        ("MONT", "MONTHLY", F::Month),
        ("RUNP", "RUNPERIOD", F::Simulation),
        ("ENVI", "ENVIRONMENT", F::Simulation),
        ("ANNU", "ANNUAL", F::Year),
    ];
    for (prefix, exact, value) in values {
        if text
            .get(..4)
            .is_some_and(|head| head.eq_ignore_ascii_case(prefix))
        {
            if !text.eq_ignore_ascii_case(exact) {
                // Actual warning/continue output is not replaced by acceptance.
                return Err(OrdinaryRadiationUnavailable::InputBranch.into());
            }
            return Ok(value.max(minimum));
        }
    }
    Ok(F::Hour)
}
fn is_regex(key: &str) -> bool {
    key != "*" && key.chars().any(|ch| "*+?()|[]\\.".contains(ch))
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
        input: &mut SourceReportVariableInput<'_>,
        _host: &impl SourceOutputFileHost,
        _eio: &mut dyn std::io::Write,
    ) -> Result<(), OrdinaryRadiationError> {
        // Original allocates all six local arrays even on the already-read return.
        let mut arguments = LocalArguments::default();
        if !self.get_output_input {
            return Ok(());
        }
        self.get_output_input = false;
        if !input.minimum_report_frequency.is_empty() {
            // Actual system MinReportFrequency is borrowed from its sole early owner.
            // determineFrequency/warning/EIO handling is an explicit unported
            // branch, before object count/getters and before any ReqVar push.
            return Err(OrdinaryRadiationUnavailable::InputBranch.into());
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
            request.is_simple_string = !is_regex(&request.key);
            if !request.is_simple_string {
                // Real RE2 compilation/matching is not represented yet. The
                // pushed request keeps its actual key and constructor defaults.
                return Err(OrdinaryRadiationUnavailable::InputBranch.into());
            }
            request.name = variable_name(&arguments.alphas[1]).into();
            request.frequency = frequency(&arguments.alphas[2], self.minimum_report_frequency)?;
            if !arguments.alpha_blanks[3] {
                // Real Sched::GetSchedule and its severe/fatal diagnostics need
                // the actual schedule owner; never invent a null-success result.
                return Err(OrdinaryRadiationUnavailable::InputBranch.into());
            }
            request.used = false;
        }
        Ok(())
    }
}
