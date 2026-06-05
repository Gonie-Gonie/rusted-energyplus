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
    /// First tolerance or length divergence, if any.
    pub first_divergence: Option<SeriesDivergence>,
    /// True when every sample is within tolerance.
    pub passed: bool,
}

/// First point where two numeric series diverged.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SeriesDivergence {
    /// Zero-based sample index where the divergence starts.
    pub index: usize,
    /// Expected value, absent when the expected series ended first.
    pub expected: Option<f64>,
    /// Observed value, absent when the observed series ended first.
    pub observed: Option<f64>,
    /// Absolute delta, absent for length-only divergence.
    pub abs_delta: Option<f64>,
}

/// Zone geometry values read from EnergyPlus `eplusout.eio`.
#[derive(Clone, Debug, PartialEq)]
pub struct EioZoneGeometry {
    /// EnergyPlus-normalized zone name.
    pub zone_name: String,
    /// EIO `Number of Surfaces`.
    pub surface_count: usize,
    /// EIO `Floor Area {m2}`.
    pub floor_area_m2: f64,
    /// EIO `Volume {m3}`.
    pub volume_m3: f64,
    /// EIO `Exterior Gross Wall Area {m2}`.
    pub exterior_gross_wall_area_m2: f64,
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
    let mut first_divergence = None;

    for (index, (left, right)) in expected.iter().zip(observed).enumerate() {
        let delta = (left - right).abs();
        max_abs_delta = max_abs_delta.max(delta);
        if !tolerance.accepts(*left, *right) {
            passed = false;
            if first_divergence.is_none() {
                first_divergence = Some(SeriesDivergence {
                    index,
                    expected: Some(*left),
                    observed: Some(*right),
                    abs_delta: Some(delta),
                });
            }
        }
    }

    let samples = expected.len().min(observed.len());
    if expected.len() != observed.len() && first_divergence.is_none() {
        first_divergence = Some(SeriesDivergence {
            index: samples,
            expected: expected.get(samples).copied(),
            observed: observed.get(samples).copied(),
            abs_delta: None,
        });
    }

    SeriesComparison {
        samples,
        max_abs_delta,
        first_divergence,
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

/// Error returned while reading EnergyPlus EIO tabular diagnostics.
#[derive(Debug)]
pub enum EioError {
    /// File read failed.
    Io(std::io::Error),
    /// No `Zone Information` rows were present.
    MissingZoneInformation,
    /// A `Zone Information` row could not be parsed.
    InvalidZoneInformation {
        /// One-based line number.
        line: usize,
        /// Raw line text.
        text: String,
        /// Parse failure reason.
        reason: String,
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

impl Display for EioError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "failed to read EIO: {error}"),
            Self::MissingZoneInformation => write!(formatter, "EIO Zone Information not found"),
            Self::InvalidZoneInformation { line, text, reason } => write!(
                formatter,
                "invalid EIO Zone Information at line {line}: {reason}: {text}"
            ),
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

impl std::error::Error for EioError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::MissingZoneInformation | Self::InvalidZoneInformation { .. } => None,
        }
    }
}

impl From<std::io::Error> for EsoError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<std::io::Error> for EioError {
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

/// Loads zone geometry rows from an EnergyPlus EIO file.
pub fn load_eio_zone_geometry(path: impl AsRef<Path>) -> Result<Vec<EioZoneGeometry>, EioError> {
    let contents = std::fs::read_to_string(path)?;
    parse_eio_zone_geometry(&contents)
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

/// Parses `Zone Information` rows from EnergyPlus EIO contents.
pub fn parse_eio_zone_geometry(contents: &str) -> Result<Vec<EioZoneGeometry>, EioError> {
    let mut zones = Vec::new();
    for (line_index, line) in contents.lines().enumerate() {
        let line_number = line_index + 1;
        let trimmed = line.trim();
        if !trimmed.starts_with("Zone Information,") {
            continue;
        }

        let fields = trimmed.split(',').map(str::trim).collect::<Vec<_>>();
        if fields.len() <= 26 {
            return Err(EioError::InvalidZoneInformation {
                line: line_number,
                text: line.to_string(),
                reason: format!("expected at least 27 fields, found {}", fields.len()),
            });
        }

        zones.push(EioZoneGeometry {
            zone_name: required_field(&fields, 1).to_ascii_uppercase(),
            volume_m3: parse_f64_field(&fields, 19, line_number, line, "Volume {m3}")?,
            floor_area_m2: parse_f64_field(&fields, 22, line_number, line, "Floor Area {m2}")?,
            exterior_gross_wall_area_m2: parse_f64_field(
                &fields,
                23,
                line_number,
                line,
                "Exterior Gross Wall Area {m2}",
            )?,
            surface_count: parse_usize_field(&fields, 26, line_number, line, "Number of Surfaces")?,
        });
    }

    if zones.is_empty() {
        return Err(EioError::MissingZoneInformation);
    }

    Ok(zones)
}

fn required_field<'a>(fields: &'a [&str], index: usize) -> &'a str {
    fields.get(index).copied().unwrap_or("")
}

