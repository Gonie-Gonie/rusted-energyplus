//! CP385 post-saturation capacity-limited dehumidifying supply-enthalpy evidence.

mod serialization;
mod validation;

pub(in crate::pipeline) use serialization::lifecycle_json;
pub(in crate::pipeline) use validation::validate_direct_lifecycle;
