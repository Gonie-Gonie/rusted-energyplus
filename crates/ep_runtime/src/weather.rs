//! EPW weather parsing helpers.

use crate::psychrometrics::{
    energyplus_outdoor_wet_bulb_c, energyplus_psychrometric_humidity_ratio_from_rh,
};
use ep_model::FirstHourInterpolationStartingValues;
use std::fmt::{Display, Formatter};
use std::path::Path;

/// Error returned while reading EPW weather data.
#[derive(Debug)]
pub enum EpwError {
    /// File read failed.
    Io(std::io::Error),
    /// EPW data row was missing a required column.
    MissingField {
        /// One-based line number.
        line: usize,
        /// EPW field name.
        field: &'static str,
    },
    /// EPW numeric field could not be parsed.
    InvalidNumber {
        /// One-based line number.
        line: usize,
        /// EPW field name.
        field: &'static str,
        /// Raw field text.
        value: String,
    },
}

impl Display for EpwError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "failed to read EPW: {error}"),
            Self::MissingField { line, field } => {
                write!(formatter, "EPW row at line {line} is missing {field}")
            }
            Self::InvalidNumber { line, field, value } => {
                write!(
                    formatter,
                    "EPW row at line {line} has invalid {field} value '{value}'"
                )
            }
        }
    }
}

impl std::error::Error for EpwError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::MissingField { .. } | Self::InvalidNumber { .. } => None,
        }
    }
}

impl From<std::io::Error> for EpwError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

/// One hourly EPW weather record for the current compatibility subset.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EpwRecord {
    /// Calendar year.
    pub year: u32,
    /// Month number, 1-12.
    pub month: u32,
    /// Day of month.
    pub day: u32,
    /// Hour ending, 1-24.
    pub hour: u32,
    /// Minute field from EPW.
    pub minute: u32,
    /// Outdoor dry-bulb temperature in C.
    pub dry_bulb_c: f64,
    /// Outdoor dew-point temperature in C.
    pub dew_point_c: f64,
    /// Relative humidity in percent.
    pub relative_humidity_percent: f64,
    /// Atmospheric station pressure in Pa.
    pub atmospheric_pressure_pa: f64,
    /// Horizontal infrared radiation intensity in Wh/m2.
    pub horizontal_infrared_radiation_wh_per_m2: f64,
    /// Global horizontal radiation in Wh/m2.
    pub global_horizontal_radiation_wh_per_m2: f64,
    /// Direct normal radiation in Wh/m2.
    pub direct_normal_radiation_wh_per_m2: f64,
    /// Diffuse horizontal radiation in Wh/m2.
    pub diffuse_horizontal_radiation_wh_per_m2: f64,
    /// Wind direction in degrees.
    pub wind_direction_deg: f64,
    /// Wind speed in m/s.
    pub wind_speed_m_per_s: f64,
    /// Liquid precipitation depth in mm for the hour when present.
    pub liquid_precipitation_depth_mm: f64,
}

#[derive(Clone, Copy)]
pub(crate) struct HeatBalanceWeatherContext<'a> {
    pub(crate) records: &'a [EpwRecord],
    pub(crate) sample: Option<&'a WeatherTimestepSample>,
    pub(crate) record_index: usize,
    pub(crate) zone_steps_per_hour: u32,
    pub(crate) zone_timestep: Option<u32>,
    pub(crate) first_hour_interpolation_starting_values: FirstHourInterpolationStartingValues,
}

/// One weather sample precomputed for a zone timestep.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WeatherTimestepSample {
    /// Zero-based EPW record index.
    pub record_index: usize,
    /// One-based zone timestep within the hour.
    pub timestep: u32,
    /// Interpolated outdoor dry-bulb temperature in C.
    pub dry_bulb_c: f64,
    /// Interpolated outdoor wet-bulb temperature in C.
    pub wet_bulb_c: f64,
    /// Interpolated relative humidity in percent.
    pub relative_humidity_percent: f64,
    /// Interpolated outdoor humidity ratio in kgWater/kgDryAir.
    pub outdoor_humidity_ratio: f64,
    /// Interpolated atmospheric pressure in Pa.
    pub atmospheric_pressure_pa: f64,
    /// Interpolated horizontal infrared radiation in W/m2.
    pub horizontal_infrared_radiation_w_per_m2: f64,
    /// Interpolated global horizontal solar radiation in W/m2.
    pub global_horizontal_radiation_w_per_m2: f64,
    /// Interpolated direct normal solar radiation in W/m2.
    pub direct_normal_radiation_w_per_m2: f64,
    /// Interpolated diffuse horizontal solar radiation in W/m2.
    pub diffuse_horizontal_radiation_w_per_m2: f64,
    /// Interpolated wind speed in m/s.
    pub wind_speed_m_per_s: f64,
    /// Interpolated wind direction in degrees.
    pub wind_direction_deg: f64,
    /// Interpolated liquid precipitation depth in mm.
    pub liquid_precipitation_depth_mm: f64,
}

