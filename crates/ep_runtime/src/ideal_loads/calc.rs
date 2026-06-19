//! IdealLoads `CalcPurchAirLoads` compatibility calculations.

mod humidity;
mod limits;
mod mass_flow;
mod moisture_demand;
mod no_oa;
mod psychrometrics;
mod types;

#[cfg(test)]
mod no_oa_tests;

pub use limits::IdealLoadsSensibleLimitContext;
pub use moisture_demand::{
    NoOaThirdOrderMoistureDemand, NoOaThirdOrderMoistureDemandInput,
    calc_no_oa_third_order_moisture_demand_compat,
};
pub use no_oa::*;
pub use psychrometrics::{energyplus_standard_air_density_kg_per_m3, moist_air_enthalpy_j_per_kg};
pub use types::*;
