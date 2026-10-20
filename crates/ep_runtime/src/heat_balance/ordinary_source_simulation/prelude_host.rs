//! Actual pre-primary adapters over the same completed kickoff session.
//! Explicit required-child boundaries remain until their genuine bodies are ported.
use super::prelude::{SourcePrimaryPreludeChildren, prepare_source_primary_simulation};
use crate::RuntimeError;
use crate::heat_balance::ctf_initialization_context::{CtfInitializationReports, hb_input_error};
use crate::heat_balance::error_backend::{SourceDiagnosticFlags, SourceErrorBackend};
use crate::heat_balance::ordinary_ems_plugin::SourcePluginData;
use crate::heat_balance::ordinary_pollution::SourcePollutionOutputContext;
use crate::heat_balance::ordinary_source_session::OrdinarySourceSession;
use crate::heat_balance::ordinary_system_reports::{
    SourceSystemReportOwners, SourceSystemReportsOutputContext,
};
use crate::heat_balance::run_input_prefix::SelectedPluginManager;
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use crate::heat_balance::source_initialization::SourceInitializationFiles;
use crate::heat_balance::source_output_registration::{
    SourceMeterReportingOwners, SourceMtdFileHost, SourceNodeReportingOutputContext,
};
use crate::weather::day::{ProductionWeatherTimestepSeries, WeatherDayState, WeatherSession};
use ep_model::TypedModel;
use std::convert::Infallible;

