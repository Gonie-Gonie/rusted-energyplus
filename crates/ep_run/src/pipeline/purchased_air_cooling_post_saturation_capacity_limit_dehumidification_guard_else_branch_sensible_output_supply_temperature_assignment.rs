//! CP423 direct-run validation and lossless JSON serialization.

pub(in crate::pipeline) mod serialization;
mod validation;

pub(in crate::pipeline) use serialization::lifecycle_json;
pub(in crate::pipeline) use validation::validate_direct_lifecycle;
