//! Move the sole actual Project weather owner without legacy eager initialization.
use super::{CtfInitializationReports, SourceInitializationError};
use crate::weather::day::{ProductionWeatherTimestepSeries, WeatherEnvironmentConfiguration};
impl CtfInitializationReports {
    /// Attach actual bytes to the unopened Source cursor, before genuine GetNext.
    /// # Errors
    /// Missing Project clock/zero timestep retains the same original weather state.
    pub fn initialize_unopened_source_weather(
        &mut self,
        bytes: Vec<u8>,
        mut configuration: WeatherEnvironmentConfiguration,
    ) -> Result<ProductionWeatherTimestepSeries, SourceInitializationError> {
        let weather = self.source_initialization_weather.as_mut().ok_or(
            SourceInitializationError::Association("same actual Project weather owner"),
        )?;
        let steps = u32::try_from(weather.global.time_steps_in_hour)
            .map_err(|_| SourceInitializationError::Association("actual Project timestep"))?;
        if steps == 0 {
            return Err(SourceInitializationError::Association(
                "zero actual Project timestep",
            ));
        }
        // Transport metadata only; genuine clock and every physical field remain
        // in the same moved state. No compatibility admission/getter is invoked.
        let profile = crate::time_axis::time_axis_timestep_profile_for_zone_timesteps(steps);
        configuration.time_axis.zone_timestep = profile.zone_timestep;
        configuration.time_axis.system_timestep = profile.system_timestep;
        let series = ProductionWeatherTimestepSeries::from_unopened_source_project_state(
            bytes,
            configuration,
            weather,
        )?;
        self.source_initialization_weather = None;
        Ok(series)
    }
}
