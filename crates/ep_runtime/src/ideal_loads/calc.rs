//! IdealLoads `CalcPurchAirLoads` compatibility calculations.

mod cooling_economizer_guard;
// CP316 follows the CP315 outer guard in source order.
mod cooling_economizer_condition;
// CP317 follows the CP316 condition in source order.
mod cooling_economizer_body;
mod cooling_entry_gate;
mod cooling_oa_max_flow_body;
mod cooling_oa_max_flow_gate;
mod humidity;
mod lifecycle;
mod limits;
mod mass_flow;
mod minimum_oa_prefix;
mod moisture_demand;
mod no_oa;
mod psychrometrics;
mod types;

#[cfg(test)]
mod cooling_economizer_guard_tests;
// CP316 tests follow the CP315 characterization module in source order.
#[cfg(test)]
mod cooling_economizer_condition_release_tests;
#[cfg(test)]
mod cooling_economizer_condition_tests;
// CP317 tests follow the CP316 characterization module in source order.
#[cfg(test)]
mod cooling_economizer_body_release_tests;
#[cfg(test)]
mod cooling_economizer_body_tests;
#[cfg(test)]
mod cooling_entry_gate_tests;
#[cfg(test)]
mod cooling_oa_max_flow_body_tests;
#[cfg(test)]
mod cooling_oa_max_flow_gate_tests;
#[cfg(test)]
mod lifecycle_tests;
#[cfg(test)]
mod minimum_oa_prefix_tests;
#[cfg(test)]
mod moisture_demand_tests;
#[cfg(test)]
mod no_oa_tests;

pub use cooling_economizer_guard::*;
// CP316 exports follow the CP315 guard contract in source order.
pub use cooling_economizer_condition::*;
// CP317 exports follow the CP316 condition contract in source order.
pub(in crate::ideal_loads) use cooling_economizer_body::release::body_snapshot_is_exact_direct_release as cooling_economizer_body_snapshot_is_exact_direct_release;
pub use cooling_economizer_body::*;
pub use cooling_entry_gate::*;
pub use cooling_oa_max_flow_body::*;
pub use cooling_oa_max_flow_gate::*;
pub use lifecycle::*;
pub use limits::IdealLoadsSensibleLimitContext;
pub use minimum_oa_prefix::*;
pub use moisture_demand::{
    NoOaThirdOrderHumidityCorrector, NoOaThirdOrderHumidityCorrectorInput,
    NoOaThirdOrderMoistureDemand, NoOaThirdOrderMoistureDemandInput,
    calc_no_oa_third_order_moisture_demand_compat, correct_no_oa_third_order_humidity_ratio_compat,
    third_order_humidity_history_term,
};
pub use no_oa::*;
pub use psychrometrics::{energyplus_standard_air_density_kg_per_m3, moist_air_enthalpy_j_per_kg};
pub use types::*;
