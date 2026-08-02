//! CP398 direct-run validation and JSON serialization.

mod serialization;
mod validation;

pub(in crate::pipeline) use serialization::lifecycle_json;
#[cfg(test)]
pub(in crate::pipeline) use serialization::test_snapshot;
pub(in crate::pipeline) use validation::validate_direct_lifecycle;
