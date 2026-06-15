//! Air-side node compatibility facade.

pub use crate::{
    AirNodeState, NODE_STATE_EXCLUDED_SETPOINT_VARIABLE, NODE_STATE_SENTINEL_RULE,
    NODE_STATE_SOURCE_MAP_PATH, NODE_STATE_TIMESTAMP_RULE, NODE_STATE_WARMUP_RULE, NodeStateRole,
    NodeStateStore, node_temperature_setpoint_from_energyplus,
};

use ep_model::NodeId;

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
