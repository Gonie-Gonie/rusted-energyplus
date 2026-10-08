//! Weather forcing sampled once for each run-period zone timestep.

use crate::error::RuntimeError;
use crate::heat_balance::longwave::horizontal_infrared_sky_temperature_c;
use crate::heat_balance::surface_weather::{
    energyplus_exterior_wet_reference_temperature_c,
    energyplus_weather_record_is_rain_at_timestep_with_starting_values,
};
use crate::heat_balance::weather_driver::HeatBalanceWeatherDriver;
use crate::weather::{
    EpwRecord, HeatBalanceWeatherContext,
    energyplus_weather_dry_bulb_at_timestep_with_starting_values,
    energyplus_weather_horizontal_infrared_for_context,
};
use ep_model::FirstHourInterpolationStartingValues;

pub(super) struct RunPeriodWeatherSample<'weather> {
    pub weather_context: Option<HeatBalanceWeatherContext<'weather>>,
    pub timestep_outdoor_dry_bulb_c: f64,
    pub timestep_outdoor_wet_bulb_c: f64,
    pub timestep_horizontal_infrared_radiation_w_per_m2: f64,
    pub timestep_sky_temperature_c: f64,
    pub timestep_rain_status: f64,
}

pub(super) fn sample_run_period_weather<'weather>(
    weather_records: Option<&[EpwRecord]>,
    weather_driver: Option<HeatBalanceWeatherDriver<'weather>>,
    hour_index: usize,
    outdoor_dry_bulb_c: Option<f64>,
    steps: u32,
    substep: u32,
    first_hour_interpolation_starting_values: FirstHourInterpolationStartingValues,
) -> Result<RunPeriodWeatherSample<'weather>, RuntimeError> {
    let weather_context = weather_driver
        .map(|driver| {
            driver.current_context(
                hour_index,
                steps,
                substep,
                first_hour_interpolation_starting_values,
            )
        })
        .transpose()?;
    let outdoor_dry_bulb_c = weather_context
        .and_then(|context| context.current_record().map(|record| record.dry_bulb_c))
        .or(outdoor_dry_bulb_c)
        .ok_or(RuntimeError::NoWeatherData)?;
    let timestep_outdoor_dry_bulb_c = weather_context
        .and_then(|context| context.sample_value().map(|sample| sample.dry_bulb_c))
        .unwrap_or_else(|| {
            energyplus_weather_dry_bulb_at_timestep_with_starting_values(
                weather_records,
                hour_index,
                outdoor_dry_bulb_c,
                steps,
                substep,
                first_hour_interpolation_starting_values,
            )
        });
    let timestep_outdoor_wet_bulb_c = weather_context
        .map(|context| {
            energyplus_exterior_wet_reference_temperature_c(context, timestep_outdoor_dry_bulb_c)
        })
        .unwrap_or(timestep_outdoor_dry_bulb_c);
    let timestep_horizontal_infrared_radiation_w_per_m2 = weather_context
        .and_then(|context| {
            context.current_record().map(|record| {
                energyplus_weather_horizontal_infrared_for_context(
                    context,
                    record.horizontal_infrared_radiation_wh_per_m2,
                )
            })
        })
        .unwrap_or(0.0);
    let timestep_sky_temperature_c = weather_context
        .and_then(|context| context.owned.map(|current| current.weather.sky_temp))
        .unwrap_or_else(|| {
            horizontal_infrared_sky_temperature_c(
                timestep_horizontal_infrared_radiation_w_per_m2,
                timestep_outdoor_dry_bulb_c,
            )
        });
    let timestep_rain_status = weather_context
        .map(|context| {
            let is_raining = context
                .owned
                .map(|current| current.weather.is_rain)
                .unwrap_or_else(|| {
                    context
                        .sample_value()
                        .map(|sample| sample.liquid_precipitation_depth_mm >= 0.8)
                        .unwrap_or_else(|| {
                            energyplus_weather_record_is_rain_at_timestep_with_starting_values(
                                context.records,
                                context.record_index,
                                substep,
                                steps,
                                context.first_hour_interpolation_starting_values,
                            )
                        })
                });
            if is_raining { 1.0 } else { 0.0 }
        })
        .unwrap_or(0.0);

    Ok(RunPeriodWeatherSample {
        weather_context,
        timestep_outdoor_dry_bulb_c,
        timestep_outdoor_wet_bulb_c,
        timestep_horizontal_infrared_radiation_w_per_m2,
        timestep_sky_temperature_c,
        timestep_rain_status,
    })
}
