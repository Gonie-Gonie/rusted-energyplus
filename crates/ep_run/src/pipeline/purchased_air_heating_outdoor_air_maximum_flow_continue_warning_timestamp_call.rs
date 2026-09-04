//! CP441 pipeline evidence for the heating maximum-flow continue-warning timestamp call site.

pub(in crate::pipeline) mod serialization;
mod validation;

pub(in crate::pipeline) use serialization::lifecycle_json;
pub(in crate::pipeline) use validation::validate_direct_lifecycle;
