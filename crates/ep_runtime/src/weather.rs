//! EPW weather parsing helpers.

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
    pub(crate) record_index: usize,
    pub(crate) zone_steps_per_hour: u32,
    pub(crate) zone_timestep: Option<u32>,
    pub(crate) first_hour_interpolation_starting_values: FirstHourInterpolationStartingValues,
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
    weather_records: Option<&[EpwRecord]>,
    record_index: usize,
    zone_steps_per_hour: u32,
    zone_timestep: u32,
    first_hour_interpolation_starting_values: FirstHourInterpolationStartingValues,
) -> Option<HeatBalanceWeatherContext<'_>> {
    weather_records.map(|records| HeatBalanceWeatherContext {
        records,
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
