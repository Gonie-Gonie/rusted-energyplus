//! Preserve the real weather/raw and borrowed Source input/backend causes.
use super::WeatherDayError;
use crate::heat_balance::error_backend::SourceErrorBackendFailure;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError;
use crate::weather::raw::RawDayFailure;
#[derive(Debug)]
pub(crate) enum SourceWeatherInitializationError {
    Input(HeatBalanceInputError),
    Weather(WeatherDayError),
    Raw(RawDayFailure),
}
impl From<HeatBalanceInputError> for SourceWeatherInitializationError {
    fn from(error: HeatBalanceInputError) -> Self {
        Self::Input(error)
    }
}
impl From<SourceErrorBackendFailure> for SourceWeatherInitializationError {
    fn from(error: SourceErrorBackendFailure) -> Self {
        Self::Input(HeatBalanceInputError::Backend(error))
    }
}
impl From<WeatherDayError> for SourceWeatherInitializationError {
    fn from(error: WeatherDayError) -> Self {
        Self::Weather(error)
    }
}
impl From<RawDayFailure> for SourceWeatherInitializationError {
    fn from(error: RawDayFailure) -> Self {
        Self::Raw(error)
    }
}
impl std::fmt::Display for SourceWeatherInitializationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Input(e) => e.fmt(f),
            Self::Weather(e) => e.fmt(f),
            Self::Raw(e) => e.fmt(f),
        }
    }
}
impl std::error::Error for SourceWeatherInitializationError {}
impl SourceWeatherInitializationError {
    pub(crate) fn into_runtime(self) -> crate::RuntimeError {
        match self {
            Self::Input(e) => crate::RuntimeError::HeatBalanceInput(Box::new(e)),
            Self::Weather(e) => e.into(),
            Self::Raw(e) => WeatherDayError::from(e).into(),
        }
    }
}
