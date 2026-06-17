//! Zone equipment demand state used by compatibility-mode HVAC components.

use ep_model::{
    IdealLoadsAirSystemId, NodeId, NormalizedName, SimulationModel, TypedModel,
    ZoneEquipmentListId, ZoneId,
};

/// EnergyPlus `ZoneSysEnergyDemand` subset needed by the first IdealLoads path.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ZoneSysEnergyDemand {
    /// Controlled zone.
    pub zone: ZoneId,
    /// EnergyPlus `RemainingOutputReqToHeatSP` equivalent in W.
    pub remaining_output_req_to_heat_sp_w: f64,
    /// EnergyPlus `RemainingOutputReqToCoolSP` equivalent in W.
    pub remaining_output_req_to_cool_sp_w: f64,
    /// Moisture demand to humidifying setpoint. Inactive in the first subset.
    pub remaining_output_req_to_humid_sp_kg_per_s: f64,
    /// Moisture demand to dehumidifying setpoint. Inactive in the first subset.
    pub remaining_output_req_to_dehumid_sp_kg_per_s: f64,
}

impl ZoneSysEnergyDemand {
    /// Creates a sensible-only zone demand snapshot.
    #[must_use]
    pub const fn sensible_only(
        zone: ZoneId,
        remaining_output_req_to_heat_sp_w: f64,
        remaining_output_req_to_cool_sp_w: f64,
    ) -> Self {
        Self {
            zone,
            remaining_output_req_to_heat_sp_w,
            remaining_output_req_to_cool_sp_w,
            remaining_output_req_to_humid_sp_kg_per_s: 0.0,
            remaining_output_req_to_dehumid_sp_kg_per_s: 0.0,
        }
    }

    /// Returns true when moisture demand branches are inactive.
    #[must_use]
    pub fn has_inactive_moisture_demand(self) -> bool {
        self.remaining_output_req_to_humid_sp_kg_per_s.abs() <= f64::EPSILON
            && self.remaining_output_req_to_dehumid_sp_kg_per_s.abs() <= f64::EPSILON
    }
}

/// Source-order entry point reserved for zone equipment orchestration.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ZoneEquipmentCompatibilityStage {
    /// Stable Rust stage name.
    pub stage_name: &'static str,
    /// EnergyPlus source file.
    pub source_file: &'static str,
    /// EnergyPlus source routine.
    pub source_routine: &'static str,
}

/// EnergyPlus zone equipment order relevant to PurchasedAir/IdealLoads.
#[must_use]
pub const fn ideal_loads_zone_equipment_stages() -> [ZoneEquipmentCompatibilityStage; 3] {
    [
        ZoneEquipmentCompatibilityStage {
            stage_name: "manage-zone-equipment",
            source_file: "src/EnergyPlus/ZoneEquipmentManager.cc",
            source_routine: "ManageZoneEquipment",
        },
        ZoneEquipmentCompatibilityStage {
            stage_name: "simulate-zone-equipment",
            source_file: "src/EnergyPlus/ZoneEquipmentManager.cc",
            source_routine: "SimZoneEquipment",
        },
        ZoneEquipmentCompatibilityStage {
            stage_name: "simulate-purchased-air",
            source_file: "src/EnergyPlus/PurchasedAirManager.cc",
            source_routine: "SimPurchasedAir",
        },
    ]
}

/// Source-equivalent dispatch path for IdealLoads through zone equipment.
pub const IDEAL_LOADS_ZONE_EQUIPMENT_DISPATCH_PATH: &str = "ZoneEquipmentManager::ManageZoneEquipment -> SimZoneEquipment -> ZoneEquipType::PurchasedAir -> PurchasedAirManager::SimPurchasedAir";

