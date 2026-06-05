//! Comparison and tolerance helpers.

use std::fmt::{Display, Formatter};
use std::path::Path;

/// Absolute and relative tolerance policy.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Tolerance {
    /// Absolute tolerance.
    pub absolute: f64,
    /// Relative tolerance.
    pub relative: f64,
}

impl Tolerance {
    /// Returns true when two values are within tolerance.
    #[must_use]
    pub fn accepts(self, expected: f64, observed: f64) -> bool {
        let delta = (expected - observed).abs();
        if delta <= self.absolute {
            return true;
        }

        let scale = expected.abs().max(observed.abs());
        delta <= self.relative * scale
    }
}

impl Default for Tolerance {
    fn default() -> Self {
        Self {
            absolute: 1.0e-9,
            relative: 1.0e-6,
        }
    }
}

/// Comparison summary for two numeric series.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SeriesComparison {
    /// Number of compared samples.
    pub samples: usize,
    /// Maximum absolute difference.
    pub max_abs_delta: f64,
    /// True when every sample is within tolerance.
    pub passed: bool,
}

/// Compares two equally-sized numeric series.
#[must_use]
pub fn compare_series(
    expected: &[f64],
    observed: &[f64],
    tolerance: Tolerance,
) -> SeriesComparison {
    let mut max_abs_delta: f64 = 0.0;
    let mut passed = expected.len() == observed.len();

    for (left, right) in expected.iter().zip(observed) {
        let delta = (left - right).abs();
        max_abs_delta = max_abs_delta.max(delta);
        if !tolerance.accepts(*left, *right) {
            passed = false;
        }
    }

    SeriesComparison {
        samples: expected.len().min(observed.len()),
        max_abs_delta,
        passed,
    }
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

#[cfg(test)]
mod tests {
    use super::{Tolerance, compare_series, parse_eso_series};

    #[test]
    fn tolerance_accepts_close_values() {
        let tolerance = Tolerance::default();

        assert!(tolerance.accepts(1.0, 1.0 + 1.0e-10));
        assert!(!tolerance.accepts(1.0, 1.1));
    }

    #[test]
    fn parses_eso_series_by_key_and_variable() -> Result<(), Box<dyn std::error::Error>> {
        let values = parse_eso_series(
            r#"Program Version,EnergyPlus
1,5,Environment Title[],Latitude[deg],Longitude[deg],Time Zone[],Elevation[m]
494,1,ALWAYSON,Schedule Value [] !Hourly
End of Data Dictionary
1,Run Period,39.74,-105.18,-7.00,1829.00
494,1.0
494,1.0
"#,
            "AlwaysOn",
            "Schedule Value",
        )?;

        assert_eq!(values, vec![1.0, 1.0]);

        Ok(())
    }

    #[test]
    fn series_comparison_tracks_max_delta() {
        let result = compare_series(&[1.0, 2.0], &[1.0, 2.000_000_000_1], Tolerance::default());

        assert!(result.passed);
        assert_eq!(result.samples, 2);
        assert!(result.max_abs_delta > 0.0);
    }
}
