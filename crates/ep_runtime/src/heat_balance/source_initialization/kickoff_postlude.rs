//! Actual RunSimulation274-281 after SetupSimulation returns successfully.
mod faults;
use super::{CtfInitializationReports, SourceInitializationError, SourceInitializationFiles};
use crate::RuntimeError;
use crate::heat_balance::error_backend::SourceDiagnosticFlags;
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::setup_postludes::SourceCurveReportContext;
use crate::heat_balance::source_output_registration::{SourceMtdFileHost, SourceOutputFileHost};
use crate::weather::day::{ProductionWeatherTimestepSeries, WeatherSession};
use ep_model::ReportInputs;
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
        report_inputs: &ReportInputs,
        output_host: &mut SourceMtdFileHost,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), RuntimeError> {
        weather.with_source_weather_session_mut(|actual| {
            self.finish_source_kickoff_on_session(
                reports,
                actual,
                report_inputs,
                output_host,
                backend,
            )
        })
    }
    pub(crate) fn finish_source_kickoff_on_session<H: SourceOutputFileHost>(
        &mut self,
        reports: &mut CtfInitializationReports,
        weather: &mut WeatherSession,
        report_inputs: &ReportInputs,
        output_host: &mut H,
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
        let project = reports.selected_project_data.as_ref();
        let minimum_report_frequency =
            project.map(|owner| owner.system.minimum_report_frequency.as_str());
        let diagnostic_flags = project.map(|owner| SourceDiagnosticFlags {
            kickoff: &globals.kickoff_simulation,
            developer: &owner.system.developer_flag,
            warmup: &weather.state.global.warmup_flag,
            doing_sizing: &globals.doing_sizing,
        });
        let (mut dictionary_host, eio, _debug) = self.source_report_streams_eio_debug(
            &reports.source_sqlite,
            &mut reports.source_results_output_variables,
        );
        let mut context = SourceCurveReportContext {
            globals,
            input: reports
                .selected_input_processor
                .as_mut()
                .ok_or_else(|| missing("same InputProcessor before InitCurveReporting276"))?,
            schedules: reports.selected_schedule_manager.as_mut(),
            reports: report_inputs,
            registration: &mut reports.output_registration,
            prescan: &reports.output_prescan,
            minimum_report_frequency,
            minutes_in_time_step: weather.state.global.minutes_in_time_step,
            diagnostic_flags,
            backend,
            host: output_host,
            dictionary_host: &mut dictionary_host,
            eio,
        };
        curves
            .initialize_source_curve_reporting(&mut context)
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