/// Blocking issue in the IdealLoads zone-equipment dispatch prerequisites.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IdealLoadsZoneEquipmentDispatchIssue {
    /// The IdealLoads system is not present in any typed `ZoneHVAC:EquipmentList` edge.
    MissingZoneEquipmentListEntry,
    /// The selected graph edge does not have a matching `ZoneHVAC:EquipmentConnections` object.
    MissingZoneEquipmentConnection,
    /// The zone equipment connection has no inlet node or node-list reference.
    MissingZoneInletNodeList,
    /// The IdealLoads supply node name did not resolve to a typed node.
    MissingIdealLoadsSupplyNode,
    /// At least one resolved IdealLoads supply node is not included in the zone inlet list.
    SupplyNodeNotInZoneInletList,
    /// The same IdealLoads system is connected through more than one zone equipment edge.
    MultipleEdgesForIdealLoadsSystem,
    /// Two IdealLoads systems in the same zone share the same heating/cooling sequence pair.
    SequenceAmbiguity,
}

impl IdealLoadsZoneEquipmentDispatchIssue {
    /// Stable machine-readable issue code.
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::MissingZoneEquipmentListEntry => "missing_zone_equipment_list_entry",
            Self::MissingZoneEquipmentConnection => "missing_zone_equipment_connection",
            Self::MissingZoneInletNodeList => "missing_zone_inlet_node_list",
            Self::MissingIdealLoadsSupplyNode => "missing_ideal_loads_supply_node",
            Self::SupplyNodeNotInZoneInletList => "supply_node_not_in_zone_inlet_list",
            Self::MultipleEdgesForIdealLoadsSystem => "multiple_edges_for_ideal_loads_system",
            Self::SequenceAmbiguity => "sequence_ambiguity",
        }
    }
}

/// Diagnostic-only warning in the IdealLoads zone-equipment dispatch prerequisites.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IdealLoadsZoneEquipmentDispatchWarning {
    /// More than one IdealLoads system is connected to the same zone.
    MultipleZoneEquipmentDiagnosticOnly,
}

impl IdealLoadsZoneEquipmentDispatchWarning {
    /// Stable machine-readable warning code.
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::MultipleZoneEquipmentDiagnosticOnly => "multiple_zone_equipment_diagnostic_only",
        }
    }
}

/// Validation evidence for dispatching IdealLoads through ZoneEquipmentManager.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdealLoadsZoneEquipmentDispatchValidation {
    /// IdealLoads system being dispatched.
    pub ideal_loads_air_system: IdealLoadsAirSystemId,
    /// Controlled zone resolved through `ZoneHVAC:EquipmentConnections`.
    pub zone: Option<ZoneId>,
    /// Zone equipment list resolved through `ZoneHVAC:EquipmentConnections`.
    pub equipment_list: Option<ZoneEquipmentListId>,
    /// Cooling sequence fixed at compile/graph construction time.
    pub cooling_sequence: Option<u32>,
    /// Heating-or-no-load sequence fixed at compile/graph construction time.
    pub heating_or_no_load_sequence: Option<u32>,
    /// IdealLoads supply nodes resolved to typed `NodeId`s.
    pub supply_nodes: Vec<NodeId>,
    /// Zone inlet nodes resolved from the connection node or node-list.
    pub zone_inlet_nodes: Vec<NodeId>,
    /// Blocking issues.
    pub issues: Vec<IdealLoadsZoneEquipmentDispatchIssue>,
    /// Diagnostic-only scope warnings.
    pub warnings: Vec<IdealLoadsZoneEquipmentDispatchWarning>,
}

impl IdealLoadsZoneEquipmentDispatchValidation {
    /// True when the typed model can dispatch this IdealLoads object through zone equipment.
    #[must_use]
    pub fn is_dispatchable(&self) -> bool {
        self.issues.is_empty()
    }

    /// True when the current scope is narrow enough for a conformance candidate.
    #[must_use]
    pub fn is_conformance_candidate(&self) -> bool {
        self.issues.is_empty() && self.warnings.is_empty()
    }

