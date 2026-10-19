//! Genuine keyed Integer overload3146–3240 with a borrowed i32 Which target.
use super::bound_real::SourceOutputVariableType;
use super::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SourceIntegerOutputTarget {
    PurchasedAirAvailabilityStatus { index: i32 },
    Weather {
        field: super::SourceWeatherIntegerOutputField,
    },
    ZoneThermostatControlType {
        index: i32,
    },
    SurfaceInsideIterationCount,
    HvacSystemSolverIterationCount,
    AirSystemSolverIterationCount,
    PlantSubIterationCount,
    PlantHalfLoopCallCount,
    AirLoopMaximumIterations,
    AirLoopTotalIterations,
    AirLoopTotalCalls,
}
pub(crate) trait SourceIntegerOutputOwners {
    fn integer_output(
        &self,
        target: SourceIntegerOutputTarget,
    ) -> Result<&i32, OrdinaryRadiationError>;
}
pub(crate) struct SourceIntegerOutputBinding<'a> {
    target: SourceIntegerOutputTarget,
    _actual: &'a i32,
}
impl<'a> SourceIntegerOutputBinding<'a> {
    pub(crate) fn new(
        target: SourceIntegerOutputTarget,
        owners: &'a impl SourceIntegerOutputOwners,
    ) -> Result<Self, OrdinaryRadiationError> {
        Ok(Self {
            target,
            _actual: owners.integer_output(target)?,
        })
    }
}
pub(crate) struct SourceRequestedIntegerValue<'a> {
    pub(crate) out_var_number: usize,
    pub(crate) report_id: i32,
    pub(crate) request: crate::RuntimeOutputRequest,
    pub(crate) units: &'static str,
    pub(crate) target: SourceIntegerOutputTarget,
    pub(crate) value: &'a i32,
}
impl SourceOutputRegistration {
    pub(crate) fn setup_bound_zone_integer(
        &mut self,
        input: &mut SourceReportVariableInput<'_>,
        prescan: &SourceOutputPreScan,
        declaration: SourceIntegerOutputDeclaration<'_>,
        minutes_in_time_step: i32,
        host: &mut impl SourceOutputFileHost,
        eio: &mut dyn std::io::Write,
        binding: SourceIntegerOutputBinding<'_>,
        dictionary_host: &mut impl SourceOutputDictionaryHost,
    ) -> Result<(), OrdinaryRadiationError> {
        self.setup_bound_integer(
            input,
            prescan,
            declaration,
            minutes_in_time_step,
            host,
            eio,
            binding,
            dictionary_host,
            TimeStepKind::Zone,
        )
    }
    pub(crate) fn setup_bound_integer(
        &mut self,
        input: &mut SourceReportVariableInput<'_>,
        prescan: &SourceOutputPreScan,
        declaration: SourceIntegerOutputDeclaration<'_>,
        minutes_in_time_step: i32,
        host: &mut impl SourceOutputFileHost,
        eio: &mut dyn std::io::Write,
        binding: SourceIntegerOutputBinding<'_>,
        dictionary_host: &mut impl SourceOutputDictionaryHost,
        time_step: TimeStepKind,
    ) -> Result<(), OrdinaryRadiationError> {
        if !self.output_initialized {
            self.initialize_output(minutes_in_time_step, host)?;
        }
        self.get_report_variable_input(input, host, eio)?;
        let indexes = self.match_report_variables(declaration.name, declaration.key);
        let requests: Vec<Option<usize>> = if indexes.is_empty() {
            vec![None]
        } else {
            indexes.into_iter().map(Some).collect()
        };
        let dictionary = self.add_typed_dictionary_variable_at(
            SourceRealOutputDeclaration {
                name: declaration.name,
                key: declaration.key,
                units: declaration.units,
                store_type: declaration.store_type,
            },
            true,
            time_step,
        );
        self.num_integer_variables_setup = self
            .num_integer_variables_setup
            .checked_add(1)
            .ok_or(OrdinaryRadiationUnavailable::StorageDomain)?;
        self.num_total_integer_variables = self
            .num_total_integer_variables
            .checked_add(
                i32::try_from(requests.len())
                    .map_err(|_| OrdinaryRadiationUnavailable::StorageDomain)?,
            )
            .ok_or(OrdinaryRadiationUnavailable::StorageDomain)?;
        if !prescan.find_item(declaration.key, declaration.name) {
            return Ok(());
        }
        if declaration.store_type == SourceOutputStoreType::Sum {
            self.num_integer_variables_sum = self
                .num_integer_variables_sum
                .checked_add(1)
                .ok_or(OrdinaryRadiationUnavailable::StorageDomain)?;
        }
        for request in requests {
            self.num_integer_variables = self
                .num_integer_variables
                .checked_add(1)
                .ok_or(OrdinaryRadiationUnavailable::StorageDomain)?;
            // All common constructor fields are identical in OutVarInt/Real;
            // the pointer type remains i32 and is never cast to a real pointer.
            self.out_vars.push(SourceOutVarReal {
                variable_type: SourceOutputVariableType::Integer,
                ..SourceOutVarReal::default()
            });
            let number = self.out_vars.len() - 1;
            self.dictionary_variables[dictionary]
                .key_out_var_nums
                .push(number);
            let var = &mut self.out_vars[number];
            var.variable_type = SourceOutputVariableType::Integer;
            var.time_step_type = time_step;
            var.store_type = declaration.store_type;
            var.name = declaration.name.into();
            var.name_upper = var.name.to_ascii_uppercase();
            var.key = declaration.key.into();
            var.key_upper = var.key.to_ascii_uppercase();
            var.key_colon_name = format!("{}:{}", declaration.key, declaration.name);
            var.key_colon_name_upper = var.key_colon_name.to_ascii_uppercase();
            var.units = declaration.units;
            self.report_number_counter = self
                .report_number_counter
                .checked_add(1)
                .ok_or(OrdinaryRadiationUnavailable::StorageDomain)?;
            var.report_id = self.report_number_counter;
            var.integer_which = Some(binding.target);
            var.index_group_key = -1;
            let Some(request) = request else {
                continue;
            };
            var.report = true;
            var.frequency = self.requested_variables[request].frequency;
            var.schedule = self.requested_variables[request].reporting_schedule;
            self.write_real_report_dictionary(number, dictionary_host)?;
        }
        Ok(())
    }
    pub(crate) fn read_registered_integer<'a>(
        &self,
        number: usize,
        owners: &'a impl SourceIntegerOutputOwners,
    ) -> Result<&'a i32, OrdinaryRadiationError> {
        let target = self
            .out_vars
            .get(number)
            .and_then(|v| v.integer_which)
            .ok_or(OrdinaryRadiationUnavailable::StorageDomain)?;
        owners.integer_output(target)
    }
    pub(crate) fn requested_integer_values<'a>(
        &self,
        owners: &'a impl SourceIntegerOutputOwners,
    ) -> Result<Vec<SourceRequestedIntegerValue<'a>>, OrdinaryRadiationError> {
        self.out_vars
            .iter()
            .enumerate()
            .filter(|(_, v)| v.report && v.variable_type == SourceOutputVariableType::Integer)
            .map(|(number, v)| {
                if v.schedule.is_some() {
                    return Err(OrdinaryRadiationUnavailable::InputBranch.into());
                }
                Ok(SourceRequestedIntegerValue {
                    out_var_number: number,
                    report_id: v.report_id,
                    request: crate::RuntimeOutputRequest {
                        key: v.key.clone(),
                        variable_name: v.name.clone(),
                        frequency: super::bound_real::runtime_frequency(v.frequency)?,
                    },
                    units: v.units.symbol(),
                    target: v
                        .integer_which
                        .ok_or(OrdinaryRadiationUnavailable::StorageDomain)?,
                    value: self.read_registered_integer(number, owners)?,
                })
            })
            .collect()
    }
}