struct PreludeHost<'a, 'backend> {
    files: &'a mut SourceInitializationFiles,
    session: &'a mut Option<OrdinarySourceSession>,
    reports: &'a mut CtfInitializationReports,
    weather: &'a mut WeatherSession,
    model: &'a TypedModel,
    output_host: &'a mut SourceMtdFileHost,
    backend: &'a mut SourceErrorBackend<'backend>,
}
fn unavailable(name: &'static str) -> RuntimeError {
    hb_input_error(Error::Unavailable(name))
}
fn required<T>(value: Option<T>, name: &'static str) -> Result<T, RuntimeError> {
    value.ok_or_else(|| unavailable(name))
}
impl SourcePrimaryPreludeChildren for PreludeHost<'_, '_> {
    fn state(&self) -> &WeatherDayState {
        &self.weather.state
    }
    fn globals(&self) -> Result<&SelectedHeatBalanceGlobals, RuntimeError> {
        required(
            self.reports.selected_heat_balance_globals.as_ref(),
            "actual DataGlobal before primary startup",
        )
    }
    fn globals_mut(&mut self) -> Result<&mut SelectedHeatBalanceGlobals, RuntimeError> {
        required(
            self.reports.selected_heat_balance_globals.as_mut(),
            "actual DataGlobal before primary startup",
        )
    }
    fn plugin_manager(&self) -> &Option<SelectedPluginManager> {
        &self.reports.selected_plugin_manager
    }
    fn plugin_data_mut(&mut self) -> &mut SourcePluginData {
        &mut self.reports.source_plugin_data
    }
    fn display_string(&mut self, message: &str) -> Result<(), RuntimeError> {
        let globals = required(
            self.reports.selected_heat_balance_globals.as_ref(),
            "actual globals before live DisplayString",
        )?;
        let project = required(
            self.reports.selected_project_data.as_ref(),
            "actual SysVars before live DisplayString",
        )?;
        self.backend
            .display_source_string(
                message,
                &SourceDiagnosticFlags {
                    kickoff: &globals.kickoff_simulation,
                    developer: &project.system.developer_flag,
                    warmup: &self.weather.state.global.warmup_flag,
                    doing_sizing: &globals.doing_sizing,
                },
            )
            .map_err(hb_input_error)
    }
    fn show_message(&mut self, message: &str) -> Result<(), RuntimeError> {
        self.backend.show_message(message).map_err(Into::into)
    }
    fn report_surfaces(&mut self) -> Result<(), RuntimeError> {
        let session = required(
            self.session.as_ref(),
            "actual same daylighting module before ReportSurfaces",
        )?;
        self.reports.report_source_surfaces(
            &self.model.report_inputs,
            &mut *self.files,
            &session.daylighting,
            self.backend,
        )
    }
    fn setup_node_variables_for_reporting(&mut self) -> Result<(), RuntimeError> {
        self.backend.with_scoped(
            &mut (
                &mut *self.reports,
                &mut *self.files,
                &mut *self.output_host,
                &mut *self.weather,
            ),
            |mut backend, (reports, files, output_host, weather)| {
                let globals = required(
                    reports.selected_heat_balance_globals.as_ref(),
                    "actual globals before Node reporting setup",
                )?;
                let project = required(
                    reports.selected_project_data.as_ref(),
                    "actual Project before Node reporting setup",
                )?;
                let input = required(
                    reports.selected_input_processor.as_mut(),
                    "actual same IP before Node reporting setup",
                )?;
                let (mut dictionary, eio, bnd) = files.source_node_reporting_streams(
                    &reports.source_sqlite,
                    &mut reports.source_results_output_variables,
                );
                required(
                    reports.source_zone_equipment.as_mut(),
                    "actual sole NodeOwner before reporting setup",
                )?
                .setup_source_node_variables_for_reporting(
                    globals,
                    &reports.heat_balance_input_state,
                    &mut backend,
                    &mut SourceNodeReportingOutputContext {
                        registration: &mut reports.output_registration,
                        input,
                        prescan: &reports.output_prescan,
                        reports: &self.model.report_inputs,
                        minimum_report_frequency: &project.system.minimum_report_frequency,
                        minutes_in_time_step: weather.state.global.minutes_in_time_step,
                        host: &mut **output_host,
                        dictionary_host: &mut dictionary,
                        eio,
                    },
                    bnd,
                )
                .map_err(hb_input_error)
            },
        )
    }
    fn setup_pollution_meter_reporting(&mut self) -> Result<(), RuntimeError> {
        self.backend.with_scoped(
            &mut (
                &mut *self.reports,
                &mut *self.session,
                &mut *self.files,
                &mut *self.output_host,
                &mut *self.weather,
            ),
            |mut backend, (reports, session, files, output_host, weather)| {
                let session = required(
                    session.as_mut(),
                    "actual same Pollution module before startup",
                )?;
                let project = required(
                    reports.selected_project_data.as_ref(),
                    "actual Project before Pollution startup",
                )?;
                let input = required(
                    reports.selected_input_processor.as_mut(),
                    "actual same IP before Pollution startup",
                )?;
                let (mut dictionary, eio) = files.source_report_streams_and_eio(
                    &reports.source_sqlite,
                    &mut reports.source_results_output_variables,
                );
                session
                    .hvac
                    .ancillary
                    .pollution
                    .setup_source_pollution_meter_reporting(
                        &weather.state.global,
                        input,
                        &mut reports.input_processor_shortcut,
                        &mut backend,
                        &mut SourcePollutionOutputContext {
                            registration: &mut reports.output_registration,
                            prescan: &reports.output_prescan,
                            reports: &self.model.report_inputs,
                            minimum_report_frequency: &project.system.minimum_report_frequency,
                            host: &mut **output_host,
                            dictionary_host: &mut dictionary,
                        },
                        eio,
                    )
                    .map_err(hb_input_error)
            },
        )
    }
    fn allocate_and_setup_ventilation_reports(&mut self) -> Result<(), RuntimeError> {
        self.backend.with_scoped(
            &mut (
                &mut *self.reports,
                &mut *self.session,
                &mut *self.files,
                &mut *self.output_host,
                &mut *self.weather,
            ),
            |mut backend, (reports, session, files, output_host, weather)| {
                let session =
                    required(session.as_mut(), "actual same SystemReports before startup")?;
                let project = required(
                    reports.selected_project_data.as_ref(),
                    "actual Project before ventilation outputs",
                )?;
                let input = required(
                    reports.selected_input_processor.as_mut(),
                    "actual same IP before ventilation outputs",
                )?;
                let (mut dictionary, eio) = files.source_report_streams_and_eio(
                    &reports.source_sqlite,
                    &mut reports.source_results_output_variables,
                );
                let mut owners = SourceSystemReportOwners {
                    globals: required(
                        reports.selected_heat_balance_globals.as_ref(),
                        "actual globals before ventilation outputs",
                    )?,
                    hvac: &required(
                        reports.selected_simulation_startup.as_ref(),
                        "actual HVAC globals before ventilation outputs",
                    )?
                    .hvac_globals,
                    equipment: required(
                        reports.source_zone_equipment.as_ref(),
                        "actual zone equipment before ventilation outputs",
                    )?,
                    zones: required(
                        reports.selected_zone_input.as_ref(),
                        "actual zones before ventilation outputs",
                    )?,
                    air_loop: &session.hvac.simulation.air_loop,
                    predefined_registry: &reports.registry,
                    predefined_table: &mut reports.table,
                    predefined_summary: &mut session.comfort_summary,
                };
                session
                    .hvac
                    .ancillary
                    .system_reports
                    .allocate_and_setup_source_vent_reports(
                        &mut owners,
                        &mut reports.output_registration,
                        input,
                        &mut backend,
                        &mut SourceSystemReportsOutputContext {
                            prescan: &reports.output_prescan,
                            reports: &self.model.report_inputs,
                            minimum_report_frequency: &project.system.minimum_report_frequency,
                            minutes_in_time_step: weather.state.global.minutes_in_time_step,
                            host: &mut **output_host,
                            eio,
                            dictionary_host: &mut dictionary,
                        },
                    )
                    .map_err(hb_input_error)
            },
        )
    }
    fn setup_plugin_output_variables(&mut self) -> Result<(), RuntimeError> {
        // Actual installed26.1 reference CLI proves LINK_WITH_PYTHON enabled.
        // The literal parent has already tested this same manager's presence.
        let manager = required(
            self.reports.selected_plugin_manager.as_ref(),
            "actual retained PluginManager before output setup296",
        )?;
        let input = required(
            self.reports.selected_input_processor.as_mut(),
            "actual same IP before PluginManager output setup296",
        )?;
        manager
            .setup_source_output_variables_with_python(input, self.backend)
            .map_err(|source| {
                crate::heat_balance::source_initialization::SourceInitializationError::Handler {
                    handler: "PluginManager::setupOutputVariables296",
                    source: Box::new(source),
                }
                .into()
            })
    }
    fn update_meter_reporting(&mut self) -> Result<(), RuntimeError> {
        self.backend.with_scoped(
            &mut (
                &mut *self.reports,
                &mut *self.files,
                &mut *self.output_host,
                &*self.weather,
            ),
            |mut backend, (reports, files, output_host, weather)| {
                let raw = required(
                    reports.heat_balance_input.as_ref(),
                    "actual raw input before UpdateMeterReporting",
                )?;
                let input = required(
                    reports.selected_input_processor.as_mut(),
                    "actual same IP before UpdateMeterReporting",
                )?;
                let (mut streams, _eio) = files.source_report_streams_and_eio(
                    &reports.source_sqlite,
                    &mut reports.source_results_output_variables,
                );
                let mut meters = streams.meter_streams(output_host);
                reports
                    .output_registration
                    .update_source_meter_reporting(
                        &mut SourceMeterReportingOwners {
                            raw,
                            input,
                            shortcut: &mut reports.input_processor_shortcut,
                            backend: &mut backend,
                            diagnostic_flags: SourceDiagnosticFlags {
                                kickoff: &required(
                                    reports.selected_heat_balance_globals.as_ref(),
                                    "actual meter globals",
                                )?
                                .kickoff_simulation,
                                developer: &required(
                                    reports.selected_project_data.as_ref(),
                                    "actual meter SystemVariables",
                                )?
                                .system
                                .developer_flag,
                                warmup: &weather.state.global.warmup_flag,
                                doing_sizing: &required(
                                    reports.selected_heat_balance_globals.as_ref(),
                                    "actual meter sizing flag",
                                )?
                                .doing_sizing,
                            },
                        },
                        &mut meters,
                    )
                    .map_err(hb_input_error)
            },
        )
    }
    fn check_pollution_meter_reporting(&mut self) -> Result<(), RuntimeError> {
        required(self.session.as_mut(), "actual Pollution before meter check")?
            .hvac
            .ancillary
            .pollution
            .check_source_pollution_meter_reporting(&self.reports.output_registration, self.backend)
            .map_err(hb_input_error)
    }
    fn verify_custom_electric_power_meters(&mut self) -> Result<(), RuntimeError> {
        required(
            self.reports.facility_electric_service.as_mut(),
            "actual electric service before custom meter check",
        )?
        .verify_source_custom_meters(&self.reports.output_registration)
        .map_err(hb_input_error)
    }
    fn setup_pollution_calculations(&mut self) -> Result<(), RuntimeError> {
        let input = required(
            self.reports.selected_input_processor.as_mut(),
            "actual same IP before Pollution calculations",
        )?;
        required(
            self.session.as_mut(),
            "actual Pollution before calculations setup",
        )?
        .hvac
        .ancillary
        .pollution
        .setup_source_pollution_calculations(input, &mut self.reports.input_processor_shortcut)
        .map_err(hb_input_error)
    }
    fn initialize_demand_managers(&mut self) -> Result<(), RuntimeError> {
        let input = required(
            self.reports.selected_input_processor.as_ref(),
            "actual same IP before InitDemandManagers",
        )?;
        required(self.session.as_mut(), "actual DemandManager before startup")?
            .hvac
            .ancillary
            .postlude_modules
            .demand
            .initialize_source_demand_managers(input)
            .map_err(hb_input_error)
    }
    fn test_branch_integrity(&mut self, errors_found: &mut bool) -> Result<(), RuntimeError> {
        self.reports
            .selected_branches
            .test_source_branch_integrity(errors_found, self.files.bnd_stream(), self.backend)
            .map_err(hb_input_error)
    }
    fn test_air_path_integrity(&mut self, errors_found: &mut bool) -> Result<(), RuntimeError> {
        let session = required(
            self.session.as_ref(),
            "actual same air loop/ancillary before TestAirPathIntegrity",
        )?;
        let startup = required(
            self.reports.selected_simulation_startup.as_ref(),
            "actual HVAC globals before TestAirPathIntegrity",
        )?;
        let equipment = required(
            self.reports.source_zone_equipment.as_ref(),
            "actual node/zone equipment before TestAirPathIntegrity",
        )?;
        let input = required(
            self.reports.selected_input_processor.as_ref(),
            "actual same IP before TestAirPathIntegrity",
        )?;
        crate::heat_balance::startup_integrity::test_source_air_path_integrity(
            equipment,
            &startup.hvac_globals,
            &session.hvac.simulation.air_loop,
            &session.hvac.ancillary,
            input,
            errors_found,
            self.files.bnd_stream(),
            self.backend,
        )
        .map_err(hb_input_error)
    }
    fn check_marked_nodes(&mut self, errors_found: &mut bool) -> Result<(), RuntimeError> {
        required(
            self.reports.source_zone_equipment.as_ref(),
            "actual same NodeOwner before CheckMarkedNodes",
        )?
        .check_source_marked_nodes(errors_found)
        .map_err(hb_input_error)
    }
    fn check_node_connections(&mut self, errors_found: &mut bool) -> Result<(), RuntimeError> {
        required(
            self.session.as_mut(),
            "actual same BranchNodeConnections before check",
        )?
        .hvac
        .ancillary
        .branch_node_connections
        .check_source_node_connections(errors_found)
        .map_err(hb_input_error)
    }
    fn test_component_set_inlet_outlet_nodes(
        &mut self,
        errors_found: &mut bool,
    ) -> Result<(), RuntimeError> {
        required(
            self.session.as_ref(),
            "actual same BranchNodeConnections before component check",
        )?
        .hvac
        .ancillary
        .branch_node_connections
        .test_source_component_sets(errors_found)
        .map_err(hb_input_error)
    }
    fn check_controller_lists(&mut self, errors_found: &mut bool) -> Result<(), RuntimeError> {
        let session = required(
            self.session.as_mut(),
            "actual same MixedAir and AirLoop before controller check",
        )?;
        let sizing = &mut self.reports.selected_sizing_manager;
        let input = required(
            self.reports.selected_input_processor.as_mut(),
            "actual same IP before controller check",
        )?;
        session
            .hvac
            .ancillary
            .mixed_air
            .check_source_controller_lists(
                &mut session.hvac.simulation.air_loop,
                sizing,
                input,
                &mut self.reports.input_processor_shortcut,
                errors_found,
                self.backend,
            )
            .map_err(hb_input_error)
    }
    fn report_loop_connections(&mut self) -> Result<(), RuntimeError> {
        let session = required(
            self.session.as_ref(),
            "actual same connection/Plant/DualDuct owners before loop report",
        )?;
        let startup = required(
            self.reports.selected_simulation_startup.as_ref(),
            "actual same HVAC globals before loop report",
        )?;
        let owners = crate::heat_balance::startup_integrity::SourceLoopConnectionReportOwners {
            connections: &session.hvac.ancillary.branch_node_connections,
            outdoor_air_nodes: &session.hvac.ancillary.outdoor_air_nodes,
            hvac: &startup.hvac_globals,
            plant: &session.hvac.simulation.plant,
            equipment: required(
                self.reports.source_zone_equipment.as_ref(),
                "actual same ZoneEquipConfig/Node owner before loop report",
            )?,
            globals: required(
                self.reports.selected_heat_balance_globals.as_ref(),
                "actual same DataGlobal before loop report",
            )?,
            dual_duct: &session.hvac.ancillary.dual_duct,
        };
        crate::heat_balance::startup_integrity::report_source_loop_connections(
            &owners,
            self.files.bnd_stream(),
            self.backend,
        )
        .map_err(hb_input_error)
    }
    fn report_air_loop_connections(&mut self) -> Result<(), RuntimeError> {
        let startup = required(
            self.reports.selected_simulation_startup.as_ref(),
            "actual same HVAC globals before air loop report",
        )?;
        crate::heat_balance::startup_integrity::report_source_air_loop_connections(
            &startup.hvac_globals,
            self.files.bnd_stream(),
        )
        .map_err(hb_input_error)
    }
    fn report_node_connections(&mut self) -> Result<(), RuntimeError> {
        let session = required(
            self.session.as_mut(),
            "actual same BranchNodeConnections owner before node report",
        )?;
        let equipment = required(
            self.reports.source_zone_equipment.as_ref(),
            "actual same NodeOwner before node report",
        )?;
        session
            .hvac
            .ancillary
            .branch_node_connections
            .report_source_node_connections(equipment, self.files.bnd_stream())
            .map_err(hb_input_error)
    }
    fn create_energy_report_structure(&mut self) -> Result<(), RuntimeError> {
        let session = required(
            self.session.as_mut(),
            "actual SystemReports before energy structure",
        )?;
        let globals = required(
            self.reports.selected_heat_balance_globals.as_ref(),
            "actual globals before energy structure",
        )?;
        let hvac = &required(
            self.reports.selected_simulation_startup.as_ref(),
            "actual HVAC globals before energy structure",
        )?
        .hvac_globals;
        let equipment = required(
            self.reports.source_zone_equipment.as_mut(),
            "actual zone equipment before energy structure",
        )?;
        let ancillary = &mut session.hvac.ancillary;
        ancillary
            .system_reports
            .create_source_energy_report_structure(
                globals,
                hvac,
                equipment,
                &ancillary.branch_node_connections,
                &self.reports.output_registration,
            )
            .map_err(hb_input_error)
    }
    fn produce_rdd_mdd(&mut self) -> Result<(), RuntimeError> {
        self.reports
            .produce_source_rdd_mdd(&self.model.report_inputs, self.files, self.backend)
    }
    fn show_fatal_error(&mut self, message: &str) -> Result<Infallible, RuntimeError> {
        self.backend
            .show_fatal_error(message)
            .map_err(|stop| hb_input_error(Error::Fatal(stop)))
    }
    fn sqlite_present(&self) -> bool {
        self.reports.source_sqlite.present()
    }
    fn sqlite_begin(&mut self) -> Result<(), RuntimeError> {
        Err(unavailable(
            "actual positive pre-primary SQLite sqliteBegin",
        ))
    }
    fn sqlite_update_simulation_record(
        &mut self,
        _simulation_index: i32,
        _time_steps_in_hour: i32,
    ) -> Result<(), RuntimeError> {
        Err(unavailable(
            "actual positive pre-primary SQLite updateSQLiteSimulationRecord",
        ))
    }
    fn sqlite_commit(&mut self) -> Result<(), RuntimeError> {
        Err(unavailable(
            "actual positive pre-primary SQLite sqliteCommit",
        ))
    }
    fn get_life_cycle_cost_input(&mut self) -> Result<(), RuntimeError> {
        let input = required(
            self.reports.selected_input_processor.as_mut(),
            "actual same IP before LCC input",
        )?;
        self.reports
            .source_life_cycle_cost
            .get_source_input(
                input,
                &mut self.reports.input_processor_shortcut,
                self.backend,
            )
            .map_err(hb_input_error)
    }
    fn read_variable_location_orientation(&mut self) -> Result<(), RuntimeError> {
        let input = required(
            self.reports.selected_input_processor.as_ref(),
            "actual same IP before variable location",
        )?;
        self.weather
            .state
            .read_source_variable_location_orientation(
                input,
                &mut self.reports.input_processor_shortcut,
            )
            .map_err(hb_input_error)
    }
    fn manage_hvac_sizing_simulation(&mut self) -> Result<(), RuntimeError> {
        Err(unavailable(
            "actual positive ManageHVACSizingSimulation child",
        ))
    }
}
impl SourceInitializationFiles {
    /// Actual284–367 continuation after genuine kickoff274–281.
    /// # Errors
    /// Retains same session/report/file prefixes. Required integrity/exporter
    /// and positive domain children stay explicit until their bodies are ported.
    pub fn source_primary_startup(
        &mut self,
        session: &mut Option<OrdinarySourceSession>,
        reports: &mut CtfInitializationReports,
        weather: &ProductionWeatherTimestepSeries,
        model: &TypedModel,
        output_host: &mut SourceMtdFileHost,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), RuntimeError> {
        weather.with_source_weather_session_mut(|weather| {
            self.source_primary_startup_on_session(
                session,
                reports,
                weather,
                model,
                output_host,
                backend,
            )
        })
    }
    pub(crate) fn source_primary_startup_on_session(
        &mut self,
        session: &mut Option<OrdinarySourceSession>,
        reports: &mut CtfInitializationReports,
        weather: &mut WeatherSession,
        model: &TypedModel,
        output_host: &mut SourceMtdFileHost,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), RuntimeError> {
        prepare_source_primary_simulation(&mut PreludeHost {
            files: self,
            session,
            reports,
            weather,
            model,
            output_host,
            backend,
        })
    }
}