/// Weather values resolved once for a model timestep configuration.
#[derive(Clone, Debug, PartialEq)]
pub struct WeatherTimestepSeries {
    /// Zone timesteps per hour used to sample hourly EPW records.
    pub zone_steps_per_hour: u32,
    /// First-hour interpolation policy selected by the run period.
    pub first_hour_interpolation_starting_values: FirstHourInterpolationStartingValues,
    hourly_records: Vec<EpwRecord>,
    hourly_dry_bulb_c: Vec<f64>,
    timestep_dry_bulb_c: Vec<f64>,
    timestep_wet_bulb_c: Vec<f64>,
    timestep_relative_humidity_percent: Vec<f64>,
    timestep_atmospheric_pressure_pa: Vec<f64>,
    timestep_wind_speed_m_per_s: Vec<f64>,
    timestep_wind_direction_deg: Vec<f64>,
    timestep_global_horizontal_radiation_w_per_m2: Vec<f64>,
    timestep_direct_normal_radiation_w_per_m2: Vec<f64>,
    timestep_diffuse_horizontal_radiation_w_per_m2: Vec<f64>,
    timestep_horizontal_infrared_radiation_w_per_m2: Vec<f64>,
    timestep_samples: Vec<WeatherTimestepSample>,
}

impl WeatherTimestepSeries {
    /// Precomputes hourly and zone-timestep weather samples from EPW records.
    #[must_use]
    pub fn from_records(
        records: &[EpwRecord],
        zone_steps_per_hour: u32,
        first_hour_interpolation_starting_values: FirstHourInterpolationStartingValues,
    ) -> Self {
        let steps = zone_steps_per_hour.max(1);
        let hourly_dry_bulb_c = records
            .iter()
            .map(|record| record.dry_bulb_c)
            .collect::<Vec<_>>();
        let mut timestep_dry_bulb_c = Vec::with_capacity(records.len() * steps as usize);
        let mut timestep_wet_bulb_c = Vec::with_capacity(records.len() * steps as usize);
        let mut timestep_relative_humidity_percent =
            Vec::with_capacity(records.len() * steps as usize);
        let mut timestep_atmospheric_pressure_pa =
            Vec::with_capacity(records.len() * steps as usize);
        let mut timestep_wind_speed_m_per_s = Vec::with_capacity(records.len() * steps as usize);
        let mut timestep_wind_direction_deg = Vec::with_capacity(records.len() * steps as usize);
        let mut timestep_global_horizontal_radiation_w_per_m2 =
            Vec::with_capacity(records.len() * steps as usize);
        let mut timestep_direct_normal_radiation_w_per_m2 =
            Vec::with_capacity(records.len() * steps as usize);
        let mut timestep_diffuse_horizontal_radiation_w_per_m2 =
            Vec::with_capacity(records.len() * steps as usize);
        let mut timestep_horizontal_infrared_radiation_w_per_m2 =
            Vec::with_capacity(records.len() * steps as usize);
        let mut timestep_samples = Vec::with_capacity(records.len() * steps as usize);
        for (record_index, record) in records.iter().enumerate() {
            for timestep in 1..=steps {
                let dry_bulb_c = energyplus_weather_dry_bulb_at_timestep_with_starting_values(
                    Some(records),
                    record_index,
                    record.dry_bulb_c,
                    steps,
                    timestep,
                    first_hour_interpolation_starting_values,
                );
                let relative_humidity_percent =
                    energyplus_weather_relative_humidity_at_timestep_with_starting_values(
                        records,
                        record_index,
                        record.relative_humidity_percent,
                        steps,
                        timestep,
                        first_hour_interpolation_starting_values,
                    );
                let atmospheric_pressure_pa =
                    energyplus_weather_atmospheric_pressure_at_timestep_with_starting_values(
                        records,
                        record_index,
                        record.atmospheric_pressure_pa,
                        steps,
                        timestep,
                        first_hour_interpolation_starting_values,
                    );
                let horizontal_infrared_radiation_w_per_m2 =
                    energyplus_weather_horizontal_infrared_at_timestep_with_starting_values(
                        records,
                        record_index,
                        record.horizontal_infrared_radiation_wh_per_m2,
                        steps,
                        timestep,
                        first_hour_interpolation_starting_values,
                    );
                let global_horizontal_radiation_w_per_m2 =
                    energyplus_weather_global_horizontal_radiation_at_timestep_with_starting_values(
                        records,
                        record_index,
                        record.global_horizontal_radiation_wh_per_m2,
                        steps,
                        timestep,
                        first_hour_interpolation_starting_values,
                    );
                let direct_normal_radiation_w_per_m2 =
                    energyplus_weather_direct_normal_radiation_at_timestep_with_starting_values(
                        records,
                        record_index,
                        record.direct_normal_radiation_wh_per_m2,
                        steps,
                        timestep,
                        first_hour_interpolation_starting_values,
                    );
                let diffuse_horizontal_radiation_w_per_m2 =
                    energyplus_weather_diffuse_horizontal_radiation_at_timestep_with_starting_values(
                        records,
                        record_index,
                        record.diffuse_horizontal_radiation_wh_per_m2,
                        steps,
                        timestep,
                        first_hour_interpolation_starting_values,
                    );
                let wind_speed_m_per_s =
                    energyplus_weather_wind_speed_at_timestep_with_starting_values(
                        records,
                        record_index,
                        record.wind_speed_m_per_s,
                        steps,
                        timestep,
                        first_hour_interpolation_starting_values,
                    );
                let wind_direction_deg =
                    energyplus_weather_wind_direction_at_timestep_with_starting_values(
                        records,
                        record_index,
                        record.wind_direction_deg,
                        steps,
                        timestep,
                        first_hour_interpolation_starting_values,
                    );
                let liquid_precipitation_depth_mm =
                    energyplus_weather_liquid_precipitation_at_timestep_with_starting_values(
                        records,
                        record_index,
                        record.liquid_precipitation_depth_mm,
                        steps,
                        timestep,
                        first_hour_interpolation_starting_values,
                    );
                let outdoor_humidity_ratio = energyplus_psychrometric_humidity_ratio_from_rh(
                    dry_bulb_c,
                    (relative_humidity_percent * 0.01).clamp(0.0, 1.0),
                    atmospheric_pressure_pa,
                )
                .unwrap_or(0.0);
                let wet_bulb_c = energyplus_outdoor_wet_bulb_c(
                    dry_bulb_c,
                    relative_humidity_percent,
                    atmospheric_pressure_pa,
                )
                .unwrap_or(dry_bulb_c);

                timestep_dry_bulb_c.push(dry_bulb_c);
                timestep_wet_bulb_c.push(wet_bulb_c);
                timestep_relative_humidity_percent.push(relative_humidity_percent);
                timestep_atmospheric_pressure_pa.push(atmospheric_pressure_pa);
                timestep_wind_speed_m_per_s.push(wind_speed_m_per_s);
                timestep_wind_direction_deg.push(wind_direction_deg);
                timestep_global_horizontal_radiation_w_per_m2
                    .push(global_horizontal_radiation_w_per_m2);
                timestep_direct_normal_radiation_w_per_m2.push(direct_normal_radiation_w_per_m2);
                timestep_diffuse_horizontal_radiation_w_per_m2
                    .push(diffuse_horizontal_radiation_w_per_m2);
                timestep_horizontal_infrared_radiation_w_per_m2
                    .push(horizontal_infrared_radiation_w_per_m2);

                timestep_samples.push(WeatherTimestepSample {
                    record_index,
                    timestep,
                    dry_bulb_c,
                    wet_bulb_c,
                    relative_humidity_percent,
                    outdoor_humidity_ratio,
                    atmospheric_pressure_pa,
                    horizontal_infrared_radiation_w_per_m2,
                    global_horizontal_radiation_w_per_m2,
                    direct_normal_radiation_w_per_m2,
                    diffuse_horizontal_radiation_w_per_m2,
                    wind_speed_m_per_s,
                    wind_direction_deg,
                    liquid_precipitation_depth_mm,
                });
            }
        }

        Self {
            zone_steps_per_hour: steps,
            first_hour_interpolation_starting_values,
            hourly_records: records.to_vec(),
            hourly_dry_bulb_c,
            timestep_dry_bulb_c,
            timestep_wet_bulb_c,
            timestep_relative_humidity_percent,
            timestep_atmospheric_pressure_pa,
            timestep_wind_speed_m_per_s,
            timestep_wind_direction_deg,
            timestep_global_horizontal_radiation_w_per_m2,
            timestep_direct_normal_radiation_w_per_m2,
            timestep_diffuse_horizontal_radiation_w_per_m2,
            timestep_horizontal_infrared_radiation_w_per_m2,
            timestep_samples,
        }
    }

