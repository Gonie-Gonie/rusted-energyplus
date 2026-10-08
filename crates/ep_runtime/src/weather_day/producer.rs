//! Selected source interpolation between processed hourly and daily transport.
//!
//! Hourly missing/range/history processing already ran in the reader callback.
//! Solar interpolation uses stored source weights and processed hourly values.
//! Sky, snow and optional compatibility fields retain separate unpaired
//! policies. No native outputs are inputs to this owner.

use super::hourly::interpolate_wind_direction;
use super::solar::interpolation_weights;
use super::{WeatherDayError, WeatherDayState, WeatherVars};
use crate::heat_balance::longwave::horizontal_infrared_sky_temperature_c;
use crate::psychrometrics::with_fresh_psychrometric_state;
use crate::weather::raw::{RawEpwOutputs, RawWeatherDay, project_record};
use crate::weather::{EpwRecord, WeatherTimestepSample, weather_timestep_sample_with_neighbors};
use ep_model::FirstHourInterpolationStartingValues;

/// A successfully produced day, retaining actual projected hourly records.
#[derive(Clone, Debug)]
pub struct ProducedWeatherDay {
    /// The actual 24 raw rows' existing compatibility projection in source order.
    pub records: [EpwRecord; 24],
    /// Actual processed interval-one carriers, saved before interpolation.
    /// These are Rust storage, not observations of native private hourly locals.
    pub hourly_values: [WeatherVars; 24],
    /// Samples in hour-major order; record_index is local to this produced day.
    pub samples: Vec<WeatherTimestepSample>,
}

/// Base carrier for the selected hourly processing callback.
///
/// The callback replaces the seven selected raw scalar fields and rain flag.
/// Sky and raw optional fields here remain compatibility policy, without native
/// parity. The raw EpwRecord projection itself is retained separately.
pub fn weather_vars_from_raw(raw: &RawEpwOutputs, record: EpwRecord) -> WeatherVars {
    WeatherVars {
        is_rain: record.liquid_precipitation_depth_mm >= 0.8,
        // Explicit compatibility treatment of the raw 999 snow-depth sentinel.
        is_snow: raw.optional_reals[2] > 0.0 && raw.optional_reals[2] < 999.0,
        out_dry_bulb_temp: record.dry_bulb_c,
        out_dew_point_temp: record.dew_point_c,
        out_baro_press: record.atmospheric_pressure_pa,
        out_rel_hum: record.relative_humidity_percent,
        wind_speed: record.wind_speed_m_per_s,
        wind_dir: record.wind_direction_deg,
        sky_temp: horizontal_infrared_sky_temperature_c(
            record.horizontal_infrared_radiation_wh_per_m2,
            record.dry_bulb_c,
        ),
        horiz_ir_sky: record.horizontal_infrared_radiation_wh_per_m2,
        beam_solar_rad: record.direct_normal_radiation_wh_per_m2,
        dif_solar_rad: record.diffuse_horizontal_radiation_wh_per_m2,
        albedo: raw.optional_reals[4],
        water_precip: raw.optional_reals[0],
        liquid_precip: record.liquid_precipitation_depth_mm,
        total_sky_cover: raw.mandatory_reals[16],
        opaque_sky_cover: raw.mandatory_reals[17],
    }
}

