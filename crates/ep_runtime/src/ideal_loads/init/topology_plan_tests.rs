use ep_model::{
    DesignSpecificationOutdoorAirId, IdealLoadsLimit, NodeId, NormalizedName,
    OutdoorAirEconomizerType, TypedModel, ZoneEquipmentConnection, ZoneEquipmentConnectionId,
    ZoneEquipmentListId, ZoneId,
};

use super::lifecycle_tests::{SYSTEM, finite_flow_system};
use super::*;

#[test]
fn blank_exhaust_with_one_return_assigns_the_return_node() {
    let system = finite_flow_system();
    let evaluation = plan(None, vec![], vec![NodeId(8)], false).evaluate(&system);
    assert_eq!(
        evaluation.outcome,
        Ok(PurchasedAirInitTopologyOutcome {
            recirculation_node: Some(NodeId(8)),
            recirculation_source: PurchasedAirRecirculationSource::SingleZoneReturn,
            rejected_exhaust_node: None,
            reported_first_return_node: None,
        })
    );
    assert!(evaluation.diagnostics.is_empty());
    assert!(!evaluation.economizer_flow_limit_warning);
}

#[test]
fn valid_exhaust_bypasses_an_empty_return_arena() {
    let system = finite_flow_system();
    let evaluation = plan(Some(NodeId(4)), vec![NodeId(4)], vec![], false).evaluate(&system);
    assert_eq!(
        evaluation.outcome,
        Ok(PurchasedAirInitTopologyOutcome {
            recirculation_node: Some(NodeId(4)),
            recirculation_source: PurchasedAirRecirculationSource::ConfiguredZoneExhaust,
            rejected_exhaust_node: None,
            reported_first_return_node: None,
        })
    );
    assert!(evaluation.diagnostics.is_empty());
}

#[test]
fn invalid_exhaust_emits_severe_before_single_return_fallback() {
    let system = finite_flow_system();
    let evaluation =
        plan(Some(NodeId(4)), vec![NodeId(5)], vec![NodeId(8)], false).evaluate(&system);
    assert_eq!(
        evaluation.outcome,
        Ok(PurchasedAirInitTopologyOutcome {
            recirculation_node: Some(NodeId(8)),
            recirculation_source: PurchasedAirRecirculationSource::SingleZoneReturn,
            rejected_exhaust_node: Some(NodeId(4)),
            reported_first_return_node: None,
        })
    );
    assert_eq!(
        diagnostic_pairs(&evaluation),
        vec![(
            PurchasedAirInitTopologyDiagnosticSeverity::Severe,
            PurchasedAirInitTopologyDiagnosticKind::ExhaustNodeNotZoneExhaust,
        )]
    );
}

#[test]
fn multiple_returns_warn_but_preserve_the_source_unassigned_quirk() {
    let system = finite_flow_system();
    let evaluation = plan(None, vec![], vec![NodeId(8), NodeId(3)], false).evaluate(&system);
    assert_eq!(
        evaluation.outcome,
        Ok(PurchasedAirInitTopologyOutcome {
            recirculation_node: None,
            recirculation_source: PurchasedAirRecirculationSource::MultipleZoneReturnsUnassigned,
            rejected_exhaust_node: None,
            reported_first_return_node: Some(NodeId(8)),
        })
    );
    assert_eq!(
        diagnostic_pairs(&evaluation),
        vec![(
            PurchasedAirInitTopologyDiagnosticSeverity::Warning,
            PurchasedAirInitTopologyDiagnosticKind::MultipleReturnNodesUnassigned,
        )]
    );
}

#[test]
fn invalid_exhaust_multiple_returns_and_economizer_keep_source_order() {
    let mut system = finite_flow_system();
    system.outdoor_air_economizer_type = OutdoorAirEconomizerType::DifferentialDryBulb;
    system.cooling_limit = IdealLoadsLimit::NoLimit;
    let evaluation = plan(
        Some(NodeId(4)),
        vec![NodeId(5)],
        vec![NodeId(8), NodeId(3)],
        true,
    )
    .evaluate(&system);
    assert_eq!(
        diagnostic_pairs(&evaluation),
        vec![
            (
                PurchasedAirInitTopologyDiagnosticSeverity::Severe,
                PurchasedAirInitTopologyDiagnosticKind::ExhaustNodeNotZoneExhaust,
            ),
            (
                PurchasedAirInitTopologyDiagnosticSeverity::Warning,
                PurchasedAirInitTopologyDiagnosticKind::MultipleReturnNodesUnassigned,
            ),
            (
                PurchasedAirInitTopologyDiagnosticSeverity::Severe,
                PurchasedAirInitTopologyDiagnosticKind::EconomizerWithoutCoolingFlowLimit,
            ),
        ]
    );
    assert!(evaluation.economizer_flow_limit_warning);
}