    /// Returns hourly EPW records parsed before runtime execution.
    #[must_use]
    pub fn hourly_records(&self) -> &[EpwRecord] {
        &self.hourly_records
    }

    /// Returns hourly dry-bulb values in EPW record order.
    #[must_use]
    pub fn hourly_dry_bulb_c(&self) -> &[f64] {
        &self.hourly_dry_bulb_c
    }

    /// Returns precomputed dry-bulb values in zone-timestep order.
    #[must_use]
    pub fn timestep_dry_bulb_c(&self) -> &[f64] {
        &self.timestep_dry_bulb_c
    }

    /// Returns precomputed wet-bulb values in zone-timestep order.
    #[must_use]
    pub fn timestep_wet_bulb_c(&self) -> &[f64] {
        &self.timestep_wet_bulb_c
    }

    /// Returns precomputed direct-normal solar values in zone-timestep order.
    #[must_use]
    pub fn timestep_direct_normal_radiation_w_per_m2(&self) -> &[f64] {
        &self.timestep_direct_normal_radiation_w_per_m2
    }

    /// Returns precomputed zone-timestep weather samples.
    #[must_use]
    pub fn timestep_samples(&self) -> &[WeatherTimestepSample] {
        &self.timestep_samples
    }

    /// Returns one precomputed zone-timestep weather sample.
    #[must_use]
    pub fn sample_for(&self, record_index: usize, timestep: u32) -> Option<&WeatherTimestepSample> {
        let timestep_index = timestep.checked_sub(1)? as usize;
        if timestep_index >= self.zone_steps_per_hour as usize {
            return None;
        }
        self.timestep_samples
            .get(record_index * self.zone_steps_per_hour as usize + timestep_index)
    }
}

