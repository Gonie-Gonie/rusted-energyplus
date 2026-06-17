//! Air-side node state storage and diagnostic metadata.

use ep_model::{NodeId, NormalizedName, TypedModel};

/// EnergyPlus `SensedNodeFlagValue` used for unset node temperature setpoints.
pub const NODE_TEMPERATURE_SETPOINT_SENTINEL_C: f64 = -999.0;
/// Source map that owns node-state output registration and update paths.
pub const NODE_STATE_SOURCE_MAP_PATH: &str = "docs/src/porting-map/node-state-source-map.md";
/// Timestamp rule for the diagnostic node-state projection.
pub const NODE_STATE_TIMESTAMP_RULE: &str =
    "hour-ending hourly samples aligned to the run-period time axis";
/// Warmup handling rule for the diagnostic node-state projection.
pub const NODE_STATE_WARMUP_RULE: &str =
    "EnergyPlus warmup samples are not represented in this diagnostic projection";
/// Sentinel handling rule for excluded node setpoint output.
pub const NODE_STATE_SENTINEL_RULE: &str = "System Node Setpoint Temperature remains excluded; EnergyPlus SensedNodeFlagValue (-999) is represented as None";
/// Node output variable excluded until setpoint ownership and sentinel filtering are ported.
pub const NODE_STATE_EXCLUDED_SETPOINT_VARIABLE: &str = "System Node Setpoint Temperature";

/// Role assigned to a node-state projection row.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NodeStateRole {
    /// Zone inlet or IdealLoads supply node.
    Supply,
    /// Zone air node.
    ZoneAir,
    /// Zone return node.
    ReturnAir,
}

impl NodeStateRole {
    /// Stable diagnostic label.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Supply => "supply",
            Self::ZoneAir => "zone-air",
            Self::ReturnAir => "return-air",
        }
    }
}

/// Runtime scalar state for one air-side node.
#[derive(Clone, Debug, PartialEq)]
pub struct AirNodeState {
    /// Resolved typed node ID.
    pub node_id: NodeId,
    /// EnergyPlus-normalized node key.
    pub node_name: String,
    /// Current node temperature in C.
    pub temperature_c: f64,
    /// Current node humidity ratio in kgWater/kgDryAir.
    pub humidity_ratio: f64,
    /// Current node mass flow rate in kg/s.
    pub mass_flow_rate_kg_per_s: f64,
    /// Optional node temperature setpoint in C.
    pub temperature_setpoint_c: Option<f64>,
}

/// Diagnostic air-side node state store.
#[derive(Clone, Debug, PartialEq)]
pub struct NodeStateStore {
    /// Air-side node states in typed-node order.
    pub air_nodes: Vec<AirNodeState>,
}

impl NodeStateStore {
    /// Initializes one diagnostic air-node state for each typed model node.
    #[must_use]
    pub fn from_typed_model(
        model: &TypedModel,
        default_temperature_c: f64,
        default_humidity_ratio: f64,
    ) -> Self {
        Self {
            air_nodes: model
                .nodes
                .iter()
                .map(|node| AirNodeState {
                    node_id: node.id,
                    node_name: node.name.0.clone(),
                    temperature_c: default_temperature_c,
                    humidity_ratio: default_humidity_ratio,
                    mass_flow_rate_kg_per_s: 0.0,
                    temperature_setpoint_c: None,
                })
                .collect(),
        }
    }

    /// Number of stored air nodes.
    #[must_use]
    pub fn len(&self) -> usize {
        self.air_nodes.len()
    }

    /// Returns true when no air nodes are stored.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.air_nodes.is_empty()
    }

    /// Finds an air-node state by typed node ID.
    #[must_use]
    pub fn find_by_id(&self, node_id: NodeId) -> Option<&AirNodeState> {
        self.air_nodes.iter().find(|node| node.node_id == node_id)
    }

    /// Finds an air-node state by EnergyPlus key.
    #[must_use]
    pub fn find_by_key(&self, key: &str) -> Option<&AirNodeState> {
        let normalized = NormalizedName::new(key);
        self.air_nodes
            .iter()
            .find(|node| node.node_name == normalized.0)
    }

    pub(crate) fn find_mut_by_id(&mut self, node_id: NodeId) -> Option<&mut AirNodeState> {
        self.air_nodes
            .iter_mut()
            .find(|node| node.node_id == node_id)
    }
}

/// Converts an EnergyPlus node temperature setpoint scalar into diagnostic state.
#[must_use]
pub fn node_temperature_setpoint_from_energyplus(value_c: f64) -> Option<f64> {
    if (value_c - NODE_TEMPERATURE_SETPOINT_SENTINEL_C).abs() < 1.0e-9 {
        None
    } else {
        Some(value_c)
    }
}
