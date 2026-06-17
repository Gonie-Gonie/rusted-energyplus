//! Air-side node compatibility facade.

mod state;

pub use state::*;

use ep_model::NodeId;

/// Store that owns IdealLoads node output time series in Rust reports.
pub const IDEAL_LOADS_NODE_OUTPUT_STORE_TYPE: &str = "ep_runtime::ResultStore";
/// State transfer struct populated from the final IdealLoads supply-node update.
pub const IDEAL_LOADS_NODE_OUTPUT_STATE_STRUCT: &str =
    "ep_runtime::node::IdealLoadsSupplyNodeUpdate";
/// EnergyPlus source routine that finalizes supply node state before reporting.
pub const IDEAL_LOADS_NODE_OUTPUT_UPDATE_SOURCE: &str = "UpdatePurchasedAir";
/// EnergyPlus source routine that reports purchased-air and node output values.
pub const IDEAL_LOADS_NODE_OUTPUT_REPORT_SOURCE: &str = "ReportPurchasedAir";

/// Final node values written by the narrow IdealLoads compatibility path.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IdealLoadsSupplyNodeUpdate {
    /// Supply node.
    pub node: NodeId,
    /// Final supply temperature in C.
    pub temperature_c: f64,
    /// Final supply humidity ratio in kgWater/kgDryAir.
    pub humidity_ratio: f64,
    /// Final supply mass flow rate in kg/s.
    pub mass_flow_rate_kg_per_s: f64,
    /// Final supply enthalpy in J/kg.
    pub enthalpy_j_per_kg: f64,
}
