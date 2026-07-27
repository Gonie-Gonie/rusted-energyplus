use ep_model::{
    AutosizeOrNumber, IdealLoadsAirSystem, IdealLoadsAirSystemId, NodeId, ZoneEquipmentListId,
    ZoneId,
};

use super::{
    lifecycle_tests::{context, finite_flow_system},
    *,
};

const SYSTEM_SEVEN: IdealLoadsAirSystemId = IdealLoadsAirSystemId(7);
const SYSTEM_TWO: IdealLoadsAirSystemId = IdealLoadsAirSystemId(2);
const SYSTEM_NINE: IdealLoadsAirSystemId = IdealLoadsAirSystemId(9);
const LIST_SEVEN: ZoneEquipmentListId = ZoneEquipmentListId(70);
const LIST_TWO: ZoneEquipmentListId = ZoneEquipmentListId(20);
const LIST_NINE: ZoneEquipmentListId = ZoneEquipmentListId(90);

#[test]
fn manager_sweep_preserves_declaration_order_and_initializes_only_selected_unit() {
    let plan = manager_plan(&[
        (SYSTEM_SEVEN, Some(LIST_SEVEN)),
        (SYSTEM_TWO, Some(LIST_TWO)),
        (SYSTEM_NINE, Some(LIST_NINE)),
    ]);
    let system = finite_flow_system_for(SYSTEM_NINE);
    let mut state = PurchasedAirRuntimeState::default();

    let snapshot = init_purchased_air_runtime(
        &mut state,
        &plan,
        &topology_for(SYSTEM_NINE, LIST_NINE),
        &system,
        context(true),
    )
    .expect("declaration-order manager sweep");

    assert!(snapshot.transition.module_initialized);
    assert!(snapshot.transition.equipment_list_checked);
    assert_eq!(
        state.declared_system_order,
        vec![SYSTEM_SEVEN, SYSTEM_TWO, SYSTEM_NINE]
    );
    assert_eq!(
        state.equipment_list_scan_order,
        vec![SYSTEM_SEVEN, SYSTEM_TWO, SYSTEM_NINE]
    );
    assert_eq!(state.equipment_list_scanned_unit_count, 3);
    assert_eq!(state.equipment_list_check_count, 1);
    assert_eq!(
        state.units[&SYSTEM_SEVEN].equipment_list_scan_ordinal,
        Some(1)
    );
    assert_eq!(
        state.units[&SYSTEM_TWO].equipment_list_scan_ordinal,
        Some(2)
    );
    assert_eq!(
        state.units[&SYSTEM_NINE].equipment_list_scan_ordinal,
        Some(3)
    );
    assert_eq!(
        state.units[&SYSTEM_SEVEN].first_matching_equipment_list,
        Some(LIST_SEVEN)
    );
    assert_eq!(
        state.units[&SYSTEM_TWO].first_matching_equipment_list,
        Some(LIST_TWO)
    );
    assert_eq!(
        state.units[&SYSTEM_NINE].first_matching_equipment_list,
        Some(LIST_NINE)
    );
    assert_eq!(state.units[&SYSTEM_SEVEN].init_call_count, 0);
    assert_eq!(state.units[&SYSTEM_TWO].init_call_count, 0);
    assert_eq!(state.units[&SYSTEM_NINE].init_call_count, 1);
    assert!(!state.units[&SYSTEM_SEVEN].one_time_latched);
    assert!(!state.units[&SYSTEM_TWO].one_time_latched);
    assert!(state.units[&SYSTEM_NINE].one_time_latched);
}

#[test]
fn deferred_sweep_runs_once_and_is_not_repeated_across_selected_units() {
    let plan = manager_plan(&[
        (SYSTEM_SEVEN, Some(LIST_SEVEN)),
        (SYSTEM_TWO, Some(LIST_TWO)),
    ]);
    let system_seven = finite_flow_system_for(SYSTEM_SEVEN);
    let system_two = finite_flow_system_for(SYSTEM_TWO);
    let mut state = PurchasedAirRuntimeState::default();
    let mut deferred = context(true);
    deferred.zone_equipment_inputs_filled = false;

    let first = init_purchased_air_runtime(
        &mut state,
        &plan,
        &topology_for(SYSTEM_SEVEN, LIST_SEVEN),
        &system_seven,
        deferred,
    )
    .expect("manager sweep defers until Zone equipment input is ready");
    assert!(first.transition.module_initialized);
    assert!(!first.transition.equipment_list_checked);
    assert!(first.transition.one_time_initialized);
    assert!(!state.equipment_list_checked);
    assert!(state.equipment_list_scan_order.is_empty());
    assert_eq!(state.equipment_list_scanned_unit_count, 0);

    let ready = init_purchased_air_runtime(
        &mut state,
        &plan,
        &topology_for(SYSTEM_TWO, LIST_TWO),
        &system_two,
        context(true),
    )
    .expect("first ready call scans every declared unit");
    assert!(ready.transition.equipment_list_checked);
    assert_eq!(
        state.equipment_list_scan_order,
        vec![SYSTEM_SEVEN, SYSTEM_TWO]
    );
    assert_eq!(state.equipment_list_check_count, 1);
    assert_eq!(state.equipment_list_scanned_unit_count, 2);

    let replay = init_purchased_air_runtime(
        &mut state,
        &plan,
        &topology_for(SYSTEM_SEVEN, LIST_SEVEN),
        &system_seven,
        context(true),
    )
    .expect("later selected unit reuses the completed manager sweep");
    assert!(!replay.transition.equipment_list_checked);
    assert_eq!(
        state.equipment_list_scan_order,
        vec![SYSTEM_SEVEN, SYSTEM_TWO]
    );
    assert_eq!(state.equipment_list_check_count, 1);
    assert_eq!(state.equipment_list_scanned_unit_count, 2);
    assert_eq!(state.units[&SYSTEM_SEVEN].init_call_count, 2);
    assert_eq!(state.units[&SYSTEM_TWO].init_call_count, 1);
}

