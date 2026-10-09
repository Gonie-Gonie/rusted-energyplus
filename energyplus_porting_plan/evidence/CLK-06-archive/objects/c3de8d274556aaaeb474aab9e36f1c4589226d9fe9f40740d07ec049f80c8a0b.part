//! Shared weather-day boundary and owned-current-weather dispatch for A and B.

use crate::error::RuntimeError;
use crate::weather::day::{ProductionWeatherTimestepSeries, WeatherDayPhase};
use crate::weather::{
    HeatBalanceWeatherContext, WeatherTimestepSeries, heat_balance_weather_context_for_timestep,
};
use ep_model::FirstHourInterpolationStartingValues;

#[derive(Clone, Copy)]
pub(crate) enum HeatBalanceWeatherDriver<'a> {
    Legacy(&'a WeatherTimestepSeries),
    Production(&'a ProductionWeatherTimestepSeries),
}

impl<'a> HeatBalanceWeatherDriver<'a> {
    pub(crate) fn hourly_count(self) -> usize {
        match self {
            Self::Legacy(series) => series.hourly_dry_bulb_c().len(),
            Self::Production(series) => series.hourly_count(),
        }
    }

    pub(crate) fn prepare_initial_phase(self, phase: WeatherDayPhase) -> Result<(), RuntimeError> {
        if let Self::Production(series) = self {
            series.prepare_initial_phase(phase)?;
        }
        Ok(())
    }

    pub(crate) fn begin_day(self, phase: WeatherDayPhase) -> Result<(), RuntimeError> {
        if let Self::Production(series) = self {
            series.begin_day(phase)?;
        }
        Ok(())
    }

    pub(crate) fn finish_environment(self) -> Result<(), RuntimeError> {
        if let Self::Production(series) = self {
            series.finish_environment()?;
        }
        Ok(())
    }

    pub(crate) fn initial_hourly_dry_bulb_c(self) -> Result<f64, RuntimeError> {
        match self {
            Self::Legacy(series) => series
                .hourly_dry_bulb_c()
                .first()
                .copied()
                .ok_or(RuntimeError::NoWeatherData),
            Self::Production(series) => Ok(series.initial_hourly_dry_bulb_c()?),
        }
    }

    pub(crate) fn current_context(
        self,
        record_index: usize,
        zone_steps_per_hour: u32,
        zone_timestep: u32,
        first_hour_interpolation_starting_values: FirstHourInterpolationStartingValues,
    ) -> Result<HeatBalanceWeatherContext<'a>, RuntimeError> {
        match self {
            Self::Legacy(series) => heat_balance_weather_context_for_timestep(
                Some(series),
                record_index,
                zone_steps_per_hour,
                zone_timestep,
                first_hour_interpolation_starting_values,
            )
            .ok_or(RuntimeError::NoWeatherData),
            Self::Production(series) => Ok(HeatBalanceWeatherContext {
                records: &[],
                sample: None,
                owned: Some(series.current_for(record_index, zone_timestep)?),
                record_index,
                zone_steps_per_hour,
                zone_timestep: Some(zone_timestep),
                first_hour_interpolation_starting_values,
            }),
        }
    }
}
