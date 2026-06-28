//! Diagnostic IdealLoads node-state projection.

use crate::{OutputSeries, ResultStore, RuntimeError};
use ep_model::{
    AutosizeOrNumber, IdealLoadsAirSystem, NodeId, NormalizedName, OutputHandle, SimulationModel,
    TypedModel, ZoneEquipmentConnection,
};

use super::state::{
    NODE_STATE_SENTINEL_RULE, NODE_STATE_SETPOINT_VARIABLE, NODE_STATE_SOURCE_MAP_PATH,
    NODE_STATE_TIMESTAMP_RULE, NODE_STATE_WARMUP_RULE, NODE_TEMPERATURE_SETPOINT_SENTINEL_C,
    NodeStateRole, NodeStateStore,
};

const AIR_DENSITY_KG_PER_M3: f64 = 1.2;
const ENERGYPLUS_ZONE_INITIAL_TEMP_C: f64 = 23.0;

/// Options for the diagnostic IdealLoads node-state projection.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NodeStateProjectionOptions {
    /// Number of hourly samples to write.
    pub sample_count: usize,
    /// Fallback zone-air temperature in C.
    pub default_zone_air_temperature_c: f64,
    /// Fallback zone-air humidity ratio in kgWater/kgDryAir.
    pub default_zone_air_humidity_ratio: f64,
    /// Fallback supply-air temperature in C when no IdealLoads value exists.
    pub default_supply_air_temperature_c: f64,
    /// Fallback supply-air humidity ratio in kgWater/kgDryAir.
    pub default_supply_air_humidity_ratio: f64,
    /// Fallback supply-air mass flow rate in kg/s when no design flow exists.
    pub default_supply_air_mass_flow_rate_kg_per_s: f64,
}

impl NodeStateProjectionOptions {
    /// Creates options with a fixed hourly sample count.
    #[must_use]
    pub const fn hourly_samples(sample_count: usize) -> Self {
        Self {
            sample_count,
            default_zone_air_temperature_c: ENERGYPLUS_ZONE_INITIAL_TEMP_C,
            default_zone_air_humidity_ratio: 0.008,
            default_supply_air_temperature_c: 50.0,
            default_supply_air_humidity_ratio: 0.0156,
            default_supply_air_mass_flow_rate_kg_per_s: 0.5,
        }
    }
}

/// Evidence policy attached to diagnostic node-state projection artifacts.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NodeStateProjectionEvidencePolicy {
    /// Source map that owns the EnergyPlus routine and field mapping.
    pub source_map_path: &'static str,
    /// Timestamp alignment rule for samples written by the projection.
    pub timestamp_rule: &'static str,
    /// Warmup handling rule for samples written by the projection.
    pub warmup_rule: &'static str,
    /// Sentinel handling rule for setpoint sampling.
    pub sentinel_rule: &'static str,
    /// Output variable written through the setpoint sentinel rule.
    pub setpoint_variable: &'static str,
}

impl NodeStateProjectionEvidencePolicy {
    /// Returns the diagnostic-only v0.12 node-state evidence policy.
    #[must_use]
    pub const fn diagnostic() -> Self {
        Self {
            source_map_path: NODE_STATE_SOURCE_MAP_PATH,
            timestamp_rule: NODE_STATE_TIMESTAMP_RULE,
            warmup_rule: NODE_STATE_WARMUP_RULE,
            sentinel_rule: NODE_STATE_SENTINEL_RULE,
            setpoint_variable: NODE_STATE_SETPOINT_VARIABLE,
        }
    }
}

/// One resolved node represented by the node-state projection.
#[derive(Clone, Debug, PartialEq)]
pub struct NodeStateProjectionNode {
    /// Resolved typed node ID.
    pub node_id: NodeId,
    /// EnergyPlus-normalized node key.
    pub node_name: String,
    /// Diagnostic role for the node.
    pub role: NodeStateRole,
}

