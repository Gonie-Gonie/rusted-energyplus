//! EnergyPlus ESO selected-series reader.

use std::fmt::{Display, Formatter};
use std::path::Path;

use crate::SeriesSample;

/// Metadata for one selected EnergyPlus ESO time series.
#[derive(Clone, Debug, PartialEq)]
pub struct EsoSeriesMetadata {
    /// ESO dictionary row id for this series.
    pub id: String,
    /// EnergyPlus key value, such as a schedule, zone, or surface name.
    pub key: String,
    /// EnergyPlus output variable name without units or frequency suffix.
    pub variable: String,
    /// Parsed units from the ESO dictionary, if present and non-empty.
    pub units: Option<String>,
    /// Parsed reporting frequency, such as `Hourly`, if present.
    pub frequency: Option<String>,
}

/// One selected EnergyPlus ESO series with optional timestamp labels per sample.
#[derive(Clone, Debug, PartialEq)]
pub struct EsoTimeSeries {
    /// Selected series dictionary metadata.
    pub metadata: EsoSeriesMetadata,
    /// Numeric samples in file order.
    pub samples: Vec<SeriesSample>,
}

/// Error returned while reading an EnergyPlus ESO output.
#[derive(Debug)]
pub enum EsoError {
    /// File read failed.
    Io(std::io::Error),
    /// Requested variable was not present in the ESO dictionary.
    MissingSeries {
        /// EnergyPlus key value, such as schedule name.
        key: String,
        /// EnergyPlus output variable name.
        variable: String,
    },
    /// A matching data row could not be parsed.
    InvalidValue {
        /// One-based line number.
        line: usize,
        /// Raw line text.
        text: String,
    },
}

impl Display for EsoError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "failed to read ESO: {error}"),
            Self::MissingSeries { key, variable } => {
                write!(formatter, "ESO series not found: {key}/{variable}")
            }
            Self::InvalidValue { line, text } => {
                write!(formatter, "invalid ESO value at line {line}: {text}")
            }
        }
    }
}

impl std::error::Error for EsoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::MissingSeries { .. } | Self::InvalidValue { .. } => None,
        }
    }
}

impl From<std::io::Error> for EsoError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

/// Loads one numeric ESO series by key and variable name.
pub fn load_eso_series(
    path: impl AsRef<Path>,
    key: &str,
    variable: &str,
) -> Result<Vec<f64>, EsoError> {
    let time_series = load_eso_time_series(path, key, variable)?;
    Ok(time_series
        .samples
        .into_iter()
        .map(|sample| sample.value)
        .collect())
}

/// Loads one timestamp-aware ESO series by key and variable name.
pub fn load_eso_time_series(
    path: impl AsRef<Path>,
    key: &str,
    variable: &str,
) -> Result<EsoTimeSeries, EsoError> {
    let contents = std::fs::read_to_string(path)?;
    parse_eso_time_series(&contents, key, variable)
}

/// Parses one numeric ESO series by key and variable name.
pub fn parse_eso_series(contents: &str, key: &str, variable: &str) -> Result<Vec<f64>, EsoError> {
    let time_series = parse_eso_time_series(contents, key, variable)?;
    Ok(time_series
        .samples
        .into_iter()
        .map(|sample| sample.value)
        .collect())
}

/// Parses one timestamp-aware ESO series by key and variable name.
pub fn parse_eso_time_series(
    contents: &str,
    key: &str,
    variable: &str,
) -> Result<EsoTimeSeries, EsoError> {
    let normalized_key = normalize_key(key);
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
            if let Some(metadata) = matching_dictionary_metadata(trimmed, &normalized_key, variable)
            {
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
            return Err(EsoError::InvalidValue {
                line: line_number,
                text: line.to_string(),
            });
        };
        let Ok(value) = first_value.trim().parse::<f64>() else {
            return Err(EsoError::InvalidValue {
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
        return Err(EsoError::MissingSeries {
            key: key.to_string(),
            variable: variable.to_string(),
        });
    };

    Ok(EsoTimeSeries { metadata, samples })
}

fn matching_dictionary_metadata(
    line: &str,
    normalized_key: &str,
    variable: &str,
) -> Option<EsoSeriesMetadata> {
    let mut parts = line.splitn(4, ',');
    let id = parts.next()?.trim();
    let _value_count = parts.next()?;
    let key = parts.next()?.trim();
    let variable_text = parts.next()?.trim();
    let (variable_name, units, frequency) = parse_variable_metadata(variable_text);
    if normalize_key(key) == normalized_key && variable_name.eq_ignore_ascii_case(variable) {
        return Some(EsoSeriesMetadata {
            id: id.to_string(),
            key: key.to_string(),
            variable: variable_name,
            units,
            frequency,
        });
    }

    None
}

fn parse_variable_metadata(text: &str) -> (String, Option<String>, Option<String>) {
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
    let variable = value_text[..open_index].trim().to_string();
    let units = value_text[(open_index + 1)..close_index].trim().to_string();
    let units = if units.is_empty() { None } else { Some(units) };

    (variable, units, frequency)
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

fn normalize_key(value: &str) -> String {
    value.trim().to_ascii_uppercase()
}
