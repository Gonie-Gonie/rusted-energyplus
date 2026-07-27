use ep_model::{
    IdealLoadsAirSystemId, IdealLoadsLimit, NodeId, OutdoorAirEconomizerType, ZoneEquipmentListId,
    ZoneId,
};

use super::{
    lifecycle_tests::{SYSTEM, context, finite_flow_system, single_manager_plan},
    *,
};

const OTHER_SYSTEM: IdealLoadsAirSystemId = IdealLoadsAirSystemId(7);

#[test]
fn selected_latch_precedes_supply_fatal_and_is_not_replayed() {
    let system = finite_flow_system();
    let manager = single_manager_plan();
    let topology = resolved_topology(
        NodeId(1),
        vec![NodeId(2)],
        None,
        vec![],
        vec![NodeId(8)],
        false,
    );
    let failure = PurchasedAirInitTopologyError::SupplyNodeNotZoneInlet {
        system: SYSTEM,
        controlled_zone: ZoneId(0),
        supply_node: NodeId(1),
    };
    let mut state = PurchasedAirRuntimeState::default();

    assert_eq!(
        init_purchased_air_runtime(&mut state, &manager, &topology, &system, context(true)),
        Err(PurchasedAirInitError::Topology(failure))
    );
    assert_eq!(state.module_initialization_count, 1);
    assert_eq!(state.equipment_list_check_count, 1);
    assert_eq!(state.equipment_list_scan_order, vec![SYSTEM]);
    let first = &state.units[&SYSTEM];
    assert!(first.one_time_latched);
    assert!(!first.topology_completed);
    assert_eq!(first.one_time_initialization_count, 1);
    assert_eq!(first.topology_completion_count, 0);
    assert_eq!(first.init_call_count, 1);
    assert_eq!(first.topology_failure, Some(failure));
    assert_eq!(
        first.topology_diagnostics,
        vec![topology_diagnostic(
            1,
            PurchasedAirInitTopologyDiagnosticSeverity::Fatal,
            PurchasedAirInitTopologyDiagnosticKind::SupplyNodeNotZoneInlet,
        )]
    );
    assert_eq!(first.sizing_check_count, 0);
    assert_eq!(first.environment_initialization_count, 0);

    let mut expected_after_retry = state.clone();
    expected_after_retry
        .units
        .get_mut(&SYSTEM)
        .expect("selected unit")
        .init_call_count += 1;
    assert_eq!(
        init_purchased_air_runtime(&mut state, &manager, &topology, &system, context(true)),
        Err(PurchasedAirInitError::Topology(failure))
    );
    assert_eq!(state, expected_after_retry);
}

#[test]
fn invalid_exhaust_diagnostic_precedes_single_return_fallback() {
    let system = finite_flow_system();
    let manager = single_manager_plan();
    let topology = resolved_topology(
        NodeId(1),
        vec![NodeId(1)],
        Some(NodeId(4)),
        vec![NodeId(5)],
        vec![NodeId(8)],
        false,
    );
    let mut state = PurchasedAirRuntimeState::default();

    let snapshot =
        init_purchased_air_runtime(&mut state, &manager, &topology, &system, context(true))
            .expect("invalid exhaust must fall back to the single return node");

    assert_eq!(
        snapshot.transition,
        PurchasedAirInitTransition {
            module_initialized: true,
            equipment_list_checked: true,
            equipment_list_units_scanned: 1,
            one_time_initialized: true,
            topology_completed: true,
            topology_diagnostics_emitted: 1,
            sizing_checked: true,
            environment_initialized: true,
            ..PurchasedAirInitTransition::default()
        }
    );
    assert_eq!(snapshot.recirculation_node, Some(NodeId(8)));
    assert_eq!(
        snapshot.recirculation_source,
        Some(PurchasedAirRecirculationSource::SingleZoneReturn)
    );
    assert_eq!(snapshot.rejected_exhaust_node, Some(NodeId(4)));
    assert!(snapshot.flags.topology_ready);

    let unit = &state.units[&SYSTEM];
    assert_eq!(unit.one_time_initialization_count, 1);
    assert_eq!(unit.topology_completion_count, 1);
    assert_eq!(
        unit.topology_diagnostics,
        vec![topology_diagnostic(
            1,
            PurchasedAirInitTopologyDiagnosticSeverity::Severe,
            PurchasedAirInitTopologyDiagnosticKind::ExhaustNodeNotZoneExhaust,
        )]
    );
    assert_eq!(unit.sizing_check_count, 1);
    assert_eq!(unit.environment_initialization_count, 1);
}

