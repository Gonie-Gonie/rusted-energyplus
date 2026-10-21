//! Actual InitSurfaceHeatBalance272-620 sequencing on the sole moved owners.
use super::{
    OrdinarySurfaceInitialization, SourceSurfaceHeatBalanceArrays, output, prefix, registration,
};
use crate::heat_balance::ctf_initialization_context::internal_heat_gains_input::{
    SourceGainStepOwners, SourceZoneGenericGainAllocation,
};
use crate::heat_balance::ordinary_surface_predefined::{
    SourceSurfacePredefinedError, gather_source_surface_predefined_report,
};
use crate::heat_balance::radiation::ordinary::{
    RadiantExchangeSource, RadiantInitializationOutput,
};
use crate::heat_balance::selected_gain_modules::SourceModuleGainProvider;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use crate::heat_balance::source_output_registration::{
    SourceOutputDictionaryHost, SourceOutputFileHost, SourceReportVariableInput,
};
mod gains;
mod owners;
mod providers;
pub(crate) use owners::SourceInitSurfaceOwners;
pub(crate) use providers::SourceInitSurfaceProviders;

/// Preserve the actual child failure; no unsupported callback becomes success.
#[derive(Debug)]
pub enum SourceInitSurfaceError {
    /// Actual heat-balance child failure.
    HeatBalance(Error),
    /// Actual surface predefined-report failure.
    Predefined(SourceSurfacePredefinedError),
}
impl From<Error> for SourceInitSurfaceError {
    fn from(error: Error) -> Self {
        Self::HeatBalance(error)
    }
}
impl From<SourceSurfacePredefinedError> for SourceInitSurfaceError {
    fn from(error: SourceSurfacePredefinedError) -> Self {
        Self::Predefined(error)
    }
}
impl std::fmt::Display for SourceInitSurfaceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HeatBalance(error) => std::fmt::Display::fmt(error, f),
            Self::Predefined(error) => std::fmt::Display::fmt(error, f),
        }
    }
}
impl std::error::Error for SourceInitSurfaceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::HeatBalance(error) => Some(error),
            Self::Predefined(error) => Some(error),
        }
    }
}

