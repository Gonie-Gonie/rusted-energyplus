//! Actual retained ThirdOrder Zone/Space caller bodies; no legacy air copy.
use super::{SourceAirHeatBalance, SourceZoneSpaceHeatBalanceData, index};
use crate::heat_balance::ctf_initialization_context::internal_heat_gains_input::SelectedInternalHeatGains;
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::ordinary_surface_initialization::SourceSurfaceHeatBalanceArrays;
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_heat_balance_input::{
    HeatBalanceInputError as Error, SelectedHeatBalanceInputState, SelectedZoneAirSolution,
    surface::{SurfaceClass, SurfaceGeometryInputState},
    zone::SelectedZoneInputOwners,
};
use crate::heat_balance::selected_heat_balance_space_controls::SelectedHeatBalanceSpaceControls;
use crate::heat_balance::selected_input_processor::{
    SelectedInputProcessor, SourceInputProcessorShortCut,
};
use crate::heat_balance::selected_simulation_startup::SelectedHvacGlobals;
use crate::psychrometrics::{
    EnergyPlusPsychrometricFunction, SourcePsychrometricAccess, energyplus_psy_cp_air_fn_w,
    energyplus_psy_h_fn_tdb_w, energyplus_psy_hg_air_fn_w_tdb, energyplus_psy_rho_air_fn_pb_tdb_w,
};
use crate::weather::day::{WeatherEnvironmentState, WeatherGlobalState};
mod component_report;
mod demand;
mod gains;
mod history;
mod humidity;
mod lifecycle;
pub(crate) mod oscillation;
pub(crate) mod simple_air;
mod subsystems;
mod sums;
mod temperature;
pub(crate) use demand::{
    SourceAirReport, SourceSystemMoistureDemand, SourceSystemSensibleDemand, SourceZoneAirDemand,
    SourceZoneAirReports,
};
pub(crate) use lifecycle::{
    SourceAirThermostatArrays, SourceAirUpdate, SourceZoneAirRegistrationChildren,
    SourceZoneListGroupLoadReports,
};
pub(crate) use subsystems::SourceZoneAirSubsystems;