#[test]
fn multiple_returns_warn_and_leave_recirculation_unassigned() {
    let system = finite_flow_system();
    let manager = single_manager_plan();
    let topology = resolved_topology(
        NodeId(1),
        vec![NodeId(1)],
        None,
        vec![],
        vec![NodeId(8), NodeId(3)],
        false,
    );
    let mut state = PurchasedAirRuntimeState::default();

    let snapshot =
        init_purchased_air_runtime(&mut state, &manager, &topology, &system, context(true))
            .expect("multiple returns are a warning-only source branch");

    assert!(snapshot.transition.one_time_initialized);
    assert!(snapshot.transition.topology_completed);
    assert_eq!(snapshot.transition.topology_diagnostics_emitted, 1);
    assert_eq!(snapshot.recirculation_node, None);
    assert_eq!(
        snapshot.recirculation_source,
        Some(PurchasedAirRecirculationSource::MultipleZoneReturnsUnassigned)
    );
    assert_eq!(snapshot.reported_first_return_node, Some(NodeId(8)));
    assert!(!snapshot.flags.topology_ready);

    let unit = &state.units[&SYSTEM];
    assert!(unit.topology_completed);
    assert_eq!(unit.topology_completion_count, 1);
    assert_eq!(unit.recirculation_node, None);
    assert_eq!(unit.reported_first_return_node, Some(NodeId(8)));
    assert_eq!(
        unit.topology_diagnostics,
        vec![topology_diagnostic(
            1,
            PurchasedAirInitTopologyDiagnosticSeverity::Warning,
            PurchasedAirInitTopologyDiagnosticKind::MultipleReturnNodesUnassigned,
        )]
    );
}

#[test]
fn zero_returns_fatal_after_latch_and_retains_diagnostic_prefix() {
    let system = finite_flow_system();
    let manager = single_manager_plan();
    let topology = resolved_topology(
        NodeId(1),
        vec![NodeId(1)],
        Some(NodeId(4)),
        vec![NodeId(5)],
        vec![],
        false,
    );
    let failure = PurchasedAirInitTopologyError::NoRecirculationNode {
        system: SYSTEM,
        controlled_zone: ZoneId(0),
        rejected_exhaust_node: Some(NodeId(4)),
    };
    let mut state = PurchasedAirRuntimeState::default();

    assert_eq!(
        init_purchased_air_runtime(&mut state, &manager, &topology, &system, context(true)),
        Err(PurchasedAirInitError::Topology(failure))
    );

    let unit = &state.units[&SYSTEM];
    assert!(unit.one_time_latched);
    assert!(!unit.topology_completed);
    assert_eq!(unit.one_time_initialization_count, 1);
    assert_eq!(unit.topology_completion_count, 0);
    assert_eq!(unit.topology_failure, Some(failure));
    assert_eq!(unit.rejected_exhaust_node, Some(NodeId(4)));
    assert_eq!(unit.recirculation_node, None);
    assert_eq!(
        unit.topology_diagnostics,
        vec![
            topology_diagnostic(
                1,
                PurchasedAirInitTopologyDiagnosticSeverity::Severe,
                PurchasedAirInitTopologyDiagnosticKind::ExhaustNodeNotZoneExhaust,
            ),
            topology_diagnostic(
                2,
                PurchasedAirInitTopologyDiagnosticSeverity::Fatal,
                PurchasedAirInitTopologyDiagnosticKind::NoRecirculationNode,
            ),
        ]
    );
    assert_eq!(unit.sizing_check_count, 0);
    assert_eq!(unit.environment_initialization_count, 0);
}

