//! Actual RunSimulation274-281 after SetupSimulation returns successfully.
mod faults;
use super::{CtfInitializationReports, SourceInitializationError, SourceInitializationFiles};
use crate::RuntimeError;
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::weather::day::{ProductionWeatherTimestepSeries, WeatherSession};
pub(crate) use faults::SourceFaultsManager;

fn missing(owner: &'static str) -> RuntimeError {
    SourceInitializationError::Association(owner).into()
}
impl SourceInitializationFiles {
    /// Run genuine fault and curve postludes, then the actual simulation flags.
    /// # Errors
    /// Preserves original typed causes, reached owners and flag-write order.
    /// The actual caller must first complete SetupSimulation272 successfully.
    pub fn finish_source_kickoff(
        &mut self,
        reports: &mut CtfInitializationReports,
        weather: &ProductionWeatherTimestepSeries,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), RuntimeError> {
        weather.with_source_weather_session_mut(|actual| {
            self.finish_source_kickoff_on_session(reports, actual, backend)
        })
    }
    pub(crate) fn finish_source_kickoff_on_session(
        &mut self,
        reports: &mut CtfInitializationReports,
        weather: &mut WeatherSession,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), RuntimeError> {
        let input = reports
            .selected_input_processor
            .as_ref()
            .ok_or_else(|| missing("same InputProcessor before CheckAndReadFaults274"))?;
        reports
            .source_faults_manager
            .check_and_read_source_faults(input)
            .map_err(|source| SourceInitializationError::Handler {
                handler: "FaultsManager::CheckAndReadFaults274",
                source: Box::new(source),
            })?;
        let curves = reports
            .selected_curves
            .as_ref()
            .ok_or_else(|| missing("same CurveManager before InitCurveReporting276"))?;
        let globals = reports
            .selected_heat_balance_globals
            .as_mut()
            .ok_or_else(|| missing("same DataGlobal before InitCurveReporting276"))?;
        curves
            .initialize_source_curve_reporting(globals)
            .map_err(|source| SourceInitializationError::Handler {
                handler: "Curve::InitCurveReporting276",
                source: Box::new(source),
            })?;
        backend.state.tracking.ask_for_connections_report = true; // Actual278.
        globals.kickoff_simulation = false; // Actual279.
        weather.state.global.warmup_flag = false; // Actual280.
        reports.source_report_flags.do_weather_init_reporting = true; // Actual281.
        // The legacy backend's kickoff/warmup carriers are not synchronized here.
        Ok(())
    }
}
