//! Genuine GetDesignDayData5936–6655 on the sole shared input and weather owners.
//! Schedule lookup is an honest first-operation boundary; no typed-input replay.
mod current;
mod diagnostics;
mod getter;
mod humidity;
mod reports;
mod setup;
mod state;
mod validation;
use super::{DailyWeatherVariables, SourceEnvironmentData, WeatherGlobalState};
use crate::heat_balance::ctf_reports::PredefinedStringTable;
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::predefined_tables::PredefinedRegistry;
use crate::heat_balance::selected_heat_balance_input::{
    HeatBalanceInputError as Error, SelectedHeatBalanceInput,
};
use crate::heat_balance::selected_input_processor::{
    SelectedInputProcessor, SourceInputProcessorShortCut,
};
pub(crate) use setup::SourceDesignDaySetupOwners;
pub(crate) use state::{SourceDesignDayData, SourceDesignDayInputState, SourceDesignDayModifiers};

pub(crate) struct SourceDesignDayInputOwners<'a, 'backend> {
    pub(crate) raw: &'a SelectedHeatBalanceInput,
    pub(crate) input: &'a mut SelectedInputProcessor,
    pub(crate) shortcut: &'a mut SourceInputProcessorShortCut,
    pub(crate) clock: &'a WeatherGlobalState,
    pub(crate) force_begin_env_reset_suppress: &'a bool,
    pub(crate) reverse_design_days: &'a bool,
    pub(crate) interpolation: &'a Option<Vec<f64>>,
    pub(crate) environments: &'a mut [SourceEnvironmentData],
    pub(crate) design_day_weather: &'a [DailyWeatherVariables],
    pub(crate) registry: &'a PredefinedRegistry,
    pub(crate) table: &'a mut PredefinedStringTable,
    pub(crate) backend: &'a mut SourceErrorBackend<'backend>,
}
const FAMILY: &str = "SizingPeriod:DesignDay";
const DEFAULT_TEMPERATURE_RANGE_MULTIPLIERS: [f64; 24] = [
    0.88, 0.92, 0.95, 0.98, 1.0, 0.98, 0.91, 0.74, 0.55, 0.38, 0.23, 0.13, 0.05, 0.00, 0.00, 0.06,
    0.14, 0.24, 0.39, 0.50, 0.59, 0.68, 0.75, 0.82,
];
fn size(value: i32) -> Result<usize, Error> {
    usize::try_from(value)
        .map_err(|_| Error::Unavailable("actual DesignDay signed allocation domain"))
}
fn ordinal(value: i32) -> Result<usize, Error> {
    value
        .checked_sub(1)
        .and_then(|v| usize::try_from(v).ok())
        .ok_or(Error::Unavailable("actual DesignDay one-based ordinal"))
}
fn integer(value: f64) -> Result<i32, Error> {
    let value = value.trunc();
    if !value.is_finite() || value < f64::from(i32::MIN) || value > f64::from(i32::MAX) {
        return Err(Error::Unavailable(
            "actual DesignDay Native float-to-int defined domain",
        ));
    }
    Ok(value as i32)
}
fn rounded(value: f64, precision: usize) -> Result<String, Error> {
    crate::heat_balance::source_round_format::round(value, precision)
        .map_err(|e| Error::Format(format!("actual DesignDay .{precision}R: {e:?}")))
}
fn enum_value(names: &[&str], value: &str) -> i32 {
    names
        .iter()
        .position(|name| *name == value)
        .map_or(-1, |i| i as i32)
}
