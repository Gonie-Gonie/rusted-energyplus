//! Live weather-file cursor, daily transport and production weather operands.
//!
//! The raw cursor and complete daily copy have separate owners. Selected hourly
//! processing, default ClarkAllen sky, solar interpolation and current weather
//! use source-owned state. Alternative sky and current-solar models are separate.

mod configuration;
mod current;
mod error;
pub mod handoff;
mod hourly;
mod lifecycle;
mod producer;
mod production;
pub mod production_trace;
mod sky;
pub mod sky_transport_trace;
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
pub use sky::{WeatherSky, default_clark_allen_sky_emissivity, default_weather_file_sky};
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
