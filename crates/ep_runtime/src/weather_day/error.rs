//! Weather ownership errors preserve source termination versus Rust admission.

use std::fmt::{Display, Formatter};

/// Failure while advancing a live weather owner.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WeatherDayError {
    reason: String,
    source_fatal: bool,
}

impl WeatherDayError {
    /// A defined source-fatal branch in the selected port.
    pub fn source_fatal(reason: impl Into<String>) -> Self {
        Self {
            reason: reason.into(),
            source_fatal: true,
        }
    }

    /// A Rust storage or currently unsupported-domain failure.
    pub fn admission(reason: impl Into<String>) -> Self {
        Self {
            reason: reason.into(),
            source_fatal: false,
        }
    }

    /// Whether the selected source operation has a fatal outcome.
    #[must_use]
    pub fn is_source_fatal(&self) -> bool {
        self.source_fatal
    }
}

impl Display for WeatherDayError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.reason)
    }
}
impl std::error::Error for WeatherDayError {}
impl From<super::WeatherDayValuesError> for WeatherDayError {
    fn from(error: super::WeatherDayValuesError) -> Self {
        Self::admission(error.to_string())
    }
}
impl From<crate::weather::raw::RawDayFailure> for WeatherDayError {
    fn from(error: crate::weather::raw::RawDayFailure) -> Self {
        Self {
            source_fatal: error.is_source_fatal(),
            reason: error.to_string(),
        }
    }
}