/// Precomputes weather samples for the supplied timestep configuration.
#[must_use]
pub fn precompute_weather_timestep_series(
    records: &[EpwRecord],
    zone_steps_per_hour: u32,
    first_hour_interpolation_starting_values: FirstHourInterpolationStartingValues,
) -> WeatherTimestepSeries {
    WeatherTimestepSeries::from_records(
        records,
        zone_steps_per_hour,
        first_hour_interpolation_starting_values,
    )
}

pub(crate) fn previous_weather_record(records: &[EpwRecord], record_index: usize) -> &EpwRecord {
    previous_weather_record_with_first_hour_starting_values(
        records,
        record_index,
        FirstHourInterpolationStartingValues::Hour24,
    )
}

pub(crate) fn previous_weather_record_with_first_hour_starting_values(
    records: &[EpwRecord],
    record_index: usize,
    first_hour_interpolation_starting_values: FirstHourInterpolationStartingValues,
) -> &EpwRecord {
    if record_index == 0 {
        let first_day_record_index = match first_hour_interpolation_starting_values {
            FirstHourInterpolationStartingValues::Hour1 => 0,
            FirstHourInterpolationStartingValues::Hour24 => records.len().min(24) - 1,
        };
        &records[first_day_record_index]
    } else {
        &records[record_index - 1]
    }
}

pub(crate) fn next_weather_record(records: &[EpwRecord], record_index: usize) -> &EpwRecord {
    let next_index = if record_index + 1 >= records.len() {
        0
    } else {
        record_index + 1
    };
    &records[next_index]
}

pub(crate) fn heat_balance_weather_context_for_timestep(
    weather_series: Option<&WeatherTimestepSeries>,
    record_index: usize,
    zone_steps_per_hour: u32,
    zone_timestep: u32,
    first_hour_interpolation_starting_values: FirstHourInterpolationStartingValues,
) -> Option<HeatBalanceWeatherContext<'_>> {
    weather_series.map(|series| HeatBalanceWeatherContext {
        records: series.hourly_records(),
        sample: series.sample_for(record_index, zone_timestep),
        record_index,
        zone_steps_per_hour,
        zone_timestep: Some(zone_timestep),
        first_hour_interpolation_starting_values,
    })
}

#[cfg(test)]
pub(crate) fn energyplus_weather_dry_bulb_at_timestep(
    weather_records: Option<&[EpwRecord]>,
    record_index: usize,
    fallback_hourly_dry_bulb_c: f64,
    zone_steps_per_hour: u32,
    zone_timestep: u32,
) -> f64 {
    energyplus_weather_dry_bulb_at_timestep_with_starting_values(
        weather_records,
        record_index,
        fallback_hourly_dry_bulb_c,
        zone_steps_per_hour,
        zone_timestep,
        FirstHourInterpolationStartingValues::Hour24,
    )
}

pub(crate) fn energyplus_weather_dry_bulb_at_timestep_with_starting_values(
    weather_records: Option<&[EpwRecord]>,
    record_index: usize,
    fallback_hourly_dry_bulb_c: f64,
    zone_steps_per_hour: u32,
    zone_timestep: u32,
    first_hour_interpolation_starting_values: FirstHourInterpolationStartingValues,
) -> f64 {
    let Some(records) = weather_records else {
        return fallback_hourly_dry_bulb_c;
    };
    energyplus_weather_scalar_at_timestep(
        records,
        record_index,
        fallback_hourly_dry_bulb_c,
        zone_steps_per_hour,
        zone_timestep,
        first_hour_interpolation_starting_values,
        |record| record.dry_bulb_c,
    )
}

pub(crate) fn energyplus_weather_scalar_at_timestep(
    records: &[EpwRecord],
    record_index: usize,
    fallback_hourly_value: f64,
    zone_steps_per_hour: u32,
    zone_timestep: u32,
    first_hour_interpolation_starting_values: FirstHourInterpolationStartingValues,
    value: impl Fn(&EpwRecord) -> f64,
) -> f64 {
    let Some(record) = records.get(record_index) else {
        return fallback_hourly_value;
    };
    let previous = previous_weather_record_with_first_hour_starting_values(
        records,
        record_index,
        first_hour_interpolation_starting_values,
    );
    let interpolation_weight =
        energyplus_weather_interpolation_weight(zone_steps_per_hour, zone_timestep);

    value(previous) * (1.0 - interpolation_weight) + value(record) * interpolation_weight
}

