//! Heat-balance compatibility and diagnostic selection APIs.

pub mod algorithm;

pub use algorithm::*;
pub(crate) use algorithm::{
    heat_balance_zone_air_algorithm_execution_variant, heat_balance_zone_air_algorithm_feature_base,
};
