//! Genuine ManageSimulation566-596 prefix; later tabular report children remain bounded.
use crate::RuntimeError;
use crate::heat_balance::ctf_initialization_context::{CtfInitializationReports, hb_input_error};
use crate::heat_balance::error_backend::{SourceDiagnosticFlags, SourceErrorBackend};
use crate::heat_balance::ordinary_source_session::OrdinarySourceSession;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError;
use crate::heat_balance::source_initialization::SourceInitializationFiles;
use crate::weather::day::WeatherSession;

fn unavailable(name: &'static str) -> RuntimeError {
    RuntimeError::HeatBalanceInput(Box::new(HeatBalanceInputError::Unavailable(name)))
}
fn required<T>(value: Option<T>, name: &'static str) -> Result<T, RuntimeError> {
    value.ok_or_else(|| unavailable(name))
}
fn warning(
    reports: &CtfInitializationReports,
    weather: &WeatherSession,
    backend: &mut SourceErrorBackend<'_>,
    message: &str,
) -> Result<(), RuntimeError> {
    let globals = required(
        reports.selected_heat_balance_globals.as_ref(),
        "actual Globals before endrun warning",
    )?;
    let project = required(
        reports.selected_project_data.as_ref(),
        "actual SysVars before endrun warning",
    )?;
    backend
        .show_source_warning_error(
            message,
            &SourceDiagnosticFlags {
                kickoff: &globals.kickoff_simulation,
                developer: &project.system.developer_flag,
                warmup: &weather.state.global.warmup_flag,
                doing_sizing: &globals.doing_sizing,
            },
        )
        .map_err(hb_input_error)
}
impl SourceInitializationFiles {
    /// Continue on the original owners after Primary564, through the actual596 predefined prefix.
    /// Does not replay the primary Warmup=false writer, construct/reset a PSY
    /// scope or local owner, or close any file. Actual594 opens the same TAB files.
    /// The caller retains the same
    /// files/backend/weather session and the existing single outer PSY scope.
    ///
    /// # Errors
    /// Preserves each typed child cause and all preceding effects. Positive SQL
    /// sqliteBegin582 remains unavailable. Actual596 preserves each genuine child
    /// cause and stops at the next unported operation in Source order.
    pub fn source_simulation_endrun_prefix_on_session(
        &mut self,
        session: &mut Option<OrdinarySourceSession>,
        reports: &mut CtfInitializationReports,
        weather: &mut WeatherSession,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), RuntimeError> {
        if !*required(
            reports.source_run_sims_done.as_ref(),
            "actual SimsDone236 before endrun566",
        )? && weather.state.global.do_des_day_sim
        {
            let days = weather
                .state
                .environment
                .tot_des_days
                .checked_add(weather.state.environment.tot_run_des_pers_days)
                .ok_or_else(|| {
                    unavailable("actual endrun567 sum outside defined Native int domain")
                })?;
            if days == 0 {
                warning(
                    reports,
                    weather,
                    backend,
                    "ManageSimulation: SizingPeriod:* were requested in SimulationControl but no SizingPeriod:* objects in input.",
                )?;
            }
        }
        if !*required(
            reports.source_run_sims_done.as_ref(),
            "actual SimsDone236 before endrun573",
        )? && weather.state.global.do_weath_sim
        {
            if !required(
                reports.selected_simulation_startup.as_ref(),
                "actual SimulationManager before endrun574",
            )?
            .run_periods_in_input
            {
                warning(
                    reports,
                    weather,
                    backend,
                    "ManageSimulation: Weather Simulation was requested in SimulationControl but no RunPeriods in input.",
                )?;
            }
        }

        required(
            session.as_ref(),
            "actual same Source session before plant warnings579",
        )?
        .hvac
        .check_source_ongoing_plant_warnings()
        .map_err(hb_input_error)?;
        if reports.source_sqlite.present() {
            return Err(unavailable("actual positive endrun SQLite sqliteBegin582"));
        }
        let input = required(
            reports.selected_input_processor.as_ref(),
            "actual sole IP before endrun cost585",
        )?;
        let globals = required(
            reports.selected_heat_balance_globals.as_ref(),
            "actual same Globals before endrun cost585",
        )?;
        reports
            .cost_estimate_monetary
            .simulate_source_cost_estimate(input, globals.kickoff_simulation)
            .map_err(hb_input_error)?;
        reports
            .economic_tariff
            .compute_source_tariff(
                &reports.source_output_control,
                &reports.source_results_framework,
                &mut required(
                    session.as_mut(),
                    "actual same Source TAB owner before tariff587",
                )?
                .tabular,
            )
            .map_err(hb_input_error)?;
        required(
            reports.selected_simulation_startup.as_ref(),
            "actual RuntimeLanguage before unused actuators589",
        )?
        .runtime_language
        .check_source_unused_actuators_at_end()
        .map_err(hb_input_error)?;
        required(
            reports.source_zone_equipment.as_ref(),
            "actual same Node owner before setpoint nodes590",
        )?
        .check_source_setpoint_nodes_at_end()
        .map_err(hb_input_error)?;
        reports
            .output_registration
            .report_source_for_tabular_reports(
                &reports.registry,
                &mut reports.table,
                &weather.state.global.time_step_zone_sec,
            )
            .map_err(hb_input_error)?;
        self.open_source_output_tabular_file_on_session(session, reports, weather, backend)?;
        self.write_source_tabular_reports_prefix_on_session(session, reports, weather, backend)
    }
}

