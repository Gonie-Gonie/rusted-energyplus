//! Real SetupOutputVariable with actual Zone/System and energy meters.
use super::bound_real::SourceOutputVariableType;
use super::*;
use crate::heat_balance::error_backend::SourceErrorBackend;

#[derive(Clone, Copy)]
pub(crate) struct SourceRealRegistrationOptions<'a> {
    pub(crate) time_step: TimeStepKind,
    pub(crate) meter: Option<SourceEnergyTransferMeterArguments<'a>>,
}
impl SourceRealRegistrationOptions<'_> {
    pub(crate) fn unmetered(time_step: TimeStepKind) -> Self {
        Self {
            time_step,
            meter: None,
        }
    }
}
impl SourceOutputRegistration {
    pub(crate) fn setup_bound_real(
        &mut self,
        input: &mut SourceReportVariableInput<'_>,
        prescan: &SourceOutputPreScan,
        declaration: SourceRealOutputDeclaration<'_>,
        minutes_in_time_step: i32,
        host: &mut impl SourceOutputFileHost,
        eio: &mut dyn std::io::Write,
        binding: SourceRealOutputBinding<'_>,
        dictionary_host: &mut impl SourceOutputDictionaryHost,
        options: SourceRealRegistrationOptions<'_>,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), OrdinaryRadiationError> {
        // CheckReportVariable then FindItem precede actual meter validation/DD.
        let (request_indexes, on_the_list) = self.prepare_real_registration_front(
            input,
            prescan,
            declaration,
            minutes_in_time_step,
            host,
            eio,
        )?;
        let mut on_meter = options.meter.is_some();
        if on_meter && declaration.store_type == SourceOutputStoreType::Average {
            backend
                .show_severe_error("Meters can only be \"Summed\" variables")
                .map_err(|e| OrdinaryRadiationError::Input(Box::new(e.into())))?;
            backend
                .show_continue_error(&format!(
                    "..reference variable={}:{}",
                    declaration.key, declaration.name
                ))
                .map_err(|e| OrdinaryRadiationError::Input(Box::new(e.into())))?;
            on_meter = false;
        }
        let selected = self.finish_real_registration_prefix(
            declaration,
            options.time_step,
            request_indexes,
            on_the_list,
        )?;
        self.finish_bound_real_registration(
            selected,
            declaration,
            options,
            dictionary_host,
            binding,
            on_meter,
        )
    }
    pub(super) fn finish_bound_real_registration(
        &mut self,
        selected: SourceRealRegistrationSelection,
        declaration: SourceRealOutputDeclaration<'_>,
        options: SourceRealRegistrationOptions<'_>,
        dictionary_host: &mut impl SourceOutputDictionaryHost,
        binding: SourceRealOutputBinding<'_>,
        mut on_meter: bool,
    ) -> Result<(), OrdinaryRadiationError> {
        let first_added_out_var_number = self.out_vars.len();
        if !on_meter && !selected.on_the_list {
            return Ok(());
        }
        if declaration.store_type == SourceOutputStoreType::Sum {
            self.num_real_variables_sum = self
                .num_real_variables_sum
                .checked_add(1)
                .ok_or(OrdinaryRadiationUnavailable::StorageDomain)?;
        }
        if on_meter {
            self.num_real_variables_meter = self
                .num_real_variables_meter
                .checked_add(1)
                .ok_or(OrdinaryRadiationUnavailable::StorageDomain)?;
        }
        let requests = if selected.request_indexes.is_empty() {
            vec![None]
        } else {
            selected.request_indexes.into_iter().map(Some).collect()
        };
        for request in requests {
            self.num_real_variables = self
                .num_real_variables
                .checked_add(1)
                .ok_or(OrdinaryRadiationUnavailable::StorageDomain)?;
            self.out_vars.push(SourceOutVarReal::default());
            let number = self.out_vars.len() - 1;
            self.dictionary_variables[selected.dictionary_number]
                .key_out_var_nums
                .push(number);
            let var = &mut self.out_vars[number];
            var.dictionary_number = Some(selected.dictionary_number);
            var.variable_type = SourceOutputVariableType::Real;
            var.time_step_type = options.time_step;
            var.store_type = declaration.store_type;
            var.name = declaration.name.into();
            var.name_upper = var.name.to_ascii_uppercase();
            var.key = declaration.key.into();
            var.key_upper = var.key.to_ascii_uppercase();
            var.key_colon_name = format!("{}:{}", declaration.key, declaration.name);
            var.key_colon_name_upper = var.key_colon_name.to_ascii_uppercase();
            var.units = declaration.units;
            var.frequency = SourceReportFrequency::Hour;
            var.schedule = None;
            self.report_number_counter = self
                .report_number_counter
                .checked_add(1)
                .ok_or(OrdinaryRadiationUnavailable::StorageDomain)?;
            var.report_id = self.report_number_counter;
            var.which = Some(binding.target);
            var.zone_multiplier = options.meter.map_or(1, |m| m.zone_multiplier);
            var.zone_list_multiplier = options.meter.map_or(1, |m| m.zone_list_multiplier);
            var.index_group_key = -999;
            var.index_group = match options.time_step {
                TimeStepKind::Zone => "Zone",
                TimeStepKind::System => "System",
            }
            .into();
            if on_meter {
                let meter = options
                    .meter
                    .ok_or(OrdinaryRadiationUnavailable::StorageDomain)?;
                self.attach_source_energy_transfer_meters(
                    declaration.units,
                    meter,
                    first_added_out_var_number,
                )?;
                on_meter = false;
            }
            let Some(request) = request else {
                continue;
            };
            let var = &mut self.out_vars[number];
            var.report = true;
            var.frequency = self.requested_variables[request].frequency;
            var.schedule = self.requested_variables[request].reporting_schedule;
            self.write_real_report_dictionary(number, dictionary_host)?;
        }
        Ok(())
    }
}