pub(crate) fn energyplus_weather_relative_humidity_for_context(
    context: HeatBalanceWeatherContext<'_>,
    fallback_relative_humidity_percent: f64,
) -> f64 {
    if let Some(sample) = context.sample {
        return sample.relative_humidity_percent;
    }
    let Some(timestep) = context.zone_timestep else {
        return fallback_relative_humidity_percent;
    };

    energyplus_weather_relative_humidity_at_timestep_with_starting_values(
        context.records,
        context.record_index,
        fallback_relative_humidity_percent,
        context.zone_steps_per_hour,
        timestep,
        context.first_hour_interpolation_starting_values,
    )
}

#[cfg(test)]
pub(crate) fn energyplus_weather_relative_humidity_at_timestep(
    records: &[EpwRecord],
    record_index: usize,
    fallback_relative_humidity_percent: f64,
    zone_steps_per_hour: u32,
    zone_timestep: u32,
) -> f64 {
    energyplus_weather_relative_humidity_at_timestep_with_starting_values(
        records,
        record_index,
        fallback_relative_humidity_percent,
        zone_steps_per_hour,
        zone_timestep,
        FirstHourInterpolationStartingValues::Hour24,
    )
}

pub(crate) fn energyplus_weather_relative_humidity_at_timestep_with_starting_values(
    records: &[EpwRecord],
    record_index: usize,
    fallback_relative_humidity_percent: f64,
    zone_steps_per_hour: u32,
    zone_timestep: u32,
    first_hour_interpolation_starting_values: FirstHourInterpolationStartingValues,
) -> f64 {
    energyplus_weather_scalar_at_timestep(
        records,
        record_index,
        fallback_relative_humidity_percent,
        zone_steps_per_hour,
        zone_timestep,
        first_hour_interpolation_starting_values,
        |record| record.relative_humidity_percent,
    )
}

pub(crate) fn energyplus_weather_atmospheric_pressure_for_context(
    context: HeatBalanceWeatherContext<'_>,
    fallback_atmospheric_pressure_pa: f64,
) -> f64 {
    if let Some(sample) = context.sample {
        return sample.atmospheric_pressure_pa;
    }
    let Some(timestep) = context.zone_timestep else {
        return fallback_atmospheric_pressure_pa;
    };

    energyplus_weather_atmospheric_pressure_at_timestep_with_starting_values(
        context.records,
        context.record_index,
        fallback_atmospheric_pressure_pa,
        context.zone_steps_per_hour,
        timestep,
        context.first_hour_interpolation_starting_values,
    )
}

#[cfg(test)]
pub(crate) fn energyplus_weather_atmospheric_pressure_at_timestep(
    records: &[EpwRecord],
    record_index: usize,
    fallback_atmospheric_pressure_pa: f64,
    zone_steps_per_hour: u32,
    zone_timestep: u32,
) -> f64 {
    energyplus_weather_atmospheric_pressure_at_timestep_with_starting_values(
        records,
        record_index,
        fallback_atmospheric_pressure_pa,
        zone_steps_per_hour,
        zone_timestep,
        FirstHourInterpolationStartingValues::Hour24,
    )
}

pub(crate) fn energyplus_weather_atmospheric_pressure_at_timestep_with_starting_values(
    records: &[EpwRecord],
    record_index: usize,
    fallback_atmospheric_pressure_pa: f64,
    zone_steps_per_hour: u32,
    zone_timestep: u32,
    first_hour_interpolation_starting_values: FirstHourInterpolationStartingValues,
) -> f64 {
    energyplus_weather_scalar_at_timestep(
        records,
        record_index,
        fallback_atmospheric_pressure_pa,
        zone_steps_per_hour,
        zone_timestep,
        first_hour_interpolation_starting_values,
        |record| record.atmospheric_pressure_pa,
    )
}

pub(crate) fn weather_context_outdoor_humidity_ratio(
    context: HeatBalanceWeatherContext<'_>,
    fallback_dry_bulb_c: f64,
) -> Option<f64> {
    if let Some(sample) = context.sample {
        return Some(sample.outdoor_humidity_ratio);
    }
    let record = context.records.get(context.record_index)?;
    let dry_bulb_c = context
        .zone_timestep
        .map(|timestep| {
            energyplus_weather_dry_bulb_at_timestep_with_starting_values(
                Some(context.records),
                context.record_index,
                fallback_dry_bulb_c,
                context.zone_steps_per_hour,
                timestep,
                context.first_hour_interpolation_starting_values,
            )
        })
        .unwrap_or(fallback_dry_bulb_c);
    let relative_humidity_percent =
        energyplus_weather_relative_humidity_for_context(context, record.relative_humidity_percent);
    let atmospheric_pressure_pa = energyplus_weather_atmospheric_pressure_for_context(
        context,
        record.atmospheric_pressure_pa,
    );

    energyplus_psychrometric_humidity_ratio_from_rh(
        dry_bulb_c,
        (relative_humidity_percent * 0.01).clamp(0.0, 1.0),
        atmospheric_pressure_pa,
    )
}

