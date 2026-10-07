//! CP442 pipeline evidence for the heating maximum-flow first-warning guard else entry.

pub(in crate::pipeline) mod serialization;
mod validation;

pub(in crate::pipeline) use serialization::lifecycle_json;
pub(in crate::pipeline) use validation::validate_direct_lifecycle;
