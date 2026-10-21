//! Actual ManageThermalComfort105-164 and SimpleASH55 on retained People/Air owners.
use crate::heat_balance::ctf_initialization_context::internal_heat_gains_input::SelectedInternalHeatGains;
use crate::heat_balance::ctf_reports::PredefinedStringTable;
use crate::heat_balance::ctf_thermal_history::CtfThermalHistoryState;
use crate::heat_balance::error_backend::{SourceDiagnosticFlags, SourceErrorBackend};
use crate::heat_balance::ordinary_air_heat_balance::SourceAirHeatBalance;
use crate::heat_balance::ordinary_air_heat_balance::environment::SourceZoneFanHeatBalanceArrays;
use crate::heat_balance::ordinary_air_heat_balance::predictor_corrector::SourceZoneAirPredictorCorrector;
use crate::heat_balance::predefined_tables::PredefinedRegistry;
use crate::heat_balance::radiation::ordinary::OrdinarySolarRadiation;
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_heat_balance_input::surface::SurfaceGeometryInputState;
use crate::heat_balance::selected_heat_balance_input::{
    HeatBalanceInputError as Error, SelectedHeatBalanceInputState, zone::SelectedZoneInputOwners,
};
use crate::heat_balance::selected_input_processor::{
    SelectedInputProcessor, SourceInputProcessorShortCut,
};
use crate::heat_balance::selected_schedule_manager::SelectedScheduleManager;
use crate::heat_balance::selected_simulation_startup::SelectedHvacGlobals;
use crate::heat_balance::source_initialization::SourceRecurringErrorState;
use crate::heat_balance::source_output_prescan::SourceOutputPreScan;
use crate::heat_balance::source_output_registration::{
    SourceOutputDictionaryHost, SourceOutputFileHost, SourceOutputRegistration,
};
use crate::psychrometrics::{SourcePsychrometricAccess, SourcePsychrometricOwners};
use crate::weather::day::{WeatherEnvironmentState, WeatherGlobalState};
use ep_model::ReportInputs;
use std::io::Write;
mod fanger;
mod initialization;
mod output;
mod setpoint;
mod simple;
mod state;
pub(crate) use state::{SourceComfortPredefinedSummary, SourceThermalComfort};

