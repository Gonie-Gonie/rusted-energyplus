//! Diagnostic plant-state projection.

use crate::{OutputSeries, ResultStore, RuntimeError};
use ep_model::{
    BranchId, BranchListId, LoopId, NodeId, OutputHandle, PlantBranchComponent, PlantLoop,
    SimulationModel, TypedModel,
};

/// Source map that owns plant diagnostic output registration and future update paths.
pub const PLANT_STATE_SOURCE_MAP_PATH: &str = "docs/src/porting-map/plant-source-map.md";
/// Timestamp rule for the diagnostic plant-state projection.
pub const PLANT_STATE_TIMESTAMP_RULE: &str =
    "hour-ending hourly samples aligned to the plant diagnostic case time axis";
/// Warmup handling rule for the diagnostic plant-state projection.
pub const PLANT_STATE_WARMUP_RULE: &str =
    "EnergyPlus warmup samples are not represented in this diagnostic projection";
/// Sizing/design-day boundary for the diagnostic plant-state projection.
pub const PLANT_STATE_SIZING_RULE: &str = "PlantLoop sizing-period baseline rows remain diagnostic-only until plant loop algorithms are ported";

/// Diagnostic role assigned to one plant equipment projection row.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlantEquipmentRole {
    /// Pump component row.
    Pump,
    /// Purchased heating source row.
    PurchasedHeating,
    /// Plant load profile row.
    LoadProfile,
}

impl PlantEquipmentRole {
    /// Stable diagnostic label.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Pump => "pump",
            Self::PurchasedHeating => "purchased-heating",
            Self::LoadProfile => "load-profile",
        }
    }
}

/// Options for the diagnostic PlantLoadProfile plant-state projection.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlantStateProjectionOptions {
    /// Number of hourly samples to write.
    pub sample_count: usize,
    /// Fallback plant supply-side cooling demand in W.
    pub default_cooling_demand_w: f64,
    /// Fallback plant supply-side heating demand in W.
    pub default_heating_demand_w: f64,
    /// Fallback plant supply inlet mass flow rate in kg/s.
    pub default_inlet_mass_flow_rate_kg_per_s: f64,
    /// Fallback plant supply inlet temperature in C.
    pub default_inlet_temperature_c: f64,
    /// Fallback plant supply outlet temperature in C.
    pub default_outlet_temperature_c: f64,
    /// Fallback pump electricity rate in W.
    pub default_pump_electricity_rate_w: f64,
    /// Fallback purchased heating water rate in W.
    pub default_district_heating_rate_w: f64,
    /// Fallback load profile heat transfer rate in W.
    pub default_load_profile_heat_transfer_rate_w: f64,
}

impl PlantStateProjectionOptions {
    /// Creates options with a fixed hourly sample count.
    #[must_use]
    pub const fn hourly_samples(sample_count: usize) -> Self {
        Self {
            sample_count,
            default_cooling_demand_w: 500.0,
            default_heating_demand_w: 1_000.0,
            default_inlet_mass_flow_rate_kg_per_s: 1.0,
            default_inlet_temperature_c: 60.0,
            default_outlet_temperature_c: 65.0,
            default_pump_electricity_rate_w: 250.0,
            default_district_heating_rate_w: 1_000.0,
            default_load_profile_heat_transfer_rate_w: 1_000.0,
        }
    }
}

/// Evidence policy attached to diagnostic plant-state projection artifacts.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlantStateProjectionEvidencePolicy {
    /// Source map that owns the EnergyPlus routine and field mapping.
    pub source_map_path: &'static str,
    /// Timestamp alignment rule for samples written by the projection.
    pub timestamp_rule: &'static str,
    /// Warmup handling rule for samples written by the projection.
    pub warmup_rule: &'static str,
    /// Sizing/design-day boundary for the projection.
    pub sizing_rule: &'static str,
}

impl PlantStateProjectionEvidencePolicy {
    /// Returns the diagnostic-only plant-state evidence policy.
    #[must_use]
    pub const fn diagnostic() -> Self {
        Self {
            source_map_path: PLANT_STATE_SOURCE_MAP_PATH,
            timestamp_rule: PLANT_STATE_TIMESTAMP_RULE,
            warmup_rule: PLANT_STATE_WARMUP_RULE,
            sizing_rule: PLANT_STATE_SIZING_RULE,
        }
    }
}

