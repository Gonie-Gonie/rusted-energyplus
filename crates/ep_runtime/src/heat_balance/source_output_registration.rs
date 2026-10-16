//! Sole reached OutputProcessor time bindings, before any variable registration.
//! Future requested-variable/dictionary storage extends this same owner.
use super::error_backend::SourceErrorBackend;
use super::selected_simulation_startup::{SelectedHvacGlobals, StartupError};
use crate::weather::day::WeatherGlobalState;
fn fatal(backend: &mut SourceErrorBackend<'_>, message: &str) -> Result<(), StartupError> {
    match backend.show_fatal_error(message) {
        Ok(never) => match never {},
        Err(error) => Err(StartupError::Fatal(error)),
    }
}
/// Original OutputProcessor Zone/System timestep discriminator.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimeStepKind {
    /// The actual DataGlobal TimeStepZone field.
    Zone,
    /// The actual DataHVACGlobals TimeStepSys field.
    System,
}

/// A concrete field identity resolved against the sole live owners on every read.
/// This represents the pointer target without unsafe addresses or cached values.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum TimeStepTarget {
    GlobalZoneTimeStep,
    HvacSystemTimeStep,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SelectedOutputTimeValue {
    pub(crate) target: Option<TimeStepTarget>,
    pub(crate) current_minute: f64,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SelectedOutputTimePointers {
    pub(crate) zone: SelectedOutputTimeValue,
    pub(crate) system: SelectedOutputTimeValue,
}

/// Root's operations boundary must borrow its same retained session/context pair.
/// The reference lifetime prevents retaining a scalar copy across later updates.
pub(crate) struct LiveTimeStepOwners<'a> {
    pub(crate) global: &'a WeatherGlobalState,
    pub(crate) hvac: &'a SelectedHvacGlobals,
}
impl SelectedOutputTimePointers {
    pub(crate) fn setup_time_pointer(
        &mut self,
        kind: TimeStepKind,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), StartupError> {
        let (slot, target, name) = match kind {
            TimeStepKind::Zone => (&mut self.zone, TimeStepTarget::GlobalZoneTimeStep, "Zone"),
            TimeStepKind::System => (
                &mut self.system,
                TimeStepTarget::HvacSystemTimeStep,
                "System",
            ),
        };
        if slot.target.is_some() {
            return fatal(
                backend,
                &format!("SetupTimePointers was already called for {name}"),
            );
        }
        slot.target = Some(target);
        Ok(())
    }

    pub(crate) fn time_step<'a>(
        &self,
        kind: TimeStepKind,
        owners: &LiveTimeStepOwners<'a>,
    ) -> Result<&'a f64, StartupError> {
        let target = match kind {
            TimeStepKind::Zone => self.zone.target,
            TimeStepKind::System => self.system.target,
        };
        match target {
            Some(TimeStepTarget::GlobalZoneTimeStep) if kind == TimeStepKind::Zone => {
                Ok(&owners.global.time_step_zone)
            }
            Some(TimeStepTarget::HvacSystemTimeStep) if kind == TimeStepKind::System => {
                Ok(&owners.hvac.time_step_sys)
            }
            _ => Err(StartupError::Unavailable(
                "OutputProcessor timestep target association",
            )),
        }
    }
}

// Reached OutputProcessor registration storage; no hourly registry substitutes for it.
// Original OutputProcessor.cc139-193,281-346,421-548,3003-3144,4827-4919.

use super::radiation::ordinary::{OrdinaryRadiationError, OrdinaryRadiationUnavailable};
use super::source_output_prescan::SourceOutputPreScan;
use crate::error::WarmupOutputError;
use std::collections::BTreeMap;
mod host;
mod requested_input;
pub use host::SourceMtdFileHost;
pub(crate) use requested_input::SourceReportVariableInput;

