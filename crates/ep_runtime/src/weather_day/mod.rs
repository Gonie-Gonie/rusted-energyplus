//! Live weather-file cursor, daily transport and production weather operands.
//!
//! The raw cursor and complete daily copy have separate owners. Selected hourly
//! processing, default ClarkAllen sky, solar interpolation and current weather
//! use source-owned state. Alternative sky and current-solar models are separate.

mod auxiliary_input;
mod configuration;
mod current;
mod daylighting;
mod design_day;
mod error;
pub(crate) mod ground_temperature;
pub mod handoff;
mod hourly;
mod lifecycle;
mod location;
#[cfg(all(test, feature = "native-controlled-air-cohort"))]
mod native_gap_cohort;
pub(crate) mod output_bindings;
mod output_headers;
pub(crate) mod output_registration;
pub(crate) mod source_current_weather;
mod source_environment_data;
mod source_environment_types;
pub use source_run_period::SourceRunPeriodData;
pub use source_user_input::SourceWeatherProperty;
pub(crate) mod source_design_day;
pub(crate) mod source_initialization;
mod source_initialization_error;
pub(crate) mod source_next_environment;
mod source_open;
mod source_primary;
pub(crate) mod source_run_period;
mod source_user_input;
pub(crate) mod source_weather_initialization;
pub use source_environment_data::SourceEnvironmentData;
pub(crate) use source_initialization_error::SourceWeatherInitializationError;
mod producer;
mod production;
pub mod production_trace;
mod sky;
pub mod sky_transport_trace;
mod solar;
mod standard_conditions;
pub mod state;

pub use configuration::WeatherSolarControls;
pub use current::CurrentWeatherState;
pub use design_day::{DesignDayHumidity, DesignDayInput, DesignDayParameters};
pub use error::WeatherDayError;
pub use handoff::update_weather_data;
pub use lifecycle::{WeatherEnvironmentConfiguration, WeatherSession};
pub use production::{
    ProductionCtfHistoryClock, ProductionSolarMetadata, ProductionWarmupReportContext,
    ProductionWeatherContext, ProductionWeatherTimestepSeries, SetupSimulationCall,
    SetupSimulationControls, SetupSimulationRole, WeatherDayPhase,
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
