//! Actual ReadEPlusWeatherForDay3050–3178 post-read interpolation and sky override.
use super::{Error, Failure, ordinal};
use crate::psychrometrics::SourcePsychrometricAccess;
use crate::weather::day::{WeatherDayError, WeatherDayState, WeatherVars};
use crate::weather::raw::RawEpwHeaderState;

pub(super) fn interpolate_source_day(
    state: &mut WeatherDayState,
    header: &RawEpwHeaderState,
    environ: usize,
    psy: &mut SourcePsychrometricAccess,
) -> Result<(), Failure> {
    if header.intervals_per_hour == 1 && state.global.time_steps_in_hour > 1 {
        // Same Native local Array1D<WeatherVars>(24), copied at its actual loop.
        let mut hourly = [WeatherVars::default(); 24];
        for hour in 1..=24 {
            hourly[hour - 1] = *state
                .tomorrow_values
                .hour(hour)
                .map_err(WeatherDayError::from)?
                .first()
                .ok_or(Error::Unavailable("actual hourly Tomorrow copy3055"))?;
        }
        if !state.weather.last_hour_set {
            let row = state
                .weather
                .source_environments
                .as_ref()
                .and_then(|v| v.get(environ))
                .ok_or(Error::Unavailable(
                    "actual firstHrInterpUseHr1 Environment3061",
                ))?;
            let used = if row.first_hour_interpolation_uses_hour_one {
                1
            } else {
                24
            };
            state.last_hour = hourly[used - 1];
            state.weather.last_hour_set = true;
        }
        for hour in 1..=24 {
            let next = if hour == 24 { 1 } else { hour + 1 };
            state.next_hour.beam_solar_rad = hourly[next - 1].beam_solar_rad;
            state.next_hour.dif_solar_rad = hourly[next - 1].dif_solar_rad;
            state.next_hour.liquid_precip = hourly[next - 1].liquid_precip;
            for timestep in 1..=state.global.time_steps_in_hour {
                let position = ordinal(timestep)?;
                let current = *state
                    .weather
                    .interpolation
                    .as_ref()
                    .and_then(|v| v.get(position))
                    .ok_or(Error::Unavailable("actual Interpolation3077"))?;
                let previous = 1.0 - current;
                let current_solar = *state
                    .weather
                    .solar_interpolation
                    .as_ref()
                    .and_then(|v| v.get(position))
                    .ok_or(Error::Unavailable("actual SolarInterpolation3082"))?;
                let previous_solar;
                let next_solar;
                if state.global.time_steps_in_hour == 1 {
                    next_solar = 1.0 - current;
                    previous_solar = 0.0;
                } else if current_solar == 1.0 {
                    previous_solar = 0.0;
                    next_solar = 0.0;
                } else if f64::from(timestep) * state.weather.time_step_fraction < 0.5 {
                    previous_solar = 1.0 - current_solar;
                    next_solar = 0.0;
                } else {
                    previous_solar = 0.0;
                    next_solar = 1.0 - current_solar;
                }
                let tomorrow = state
                    .tomorrow_values
                    .hour_mut(hour)
                    .map_err(WeatherDayError::from)?
                    .get_mut(position)
                    .ok_or(Error::Unavailable("actual Tomorrow timestep3100"))?;
                let hour_values = &hourly[hour - 1];
                tomorrow.out_dry_bulb_temp = state.last_hour.out_dry_bulb_temp * previous
                    + hour_values.out_dry_bulb_temp * current;
                tomorrow.out_baro_press = state.last_hour.out_baro_press * previous
                    + hour_values.out_baro_press * current;
                tomorrow.out_dew_point_temp = state.last_hour.out_dew_point_temp * previous
                    + hour_values.out_dew_point_temp * current;
                tomorrow.out_rel_hum =
                    state.last_hour.out_rel_hum * previous + hour_values.out_rel_hum * current;
                tomorrow.wind_speed =
                    state.last_hour.wind_speed * previous + hour_values.wind_speed * current;
                tomorrow.wind_dir = super::super::hourly::interpolate_wind_direction(
                    state.last_hour.wind_dir,
                    hour_values.wind_dir,
                    current,
                );
                tomorrow.total_sky_cover = state.last_hour.total_sky_cover * previous
                    + hour_values.total_sky_cover * current;
                tomorrow.opaque_sky_cover = state.last_hour.opaque_sky_cover * previous
                    + hour_values.opaque_sky_cover * current;
                super::sky::calc_source_sky(
                    &state.weather.source_environments,
                    &state.weather.envrn,
                    &mut tomorrow.horiz_ir_sky,
                    &mut tomorrow.sky_temp,
                    tomorrow.opaque_sky_cover,
                    tomorrow.out_dry_bulb_temp,
                    tomorrow.out_dew_point_temp,
                    tomorrow.out_rel_hum * 0.01,
                    state.last_hour.horiz_ir_sky * previous + hour_values.horiz_ir_sky * current,
                    psy,
                )?;
                tomorrow.dif_solar_rad = state.last_hour.dif_solar_rad * previous_solar
                    + hour_values.dif_solar_rad * current_solar
                    + state.next_hour.dif_solar_rad * next_solar;
                tomorrow.beam_solar_rad = state.last_hour.beam_solar_rad * previous_solar
                    + hour_values.beam_solar_rad * current_solar
                    + state.next_hour.beam_solar_rad * next_solar;
                tomorrow.liquid_precip =
                    state.last_hour.liquid_precip * previous + hour_values.liquid_precip * current;
                tomorrow.liquid_precip /= f64::from(state.global.time_steps_in_hour);
                tomorrow.is_rain = tomorrow.liquid_precip >= state.weather.is_rain_threshold;
                tomorrow.is_snow = hour_values.is_snow;
            }
            state.last_hour = hourly[hour - 1];
        }
    }
    let row = state
        .weather
        .source_environments
        .as_ref()
        .and_then(|v| v.get(environ))
        .ok_or(Error::Unavailable(
            "actual post-read Environment.WP_Type13143",
        ))?;
    if row.weather_properties_type_one != 0 {
        let property = state
            .weather
            .source_weather_properties
            .as_ref()
            .and_then(|v| v.get(ordinal(row.weather_properties_type_one).ok()?))
            .ok_or(Error::Unavailable(
                "actual WPSkyTemperature indexed read3144",
            ))?;
        if matches!(property.sky_temperature_model, 1..=3) {
            return Err(
                Error::Unavailable("actual positive weather-sky getDayVals3148/3162/3176").into(),
            );
        }
    }
    Ok(())
}
