//! IdealLoads `CalcPurchAirLoads` compatibility calculations.

mod no_oa;
mod psychrometrics;

#[cfg(test)]
mod no_oa_tests;

pub use no_oa::*;
pub use psychrometrics::{energyplus_standard_air_density_kg_per_m3, moist_air_enthalpy_j_per_kg};
