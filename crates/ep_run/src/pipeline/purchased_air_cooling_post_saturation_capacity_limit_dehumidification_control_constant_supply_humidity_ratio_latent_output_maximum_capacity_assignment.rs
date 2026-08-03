//! CP405 direct-run validation and JSON serialization.

mod serialization;
mod validation;

pub(in crate::pipeline) use serialization::lifecycle_json;
pub(in crate::pipeline) use validation::validate_direct_lifecycle;
