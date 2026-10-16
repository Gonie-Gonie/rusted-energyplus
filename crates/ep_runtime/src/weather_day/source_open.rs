//! WeatherManager::OpenWeatherFile: actual filesystem predicate then physical open.
use super::{SourceWeatherInitializationError as Failure, WeatherSession};
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use crate::heat_balance::selected_input_processor::SelectedInputProcessor;
use crate::heat_balance::source_output_registration::SourceOutputReportStreams;
use std::path::Path;

impl WeatherSession {
    /// Genuine4348–4351, using the retained RunConfig inputWeatherFilePath operand.
    /// FileSystem.cc289–292 performs exists, then is_directory only if present.
    pub(crate) fn open_source_weather_file(
        &mut self,
        path: &Path,
        input: &mut SelectedInputProcessor,
        files: &mut dyn SourceOutputReportStreams,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Failure> {
        let exists = std::fs::exists(path).map_err(Error::Io)?;
        self.state.weather.weather_file_exists = if exists {
            !std::fs::metadata(path).map_err(Error::Io)?.is_dir()
        } else {
            false
        };
        if self.state.weather.weather_file_exists {
            self.cursor.open_source_epw_file(
                path,
                true,
                &mut self.errors_found,
                input,
                files,
                backend,
            )?;
        }
        Ok(())
    }
}
