//! CP386 post-saturation dehumidification-control switch evidence.

mod serialization;
mod validation;

pub(in crate::pipeline) use serialization::lifecycle_json;
pub(in crate::pipeline) use validation::validate_direct_lifecycle;
