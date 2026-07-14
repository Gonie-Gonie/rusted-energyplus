//! IdealLoads `CalcPurchAirLoads` compatibility calculations.

mod humidity;
mod limits;
mod mass_flow;
mod moisture_demand;
mod no_oa;
mod psychrometrics;
mod types;

#[cfg(test)]
mod moisture_demand_tests;
#[cfg(test)]
mod no_oa_tests;

pub use limits::IdealLoadsSensibleLimitContext;
pub use moisture_demand::{
    NoOaThirdOrderHumidityCorrector, NoOaThirdOrderHumidityCorrectorInput,
    NoOaThirdOrderMoistureDemand, NoOaThirdOrderMoistureDemandInput,
    calc_no_oa_third_order_moisture_demand_compat, correct_no_oa_third_order_humidity_ratio_compat,
    third_order_humidity_history_term,
};
pub use no_oa::*;
pub use psychrometrics::{energyplus_standard_air_density_kg_per_m3, moist_air_enthalpy_j_per_kg};
pub use types::*;