pub(crate) fn energyplus_weather_horizontal_infrared_for_context(
    context: HeatBalanceWeatherContext<'_>,
    fallback_hourly_horizontal_infrared_w_per_m2: f64,
) -> f64 {
    if let Some(sample) = context.sample {
        return sample.horizontal_infrared_radiation_w_per_m2;
    }
    let Some(timestep) = context.zone_timestep else {
        return fallback_hourly_horizontal_infrared_w_per_m2;
    };

    energyplus_weather_horizontal_infrared_at_timestep_with_starting_values(
        context.records,
        context.record_index,
        fallback_hourly_horizontal_infrared_w_per_m2,
        context.zone_steps_per_hour,
        timestep,
        context.first_hour_interpolation_starting_values,
    )
}

#[cfg(test)]
pub(crate) fn energyplus_weather_horizontal_infrared_at_timestep(
    records: &[EpwRecord],
    record_index: usize,
    fallback_hourly_horizontal_infrared_w_per_m2: f64,
    zone_steps_per_hour: u32,
    zone_timestep: u32,
) -> f64 {
    energyplus_weather_horizontal_infrared_at_timestep_with_starting_values(
        records,
        record_index,
        fallback_hourly_horizontal_infrared_w_per_m2,
        zone_steps_per_hour,
        zone_timestep,
        FirstHourInterpolationStartingValues::Hour24,
    )
}

pub(crate) fn energyplus_weather_horizontal_infrared_at_timestep_with_starting_values(
    records: &[EpwRecord],
    record_index: usize,
    fallback_hourly_horizontal_infrared_w_per_m2: f64,
    zone_steps_per_hour: u32,
    zone_timestep: u32,
    first_hour_interpolation_starting_values: FirstHourInterpolationStartingValues,
) -> f64 {
    energyplus_weather_scalar_at_timestep(
        records,
        record_index,
        fallback_hourly_horizontal_infrared_w_per_m2,
        zone_steps_per_hour,
        zone_timestep,
        first_hour_interpolation_starting_values,
        |record| record.horizontal_infrared_radiation_wh_per_m2,
    )
}

pub(crate) fn energyplus_weather_global_horizontal_radiation_at_timestep_with_starting_values(
    records: &[EpwRecord],
    record_index: usize,
    fallback_hourly_global_horizontal_radiation_w_per_m2: f64,
    zone_steps_per_hour: u32,
    zone_timestep: u32,
    first_hour_interpolation_starting_values: FirstHourInterpolationStartingValues,
) -> f64 {
    energyplus_weather_scalar_at_timestep(
        records,
        record_index,
        fallback_hourly_global_horizontal_radiation_w_per_m2,
        zone_steps_per_hour,
        zone_timestep,
        first_hour_interpolation_starting_values,
        |record| record.global_horizontal_radiation_wh_per_m2,
    )
}

pub(crate) fn energyplus_weather_direct_normal_radiation_at_timestep_with_starting_values(
    records: &[EpwRecord],
    record_index: usize,
    fallback_hourly_direct_normal_radiation_w_per_m2: f64,
    zone_steps_per_hour: u32,
    zone_timestep: u32,
    first_hour_interpolation_starting_values: FirstHourInterpolationStartingValues,
) -> f64 {
    energyplus_weather_scalar_at_timestep(
        records,
        record_index,
        fallback_hourly_direct_normal_radiation_w_per_m2,
        zone_steps_per_hour,
        zone_timestep,
        first_hour_interpolation_starting_values,
        |record| record.direct_normal_radiation_wh_per_m2,
    )
}

pub(crate) fn energyplus_weather_diffuse_horizontal_radiation_at_timestep_with_starting_values(
    records: &[EpwRecord],
    record_index: usize,
    fallback_hourly_diffuse_horizontal_radiation_w_per_m2: f64,
    zone_steps_per_hour: u32,
    zone_timestep: u32,
    first_hour_interpolation_starting_values: FirstHourInterpolationStartingValues,
) -> f64 {
    energyplus_weather_scalar_at_timestep(
        records,
        record_index,
        fallback_hourly_diffuse_horizontal_radiation_w_per_m2,
        zone_steps_per_hour,
        zone_timestep,
        first_hour_interpolation_starting_values,
        |record| record.diffuse_horizontal_radiation_wh_per_m2,
    )
}

pub(crate) fn energyplus_weather_liquid_precipitation_at_timestep_with_starting_values(
    records: &[EpwRecord],
    record_index: usize,
    fallback_hourly_liquid_precipitation_depth_mm: f64,
    zone_steps_per_hour: u32,
    zone_timestep: u32,
    first_hour_interpolation_starting_values: FirstHourInterpolationStartingValues,
) -> f64 {
    energyplus_weather_scalar_at_timestep(
        records,
        record_index,
        fallback_hourly_liquid_precipitation_depth_mm,
        zone_steps_per_hour,
        zone_timestep,
        first_hour_interpolation_starting_values,
        |record| record.liquid_precipitation_depth_mm,
    )
}