    /// Stable pass/fail label for dispatch validation.
    #[must_use]
    pub fn dispatch_status_label(&self) -> &'static str {
        if self.is_dispatchable() {
            "pass"
        } else {
            "fail"
        }
    }

    /// Stable label for conformance-candidate scope.
    #[must_use]
    pub fn conformance_candidate_status_label(&self) -> &'static str {
        if self.is_conformance_candidate() {
            "pass"
        } else {
            "diagnostic-only"
        }
    }

    /// Stable label for the zone-equipment scope.
    #[must_use]
    pub fn scope_label(&self) -> &'static str {
        if self.warnings.is_empty() {
            "single-zone-single-equipment"
        } else {
            "multiple-zone-equipment-diagnostic-only"
        }
    }

    /// Stable issue codes for artifacts.
    #[must_use]
    pub fn issue_codes(&self) -> Vec<&'static str> {
        self.issues.iter().map(|issue| issue.code()).collect()
    }

    /// Stable warning codes for artifacts.
    #[must_use]
    pub fn warning_codes(&self) -> Vec<&'static str> {
        self.warnings.iter().map(|warning| warning.code()).collect()
    }
}

/// Validates that an IdealLoads system can be reached through the typed
/// ZoneEquipmentManager-compatible dispatch path.
#[must_use]
pub fn validate_ideal_loads_zone_equipment_dispatch(
    model: &SimulationModel,
    ideal_loads_air_system: IdealLoadsAirSystemId,
) -> IdealLoadsZoneEquipmentDispatchValidation {
    let mut issues = Vec::new();
    let mut warnings = Vec::new();
    let mut matching_edges = model
        .graph
        .zone_ideal_loads
        .iter()
        .filter(|edge| edge.ideal_loads_air_system == ideal_loads_air_system);
    let first_edge = matching_edges.next();
    if first_edge.is_none() {
        push_issue(
            &mut issues,
            IdealLoadsZoneEquipmentDispatchIssue::MissingZoneEquipmentListEntry,
        );
    }
    if matching_edges.next().is_some() {
        push_issue(
            &mut issues,
            IdealLoadsZoneEquipmentDispatchIssue::MultipleEdgesForIdealLoadsSystem,
        );
    }

    let supply_nodes = model
        .graph
        .ideal_loads_supply_nodes
        .iter()
        .filter(|edge| edge.ideal_loads_air_system == ideal_loads_air_system)
        .map(|edge| edge.node)
        .collect::<Vec<_>>();
    if supply_nodes.is_empty() {
        push_issue(
            &mut issues,
            IdealLoadsZoneEquipmentDispatchIssue::MissingIdealLoadsSupplyNode,
        );
    }

    let (zone, equipment_list, cooling_sequence, heating_or_no_load_sequence, zone_inlet_nodes) =
        if let Some(edge) = first_edge {
            let same_zone_edges = model
                .graph
                .zone_ideal_loads
                .iter()
                .filter(|candidate| candidate.zone == edge.zone)
                .collect::<Vec<_>>();
            if same_zone_edges.len() > 1 {
                push_warning(
                    &mut warnings,
                    IdealLoadsZoneEquipmentDispatchWarning::MultipleZoneEquipmentDiagnosticOnly,
                );
            }
            if same_zone_edges
                .iter()
                .filter(|candidate| {
                    candidate.cooling_sequence == edge.cooling_sequence
                        && candidate.heating_or_no_load_sequence == edge.heating_or_no_load_sequence
                })
                .count()
                > 1
            {
                push_issue(
                    &mut issues,
                    IdealLoadsZoneEquipmentDispatchIssue::SequenceAmbiguity,
                );
            }

            let zone_inlet_nodes = if let Some(connection) = model
                .typed
                .zone_equipment_connections
                .iter()
                .find(|connection| {
                    connection.zone == edge.zone && connection.equipment_list == edge.equipment_list
                }) {
                if let Some(name) = connection.zone_air_inlet_node_or_nodelist_name.as_ref() {
                    resolve_node_or_nodelist(&model.typed, name)
                } else {
                    push_issue(
                        &mut issues,
                        IdealLoadsZoneEquipmentDispatchIssue::MissingZoneInletNodeList,
                    );
                    Vec::new()
                }
            } else {
                push_issue(
                    &mut issues,
                    IdealLoadsZoneEquipmentDispatchIssue::MissingZoneEquipmentConnection,
                );
                Vec::new()
            };

            (
                Some(edge.zone),
                Some(edge.equipment_list),
                Some(edge.cooling_sequence),
                Some(edge.heating_or_no_load_sequence),
                zone_inlet_nodes,
            )
        } else {
            (None, None, None, None, Vec::new())
        };

    if !supply_nodes.is_empty() && !zone_inlet_nodes.is_empty() {
        for supply_node in &supply_nodes {
            if !zone_inlet_nodes.contains(supply_node) {
                push_issue(
                    &mut issues,
                    IdealLoadsZoneEquipmentDispatchIssue::SupplyNodeNotInZoneInletList,
                );
                break;
            }
        }
    }

    IdealLoadsZoneEquipmentDispatchValidation {
        ideal_loads_air_system,
        zone,
        equipment_list,
        cooling_sequence,
        heating_or_no_load_sequence,
        supply_nodes,
        zone_inlet_nodes,
        issues,
        warnings,
    }
}

