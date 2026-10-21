//! Native external schedule data, independent of InputProcessor number storage.
//!
//! The live manager owns the resolved-path cache. These values retain the full
//! parsed file, including unused columns, before a selected column is converted.
use std::collections::BTreeMap;

mod csv;
mod json_text;
pub(crate) use csv::{NativeExternalDataDiagnostic, decode_native_csv};
pub(crate) use json_text::decode_native_text_json;

/// The actual nlohmann JSON kinds in ScheduleManager's external-file cache.
/// CSV strings are arbitrary bytes; JSON strings and keys are decoded UTF-8.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum NativeExternalDataValue {
    Null,
    Boolean(bool),
    Signed(i64),
    Unsigned(u64),
    /// Preserve signed zero and nonfinites produced by Native's CSV parser.
    Real(f64),
    String(Vec<u8>),
    Array(Vec<Self>),
    Object(BTreeMap<Vec<u8>, Self>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct NativeExternalDataTypeError {
    pub(crate) expected: &'static str,
    pub(crate) actual: &'static str,
}

impl NativeExternalDataValue {
    pub(crate) fn type_name(&self) -> &'static str {
        match self {
            Self::Null => "null",
            Self::Boolean(_) => "boolean",
            Self::Signed(_) | Self::Unsigned(_) | Self::Real(_) => "number",
            Self::String(_) => "string",
            Self::Array(_) => "array",
            Self::Object(_) => "object",
        }
    }

    /// nlohmann get<double>: direct integer conversion, never IP UInt wrapping.
    pub(crate) fn numeric_real64(&self) -> Result<f64, NativeExternalDataTypeError> {
        match self {
            Self::Signed(value) => Ok(*value as f64),
            Self::Unsigned(value) => Ok(*value as f64),
            Self::Real(value) => Ok(*value),
            _ => Err(NativeExternalDataTypeError {
                expected: "number",
                actual: self.type_name(),
            }),
        }
    }

    /// Perform the selected column's vector<Real64> conversion only when asked.
    pub(crate) fn real64_column(&self) -> Result<Vec<f64>, NativeExternalDataTypeError> {
        match self {
            Self::Array(values) => values.iter().map(Self::numeric_real64).collect(),
            _ => Err(NativeExternalDataTypeError {
                expected: "array",
                actual: self.type_name(),
            }),
        }
    }
}
