//! Reached RunSimulation213-241 prefix, before the sole weather owner moves.
//! No SetupSimulation, HB allocation or downstream numerical completion implied.
use super::{CtfInitializationReports, SourceInitializationError};
use crate::heat_balance::ctf_initialization_context::CtfInitializationError;
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::facility_electric_service::FacilityElectricService;
use crate::heat_balance::run_input_prefix::{SelectedCoilSelectionReport, SelectedPluginManager};
use crate::heat_balance::selected_simulation_startup::{
    check_environment_specifications, check_for_requested_reporting,
};
use crate::heat_balance::source_output_registration::{LiveTimeStepOwners, TimeStepKind};
use crate::weather::day::SetupSimulationControls;
use ep_model::ReportInputs;

fn failure(
    handler: &'static str,
    source: impl std::error::Error + 'static,
) -> SourceInitializationError {
    SourceInitializationError::Handler {
        handler,
        source: Box::new(source),
    }
}

impl CtfInitializationReports {
    /// Run actual213-241 on the same reached owners after both real init guards.
    /// Reentry is not silently skipped: the original duplicate time-pointer fatal
    /// occurs after its actual repeated checks and predefined-table registration.
    ///
    /// # Errors
    /// Preserves actual files, scratch, unused identities and partial owner writes.
    pub fn initialize_source_run_prefix(
        &mut self,
        report_inputs: &ReportInputs,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), SourceInitializationError> {
        if !self.source_initialization_guards.init_state_called {
            return Err(SourceInitializationError::Association(
                "RunSimulation213 before init_state",
            ));
        }
        let raw = self.source_initialization_input.as_ref().ok_or(
            SourceInitializationError::Association("actual startup input"),
        )?;
        let input = self
            .selected_input_processor
            .as_mut()
            .ok_or(SourceInitializationError::Association("sole IP"))?;
        let project = self
            .selected_project_data
            .as_ref()
            .ok_or(SourceInitializationError::Association("project"))?;
        let globals = self
            .selected_heat_balance_globals
            .as_mut()
            .ok_or(SourceInitializationError::Association("DataGlobal"))?;
        let startup = self
            .selected_simulation_startup
            .as_mut()
            .ok_or(SourceInitializationError::Association("startup"))?;
        let schedules = self
            .selected_schedule_manager
            .as_ref()
            .ok_or(SourceInitializationError::Association("Schedule"))?;
        let weather = self.source_initialization_weather.as_mut().ok_or(
            SourceInitializationError::Association("same pre-weather state"),
        )?;
        let controls = SetupSimulationControls::from_admitted_input_flags(
            globals.do_zone_sizing,
            globals.do_system_sizing,
            globals.do_plant_sizing,
            globals.do_hvac_sizing_simulation,
            weather.global.do_des_day_sim,
            weather.global.do_weath_sim,
        );
        check_environment_specifications(
            input,
            controls,
            project.run_control_in_input,
            startup.run_periods_in_input,
            globals,
            &raw.input_weather_file_path,
            backend,
        )
        .map_err(|e| failure("CheckEnvironment213", e))?;
        check_for_requested_reporting(input, controls, globals, backend)
            .map_err(|e| failure("CheckReporting214", e))?;
        self.registry
            .set_predefined_tables(globals.do_zone_sizing, globals.do_system_sizing)
            .map_err(|e| failure("SetPredefinedTables215", CtfInitializationError::Owner(e)))?;
        self.setup_simulation_registry_prepared = true;
        startup
            .construction_bounds
            .set_preconstruction_input_parameters(input)
            .map_err(|e| failure("SetPreConstruction216", e))?;
        self.output_registration
            .time_pointers
            .setup_time_pointer(TimeStepKind::Zone, backend)
            .map_err(|e| failure("SetupTimePointers218", e))?;
        self.output_registration
            .time_pointers
            .setup_time_pointer(TimeStepKind::System, backend)
            .map_err(|e| failure("SetupTimePointers220", e))?;
        self.facility_electric_service = Some(FacilityElectricService::new());
        self.coil_selection_report = Some(SelectedCoilSelectionReport::default());
        startup
            .is_input_object_used(
                input,
                &mut self.input_processor_shortcut,
                globals,
                report_inputs,
                &mut self.scan,
                false,
                false,
                0,
                backend,
            )
            .map_err(|e| failure("isInputObjectUsed225", e))?;
        self.selected_branches
            .manage_branches(input, backend)
            .map_err(|e| failure("ManageBranchInput227", e))?;
        self.selected_branches
            .manage_connectors(input, backend)
            .map_err(|e| failure("ManageConnectorInput228", e))?;
        self.selected_plugin_manager = Some(
            SelectedPluginManager::new(input, false).map_err(|e| failure("PluginManager231", e))?,
        );
        globals.doing_sizing = true;
        backend.state.doing_sizing = globals.doing_sizing;
        let override_timestep = globals.override_timestep;
        self.selected_sizing_manager
            .manage_sizing(
                input,
                &mut self.input_processor_shortcut,
                schedules,
                &raw.sizing,
                controls,
                globals,
                &mut weather.global,
                &mut weather.environment,
                override_timestep,
                self.heat_balance_space_controls.sizing,
                self.heat_balance_space_controls.simulation,
                backend,
            )
            .map_err(|e| failure("ManageSizing234", e))?;
        if weather.global.do_des_day_sim
            || weather.global.do_weath_sim
            || globals.do_hvac_sizing_simulation
        {
            weather.global.do_output_reporting = true;
        }
        globals.doing_sizing = false;
        backend.state.doing_sizing = globals.doing_sizing;
        self.heat_balance_space_controls.current = self.heat_balance_space_controls.simulation;
        Ok(())
    }

    /// Resolve an actual OutputProcessor time target against the caller's same
    /// retained weather global and HVAC owner, including after the weather move.
    ///
    /// # Errors
    /// Fails for a missing or mismatched binding; never returns a cached scalar.
    pub fn source_output_timestep<'a>(
        &'a self,
        kind: TimeStepKind,
        live_weather: &'a crate::weather::day::WeatherGlobalState,
    ) -> Result<&'a f64, SourceInitializationError> {
        let startup = self
            .selected_simulation_startup
            .as_ref()
            .ok_or(SourceInitializationError::Association("same HVAC owner"))?;
        self.output_registration
            .time_pointers
            .time_step(
                kind,
                &LiveTimeStepOwners {
                    global: live_weather,
                    hvac: &startup.hvac_globals,
                },
            )
            .map_err(|e| failure("OutputProcessor live time target", e))
    }
}