impl SourceInitializationFiles {
    /// Reborrow the same production weather session after genuine Primary564.
    /// The single outer PSY scope and every retained run/file owner remain live.
    /// No simulation local, clock or first-call flag is initialized or reset here.
    ///
    /// # Errors
    /// Returns the actual typed prefix child failure, including the actual next tabular child boundary.
    pub fn source_simulation_endrun_prefix(
        &mut self,
        session: &mut Option<OrdinarySourceSession>,
        reports: &mut CtfInitializationReports,
        weather: &crate::weather::day::ProductionWeatherTimestepSeries,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), RuntimeError> {
        weather.with_source_weather_session_mut(|actual| {
            self.source_simulation_endrun_prefix_on_session(session, reports, actual, backend)
        })
    }
}

impl SourceInitializationFiles {
    // Actual WriteTabularReports5212-5227 on the same retained owners. The
    // existing outer on-session call already holds the single weather borrow.
    fn write_source_tabular_reports_prefix_on_session(
        &self,
        session: &mut Option<OrdinarySourceSession>,
        reports: &mut CtfInitializationReports,
        weather: &WeatherSession,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), RuntimeError> {
        self.fill_source_weather_predefined_entries_on_session(session, reports)?;
        let session = required(session.as_mut(), "actual Source TAB before FillRemaining5213")?;
        session
            .tabular
            .fill_source_remaining_predefined_entries(
                &mut crate::heat_balance::ordinary_tabular_reports::remaining::SourceRemainingPredefinedOwners {
                    hvac: &mut required(
                        reports.selected_simulation_startup.as_mut(),
                        "actual same HVAC globals before FillRemaining5213",
                    )?.hvac_globals,
                    prescan: &reports.output_prescan,
                    environment: &weather.state.environment,
                    gains: &reports.selected_internal_heat_gains,
                    exterior: &session.exterior,
                    schedules: required(
                        reports.selected_schedule_manager.as_ref(),
                        "actual same schedules before FillRemaining5213",
                    )?,
                    globals: required(
                        reports.selected_heat_balance_globals.as_ref(),
                        "actual same Globals before FillRemaining5213",
                    )?,
                    zones: required(
                        reports.selected_zone_input.as_mut(),
                        "actual same ZonePreDefRep owner before FillRemaining5213",
                    )?,
                    controller: &session.controller,
                    predefined_summary: &session.comfort_summary,
                    zone_plenum: &session.hvac.ancillary.zone_plenum,
                    hvac_manager: &session.hvac,
                    gain_modules: &session.gain_modules,
                    weather_file_location_title: &weather.cursor.header().weather_location_title,
                    registry: &reports.registry,
                    table: &mut reports.table,
                },
            )
            .map_err(hb_input_error)?;
        session
            .hvac
            .ancillary
            .water
            .report_source_rainfall(
                &reports.registry,
                &mut reports.table,
                &reports.heat_balance_input_state,
            )
            .map_err(hb_input_error)?;
        if session
            .tabular
            .continue_source_write_tabular_reports_after_predefined(
                &reports.source_results_framework,
                &reports.source_sqlite,
            )
        {
            let globals = required(
                reports.selected_heat_balance_globals.as_ref(),
                "actual same Globals before BEPS5230",
            )?;
            let project = required(
                reports.selected_project_data.as_ref(),
                "actual same SysVars before BEPS5230 diagnostics",
            )?;
            session.tabular.write_source_beps_report(
                &mut crate::heat_balance::ordinary_tabular_reports::SourceBepsReportOwners {
                    registration: &reports.output_registration,
                    zones: required(
                        reports.selected_zone_input.as_ref(),
                        "actual same Zones before BEPS5230",
                    )?,
                    globals,
                    facility: required(
                        reports.facility_electric_service.as_ref(),
                        "actual same FacilityElectricService before BEPS5230",
                    )?,
                    water: &session.hvac.ancillary.water.data,
                    hvac: &required(
                        reports.selected_simulation_startup.as_ref(),
                        "actual same HVAC globals before BEPS5230",
                    )?.hvac_globals,
                    predefined_summary: &session.comfort_summary,
                    registry: &mut reports.registry,
                    table: &mut reports.table,
                    schedules: required(
                        reports.selected_schedule_manager.as_ref(),
                        "actual same schedules before BEPS5230",
                    )?,
                    results: &reports.source_results_framework,
                    sqlite: &reports.source_sqlite,
                    writer: crate::heat_balance::ordinary_tabular_reports::SourceTabularWriterOwners {
                        monetary: &reports.cost_estimate_monetary,
                        flags: SourceDiagnosticFlags {
                            kickoff: &globals.kickoff_simulation,
                            developer: &project.system.developer_flag,
                            warmup: &weather.state.global.warmup_flag,
                            doing_sizing: &globals.doing_sizing,
                        },
                        backend,
                    },
                },
            ).map_err(hb_input_error)?;
            session.tabular.write_source_table_of_contents(
                &crate::heat_balance::ordinary_tabular_reports::SourceTabularTocOwners {
                    registry: &reports.registry,
                    life_cycle_cost: &reports.source_life_cycle_cost,
                    monetary: &reports.cost_estimate_monetary,
                    clock: &weather.state.global,
                    weather: &weather.state.weather,
                    schedules: required(
                        reports.selected_schedule_manager.as_ref(),
                        "actual same schedules before TOC5231",
                    )?,
                },
            ).map_err(hb_input_error)?;
            session.tabular.write_source_verification_summary(
                &mut crate::heat_balance::ordinary_tabular_reports::SourceVeriSumOwners {
                    globals,
                    environment: &weather.state.environment,
                    weather_file_location_title: &weather.cursor.header().weather_location_title,
                    control: &reports.heat_balance_input_state,
                    geometry: &reports.surface_geometry_input,
                    zones: required(
                        reports.selected_zone_input.as_ref(),
                        "actual same Zones before VeriSum5232",
                    )?,
                    gains: &reports.selected_internal_heat_gains,
                    controller: &session.controller,
                    radiation: required(
                        reports.ordinary_solar_radiation.as_ref(),
                        "actual same Solar enclosure owner before VeriSum5232",
                    )?,
                    input: required(
                        reports.selected_input_processor.as_ref(),
                        "actual same IP before VeriSum5232",
                    )?,
                    registry: &reports.registry,
                    table: &mut reports.table,
                    results: &reports.source_results_framework,
                    sqlite: &reports.source_sqlite,
                    writer: crate::heat_balance::ordinary_tabular_reports::SourceTabularWriterOwners {
                        monetary: &reports.cost_estimate_monetary,
                        flags: SourceDiagnosticFlags {
                            kickoff: &globals.kickoff_simulation,
                            developer: &project.system.developer_flag,
                            warmup: &weather.state.global.warmup_flag,
                            doing_sizing: &globals.doing_sizing,
                        },
                        backend: &mut *backend,
                    },
                },
            ).map_err(hb_input_error)?;
            session.tabular.write_source_demand_end_use_summary(
                &mut crate::heat_balance::ordinary_tabular_reports::SourceDemandReportOwners {
                    registration: &reports.output_registration,
                    registry: &reports.registry,
                    table: &mut reports.table,
                    results: &reports.source_results_framework,
                    sqlite: &reports.source_sqlite,
                    writer: crate::heat_balance::ordinary_tabular_reports::SourceTabularWriterOwners {
                        monetary: &reports.cost_estimate_monetary,
                        flags: SourceDiagnosticFlags {
                            kickoff: &globals.kickoff_simulation,
                            developer: &project.system.developer_flag,
                            warmup: &weather.state.global.warmup_flag,
                            doing_sizing: &globals.doing_sizing,
                        },
                        backend: &mut *backend,
                    },
                },
            ).map_err(hb_input_error)?;
            session.tabular.write_source_energy_end_use_summary(
                &mut crate::heat_balance::ordinary_tabular_reports::SourceBepsReportOwners {
                    registration: &reports.output_registration,
                    zones: required(
                        reports.selected_zone_input.as_ref(),
                        "actual same Zones before SourceEnergy5234",
                    )?,
                    globals,
                    facility: required(
                        reports.facility_electric_service.as_ref(),
                        "actual same FacilityElectricService before SourceEnergy5234",
                    )?,
                    water: &session.hvac.ancillary.water.data,
                    hvac: &required(
                        reports.selected_simulation_startup.as_ref(),
                        "actual same HVAC globals before SourceEnergy5234",
                    )?.hvac_globals,
                    predefined_summary: &session.comfort_summary,
                    registry: &mut reports.registry,
                    table: &mut reports.table,
                    schedules: required(
                        reports.selected_schedule_manager.as_ref(),
                        "actual same schedules before SourceEnergy5234",
                    )?,
                    results: &reports.source_results_framework,
                    sqlite: &reports.source_sqlite,
                    writer: crate::heat_balance::ordinary_tabular_reports::SourceTabularWriterOwners {
                        monetary: &reports.cost_estimate_monetary,
                        flags: SourceDiagnosticFlags {
                            kickoff: &globals.kickoff_simulation,
                            developer: &project.system.developer_flag,
                            warmup: &weather.state.global.warmup_flag,
                            doing_sizing: &globals.doing_sizing,
                        },
                        backend: &mut *backend,
                    },
                },
            ).map_err(hb_input_error)?;
            return Err(unavailable("actual WriteTabularReports5235 WriteComponentSizing entry"));
        }
        Err(unavailable("actual WriteTabularReports5278 audit ensure_open entry"))
    }
}
