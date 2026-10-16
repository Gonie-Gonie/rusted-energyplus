//! Concrete primary-loop children over the same retained runtime and file owners.
//! The enclosing caller must first execute the actual kickoff/startup continuation.
use super::SourceSimulationLoopLocals;
use super::primary::{SourcePrimarySimulationChildren, simulate_source_primary};
use crate::RuntimeError;
use crate::heat_balance::ctf_initialization_context::{CtfInitializationReports, hb_input_error};
use crate::heat_balance::error_backend::{SourceDiagnosticFlags, SourceErrorBackend};
use crate::heat_balance::ordinary_ems_plugin::{
    SourceHeatBalanceEmsCallingPoint, manage_source_heat_balance_ems,
};
use crate::heat_balance::ordinary_source_session::OrdinarySourceSession;
use crate::heat_balance::radiation::ordinary::source_children::SourceSolarReportFlags;
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use crate::heat_balance::selected_project_data::ProjectSystemVariables;
use crate::heat_balance::selected_schedule_manager::SourceScheduleIndex;
use crate::heat_balance::source_initialization::SourceInitializationFiles;
use crate::heat_balance::source_output_registration::{
    SourceMtdFileHost, SourceOutputRegistration,
};
use crate::heat_balance::state::HeatBalanceOrdinaryWarmupReportInputs;
use crate::weather::day::{ProductionWeatherTimestepSeries, WeatherDayState, WeatherSession};
use ep_model::TypedModel;
use std::path::Path;

