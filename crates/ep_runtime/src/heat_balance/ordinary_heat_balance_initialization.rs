//! Genuine HeatBalanceManager InitHeatBalance2594–2822 and allocations.
//! This precedes InitSurfaceHeatBalance on the same moved Source owners.
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::ordinary_air_heat_balance::SourceAirHeatBalance;
use crate::heat_balance::ordinary_air_heat_balance::environment::SourceZoneFanHeatBalanceArrays;
use crate::heat_balance::ordinary_surface_initialization::OrdinarySurfaceInitialization;
use crate::heat_balance::ordinary_window_optical_initialization::SourceWindowOpticalOwners;
use crate::heat_balance::radiation::ordinary::OrdinarySolarRadiation;
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_heat_balance_input::shadowing::SourceShadowingInput;
use crate::heat_balance::selected_heat_balance_input::surface::SurfaceGeometryInputState;
use crate::heat_balance::selected_heat_balance_input::zone::SelectedZoneInputOwners;
use crate::heat_balance::selected_heat_balance_input::{
    HeatBalanceInputError as Error, SelectedHeatBalanceInputState,
};
use crate::heat_balance::warmup::report::WarmupReportState;
use crate::weather::day::{WeatherEnvironmentState, WeatherGlobalState, WeatherOwnerState};
mod allocation;
pub(crate) mod body_host;
pub mod ctf;
pub(crate) mod first_call;
pub(crate) mod manage;
mod owners;
pub(crate) mod provider_host;
mod providers;
pub(crate) mod report;
pub(crate) mod report_warmup;
mod state;
pub(crate) mod warmup;
mod zone_outdoor;
pub(crate) use owners::SourceInitHeatBalanceOwners;
pub(crate) use providers::SourceInitHeatBalanceProviders;
pub(crate) use state::{
    SourceContaminantArrays, SourceHeatBalanceManager, SourceWarmupConvergence,
};

