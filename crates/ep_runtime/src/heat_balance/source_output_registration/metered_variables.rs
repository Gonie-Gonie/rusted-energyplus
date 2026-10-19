//! Genuine OutputProcessor4306-4413 metadata getters and589-613 constructors.
use super::bound_real::SourceOutputVariableType;
use super::{SourceMeterEndUse, SourceMeterGroup, SourceMeterResource, SourceOutputRegistration, SourceOutputUnits, TimeStepKind};
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceMeteredVariable {
    pub(crate) number: i32,
    pub(crate) name: String,
    pub(crate) resource: SourceMeterResource,
    pub(crate) units: SourceOutputUnits,
    /// None represents the genuine Native enum Invalid default, not unknown.
    pub(crate) variable_type: Option<SourceOutputVariableType>,
    /// None represents the genuine Native enum Invalid default, not unknown.
    pub(crate) time_step_type: Option<TimeStepKind>,
    pub(crate) end_use_category: SourceMeterEndUse,
    pub(crate) group: SourceMeterGroup,
    pub(crate) report_number: i32,
}
impl Default for SourceMeteredVariable {
    fn default() -> Self {
        Self { number: -1, name: String::new(), resource: SourceMeterResource::Invalid,
            units: SourceOutputUnits::Invalid, variable_type: None, time_step_type: None,
            end_use_category: SourceMeterEndUse::Invalid, group: SourceMeterGroup::Invalid,
            report_number: -1 }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub(crate) enum SourceMeterHeatOrCool {
    Invalid = -1, NoHeatNoCool = 0, CoolingOnly = 1, HeatingOnly = 2, HeatAndCool = 3, Num = 4,
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceMeterData {
    pub(crate) variable: SourceMeteredVariable,
    pub(crate) heat_or_cool: SourceMeterHeatOrCool,
    pub(crate) current_meter_reading: f64,
}
impl Default for SourceMeterData {
    fn default() -> Self {
        Self { variable: SourceMeteredVariable::default(), heat_or_cool: SourceMeterHeatOrCool::NoHeatNoCool,
            current_meter_reading: 0.0 }
    }
}

impl SourceOutputRegistration {
    pub(crate) fn get_source_number_metered_variables(&self, _component_type: &str, component_name: &str) -> Result<i32, Error> {
        let mut number = 0_i32;
        for variable in &self.out_vars {
            if variable.variable_type != SourceOutputVariableType::Real { continue; }
            if component_name != variable.key_upper { continue; }
            if !variable.meter_numbers.is_empty() {
                number = number.checked_add(1).ok_or(Error::Unavailable("undefined GetNumMeteredVariables4335 count"))?;
            }
        }
        Ok(number)
    }

    pub(crate) fn get_source_metered_variables(&self, component_name: &str, variables: &mut [SourceMeteredVariable]) -> Result<i32, Error> {
        let mut number = 0_i32;
        for (index, variable) in self.out_vars.iter().enumerate() {
            if variable.variable_type != SourceOutputVariableType::Real { continue; }
            if component_name != variable.key_upper { continue; }
            if variable.meter_numbers.is_empty() { continue; }
            number = number.checked_add(1).ok_or(Error::Unavailable("undefined GetMeteredVariables4381 count"))?;
            let destination = usize::try_from(number - 1).map_err(|_|Error::Unavailable("actual GetMeteredVariables ordinal"))?;
            let metered = variables.get_mut(destination).ok_or(Error::Unavailable("actual GetMeteredVariables4381 caller allocation"))?;
            metered.number = i32::try_from(index).map_err(|_|Error::Unavailable("actual GetMeteredVariables4383 zero-based outVars index"))?;
            metered.variable_type = Some(SourceOutputVariableType::Real);
            metered.time_step_type = Some(variable.time_step_type);
            metered.units = variable.units;
            let first_number = usize::try_from(variable.meter_numbers[0]).map_err(|_|Error::Unavailable("actual GetMeteredVariables4388 zero-based meter index"))?;
            metered.resource = self.meters.meters.get(first_number).ok_or(Error::Unavailable("actual GetMeteredVariables4388 first meter resource"))?.resource;
            metered.name = variable.key_colon_name_upper.clone();
            let mut found_end_use = false;
            let mut found_group = false;
            for &meter_number in &variable.meter_numbers {
                let index = usize::try_from(meter_number).map_err(|_|Error::Unavailable("actual GetMeteredVariables4394 zero-based meter index"))?;
                let meter = self.meters.meters.get(index).ok_or(Error::Unavailable("actual GetMeteredVariables4394 meter"))?;
                if !found_end_use && meter.end_use != SourceMeterEndUse::Invalid {
                    metered.end_use_category = meter.end_use;
                    found_end_use = true;
                }
                if !found_group && meter.group != SourceMeterGroup::Invalid {
                    metered.group = meter.group;
                    found_group = true;
                }
                if found_end_use && found_group { break; }
            }
            metered.report_number = variable.report_id;
        }
        Ok(number)
    }
}
