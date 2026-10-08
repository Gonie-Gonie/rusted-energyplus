//! Live weather-file cursor, daily transport and production weather operands.
//!
//! The raw cursor and complete daily copy have separate owners. Existing weather
//! physics remains an explicit compatibility producer until its own source cards.

mod configuration;
mod error;
pub mod handoff;
mod lifecycle;
mod producer;
mod production;
pub mod production_trace;
pub mod state;

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
pub(crate) mod lifecycle_tests;
#[cfg(test)]
mod tests;