#[test]
fn supply_fatal_precedes_exhaust_return_and_economizer_work() {
    let mut system = finite_flow_system();
    system.outdoor_air_economizer_type = OutdoorAirEconomizerType::DifferentialDryBulb;
    let topology = PurchasedAirInitTopologyPlan::from_resolved_nodes(
        SYSTEM,
        ZoneId(0),
        ZoneEquipmentListId(0),
        NodeId(1),
        vec![NodeId(2)],
        Some(NodeId(4)),
        vec![NodeId(5)],
        vec![NodeId(8), NodeId(3)],
        true,
    );
    let evaluation = topology.evaluate(&system);
    assert_eq!(
        evaluation.outcome,
        Err(PurchasedAirInitTopologyError::SupplyNodeNotZoneInlet {
            system: SYSTEM,
            controlled_zone: ZoneId(0),
            supply_node: NodeId(1),
        })
    );
    assert_eq!(
        diagnostic_pairs(&evaluation),
        vec![(
            PurchasedAirInitTopologyDiagnosticSeverity::Fatal,
            PurchasedAirInitTopologyDiagnosticKind::SupplyNodeNotZoneInlet,
        )]
    );
    assert!(!evaluation.economizer_flow_limit_warning);
}

#[test]
fn zero_returns_preserve_invalid_exhaust_then_fatal_order() {
    let system = finite_flow_system();
    let evaluation = plan(Some(NodeId(4)), vec![NodeId(5)], vec![], false).evaluate(&system);
    assert_eq!(
        evaluation.outcome,
        Err(PurchasedAirInitTopologyError::NoRecirculationNode {
            system: SYSTEM,
            controlled_zone: ZoneId(0),
            rejected_exhaust_node: Some(NodeId(4)),
        })
    );
    assert_eq!(
        diagnostic_pairs(&evaluation),
        vec![
            (
                PurchasedAirInitTopologyDiagnosticSeverity::Severe,
                PurchasedAirInitTopologyDiagnosticKind::ExhaustNodeNotZoneExhaust,
            ),
            (
                PurchasedAirInitTopologyDiagnosticSeverity::Fatal,
                PurchasedAirInitTopologyDiagnosticKind::NoRecirculationNode,
            ),
        ]
    );
}

#[test]
fn blank_exhaust_and_zero_returns_fatal_without_a_rejected_node() {
    let system = finite_flow_system();
    let evaluation = plan(None, vec![], vec![], false).evaluate(&system);
    assert_eq!(
        evaluation.outcome,
        Err(PurchasedAirInitTopologyError::NoRecirculationNode {
            system: SYSTEM,
            controlled_zone: ZoneId(0),
            rejected_exhaust_node: None,
        })
    );
    assert_eq!(
        diagnostic_pairs(&evaluation),
        vec![(
            PurchasedAirInitTopologyDiagnosticSeverity::Fatal,
            PurchasedAirInitTopologyDiagnosticKind::NoRecirculationNode,
        )]
    );
}

#[test]
fn rust_node_zero_remains_a_valid_single_return_identity() {
    let system = finite_flow_system();
    let evaluation = plan(None, vec![], vec![NodeId(0)], false).evaluate(&system);
    assert_eq!(
        evaluation.outcome,
        Ok(PurchasedAirInitTopologyOutcome {
            recirculation_node: Some(NodeId(0)),
            recirculation_source: PurchasedAirRecirculationSource::SingleZoneReturn,
            rejected_exhaust_node: None,
            reported_first_return_node: None,
        })
    );
    assert!(evaluation.diagnostics.is_empty());
}

