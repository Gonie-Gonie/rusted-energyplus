//! CP384 post-saturation dehumidifying total-output maximum-capacity assignment evidence.

mod serialization;
mod validation;

pub(in crate::pipeline) use serialization::lifecycle_json;
pub(in crate::pipeline) use validation::validate_direct_lifecycle;
