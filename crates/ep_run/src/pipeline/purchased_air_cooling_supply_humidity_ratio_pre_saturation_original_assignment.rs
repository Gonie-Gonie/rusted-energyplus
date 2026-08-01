//! CP376 pre-saturation original-assignment validation and JSON serialization.

mod serialization;
mod validation;

pub(in crate::pipeline) use serialization::lifecycle_json;
pub(in crate::pipeline) use validation::{DirectLifecyclePredecessors, validate_direct_lifecycle};
