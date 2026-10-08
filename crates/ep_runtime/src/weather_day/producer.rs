//! Explicit compatibility producer between actual raw records and daily transport.
//!
//! Existing scalar, wind, psychrometric and sky equations are retained. Physical
//! missing-value, precipitation, albedo, snow, radiation and sky policies remain
//! unpaired here pending their own weather cards; no native outputs are inputs.

use super::{WeatherDayError, WeatherDayState, WeatherVars};
use crate::heat_balance::longwave::horizontal_infrared_sky_temperature_c;
use crate::heat_balance::solar::{solar_weather_interpolation_weights, weighted_solar_value};
use crate::weather::raw::{RawEpwOutputs, RawWeatherDay, project_record};
use crate::weather::{
    EpwRecord, WeatherTimestepSample, energyplus_weather_interpolation_weight,
    weather_timestep_sample_with_neighbors,
};
use ep_model::FirstHourInterpolationStartingValues;

/// A successfully produced day, retaining actual projected hourly records.
#[derive(Clone, Debug)]
pub struct ProducedWeatherDay {
    /// The actual 24 raw rows' existing compatibility projection in source order.
    pub records: [EpwRecord; 24],
    /// Samples in hour-major order; record_index is local to this produced day.
    pub samples: Vec<WeatherTimestepSample>,
}

/// Full hourly compatibility carrier, reusable for actual partial-read writes.
///
/// Precipitation normalization is already performed by the existing raw record
/// projection. Rain and sky retain existing consumer policy. The raw optional
/// fields and cover values are literal carrier inputs, without native parity.
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
    let hourly: [WeatherVars; 24] = std::array::from_fn(|index| {
        weather_vars_from_raw(&raw_day.hours[index].raw, records[index])
    });
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
    let mut samples = Vec::with_capacity(24 * steps as usize);
    for index in 0..24 {
        let previous_record = if index == 0 {
            &initial_record
        } else {
            &records[index - 1]
        };
        let next_index = if index == 23 { 0 } else { index + 1 };
        if steps > 1 {
            state.next_hour.beam_solar_rad = hourly[next_index].beam_solar_rad;
            state.next_hour.dif_solar_rad = hourly[next_index].dif_solar_rad;
            state.next_hour.liquid_precip = hourly[next_index].liquid_precip;
        }
        for timestep in 1..=steps {
            let sample = weather_timestep_sample_with_neighbors(
                previous_record,
                &records[index],
                index,
                steps,
                timestep,
            );
            let mut value = hourly[index];
            if steps > 1 {
                let weight = energyplus_weather_interpolation_weight(steps, timestep);
                let scalar = |before: f64, now: f64| before * (1.0 - weight) + now * weight;
                value.out_dry_bulb_temp = sample.dry_bulb_c;
                value.out_dew_point_temp = scalar(
                    previous_values.out_dew_point_temp,
                    hourly[index].out_dew_point_temp,
                );
                value.out_baro_press = sample.atmospheric_pressure_pa;
                value.out_rel_hum = sample.relative_humidity_percent;
                value.wind_speed = sample.wind_speed_m_per_s;
                value.wind_dir = sample.wind_direction_deg;
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
                    value.out_dry_bulb_temp,
                );
                let (prior_weight, current_weight, next_weight) =
                    solar_weather_interpolation_weights(steps, timestep);
                value.beam_solar_rad = weighted_solar_value(
                    previous_values.beam_solar_rad,
                    hourly[index].beam_solar_rad,
                    hourly[next_index].beam_solar_rad,
                    prior_weight,
                    current_weight,
                    next_weight,
                );
                value.dif_solar_rad = weighted_solar_value(
                    previous_values.dif_solar_rad,
                    hourly[index].dif_solar_rad,
                    hourly[next_index].dif_solar_rad,
                    prior_weight,
                    current_weight,
                    next_weight,
                );
                // Existing compatibility rain/precipitation policy is intentionally unpaired.
                value.liquid_precip = sample.liquid_precipitation_depth_mm;
                value.is_rain = value.liquid_precip >= 0.8;
            }
            state.tomorrow_values.hour_mut(index + 1)?[timestep as usize - 1] = value;
            samples.push(sample);
        }
        if steps > 1 {
            state.last_hour = hourly[index];
            previous_values = hourly[index];
        }
    }
    Ok(ProducedWeatherDay { records, samples })
}

#[cfg(test)]
#[path = "producer_tests.rs"]
mod tests;
