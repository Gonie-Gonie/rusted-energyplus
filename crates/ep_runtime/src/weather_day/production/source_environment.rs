//! Selected real station/site handoff to the same live weather environment.
//! The existing constructor-eager weather subsystem is not a full input getter.
use super::ProductionWeatherTimestepSeries;
use crate::RuntimeError;
use crate::error::WarmupOutputError;
use crate::heat_balance::convection::surface_environment::SurfaceOutdoorInitializationError;
use crate::heat_balance::ctf_initialization_context::{
    CtfInitializationContext, CtfInitializationError,
};
use crate::weather::day::{WeatherDayError, WeatherDayPhase, WeatherEnvironmentState};

fn unavailable(reason: &'static str) -> RuntimeError {
    WeatherDayError::admission(reason).into()
}

fn output_error(error: SurfaceOutdoorInitializationError) -> RuntimeError {
    let output = match error {
        SurfaceOutdoorInitializationError::Io(error) => WarmupOutputError::Io(error),
        SurfaceOutdoorInitializationError::Format(reason) => WarmupOutputError::Format(reason),
    };
    RuntimeError::WarmupOutput(Box::new(output))
}

impl ProductionWeatherTimestepSeries {
    // Selected GetDesignDayData report suffix precedes GetWeatherStation4828,
    // which precedes GetNextEnvironment close/increment774. This reuses the
    // real input/EIO owners; it does not certify the omitted weather getters.
    fn initialize_source_station_handoff(
        &self,
        context: &mut CtfInitializationContext<'_>,
    ) -> Result<(), RuntimeError> {
        let mut session = self.session.borrow_mut();
        if session.state.environment.source_weather_station_initialized {
            if !self.setup_design_day_reports_registered.get() {
                return Err(unavailable(
                    "station return without the reached weather report suffix",
                ));
            }
            return Ok(());
        }
        if self.setup_design_day_reports_registered.get() {
            return Err(unavailable(
                "compatibility weather input was already consumed without the source EIO handoff",
            ));
        }
        session
            .configuration
            .register_design_day_predefined_reports(context.reports)
            .map_err(CtfInitializationError::Owner)?;
        // Actual ReadUserWeatherInput4820 precedes Station4828. The earlier
        // raw cursor already processed actual EPW headers at GetNextEnvironment710.
        session
            .initialize_source_ground_temperatures(
                context
                    .reports
                    .heat_balance_input
                    .as_ref()
                    .ok_or_else(|| unavailable("actual ground-model raw input"))?,
                context
                    .reports
                    .selected_input_processor
                    .as_mut()
                    .ok_or_else(|| unavailable("actual ground-model InputProcessor"))?,
                &mut context.reports.input_processor_shortcut,
                &mut context.backend,
                context.eio,
            )
            .map_err(|error| RuntimeError::HeatBalanceInput(Box::new(error)))?;
        // Actual4822/4824/4826, borrowing the SAME partial-reader local and
        // actual IP/shortcut/backend/EIO; full ReadUserWeatherInput is separate.
        let mut errors_found = false;
        session
            .initialize_source_weather_auxiliary_inputs(
                context
                    .reports
                    .selected_input_processor
                    .as_ref()
                    .ok_or_else(|| unavailable("actual auxiliary weather InputProcessor"))?,
                &mut context.reports.input_processor_shortcut,
                &mut context.backend,
                context.eio,
                &mut errors_found,
            )
            .map_err(|error| RuntimeError::HeatBalanceInput(Box::new(error)))?;
        context
            .reports
            .input_processor_shortcut
            .current_module_object = "Site:WeatherStation".into();
        let input = context
            .reports
            .selected_input_processor
            .as_ref()
            .ok_or_else(|| {
                unavailable("weather station requires the sole actual input processor")
            })?;
        if input.get_num_objects_found("Site:WeatherStation") != 0 {
            return Err(unavailable(
                "selected weather station requires the actual zero-object branch",
            ));
        }
        session
            .state
            .environment
            .outdoor_environment_parameters
            .initialize_absent_weather_station(context.eio)
            .map_err(output_error)?;
        if errors_found {
            // Keep all actual severe/EIO/Station prefix effects. The omitted
            // SetupEnvironmentTypes/GetWeatherProperties/fatal suffix must
            // consume this real accumulated error before any successful return.
            return Err(unavailable(
                "actual ReadUserWeatherInput accumulated ErrorsFound continuation",
            ));
        }
        // These are Rust return witnesses, never substitutes for Source flags.
        // Assignment follows the real coefficient/header/row operation return.
        session.state.environment.source_weather_station_initialized = true;
        self.setup_design_day_reports_registered.set(true);
        Ok(())
    }