pub(crate) struct SourceThermalComfortOwners<'a, 'backend, H, D> {
    pub(crate) gains: &'a mut SelectedInternalHeatGains,
    pub(crate) schedules: &'a SelectedScheduleManager,
    pub(crate) developer: &'a bool,
    pub(crate) zones: &'a SelectedZoneInputOwners,
    pub(crate) control: &'a SelectedHeatBalanceInputState,
    pub(crate) globals: &'a SelectedHeatBalanceGlobals,
    pub(crate) clock: &'a WeatherGlobalState,
    pub(crate) environment: &'a WeatherEnvironmentState,
    pub(crate) hvac: &'a SelectedHvacGlobals,
    pub(crate) air: &'a SourceAirHeatBalance,
    pub(crate) controller: &'a SourceZoneAirPredictorCorrector,
    pub(crate) fan: &'a mut SourceZoneFanHeatBalanceArrays,
    pub(crate) radiation: &'a OrdinarySolarRadiation,
    pub(crate) geometry: &'a mut SurfaceGeometryInputState,
    pub(crate) history: &'a CtfThermalHistoryState,
    pub(crate) psychrometrics: &'a mut SourcePsychrometricAccess,
    pub(crate) recurring_errors: &'a mut SourceRecurringErrorState,
    pub(crate) input: &'a mut SelectedInputProcessor,
    pub(crate) source_is_epjson: &'a bool,
    pub(crate) shortcut: &'a mut SourceInputProcessorShortCut,
    pub(crate) registration: &'a mut SourceOutputRegistration,
    pub(crate) prescan: &'a SourceOutputPreScan,
    pub(crate) report_inputs: &'a ReportInputs,
    pub(crate) minimum_report_frequency: &'a str,
    pub(crate) host: &'a mut H,
    pub(crate) dictionary_host: &'a mut D,
    pub(crate) eio: &'a mut dyn Write,
    pub(crate) backend: &'a mut SourceErrorBackend<'backend>,
    pub(crate) registry: &'a mut PredefinedRegistry,
    pub(crate) table: &'a mut PredefinedStringTable,
    pub(crate) predefined_summary: &'a mut SourceComfortPredefinedSummary,
}
fn index(number: i32) -> Result<usize, Error> {
    number
        .checked_sub(1)
        .and_then(|n| usize::try_from(n).ok())
        .ok_or(Error::Unavailable("positive actual ThermalComfort ordinal"))
}
fn count(number: i32) -> Result<usize, Error> {
    usize::try_from(number)
        .map_err(|_| Error::Unavailable("actual ThermalComfort allocation count"))
}
fn zone_name(zones: &SelectedZoneInputOwners, number: i32) -> Result<&str, Error> {
    zones
        .zones
        .as_ref()
        .and_then(|v| v.get(index(number).ok()?))
        .map(|z| z.name.as_str())
        .ok_or(Error::Unavailable("same actual ThermalComfort Zone Name"))
}
fn column(registry: &PredefinedRegistry, name: &str) -> Result<i32, Error> {
    registry
        .named_indices
        .get(name)
        .copied()
        .ok_or(Error::Unavailable(
            "same actual ThermalComfort predefined named ID",
        ))
}
fn table_entry(
    registry: &PredefinedRegistry,
    table: &mut PredefinedStringTable,
    name: &str,
    key: &str,
    value: f64,
) -> Result<(), Error> {
    table
        .fixed_real_entry_2(column(registry, name)?, key, value)
        .map_err(|_| Error::Unavailable("actual ThermalComfort predefined Real entry"))
}
fn footnote(
    registry: &mut PredefinedRegistry,
    clock: &WeatherGlobalState,
    subtable: &str,
) -> Result<(), Error> {
    let text = match clock.kind_of_sim {
        1 => "Aggregated over the Design Days",
        2 => "Aggregated over the RunPeriods for Design",
        3 => "Aggregated over the RunPeriods for Weather",
        _ => return Ok(()),
    };
    registry
        .add_footnote(column(registry, subtable)?, text)
        .map_err(|_| Error::Unavailable("actual ThermalComfort predefined footnote"))
}
impl SourceThermalComfort {
    pub(crate) fn manage_source_thermal_comfort<
        H: SourceOutputFileHost,
        D: SourceOutputDictionaryHost,
    >(
        &mut self,
        initialize_only: bool,
        o: &mut SourceThermalComfortOwners<'_, '_, H, D>,
    ) -> Result<(), Error> {
        if self.first_time_flag {
            self.initialize(o)?;
            self.first_time_flag = false;
        }
        if o.clock.day_of_sim == 1 {
            if o.clock.hour_of_day < 7 {
                self.temporary_six_am_temperature = 1.868132;
            } else if o.clock.hour_of_day == 7 && o.clock.time_step == 1 {
                self.temporary_six_am_temperature = o.environment.current_weather.out_dry_bulb_temp;
            }
        } else if o.clock.hour_of_day == 7 && o.clock.time_step == 1 {
            self.temporary_six_am_temperature = o.environment.current_weather.out_dry_bulb_temp;
        }
        if initialize_only {
            return Ok(());
        }
        if o.clock.begin_envrn_flag {
            self.zone_occ_hrs
                .as_mut()
                .ok_or(Error::Unavailable("actual dimensioned ZoneOccHrs"))?
                .fill(0.0);
        }
        if !o.globals.doing_sizing && !o.clock.warmup_flag {
            self.calc_source_fanger(o)?;
            if o.control.any_thermal_comfort_pierce_model {
                return Err(Error::Unavailable(
                    "actual CalcThermalComfortPierceASHRAE positive model",
                ));
            }
            if o.control.any_thermal_comfort_ksu_model {
                return Err(Error::Unavailable(
                    "actual CalcThermalComfortKSU positive model",
                ));
            }
            if o.control.any_thermal_comfort_cooling_effect_model {
                return Err(Error::Unavailable(
                    "actual CalcThermalComfortCoolingEffectASH positive model",
                ));
            }
            if o.control.any_thermal_comfort_ankle_draft_model {
                return Err(Error::Unavailable(
                    "actual CalcThermalComfortAnkleDraftASH positive model",
                ));
            }
            self.calc_source_simple_ash55(o)?;
            self.calc_source_if_setpoint_met(o)?;
            if o.control.adaptive_comfort_requested_ash55 {
                return Err(Error::Unavailable(
                    "actual CalcThermalComfortAdaptiveASH55 positive model",
                ));
            }
            if o.control.adaptive_comfort_requested_cen15251 {
                return Err(Error::Unavailable(
                    "actual CalcThermalComfortAdaptiveCEN15251 positive model",
                ));
            }
        }
        Ok(())
    }
}
