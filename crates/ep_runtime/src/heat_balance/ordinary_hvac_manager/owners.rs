//! Temporary disjoint borrows; every producer and consumer retains its actual owner.
use super::*;
use crate::heat_balance::ctf_initialization_context::internal_heat_gains_input::{
    SelectedInternalHeatGains, SourceGainStepOwners, SourceZoneGenericGainAllocation,
};
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::facility_electric_service::FacilityElectricService;
use crate::heat_balance::ordinary_air_heat_balance::{
    SourceAirHeatBalance,
    environment::SourceZoneFanHeatBalanceArrays,
    predictor_corrector::{SourceZoneAirOwners, SourceZoneAirPredictorCorrector},
};
use crate::heat_balance::ordinary_equipment::OrdinaryEquipment;
use crate::heat_balance::ordinary_heat_balance_initialization::SourceContaminantArrays;
use crate::heat_balance::ordinary_surface_initialization::SourceSurfaceHeatBalanceArrays;
use crate::heat_balance::ordinary_tabular_reports::SourceTabularReports;
use crate::heat_balance::selected_gain_modules::{SourceGainModules, SourceModuleGainProvider};
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_heat_balance_input::{
    SelectedHeatBalanceInputState, surface::SurfaceGeometryInputState,
    zone::SelectedZoneInputOwners,
};
use crate::heat_balance::selected_heat_balance_space_controls::SelectedHeatBalanceSpaceControls;
use crate::heat_balance::selected_input_processor::{
    SelectedInputProcessor, SourceInputProcessorShortCut,
};
use crate::heat_balance::selected_project_data::SelectedProjectData;
use crate::heat_balance::selected_schedule_manager::SelectedScheduleManager;
use crate::heat_balance::selected_simulation_startup::SelectedHvacGlobals;
use crate::psychrometrics::SourcePsychrometricAccess;
use crate::weather::day::{WeatherEnvironmentState, WeatherGlobalState};
pub(crate) struct SourceHvacOwners<'a, 'backend> {
    pub(crate) runtime_language: &'a crate::heat_balance::selected_simulation_startup::SelectedRuntimeLanguage,
    pub(crate) air: &'a mut SourceAirHeatBalance,
    pub(crate) controller: &'a mut SourceZoneAirPredictorCorrector,
    pub(crate) zones: &'a mut SelectedZoneInputOwners,
    pub(crate) sizing: &'a mut crate::heat_balance::selected_sizing::SelectedSizingManager,
    pub(crate) equipment: &'a mut OrdinaryEquipment,
    pub(crate) geometry: &'a SurfaceGeometryInputState,
    pub(crate) surface_arrays: &'a mut SourceSurfaceHeatBalanceArrays,
    pub(crate) fan: &'a mut SourceZoneFanHeatBalanceArrays,
    pub(crate) gains: &'a mut SelectedInternalHeatGains,
    pub(crate) control: &'a mut SelectedHeatBalanceInputState,
    pub(crate) space_controls: &'a SelectedHeatBalanceSpaceControls,
    pub(crate) globals: &'a mut SelectedHeatBalanceGlobals,
    pub(crate) clock: &'a mut WeatherGlobalState,
    pub(crate) environment: &'a mut WeatherEnvironmentState,
    pub(crate) hvac: &'a mut SelectedHvacGlobals,
    pub(crate) input: &'a mut SelectedInputProcessor,
    pub(crate) shortcut: &'a mut SourceInputProcessorShortCut,
    pub(crate) source_is_epjson: &'a bool,
    pub(crate) faults: &'a crate::heat_balance::source_initialization::SourceFaultsManager,
    pub(crate) psychrometrics: &'a mut SourcePsychrometricAccess,
    pub(crate) backend: &'a mut SourceErrorBackend<'backend>,
    /// Same actual DataPlant.PlantLoop allocated empty by SimulationManager; no second array.
    pub(crate) plant_loops: &'a mut Option<Vec<()>>,
    pub(crate) project: &'a mut SelectedProjectData,
    pub(crate) contaminants: &'a mut SourceContaminantArrays,
    pub(crate) gain_modules: &'a mut SourceGainModules,
    pub(crate) facility: Option<&'a mut FacilityElectricService>,
    pub(crate) schedules: &'a mut SelectedScheduleManager,
    pub(crate) tabular: &'a mut SourceTabularReports,
    /// Actual report-side owners are reborrowed during this call, never held again by R.
    pub(crate) radiation: &'a mut crate::heat_balance::radiation::ordinary::OrdinarySolarRadiation,
    pub(crate) history: &'a mut crate::heat_balance::ctf_thermal_history::CtfThermalHistoryState,
    pub(crate) physical_surfaces: &'a mut [crate::heat_balance::convection::surface_environment::source_rows::SourceSurfaceOutdoorState],
    pub(crate) comfort: &'a mut crate::heat_balance::ordinary_thermal_comfort::SourceThermalComfort,
    pub(crate) comfort_summary:
        &'a mut crate::heat_balance::ordinary_thermal_comfort::SourceComfortPredefinedSummary,
    pub(crate) predefined_registry:
        &'a mut crate::heat_balance::predefined_tables::PredefinedRegistry,
    pub(crate) predefined_table: &'a mut crate::heat_balance::ctf_reports::PredefinedStringTable,
    pub(crate) weather: &'a crate::weather::day::WeatherOwnerState,
    pub(crate) inside_surface_iterations: &'a i32,
    pub(crate) surface_heat_emission: &'a f64,
    pub(crate) surface_averages:
        &'a crate::heat_balance::ordinary_surface_source_averages::SourceFinalSurfaceHeatBalance,
    pub(crate) exterior:
        &'a crate::heat_balance::ordinary_exterior_energy_use::SourceExteriorEnergyUse,
}
impl SourceHvacOwners<'_, '_> {
    pub(super) fn zone_air_update(
        &mut self,
        availability: &crate::heat_balance::ordinary_equipment::management::availability::SourceAvailability,
        update: SourceAirUpdate,
        temperature_change: &mut f64,
        prior_time_step: f64,
        outputs: &mut impl SourceHvacChildren,
        eio: &mut dyn std::io::Write,
    ) -> Result<(), Error> {
        let shorten = self.hvac.shorten_time_step_sys;
        let use_zone_history = self.hvac.use_zone_time_step_history;
        let mut owners = SourceZoneAirOwners {
            zones: self.zones,
            equipment: &mut *self.equipment,
            geometry: self.geometry,
            surface_arrays: self.surface_arrays,
            fan: self.fan,
            gains: self.gains,
            control: self.control,
            space_controls: self.space_controls,
            globals: self.globals,
            clock: self.clock,
            environment: self.environment,
            hvac: self.hvac,
            input: self.input,
            shortcut: self.shortcut,
            source_is_epjson: self.source_is_epjson,
            faults: self.faults,
            availability,
            schedules: self.schedules,
            psychrometrics: self.psychrometrics,
            backend: self.backend,
        };
        self.controller.manage_source_zone_air_updates(
            update,
            temperature_change,
            shorten,
            use_zone_history,
            prior_time_step,
            self.air,
            &mut owners,
            outputs,
            eio,
        )
    }
    pub(super) fn update_source_internal_gains(&mut self) -> Result<(), Error> {
        let mut modules = SourceModuleGainProvider {
            modules: self.gain_modules,
            input: self.input,
            facility: self.facility.as_deref_mut(),
        };
        let mut owners = SourceGainStepOwners {
            clock: self.clock,
            globals: self.globals,
            control: self.control,
            zones: self.zones,
            schedules: self.schedules,
            backend: self.backend,
            modules: &mut modules,
            latent: Some(self.air),
            room_air: Some(&self.controller.subsystems.room_air),
            // The actual SuppressRadiationUpdate=true body never reads a radiation sink.
            radiation: None,
            // Actual generic-contaminant false avoids this branch; no fake allocated array.
            zone_generic_gain: SourceZoneGenericGainAllocation::Unavailable,
        };
        self.gains
            .update_source_internal_gain_values(&mut owners, true, true)
    }
    pub(super) fn accumulate_source_system_step_averages(&mut self) -> Result<(), Error> {
        for number in 1..=self.globals.num_of_zones {
            let at = index(number)?;
            let zone_air = &mut self
                .air
                .zones
                .as_mut()
                .and_then(|v| v.get_mut(at))
                .ok_or(Error::Unavailable("actual HVAC averaged Zone AirHB"))?
                .air;
            zone_air.ztav += zone_air.zt * self.hvac.fraction_time_step_zone;
            zone_air.air_hum_rat_avg += zone_air.air_hum_rat * self.hvac.fraction_time_step_zone;
            let zone = self
                .zones
                .zones
                .as_ref()
                .and_then(|v| v.get(at))
                .ok_or(Error::Unavailable("actual HVAC average Zone spaceIndexes"))?;
            // Native spaces always accumulate, independent of doSpaceHeatBalance.
            for &number in &zone.space_indexes {
                let space = &mut self
                    .air
                    .spaces
                    .as_mut()
                    .and_then(|v| v.get_mut(index(number).ok()?))
                    .ok_or(Error::Unavailable("actual HVAC average Space AirHB"))?
                    .air;
                space.ztav += space.zt * self.hvac.fraction_time_step_zone;
                space.air_hum_rat_avg += space.air_hum_rat * self.hvac.fraction_time_step_zone;
            }
            if self.control.carbon_dioxide_simulation {
                let current = *self
                    .contaminants
                    .zone_air_co2
                    .as_ref()
                    .and_then(|v| v.get(at))
                    .ok_or(Error::Unavailable("actual HVAC ZoneAirCO2"))?;
                *self
                    .contaminants
                    .zone_air_co2_avg
                    .as_mut()
                    .and_then(|v| v.get_mut(at))
                    .ok_or(Error::Unavailable("actual HVAC ZoneAirCO2Avg"))? +=
                    current * self.hvac.fraction_time_step_zone;
            }
            if self.control.generic_contaminant_simulation {
                let current = *self
                    .contaminants
                    .zone_air_gc
                    .as_ref()
                    .and_then(|v| v.get(at))
                    .ok_or(Error::Unavailable("actual HVAC ZoneAirGC"))?;
                *self
                    .contaminants
                    .zone_air_gc_avg
                    .as_mut()
                    .and_then(|v| v.get_mut(at))
                    .ok_or(Error::Unavailable("actual HVAC ZoneAirGCAvg"))? +=
                    current * self.hvac.fraction_time_step_zone;
            }
            if self.controller.flags.number_on_off_zones > 0 {
                let setpoint = self
                    .fan
                    .thermostat
                    .zone_tstat_setpoints
                    .as_mut()
                    .and_then(|v| v.get_mut(at))
                    .ok_or(Error::Unavailable(
                        "actual HVAC OnOff averaged Zone setpoint",
                    ))?;
                setpoint.setpt_hi_average += setpoint.setpt_hi * self.hvac.fraction_time_step_zone;
                setpoint.setpt_lo_average += setpoint.setpt_lo * self.hvac.fraction_time_step_zone;
            }
        }
        Ok(())
    }
}
