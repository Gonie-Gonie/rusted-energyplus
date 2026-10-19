//! Sole reached OutputProcessor time bindings and Source variable registrations.
//! Requested real variables retain stable identities resolved against live owners.
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
mod air_target;
mod purchased_air_target;
pub(crate) use purchased_air_target::{SourcePurchasedAirOutputField, SourcePurchasedAirReportingOutputContext};
mod node_target;
pub(crate) use node_target::{SourceNodeOutputField, SourceNodeReportingOutputContext};
mod system_target;
pub(crate) use system_target::{SourceAllIntegerOutputOwners, SourceHvacRealOutputField};
mod bound_real;
mod exterior_light_target;
mod facility_target;
mod system_reports_target;
pub(crate) use exterior_light_target::SourceExteriorLightOutputField;
pub(crate) use system_reports_target::{SourceSystemReportsOutputField, SourceZoneVentReportOutputField};
mod general_real;
mod meters;
mod metered_variables;
pub(crate) use metered_variables::{SourceMeterData, SourceMeteredVariable, SourceMeterHeatOrCool};
mod weather_target;
pub(crate) use facility_target::{SourceFacilityOutputField, SourceNonFacilityOutputOwners};
pub(crate) use weather_target::{SourceWeatherIntegerOutputField, SourceWeatherRealOutputField};
mod instant_meter;
mod meter_reporting;
mod milo_dtoa;
/// Same Native Milo dtoa used by InputProcessor numeric-to-alpha getters.
pub(crate) fn source_json_alpha_number(value: f64) -> Result<String, crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError> {
    milo_dtoa::source_dtoa(value)
}
mod report_for_tabular;
mod report_streams;
mod report_writers;
mod results_meter;
mod update_data;
mod update_meters;
pub(crate) use report_streams::SourceOutputReportStreams;
pub(crate) use results_meter::{SourceMeterDataFrame, add_source_report_meter};
pub(crate) use update_data::{SourceOutputUpdateEnvironment, SourceOutputUpdateOwners};
mod pollution_target;
pub(crate) use air_target::{
    SourceAirOutputField, SourceAllOutputOwners, SourceComfortOutputField,
};
pub(crate) use general_real::{
    SourceGeneralRealRegistrationOptions, SourceRealRegistrationOptions,
};
pub(crate) use meters::{SourceEnergyTransferMeterArguments, SourceMeterEndUse};
pub(crate) use meters::{SourceGeneralMeterArguments, SourceMeterGroup, SourceMeterResource};
mod solar_target;
pub(crate) use solar_target::{SourceCombinedOutputOwners, SourceSolarOutputField};
mod bound_integer;
mod dictionary;
mod meter_request_input;
mod produce_dictionaries;
pub(crate) use meter_request_input::{SourceMeterReportingHost, SourceMeterReportingOwners};
mod host;
mod integer;
mod oscillation_target;
mod requested_input;
pub(crate) use bound_integer::{
    SourceIntegerOutputBinding, SourceIntegerOutputOwners, SourceIntegerOutputTarget,
    SourceRequestedIntegerValue,
};
pub(crate) use bound_real::{
    SourceOutVarReal, SourceRealOutputBinding, SourceRealOutputOwners, SourceRealOutputTarget,
    SourceRequestedRealValue, SourceSurfaceOutputField,
};
pub(crate) use dictionary::{SourceOutputDictionaryHost, SourceOutputDictionaryStreams};
pub use host::SourceMtdFileHost;
pub(crate) use integer::SourceIntegerOutputDeclaration;
pub(crate) use oscillation_target::SourceOscillationOutputField;
pub(crate) use pollution_target::SourceCarbonEquivalentField;
pub(crate) use requested_input::SourceReportVariableInput;