#[test]
fn economizer_advisory_requires_resolved_oa_and_a_missing_flow_limit() {
    for (outdoor_air, economizer, limit, expected) in [
        (
            false,
            OutdoorAirEconomizerType::DifferentialDryBulb,
            IdealLoadsLimit::NoLimit,
            false,
        ),
        (
            true,
            OutdoorAirEconomizerType::NoEconomizer,
            IdealLoadsLimit::NoLimit,
            false,
        ),
        (
            true,
            OutdoorAirEconomizerType::DifferentialDryBulb,
            IdealLoadsLimit::NoLimit,
            true,
        ),
        (
            true,
            OutdoorAirEconomizerType::DifferentialEnthalpy,
            IdealLoadsLimit::LimitCapacity,
            true,
        ),
        (
            true,
            OutdoorAirEconomizerType::DifferentialDryBulb,
            IdealLoadsLimit::LimitFlowRate,
            false,
        ),
        (
            true,
            OutdoorAirEconomizerType::DifferentialDryBulb,
            IdealLoadsLimit::LimitFlowRateAndCapacity,
            false,
        ),
    ] {
        let mut system = finite_flow_system();
        system.outdoor_air_economizer_type = economizer;
        system.cooling_limit = limit;
        let evaluation = plan(None, vec![], vec![NodeId(8)], outdoor_air).evaluate(&system);
        assert_eq!(evaluation.economizer_flow_limit_warning, expected);
    }
}

#[test]
fn model_plan_preserves_node_list_order_and_resolved_oa_state() {
    let mut model = TypedModel::default();
    let mut system = finite_flow_system();
    system.zone_supply_air_node_name = NormalizedName::new("SUPPLY");
    system.zone_exhaust_air_node_name = Some(NormalizedName::new("EXHAUST"));
    system.design_specification_outdoor_air_object_name = Some(NormalizedName::new("OA SPEC"));
    model.ideal_loads_air_systems.push(system.clone());
    for (name, id) in [("SUPPLY", NodeId(1)), ("EXHAUST", NodeId(4))] {
        model.node_names.insert(name, id);
    }
    add_node_list(&mut model, 0, "INLETS", vec![NodeId(7), NodeId(1)]);
    add_node_list(&mut model, 1, "EXHAUSTS", vec![NodeId(9), NodeId(4)]);
    add_node_list(&mut model, 2, "RETURNS", vec![NodeId(8), NodeId(3)]);
    model
        .zone_equipment_connections
        .push(ZoneEquipmentConnection {
            id: ZoneEquipmentConnectionId(0),
            zone: ZoneId(0),
            equipment_list: ZoneEquipmentListId(0),
            zone_air_inlet_node_or_nodelist_name: Some(NormalizedName::new("INLETS")),
            zone_air_exhaust_node_or_nodelist_name: Some(NormalizedName::new("EXHAUSTS")),
            zone_air_node_name: NormalizedName::new("ZONE AIR"),
            zone_return_air_node_or_nodelist_name: Some(NormalizedName::new("RETURNS")),
            zone_return_air_node_1_flow_rate_fraction_schedule: None,
            zone_return_air_node_1_flow_rate_basis_node_or_nodelist_name: None,
        });
    model
        .design_specification_outdoor_air_names
        .insert("OA SPEC", DesignSpecificationOutdoorAirId(0));

    let topology = PurchasedAirInitTopologyPlan::from_model(&model, SYSTEM, ZoneId(0))
        .expect("resolved typed topology");
    assert!(topology.outdoor_air_resolved());
    assert_eq!(
        topology.resolve(),
        Ok(PurchasedAirInitTopologyOutcome {
            recirculation_node: Some(NodeId(4)),
            recirculation_source: PurchasedAirRecirculationSource::ConfiguredZoneExhaust,
            rejected_exhaust_node: None,
            reported_first_return_node: None,
        })
    );
}

fn plan(
    configured_exhaust_node: Option<NodeId>,
    zone_exhaust_nodes: Vec<NodeId>,
    zone_return_nodes: Vec<NodeId>,
    outdoor_air_resolved: bool,
) -> PurchasedAirInitTopologyPlan {
    PurchasedAirInitTopologyPlan::from_resolved_nodes(
        SYSTEM,
        ZoneId(0),
        ZoneEquipmentListId(0),
        NodeId(1),
        vec![NodeId(1)],
        configured_exhaust_node,
        zone_exhaust_nodes,
        zone_return_nodes,
        outdoor_air_resolved,
    )
}

fn diagnostic_pairs(
    evaluation: &PurchasedAirInitTopologyEvaluation,
) -> Vec<(
    PurchasedAirInitTopologyDiagnosticSeverity,
    PurchasedAirInitTopologyDiagnosticKind,
)> {
    evaluation
        .diagnostics
        .iter()
        .map(|diagnostic| (diagnostic.severity, diagnostic.kind))
        .collect()
}

fn add_node_list(model: &mut TypedModel, id: u32, name: &str, nodes: Vec<NodeId>) {
    let id = ep_model::NodeListId(id);
    model.node_list_names.insert(name, id);
    model.node_lists.push(ep_model::NodeList {
        id,
        name: NormalizedName::new(name),
        nodes,
    });
}