#[test]
fn economizer_advisory_is_one_time_and_follows_topology_diagnostics() {
    let mut system = finite_flow_system();
    system.cooling_limit = IdealLoadsLimit::NoLimit;
    system.outdoor_air_economizer_type = OutdoorAirEconomizerType::DifferentialDryBulb;
    let manager = single_manager_plan();
    let topology = resolved_topology(
        NodeId(1),
        vec![NodeId(1)],
        Some(NodeId(4)),
        vec![NodeId(5)],
        vec![NodeId(8), NodeId(3)],
        true,
    );
    let mut state = PurchasedAirRuntimeState::default();

    let first = init_purchased_air_runtime(&mut state, &manager, &topology, &system, context(true))
        .expect("economizer advisory is nonfatal");
    assert!(first.transition.economizer_flow_limit_warning);
    assert_eq!(first.transition.topology_diagnostics_emitted, 3);
    assert_eq!(
        state.units[&SYSTEM].topology_diagnostics,
        vec![
            topology_diagnostic(
                1,
                PurchasedAirInitTopologyDiagnosticSeverity::Severe,
                PurchasedAirInitTopologyDiagnosticKind::ExhaustNodeNotZoneExhaust,
            ),
            topology_diagnostic(
                2,
                PurchasedAirInitTopologyDiagnosticSeverity::Warning,
                PurchasedAirInitTopologyDiagnosticKind::MultipleReturnNodesUnassigned,
            ),
            topology_diagnostic(
                3,
                PurchasedAirInitTopologyDiagnosticSeverity::Severe,
                PurchasedAirInitTopologyDiagnosticKind::EconomizerWithoutCoolingFlowLimit,
            ),
        ]
    );
    assert_eq!(state.units[&SYSTEM].economizer_flow_limit_warning_count, 1);

    let replay =
        init_purchased_air_runtime(&mut state, &manager, &topology, &system, context(true))
            .expect("steady replay must skip one-time topology diagnostics");
    assert_eq!(replay.transition, PurchasedAirInitTransition::default());
    let unit = &state.units[&SYSTEM];
    assert_eq!(unit.init_call_count, 2);
    assert_eq!(unit.one_time_initialization_count, 1);
    assert_eq!(unit.topology_completion_count, 1);
    assert_eq!(unit.topology_diagnostics.len(), 3);
    assert_eq!(unit.economizer_flow_limit_warning_count, 1);
}