/// Actual output file/system host, supplied by the real Setup route.
pub(crate) trait SourceOutputFileHost {
    fn ensure_mtd_open(&mut self) -> std::io::Result<()>;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SourceOutputUnits {
    Invalid,
    Dimensionless,
    Percentage,
    KgWaterPerSecond,
    Pa,
    M,
    KgPerM3,
    Lux,
    LumPerW,
    KgWaterPerKgDryAir,
    JPerKg,
    Hr,
    C,
    MPerS,
    Deg,
    M2,
    Rad,
    W,
    J,
    Kg,
    WPerM2,
    WPerM2K,
    KgPerSecond,
    M3PerSecond,
    M3,
    Ach,
    JPerKgK,
    Ppm,
}
impl SourceOutputUnits {
    pub(crate) fn symbol(self) -> &'static str {
        match self {
            Self::Invalid | Self::Dimensionless => "",
            Self::C => "C",
            Self::Pa => "Pa",
            Self::M => "m",
            Self::KgPerM3 => "kg/m3",
            Self::Lux => "lux",
            Self::LumPerW => "lum/W",
            Self::Percentage => "%",
            Self::Hr => "hr",
            Self::KgWaterPerSecond => "kgWater/s",
            Self::KgWaterPerKgDryAir => "kgWater/kgDryAir",
            Self::JPerKg => "J/kg",
            Self::MPerS => "m/s",
            Self::Deg => "deg",
            Self::M2 => "m2",
            Self::Rad => "rad",
            Self::W => "W",
            Self::J => "J",
            Self::Kg => "kg",
            Self::WPerM2 => "W/m2",
            Self::WPerM2K => "W/m2-K",
            Self::KgPerSecond => "kg/s",
            Self::M3PerSecond => "m3/s",
            Self::M3 => "m3",
            Self::Ach => "ach",
            Self::JPerKgK => "J/kg-K",
            Self::Ppm => "ppm",
        }
    }
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
    pub(crate) time_step_type: TimeStepKind,
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
    // Native OutputProcessor.hh931: assigned only by actual ProduceRDDMDD.
    produce_report_vdd: produce_dictionaries::SourceReportVdd,
    /// Actual OutputProcessor.hh932–948 constructor members, never seeded.
    pub(crate) number_hours_in_month: i32,
    pub(crate) number_hours_in_simulation: i32,
    pub(crate) meter_values: Vec<f64>,
    pub(crate) frequency_stamp_report_numbers: [i32; 7],
    pub(crate) errors_logged: bool,
    pub(crate) is_final_year: bool,
    pub(crate) last_hour_printed: i32,
    pub(crate) last_start_minute: f64,
    pub(crate) last_end_minute: f64,
    pub(crate) time_pointers: SelectedOutputTimePointers,
    pub(crate) output_initialized: bool,
    pub(crate) get_output_input: bool,
    pub(crate) minimum_report_frequency: SourceReportFrequency,
    pub(crate) time_step_zone_sec: f64,
    pub(crate) end_use_categories: Option<Vec<SourceEndUseCategory>>,
    pub(crate) requested_variables: Vec<SourceRequestedVariable>,
    pub(crate) dictionary_variables: Vec<SourceDictionaryVariable>,
    pub(crate) out_vars: Vec<SourceOutVarReal>,
    pub(crate) meters: meters::SourceMeters,
    pub(crate) frequency_tracking_variables: [bool; 7],
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
    pub(crate) max_num_end_use_space_types: i32, // Native OutputProcessor.hh968
}
impl Default for SourceOutputRegistration {
    fn default() -> Self {
        Self {
            produce_report_vdd: produce_dictionaries::SourceReportVdd::No,
            number_hours_in_month: 0,
            number_hours_in_simulation: 0,
            meter_values: Vec::new(),
            frequency_stamp_report_numbers: [-1; 7],
            errors_logged: false,
            is_final_year: false,
            last_hour_printed: -1,
            last_start_minute: -1.0,
            last_end_minute: -1.0,
            time_pointers: SelectedOutputTimePointers::default(),
            output_initialized: false,
            get_output_input: true,
            minimum_report_frequency: SourceReportFrequency::EachCall,
            time_step_zone_sec: 0.0,
            end_use_categories: None,
            requested_variables: Vec::new(),
            dictionary_variables: Vec::new(),
            out_vars: Vec::new(),
            meters: meters::SourceMeters::default(),
            frequency_tracking_variables: [false; 7],
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
            max_num_end_use_space_types: 1,
        }
    }
}

