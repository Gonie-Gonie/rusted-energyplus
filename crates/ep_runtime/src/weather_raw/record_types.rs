//! Storage and outcomes for the raw EPW record owner.

use std::fmt::{Display, Formatter};

/// Original integer reference arguments, in declaration order.
pub const DATE_KEYS: [&str; 5] = ["WYear", "WMonth", "WDay", "WHour", "WMinute"];

/// Original mandatory real reference arguments, in declaration order.
pub const MANDATORY_KEYS: [&str; 20] = [
    "DryBulb",
    "DewPoint",
    "RelHum",
    "AtmPress",
    "ETHoriz",
    "ETDirect",
    "IRHoriz",
    "GLBHoriz",
    "DirectRad",
    "DiffuseRad",
    "GLBHorizIllum",
    "DirectNrmIllum",
    "DiffuseHorizIllum",
    "ZenLum",
    "WindDir",
    "WindSpeed",
    "TotalSkyCover",
    "OpaqueSkyCover",
    "Visibility",
    "CeilHeight",
];

/// Original optional real reference arguments, in declaration order.
pub const OPTIONAL_KEYS: [&str; 6] = [
    "PrecipWater",
    "AerosolOptDepth",
    "SnowDepth",
    "DaysSinceLastSnow",
    "Albedo",
    "LiquidPrecip",
];

/// Caller-owned reference argument storage for `InterpretWeatherDataLine`.
///
/// Defaults initialize this Rust caller's storage to zero. They do not represent
/// a WeatherManager record constructor. A failure preserves the writes already
/// performed by the method, so callers may supply explicit initial values.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RawEpwOutputs {
    /// The source resets this flag before reading any field.
    pub error_found: bool,
    /// Raw record date fields; the source year is separate from civil calendar.
    pub dates: [i32; 5],
    /// Raw mandatory fields without missing-value or physical normalization.
    pub mandatory_reals: [f64; 20],
    /// Rounded raw weather observation indicator, written after optional fields.
    pub observation_indicator: i32,
    /// Nine observed weather codes, interpreted only when the indicator is zero.
    pub weather_codes: [i32; 9],
    /// Optional raw fields; missing or empty fields become the source value 999.
    pub optional_reals: [f64; 6],
}

impl Default for RawEpwOutputs {
    fn default() -> Self {
        Self {
            error_found: false,
            dates: [0; 5],
            mandatory_reals: [0.0; 20],
            observation_indicator: 0,
            weather_codes: [0; 9],
            optional_reals: [0.0; 6],
        }
    }
}

/// Mutable owner state read and written by the selected raw record method.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RawEpwParserState {
    /// Source month-end table; February admits one extra raw day in this method.
    pub end_day_of_month: [i32; 12],
    /// Source `wvarsMissedCounts.WeathCodes`, incremented for invalid code length.
    pub weather_code_missed_count: i32,
}

impl Default for RawEpwParserState {
    fn default() -> Self {
        Self {
            end_day_of_month: [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31],
            weather_code_missed_count: 0,
        }
    }
}

/// Failure from the raw record method, distinct from full simulation diagnostics.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RawRecordFailure {
    /// The final integer list read failed, taking the source fatal date route.
    SourceFatalDateList,
    /// The source rejected the month or the upper day limit.
    SourceFatalDateAdmission,
    /// The final mandatory real list read failed, taking the source fatal route.
    SourceFatalMandatoryList,
    /// An optional field failed `ProcessNumber` before assignment to that field.
    SourceFatalOptionalNumber {
        /// Zero-based optional field index, in `OPTIONAL_KEYS` order.
        field: usize,
    },
    /// Input lies outside the selected defined finite source comparison domain.
    ///
    /// This is a Rust admission failure, not an assertion about an original
    /// source failure. It avoids delimiter underflow and undefined casts.
    OutsideBoundedDomain(&'static str),
}

impl Display for RawRecordFailure {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SourceFatalDateList => formatter.write_str("invalid date list in EPW record"),
            Self::SourceFatalDateAdmission => formatter.write_str("invalid raw date in EPW record"),
            Self::SourceFatalMandatoryList => {
                formatter.write_str("invalid mandatory EPW number list")
            }
            Self::SourceFatalOptionalNumber { field } => {
                write!(formatter, "invalid optional EPW number at field {field}")
            }
            Self::OutsideBoundedDomain(reason) => {
                write!(
                    formatter,
                    "EPW record is outside the bounded raw domain: {reason}"
                )
            }
        }
    }
}

impl std::error::Error for RawRecordFailure {}