/// Summary for the diagnostic IdealLoads node-state projection.
#[derive(Clone, Debug, PartialEq)]
pub struct NodeStateProjectionSummary {
    /// Hourly output sample count.
    pub samples: usize,
    /// Number of unique nodes represented.
    pub node_count: usize,
    /// Number of output series written.
    pub series_count: usize,
    /// Number of air nodes initialized in the runtime state store.
    pub state_node_count: usize,
    /// Diagnostic evidence policy attached to output artifacts.
    pub evidence_policy: NodeStateProjectionEvidencePolicy,
    /// Resolved nodes in output order.
    pub nodes: Vec<NodeStateProjectionNode>,
}

/// Result of the diagnostic IdealLoads node-state projection.
#[derive(Clone, Debug, PartialEq)]
pub struct NodeStateProjection {
    /// Final diagnostic node state.
    pub state: NodeStateStore,
    /// Native output results.
    pub results: ResultStore,
    /// Projection summary.
    pub summary: NodeStateProjectionSummary,
}

/// Writes a deterministic diagnostic projection of IdealLoads-related node
/// state outputs.
///
/// This function intentionally does not claim EnergyPlus algorithm parity. It
/// maps the typed air-side node graph to native `ResultStore` series so the
/// port can exercise NodeList expansion, node output registration, and result
/// artifact plumbing before the full HVAC manager is ported.
pub fn simulate_ideal_loads_node_state_projection(
    model: &SimulationModel,
    options: NodeStateProjectionOptions,
) -> Result<NodeStateProjection, RuntimeError> {
    let mut state = NodeStateStore::from_typed_model(
        &model.typed,
        options.default_zone_air_temperature_c,
        options.default_zone_air_humidity_ratio,
    );
    let mut projected_nodes = Vec::new();

    for connection in &model.typed.zone_equipment_connections {
        let ideal_loads = ideal_loads_for_connection(&model.typed, connection);
        let supply_temperature_c = ideal_loads
            .map(|system| system.maximum_heating_supply_air_temperature_c)
            .unwrap_or(options.default_supply_air_temperature_c);
        let supply_humidity_ratio = ideal_loads
            .map(|system| system.maximum_heating_supply_air_humidity_ratio)
            .unwrap_or(options.default_supply_air_humidity_ratio);
        let supply_mass_flow_rate_kg_per_s = ideal_loads
            .and_then(ideal_loads_design_mass_flow_rate_kg_per_s)
            .unwrap_or(options.default_supply_air_mass_flow_rate_kg_per_s);

        let supply_nodes = connection
            .zone_air_inlet_node_or_nodelist_name
            .as_ref()
            .map(|name| resolve_node_or_nodelist(&model.typed, name))
            .unwrap_or_default();
        let supply_node_count = supply_nodes.len().max(1) as f64;
        for node_id in supply_nodes {
            if let Some(node_name) = node_name_for_id(&model.typed, node_id) {
                apply_node_state_update(
                    &mut state,
                    node_id,
                    supply_temperature_c,
                    supply_humidity_ratio,
                    supply_mass_flow_rate_kg_per_s / supply_node_count,
                );
                push_projected_node_assignment(
                    &mut projected_nodes,
                    ProjectedNodeAssignment {
                        node_id,
                        node_name,
                        role: NodeStateRole::Supply,
                    },
                );
            }
        }

        if let Some(zone_air_node_id) = model
            .typed
            .node_names
            .resolve(&connection.zone_air_node_name.0)
            && let Some(node_name) = node_name_for_id(&model.typed, zone_air_node_id)
        {
            apply_node_state_update(
                &mut state,
                zone_air_node_id,
                options.default_zone_air_temperature_c,
                options.default_zone_air_humidity_ratio,
                supply_mass_flow_rate_kg_per_s,
            );
            push_projected_node_assignment(
                &mut projected_nodes,
                ProjectedNodeAssignment {
                    node_id: zone_air_node_id,
                    node_name,
                    role: NodeStateRole::ZoneAir,
                },
            );
        }

        let return_nodes = connection
            .zone_return_air_node_or_nodelist_name
            .as_ref()
            .map(|name| resolve_node_or_nodelist(&model.typed, name))
            .unwrap_or_default();
        for node_id in return_nodes {
            if let Some(node_name) = node_name_for_id(&model.typed, node_id) {
                apply_node_state_update(
                    &mut state,
                    node_id,
                    options.default_zone_air_temperature_c,
                    options.default_zone_air_humidity_ratio,
                    supply_mass_flow_rate_kg_per_s,
                );
                push_projected_node_assignment(
                    &mut projected_nodes,
                    ProjectedNodeAssignment {
                        node_id,
                        node_name,
                        role: NodeStateRole::ReturnAir,
                    },
                );
            }
        }
    }

    if projected_nodes.is_empty() {
        return Err(RuntimeError::NoNodeStateProjectionNodes);
    }

    let mut results = ResultStore::new();
    let mut handle_index = 0_u32;
    for node in &projected_nodes {
        let Some(node_state) = state.find_by_id(node.node_id) else {
            continue;
        };
        add_constant_output_series(
            &mut results,
            &mut handle_index,
            &node.node_name,
            "System Node Temperature",
            "C",
            node_state.temperature_c,
            options.sample_count,
        );
        add_constant_output_series(
            &mut results,
            &mut handle_index,
            &node.node_name,
            "System Node Humidity Ratio",
            "kgWater/kgDryAir",
            node_state.humidity_ratio,
            options.sample_count,
        );
        add_constant_output_series(
            &mut results,
            &mut handle_index,
            &node.node_name,
            "System Node Mass Flow Rate",
            "kg/s",
            node_state.mass_flow_rate_kg_per_s,
            options.sample_count,
        );
        add_constant_output_series(
            &mut results,
            &mut handle_index,
            &node.node_name,
            "System Node Setpoint Temperature",
            "C",
            node_state
                .temperature_setpoint_c
                .unwrap_or(NODE_TEMPERATURE_SETPOINT_SENTINEL_C),
            options.sample_count,
        );
    }

    Ok(NodeStateProjection {
        summary: NodeStateProjectionSummary {
            samples: options.sample_count,
            node_count: projected_nodes.len(),
            series_count: results.series.len(),
            state_node_count: state.len(),
            evidence_policy: NodeStateProjectionEvidencePolicy::diagnostic(),
            nodes: projected_nodes
                .iter()
                .map(|node| NodeStateProjectionNode {
                    node_id: node.node_id,
                    node_name: node.node_name.clone(),
                    role: node.role,
                })
                .collect(),
        },
        state,
        results,
    })
}