struct SourceRealRegistrationSelection {
    request_indexes: Vec<usize>,
    dictionary_number: usize,
    on_the_list: bool,
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

    fn add_typed_dictionary_variable(
        &mut self,
        declaration: SourceRealOutputDeclaration<'_>,
        integer: bool,
    ) -> usize {
        self.add_typed_dictionary_variable_at(declaration, integer, TimeStepKind::Zone)
    }
    fn add_typed_dictionary_variable_at(
        &mut self,
        declaration: SourceRealOutputDeclaration<'_>,
        integer: bool,
        time_step_type: TimeStepKind,
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
            time_step_type,
            name: declaration.name.into(),
            units: declaration.units,
            store_type: declaration.store_type,
            next: None,
            reported_on_dictionary_file: false,
            key_out_var_nums: Vec::new(),
        });
        index
    }

    fn match_report_variables(&mut self, name: &str, key: &str) -> Vec<usize> {
        let mut request_indexes: Vec<usize> = Vec::new();
        for index in 0..self.requested_variables.len() {
            let request = &mut self.requested_variables[index];
            if !request.name.eq_ignore_ascii_case(name)
                || !(request.key.is_empty() || request.key.eq_ignore_ascii_case(key))
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
        request_indexes
    }

    fn prepare_real_registration_details(
        &mut self,
        input: &mut SourceReportVariableInput<'_>,
        prescan: &SourceOutputPreScan,
        declaration: SourceRealOutputDeclaration<'_>,
        minutes_in_time_step: i32,
        host: &mut impl SourceOutputFileHost,
        eio: &mut dyn std::io::Write,
    ) -> Result<SourceRealRegistrationSelection, OrdinaryRadiationError> {
        let (request_indexes, this_one_on_the_list) = self.prepare_real_registration_front(
            input,
            prescan,
            declaration,
            minutes_in_time_step,
            host,
            eio,
        )?;
        self.finish_real_registration_prefix(
            declaration,
            TimeStepKind::Zone,
            request_indexes,
            this_one_on_the_list,
        )
    }
    fn prepare_real_registration_front(
        &mut self,
        input: &mut SourceReportVariableInput<'_>,
        prescan: &SourceOutputPreScan,
        declaration: SourceRealOutputDeclaration<'_>,
        minutes_in_time_step: i32,
        host: &mut impl SourceOutputFileHost,
        eio: &mut dyn std::io::Write,
    ) -> Result<(Vec<usize>, bool), OrdinaryRadiationError> {
        if !self.output_initialized {
            self.initialize_output(minutes_in_time_step, host)?;
        }
        self.get_report_variable_input(input, host, eio)?;
        let request_indexes = self.match_report_variables(declaration.name, declaration.key);
        let this_one_on_the_list = prescan.find_item(declaration.key, declaration.name);
        Ok((request_indexes, this_one_on_the_list))
    }
    fn finish_real_registration_prefix(
        &mut self,
        declaration: SourceRealOutputDeclaration<'_>,
        time_step_type: TimeStepKind,
        request_indexes: Vec<usize>,
        this_one_on_the_list: bool,
    ) -> Result<SourceRealRegistrationSelection, OrdinaryRadiationError> {
        let dictionary_number =
            self.add_typed_dictionary_variable_at(declaration, false, time_step_type);
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
        Ok(SourceRealRegistrationSelection {
            request_indexes,
            dictionary_number,
            on_the_list: this_one_on_the_list,
        })
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
        self.prepare_real_registration_details(
            input,
            prescan,
            declaration,
            minutes_in_time_step,
            host,
            eio,
        )
        .map(|selected| selected.on_the_list)
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