#[test]
fn missing_equipment_memberships_emit_ordered_diagnostics_without_fail_fast() {
    let plan = manager_plan(&[
        (SYSTEM_SEVEN, None),
        (SYSTEM_TWO, Some(LIST_TWO)),
        (SYSTEM_NINE, None),
    ]);
    let system = finite_flow_system_for(SYSTEM_NINE);
    let mut state = PurchasedAirRuntimeState::default();

    let snapshot = init_purchased_air_runtime(
        &mut state,
        &plan,
        &topology_for(SYSTEM_NINE, LIST_NINE),
        &system,
        context(true),
    )
    .expect("missing memberships are retained diagnostics, not fail-fast errors");

    assert!(snapshot.transition.equipment_list_checked);
    assert!(snapshot.transition.one_time_initialized);
    assert_eq!(state.equipment_list_scanned_unit_count, 3);
    assert_eq!(state.equipment_list_missing_unit_count, 2);
    assert_eq!(
        state.equipment_list_diagnostics,
        vec![
            PurchasedAirInitDiagnostic {
                system: SYSTEM_SEVEN,
                scan_ordinal: 1,
                kind: PurchasedAirInitDiagnosticKind::EquipmentListMembershipMissing,
            },
            PurchasedAirInitDiagnostic {
                system: SYSTEM_NINE,
                scan_ordinal: 3,
                kind: PurchasedAirInitDiagnosticKind::EquipmentListMembershipMissing,
            },
        ]
    );
    assert_eq!(
        state.units[&SYSTEM_SEVEN].equipment_list_membership_found,
        Some(false)
    );
    assert_eq!(
        state.units[&SYSTEM_TWO].equipment_list_membership_found,
        Some(true)
    );
    assert_eq!(
        state.units[&SYSTEM_NINE].equipment_list_membership_found,
        Some(false)
    );
    let summary = purchased_air_init_lifecycle_summary(&state, SYSTEM_NINE)
        .expect("selected missing-membership lifecycle");
    assert_eq!(summary.equipment_list_scan_ordinal, Some(3));
    assert_eq!(summary.first_matching_equipment_list, None);
    assert_eq!(summary.equipment_list_membership_found, Some(false));
    assert_eq!(summary.one_time_initialization_count, 1);
}

#[test]
fn changed_manager_plan_is_rejected_before_selected_unit_mutation() {
    let plan = manager_plan(&[
        (SYSTEM_SEVEN, Some(LIST_SEVEN)),
        (SYSTEM_TWO, Some(LIST_TWO)),
    ]);
    let system = finite_flow_system_for(SYSTEM_SEVEN);
    let bound = topology_for(SYSTEM_SEVEN, LIST_SEVEN);
    let mut state = PurchasedAirRuntimeState::default();
    init_purchased_air_runtime(&mut state, &plan, &bound, &system, context(true))
        .expect("seed immutable manager plan");

    let changed_order = manager_plan(&[
        (SYSTEM_TWO, Some(LIST_TWO)),
        (SYSTEM_SEVEN, Some(LIST_SEVEN)),
    ]);
    let before_order_error = state.clone();
    assert_eq!(
        init_purchased_air_runtime(&mut state, &changed_order, &bound, &system, context(true),),
        Err(PurchasedAirInitError::DeclaredSystemOrderChanged {
            expected: vec![SYSTEM_SEVEN, SYSTEM_TWO],
            actual: vec![SYSTEM_TWO, SYSTEM_SEVEN],
        })
    );
    assert_eq!(state, before_order_error);

    let changed_membership = manager_plan(&[
        (SYSTEM_SEVEN, Some(ZoneEquipmentListId(71))),
        (SYSTEM_TWO, Some(LIST_TWO)),
    ]);
    let before_membership_error = state.clone();
    assert_eq!(
        init_purchased_air_runtime(
            &mut state,
            &changed_membership,
            &bound,
            &system,
            context(true),
        ),
        Err(PurchasedAirInitError::ManagerPlanMembershipChanged {
            system: SYSTEM_SEVEN,
            expected: Some(LIST_SEVEN),
            actual: Some(ZoneEquipmentListId(71)),
        })
    );
    assert_eq!(state, before_membership_error);
}