/// Runtime scalar state for one plant loop diagnostic row set.
#[derive(Clone, Debug, PartialEq)]
pub struct PlantLoopState {
    /// Plant loop ID.
    pub loop_id: LoopId,
    /// EnergyPlus-normalized plant loop key.
    pub loop_name: String,
    /// Supply-side inlet node name.
    pub supply_inlet_node_name: String,
    /// Supply-side outlet node name.
    pub supply_outlet_node_name: String,
    /// Current supply-side cooling demand in W.
    pub supply_side_cooling_demand_w: f64,
    /// Current supply-side heating demand in W.
    pub supply_side_heating_demand_w: f64,
    /// Current supply-side inlet mass flow rate in kg/s.
    pub supply_side_inlet_mass_flow_rate_kg_per_s: f64,
    /// Current supply-side inlet temperature in C.
    pub supply_side_inlet_temperature_c: f64,
    /// Current supply-side outlet temperature in C.
    pub supply_side_outlet_temperature_c: f64,
}

/// Runtime scalar state for one plant diagnostic equipment row.
#[derive(Clone, Debug, PartialEq)]
pub struct PlantEquipmentState {
    /// Component object type.
    pub object_type: String,
    /// EnergyPlus-normalized equipment key.
    pub equipment_name: String,
    /// Diagnostic role.
    pub role: PlantEquipmentRole,
    /// Projected equipment output value.
    pub output_rate_w: f64,
}

/// Diagnostic plant state store.
#[derive(Clone, Debug, PartialEq)]
pub struct PlantStateStore {
    /// Plant loop states in typed-loop order.
    pub loops: Vec<PlantLoopState>,
    /// Plant equipment states in plant graph order.
    pub equipment: Vec<PlantEquipmentState>,
}

impl PlantStateStore {
    /// Number of stored plant loops.
    #[must_use]
    pub fn loop_count(&self) -> usize {
        self.loops.len()
    }

    /// Number of stored plant equipment rows.
    #[must_use]
    pub fn equipment_count(&self) -> usize {
        self.equipment.len()
    }
}

/// One resolved plant loop represented by the projection.
#[derive(Clone, Debug, PartialEq)]
pub struct PlantStateProjectionLoop {
    /// Plant loop ID.
    pub loop_id: LoopId,
    /// EnergyPlus-normalized loop key.
    pub loop_name: String,
    /// Supply-side inlet node name.
    pub supply_inlet_node_name: String,
    /// Supply-side outlet node name.
    pub supply_outlet_node_name: String,
}

/// One resolved plant equipment represented by the projection.
#[derive(Clone, Debug, PartialEq)]
pub struct PlantStateProjectionEquipment {
    /// Component object type.
    pub object_type: String,
    /// EnergyPlus-normalized equipment key.
    pub equipment_name: String,
    /// Diagnostic role.
    pub role: PlantEquipmentRole,
}

/// Summary for the diagnostic PlantLoadProfile plant-state projection.
#[derive(Clone, Debug, PartialEq)]
pub struct PlantStateProjectionSummary {
    /// Hourly output sample count.
    pub samples: usize,
    /// Number of plant loops represented.
    pub loop_count: usize,
    /// Number of equipment rows represented.
    pub equipment_count: usize,
    /// Number of output series written.
    pub series_count: usize,
    /// Diagnostic evidence policy attached to output artifacts.
    pub evidence_policy: PlantStateProjectionEvidencePolicy,
    /// Resolved loops in output order.
    pub loops: Vec<PlantStateProjectionLoop>,
    /// Resolved equipment rows in output order.
    pub equipment: Vec<PlantStateProjectionEquipment>,
}

/// Result of the diagnostic PlantLoadProfile plant-state projection.
#[derive(Clone, Debug, PartialEq)]
pub struct PlantStateProjection {
    /// Final diagnostic plant state.
    pub state: PlantStateStore,
    /// Native output results.
    pub results: ResultStore,
    /// Projection summary.
    pub summary: PlantStateProjectionSummary,
}