fn parse_f64_field(
    fields: &[&str],
    index: usize,
    line: usize,
    text: &str,
    field: &str,
) -> Result<f64, EioError> {
    required_field(fields, index)
        .parse::<f64>()
        .map_err(|_error| EioError::InvalidZoneInformation {
            line,
            text: text.to_string(),
            reason: format!("invalid {field}"),
        })
}

fn parse_usize_field(
    fields: &[&str],
    index: usize,
    line: usize,
    text: &str,
    field: &str,
) -> Result<usize, EioError> {
    required_field(fields, index)
        .parse::<usize>()
        .map_err(|_error| EioError::InvalidZoneInformation {
            line,
            text: text.to_string(),
            reason: format!("invalid {field}"),
        })
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
    use super::{Tolerance, compare_series, parse_eio_zone_geometry, parse_eso_series};

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
    fn parses_eio_zone_geometry_rows() -> Result<(), Box<dyn std::error::Error>> {
        let zones = parse_eio_zone_geometry(
            r#"! <Zone Information>,Zone Name,...
 Zone Information, ZONE ONE,0.0,0.00,0.00,0.00,7.62,7.62,2.29,1,1,1,0.00,15.24,0.00,15.24,0.00,4.57,4.57,1061.88,TARP,DOE-2,232.26,278.71,278.71,0.00,6,0,0,Yes
"#,
        )?;

        assert_eq!(zones.len(), 1);
        assert_eq!(zones[0].zone_name, "ZONE ONE");
        assert_eq!(zones[0].surface_count, 6);
        assert_eq!(zones[0].floor_area_m2, 232.26);
        assert_eq!(zones[0].volume_m3, 1061.88);
        assert_eq!(zones[0].exterior_gross_wall_area_m2, 278.71);

        Ok(())
    }

    #[test]
    fn series_comparison_tracks_max_delta() {
        let result = compare_series(&[1.0, 2.0], &[1.0, 2.000_000_000_1], Tolerance::default());

        assert!(result.passed);
        assert_eq!(result.samples, 2);
        assert!(result.max_abs_delta > 0.0);
        assert_eq!(result.first_divergence, None);
    }

    #[test]
    fn series_comparison_reports_first_value_divergence() -> Result<(), Box<dyn std::error::Error>>
    {
        let result = compare_series(&[1.0, 2.0, 3.0], &[1.0, 2.5, 4.0], Tolerance::default());

        assert!(!result.passed);
        let divergence = result
            .first_divergence
            .ok_or_else(|| std::io::Error::other("expected first divergence"))?;
        assert_eq!(divergence.index, 1);
        assert_eq!(divergence.expected, Some(2.0));
        assert_eq!(divergence.observed, Some(2.5));
        assert_eq!(divergence.abs_delta, Some(0.5));

        Ok(())
    }

    #[test]
    fn series_comparison_reports_length_divergence() -> Result<(), Box<dyn std::error::Error>> {
        let result = compare_series(&[1.0, 2.0], &[1.0], Tolerance::default());

        assert!(!result.passed);
        let divergence = result
            .first_divergence
            .ok_or_else(|| std::io::Error::other("expected first divergence"))?;
        assert_eq!(divergence.index, 1);
        assert_eq!(divergence.expected, Some(2.0));
        assert_eq!(divergence.observed, None);
        assert_eq!(divergence.abs_delta, None);

        Ok(())
    }
}
