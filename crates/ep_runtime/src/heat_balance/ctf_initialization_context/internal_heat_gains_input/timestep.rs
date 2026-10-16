//! Genuine ManageInternalHeatGains(false), borrowing the moved Source owners.
use super::{E, SelectedInternalHeatGains, index, unavailable};
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::radiation::ordinary::RadiantRadiationEnclosure;
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_heat_balance_input::{
    SelectedHeatBalanceInputState, zone::SelectedZoneInputOwners,
};
use crate::heat_balance::selected_schedule_manager::SelectedScheduleManager;
use crate::weather::day::WeatherGlobalState;

mod devices;
mod initialize;
mod report;

/// These eight real calls are unconditional at InternalHeatGains6935-6943.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SourceGainModule {
    WaterThermalTanks,
    PipeHeatTransfer,
    WaterUse,
    FuelCell,
    MicroChp,
    ElectricFacility,
    TubularDaylighting,
    Refrigeration,
}

/// Implemented by the caller with the same retained IP/module/facility owners.
/// There is deliberately no default callback or empty-input no-op adapter.
pub(crate) trait SourceGainModuleProvider {
    fn run_module(
        &mut self,
        module: SourceGainModule,
        gains: &mut SelectedInternalHeatGains,
        clock: &WeatherGlobalState,
        globals: &SelectedHeatBalanceGlobals,
        zones: &mut SelectedZoneInputOwners,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), E>;
}

/// Direct borrows of the actual Zone/Space AirHB latentGain members.
/// A physical heat-balance result, copied rate or numeric registry is not a sink.
pub(crate) trait SourceGainLatentOwners {
    fn space_latent_gain_mut(&mut self, space: i32) -> Result<&mut f64, E>;
    fn zone_latent_gain_mut(&mut self, zone: i32) -> Result<&mut f64, E>;
}

/// Direct access to the actual enclosure, Source Surface ordinal association,
/// SurfAbsThermalInt and SurfQdotRadIntGainsInPerArea moved into InitSurface.
pub(crate) trait SourceGainRadiationOwners {
    fn num_radiant_enclosures(&self) -> i32;
    fn radiant_enclosure_mut(&mut self, number: i32) -> Result<&mut RadiantRadiationEnclosure, E>;
    fn surface_radiant_enclosure(&self, surface: i32) -> Result<i32, E>;
    fn surface_thermal_absorptance(&self, surface: i32) -> Result<&f64, E>;
    fn surface_internal_radiant_gain_mut(&mut self, surface: i32) -> Result<&mut f64, E>;
}

/// Source unallocated differs from Rust unable to represent allocation state.
pub(crate) enum SourceZoneGenericGainAllocation<'a> {
    Unavailable,
    Unallocated,
    Allocated(&'a mut [f64]),
}

pub(crate) struct SourceGainStepOwners<'a, 'backend> {
    pub(crate) clock: &'a WeatherGlobalState,
    pub(crate) globals: &'a SelectedHeatBalanceGlobals,
    pub(crate) control: &'a SelectedHeatBalanceInputState,
    pub(crate) zones: &'a mut SelectedZoneInputOwners,
    pub(crate) schedules: &'a SelectedScheduleManager,
    pub(crate) backend: &'a mut SourceErrorBackend<'backend>,
    pub(crate) modules: &'a mut dyn SourceGainModuleProvider,
    pub(crate) latent: Option<&'a mut dyn SourceGainLatentOwners>,
    pub(crate) radiation: Option<&'a mut dyn SourceGainRadiationOwners>,
    pub(crate) zone_generic_gain: SourceZoneGenericGainAllocation<'a>,
}

impl SelectedInternalHeatGains {
    pub(crate) fn manage_timestep(
        &mut self,
        owners: &mut SourceGainStepOwners<'_, '_>,
    ) -> Result<(), E> {
        if self.get_input_flag {
            // Source206 must call the actual input getter; this API cannot
            // replace its IP/EIO/report owners or mark that call complete.
            return Err(unavailable(
                "same genuine InternalHeatGains input getter before timestep",
            ));
        }
        self.initialize_timestep(owners)?;
        self.report_timestep(owners)?;
        let zones = owners
            .zones
            .zones
            .as_deref()
            .ok_or_else(|| unavailable("same actual CheckReturnAirHeatGain Zone owner"))?;
        for zone in zones {
            if zone.has_adjusted_return_temperature_by_ite && zone.has_lights_return_air_gain {
                owners.backend.show_fatal_error(
                    "Return air heat gains from lights are not allowed when Air Flow Calculation Method = FlowControlWithApproachTemperatures in zones with ITE objects."
                ).map_err(E::Fatal)?;
            }
            if zone.has_adjusted_return_temperature_by_ite && zone.has_airflow_window_return {
                owners.backend.show_fatal_error(
                    "Return air heat gains from windows are not allowed when Air Flow Calculation Method = FlowControlWithApproachTemperatures in zones with ITE objects."
                ).map_err(E::Fatal)?;
            }
        }
        if owners.globals.zone_sizing_calculation {
            return Err(unavailable("positive Source GatherComponentLoadsIntGain"));
        }
        Ok(())
    }

    /// DaylightingManager5639-5642, on this same retained allocation.
    pub(crate) fn reset_source_space_power_reduction_factors(
        &mut self,
        num_spaces: i32,
    ) -> Result<(), E> {
        for space in 1..=num_spaces {
            *self
                .allocations
                .space_power_reduction_factor
                .as_mut()
                .and_then(|rows| rows.get_mut(index(space).ok()?))
                .ok_or_else(|| unavailable("same actual spacePowerReductionFactor owner"))? = 1.0;
        }
        Ok(())
    }

    fn positive_count(&self, family: usize) -> Result<usize, E> {
        usize::try_from(self.totals[family])
            .map_err(|_| unavailable("actual nonnegative Source gain count"))
    }
    fn require_empty_family(&self, family: usize) -> Result<(), E> {
        if self.positive_count(family)? != 0 {
            return Err(unavailable("positive unported Source timestep gain family"));
        }
        Ok(())
    }
}
