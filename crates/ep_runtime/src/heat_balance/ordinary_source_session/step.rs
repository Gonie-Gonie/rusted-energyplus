//! Actual first-input then body entry over the retained ordinary output files.
use super::OrdinarySourceSession;
use crate::RuntimeError;
use crate::heat_balance::ctf_initialization_context::{
    CtfInitializationContext, CtfInitializationReports, hb_input_error,
};
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use crate::heat_balance::source_initialization::SourceInitializationFiles;
use crate::heat_balance::source_output_registration::SourceMtdFileHost;
use crate::heat_balance::state::HeatBalanceOrdinaryWarmupReportInputs;
use crate::heat_balance::warmup::report::WarmupReportConfiguration;
use crate::weather::day::WeatherDayState;
use ep_model::TypedModel;
use std::path::Path;

impl SourceInitializationFiles {
    /// Execute genuine ManageHeatBalance on one persistent Source session.
    ///
    /// The actual simulation caller must first perform Weather/Exterior calls
    /// and retain a single whole-run psychrometric scope. This entry does not
    /// synthesize clocks, weather, sizing controls, history or numerical output.
    ///
    /// # Errors
    /// Original child/backend errors preserve their same owned typed causes and
    /// partial persistent owners; no legacy simulator fallback is invoked.
    // Keep the explicit dereference/reborrow at the persistent same-owner call boundary.
    #[allow(clippy::explicit_auto_deref)]
    pub fn source_heat_balance_step(
        &mut self,
        session: &mut Option<OrdinarySourceSession>,
        reports: &mut CtfInitializationReports,
        weather: &mut WeatherDayState,
        model: &TypedModel,
        reporting: &HeatBalanceOrdinaryWarmupReportInputs,
        output_host: &mut SourceMtdFileHost,
        mut backend: SourceErrorBackend<'_>,
        shading_path: &Path,
    ) -> Result<(), RuntimeError> {
        let equipment = reporting.equipment_input.ok_or_else(|| {
            hb_input_error(Error::Unavailable(
                "actual admitted equipment input before Source heat balance",
            ))
        })?;
        let radiation = reporting.radiation_input.ok_or_else(|| {
            hb_input_error(Error::Unavailable(
                "actual admitted radiation input before Source heat balance",
            ))
        })?;
        self.ensure_source_session(session, reports, weather, model, reporting)?;
        let session = session.as_mut().ok_or_else(|| {
            hb_input_error(Error::Unavailable(
                "retained actual Source heat balance session",
            ))
        })?;
        // EIO is borrowed only for the real first-call prefix. Releasing the
        // context lets the next phase reborrow the SAME EIO plus ESO/MTR/RF,
        // without retaining a whole reports/file borrow across both phases.
        backend.with_scoped(
            &mut (&mut *reports, &mut *self),
            |call_backend, (call_reports, call_files)| {
                let (eio, debug) = call_files.eio_and_debug_streams();
                let mut context = CtfInitializationContext {
                    reports: &mut **call_reports,
                    eio,
                    backend: call_backend,
                };
                session.prepare_first_input(
                    &mut context,
                    model,
                    &mut weather.environment,
                    weather.global.minutes_in_time_step,
                    output_host,
                    equipment,
                    radiation,
                    debug,
                )
            },
        )?;
        session
            .run_heat_balance_body(
                reports,
                weather,
                model,
                output_host,
                self,
                &mut backend,
                shading_path,
            )
            .map_err(hb_input_error)
    }

    /// Only genuine retained module constructors; no GetHBInput or weather call.
    pub(super) fn ensure_source_session(
        &mut self,
        session: &mut Option<OrdinarySourceSession>,
        reports: &CtfInitializationReports,
        weather: &WeatherDayState,
        model: &TypedModel,
        reporting: &HeatBalanceOrdinaryWarmupReportInputs,
    ) -> Result<(), RuntimeError> {
        if session.is_none() {
            let globals = reports
                .selected_heat_balance_globals
                .as_ref()
                .ok_or_else(|| {
                    hb_input_error(Error::Unavailable(
                        "actual startup globals before Source session constructor",
                    ))
                })?;
            // Names/configuration are report metadata only. Actual Source
            // branches/values read the live Zone/project/global/weather owners.
            *session = Some(
                OrdinarySourceSession::new(
                    model.zones.iter().map(|zone| zone.name.0.clone()).collect(),
                    WarmupReportConfiguration {
                        doing_sizing: globals.doing_sizing,
                        do_pure_load_calc: reporting.do_pure_load_calc,
                        run_period_environment: weather.environment.run_period_environment,
                        environment_name: weather.environment.environment_name.clone(),
                        detailed: reporting.detailed,
                    },
                )
                .map_err(hb_input_error)?,
            );
        }
        Ok(())
    }
    /// Same actual three-call timestep used by SetupSimulation and primary run.
    /// Only constructors precede Weather. Real first input remains inside HB.
    pub(crate) fn source_ordinary_step_on_session(
        &mut self,
        session: &mut Option<OrdinarySourceSession>,
        reports: &mut CtfInitializationReports,
        weather: &mut crate::weather::day::WeatherSession,
        model: &TypedModel,
        reporting: &HeatBalanceOrdinaryWarmupReportInputs,
        output_host: &mut SourceMtdFileHost,
        backend: &mut SourceErrorBackend<'_>,
        shading_path: &Path,
    ) -> Result<(), RuntimeError> {
        self.ensure_source_session(session, reports, &weather.state, model, reporting)?;
        let retained = session.as_mut().ok_or_else(|| {
            hb_input_error(Error::Unavailable(
                "actual retained Source session before Weather",
            ))
        })?;
        retained.run_source_weather(reports, self, weather, backend)?;
        retained.run_source_exterior(reports, self, weather, model, output_host, backend)?;
        backend.with_scoped(
            &mut (session, reports, self, &mut weather.state),
            |call_backend, (call_session, call_reports, call_files, call_weather)| {
                call_files.source_heat_balance_step(
                    call_session,
                    call_reports,
                    call_weather,
                    model,
                    reporting,
                    output_host,
                    call_backend,
                    shading_path,
                )
            },
        )
    }
    /// Reborrow the one production weather session once for the actual sequence.
    /// No legacy advance/current method, phase inference or extra weather owner.
    /// # Errors
    /// Retains actual typed causes and partial owners at first failed real child.
    pub fn source_ordinary_timestep(
        &mut self,
        session: &mut Option<OrdinarySourceSession>,
        reports: &mut CtfInitializationReports,
        weather: &crate::weather::day::ProductionWeatherTimestepSeries,
        model: &TypedModel,
        reporting: &HeatBalanceOrdinaryWarmupReportInputs,
        output_host: &mut SourceMtdFileHost,
        backend: &mut SourceErrorBackend<'_>,
        shading_path: &Path,
    ) -> Result<(), RuntimeError> {
        weather.with_source_weather_session_mut(|actual| {
            self.source_ordinary_step_on_session(
                session,
                reports,
                actual,
                model,
                reporting,
                output_host,
                backend,
                shading_path,
            )
        })
    }
}
