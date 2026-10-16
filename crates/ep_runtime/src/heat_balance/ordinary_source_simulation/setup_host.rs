//! Concrete SetupSimulation children over the same retained run/file owners.
use super::setup::{SourceSetupSimulationChildren, setup_source_simulation};
use crate::RuntimeError;
use crate::heat_balance::ctf_initialization_context::{CtfInitializationReports, hb_input_error};
use crate::heat_balance::error_backend::{SourceDiagnosticFlags, SourceErrorBackend};
use crate::heat_balance::ordinary_source_session::OrdinarySourceSession;
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use crate::heat_balance::selected_project_data::ProjectSystemVariables;
use crate::heat_balance::source_initialization::SourceInitializationFiles;
use crate::heat_balance::source_output_registration::SourceMtdFileHost;
use crate::heat_balance::state::HeatBalanceOrdinaryWarmupReportInputs;
use crate::weather::day::{ProductionWeatherTimestepSeries, WeatherDayState, WeatherSession};
use ep_model::TypedModel;
use std::convert::Infallible;
use std::path::Path;

struct SetupHost<'a, 'backend> {
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
impl SourceSetupSimulationChildren for SetupHost<'_, '_> {
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
            .ok_or_else(|| unavailable("actual DataGlobal owner before SetupSimulation"))
    }
    fn system(&self) -> Result<&ProjectSystemVariables, RuntimeError> {
        self.reports
            .selected_project_data
            .as_ref()
            .map(|project| &project.system)
            .ok_or_else(|| unavailable("actual SysVars owner before SetupSimulation"))
    }
    fn errors_found(&self) -> bool {
        self.weather.errors_found
    }
    fn get_next_environment(&mut self, available: &mut bool) -> Result<(), RuntimeError> {
        self.files
            .source_next_environment_on_session(
                self.session,
                self.reports,
                self.weather,
                self.model,
                self.reporting,
                self.output_host,
                self.backend,
                available,
            )
            .map(|_| ())
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
    fn simulate_ground_domains(&mut self, _initialization: bool) -> Result<(), RuntimeError> {
        // Reached only under the parent's actual Slab/Basement flag guard.
        // Neither a zero count nor a manufactured successful child is returned.
        Err(unavailable(
            "actual positive Setup SimulateGroundDomains2055 child",
        ))
    }
    fn simulate_cost_estimate(&mut self) -> Result<(), RuntimeError> {
        let input = self
            .reports
            .selected_input_processor
            .as_ref()
            .ok_or_else(|| unavailable("actual sole IP before Setup SimCostEstimate2059"))?;
        let globals = self
            .reports
            .selected_heat_balance_globals
            .as_ref()
            .ok_or_else(|| unavailable("actual DataGlobal before Setup SimCostEstimate2059"))?;
        self.reports
            .cost_estimate_monetary
            .simulate_source_cost_estimate(input, globals.kickoff_simulation)
            .map_err(hb_input_error)
    }
    fn show_fatal_error(&mut self, message: &str) -> Result<Infallible, RuntimeError> {
        self.backend
            .show_fatal_error(message)
            .map_err(|stop| hb_input_error(Error::Fatal(stop)))
    }
}
impl SourceInitializationFiles {
    /// Actual SimulationManager268/270/272 prefix after Initialize Simulation267.
    /// # Errors
    /// Actual Source child failures retain the same persistent owners and files.
    /// This returns after SetupSimulation, before the unported274 postlude.
    pub fn source_kickoff_setup_simulation(
        &mut self,
        session: &mut Option<OrdinarySourceSession>,
        reports: &mut CtfInitializationReports,
        weather: &ProductionWeatherTimestepSeries,
        model: &TypedModel,
        reporting: &HeatBalanceOrdinaryWarmupReportInputs,
        output_host: &mut SourceMtdFileHost,
        backend: &mut SourceErrorBackend<'_>,
        shading_path: &Path,
    ) -> Result<(), RuntimeError> {
        reports
            .selected_heat_balance_globals
            .as_mut()
            .ok_or_else(|| unavailable("actual DataGlobal before KickOffSimulation268"))?
            .kickoff_simulation = true;
        weather.with_source_weather_session_mut(|weather| {
            weather.state.weather.envrn = 0; // ResetEnvironmentCounter270.
            self.source_setup_simulation_on_session(
                session,
                reports,
                weather,
                model,
                reporting,
                output_host,
                backend,
                shading_path,
            )
        })
    }
    pub(crate) fn source_setup_simulation_on_session(
        &mut self,
        session: &mut Option<OrdinarySourceSession>,
        reports: &mut CtfInitializationReports,
        weather: &mut WeatherSession,
        model: &TypedModel,
        reporting: &HeatBalanceOrdinaryWarmupReportInputs,
        output_host: &mut SourceMtdFileHost,
        backend: &mut SourceErrorBackend<'_>,
        shading_path: &Path,
    ) -> Result<(), RuntimeError> {
        setup_source_simulation(&mut SetupHost {
            files: self,
            session,
            reports,
            weather,
            model,
            reporting,
            output_host,
            backend,
            shading_path,
        })
    }
}