impl SourceHeatBalanceManager {
    pub(crate) fn initialize(
        &mut self,
        owners: &mut SourceInitHeatBalanceOwners<'_, '_>,
        providers: &mut impl SourceInitHeatBalanceProviders,
    ) -> Result<(), Error> {
        if owners.clock.begin_sim_flag {
            self.allocate_heat_balance_arrays(owners, providers)?; //2619
            if owners.control.any_ctf || owners.control.any_empd {
                owners
                    .backend
                    .display_string("Initializing Response Factors")?;
                providers.initialize_conduction_transfer_functions(owners)?; //2622
            }
            // InitSurfacePropertyViewFactors9899/9902 are genuine early returns.
            if owners.globals.any_local_environments_in_model && owners.surface.first_time {
                providers.initialize_positive_surface_property_view_factors(owners)?;
            }
            owners
                .backend
                .display_string("Initializing Window Optical Properties")?;
            // Real DataWindowEquivLayer count, assigned by the same GetConstructData.
            if owners.control.total_equivalent_layer_constructions >= 1 {
                providers.initialize_positive_equivalent_layer_windows(owners)?;
            }
            owners
                .window
                .initialize_source_window_optical_calculations(&mut SourceWindowOpticalOwners {
                    arrays: &mut owners.surface.arrays,
                    geometry: owners.geometry,
                    control: owners.control,
                    materials: owners.materials,
                    bounds: owners.bounds,
                    input: owners.input,
                    shortcut: owners.shortcut,
                    report_inputs: owners.report_inputs,
                    scan: owners.scan,
                    backend: owners.backend,
                })?;
            owners.daylighting.initialize_daylighting_devices(
                owners.input,
                owners.shortcut,
                owners.gain_modules,
                owners.control,
                owners.backend,
            )?;
            owners
                .backend
                .display_string("Initializing Solar Calculations")?;
            providers.initialize_solar_calculations(owners)?;
        }
        if owners.clock.begin_envrn_flag {
            self.begin_environment()?;
            let count = extent(owners.geometry.total_surfaces)?;
            for index in 0..count {
                owners
                    .geometry
                    .surface_windows
                    .as_mut()
                    .and_then(|rows| rows.get_mut(index))
                    .ok_or(Error::Unavailable("HB2657 same SurfaceWindow allocation"))?
                    .theta_face
                    .fill(296.15);
                *owners
                    .geometry
                    .window_arrays
                    .as_mut()
                    .and_then(|rows| rows.surf_win_eff_ins_surf_temp.get_mut(index))
                    .ok_or(Error::Unavailable(
                        "HB2658 same SurfWinEffInsSurfTemp allocation",
                    ))? = 23.0;
            }
        }
        if owners.globals.any_energy_management_system_in_model {
            providers.initialize_ems_constructions(owners)?;
            providers.initialize_ems_surface_properties(owners)?;
        }
        if owners.geometry.total_storm_windows > 0 {
            providers.initialize_storm_windows(self, owners)?;
        }
        if owners.clock.begin_sim_flag
            && owners.clock.do_weath_sim
            && owners.shadowing.report_external_sunlit_fraction
        {
            providers.open_shading_file(owners)?;
        }
        if owners.clock.begin_day_flag {
            if !owners.clock.warmup_flag && owners.clock.day_of_sim == 1 {
                fill(&mut self.max_heat_load_zone, -9999.0)?;
                fill(&mut self.max_cool_load_zone, -9999.0)?;
                fill(&mut self.max_temp_zone, -9999.0)?;
                fill(&mut self.min_temp_zone, 1000.0)?;
            }
            if !owners.shadowing.detailed_solar_timestep_integration {
                providers.perform_solar_calculations(owners)?;
            }
        }
        if owners.shadowing.detailed_solar_timestep_integration {
            providers.perform_solar_calculations(owners)?;
        }
        if owners.clock.begin_day_flag
            && !owners.clock.warmup_flag
            && owners.clock.kind_of_sim == 3
            && owners.shadowing.report_external_sunlit_fraction
        {
            providers.report_external_sunlit_fractions(owners)?;
        }
        zone_outdoor::set_bulb_temperatures(owners.zones, owners.environment)?;
        zone_outdoor::check_bulb_temperatures(owners.zones, owners.backend)?;
        zone_outdoor::set_wind_speeds(owners.zones, owners.environment)?;
        zone_outdoor::set_wind_directions(owners.zones, owners.environment)?;
        if owners.globals.any_local_environments_in_model {
            providers.apply_zone_local_environments(owners)?;
        }
        if owners.globals.any_energy_management_system_in_model {
            providers.apply_zone_ems_overrides(owners)?;
        }
        if owners.clock.begin_sim_flag {
            let zones = owners
                .zones
                .zones
                .as_ref()
                .ok_or(Error::Unavailable("HB2806 same Zone array"))?;
            for zone in zones {
                for &space_number in &zone.space_indexes {
                    let space = owners
                        .zones
                        .spaces
                        .as_ref()
                        .and_then(|rows| rows.get(ordinal(space_number).ok()?))
                        .ok_or(Error::Unavailable("HB2808 actual Space ordinal"))?;
                    for surface_number in space.window_first..=space.window_last {
                        let index = ordinal(surface_number)?;
                        let model = owners
                            .geometry
                            .window_arrays
                            .as_mut()
                            .and_then(|rows| rows.surf_win_window_model_type.get_mut(index))
                            .ok_or(Error::Unavailable("HB2813 actual WindowModel allocation"))?;
                        // DataSurfaces WindowModel Detailed0/BSDF1/EQL2.
                        if *model != 1 && *model != 2 {
                            *model = 0;
                        }
                    }
                }
            }
        }
        // No shared InitSurface first_time write occurs in this caller.
        Ok(())
    }
}
fn extent(value: i32) -> Result<usize, Error> {
    usize::try_from(value).map_err(|_| Error::Unavailable("actual nonnegative HB extent"))
}
fn ordinal(value: i32) -> Result<usize, Error> {
    extent(value)?
        .checked_sub(1)
        .ok_or(Error::Unavailable("actual positive HB ordinal"))
}
fn fill(values: &mut Option<Vec<f64>>, value: f64) -> Result<(), Error> {
    values
        .as_mut()
        .ok_or(Error::Unavailable("actual allocated manager array"))?
        .fill(value);
    Ok(())
}

#[cfg(all(test, feature = "native-controlled-air-cohort"))]
impl SourceHeatBalanceManager {
    /// Test-only access to the same actual complete allocator, without staging its arrays.
    pub(crate) fn allocate_native_warmup_cohort_arrays(
        &mut self,
        owners: &mut SourceInitHeatBalanceOwners<'_, '_>,
        providers: &mut impl SourceInitHeatBalanceProviders,
    ) -> Result<(), Error> {
        self.allocate_heat_balance_arrays(owners, providers)
    }
}