/// Writes a deterministic diagnostic projection of plant loop and first plant
/// equipment output rows.
///
/// This function intentionally does not claim EnergyPlus algorithm parity. It
/// maps the typed plant-loop graph and branch component order to native
/// `ResultStore` series so the port can exercise plant output registration and
/// artifact plumbing before plant loop manager algorithms are ported.
pub fn simulate_plant_state_projection(
    model: &SimulationModel,
    options: PlantStateProjectionOptions,
) -> Result<PlantStateProjection, RuntimeError> {
    if model.typed.plant_loops.is_empty() {
        return Err(RuntimeError::NoPlantStateProjectionLoops);
    }

    let loop_states: Vec<_> = model
        .typed
        .plant_loops
        .iter()
        .map(|plant_loop| plant_loop_state(model, plant_loop, options))
        .collect();
    let equipment_states = plant_equipment_states(model, options);

    let state = PlantStateStore {
        loops: loop_states,
        equipment: equipment_states,
    };

    let mut results = ResultStore::new();
    let mut handle_index = 0_u32;
    for loop_state in &state.loops {
        add_constant_output_series(
            &mut results,
            &mut handle_index,
            &loop_state.loop_name,
            "Plant Supply Side Cooling Demand Rate",
            "W",
            loop_state.supply_side_cooling_demand_w,
            options.sample_count,
        );
        add_constant_output_series(
            &mut results,
            &mut handle_index,
            &loop_state.loop_name,
            "Plant Supply Side Heating Demand Rate",
            "W",
            loop_state.supply_side_heating_demand_w,
            options.sample_count,
        );
        add_constant_output_series(
            &mut results,
            &mut handle_index,
            &loop_state.loop_name,
            "Plant Supply Side Inlet Mass Flow Rate",
            "kg/s",
            loop_state.supply_side_inlet_mass_flow_rate_kg_per_s,
            options.sample_count,
        );
        add_constant_output_series(
            &mut results,
            &mut handle_index,
            &loop_state.loop_name,
            "Plant Supply Side Inlet Temperature",
            "C",
            loop_state.supply_side_inlet_temperature_c,
            options.sample_count,
        );
        add_constant_output_series(
            &mut results,
            &mut handle_index,
            &loop_state.loop_name,
            "Plant Supply Side Outlet Temperature",
            "C",
            loop_state.supply_side_outlet_temperature_c,
            options.sample_count,
        );
    }

    for equipment_state in &state.equipment {
        add_constant_output_series(
            &mut results,
            &mut handle_index,
            &equipment_state.equipment_name,
            plant_equipment_variable_name(equipment_state.role),
            "W",
            equipment_state.output_rate_w,
            options.sample_count,
        );
    }

    Ok(PlantStateProjection {
        summary: PlantStateProjectionSummary {
            samples: options.sample_count,
            loop_count: state.loop_count(),
            equipment_count: state.equipment_count(),
            series_count: results.series.len(),
            evidence_policy: PlantStateProjectionEvidencePolicy::diagnostic(),
            loops: state
                .loops
                .iter()
                .map(|loop_state| PlantStateProjectionLoop {
                    loop_id: loop_state.loop_id,
                    loop_name: loop_state.loop_name.clone(),
                    supply_inlet_node_name: loop_state.supply_inlet_node_name.clone(),
                    supply_outlet_node_name: loop_state.supply_outlet_node_name.clone(),
                })
                .collect(),
            equipment: state
                .equipment
                .iter()
                .map(|equipment| PlantStateProjectionEquipment {
                    object_type: equipment.object_type.clone(),
                    equipment_name: equipment.equipment_name.clone(),
                    role: equipment.role,
                })
                .collect(),
        },
        state,
        results,
    })
}

fn plant_loop_state(
    model: &SimulationModel,
    plant_loop: &PlantLoop,
    options: PlantStateProjectionOptions,
) -> PlantLoopState {
    PlantLoopState {
        loop_id: plant_loop.id,
        loop_name: plant_loop.name.0.clone(),
        supply_inlet_node_name: node_name_for_id(&model.typed, plant_loop.plant_side_inlet_node)
            .unwrap_or_else(|| format!("NODE {}", plant_loop.plant_side_inlet_node.0)),
        supply_outlet_node_name: node_name_for_id(&model.typed, plant_loop.plant_side_outlet_node)
            .unwrap_or_else(|| format!("NODE {}", plant_loop.plant_side_outlet_node.0)),
        supply_side_cooling_demand_w: options.default_cooling_demand_w,
        supply_side_heating_demand_w: options.default_heating_demand_w,
        supply_side_inlet_mass_flow_rate_kg_per_s: options.default_inlet_mass_flow_rate_kg_per_s,
        supply_side_inlet_temperature_c: options.default_inlet_temperature_c,
        supply_side_outlet_temperature_c: options.default_outlet_temperature_c,
    }
}

