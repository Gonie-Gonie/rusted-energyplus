//! Genuine ManageSimulation566-620 endrun caller on the same retained owners.
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
    /// Continue on the original owners after Primary564 through actual620.
    /// Does not replay the primary Warmup=false writer or construct/reset a PSY
    /// scope or simulation local. Actual594 opens and602 closes the same TAB files.
    /// The caller retains the same
    /// files/backend/weather session and the existing single outer PSY scope.
    ///
    /// # Errors
    /// Preserves each typed child cause and all preceding effects. Positive SQL
    /// sqliteBegin582 remains unavailable. Represented children execute through
    /// their real completion, including physical output closure606 and final620.
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
            reports.source_zone_equipment.as_mut(),
            "actual same Node owner before setpoint nodes590",
        )?
        .check_source_setpoint_nodes_at_end(
            &required(
                reports.selected_simulation_startup.as_ref(),
                "actual same RuntimeLanguage before setpoint node helper590",
            )?
            .runtime_language,
        )
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
        self.write_source_tabular_reports_prefix_on_session(session, reports, weather, backend)?;
        // The Native children below are outside WriteTabularReports and run
        // after its unconditional audit suffix, even if WriteTabularFiles=false.
        let session = required(session.as_mut(), "actual same session before tariff reports598")?;
        let globals = required(
            reports.selected_heat_balance_globals.as_ref(),
            "actual same Globals before tariff reports598",
        )?;
        let project = required(
            reports.selected_project_data.as_ref(),
            "actual same SysVars before outer endrun598",
        )?;
        reports.economic_tariff.write_source_tabular_tariff_reports(
            &mut session.tabular,
            globals,
            required(
                reports.selected_zone_input.as_ref(),
                "actual same Zones before tariff floor-area prerequisite598",
            )?,
            &mut *backend,
            &SourceDiagnosticFlags {
                kickoff: &globals.kickoff_simulation,
                developer: &project.system.developer_flag,
                warmup: &weather.state.global.warmup_flag,
                doing_sizing: &globals.doing_sizing,
            },
        ).map_err(hb_input_error)?;
        reports.source_life_cycle_cost.compute_source_life_cycle_cost_and_report(
            &mut *backend,
            &SourceDiagnosticFlags {
                kickoff: &globals.kickoff_simulation,
                developer: &project.system.developer_flag,
                warmup: &weather.state.global.warmup_flag,
                doing_sizing: &globals.doing_sizing,
            },
        ).map_err(hb_input_error)?;
        session.tabular.close_source_tabular_files().map_err(hb_input_error)?;
        session.hvac.dump_source_air_loop_statistics(&project.system).map_err(hb_input_error)?;
        self.close_source_output_files(
            &mut crate::heat_balance::source_initialization::SourceCloseOutputFilesOwners {
                registration: &reports.output_registration,
                tabular: &session.tabular,
                solar: required(
                    reports.ordinary_solar_radiation.as_ref(),
                    "actual same SolarShading before CloseOutputFiles606",
                )?,
                geometry: &reports.surface_geometry_input,
                system: &project.system,
                prescan: &reports.output_prescan,
                runtime_language: &required(
                    reports.selected_simulation_startup.as_ref(),
                    "actual same RuntimeLanguage before CloseOutputFiles606",
                )?.runtime_language,
                connections: &session.hvac.ancillary.branch_node_connections,
                heat_balance: &reports.heat_balance_input_state,
                standard_output_record_count: &reports.source_standard_output_record_count,
                standard_meter_record_count: &reports.source_standard_meter_record_count,
                shading_stream: &mut session.shading_stream,
                output_control: &reports.source_output_control,
            },
        ).map_err(hb_input_error)?;
        // Actual609 child first tests this same database pointer. Null has no
        // further reads/effects; positive first needs its real DB predicate.
        if reports.source_sqlite.present() {
            return Err(unavailable(
                "actual CreateSQLiteZoneExtendedOutput167 sqlite.writeOutputToSQLite predicate",
            ));
        }
        if reports.source_sqlite.present() {
            backend.display_source_string(
                "Writing final SQL reports",
                &SourceDiagnosticFlags {
                    kickoff: &globals.kickoff_simulation,
                    developer: &project.system.developer_flag,
                    warmup: &weather.state.global.warmup_flag,
                    doing_sizing: &globals.doing_sizing,
                },
            ).map_err(hb_input_error)?;
            return Err(unavailable(
                "actual final SQL sqliteCommit613 before initializeIndexes614",
            ));
        }
        // The actual SimulationManager179 ErrorsFound cell is retained by
        // this same WeatherSession across prelude, Setup and Primary. It is
        // distinct from HeatBalance's separate input ErrorsFound member.
        if weather.errors_found {
            return match backend.show_fatal_error(
                "Error condition occurred.  Previous Severe Errors cause termination.",
            ) {
                Ok(never) => match never {},
                Err(stop) => Err(hb_input_error(HeatBalanceInputError::Fatal(stop))),
            };
        }
        Ok(()) // Genuine ManageSimulation620, after all real preceding children.
    }
}

