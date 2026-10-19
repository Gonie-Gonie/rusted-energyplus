//! Concrete Native189-238 children borrowing the same post-input run owners.
use super::SourceInitHeatBalanceOwners;
use super::manage::SourceManageHeatBalanceBodyChildren;
use super::provider_host::SourceOrdinaryInitHeatBalanceHost;
use super::report::SourceHeatBalanceReportHost;
use super::warmup::{SourceWarmupCheckOwners, SourceWarmupOutputOwners, SourceWarmupRecordOwners};
use crate::heat_balance::ordinary_air_heat_balance::predictor_corrector::SourceZoneAirPredictorCorrector;
use crate::heat_balance::ordinary_air_heat_balance::provider_host::SourceOrdinaryAirProvider;
use crate::heat_balance::ordinary_ems_plugin::SourcePluginData;
use crate::heat_balance::ordinary_hvac_manager::SourceHvacManager;
use crate::heat_balance::ordinary_hvac_manager::remaining::SourceOrdinaryHvacRemaining;
use crate::heat_balance::ordinary_surface_initialization::OrdinarySurfaceInitialization;
use crate::heat_balance::ordinary_surface_management::SourceSurfaceManagement;
use crate::heat_balance::ordinary_surface_management::provider_host::{
    SourceManageSurfaceOwners, SourceOrdinaryManageSurfaceHost,
};
use crate::heat_balance::ordinary_window_optical_initialization::SourceWindowOpticalInitialization;
use crate::heat_balance::radiation::ordinary::shading_report::SourceShadingStream;
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use crate::heat_balance::source_output_registration::{
    SourceOutputDictionaryHost, SourceOutputFileHost, SourceOutputReportStreams,
};
use crate::weather::day::WeatherGlobalState;
use ep_model::TypedModel;
use std::path::Path;

/// Constructed only after the actual first GetHBInput/computed geometry suffix.
/// Persistent numerical/input owners remain outside this short-lived view.
pub(crate) struct SourceHeatBalanceBodyHost<'a, 'owners, 'backend, H, D> {
    pub(crate) owners: &'a mut SourceManageSurfaceOwners<'owners, 'backend, H, D>,
    pub(crate) surface: &'a mut OrdinarySurfaceInitialization,
    pub(crate) controller: &'a mut SourceZoneAirPredictorCorrector,
    pub(crate) hvac: &'a mut SourceHvacManager,
    pub(crate) surface_management: &'a mut SourceSurfaceManagement,
    pub(crate) window: &'a mut SourceWindowOpticalInitialization,
    pub(crate) model: &'a TypedModel,
    pub(crate) shading_stream: &'a mut SourceShadingStream,
    pub(crate) shading_path: &'a Path,
}

