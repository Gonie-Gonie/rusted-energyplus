//! IdealLoads node-update helpers.

use crate::{ideal_loads::IdealLoadsSensibleResult, node::IdealLoadsSupplyNodeUpdate};
use ep_model::NodeId;

/// Builds the supply-node write from a calculated IdealLoads result.
#[must_use]
pub const fn supply_node_update_from_result(
    node: NodeId,
    result: IdealLoadsSensibleResult,
) -> IdealLoadsSupplyNodeUpdate {
    IdealLoadsSupplyNodeUpdate {
        node,
        temperature_c: result.supply_temperature_c,
        humidity_ratio: result.supply_humidity_ratio,
        mass_flow_rate_kg_per_s: result.supply_mass_flow_rate_kg_per_s,
        enthalpy_j_per_kg: result.supply_enthalpy_j_per_kg,
    }
}
