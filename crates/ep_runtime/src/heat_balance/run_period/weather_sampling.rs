//! Weather forcing sampled once for each run-period zone timestep.

use crate::heat_balance::longwave::horizontal_infrared_sky_temperature_c;
use crate::heat_balance::surface_weather::{
    energyplus_exterior_wet_reference_temperature_c,
    energyplus_weather_record_is_rain_at_timestep_with_starting_values,
};
use crate::weather::{
    EpwRecord, HeatBalanceWeatherContext, WeatherTimestepSeries,
    energyplus_weather_dry_bulb_at_timestep_with_starting_values,
    energyplus_weather_horizontal_infrared_for_context, heat_balance_weather_context_for_timestep,
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
    weather_series: Option<&'weather WeatherTimestepSeries>,
    hour_index: usize,
    outdoor_dry_bulb_c: f64,
    steps: u32,
    substep: u32,
    first_hour_interpolation_starting_values: FirstHourInterpolationStartingValues,
) -> RunPeriodWeatherSample<'weather> {
    let weather_context = heat_balance_weather_context_for_timestep(
        weather_series,
        hour_index,
        steps,
        substep,
        first_hour_interpolation_starting_values,
    );
    let timestep_outdoor_dry_bulb_c = weather_context
        .and_then(|context| context.sample.map(|sample| sample.dry_bulb_c))
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
            context.records.get(context.record_index).map(|record| {
                energyplus_weather_horizontal_infrared_for_context(
                    context,
                    record.horizontal_infrared_radiation_wh_per_m2,
                )
            })
        })
        .unwrap_or(0.0);
    let timestep_sky_temperature_c = horizontal_infrared_sky_temperature_c(
        timestep_horizontal_infrared_radiation_w_per_m2,
        timestep_outdoor_dry_bulb_c,
    );
    let timestep_rain_status = weather_context
        .map(|context| {
            let is_raining = context
                .sample
                .map(|sample| sample.liquid_precipitation_depth_mm >= 0.8)
                .unwrap_or_else(|| {
                    energyplus_weather_record_is_rain_at_timestep_with_starting_values(
                        context.records,
                        context.record_index,
                        substep,
                        steps,
                        context.first_hour_interpolation_starting_values,
                    )
                });
            if is_raining { 1.0 } else { 0.0 }
        })
        .unwrap_or(0.0);

    RunPeriodWeatherSample {
        weather_context,
        timestep_outdoor_dry_bulb_c,
        timestep_outdoor_wet_bulb_c,
        timestep_horizontal_infrared_radiation_w_per_m2,
        timestep_sky_temperature_c,
        timestep_rain_status,
    }
}
