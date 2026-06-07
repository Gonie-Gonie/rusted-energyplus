//! EnergyPlus ESO selected-series reader.

use std::fmt::{Display, Formatter};
use std::path::Path;

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
    let contents = std::fs::read_to_string(path)?;
    parse_eso_series(&contents, key, variable)
}

/// Parses one numeric ESO series by key and variable name.
pub fn parse_eso_series(contents: &str, key: &str, variable: &str) -> Result<Vec<f64>, EsoError> {
    let normalized_key = normalize_key(key);
    let mut dictionary_done = false;
    let mut series_id = None;
    let mut values = Vec::new();

    for (line_index, line) in contents.lines().enumerate() {
        let line_number = line_index + 1;
        let trimmed = line.trim();
        if trimmed.eq_ignore_ascii_case("End of Data Dictionary") {
            dictionary_done = true;
            continue;
        }

        if !dictionary_done {
            if let Some(id) = matching_dictionary_id(trimmed, &normalized_key, variable) {
                series_id = Some(id);
            }
            continue;
        }

        let Some(id) = series_id.as_deref() else {
            continue;
        };
        let Some((row_id, value_text)) = trimmed.split_once(',') else {
            continue;
        };
        if row_id.trim() != id {
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
        values.push(value);
    }

    if series_id.is_none() {
        return Err(EsoError::MissingSeries {
            key: key.to_string(),
            variable: variable.to_string(),
        });
    }

    Ok(values)
}

fn matching_dictionary_id(line: &str, normalized_key: &str, variable: &str) -> Option<String> {
    let mut parts = line.splitn(4, ',');
    let id = parts.next()?.trim();
    let _value_count = parts.next()?;
    let key = parts.next()?.trim();
    let variable_text = parts.next()?.trim();
    if normalize_key(key) == normalized_key && variable_text.starts_with(variable) {
        return Some(id.to_string());
    }

    None
}

fn normalize_key(value: &str) -> String {
    value.trim().to_ascii_uppercase()
}