impl OrdinarySurfaceInitialization {
    /// The caller must first perform real InitHeatBalance solar producers and
    /// allocate real Zone/Space AirHB/enclosure arrays at their earlier positions.
    /// This method never calls compatibility surface/outside/zone solve adapters.
    pub(crate) fn initialize_source_surface_heat_balance<
        H: SourceOutputFileHost,
        D: SourceOutputDictionaryHost,
        P: SourceInitSurfaceProviders<H, D>,
    >(
        &mut self,
        owners: &mut SourceInitSurfaceOwners<'_, '_, H, D>,
        zone_generic_gain: SourceZoneGenericGainAllocation<'_>,
        providers: &mut P,
    ) -> Result<(), SourceInitSurfaceError> {
        if self.first_time {
            owners
                .backend
                .display_string("Initializing Outdoor environment for Surfaces")
                .map_err(Error::Backend)?;
        }
        // Four distinct real Source operations311-315. Zone outdoor setup is
        // an earlier independent caller and is not repeated here.
        providers.set_surface_outbulb_temperature(owners)?;
        providers.check_surface_outbulb_temperature(owners)?;
        providers.set_surface_wind_speed(owners)?;
        providers.set_surface_wind_direction(owners)?;
        if owners.globals.any_local_environments_in_model {
            providers.apply_linked_outdoor_nodes(owners)?;
        }
        if owners.globals.any_energy_management_system_in_model {
            providers.apply_outdoor_ems_overrides(owners)?;
        }

        if owners.clock.begin_sim_flag {
            let count = usize::try_from(owners.geometry.total_surfaces)
                .map_err(|_| Error::Unavailable("actual InitSurface TotSurfaces"))?;
            self.arrays.allocate(
                count,
                owners.constructions.any_internal_heat_source_in_input,
                owners
                    .public_ctf
                    .aggregate_after_returned_calls
                    .simple_ctf_only,
                owners.globals.any_energy_management_system_in_model,
                owners.history,
            )?;
            // AllocateSurfaceHeatBalArrays registers outputs at1607-2205 before
            // returning; do not move these calls past the subsequent any_of.
            {
                let live = output::SourceSurfaceOutputOwners {
                    arrays: &self.arrays,
                    sum_surface_heat_emission: &self.sum_surface_heat_emission,
                    geometry: owners.geometry,
                    history: owners.history,
                    physical_surfaces: owners.physical_surfaces,
                };
                let mut input = SourceReportVariableInput::new(
                    owners.report_inputs,
                    owners.input,
                    owners.minimum_report_frequency,
                );
                registration::register_source_surface_outputs(
                    &live,
                    owners.constructions,
                    owners.globals.display_advanced_report_variables,
                    owners.registration,
                    &mut input,
                    owners.prescan,
                    owners.clock.minutes_in_time_step,
                    owners.output_host,
                    owners.eio,
                    owners.dictionary_host,
                )
                .map_err(Error::Output)?;
            }
            self.interzone_window = match owners.radiation.enclosure_storage() {
                Some(rows) => rows.iter().any(|row| row.has_interzone_window),
                None if owners.radiation.num_solar_enclosures() == 0 => false,
                None => {
                    return Err(Error::Unavailable("same actual EnclSolInfo any_of owner").into());
                }
            };
        }
        if owners.clock.begin_sim_flag || owners.globals.any_surface_property_overrides_in_model {
            self.arrays.copy_source_construction_absorptance(
                owners.geometry,
                owners.zones,
                owners.constructions,
                owners.globals,
            )?;
        }
        prefix::update_source_variable_absorptances(owners.geometry)?;
        if owners.clock.begin_envrn_flag {
            if self.first_time {
                owners
                    .backend
                    .display_string("Initializing Temperature and Flux Histories")
                    .map_err(Error::Backend)?;
            }
            providers.initialize_thermal_and_flux_histories(self, owners)?;
        }
        if owners.geometry.any_movable_insulation {
            providers.evaluate_outside_movable_insulation(owners)?;
            providers.evaluate_inside_movable_insulation(owners)?;
        }
        prefix::get_source_ground_surface_reflectance_average(owners.globals)?;
        prefix::timestep_source_complex_fenestration(owners.geometry)?;
        if owners.environment.current_weather.sun_is_up
            && owners.environment.current_weather.dif_solar_rad > 0.0
        {
            owners.radiation.source_anisotropic_sky_view_factors(
                owners.geometry,
                owners.constructions,
                owners.clock,
                owners.environment,
                owners.shadowing.detailed_sky_diffuse_algorithm,
                owners.geometry.shading_transmittance_varies,
                owners.backend,
            )?;
        } else {
            owners
                .radiation
                .solar_timestep
                .shading
                .surf_aniso_sky_mult
                .fill(0.0)?;
        }
        if self.first_time {
            owners
                .backend
                .display_string("Initializing Window Shading")
                .map_err(Error::Backend)?;
        }
        prefix::window_source_shading_manager(owners.zones, owners.globals)?;
        prefix::check_glazing_shading_status_change(
            owners.geometry,
            owners.zones,
            owners.constructions,
            owners.globals,
            owners.clock,
            owners.radiation,
        )?;
        if self.first_time {
            owners
                .backend
                .display_string("Computing Interior Absorption Factors")
                .map_err(Error::Backend)?;
        }
        if self.first_time {
            let source = RadiantExchangeSource {
                geometry: owners.geometry,
                zones: owners.zones,
                constructions: owners.constructions,
                carroll_method: owners.project.carroll_method,
                any_inside_shelf: owners.globals.any_inside_shelf,
            };
            let mut output = RadiantInitializationOutput {
                input: owners.input,
                shortcut: owners.shortcut,
                scan: owners.scan,
                report_inputs: owners.report_inputs,
                backend: owners.backend,
                eio: owners.eio,
                debug: Some(&mut *owners.debug),
                advanced_report_variables: owners.globals.display_advanced_report_variables,
            };
            owners
                .radiation
                .initialize_source_radiant(&source, &mut output)
                .map_err(Error::Output)?;
        }
        {
            let source = RadiantExchangeSource {
                geometry: owners.geometry,
                zones: owners.zones,
                constructions: owners.constructions,
                carroll_method: owners.project.carroll_method,
                any_inside_shelf: owners.globals.any_inside_shelf,
            };
            owners
                .radiation
                .compute_source_thermal_absorption(
                    &source,
                    self.arrays
                        .surf_abs_thermal_int
                        .as_deref_mut()
                        .ok_or(Error::Unavailable("actual InitSurface thermal absorptance"))?,
                    owners.geometry.any_movable_slat,
                )
                .map_err(Error::Output)?;
        }
        if self.first_time {
            owners
                .backend
                .display_string("Computing Interior Diffuse Solar Absorption Factors")
                .map_err(Error::Backend)?;
        }
        {
            let source = RadiantExchangeSource {
                geometry: owners.geometry,
                zones: owners.zones,
                constructions: owners.constructions,
                carroll_method: owners.project.carroll_method,
                any_inside_shelf: owners.globals.any_inside_shelf,
            };
            owners
                .radiation
                .compute_source_shortwave_absorption(
                    &source,
                    self.arrays
                        .surf_abs_solar_int
                        .as_deref()
                        .ok_or(Error::Unavailable("actual InitSurface solar absorptance"))?,
                    owners.backend,
                )
                .map_err(Error::Output)?;
        }
        if self.interzone_window {
            if self.first_time {
                owners
                    .backend
                    .display_string(
                        "Computing Interior Diffuse Solar Exchange through Interzone Windows",
                    )
                    .map_err(Error::Backend)?;
            }
            providers.compute_diffuse_solar_interzone_exchange(self, owners)?;
        }
        providers.initialize_daylighting(self.first_time, owners)?;
        self.exchange_source_main(
            owners.radiation,
            owners.geometry,
            owners.constructions,
            owners.history,
            owners.globals,
            owners.clock,
            owners.project.carroll_method,
            owners.backend,
        )?;
        if owners.geometry.airflow_windows {
            providers.control_window_gap_airflow(owners)?;
        }
        if self.first_time {
            owners
                .backend
                .display_string("Initializing Solar Heat Gains")
                .map_err(Error::Backend)?;
        }
        owners.radiation.source_init_solar_heat_gains(
            owners.geometry,
            owners.zones,
            owners.constructions,
            owners.clock,
            owners.environment,
            &mut self.arrays,
            owners.gain_modules,
            self.first_time,
            self.interzone_window,
        )?;
        providers.manage_daylighting(owners)?;
        if self.first_time {
            owners
                .backend
                .display_string("Initializing Internal Heat Gains")
                .map_err(Error::Backend)?;
        }
        {
            // Concrete genuine eight-call provider062c; no manual getters are
            // appended here and no empty/default module callback is accepted.
            let mut module_provider = SourceModuleGainProvider {
                modules: owners.gain_modules,
                input: owners.input,
                facility: owners.facility.as_deref_mut(),
            };
            let mut radiant = gains::SourceSurfaceGainRadiation {
                radiation: owners.radiation,
                geometry: owners.geometry,
                arrays: &mut self.arrays,
            };
            let mut step = SourceGainStepOwners {
                clock: owners.clock,
                globals: owners.globals,
                control: owners.constructions,
                zones: owners.zones,
                schedules: owners.schedules,
                backend: owners.backend,
                modules: &mut module_provider,
                latent: Some(owners.air),
                room_air: Some(owners.room_air),
                radiation: Some(&mut radiant),
                zone_generic_gain,
            };
            owners.gains.manage_timestep(&mut step)?;
        }
        if self.first_time {
            owners
                .backend
                .display_string("Initializing Interior Solar Distribution")
                .map_err(Error::Backend)?;
        }
        owners.radiation.source_init_interior_solar_distribution(
            owners.geometry,
            owners.zones,
            owners.constructions,
            owners.clock,
            owners.environment,
            &mut self.arrays,
            owners.gains,
            owners.gain_modules,
            self.interzone_window,
        )?;
        if self.first_time {
            owners
                .backend
                .display_string("Initializing Interior Convection Coefficients")
                .map_err(Error::Backend)?;
        }
        providers.initialize_inside_convection_temporary(self, owners)?;
        if owners.clock.begin_sim_flag {
            if self.first_time {
                owners
                    .backend
                    .display_string("Gathering Information for Predefined Reporting")
                    .map_err(Error::Backend)?;
            }
            gather_source_surface_predefined_report(
                owners.geometry,
                owners.constructions,
                owners.zones,
                owners.materials,
                owners.scan,
                owners.predefined_registry,
                owners.predefined_table,
                owners.input,
            )?;
        }
        if owners.constructions.any_cond_fd {
            providers.initialize_finite_difference(self, owners)?;
        }
        self.arrays.compute_source_ctf_constants(
            owners.geometry,
            owners.zones,
            owners.constructions,
            owners.public_ctf,
            owners.history,
            owners.globals,
        )?;
        if owners.constructions.any_internal_heat_source_in_input {
            providers.initialize_internal_source_ctf_constants(self, owners)?;
        }
        self.arrays
            .reset_source_radiant_system_terms(owners.zones, owners.globals)?;
        self.arrays
            .reset_source_hvac_radiant_flux(&owners.geometry.surface_lists.all_gets_radiant_heat)?;
        if owners.globals.zone_sizing_calculation {
            providers.gather_component_surface_absorption_factors(self, owners)?;
        }
        if self.first_time {
            owners
                .backend
                .display_string("Completed Initializing Surface Heat Balance")
                .map_err(Error::Backend)?;
        }
        // The sole shared flag write620 is reached only after every real child
        // call/guard/output above succeeded. All error returns preserve it.
        self.first_time = false;
        Ok(())
    }
}
