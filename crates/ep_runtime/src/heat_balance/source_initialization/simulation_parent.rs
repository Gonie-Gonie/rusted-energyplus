//! Genuine SimulationManager201-204, reached only after actual PostIP caller.
use super::{CtfInitializationReports, SourceInitializationError};
impl CtfInitializationReports {
    /// Set the actual simulation flags after successful PostIPProcessing199.
    /// The parent supplies source order; no extra readiness flag or state copy.
    ///
    /// # Errors
    /// Returns the real missing constructor owner boundary before any flag write.
    pub fn begin_source_simulation_after_post_input_processing(
        &mut self,
    ) -> Result<(), SourceInitializationError> {
        let weather = self.source_initialization_weather.as_mut().ok_or(
            SourceInitializationError::Association("actual pre-weather DataGlobal owner"),
        )?;
        weather.global.begin_sim_flag = true;
        weather.global.do_output_reporting = false;
        self.source_report_flags.display_performance_simulation_flag = false;
        self.source_report_flags.do_weather_init_reporting = false;
        Ok(())
    }
}