#[derive(Clone, Debug, PartialEq)]
struct ProjectedNodeAssignment {
    node_id: NodeId,
    node_name: String,
    role: NodeStateRole,
}

fn push_projected_node_assignment(
    nodes: &mut Vec<ProjectedNodeAssignment>,
    node: ProjectedNodeAssignment,
) {
    if let Some(existing) = nodes
        .iter_mut()
        .find(|existing| existing.node_id == node.node_id)
    {
        existing.role = merged_node_state_role(existing.role, node.role);
        return;
    }

    nodes.push(node);
}

fn merged_node_state_role(existing: NodeStateRole, next: NodeStateRole) -> NodeStateRole {
    if existing == next {
        return existing;
    }

    match (existing, next) {
        (NodeStateRole::ZoneAir, _) | (_, NodeStateRole::ZoneAir) => NodeStateRole::ZoneAir,
        (NodeStateRole::Supply, NodeStateRole::ReturnAir)
        | (NodeStateRole::ReturnAir, NodeStateRole::Supply) => NodeStateRole::Supply,
        _ => existing,
    }
}

fn apply_node_state_update(
    state: &mut NodeStateStore,
    node_id: NodeId,
    temperature_c: f64,
    humidity_ratio: f64,
    mass_flow_rate_kg_per_s: f64,
) {
    let Some(node_state) = state.find_mut_by_id(node_id) else {
        return;
    };

    let previous_flow = node_state.mass_flow_rate_kg_per_s;
    let total_flow = previous_flow + mass_flow_rate_kg_per_s;
    if previous_flow > 0.0 && total_flow > 0.0 {
        node_state.temperature_c = weighted_value(
            node_state.temperature_c,
            previous_flow,
            temperature_c,
            mass_flow_rate_kg_per_s,
            total_flow,
        );
        node_state.humidity_ratio = weighted_value(
            node_state.humidity_ratio,
            previous_flow,
            humidity_ratio,
            mass_flow_rate_kg_per_s,
            total_flow,
        );
    } else {
        node_state.temperature_c = temperature_c;
        node_state.humidity_ratio = humidity_ratio;
    }
    node_state.mass_flow_rate_kg_per_s = total_flow;
}