pub(crate) fn energyplus_weather_wind_speed_for_context(
    context: HeatBalanceWeatherContext<'_>,
    fallback_hourly_wind_speed_m_per_s: f64,
) -> f64 {
    if let Some(sample) = context.sample {
        return sample.wind_speed_m_per_s;
    }
    let Some(timestep) = context.zone_timestep else {
        return fallback_hourly_wind_speed_m_per_s;
    };

    energyplus_weather_wind_speed_at_timestep_with_starting_values(
        context.records,
        context.record_index,
        fallback_hourly_wind_speed_m_per_s,
        context.zone_steps_per_hour,
        timestep,
        context.first_hour_interpolation_starting_values,
    )
}

#[cfg(test)]
pub(crate) fn energyplus_weather_wind_speed_at_timestep(
    records: &[EpwRecord],
    record_index: usize,
    fallback_hourly_wind_speed_m_per_s: f64,
    zone_steps_per_hour: u32,
    zone_timestep: u32,
) -> f64 {
    energyplus_weather_wind_speed_at_timestep_with_starting_values(
        records,
        record_index,
        fallback_hourly_wind_speed_m_per_s,
        zone_steps_per_hour,
        zone_timestep,
        FirstHourInterpolationStartingValues::Hour24,
    )
}

pub(crate) fn energyplus_weather_wind_speed_at_timestep_with_starting_values(
    records: &[EpwRecord],
    record_index: usize,
    fallback_hourly_wind_speed_m_per_s: f64,
    zone_steps_per_hour: u32,
    zone_timestep: u32,
    first_hour_interpolation_starting_values: FirstHourInterpolationStartingValues,
) -> f64 {
    energyplus_weather_scalar_at_timestep(
        records,
        record_index,
        fallback_hourly_wind_speed_m_per_s,
        zone_steps_per_hour,
        zone_timestep,
        first_hour_interpolation_starting_values,
        |record| record.wind_speed_m_per_s,
    )
}

pub(crate) fn energyplus_weather_wind_direction_for_context(
    context: HeatBalanceWeatherContext<'_>,
    fallback_hourly_wind_direction_deg: f64,
) -> f64 {
    if let Some(sample) = context.sample {
        return sample.wind_direction_deg;
    }
    let Some(timestep) = context.zone_timestep else {
        return fallback_hourly_wind_direction_deg;
    };

    energyplus_weather_wind_direction_at_timestep_with_starting_values(
        context.records,
        context.record_index,
        fallback_hourly_wind_direction_deg,
        context.zone_steps_per_hour,
        timestep,
        context.first_hour_interpolation_starting_values,
    )
}

#[cfg(test)]
pub(crate) fn energyplus_weather_wind_direction_at_timestep(
    records: &[EpwRecord],
    record_index: usize,
    fallback_hourly_wind_direction_deg: f64,
    zone_steps_per_hour: u32,
    zone_timestep: u32,
) -> f64 {
    energyplus_weather_wind_direction_at_timestep_with_starting_values(
        records,
        record_index,
        fallback_hourly_wind_direction_deg,
        zone_steps_per_hour,
        zone_timestep,
        FirstHourInterpolationStartingValues::Hour24,
    )
}

pub(crate) fn energyplus_weather_wind_direction_at_timestep_with_starting_values(
    records: &[EpwRecord],
    record_index: usize,
    fallback_hourly_wind_direction_deg: f64,
    zone_steps_per_hour: u32,
    zone_timestep: u32,
    first_hour_interpolation_starting_values: FirstHourInterpolationStartingValues,
) -> f64 {
    let Some(record) = records.get(record_index) else {
        return fallback_hourly_wind_direction_deg;
    };
    let previous = previous_weather_record_with_first_hour_starting_values(
        records,
        record_index,
        first_hour_interpolation_starting_values,
    );
    let interpolation_weight =
        energyplus_weather_interpolation_weight(zone_steps_per_hour, zone_timestep);

    energyplus_interpolate_wind_direction_deg(
        previous.wind_direction_deg,
        record.wind_direction_deg,
        interpolation_weight,
    )
}

fn energyplus_interpolate_wind_direction_deg(
    previous_wind_direction_deg: f64,
    current_wind_direction_deg: f64,
    current_hour_weight: f64,
) -> f64 {
    let mut current = current_wind_direction_deg;
    let mut previous = previous_wind_direction_deg;
    if (current - previous).abs() > 180.0 {
        if current > previous {
            previous += 360.0;
        } else {
            current += 360.0;
        }
    }

    (previous + (current - previous) * current_hour_weight).rem_euclid(360.0)
}