fn resolve_node_or_nodelist(model: &TypedModel, name: &NormalizedName) -> Vec<NodeId> {
    if let Some(node) = model.node_names.resolve(&name.0) {
        return vec![node];
    }
    if let Some(node_list) = model.node_list_names.resolve(&name.0)
        && let Some(list) = model.node_lists.iter().find(|list| list.id == node_list)
    {
        return list.nodes.clone();
    }
    Vec::new()
}

fn push_issue(
    issues: &mut Vec<IdealLoadsZoneEquipmentDispatchIssue>,
    issue: IdealLoadsZoneEquipmentDispatchIssue,
) {
    if !issues.contains(&issue) {
        issues.push(issue);
    }
}

fn push_warning(
    warnings: &mut Vec<IdealLoadsZoneEquipmentDispatchWarning>,
    warning: IdealLoadsZoneEquipmentDispatchWarning,
) {
    if !warnings.contains(&warning) {
        warnings.push(warning);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ep_model::{
        AutoOrNumber, AutosizeOrNumber, DehumidificationControlType,
        DemandControlledVentilationType, HeatRecoveryType, HumidificationControlType,
        IdealLoadsAirSystem, IdealLoadsFuelType, IdealLoadsLimit, LoadDistributionScheme, Node,
        NodeList, NodeListId, OutdoorAirEconomizerType, Point3, SimulationModel, TypedModel, Zone,
        ZoneEquipmentConnection, ZoneEquipmentConnectionId, ZoneEquipmentList,
        ZoneEquipmentListEntry, ZoneEquipmentObjectType,
    };

    #[test]
    fn zone_equipment_dispatch_validation_accepts_single_equipment_path() {
        let model = single_ideal_loads_model("Zone Inlet", "Zone Inlet");

        let validation =
            validate_ideal_loads_zone_equipment_dispatch(&model, IdealLoadsAirSystemId(0));

        assert!(validation.is_dispatchable());
        assert!(validation.is_conformance_candidate());
        assert_eq!(validation.dispatch_status_label(), "pass");
        assert_eq!(validation.conformance_candidate_status_label(), "pass");
        assert_eq!(validation.scope_label(), "single-zone-single-equipment");
        assert_eq!(validation.zone, Some(ZoneId(0)));
        assert_eq!(validation.equipment_list, Some(ZoneEquipmentListId(0)));
        assert_eq!(validation.cooling_sequence, Some(1));
        assert_eq!(validation.heating_or_no_load_sequence, Some(1));
        assert_eq!(validation.supply_nodes, vec![NodeId(0)]);
        assert_eq!(validation.zone_inlet_nodes, vec![NodeId(0)]);
        assert!(validation.issue_codes().is_empty());
        assert!(validation.warning_codes().is_empty());
    }

    #[test]
    fn zone_equipment_dispatch_validation_rejects_supply_node_outside_zone_inlet_list() {
        let model = single_ideal_loads_model("Ideal Loads Supply", "Zone Inlet");

        let validation =
            validate_ideal_loads_zone_equipment_dispatch(&model, IdealLoadsAirSystemId(0));

        assert!(!validation.is_dispatchable());
        assert_eq!(
            validation.issues,
            vec![IdealLoadsZoneEquipmentDispatchIssue::SupplyNodeNotInZoneInletList]
        );
        assert_eq!(
            validation.issue_codes(),
            vec!["supply_node_not_in_zone_inlet_list"]
        );
    }

    #[test]
    fn zone_equipment_dispatch_validation_marks_multiple_equipment_diagnostic_only() {
        let mut typed = single_ideal_loads_typed_model("Zone Inlet", "Zone Inlet");
        typed.nodes.push(Node {
            id: NodeId(3),
            name: NormalizedName::new("Second Zone Inlet"),
        });
        typed.node_names.insert("Second Zone Inlet", NodeId(3));
        typed.ideal_loads_air_systems.push(ideal_loads_system(
            IdealLoadsAirSystemId(1),
            "Second Ideal Loads",
            "Second Zone Inlet",
        ));
        typed.zone_equipment_lists[0]
            .equipment
            .push(ZoneEquipmentListEntry {
                object_type: ZoneEquipmentObjectType::IdealLoadsAirSystem,
                ideal_loads_air_system: IdealLoadsAirSystemId(1),
                cooling_sequence: 2,
                heating_or_no_load_sequence: 2,
                sequential_cooling_fraction_schedule: None,
                sequential_heating_fraction_schedule: None,
            });

        let model = SimulationModel::from_typed(typed);
        let validation =
            validate_ideal_loads_zone_equipment_dispatch(&model, IdealLoadsAirSystemId(0));

        assert!(validation.is_dispatchable());
        assert!(!validation.is_conformance_candidate());
        assert_eq!(
            validation.warnings,
            vec![IdealLoadsZoneEquipmentDispatchWarning::MultipleZoneEquipmentDiagnosticOnly]
        );
        assert_eq!(
            validation.warning_codes(),
            vec!["multiple_zone_equipment_diagnostic_only"]
        );
        assert_eq!(
            validation.conformance_candidate_status_label(),
            "diagnostic-only"
        );
    }

    #[test]
    fn zone_equipment_dispatch_validation_rejects_sequence_ambiguity() {
        let mut typed = single_ideal_loads_typed_model("Zone Inlet", "Zone Inlet");
        typed.nodes.push(Node {
            id: NodeId(3),
            name: NormalizedName::new("Second Zone Inlet"),
        });
        typed.node_names.insert("Second Zone Inlet", NodeId(3));
        typed.ideal_loads_air_systems.push(ideal_loads_system(
            IdealLoadsAirSystemId(1),
            "Second Ideal Loads",
            "Second Zone Inlet",
        ));
        typed.zone_equipment_lists[0]
            .equipment
            .push(ZoneEquipmentListEntry {
                object_type: ZoneEquipmentObjectType::IdealLoadsAirSystem,
                ideal_loads_air_system: IdealLoadsAirSystemId(1),
                cooling_sequence: 1,
                heating_or_no_load_sequence: 1,
                sequential_cooling_fraction_schedule: None,
                sequential_heating_fraction_schedule: None,
            });

        let model = SimulationModel::from_typed(typed);
        let validation =
            validate_ideal_loads_zone_equipment_dispatch(&model, IdealLoadsAirSystemId(0));

        assert!(!validation.is_dispatchable());
        assert!(
            validation
                .issues
                .contains(&IdealLoadsZoneEquipmentDispatchIssue::SequenceAmbiguity)
        );
    }

    fn single_ideal_loads_model(system_supply: &str, connection_inlet: &str) -> SimulationModel {
        SimulationModel::from_typed(single_ideal_loads_typed_model(
            system_supply,
            connection_inlet,
        ))
    }

    fn single_ideal_loads_typed_model(system_supply: &str, connection_inlet: &str) -> TypedModel {
        let mut typed = TypedModel::default();
        typed.zones.push(Zone {
            id: ZoneId(0),
            name: NormalizedName::new("Zone One"),
            direction_of_relative_north_deg: 0.0,
            origin: Point3 {
                x_m: 0.0,
                y_m: 0.0,
                z_m: 0.0,
            },
            zone_type: 1,
            multiplier: 1,
            ceiling_height: AutoOrNumber::AutoCalculate,
            volume: AutoOrNumber::AutoCalculate,
        });
        for (id, name) in [
            (NodeId(0), "Zone Inlet"),
            (NodeId(1), "Ideal Loads Supply"),
            (NodeId(2), "Zone Air Node"),
        ] {
            typed.nodes.push(Node {
                id,
                name: NormalizedName::new(name),
            });
            typed.node_names.insert(name, id);
        }
        typed.node_lists.push(NodeList {
            id: NodeListId(0),
            name: NormalizedName::new("Zone Inlet List"),
            nodes: vec![NodeId(0)],
        });
        typed
            .node_list_names
            .insert("Zone Inlet List", NodeListId(0));
        typed.ideal_loads_air_systems.push(ideal_loads_system(
            IdealLoadsAirSystemId(0),
            "Zone Ideal Loads",
            system_supply,
        ));
        typed.zone_equipment_lists.push(ZoneEquipmentList {
            id: ZoneEquipmentListId(0),
            name: NormalizedName::new("Zone Equipment"),
            load_distribution_scheme: LoadDistributionScheme::SequentialLoad,
            equipment: vec![ZoneEquipmentListEntry {
                object_type: ZoneEquipmentObjectType::IdealLoadsAirSystem,
                ideal_loads_air_system: IdealLoadsAirSystemId(0),
                cooling_sequence: 1,
                heating_or_no_load_sequence: 1,
                sequential_cooling_fraction_schedule: None,
                sequential_heating_fraction_schedule: None,
            }],
        });
        typed
            .zone_equipment_connections
            .push(ZoneEquipmentConnection {
                id: ZoneEquipmentConnectionId(0),
                zone: ZoneId(0),
                equipment_list: ZoneEquipmentListId(0),
                zone_air_inlet_node_or_nodelist_name: Some(NormalizedName::new(connection_inlet)),
                zone_air_exhaust_node_or_nodelist_name: None,
                zone_air_node_name: NormalizedName::new("Zone Air Node"),
                zone_return_air_node_or_nodelist_name: None,
                zone_return_air_node_1_flow_rate_fraction_schedule: None,
                zone_return_air_node_1_flow_rate_basis_node_or_nodelist_name: None,
            });
        typed
    }

    fn ideal_loads_system(
        id: IdealLoadsAirSystemId,
        name: &str,
        supply_node_name: &str,
    ) -> IdealLoadsAirSystem {
        IdealLoadsAirSystem {
            id,
            name: NormalizedName::new(name),
            availability_schedule: None,
            zone_supply_air_node_name: NormalizedName::new(supply_node_name),
            zone_exhaust_air_node_name: None,
            system_inlet_air_node_name: None,
            maximum_heating_supply_air_temperature_c: 50.0,
            minimum_cooling_supply_air_temperature_c: 13.0,
            maximum_heating_supply_air_humidity_ratio: 0.0156,
            minimum_cooling_supply_air_humidity_ratio: 0.0077,
            heating_limit: IdealLoadsLimit::NoLimit,
            maximum_heating_air_flow_rate_m3_per_s: Some(AutosizeOrNumber::Value(0.25)),
            maximum_sensible_heating_capacity_w: None,
            cooling_limit: IdealLoadsLimit::NoLimit,
            maximum_cooling_air_flow_rate_m3_per_s: None,
            maximum_total_cooling_capacity_w: None,
            heating_availability_schedule: None,
            cooling_availability_schedule: None,
            dehumidification_control_type: DehumidificationControlType::ConstantSensibleHeatRatio,
            cooling_sensible_heat_ratio: 0.7,
            humidification_control_type: HumidificationControlType::None,
            design_specification_outdoor_air_object_name: None,
            outdoor_air_inlet_node_name: None,
            demand_controlled_ventilation_type: DemandControlledVentilationType::None,
            outdoor_air_economizer_type: OutdoorAirEconomizerType::NoEconomizer,
            heat_recovery_type: HeatRecoveryType::None,
            sensible_heat_recovery_effectiveness: 0.7,
            latent_heat_recovery_effectiveness: 0.65,
            design_specification_zonehvac_sizing_object_name: None,
            heating_fuel_efficiency_schedule: None,
            heating_fuel_type: IdealLoadsFuelType::DistrictHeatingWater,
            cooling_fuel_efficiency_schedule: None,
            cooling_fuel_type: IdealLoadsFuelType::DistrictCooling,
        }
    }
}
