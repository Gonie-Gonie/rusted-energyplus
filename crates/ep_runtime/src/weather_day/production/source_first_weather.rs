//! New Source-only production attachment; the legacy eager adapter is preserved.
use super::ProductionWeatherTimestepSeries;
use crate::weather::day::SourceWeatherInitializationError;
use crate::weather::day::source_initialization::{
    SourceWeatherInitializationChildren, SourceWeatherInitializationOwners,
};
use crate::weather::day::{
    WeatherDayError, WeatherDayState, WeatherEnvironmentConfiguration, WeatherSession,
};
impl ProductionWeatherTimestepSeries {
    /// Move genuine Project owners into an unopened weather session.
    /// # Errors
    /// Returns the actual prepared Project/timestep association boundary.
    pub fn from_unopened_source_project_state(
        bytes: Vec<u8>,
        configuration: WeatherEnvironmentConfiguration,
        state: &mut WeatherDayState,
    ) -> Result<Self, WeatherDayError> {
        Ok(Self::from_session(
            WeatherSession::from_source_project_state(bytes, configuration, state)?,
        ))
    }
    #[allow(
        dead_code,
        reason = "Retain the original GetNextEnvironment and live-weather-owner borrowing entries; selected frontend code uses the corresponding output-aware/session interfaces."
    )]
    pub(crate) fn get_next_source_environment(
        &self,
        owners: &mut SourceWeatherInitializationOwners<'_, '_>,
        children: &mut impl SourceWeatherInitializationChildren,
        available: &mut bool,
    ) -> Result<bool, SourceWeatherInitializationError> {
        self.session
            .borrow_mut()
            .get_next_source_environment(owners, children, available)
    }
    /// Reborrow the sole actual weather global/environment/WeatherManager owners
    /// for one caller, with the session guard held until all operations return.
    /// No copied clock, weather snapshot or live references escape this scope.
    #[allow(
        dead_code,
        reason = "Retain the original GetNextEnvironment and live-weather-owner borrowing entries; selected frontend code uses the corresponding output-aware/session interfaces."
    )]
    pub(crate) fn with_source_weather_owners_mut<T>(
        &self,
        consume: impl FnOnce(
            &mut super::super::WeatherGlobalState,
            &mut super::super::WeatherEnvironmentState,
            &super::super::WeatherOwnerState,
        ) -> T,
    ) -> T {
        let mut session = self.session.borrow_mut();
        let super::super::WeatherDayState {
            global,
            environment,
            weather,
            ..
        } = &mut session.state;
        consume(global, environment, weather)
    }
}

impl ProductionWeatherTimestepSeries {
    /// One guard over the same actual session for genuine whole callers only.
    /// No snapshots, clock updates, legacy lifecycle or escaped live references.
    pub(crate) fn with_source_weather_session_mut<T>(
        &self,
        consume: impl FnOnce(&mut WeatherSession) -> T,
    ) -> T {
        let mut session = self.session.borrow_mut();
        consume(&mut session)
    }
}
