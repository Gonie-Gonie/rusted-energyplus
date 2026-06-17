//! Zone equipment dispatch validation for IdealLoads compatibility.

use ep_model::{
    IdealLoadsAirSystemId, NodeId, NormalizedName, SimulationModel, TypedModel,
    ZoneEquipmentListId, ZoneId,
};

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
