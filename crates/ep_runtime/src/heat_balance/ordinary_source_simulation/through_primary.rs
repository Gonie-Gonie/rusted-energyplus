//! Same-owner RunSimulation268-564 continuation, ending before final reports566.
use super::SourceSimulationLoopLocals;
use crate::RuntimeError;
use crate::heat_balance::ctf_initialization_context::CtfInitializationReports;
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::ordinary_source_session::OrdinarySourceSession;
use crate::heat_balance::source_initialization::{
    SourceInitializationError, SourceInitializationFiles,
};
use crate::heat_balance::source_output_registration::SourceMtdFileHost;
use crate::heat_balance::state::HeatBalanceOrdinaryWarmupReportInputs;
use crate::weather::day::ProductionWeatherTimestepSeries;
use ep_model::TypedModel;
use std::path::Path;

impl SourceInitializationFiles {
    /// Continue after the genuine initialization267 through the primary loop564.
    /// Uses one mutable borrow of the retained weather session for all phases.
    ///
    /// The caller must retain the single outer psychrometric scope established
    /// before input processing. This function neither creates nor resets one.
    /// All native180 locals, Source owners, file handles and diagnostics must be
    /// the original run's borrows. Final processing566-620 is not performed here.
    ///
    /// # Errors
    /// Returns the unchanged typed child cause and retains every preceding state
    /// and file effect; no flags or partial owners are restored after a failure.
    pub fn source_simulation_through_primary(
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
        reports
            .selected_heat_balance_globals
            .as_mut()
            .ok_or(SourceInitializationError::Association(
                "actual DataGlobal before continuation KickOffSimulation268",
            ))?
            .kickoff_simulation = true;
        weather.with_source_weather_session_mut(|weather| {
            weather.state.weather.envrn = 0; // Actual ResetEnvironmentCounter270.
            self.source_setup_simulation_on_session(
                session,
                reports,
                weather,
                model,
                reporting,
                output_host,
                backend,
                shading_path,
            )?;
            self.finish_source_kickoff_on_session(reports, weather, backend)?;
            self.source_primary_startup_on_session(
                session,
                reports,
                weather,
                model,
                output_host,
                backend,
            )?;
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
}