#[test]
fn manager_sweep_survives_selected_topology_fatal() {
    let manager = PurchasedAirInitManagerPlan::try_from_rows(vec![
        PurchasedAirInitManagerPlanRow {
            system: OTHER_SYSTEM,
            first_matching_equipment_list: None,
            return_plenum_active: false,
        },
        PurchasedAirInitManagerPlanRow {
            system: SYSTEM,
            first_matching_equipment_list: Some(ZoneEquipmentListId(0)),
            return_plenum_active: false,
        },
    ])
    .expect("valid two-unit manager plan");
    let system = finite_flow_system();
    let topology = resolved_topology(
        NodeId(1),
        vec![NodeId(2)],
        None,
        vec![],
        vec![NodeId(8)],
        false,
    );
    let failure = PurchasedAirInitTopologyError::SupplyNodeNotZoneInlet {
        system: SYSTEM,
        controlled_zone: ZoneId(0),
        supply_node: NodeId(1),
    };
    let mut state = PurchasedAirRuntimeState::default();

    assert_eq!(
        init_purchased_air_runtime(&mut state, &manager, &topology, &system, context(true)),
        Err(PurchasedAirInitError::Topology(failure))
    );

    assert!(state.equipment_list_checked);
    assert_eq!(state.equipment_list_check_count, 1);
    assert_eq!(state.declared_system_order, vec![OTHER_SYSTEM, SYSTEM]);
    assert_eq!(state.equipment_list_scan_order, vec![OTHER_SYSTEM, SYSTEM]);
    assert_eq!(state.equipment_list_scanned_unit_count, 2);
    assert_eq!(state.equipment_list_missing_unit_count, 1);
    assert_eq!(
        state.equipment_list_diagnostics,
        vec![PurchasedAirInitDiagnostic {
            system: OTHER_SYSTEM,
            scan_ordinal: 1,
            kind: PurchasedAirInitDiagnosticKind::EquipmentListMembershipMissing,
        }]
    );
    assert_eq!(
        state.units[&OTHER_SYSTEM].equipment_list_scan_ordinal,
        Some(1)
    );
    assert_eq!(state.units[&SYSTEM].equipment_list_scan_ordinal, Some(2));
    assert!(!state.units[&OTHER_SYSTEM].one_time_latched);
    assert!(state.units[&SYSTEM].one_time_latched);
    assert_eq!(state.units[&SYSTEM].topology_failure, Some(failure));
}

#[test]
fn environment_validation_never_commits_half_cache() {
    let mut system = finite_flow_system();
    system.maximum_cooling_air_flow_rate_m3_per_s = None;
    let manager = single_manager_plan();
    let topology = resolved_topology(
        NodeId(1),
        vec![NodeId(1)],
        None,
        vec![],
        vec![NodeId(8)],
        false,
    );
    let mut call = context(true);
    call.system_sizing_calculation = true;
    let mut state = PurchasedAirRuntimeState::default();

    assert_eq!(
        init_purchased_air_runtime(&mut state, &manager, &topology, &system, call),
        Err(PurchasedAirInitError::Sizing(
            crate::ideal_loads::PurchasedAirHardSizeLegacyError::MissingRequiredHardSize {
                system: SYSTEM,
                field: crate::ideal_loads::PurchasedAirHardSizeField::MaximumCoolingAirFlowRate,
            }
        ))
    );
    let unit = &state.units[&SYSTEM];
    assert!(unit.one_time_latched);
    assert!(unit.topology_completed);
    assert_eq!(unit.maximum_heating_air_mass_flow_rate_kg_per_s, 0.0);
    assert_eq!(unit.maximum_cooling_air_mass_flow_rate_kg_per_s, 0.0);
    assert_eq!(unit.standard_air_density_kg_per_m3, None);
    assert_eq!(unit.environment_initialization_count, 0);
    assert_eq!(unit.sizing_attempt_count, 0);
    assert!(unit.sizing_needed);
}

#[allow(clippy::too_many_arguments)]
fn resolved_topology(
    supply_node: NodeId,
    zone_inlet_nodes: Vec<NodeId>,
    configured_exhaust_node: Option<NodeId>,
    zone_exhaust_nodes: Vec<NodeId>,
    zone_return_nodes: Vec<NodeId>,
    outdoor_air_resolved: bool,
) -> PurchasedAirInitTopologyPlan {
    PurchasedAirInitTopologyPlan::from_resolved_nodes(
        SYSTEM,
        ZoneId(0),
        ZoneEquipmentListId(0),
        supply_node,
        zone_inlet_nodes,
        configured_exhaust_node,
        zone_exhaust_nodes,
        zone_return_nodes,
        outdoor_air_resolved,
    )
}

fn topology_diagnostic(
    ordinal: usize,
    severity: PurchasedAirInitTopologyDiagnosticSeverity,
    kind: PurchasedAirInitTopologyDiagnosticKind,
) -> PurchasedAirInitTopologyDiagnostic {
    PurchasedAirInitTopologyDiagnostic {
        system: SYSTEM,
        ordinal,
        severity,
        kind,
    }
}