fn plant_equipment_states(
    model: &SimulationModel,
    options: PlantStateProjectionOptions,
) -> Vec<PlantEquipmentState> {
    let mut equipment = Vec::new();
    for plant_loop in &model.typed.plant_loops {
        for branch_list in [
            plant_loop.plant_side_branch_list,
            plant_loop.demand_side_branch_list,
        ] {
            for branch_id in plant_branch_ids_for_list(&model.typed, branch_list) {
                let Some(branch) = model
                    .typed
                    .plant_branches
                    .iter()
                    .find(|branch| branch.id == branch_id)
                else {
                    continue;
                };
                for component in &branch.components {
                    push_plant_equipment_state(&mut equipment, component, options);
                }
            }
        }
    }
    equipment
}

fn plant_branch_ids_for_list(model: &TypedModel, branch_list_id: BranchListId) -> Vec<BranchId> {
    model
        .plant_branch_lists
        .iter()
        .find(|list| list.id == branch_list_id)
        .map(|list| list.branches.clone())
        .unwrap_or_default()
}

fn push_plant_equipment_state(
    equipment: &mut Vec<PlantEquipmentState>,
    component: &PlantBranchComponent,
    options: PlantStateProjectionOptions,
) {
    let Some(role) = plant_equipment_role(&component.object_type.0) else {
        return;
    };
    if equipment
        .iter()
        .any(|existing| existing.equipment_name == component.name.0 && existing.role == role)
    {
        return;
    }

    equipment.push(PlantEquipmentState {
        object_type: component.object_type.0.clone(),
        equipment_name: component.name.0.clone(),
        role,
        output_rate_w: plant_equipment_output_rate_w(role, options),
    });
}

fn plant_equipment_role(object_type: &str) -> Option<PlantEquipmentRole> {
    match object_type.to_ascii_lowercase().as_str() {
        "pump:constantspeed" | "pump:variablespeed" => Some(PlantEquipmentRole::Pump),
        "districtheating:water" => Some(PlantEquipmentRole::PurchasedHeating),
        "loadprofile:plant" => Some(PlantEquipmentRole::LoadProfile),
        _ => None,
    }
}

fn plant_equipment_output_rate_w(
    role: PlantEquipmentRole,
    options: PlantStateProjectionOptions,
) -> f64 {
    match role {
        PlantEquipmentRole::Pump => options.default_pump_electricity_rate_w,
        PlantEquipmentRole::PurchasedHeating => options.default_district_heating_rate_w,
        PlantEquipmentRole::LoadProfile => options.default_load_profile_heat_transfer_rate_w,
    }
}

fn plant_equipment_variable_name(role: PlantEquipmentRole) -> &'static str {
    match role {
        PlantEquipmentRole::Pump => "Pump Electricity Rate",
        PlantEquipmentRole::PurchasedHeating => "District Heating Water Rate",
        PlantEquipmentRole::LoadProfile => "Plant Load Profile Heat Transfer Rate",
    }
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

fn node_name_for_id(model: &TypedModel, node_id: NodeId) -> Option<String> {
    model
        .nodes
        .iter()
        .find(|node| node.id == node_id)
        .map(|node| node.name.0.clone())
}

#[cfg(test)]
mod tests {
    use super::{
        PLANT_STATE_SOURCE_MAP_PATH, PlantEquipmentRole, PlantStateProjectionOptions,
        simulate_plant_state_projection,
    };
    use ep_model::{
        BranchId, BranchListId, LoopId, Node, NodeId, NormalizedName, PlantBranch,
        PlantBranchComponent, PlantBranchList, PlantLoop, SimulationModel, TypedModel,
    };

    #[test]
    fn plant_state_projection_writes_diagnostic_loop_and_equipment_series()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = plant_state_projection_model();

        let projection = simulate_plant_state_projection(
            &model,
            PlantStateProjectionOptions::hourly_samples(48),
        )?;

        assert_eq!(projection.summary.samples, 48);
        assert_eq!(projection.summary.loop_count, 1);
        assert_eq!(projection.summary.equipment_count, 3);
        assert_eq!(projection.summary.series_count, 8);
        assert_eq!(projection.results.sample_count(), 48);
        assert_eq!(projection.results.series.len(), 8);
        assert_eq!(
            projection.summary.evidence_policy.source_map_path,
            PLANT_STATE_SOURCE_MAP_PATH
        );
        assert_eq!(projection.summary.loops[0].loop_name, "MAIN LOOP");
        assert_eq!(
            projection.summary.loops[0].supply_inlet_node_name,
            "SUPPLY INLET NODE"
        );
        assert_eq!(
            projection.summary.loops[0].supply_outlet_node_name,
            "SUPPLY OUTLET NODE"
        );

