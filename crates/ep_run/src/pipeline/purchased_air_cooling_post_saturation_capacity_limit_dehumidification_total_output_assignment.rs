//! CP382 post-saturation dehumidifying total-output assignment validation and JSON serialization.

mod serialization;
mod validation;

pub(in crate::pipeline) use serialization::lifecycle_json;
pub(in crate::pipeline) use validation::validate_direct_lifecycle;