pub(crate) struct SourceZoneAirOwners<'a, 'b> {
    pub(crate) zones: &'a mut SelectedZoneInputOwners,
    pub(crate) equipment: &'a mut crate::heat_balance::ordinary_equipment::OrdinaryEquipment,
    pub(crate) geometry: &'a SurfaceGeometryInputState,
    pub(crate) surface_arrays: &'a SourceSurfaceHeatBalanceArrays,
    pub(crate) fan: &'a mut super::environment::SourceZoneFanHeatBalanceArrays,
    pub(crate) gains: &'a SelectedInternalHeatGains,
    pub(crate) control: &'a mut SelectedHeatBalanceInputState,
    pub(crate) space_controls: &'a SelectedHeatBalanceSpaceControls,
    pub(crate) globals: &'a SelectedHeatBalanceGlobals,
    pub(crate) clock: &'a WeatherGlobalState,
    pub(crate) environment: &'a WeatherEnvironmentState,
    pub(crate) hvac: &'a mut SelectedHvacGlobals,
    pub(crate) input: &'a mut SelectedInputProcessor,
    pub(crate) shortcut: &'a mut SourceInputProcessorShortCut,
    pub(crate) source_is_epjson: &'a bool,
    pub(crate) faults: &'a crate::heat_balance::source_initialization::SourceFaultsManager,
    pub(crate) schedules: &'a mut crate::heat_balance::selected_schedule_manager::SelectedScheduleManager,
    pub(crate) availability: &'a crate::heat_balance::ordinary_equipment::management::availability::SourceAvailability,
    pub(crate) psychrometrics: &'a mut SourcePsychrometricAccess,
    pub(crate) backend: &'a mut SourceErrorBackend<'b>,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceZoneAirPredictorCorrector {
    pub(crate) subsystems: SourceZoneAirSubsystems,
    pub(crate) demand: SourceZoneAirDemand,
    pub(crate) reports: SourceZoneAirReports,
    pub(crate) controls: lifecycle::SourceZoneAirControls,
    pub(crate) flags: lifecycle::SourceZoneAirFlags,
    pub(crate) simple_air: simple_air::SourceSimpleAir,
    pub(crate) oscillation: oscillation::SourceZoneTemperatureOscillation,
}
fn value(values: &Option<Vec<f64>>, i: usize) -> Result<f64, Error> {
    values
        .as_deref()
        .and_then(|v| v.get(i))
        .copied()
        .ok_or(Error::Unavailable("same actual Source air scalar array"))
}
fn cpp_max(a: f64, b: f64) -> f64 {
    if a < b { b } else { a }
}
fn cpp_min(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
}
fn zone<'a>(
    owners: &'a SourceZoneAirOwners<'_, '_>,
    number: i32,
) -> Result<&'a crate::heat_balance::selected_heat_balance_input::zone::SelectedZoneData, Error> {
    owners
        .zones
        .zones
        .as_deref()
        .and_then(|rows| rows.get(index(number).ok()?))
        .ok_or(Error::Unavailable("same actual Source Zone ordinal"))
}
fn relative_humidity(
    psy: &mut SourcePsychrometricAccess,
    temperature: f64,
    humidity: f64,
    pressure: f64,
) -> Result<f64, Error> {
    let pws = psy
        .evaluate(EnergyPlusPsychrometricFunction::Psat, &[temperature])
        .map_err(|_| Error::Unavailable("actual Source RH Psat instance/cache"))?
        .result;
    let w = cpp_max(humidity, 1.0e-5);
    let u = w / (0.62198 * pws / (pressure - pws));
    let rh = u / (1.0 - (1.0 - u) * (pws / pressure));
    if rh < 0.0 || rh > 1.0 {
        if rh < -0.05 || rh > 1.01 {
            return Err(Error::Unavailable(
                "positive Source PsyRhFnTdbWPb_error1263",
            ));
        }
        return Ok(cpp_min(cpp_max(rh, 0.01), 1.0));
    }
    Ok(rh)
}
impl SourceAirHeatBalance {
    /// Genuine correctZoneAirTemps3817-3860 Zone-first then actual Space writes.
    pub(crate) fn correct_source_zone_air_temperatures(
        &mut self,
        use_zone_time_step_history: bool,
        controller: &mut SourceZoneAirPredictorCorrector,
        owners: &mut SourceZoneAirOwners<'_, '_>,
    ) -> Result<f64, Error> {
        let mut maximum_change = 0.0;
        for number in 1..=owners.globals.num_of_zones {
            let change = self
                .zones
                .as_mut()
                .and_then(|rows| rows.get_mut(index(number).ok()?))
                .ok_or(Error::Unavailable("actual CorrectStep Zone AirHB"))?
                .air
                .correct_source_zone_temperature(
                    number,
                    use_zone_time_step_history,
                    controller,
                    owners,
                )?;
            let spaces = zone(owners, number)?.space_indexes.clone(); // ordinal traversal only, no scientific clone
            for space_number in spaces {
                if owners.space_controls.simulation && !owners.globals.doing_sizing {
                    return Err(Error::Unavailable("positive Space correctAirTemp3831"));
                } else {
                    if owners.space_controls.sizing && zone(owners, number)?.is_controlled {
                        return Err(Error::Unavailable("positive sized Space node handoff3836"));
                    }
                    let from = &self
                        .zones
                        .as_ref()
                        .and_then(|rows| rows.get(index(number).ok()?))
                        .ok_or(Error::Unavailable("actual CorrectStep Zone read"))?
                        .air;
                    let to = &mut self
                        .spaces
                        .as_mut()
                        .and_then(|rows| rows.get_mut(index(space_number).ok()?))
                        .ok_or(Error::Unavailable("actual CorrectStep Space AirHB"))?
                        .air;
                    to.zt = from.zt;
                    to.ztm = from.ztm;
                    to.mat = from.mat;
                    to.air_hum_rat = from.air_hum_rat;
                    to.air_rel_hum = from.air_rel_hum;
                }
            }
            maximum_change = cpp_max(maximum_change, change);
            let air = &self
                .zones
                .as_ref()
                .and_then(|rows| rows.get(index(number).ok()?))
                .ok_or(Error::Unavailable("actual component-report AirHB"))?
                .air;
            air.calculate_source_component_loads(number, self, controller, owners)?;
            if owners.space_controls.simulation {
                return Err(Error::Unavailable(
                    "positive Space component load report3856",
                ));
            }
        }
        Ok(maximum_change)
    }
}