#[test]
fn selected_system_missing_from_plan_is_rejected_before_allocation() {
    let plan = manager_plan(&[(SYSTEM_SEVEN, Some(LIST_SEVEN))]);
    let system = finite_flow_system_for(SYSTEM_NINE);
    let mut state = PurchasedAirRuntimeState::default();
    assert_eq!(
        init_purchased_air_runtime(
            &mut state,
            &plan,
            &topology_for(SYSTEM_NINE, LIST_NINE),
            &system,
            context(true),
        ),
        Err(
            PurchasedAirInitError::SelectedSystemMissingFromManagerPlan {
                system: SYSTEM_NINE,
            }
        )
    );
    assert_eq!(state, PurchasedAirRuntimeState::default());
}

#[test]
fn completed_manager_scan_survives_autosize_retry_and_topology_error() {
    let plan = manager_plan(&[
        (SYSTEM_SEVEN, Some(LIST_SEVEN)),
        (SYSTEM_TWO, Some(LIST_TWO)),
    ]);
    let mut system = finite_flow_system_for(SYSTEM_SEVEN);
    system.maximum_heating_air_flow_rate_m3_per_s = Some(AutosizeOrNumber::Autosize);
    let bound = topology_for(SYSTEM_SEVEN, LIST_SEVEN);
    let mut state = PurchasedAirRuntimeState::default();

    let error = init_purchased_air_runtime(&mut state, &plan, &bound, &system, context(true))
        .expect_err("autosize remains beyond the bounded manager-sweep slice");
    assert_eq!(
        error,
        PurchasedAirInitError::AutosizingNotImplemented {
            system: SYSTEM_SEVEN,
            field: "maximum_heating_air_flow_rate_m3_per_s",
        }
    );
    assert!(state.equipment_list_checked);
    assert_eq!(
        state.equipment_list_scan_order,
        vec![SYSTEM_SEVEN, SYSTEM_TWO]
    );
    assert_eq!(state.equipment_list_check_count, 1);
    assert_eq!(state.equipment_list_scanned_unit_count, 2);
    assert!(state.units[&SYSTEM_SEVEN].sizing_needed);
    assert_eq!(
        state.units[&SYSTEM_SEVEN].environment_initialization_count,
        0
    );

    let hard_sized = finite_flow_system_for(SYSTEM_SEVEN);
    let retry = init_purchased_air_runtime(&mut state, &plan, &bound, &hard_sized, context(true))
        .expect("hard-size retry reuses the completed manager scan");
    assert!(!retry.transition.equipment_list_checked);
    assert!(retry.transition.sizing_checked);
    assert_eq!(state.equipment_list_check_count, 1);

    let changed = topology_for_supply(SYSTEM_SEVEN, LIST_SEVEN, NodeId(99));
    assert_eq!(
        init_purchased_air_runtime(&mut state, &plan, &changed, &hard_sized, context(true),),
        Err(PurchasedAirInitError::LatchedTopologyChanged {
            system: SYSTEM_SEVEN,
        })
    );
    assert_eq!(
        state.equipment_list_scan_order,
        vec![SYSTEM_SEVEN, SYSTEM_TWO]
    );
    assert_eq!(state.equipment_list_check_count, 1);
    assert_eq!(state.equipment_list_scanned_unit_count, 2);
}

fn manager_plan(
    rows: &[(IdealLoadsAirSystemId, Option<ZoneEquipmentListId>)],
) -> PurchasedAirInitManagerPlan {
    PurchasedAirInitManagerPlan::try_from_rows(
        rows.iter()
            .map(
                |&(system, first_matching_equipment_list)| PurchasedAirInitManagerPlanRow {
                    system,
                    first_matching_equipment_list,
                    return_plenum_active: false,
                },
            )
            .collect(),
    )
    .expect("valid test manager plan")
}

fn topology_for(
    system: IdealLoadsAirSystemId,
    equipment_list: ZoneEquipmentListId,
) -> PurchasedAirInitTopologyPlan {
    topology_for_supply(system, equipment_list, NodeId(1))
}

fn topology_for_supply(
    system: IdealLoadsAirSystemId,
    equipment_list: ZoneEquipmentListId,
    supply_node: NodeId,
) -> PurchasedAirInitTopologyPlan {
    PurchasedAirInitTopologyPlan::from_resolved_nodes(
        system,
        ZoneId(0),
        equipment_list,
        supply_node,
        vec![supply_node],
        None,
        Vec::new(),
        vec![NodeId(2)],
        false,
    )
}

fn finite_flow_system_for(system: IdealLoadsAirSystemId) -> IdealLoadsAirSystem {
    let mut configured = finite_flow_system();
    configured.id = system;
    configured
}
