//! Move the sole actual Project weather owner without legacy eager initialization.
use super::{CtfInitializationReports, SourceInitializationError};
use crate::weather::day::ProductionWeatherTimestepSeries;
impl CtfInitializationReports {
    /// Attach actual bytes to the unopened Source cursor, before genuine GetNext.
    /// # Errors
    /// Missing Project clock/zero timestep retains the same original weather state.
    pub fn initialize_unopened_source_weather(
        &mut self,
        bytes: Vec<u8>,
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
        // Every clock and physical field remains in the same moved state.
        // Genuine Source has no prepared legacy configuration or calendar axis.
        let series =
            ProductionWeatherTimestepSeries::from_unopened_source_project_state(bytes, weather)?;
        self.source_initialization_weather = None;
        Ok(series)
    }
}