    /// Advance the selected Setup getter using the actual sole IP and EIO owners.
    /// This adds the real absent-station handoff before the existing getter close.
    ///
    /// # Errors
    /// Rejects inactive/reentered traversal, unsupported station input and actual
    /// report/format/write errors; every reached owner and EIO prefix is retained.
    pub fn next_setup_environment_with_source_context(
        &self,
        context: &mut CtfInitializationContext<'_>,
    ) -> Result<(bool, bool), RuntimeError> {
        if !self.setup_started.get() || self.setup_completed.get() {
            return Err(unavailable(
                "SetupSimulation environment traversal is not active",
            ));
        }
        self.initialize_source_station_handoff(context)?;
        self.next_setup_environment(context.reports)
    }

    /// Prepare the ordinary phase with the same real first-input station handoff.
    /// No Setup role, ordinal or flag is fabricated for this RunPeriod route.
    ///
    /// # Errors
    /// Returns actual input/EIO/weather errors, or a duplicate/invalid phase.
    /// The existing constructor-eager allocation/location limitation is unchanged.
    pub fn prepare_initial_phase_with_source_context(
        &self,
        phase: WeatherDayPhase,
        context: &mut CtfInitializationContext<'_>,
    ) -> Result<(), RuntimeError> {
        if self.current_phase.borrow().is_some() {
            return Err(unavailable("initial weather phase already prepared"));
        }
        self.admit_phase(phase)?;
        self.initialize_source_station_handoff(context)?;
        self.prepare_initial_phase(phase).map_err(Into::into)
    }

    /// Borrow the same live environment and its actual timestep minutes once.
    /// The callback completes before any subsequent weather advance; no cursor
    /// guard, cloned environment or second weather state escapes this boundary.
    pub(crate) fn with_source_environment_mut<T>(
        &self,
        consume: impl FnOnce(&mut WeatherEnvironmentState, i32) -> T,
    ) -> T {
        let mut session = self.session.borrow_mut();
        let minutes = session.state.global.minutes_in_time_step;
        consume(&mut session.state.environment, minutes)
    }
}

impl CtfInitializationContext<'_> {
    /// Genuine selected GetSiteAtmosphereData264, after ProjectControl terrain.
    /// Uses the already reached station and sole HBM string/IP/EIO owners.
    pub(crate) fn initialize_source_site_atmosphere(
        &mut self,
        environment: &mut WeatherEnvironmentState,
    ) -> Result<(), RuntimeError> {
        if !environment.source_weather_station_initialized
            || environment.source_site_atmosphere_initialized
        {
            return Err(unavailable("actual station/site getter call order"));
        }
        self.reports.heat_balance_current_module_object = "Site:HeightVariation".into();
        let input = self
            .reports
            .selected_input_processor
            .as_ref()
            .ok_or_else(|| {
                unavailable("site atmosphere requires the sole actual input processor")
            })?;
        if input.get_num_objects_found("Site:HeightVariation") != 0 {
            return Err(unavailable(
                "selected site atmosphere requires the actual zero-object branch",
            ));
        }
        // Only SiteTempGradient is assigned by the zero-object getter. The
        // actual preceding Building terrain wind operands remain untouched.
        environment
            .outdoor_environment_parameters
            .initialize_absent_site_height_variation(self.eio)
            .map_err(output_error)?;
        environment.source_site_atmosphere_initialized = true;
        Ok(())
    }
}