impl SourceInitializationFiles {
    /// Reborrow the same production weather session after genuine Primary564.
    /// The single outer PSY scope and every retained run/file owner remain live.
    /// No simulation local, clock or first-call flag is initialized or reset here.
    ///
    /// # Errors
    /// Returns the actual typed child failure or genuine successful endrun620.
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
    // Actual WriteTabularReports5212-5295 on the same retained owners. The
    // outer on-session call already holds the single real weather borrow.
    fn write_source_tabular_reports_prefix_on_session(
        &mut self,
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
                    weather: &weather.state.weather,
                    clock: &weather.state.global,
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
                    equipment: required(reports.source_zone_equipment.as_ref(),
                        "actual same ZoneEquipConfig before FillRemaining5213")?,
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
            session.tabular.write_source_component_sizing(&mut reports.registry)
                .map_err(hb_input_error)?;
            session.tabular.write_source_surface_shadowing(
                &mut crate::heat_balance::ordinary_tabular_reports::SourceSurfaceShadowingReportOwners {
                    geometry: &reports.surface_geometry_input,
                    radiation: required(
                        reports.ordinary_solar_radiation.as_ref(),
                        "actual same ShadowComb owner before SurfaceShadowing5236",
                    )?,
                    registry: &mut reports.registry,
                    sqlite: &reports.source_sqlite,
                    results: &reports.source_results_framework,
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
            session.tabular.write_source_component_cost_table(&reports.cost_estimate_monetary)
                .map_err(hb_input_error)?;
            session.tabular.write_source_adaptive_comfort_table(&reports.selected_internal_heat_gains)
                .map_err(hb_input_error)?;
            {
                let mut next_owners = crate::heat_balance::ordinary_tabular_reports::SourcePostSummaryReportOwners {
                    globals,
                    zones: required(
                        reports.selected_zone_input.as_ref(),
                        "actual same BuildingPreDefRep before EIO5239",
                    )?,
                    hvac: &required(
                        reports.selected_simulation_startup.as_ref(),
                        "actual same HVAC globals before EIO5239",
                    )?.hvac_globals,
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
                };
                session.tabular.write_source_eio_tables(self, &mut next_owners)
                    .map_err(hb_input_error)?;
                session.tabular.write_source_load_component_summary(&next_owners)
                    .map_err(hb_input_error)?;
                session.tabular.write_source_heat_emission_table(&mut next_owners)
                    .map_err(hb_input_error)?;
            }
            if session.tabular.display_thermal_resilience_summary && !globals.do_pure_load_calc {
                session.tabular.write_source_thermal_resilience_tables(
                    &mut crate::heat_balance::ordinary_tabular_reports::SourceResilienceTableOwners {
                        globals,
                        zones: required(
                            reports.selected_zone_input.as_ref(),
                            "actual same Zone/Resilience/ZoneDaylight before thermal resilience5244",
                        )?,
                        gains: &reports.selected_internal_heat_gains,
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
            }
            if session.tabular.display_co2_resilience_summary && !globals.do_pure_load_calc {
                session.tabular.write_source_co2_resilience_tables(
                    &mut crate::heat_balance::ordinary_tabular_reports::SourceResilienceTableOwners {
                        globals,
                        zones: required(
                            reports.selected_zone_input.as_ref(),
                            "actual same Zone/Resilience/ZoneDaylight before co2 resilience5247",
                        )?,
                        gains: &reports.selected_internal_heat_gains,
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
            }
            if session.tabular.display_visual_resilience_summary && !globals.do_pure_load_calc {
                session.tabular.write_source_visual_resilience_tables(
                    &mut crate::heat_balance::ordinary_tabular_reports::SourceResilienceTableOwners {
                        globals,
                        zones: required(
                            reports.selected_zone_input.as_ref(),
                            "actual same Zone/Resilience/ZoneDaylight before visual resilience5250",
                        )?,
                        gains: &reports.selected_internal_heat_gains,
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
            }
            if weather.state.weather.tot_report_periods > 0 {
                return Err(unavailable("actual WriteTabularReports5253 WriteReportPeriodTimeConsumption12480 local array allocation entry"));
            }
            for _period in 1..=weather.state.weather.tot_thermal_report_periods {
                return Err(unavailable("actual WriteTabularReports5256 WriteThermalResilienceTablesRepPeriod12539 actual WriteTabularFiles and styles entry"));
            }
            for _period in 1..=weather.state.weather.tot_co2_report_periods {
                // Native13690 intentionally returns when WriteTabularFiles is
                // true. Preserve that original guard, including its TODO bug.
                if !*session.tabular.source_write_tabular_files_mut() {
                    return Err(unavailable("actual CO2ResilienceRepPeriod13692 tabularReportPasses first header operation"));
                }
            }
            for _period in 1..=weather.state.weather.tot_visual_report_periods {
                return Err(unavailable("actual WriteTabularReports5262 WriteVisualResilienceTablesRepPeriod13801 actual WriteTabularFiles and styles entry"));
            }
            required(
                reports.coil_selection_report.as_mut(),
                "actual same coilSelectionReportObj before finishCoilSummary5265",
            )?.finish_source_coil_summary_report_table().map_err(hb_input_error)?;
            session.tabular.write_source_predefined_tables(
                &mut crate::heat_balance::ordinary_tabular_reports::SourcePredefinedReportOwners {
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
            if weather.state.global.do_weath_sim {
                session.tabular.write_source_monthly_tables().map_err(hb_input_error)?;
                session.tabular.write_source_time_bin_tables().map_err(hb_input_error)?;
                session.tabular.write_source_annual_tables().map_err(hb_input_error)?;
            }
        }
        session.tabular.write_source_tabular_audit(
            self,
            &reports.source_output_control,
            &reports.registry,
            &reports.table,
        ).map_err(hb_input_error)?;
        Ok(())
    }
}