/// Produces one real read day with an explicit previous-day scalar predecessor.
///
/// The hourly array remains exactly 24 rows. The source next-hour cache writes
/// only beam, diffuse and liquid fields; the other cache fields are retained.
/// No raw/header/parser/cursor or daily calendar owner is changed here.
pub fn produce_day(
    raw_day: &RawWeatherDay,
    prior_hour: Option<EpwRecord>,
    steps: u32,
    first_policy: FirstHourInterpolationStartingValues,
    state: &mut WeatherDayState,
) -> Result<ProducedWeatherDay, WeatherDayError> {
    if raw_day.hours.len() != 24 || steps == 0 {
        return Err(WeatherDayError::admission(
            "compatibility producer requires 24 hours and nonzero timesteps",
        ));
    }
    if !state.tomorrow_values.is_allocated() || state.tomorrow_values.time_steps() != steps as usize
    {
        return Err(WeatherDayError::admission(
            "prepared tomorrow allocation differs from producer timestep shape",
        ));
    }
    let weights = state
        .weather
        .interpolation
        .as_ref()
        .filter(|values| values.len() == steps as usize)
        .ok_or_else(|| WeatherDayError::admission("prepared interpolation owner unavailable"))?
        .clone();
    let solar_weights = if steps > 1 {
        Some(
            state
                .weather
                .solar_interpolation
                .as_ref()
                .filter(|values| values.len() == steps as usize)
                .ok_or_else(|| {
                    WeatherDayError::admission("prepared solar interpolation owner unavailable")
                })?
                .clone(),
        )
    } else {
        None
    };
    let projected = raw_day
        .hours
        .iter()
        .map(|slot| {
            project_record(&slot.raw, slot.provenance.line_read_attempt)
                .map_err(|error| WeatherDayError::admission(error.to_string()))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let records: [EpwRecord; 24] = projected
        .try_into()
        .map_err(|_| WeatherDayError::admission("projected weather day does not have 24 hours"))?;
    // Do not replay hourly processing: its writes and counts are observable
    // after every read, including a later record's failure.
    let mut hourly = [WeatherVars::default(); 24];
    for (index, value) in hourly.iter_mut().enumerate() {
        *value = state
            .tomorrow_values
            .hour(index + 1)?
            .first()
            .copied()
            .ok_or_else(|| WeatherDayError::admission("processed hourly interval unavailable"))?;
    }
    let seed_index = match first_policy {
        FirstHourInterpolationStartingValues::Hour1 => 0,
        FirstHourInterpolationStartingValues::Hour24 => 23,
    };
    let initial_record = prior_hour.unwrap_or(records[seed_index]);
    if steps > 1 && !state.weather.last_hour_set {
        state.last_hour = hourly[seed_index];
        state.weather.last_hour_set = true;
    }
    let mut previous_values = if steps > 1 {
        state.last_hour
    } else {
        hourly[seed_index]
    };
    // Compatibility preview sampling invokes PSY caches. It has a separate
    // lifetime from genuine SetCurrentWeather so it cannot seed that owner.
    let samples = with_fresh_psychrometric_state(|| {
        let mut samples = Vec::with_capacity(24 * steps as usize);
        for (index, current) in records.iter().enumerate() {
            let previous = if index == 0 {
                &initial_record
            } else {
                &records[index - 1]
            };
            for timestep in 1..=steps {
                samples.push(weather_timestep_sample_with_neighbors(
                    previous, current, index, steps, timestep,
                ));
            }
        }
        samples
    });
    for index in 0..24 {
        let next_index = if index == 23 { 0 } else { index + 1 };
        if steps > 1 {
            state.next_hour.beam_solar_rad = hourly[next_index].beam_solar_rad;
            state.next_hour.dif_solar_rad = hourly[next_index].dif_solar_rad;
            state.next_hour.liquid_precip = hourly[next_index].liquid_precip;
        }
        for timestep in 1..=steps {
            let sample = &samples[index * steps as usize + timestep as usize - 1];
            let mut value = hourly[index];
            if steps > 1 {
                let weight = weights[timestep as usize - 1];
                let scalar = |before: f64, now: f64| before * (1.0 - weight) + now * weight;
                value.out_dry_bulb_temp = scalar(
                    previous_values.out_dry_bulb_temp,
                    hourly[index].out_dry_bulb_temp,
                );
                value.out_dew_point_temp = scalar(
                    previous_values.out_dew_point_temp,
                    hourly[index].out_dew_point_temp,
                );
                value.out_baro_press =
                    scalar(previous_values.out_baro_press, hourly[index].out_baro_press);
                value.out_rel_hum = scalar(previous_values.out_rel_hum, hourly[index].out_rel_hum);
                value.wind_speed = scalar(previous_values.wind_speed, hourly[index].wind_speed);
                value.wind_dir = interpolate_wind_direction(
                    previous_values.wind_dir,
                    hourly[index].wind_dir,
                    weight,
                );
                value.total_sky_cover = scalar(
                    previous_values.total_sky_cover,
                    hourly[index].total_sky_cover,
                );
                value.opaque_sky_cover = scalar(
                    previous_values.opaque_sky_cover,
                    hourly[index].opaque_sky_cover,
                );
                value.horiz_ir_sky = sample.horizontal_infrared_radiation_w_per_m2;
                value.sky_temp = horizontal_infrared_sky_temperature_c(
                    value.horiz_ir_sky,
                    // Keep the existing raw-preview sky policy separate from
                    // the selected processed dry-bulb interpolation.
                    sample.dry_bulb_c,
                );
                let current_weight = *solar_weights
                    .as_ref()
                    .and_then(|values| values.get(timestep as usize - 1))
                    .ok_or_else(|| {
                        WeatherDayError::admission("solar timestep storage unavailable")
                    })?;
                let (prior_weight, next_weight) = interpolation_weights(
                    current_weight,
                    timestep,
                    state.weather.time_step_fraction,
                );
                value.dif_solar_rad = previous_values.dif_solar_rad * prior_weight
                    + hourly[index].dif_solar_rad * current_weight
                    + state.next_hour.dif_solar_rad * next_weight;
                value.beam_solar_rad = previous_values.beam_solar_rad * prior_weight
                    + hourly[index].beam_solar_rad * current_weight
                    + state.next_hour.beam_solar_rad * next_weight;
                value.liquid_precip =
                    scalar(previous_values.liquid_precip, hourly[index].liquid_precip);
                value.liquid_precip /= f64::from(steps);
                value.is_rain = value.liquid_precip >= state.weather.is_rain_threshold;
            }
            state.tomorrow_values.hour_mut(index + 1)?[timestep as usize - 1] = value;
        }
        if steps > 1 {
            state.last_hour = hourly[index];
            previous_values = hourly[index];
        }
    }
    Ok(ProducedWeatherDay {
        records,
        hourly_values: hourly,
        samples,
    })
}

#[cfg(test)]
#[path = "producer_tests.rs"]
mod tests;