/// Actual output file/system host, supplied by the real Setup route.
pub(crate) trait SourceOutputFileHost {
    fn ensure_mtd_open(&mut self) -> std::io::Result<()>;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SourceOutputUnits {
    C,
    MPerS,
    Deg,
    W,
    J,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SourceOutputStoreType {
    Average,
    Sum,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum SourceReportFrequency {
    EachCall,
    TimeStep,
    Hour,
    Day,
    Month,
    Simulation,
    Year,
}

#[derive(Clone, Copy)]
pub(crate) struct SourceRealOutputDeclaration<'a> {
    pub(crate) name: &'a str,
    pub(crate) key: &'a str,
    pub(crate) units: SourceOutputUnits,
    pub(crate) store_type: SourceOutputStoreType,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct SourceDictionaryVariable {
    pub(crate) integer: bool,
    pub(crate) name: String,
    pub(crate) units: SourceOutputUnits,
    pub(crate) store_type: SourceOutputStoreType,
    pub(crate) next: Option<usize>,
    pub(crate) reported_on_dictionary_file: bool,
    pub(crate) key_out_var_nums: Vec<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct SourceRequestedVariable {
    pub(crate) key: String,
    pub(crate) name: String,
    pub(crate) frequency: SourceReportFrequency,
    pub(crate) used: bool,
    pub(crate) is_simple_string: bool,
    pub(crate) reporting_schedule: Option<super::selected_schedule_manager::SourceScheduleIndex>,
}

impl Default for SourceRequestedVariable {
    fn default() -> Self {
        Self {
            key: String::new(),
            name: String::new(),
            frequency: SourceReportFrequency::Hour,
            used: false,
            is_simple_string: true,
            reporting_schedule: None,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct SourceEndUseCategory {
    pub(crate) name: String,
    pub(crate) display_name: String,
    pub(crate) num_subcategories: i32,
    pub(crate) subcategory_names: Option<Vec<String>>,
    pub(crate) num_space_types: i32,
    pub(crate) space_type_names: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceOutputRegistration {
    pub(crate) time_pointers: SelectedOutputTimePointers,
    pub(crate) output_initialized: bool,
    pub(crate) get_output_input: bool,
    pub(crate) minimum_report_frequency: SourceReportFrequency,
    pub(crate) time_step_zone_sec: f64,
    pub(crate) end_use_categories: Option<Vec<SourceEndUseCategory>>,
    pub(crate) requested_variables: Vec<SourceRequestedVariable>,
    pub(crate) dictionary_variables: Vec<SourceDictionaryVariable>,
    dictionary_by_name: BTreeMap<String, usize>,
    pub(crate) num_total_real_variables: i32,
    pub(crate) num_real_variables: i32,
    pub(crate) num_real_variables_setup: i32,
    pub(crate) num_total_integer_variables: i32,
    pub(crate) num_integer_variables_setup: i32,
    pub(crate) num_integer_variables: i32,
    pub(crate) num_integer_variables_sum: i32,
    pub(crate) num_real_variables_sum: i32,
    pub(crate) num_real_variables_meter: i32,
    pub(crate) report_number_counter: i32,
    pub(crate) max_num_subcategories: i32,
}
impl Default for SourceOutputRegistration {
    fn default() -> Self {
        Self {
            time_pointers: SelectedOutputTimePointers::default(),
            output_initialized: false,
            get_output_input: true,
            minimum_report_frequency: SourceReportFrequency::EachCall,
            time_step_zone_sec: 0.0,
            end_use_categories: None,
            requested_variables: Vec::new(),
            dictionary_variables: Vec::new(),
            dictionary_by_name: BTreeMap::new(),
            num_total_real_variables: 0,
            num_real_variables: 0,
            num_real_variables_setup: 0,
            num_total_integer_variables: 0,
            num_integer_variables_setup: 0,
            num_integer_variables: 0,
            num_integer_variables_sum: 0,
            num_real_variables_sum: 0,
            num_real_variables_meter: 0,
            report_number_counter: 0,
            max_num_subcategories: 1,
        }
    }
}

impl SourceOutputRegistration {
    fn initialize_output(
        &mut self,
        minutes_in_time_step: i32,
        host: &mut impl SourceOutputFileHost,
    ) -> Result<(), OrdinaryRadiationError> {
        const NAMES: [&str; 14] = [
            "Heating",
            "Cooling",
            "InteriorLights",
            "ExteriorLights",
            "InteriorEquipment",
            "ExteriorEquipment",
            "Fans",
            "Pumps",
            "HeatRejection",
            "Humidifier",
            "HeatRecovery",
            "WaterSystems",
            "Refrigeration",
            "Cogeneration",
        ];
        const DISPLAY: [&str; 14] = [
            "Heating",
            "Cooling",
            "Interior Lighting",
            "Exterior Lighting",
            "Interior Equipment",
            "Exterior Equipment",
            "Fans",
            "Pumps",
            "Heat Rejection",
            "Humidification",
            "Heat Recovery",
            "Water Systems",
            "Refrigeration",
            "Generators",
        ];
        self.end_use_categories = Some(
            NAMES
                .iter()
                .zip(DISPLAY)
                .map(|(&name, display)| SourceEndUseCategory {
                    name: name.into(),
                    display_name: display.into(),
                    ..Default::default()
                })
                .collect(),
        );
        self.output_initialized = true;
        self.time_step_zone_sec = f64::from(minutes_in_time_step) * 60.0;
        host.ensure_mtd_open()
            .map_err(|error| OrdinaryRadiationError::Output(WarmupOutputError::Io(error)))
    }

    fn add_dictionary_variable(&mut self, declaration: SourceRealOutputDeclaration<'_>) -> usize {
        self.add_typed_dictionary_variable(declaration, false)
    }
    fn add_typed_dictionary_variable(
        &mut self,
        declaration: SourceRealOutputDeclaration<'_>,
        integer: bool,
    ) -> usize {
        let name_upper = declaration.name.to_ascii_uppercase();
        if let Some(&first) = self.dictionary_by_name.get(&name_upper) {
            let mut index = first;
            loop {
                if self.dictionary_variables[index].units == declaration.units {
                    return index;
                }
                match self.dictionary_variables[index].next {
                    Some(next) => index = next,
                    None => {
                        self.dictionary_variables[index].next =
                            Some(self.dictionary_variables.len());
                        break;
                    }
                }
            }
        } else {
            self.dictionary_by_name
                .insert(name_upper, self.dictionary_variables.len());
        }
        let index = self.dictionary_variables.len();
        self.dictionary_variables.push(SourceDictionaryVariable {
            integer,
            name: declaration.name.into(),
            units: declaration.units,
            store_type: declaration.store_type,
            next: None,
            reported_on_dictionary_file: false,
            key_out_var_nums: Vec::new(),
        });
        index
    }

    pub(crate) fn prepare_real_registration(
        &mut self,
        input: &mut SourceReportVariableInput<'_>,
        prescan: &SourceOutputPreScan,
        declaration: SourceRealOutputDeclaration<'_>,
        minutes_in_time_step: i32,
        host: &mut impl SourceOutputFileHost,
        eio: &mut dyn std::io::Write,
    ) -> Result<bool, OrdinaryRadiationError> {
        if !self.output_initialized {
            self.initialize_output(minutes_in_time_step, host)?;
        }
        self.get_report_variable_input(input, host, eio)?;
        let mut request_indexes: Vec<usize> = Vec::new();
        for index in 0..self.requested_variables.len() {
            let request = &mut self.requested_variables[index];
            if !request.name.eq_ignore_ascii_case(declaration.name)
                || !(request.key.is_empty() || request.key.eq_ignore_ascii_case(declaration.key))
            {
                continue;
            }
            request.used = true;
            let frequency = request.frequency;
            if !request_indexes.iter().any(|&prior| {
                self.requested_variables[prior].frequency == frequency
                    && self.requested_variables[prior].reporting_schedule
                        == self.requested_variables[index].reporting_schedule
            }) {
                request_indexes.push(index);
            }
        }
        let this_one_on_the_list = prescan.find_item(declaration.key, declaration.name);
        self.add_dictionary_variable(declaration);
        self.num_real_variables_setup = self
            .num_real_variables_setup
            .checked_add(1)
            .ok_or(OrdinaryRadiationUnavailable::StorageDomain)?;
        self.num_total_real_variables = self
            .num_total_real_variables
            .checked_add(
                i32::try_from(request_indexes.len().max(1))
                    .map_err(|_| OrdinaryRadiationUnavailable::StorageDomain)?,
            )
            .ok_or(OrdinaryRadiationUnavailable::StorageDomain)?;
        Ok(this_one_on_the_list)
    }

    /// Literal no-meter return: dictionary/setup/total writes still occur.
    pub(crate) fn setup_unmetered_zone_real(
        &mut self,
        input: &mut SourceReportVariableInput<'_>,
        prescan: &SourceOutputPreScan,
        declaration: SourceRealOutputDeclaration<'_>,
        minutes_in_time_step: i32,
        host: &mut impl SourceOutputFileHost,
        eio: &mut dyn std::io::Write,
    ) -> Result<(), OrdinaryRadiationError> {
        let prefix = self.prepare_real_registration(
            input,
            prescan,
            declaration,
            minutes_in_time_step,
            host,
            eio,
        )?;
        if prefix {
            // Actual OutVar/report-dictionary writer needs its own consumed port.
            return Err(OrdinaryRadiationUnavailable::InputBranch.into());
        }
        Ok(())
    }
}
