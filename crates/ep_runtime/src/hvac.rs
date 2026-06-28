//! HVAC graph and node-state diagnostic helpers.

use ep_model::{NodeId, NormalizedName};

use crate::node::{AirNodeState, NodeStateStore};

/// Diagnostic boundary for AirLoopHVAC component simulation.
pub const HVAC_COMPONENT_NODE_STATE_TRACE_POLICY: &str = "record NodeStateStore before and after component simulation; diagnostic-only until AirLoopHVAC conformance is promoted";

/// Node-state snapshot around one component simulation call.
#[derive(Clone, Debug, PartialEq)]
pub struct HvacComponentNodeStateTrace {
    /// Component object type.
    pub component_type: NormalizedName,
    /// Component object name.
    pub component_name: NormalizedName,
    /// Inlet node.
    pub inlet_node: NodeId,
    /// Outlet node.
    pub outlet_node: NodeId,
    /// Node state before the component simulation.
    pub before: Option<AirNodeState>,
    /// Node state after the component simulation.
    pub after: Option<AirNodeState>,
}

/// Captures a before/after node-state trace for an HVAC component.
#[must_use]
pub fn trace_hvac_component_node_state_transition(
    state_before: &NodeStateStore,
    state_after: &NodeStateStore,
    component_type: NormalizedName,
    component_name: NormalizedName,
    inlet_node: NodeId,
    outlet_node: NodeId,
) -> HvacComponentNodeStateTrace {
    HvacComponentNodeStateTrace {
        component_type,
        component_name,
        inlet_node,
        outlet_node,
        before: state_before.find_by_id(inlet_node).cloned(),
        after: state_after.find_by_id(outlet_node).cloned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node::AirNodeState;
    use ep_model::{Node, NormalizedName, TypedModel};

    #[test]
    fn hvac_component_node_state_trace_records_before_and_after_nodes() {
        let mut model = TypedModel::default();
        model.nodes.push(Node {
            id: NodeId(0),
            name: NormalizedName::new("fan inlet"),
        });
        model.nodes.push(Node {
            id: NodeId(1),
            name: NormalizedName::new("fan outlet"),
        });
        let mut before = NodeStateStore::from_typed_model(&model, 20.0, 0.008);
        let mut after = NodeStateStore::from_typed_model(&model, 20.0, 0.008);
        before.air_nodes[0] = AirNodeState {
            node_id: NodeId(0),
            node_name: "FAN INLET".to_string(),
            temperature_c: 20.0,
            humidity_ratio: 0.008,
            mass_flow_rate_kg_per_s: 0.5,
            temperature_setpoint_c: Some(21.0),
        };
        after.air_nodes[1] = AirNodeState {
            node_id: NodeId(1),
            node_name: "FAN OUTLET".to_string(),
            temperature_c: 20.3,
            humidity_ratio: 0.008,
            mass_flow_rate_kg_per_s: 0.5,
            temperature_setpoint_c: Some(21.0),
        };

        let trace = trace_hvac_component_node_state_transition(
            &before,
            &after,
            NormalizedName::new("Fan:ConstantVolume"),
            NormalizedName::new("Supply Fan"),
            NodeId(0),
            NodeId(1),
        );

        assert_eq!(trace.before.unwrap().temperature_c, 20.0);
        assert_eq!(trace.after.unwrap().temperature_c, 20.3);
        assert_eq!(
            trace.component_type,
            NormalizedName::new("Fan:ConstantVolume")
        );
    }
}