pub(crate) fn energyplus_weather_interpolation_weight(
    zone_steps_per_hour: u32,
    zone_timestep: u32,
) -> f64 {
    let steps = zone_steps_per_hour.max(1);
    if steps == 1 {
        return 1.0;
    }

    (f64::from(zone_timestep.clamp(1, steps)) / f64::from(steps)).min(1.0)
}

/// Loads hourly EPW records from a weather file.
pub fn load_epw_records(path: impl AsRef<Path>) -> Result<Vec<EpwRecord>, EpwError> {
    let contents = std::fs::read_to_string(path)?;
    parse_epw_records(&contents)
}

/// Parses hourly EPW records from weather text.
pub fn parse_epw_records(contents: &str) -> Result<Vec<EpwRecord>, EpwError> {
    let mut records = Vec::new();

    for (line_index, line) in contents.lines().enumerate().skip(8) {
        let line_number = line_index + 1;
        if line.trim().is_empty() {
            continue;
        }
        let fields = line.split(',').collect::<Vec<_>>();
        records.push(EpwRecord {
            year: parse_epw_u32(&fields, line_number, 0, "year")?,
            month: parse_epw_u32(&fields, line_number, 1, "month")?,
            day: parse_epw_u32(&fields, line_number, 2, "day")?,
            hour: parse_epw_u32(&fields, line_number, 3, "hour")?,
            minute: parse_epw_u32(&fields, line_number, 4, "minute")?,
            dry_bulb_c: parse_epw_f64(&fields, line_number, 6, "dry-bulb")?,
            dew_point_c: parse_epw_f64(&fields, line_number, 7, "dew-point")?,
            relative_humidity_percent: parse_epw_f64(&fields, line_number, 8, "relative humidity")?,
            atmospheric_pressure_pa: parse_epw_f64(
                &fields,
                line_number,
                9,
                "atmospheric pressure",
            )?,
            horizontal_infrared_radiation_wh_per_m2: parse_epw_f64(
                &fields,
                line_number,
                12,
                "horizontal infrared radiation",
            )?,
            global_horizontal_radiation_wh_per_m2: parse_epw_f64(
                &fields,
                line_number,
                13,
                "global horizontal radiation",
            )?,
            direct_normal_radiation_wh_per_m2: parse_epw_f64(
                &fields,
                line_number,
                14,
                "direct normal radiation",
            )?,
            diffuse_horizontal_radiation_wh_per_m2: parse_epw_f64(
                &fields,
                line_number,
                15,
                "diffuse horizontal radiation",
            )?,
            wind_direction_deg: parse_epw_f64(&fields, line_number, 20, "wind direction")?,
            wind_speed_m_per_s: parse_epw_f64(&fields, line_number, 21, "wind speed")?,
            liquid_precipitation_depth_mm: parse_epw_liquid_precipitation_depth_mm(&fields, 33),
        });
    }

    Ok(records)
}

/// Loads hourly outdoor dry-bulb values from an EPW file.
pub fn load_epw_dry_bulb_series(path: impl AsRef<Path>) -> Result<Vec<f64>, EpwError> {
    let contents = std::fs::read_to_string(path)?;
    parse_epw_dry_bulb_series(&contents)
}

/// Parses hourly outdoor dry-bulb values from EPW text.
pub fn parse_epw_dry_bulb_series(contents: &str) -> Result<Vec<f64>, EpwError> {
    parse_epw_records(contents).map(|records| {
        records
            .into_iter()
            .map(|record| record.dry_bulb_c)
            .collect()
    })
}

fn parse_epw_u32(
    fields: &[&str],
    line: usize,
    index: usize,
    field: &'static str,
) -> Result<u32, EpwError> {
    let value = epw_field(fields, line, index, field)?;
    value
        .trim()
        .parse::<u32>()
        .map_err(|_error| EpwError::InvalidNumber {
            line,
            field,
            value: value.to_string(),
        })
}

fn parse_epw_f64(
    fields: &[&str],
    line: usize,
    index: usize,
    field: &'static str,
) -> Result<f64, EpwError> {
    let value = epw_field(fields, line, index, field)?;
    value
        .trim()
        .parse::<f64>()
        .map_err(|_error| EpwError::InvalidNumber {
            line,
            field,
            value: value.to_string(),
        })
}

fn parse_epw_optional_f64_default(fields: &[&str], index: usize, default: f64) -> f64 {
    let Some(value) = fields.get(index).map(|value| value.trim()) else {
        return default;
    };
    if value.is_empty() {
        default
    } else {
        value.parse::<f64>().unwrap_or(default)
    }
}

fn parse_epw_liquid_precipitation_depth_mm(fields: &[&str], index: usize) -> f64 {
    let value = parse_epw_optional_f64_default(fields, index, 0.0);
    if value >= 99.0 { 0.0 } else { value.max(0.0) }
}

fn epw_field<'a>(
    fields: &'a [&str],
    line: usize,
    index: usize,
    field: &'static str,
) -> Result<&'a str, EpwError> {
    fields
        .get(index)
        .copied()
        .ok_or(EpwError::MissingField { line, field })
}