struct PrimaryHost<'a, 'backend> {
    files: &'a mut SourceInitializationFiles,
    session: &'a mut Option<OrdinarySourceSession>,
    reports: &'a mut CtfInitializationReports,
    weather: &'a mut WeatherSession,
    model: &'a TypedModel,
    reporting: &'a HeatBalanceOrdinaryWarmupReportInputs,
    output_host: &'a mut SourceMtdFileHost,
    backend: &'a mut SourceErrorBackend<'backend>,
    shading_path: &'a Path,
}
fn unavailable(message: &'static str) -> RuntimeError {
    hb_input_error(Error::Unavailable(message))
}
impl SourcePrimarySimulationChildren for PrimaryHost<'_, '_> {
    fn state(&self) -> &WeatherDayState {
        &self.weather.state
    }
    fn state_mut(&mut self) -> &mut WeatherDayState {
        &mut self.weather.state
    }
    fn globals(&self) -> Result<&SelectedHeatBalanceGlobals, RuntimeError> {
        self.reports
            .selected_heat_balance_globals
            .as_ref()
            .ok_or_else(|| unavailable("actual same DataGlobal before primary loop"))
    }
    fn system(&self) -> Result<&ProjectSystemVariables, RuntimeError> {
        self.reports
            .selected_project_data
            .as_ref()
            .map(|project| &project.system)
            .ok_or_else(|| unavailable("actual same SysVars before primary loop"))
    }
    fn report_flags(&self) -> &SourceSolarReportFlags {
        &self.reports.source_report_flags
    }
    fn report_flags_mut(&mut self) -> &mut SourceSolarReportFlags {
        &mut self.reports.source_report_flags
    }
    fn output_registration_mut(&mut self) -> &mut SourceOutputRegistration {
        &mut self.reports.output_registration
    }
    fn primary_available(&self) -> Result<&bool, RuntimeError> {
        self.reports
            .source_run_available
            .as_ref()
            .ok_or_else(|| unavailable("actual ManageSimulation249 Available before primary loop"))
    }
    fn get_next_primary_environment(&mut self) -> Result<(), RuntimeError> {
        self.files
            .source_next_primary_environment_on_session(
                self.session,
                self.reports,
                self.weather,
                self.model,
                self.reporting,
                self.output_host,
                self.backend,
            )
            .map(|_| ())
    }
    fn errors_found(&self) -> bool {
        self.weather.errors_found
    }
    fn sims_done_mut(&mut self) -> Result<&mut bool, RuntimeError> {
        self.reports
            .source_run_sims_done
            .as_mut()
            .ok_or_else(|| unavailable("actual ManageSimulation236 SimsDone before primary loop"))
    }
    fn exit_during_simulations_mut(&mut self) -> Result<&mut bool, RuntimeError> {
        Ok(&mut self.backend.state.tracking.exit_during_simulations)
    }
    fn sqlite_present(&self) -> bool {
        self.reports.source_sqlite.present()
    }
    fn sqlite_begin(&mut self) -> Result<(), RuntimeError> {
        Err(unavailable("actual positive primary SQLite sqliteBegin"))
    }
    fn sqlite_environment_period_record(&mut self) -> Result<(), RuntimeError> {
        Err(unavailable(
            "actual positive primary SQLite createSQLiteEnvironmentPeriodRecord",
        ))
    }
    fn sqlite_commit(&mut self) -> Result<(), RuntimeError> {
        Err(unavailable("actual positive primary SQLite sqliteCommit"))
    }
    fn display_string(&mut self, message: &str) -> Result<(), RuntimeError> {
        let globals = self
            .reports
            .selected_heat_balance_globals
            .as_ref()
            .ok_or_else(|| unavailable("actual globals before live DisplayString"))?;
        let project = self
            .reports
            .selected_project_data
            .as_ref()
            .ok_or_else(|| unavailable("actual SysVars before live DisplayString"))?;
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
    fn reset_node_data(&mut self) -> Result<(), RuntimeError> {
        self.reports
            .source_zone_equipment
            .as_mut()
            .ok_or_else(|| unavailable("actual sole node module before ResetNodeData430"))?
            .reset_source_node_data()
            .map_err(hb_input_error)
    }
    fn manage_begin_new_environment_ems(
        &mut self,
        any_ran: &mut Option<bool>,
    ) -> Result<(), RuntimeError> {
        manage_source_heat_balance_ems(
            self.globals()?,
            SourceHeatBalanceEmsCallingPoint::BeginNewEnvironment,
            any_ran,
        )
        .map_err(hb_input_error)
    }
    fn display_simulation_days_progress(&mut self) -> Result<(), RuntimeError> {
        self.weather
            .state
            .display_source_primary_simulation_days(self.globals()?, self.system()?)
            .map_err(hb_input_error)
    }
    fn reset_accumulation_when_warmup_complete(&mut self) -> Result<(), RuntimeError> {
        self.reports
            .output_registration
            .reset_source_accumulation_when_warmup_complete()
            .map_err(hb_input_error)
    }
    fn write_eio(&mut self, bytes: &[u8]) -> Result<(), RuntimeError> {
        self.files
            .eio_stream()
            .write_all(bytes)
            .map_err(|source| hb_input_error(Error::Io(source)))
    }
    fn reset_tabular_reports(&mut self) -> Result<(), RuntimeError> {
        Err(unavailable(
            "actual multiyear primary ResetTabularReports482",
        ))
    }
    fn simulate_ground_domains(&mut self, _initialization: bool) -> Result<(), RuntimeError> {
        Err(unavailable(
            "actual positive primary SimulateGroundDomains500",
        ))
    }
    fn update_underwater_boundaries(&mut self) -> Result<(), RuntimeError> {
        self.weather
            .state
            .update_source_underwater_boundaries()
            .map_err(hb_input_error)
    }
    fn varying_orientation_schedule(&self) -> Result<&Option<SourceScheduleIndex>, RuntimeError> {
        Ok(&self.weather.state.environment.varying_orientation_schedule)
    }
    fn update_location_and_orientation(&mut self) -> Result<(), RuntimeError> {
        Err(unavailable(
            "actual positive primary UpdateLocationAndOrientation509",
        ))
    }
    fn external_interface_exchange_variables(&mut self) -> Result<(), RuntimeError> {
        let input = self
            .reports
            .selected_input_processor
            .as_ref()
            .ok_or_else(|| {
                unavailable("actual same IP before ExternalInterfaceExchangeVariables")
            })?;
        self.reports
            .source_external_interface
            .exchange_source_variables(
                input,
                &mut self.reports.input_processor_shortcut,
                self.backend,
            )
            .map_err(hb_input_error)
    }
    fn manage_weather_exterior_heat_balance(&mut self) -> Result<(), RuntimeError> {
        self.files.source_ordinary_step_on_session(
            self.session,
            self.reports,
            self.weather,
            self.model,
            self.reporting,
            self.output_host,
            self.backend,
            self.shading_path,
        )
    }
    fn check_if_any_underwater_boundaries(&mut self) -> Result<bool, RuntimeError> {
        let input = self
            .reports
            .selected_input_processor
            .as_ref()
            .ok_or_else(|| unavailable("actual same IP before CheckIfAnyUnderwaterBoundaries"))?;
        self.weather
            .state
            .check_source_underwater_boundaries(input, &mut self.reports.input_processor_shortcut)
            .map_err(hb_input_error)
    }
}
impl SourceInitializationFiles {
    /// Enter the actual368 primary parent with the original180 caller locals.
    /// The caller must have completed genuine274–367; no readiness is fabricated.
    ///
    /// # Errors
    /// Retains the real state and file prefixes on each child error. Positive
    /// SQL/ground/location/multiyear children remain explicit Source boundaries.
    /// Live diagnostic flag ownership and whole-run PSY scope remain caller work.
    pub fn source_primary_simulation(
        &mut self,
        session: &mut Option<OrdinarySourceSession>,
        reports: &mut CtfInitializationReports,
        weather: &ProductionWeatherTimestepSeries,
        model: &TypedModel,
        reporting: &HeatBalanceOrdinaryWarmupReportInputs,
        output_host: &mut SourceMtdFileHost,
        backend: &mut SourceErrorBackend<'_>,
        shading_path: &Path,
        locals: &mut SourceSimulationLoopLocals,
    ) -> Result<(), RuntimeError> {
        weather.with_source_weather_session_mut(|weather| {
            self.source_primary_simulation_on_session(
                session,
                reports,
                weather,
                model,
                reporting,
                output_host,
                backend,
                shading_path,
                locals,
            )
        })
    }
    pub(crate) fn source_primary_simulation_on_session(
        &mut self,
        session: &mut Option<OrdinarySourceSession>,
        reports: &mut CtfInitializationReports,
        weather: &mut WeatherSession,
        model: &TypedModel,
        reporting: &HeatBalanceOrdinaryWarmupReportInputs,
        output_host: &mut SourceMtdFileHost,
        backend: &mut SourceErrorBackend<'_>,
        shading_path: &Path,
        locals: &mut SourceSimulationLoopLocals,
    ) -> Result<(), RuntimeError> {
        let (one_time_check, any_underwater) = locals.underwater_borrows();
        simulate_source_primary(
            &mut PrimaryHost {
                files: self,
                session,
                reports,
                weather,
                model,
                reporting,
                output_host,
                backend,
                shading_path,
            },
            one_time_check,
            any_underwater,
        )
    }
}