fn weighted_value(
    existing_value: f64,
    existing_weight: f64,
    new_value: f64,
    new_weight: f64,
    total_weight: f64,
) -> f64 {
    (existing_value * existing_weight + new_value * new_weight) / total_weight
}

fn add_constant_output_series(
    results: &mut ResultStore,
    handle_index: &mut u32,
    key: &str,
    variable_name: &str,
    units: &str,
    value: f64,
    sample_count: usize,
) {
    results.add_series(OutputSeries {
        handle: OutputHandle(*handle_index),
        key: key.to_string(),
        variable_name: variable_name.to_string(),
        units: units.to_string(),
        values: vec![value; sample_count],
    });
    *handle_index += 1;
}

fn resolve_node_or_nodelist(model: &TypedModel, name: &NormalizedName) -> Vec<NodeId> {
    if let Some(node_id) = model.node_names.resolve(&name.0) {
        return vec![node_id];
    }

    if let Some(node_list_id) = model.node_list_names.resolve(&name.0)
        && let Some(node_list) = model
            .node_lists
            .iter()
            .find(|node_list| node_list.id == node_list_id)
    {
        return node_list.nodes.clone();
    }

    Vec::new()
}

fn node_name_for_id(model: &TypedModel, node_id: NodeId) -> Option<String> {
    model
        .nodes
        .iter()
        .find(|node| node.id == node_id)
        .map(|node| node.name.0.clone())
}

fn ideal_loads_for_connection<'a>(
    model: &'a TypedModel,
    connection: &ZoneEquipmentConnection,
) -> Option<&'a IdealLoadsAirSystem> {
    let list = model
        .zone_equipment_lists
        .iter()
        .find(|list| list.id == connection.equipment_list)?;
    let entry = list.equipment.iter().min_by_key(|entry| {
        (
            entry.heating_or_no_load_sequence,
            entry.cooling_sequence,
            entry.ideal_loads_air_system,
        )
    })?;
    model
        .ideal_loads_air_systems
        .iter()
        .find(|system| system.id == entry.ideal_loads_air_system)
}

fn ideal_loads_design_mass_flow_rate_kg_per_s(system: &IdealLoadsAirSystem) -> Option<f64> {
    let heating_flow_m3_per_s =
        autosized_or_numeric_value(system.maximum_heating_air_flow_rate_m3_per_s);
    let cooling_flow_m3_per_s =
        autosized_or_numeric_value(system.maximum_cooling_air_flow_rate_m3_per_s);
    heating_flow_m3_per_s
        .into_iter()
        .chain(cooling_flow_m3_per_s)
        .filter(|value| *value > 0.0)
        .reduce(f64::max)
        .map(|flow_m3_per_s| flow_m3_per_s * AIR_DENSITY_KG_PER_M3)
}

fn autosized_or_numeric_value(value: Option<AutosizeOrNumber>) -> Option<f64> {
    match value {
        Some(AutosizeOrNumber::Value(value)) => Some(value),
        Some(AutosizeOrNumber::Autosize) | None => None,
    }
}
