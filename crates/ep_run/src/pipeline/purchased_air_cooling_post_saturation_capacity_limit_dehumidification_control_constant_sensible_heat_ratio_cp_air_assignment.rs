//! CP387 post-saturation constant-SHR case-entry and `CpAir` assignment evidence.

mod serialization;
mod validation;

pub(in crate::pipeline) use serialization::lifecycle_json;
pub(in crate::pipeline) use validation::validate_direct_lifecycle;