        let roles: Vec<_> = projection
            .summary
            .equipment
            .iter()
            .map(|equipment| (equipment.equipment_name.as_str(), equipment.role))
            .collect();
        assert_eq!(
            roles,
            vec![
                ("PUMP", PlantEquipmentRole::Pump),
                ("PURCHASED HEATING", PlantEquipmentRole::PurchasedHeating),
                ("LOAD PROFILE 1", PlantEquipmentRole::LoadProfile),
            ]
        );

        for (key, variable) in [
            ("MAIN LOOP", "Plant Supply Side Cooling Demand Rate"),
            ("MAIN LOOP", "Plant Supply Side Heating Demand Rate"),
            ("MAIN LOOP", "Plant Supply Side Inlet Mass Flow Rate"),
            ("MAIN LOOP", "Plant Supply Side Inlet Temperature"),
            ("MAIN LOOP", "Plant Supply Side Outlet Temperature"),
            ("PUMP", "Pump Electricity Rate"),
            ("PURCHASED HEATING", "District Heating Water Rate"),
            ("LOAD PROFILE 1", "Plant Load Profile Heat Transfer Rate"),
        ] {
            let Some(series) = projection.results.find_series(key, variable) else {
                return Err(std::io::Error::other(format!("missing {key} / {variable}")).into());
            };
            assert_eq!(series.values.len(), 48);
            assert!(series.values.iter().all(|value| value.abs() > 1.0e-9));
        }

        Ok(())
    }

    fn plant_state_projection_model() -> SimulationModel {
        let mut typed = TypedModel::default();
        let supply_inlet = push_node(&mut typed, "Supply Inlet Node");
        let pump_outlet = push_node(&mut typed, "Supply Pump-Heating Node");
        let supply_outlet = push_node(&mut typed, "Supply Outlet Node");
        let demand_inlet = push_node(&mut typed, "Demand Inlet Node");
        let load_profile_inlet = push_node(&mut typed, "Demand Load Profile 1 Inlet Node");
        let load_profile_outlet = push_node(&mut typed, "Demand Load Profile 1 Outlet Node");
        let demand_outlet = push_node(&mut typed, "Demand Outlet Node");

        typed.plant_branches.extend([
            PlantBranch {
                id: BranchId(0),
                name: NormalizedName::new("Supply Inlet Branch"),
                components: vec![PlantBranchComponent {
                    object_type: NormalizedName::new("Pump:VariableSpeed"),
                    name: NormalizedName::new("Pump"),
                    inlet_node: supply_inlet,
                    outlet_node: pump_outlet,
                }],
            },
            PlantBranch {
                id: BranchId(1),
                name: NormalizedName::new("Heating Branch"),
                components: vec![PlantBranchComponent {
                    object_type: NormalizedName::new("DistrictHeating:Water"),
                    name: NormalizedName::new("Purchased Heating"),
                    inlet_node: pump_outlet,
                    outlet_node: supply_outlet,
                }],
            },
            PlantBranch {
                id: BranchId(2),
                name: NormalizedName::new("Load Profile Branch 1"),
                components: vec![PlantBranchComponent {
                    object_type: NormalizedName::new("LoadProfile:Plant"),
                    name: NormalizedName::new("Load Profile 1"),
                    inlet_node: load_profile_inlet,
                    outlet_node: load_profile_outlet,
                }],
            },
        ]);
        typed.plant_branch_lists.extend([
            PlantBranchList {
                id: BranchListId(0),
                name: NormalizedName::new("Supply Branches"),
                branches: vec![BranchId(0), BranchId(1)],
            },
            PlantBranchList {
                id: BranchListId(1),
                name: NormalizedName::new("Demand Branches"),
                branches: vec![BranchId(2)],
            },
        ]);
        typed.plant_loops.push(PlantLoop {
            id: LoopId(0),
            name: NormalizedName::new("Main Loop"),
            fluid_type: NormalizedName::new("Water"),
            plant_side_inlet_node: supply_inlet,
            plant_side_outlet_node: supply_outlet,
            plant_side_branch_list: BranchListId(0),
            plant_side_connector_list: None,
            demand_side_inlet_node: demand_inlet,
            demand_side_outlet_node: demand_outlet,
            demand_side_branch_list: BranchListId(1),
            demand_side_connector_list: None,
            load_distribution_scheme: Some(NormalizedName::new("SequentialLoad")),
        });

        SimulationModel::from_typed(typed)
    }

    fn push_node(model: &mut TypedModel, name: &str) -> NodeId {
        let id = NodeId(model.nodes.len() as u32);
        model.nodes.push(Node {
            id,
            name: NormalizedName::new(name),
        });
        model.node_names.insert(name, id);
        id
    }
}