// Actual same Real overload, with independently specified general meter arguments.
#[derive(Clone, Copy)]
pub(crate) struct SourceGeneralRealRegistrationOptions<'a> {
    pub(crate) time_step: TimeStepKind,
    pub(crate) meter: Option<SourceGeneralMeterArguments<'a>>,
}
impl SourceGeneralRealRegistrationOptions<'_> {
    pub(crate) fn unmetered(time_step: TimeStepKind) -> Self {
        Self {
            time_step,
            meter: None,
        }
    }
}
impl SourceOutputRegistration {
    pub(crate) fn setup_bound_general_real(
        &mut self,
        input: &mut SourceReportVariableInput<'_>,
        prescan: &SourceOutputPreScan,
        declaration: SourceRealOutputDeclaration<'_>,
        minutes_in_time_step: i32,
        host: &mut impl SourceOutputFileHost,
        eio: &mut dyn std::io::Write,
        binding: SourceRealOutputBinding<'_>,
        dictionary_host: &mut impl SourceOutputDictionaryHost,
        options: SourceGeneralRealRegistrationOptions<'_>,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), OrdinaryRadiationError> {
        self.setup_bound_general_real_optional_dictionary(
            input,
            prescan,
            declaration,
            minutes_in_time_step,
            host,
            eio,
            binding,
            Some(dictionary_host),
            options,
            backend,
        )
    }
    pub(crate) fn setup_bound_general_real_source_input(
        &mut self,
        input: &mut SourceReportVariableInput<'_>,
        prescan: &SourceOutputPreScan,
        declaration: SourceRealOutputDeclaration<'_>,
        minutes_in_time_step: i32,
        host: &mut impl SourceOutputFileHost,
        eio: &mut dyn std::io::Write,
        binding: SourceRealOutputBinding<'_>,
        options: SourceGeneralRealRegistrationOptions<'_>,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), OrdinaryRadiationError> {
        self.setup_bound_general_real_optional_dictionary(
            input,
            prescan,
            declaration,
            minutes_in_time_step,
            host,
            eio,
            binding,
            None,
            options,
            backend,
        )
    }
    fn setup_bound_general_real_optional_dictionary(
        &mut self,
        input: &mut SourceReportVariableInput<'_>,
        prescan: &SourceOutputPreScan,
        declaration: SourceRealOutputDeclaration<'_>,
        minutes_in_time_step: i32,
        host: &mut impl SourceOutputFileHost,
        eio: &mut dyn std::io::Write,
        binding: SourceRealOutputBinding<'_>,
        dictionary_host: Option<&mut dyn SourceOutputDictionaryHost>,
        options: SourceGeneralRealRegistrationOptions<'_>,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), OrdinaryRadiationError> {
        // CheckReportVariable then FindItem precede actual meter validation/DD.
        let (request_indexes, on_the_list) = self.prepare_real_registration_front(
            input,
            prescan,
            declaration,
            minutes_in_time_step,
            host,
            eio,
        )?;
        let mut on_meter = options.meter.is_some();
        if on_meter && declaration.store_type == SourceOutputStoreType::Average {
            backend
                .show_severe_error("Meters can only be \"Summed\" variables")
                .map_err(|e| OrdinaryRadiationError::Input(Box::new(e.into())))?;
            backend
                .show_continue_error(&format!(
                    "..reference variable={}:{}",
                    declaration.key, declaration.name
                ))
                .map_err(|e| OrdinaryRadiationError::Input(Box::new(e.into())))?;
            on_meter = false;
        }
        let selected = self.finish_real_registration_prefix(
            declaration,
            options.time_step,
            request_indexes,
            on_the_list,
        )?;
        self.finish_bound_general_real_registration(
            selected,
            declaration,
            options,
            dictionary_host,
            binding,
            on_meter,
        )
    }
    pub(super) fn finish_bound_general_real_registration(
        &mut self,
        selected: SourceRealRegistrationSelection,
        declaration: SourceRealOutputDeclaration<'_>,
        options: SourceGeneralRealRegistrationOptions<'_>,
        mut dictionary_host: Option<&mut dyn SourceOutputDictionaryHost>,
        binding: SourceRealOutputBinding<'_>,
        mut on_meter: bool,
    ) -> Result<(), OrdinaryRadiationError> {
        let first_added_out_var_number = self.out_vars.len();
        if !on_meter && !selected.on_the_list {
            return Ok(());
        }
        if declaration.store_type == SourceOutputStoreType::Sum {
            self.num_real_variables_sum = self
                .num_real_variables_sum
                .checked_add(1)
                .ok_or(OrdinaryRadiationUnavailable::StorageDomain)?;
        }
        if on_meter {
            self.num_real_variables_meter = self
                .num_real_variables_meter
                .checked_add(1)
                .ok_or(OrdinaryRadiationUnavailable::StorageDomain)?;
        }
        let requests = if selected.request_indexes.is_empty() {
            vec![None]
        } else {
            selected.request_indexes.into_iter().map(Some).collect()
        };
        for request in requests {
            self.num_real_variables = self
                .num_real_variables
                .checked_add(1)
                .ok_or(OrdinaryRadiationUnavailable::StorageDomain)?;
            self.out_vars.push(SourceOutVarReal::default());
            let number = self.out_vars.len() - 1;
            self.dictionary_variables[selected.dictionary_number]
                .key_out_var_nums
                .push(number);
            let var = &mut self.out_vars[number];
            var.dictionary_number = Some(selected.dictionary_number);
            var.variable_type = SourceOutputVariableType::Real;
            var.time_step_type = options.time_step;
            var.store_type = declaration.store_type;
            var.name = declaration.name.into();
            var.name_upper = var.name.to_ascii_uppercase();
            var.key = declaration.key.into();
            var.key_upper = var.key.to_ascii_uppercase();
            var.key_colon_name = format!("{}:{}", declaration.key, declaration.name);
            var.key_colon_name_upper = var.key_colon_name.to_ascii_uppercase();
            var.units = declaration.units;
            var.frequency = SourceReportFrequency::Hour;
            var.schedule = None;
            self.report_number_counter = self
                .report_number_counter
                .checked_add(1)
                .ok_or(OrdinaryRadiationUnavailable::StorageDomain)?;
            var.report_id = self.report_number_counter;
            var.which = Some(binding.target);
            var.zone_multiplier = options.meter.map_or(1, |m| m.zone_multiplier);
            var.zone_list_multiplier = options.meter.map_or(1, |m| m.zone_list_multiplier);
            var.index_group_key = -999;
            var.index_group = match options.time_step {
                TimeStepKind::Zone => "Zone",
                TimeStepKind::System => "System",
            }
            .into();
            if on_meter {
                let meter = options
                    .meter
                    .ok_or(OrdinaryRadiationUnavailable::StorageDomain)?;
                self.attach_source_general_meters(
                    declaration.units,
                    meter,
                    first_added_out_var_number,
                )?;
                on_meter = false;
            }
            let Some(request) = request else {
                continue;
            };
            let var = &mut self.out_vars[number];
            var.report = true;
            var.frequency = self.requested_variables[request].frequency;
            var.schedule = self.requested_variables[request].reporting_schedule;
            let actual_dictionary_host = dictionary_host
                .as_deref_mut()
                .ok_or(OrdinaryRadiationUnavailable::InputBranch)?;
            self.write_real_report_dictionary(number, actual_dictionary_host)?;
        }
        Ok(())
    }
}
