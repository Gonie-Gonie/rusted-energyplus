//! EnergyPlus MTR selected-meter reader.

use std::fmt::{Display, Formatter};
use std::path::Path;

use crate::SeriesSample;

/// Metadata for one selected EnergyPlus MTR time series.
#[derive(Clone, Debug, PartialEq)]
pub struct MtrSeriesMetadata {
    /// MTR dictionary row id for this meter.
    pub id: String,
    /// EnergyPlus meter name without units or frequency suffix.
    pub meter: String,
    /// Parsed units from the MTR dictionary, if present and non-empty.
    pub units: Option<String>,
    /// Parsed reporting frequency, such as `Hourly`, if present.
    pub frequency: Option<String>,
}

/// One selected EnergyPlus MTR series with optional timestamp labels per sample.
#[derive(Clone, Debug, PartialEq)]
pub struct MtrTimeSeries {
    /// Selected meter dictionary metadata.
    pub metadata: MtrSeriesMetadata,
    /// Numeric samples in file order.
    pub samples: Vec<SeriesSample>,
}

/// Error returned while reading an EnergyPlus MTR output.
#[derive(Debug)]
pub enum MtrError {
    /// File read failed.
    Io(std::io::Error),
    /// Requested meter was not present in the MTR dictionary.
    MissingMeter {
        /// EnergyPlus meter name.
        meter: String,
    },
    /// A matching data row could not be parsed.
    InvalidValue {
        /// One-based line number.
        line: usize,
        /// Raw line text.
        text: String,
    },
}

impl Display for MtrError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "failed to read MTR: {error}"),
            Self::MissingMeter { meter } => write!(formatter, "MTR meter not found: {meter}"),
            Self::InvalidValue { line, text } => {
                write!(formatter, "invalid MTR value at line {line}: {text}")
            }
        }
    }
}

impl std::error::Error for MtrError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::MissingMeter { .. } | Self::InvalidValue { .. } => None,
        }
    }
}

impl From<std::io::Error> for MtrError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

/// Loads one timestamp-aware MTR series by meter name.
pub fn load_mtr_time_series(
    path: impl AsRef<Path>,
    meter: &str,
) -> Result<MtrTimeSeries, MtrError> {
    let contents = std::fs::read_to_string(path)?;
    parse_mtr_time_series(&contents, meter)
}

/// Parses one timestamp-aware MTR series by meter name.
pub fn parse_mtr_time_series(contents: &str, meter: &str) -> Result<MtrTimeSeries, MtrError> {
    let normalized_meter = normalize_name(meter);
    let mut dictionary_done = false;
    let mut selected_metadata = None;
    let mut samples = Vec::new();
    let mut current_environment = None;
    let mut current_timestamp = None;

    for (line_index, line) in contents.lines().enumerate() {
        let line_number = line_index + 1;
        let trimmed = line.trim();
        if trimmed.eq_ignore_ascii_case("End of Data Dictionary") {
            dictionary_done = true;
            continue;
        }

        if !dictionary_done {
            if let Some(metadata) = matching_dictionary_metadata(trimmed, &normalized_meter) {
                selected_metadata = Some(metadata);
            }
            continue;
        }

        let Some((row_id, value_text)) = trimmed.split_once(',') else {
            continue;
        };
        let row_id = row_id.trim();
        if row_id == "1" {
            current_environment = value_text
                .split(',')
                .next()
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(str::to_string);
            continue;
        }
        if row_id == "2" {
            current_timestamp = Some(hourly_timestamp_label(
                current_environment.as_deref(),
                value_text,
            ));
            continue;
        }

        let Some(metadata) = selected_metadata.as_ref() else {
            continue;
        };
        if row_id != metadata.id {
            continue;
        }
        let Some(first_value) = value_text.split(',').next() else {
            return Err(MtrError::InvalidValue {
                line: line_number,
                text: line.to_string(),
            });
        };
        let Ok(value) = first_value.trim().parse::<f64>() else {
            return Err(MtrError::InvalidValue {
                line: line_number,
                text: line.to_string(),
            });
        };
        samples.push(SeriesSample {
            index: samples.len(),
            timestamp: current_timestamp.clone(),
            value,
        });
    }

    let Some(metadata) = selected_metadata else {
        return Err(MtrError::MissingMeter {
            meter: meter.to_string(),
        });
    };

    Ok(MtrTimeSeries { metadata, samples })
}

fn matching_dictionary_metadata(line: &str, normalized_meter: &str) -> Option<MtrSeriesMetadata> {
    let mut parts = line.splitn(3, ',');
    let id = parts.next()?.trim();
    let _value_count = parts.next()?;
    let meter_text = parts.next()?.trim();
    let (meter, units, frequency) = parse_meter_metadata(meter_text);
    if normalize_name(&meter) == normalized_meter {
        return Some(MtrSeriesMetadata {
            id: id.to_string(),
            meter,
            units,
            frequency,
        });
    }

    None
}

fn parse_meter_metadata(text: &str) -> (String, Option<String>, Option<String>) {
    let (value_text, frequency_text) = match text.split_once('!') {
        Some((left, right)) => (left.trim(), Some(right.trim())),
        None => (text.trim(), None),
    };
    let frequency = frequency_text
        .and_then(|value| value.split_whitespace().next())
        .filter(|value| !value.is_empty())
        .map(str::to_string);

    let Some(open_index) = value_text.rfind('[') else {
        return (value_text.to_string(), None, frequency);
    };
    let Some(close_offset) = value_text[open_index..].find(']') else {
        return (value_text.to_string(), None, frequency);
    };
    let close_index = open_index + close_offset;
    let meter = value_text[..open_index].trim().to_string();
    let units = value_text[(open_index + 1)..close_index].trim().to_string();
    let units = if units.is_empty() { None } else { Some(units) };

    (meter, units, frequency)
}

fn hourly_timestamp_label(environment: Option<&str>, value_text: &str) -> String {
    let fields = value_text.split(',').map(str::trim).collect::<Vec<_>>();
    let field = |index: usize| fields.get(index).copied().unwrap_or("");
    format!(
        "env={};day={};month={};date={};dst={};hour={};start={};end={};day_type={}",
        environment.unwrap_or(""),
        field(0),
        field(1),
        field(2),
        field(3),
        field(4),
        field(5),
        field(6),
        field(7)
    )
}

fn normalize_name(value: &str) -> String {
    value.trim().to_ascii_uppercase()
}
