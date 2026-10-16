//! Genuine ManageSimulation566-594; completion stops before WriteTabularReports596.
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
    /// Continue on the original owners after Primary564, through actual594.
    /// Does not replay the primary Warmup=false writer, construct/reset a PSY
    /// scope or local owner, or close any file. Actual594 opens the same TAB files.
    /// The caller retains the same
    /// files/backend/weather session and the existing single outer PSY scope.
    ///
    /// # Errors
    /// Preserves each typed child cause and all preceding effects. Positive SQL
    /// sqliteBegin582 remains unavailable; after actual594 the genuine next
    /// WriteTabularReports596 entry before FillWeather remains unavailable.
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
        Err(unavailable("actual endrun WriteTabularReports596 entry before FillWeather"))
    }
}

impl SourceInitializationFiles {
    /// Reborrow the same production weather session after genuine Primary564.
    /// The single outer PSY scope and every retained run/file owner remain live.
    /// No simulation local, clock or first-call flag is initialized or reset here.
    ///
    /// # Errors
    /// Returns the actual typed prefix child failure, including the next596 boundary.
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
