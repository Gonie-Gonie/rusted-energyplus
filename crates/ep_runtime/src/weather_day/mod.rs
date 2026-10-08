//! Live weather-file cursor, daily transport and production weather operands.
//!
//! The raw cursor and complete daily copy have separate owners. Existing weather
//! Selected non-solar hourly processing and current weather use source-owned
//! state. Sky and solar generation retain separate compatibility boundaries.

mod configuration;
mod current;
mod error;
pub mod handoff;
mod hourly;
mod lifecycle;
mod producer;
mod production;
pub mod production_trace;
mod solar;
pub mod state;

pub use configuration::WeatherSolarControls;
pub use current::CurrentWeatherState;
pub use error::WeatherDayError;
pub use handoff::update_weather_data;
pub use lifecycle::{WeatherEnvironmentConfiguration, WeatherSession};
pub use production::{
    ProductionSolarMetadata, ProductionWeatherContext, ProductionWeatherTimestepSeries,
    WeatherDayPhase,
};
pub use state::{
    DailyWeatherVariables, ExtendedWeatherVars, WeatherDayState, WeatherDayValues,
    WeatherDayValuesError, WeatherEnvironmentState, WeatherGlobalState, WeatherOwnerState,
    WeatherVarCounts, WeatherVars,
};

#[cfg(test)]
mod current_tests;
#[cfg(test)]
pub(crate) mod lifecycle_tests;
#[cfg(test)]
mod tests;