impl<H: SourceOutputFileHost, D: SourceOutputDictionaryHost + SourceOutputReportStreams>
    SourceManageHeatBalanceBodyChildren for SourceHeatBalanceBodyHost<'_, '_, '_, H, D>
{
    fn globals(&self) -> &SelectedHeatBalanceGlobals {
        self.owners.globals
    }
    fn clock(&self) -> &WeatherGlobalState {
        self.owners.clock
    }
    fn clock_mut(&mut self) -> &mut WeatherGlobalState {
        self.owners.clock
    }

    fn initialize_heat_balance(&mut self) -> Result<(), Error> {
        if crate::heat_balance::source_surface_cadence::active()
            && !self.owners.clock.warmup_flag
            && self.owners.clock.kind_of_sim == 3
            && self.owners.clock.day_of_sim == 1
            && self.owners.clock.hour_of_day == 1
            && self.owners.clock.time_step == 1
            && self.owners.clock.begin_day_flag
            && !self.owners.globals.doing_sizing
            && !self.owners.backend.state.kickoff_simulation
        {
            crate::heat_balance::source_surface_cadence::record_handoff(
                "before_first_run_init",
                crate::heat_balance::source_surface_cadence::Frame::from_owners(
                    self.surface,
                    self.owners.clock,
                    self.owners.globals,
                    self.owners.environment,
                    self.owners.backend.state.kickoff_simulation,
                    self.surface.inside_surface_iterations,
                ),
                self.owners.clock.kind_of_sim,
                self.owners.globals,
                &self.surface.arrays,
                self.owners.history,
                self.owners.air,
            );
        }
        let o = &mut self.owners;
        let mut providers = SourceOrdinaryInitHeatBalanceHost {
            model: self.model,
            ctf: o.ctf,
            registry: o.predefined_registry,
            table: o.predefined_table,
            registration: o.registration,
            prescan: o.prescan,
            minimum_report_frequency: o.minimum_report_frequency,
            output_host: o.output_host,
            dictionary_host: o.dictionary_host,
            eio: o.eio,
            shading_stream: self.shading_stream,
            shading_path: self.shading_path,
            report_flags: o.report_flags,
        };
        o.heat_balance.initialize(
            &mut SourceInitHeatBalanceOwners {
                surface: self.surface,
                geometry: o.geometry,
                zones: o.zones,
                control: o.control,
                air: o.air,
                radiation: o.radiation,
                fan_zone: o.fan,
                gains: o.gains,
                gain_modules: o.gain_modules,
                daylighting: o.daylighting,
                window: self.window,
                globals: o.globals,
                clock: o.clock,
                environment: o.environment,
                weather_owner: o.weather,
                shadowing: o.shadowing,
                materials: o.materials,
                bounds: &o.startup.construction_bounds,
                input: o.input,
                shortcut: o.shortcut,
                report_inputs: o.report_inputs,
                scan: o.scan,
                backend: o.backend,
            },
            &mut providers,
        )
    }

    fn manage_surface_heat_balance(&mut self) -> Result<(), Error> {
        let cadence_scope = crate::heat_balance::source_surface_cadence::enter(
            crate::heat_balance::source_surface_cadence::Procedure::Manage,
            crate::heat_balance::source_surface_cadence::Frame::from_owners(
                self.surface,
                self.owners.clock,
                self.owners.globals,
                self.owners.environment,
                self.owners.backend.state.kickoff_simulation,
                self.surface.inside_surface_iterations,
            ),
        );
        // Keep the once-invoked observer closure: local Result propagation must not bypass the after-call exit observation.
        #[allow(clippy::redundant_closure_call)]
        let cadence_result = (|| {
            let mut remaining = SourceOrdinaryHvacRemaining;
            let mut air_child = SourceOrdinaryAirProvider {
                hvac: self.hvac,
                remaining: &mut remaining,
            };
            self.surface_management.manage_source_surface_heat_balance(
                &mut SourceOrdinaryManageSurfaceHost {
                    surface: self.surface,
                    controller: self.controller,
                    owners: self.owners,
                    air_child: &mut air_child,
                },
            )
        })();
        cadence_scope.finish(
            crate::heat_balance::source_surface_cadence::Frame::from_owners(
                self.surface,
                self.owners.clock,
                self.owners.globals,
                self.owners.environment,
                self.owners.backend.state.kickoff_simulation,
                self.surface.inside_surface_iterations,
            ),
            cadence_result.is_ok(),
        );
        cadence_result
    }

    fn record_heat_balance(&mut self) -> Result<(), Error> {
        let cadence_scope = crate::heat_balance::source_surface_cadence::enter(
            crate::heat_balance::source_surface_cadence::Procedure::SourceRecord,
            crate::heat_balance::source_surface_cadence::Frame::from_owners(
                self.surface,
                self.owners.clock,
                self.owners.globals,
                self.owners.environment,
                self.owners.backend.state.kickoff_simulation,
                self.surface.inside_surface_iterations,
            ),
        );
        // Keep the once-invoked observer closure: local Result propagation must not bypass the after-call exit observation.
        #[allow(clippy::redundant_closure_call)]
        let cadence_result = (|| {
            let o = &mut self.owners;
            o.heat_balance
                .record_source_heat_balance(&mut SourceWarmupRecordOwners {
                    air: o.air,
                    demand: &self.controller.demand,
                    geometry: o.geometry,
                    history: o.history,
                    radiation: o.radiation,
                    zones: o.zones,
                    control: o.control,
                    globals: o.globals,
                    clock: o.clock,
                    project: o.project,
                    eio: o.eio,
                })
        })();
        cadence_scope.finish(
            crate::heat_balance::source_surface_cadence::Frame::from_owners(
                self.surface,
                self.owners.clock,
                self.owners.globals,
                self.owners.environment,
                self.owners.backend.state.kickoff_simulation,
                self.surface.inside_surface_iterations,
            ),
            cadence_result.is_ok(),
        );
        cadence_result
    }

    fn report_heat_balance(&mut self) -> Result<(), Error> {
        SourceHeatBalanceReportHost {
            owners: self.owners,
            surface: self.surface,
            controller: self.controller,
            hvac: self.hvac,
        }
        .report_source_heat_balance()
    }

    fn update_ems_trend_variables(&mut self) -> Result<(), Error> {
        self.owners
            .startup
            .runtime_language
            .update_source_ems_trend_variables(self.owners.globals)
    }
    fn plugin_data(&mut self) -> &mut SourcePluginData {
        self.owners.plugin_data
    }

    fn check_warmup_convergence(&mut self) -> Result<(), Error> {
        let cadence_scope = crate::heat_balance::source_surface_cadence::enter(
            crate::heat_balance::source_surface_cadence::Procedure::SourceCheck,
            crate::heat_balance::source_surface_cadence::Frame::from_owners(
                self.surface,
                self.owners.clock,
                self.owners.globals,
                self.owners.environment,
                self.owners.backend.state.kickoff_simulation,
                self.surface.inside_surface_iterations,
            ),
        );
        // Keep the once-invoked observer closure: local Result propagation must not bypass the after-call exit observation.
        #[allow(clippy::redundant_closure_call)]
        let cadence_result = (|| {
            let o = &mut self.owners;
            o.heat_balance
                .check_source_warmup_convergence(&mut SourceWarmupCheckOwners {
                    zones: o.zones,
                    control: o.control,
                    project: o.project,
                    globals: o.globals,
                    clock: o.clock,
                    environment: o.environment,
                    backend: o.backend,
                })
        })();
        cadence_scope.finish(
            crate::heat_balance::source_surface_cadence::Frame::from_owners(
                self.surface,
                self.owners.clock,
                self.owners.globals,
                self.owners.environment,
                self.owners.backend.state.kickoff_simulation,
                self.surface.inside_surface_iterations,
            ),
            cadence_result.is_ok(),
        );
        cadence_result
    }

    fn report_warmup_convergence(&mut self) -> Result<(), Error> {
        let o = &mut self.owners;
        o.heat_balance
            .report_source_warmup_convergence(&mut SourceWarmupOutputOwners {
                zones: o.zones,
                globals: o.globals,
                clock: o.clock,
                environment: o.environment,
                eio: o.eio,
            })
    }
}
