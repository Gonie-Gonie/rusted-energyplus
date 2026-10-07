//! CP442 boundary, exhaustive route, forgery, overflow, and bounded-path tests.

mod schema_prefix;

use super::PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardElseBranchEntryRuntimeState as State;
use super::transition::{
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardElseBranchEntryRetainedRoute as Route,
    advance_heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry_state as advance,
    advance_heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry_state_with_validated_route as advance_validated,
    heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry_route_from_committed_predecessor as successor_route,
};
use crate::ideal_loads::calc::{
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowContinueWarningTimestampCallRetainedRoute as PredecessorRoute,
    cp441_all_snapshots_for_successor_tests, cp441_guard_false_fixture_unit_for_successor_tests,
    heating_outdoor_air_maximum_flow_continue_warning_timestamp_call_snapshot_route as predecessor_route,
};
use crate::ideal_loads::{
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowContinueWarningTimestampCallSnapshot as Predecessor,
    PurchasedAirUnitRuntimeState,
    heating_outdoor_air_maximum_flow_continue_warning_timestamp_call_snapshots_match_bit_exact,
};
use ep_model::IdealLoadsAirSystemId;

#[test]
fn cp442_boundary_maps_structural_2375_and_excludes_recurring_call_2376() {
    assert_eq!(
        super::PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_GUARD_ELSE_BRANCH_ENTRY_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2375",
    );
    assert_eq!(
        super::PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_GUARD_ELSE_BRANCH_ENTRY_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2376",
    );
    assert_eq!(
        super::PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_GUARD_ELSE_BRANCH_ENTRY_SOURCE_ORDER,
        &["enter-heating-outdoor-air-maximum-flow-first-warning-guard-else-branch-after-guard-false-fallthrough"],
    );
}

#[test]
fn exhaustive_67_routes_have_exact_three_private_else_entries() {
    let predecessors = cp441_all_snapshots_for_successor_tests();
    assert_eq!(predecessors.len(), 67);
    let mut state = State::new(predecessors[0].system);
    let mut expected = [[0usize; 36]; 2];
    let mut public = 0usize;
    let mut public_entries = 0usize;
    let mut private_entries = 0usize;

    for predecessor in predecessors {
        let predecessor_route = predecessor_route_for(predecessor);
        let route = route_for(predecessor);
        let snapshot = advance_validated(&mut state, predecessor, predecessor_route, route)
            .expect("CP442 snapshot");
        expected[0][route.logical_index] += 1;
        expected[1][route.logical_index] += usize::from(route.entered);
        let is_public = is_public_route(predecessor, route);
        public += usize::from(is_public);
        public_entries += usize::from(is_public && route.entered);
        private_entries += usize::from(!is_public && route.entered);

        assert_eq!(
            route.entered,
            route.predecessor_first_warning_guard_false_fallthrough
        );
        assert_eq!(
            route.entered,
            predecessor.heating_outdoor_air_maximum_flow_first_warning_guard_false_fallthrough
        );
        assert!(!(route.entered && route.predecessor_continue_warning_timestamp_call_site_reached));
        assert_eq!(
            route.entered || route.predecessor_continue_warning_timestamp_call_site_reached,
            route.predecessor_first_warning_guard_evaluated
        );
        assert_eq!(
            snapshot.heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entered,
            route.entered
        );
        assert!(
            heating_outdoor_air_maximum_flow_continue_warning_timestamp_call_snapshots_match_bit_exact(
                super::heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry_predecessor_cp441_snapshot(snapshot),
                predecessor,
            )
        );
        assert_bits(
            snapshot.resulting_supply_humidity_ratio,
            predecessor.resulting_supply_humidity_ratio,
        );
        assert_bits(
            snapshot.resulting_supply_enthalpy_j_per_kg,
            predecessor.resulting_supply_enthalpy_j_per_kg,
        );
        assert_bits(
            snapshot.resulting_supply_temperature_c,
            predecessor.resulting_supply_temperature_c,
        );
        assert!(
            super::heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry_snapshot_is_exact(
                snapshot
            )
        );
    }

    assert_eq!((public, 67 - public), (20, 47));
    assert_eq!((public_entries, private_entries), (0, 3));
    assert_eq!(state.transition_count, 67);
    assert_eq!(state.inactive_transition_count, 64);
    assert_eq!(
        state.heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry_count,
        3
    );
    assert_eq!(state.source_site_execution_count, 3);
    assert_eq!(state.predecessor_route_counts, expected[0]);
    assert_eq!(
        state.heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry_route_counts,
        expected[1]
    );
    assert_eq!(expected[0][1], 9);
    assert_eq!(expected[1][1], 3);
    assert_eq!(state.cp441_supply_humidity_ratio_state_owner_count, 37);
    assert_eq!(state.unchanged_supply_humidity_ratio_preservation_count, 37);
    assert_eq!(state.cp441_supply_enthalpy_state_owner_count, 42);
    assert_eq!(state.unchanged_supply_enthalpy_preservation_count, 42);
    assert_eq!(state.cp441_supply_temperature_state_owner_count, 57);
    assert_eq!(state.unchanged_supply_temperature_preservation_count, 57);
    assert!(super::release::state_counts_are_consistent_for_test(&state));
}

#[test]
fn cp442_new_state_has_exactly_two_zeroed_route_arrays_and_no_counter_owner() {
    let state = State::new(IdealLoadsAirSystemId(0));
    let arrays = [
        state.predecessor_route_counts,
        state.heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry_route_counts,
    ];
    assert_eq!(arrays.len(), 2);
    assert!(arrays.into_iter().flatten().all(|count| count == 0));
    let source = include_str!("state.rs");
    assert_eq!(source.matches("[usize; 36]").count(), 2);
    assert_eq!(source.matches("_state_owner_count: usize").count(), 3);
    assert_eq!(source.matches("_preservation_count: usize").count(), 3);
    assert!(!source.contains("outdoor_air_flow_maximum_heating_output_error_count"));
    assert_eq!(
        include_str!("transition.rs").matches("    pub ").count(),
        12
    );
    for forbidden in ["message", "sink", "format", "sqlite", "callback"] {
        assert!(
            !source.to_ascii_lowercase().contains(forbidden),
            "{forbidden}"
        );
    }
}

#[test]
fn every_cp442_and_supplied_cp441_route_component_forgery_is_transactional() {
    let predecessor = guard_false_predecessor();
    let predecessor_route = predecessor_route_for(predecessor);
    let exact = route_for(predecessor);
    for component in 0..12 {
        let mut forged = exact;
        flip_route_component(&mut forged, component);
        let mut state = State::new(predecessor.system);
        let before = state.clone();
        assert!(
            advance_validated(&mut state, predecessor, predecessor_route, forged).is_none(),
            "CP442 component {component}",
        );
        assert_eq!(state, before, "CP442 component {component}");
    }
    for component in 0..11 {
        let mut forged = predecessor_route;
        flip_predecessor_route_component(&mut forged, component);
        let mut state = State::new(predecessor.system);
        let before = state.clone();
        assert!(
            advance_validated(&mut state, predecessor, forged, exact).is_none(),
            "CP441 component {component}",
        );
        assert_eq!(state, before, "CP441 component {component}");
    }
}

#[test]
fn all_count_route_and_wht_overflows_are_transactional() {
    let entry = guard_false_predecessor();
    let inactive = cp441_all_snapshots_for_successor_tests()
        .into_iter()
        .find(|snapshot| !route_for(*snapshot).entered)
        .expect("inactive predecessor");
    for (predecessor, slot) in [
        (entry, 0usize),
        (entry, 1),
        (entry, 2),
        (entry, 3),
        (entry, 4),
        (inactive, 5),
    ] {
        let predecessor_route = predecessor_route_for(predecessor);
        let route = route_for(predecessor);
        let mut state = State::new(predecessor.system);
        match slot {
            0 => state.transition_count = usize::MAX,
            1 => state.predecessor_route_counts[route.logical_index] = usize::MAX,
            2 => {
                state.heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry_count =
                    usize::MAX
            }
            3 => {
                state.heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry_route_counts
                    [route.logical_index] = usize::MAX
            }
            4 => state.source_site_execution_count = usize::MAX,
            _ => state.inactive_transition_count = usize::MAX,
        }
        let before = state.clone();
        assert!(advance_validated(&mut state, predecessor, predecessor_route, route).is_none());
        assert_eq!(state, before, "overflow {slot}");
    }

    let owned = cp441_all_snapshots_for_successor_tests()
        .into_iter()
        .find(|snapshot| {
            snapshot.resulting_supply_humidity_ratio.is_some()
                && snapshot.resulting_supply_enthalpy_j_per_kg.is_some()
                && snapshot.resulting_supply_temperature_c.is_some()
        })
        .expect("W/H/T owner route");
    for slot in 0..6 {
        let predecessor_route = predecessor_route_for(owned);
        let route = route_for(owned);
        let mut state = State::new(owned.system);
        match slot {
            0 => state.cp441_supply_humidity_ratio_state_owner_count = usize::MAX,
            1 => state.unchanged_supply_humidity_ratio_preservation_count = usize::MAX,
            2 => state.cp441_supply_enthalpy_state_owner_count = usize::MAX,
            3 => state.unchanged_supply_enthalpy_preservation_count = usize::MAX,
            4 => state.cp441_supply_temperature_state_owner_count = usize::MAX,
            _ => state.unchanged_supply_temperature_preservation_count = usize::MAX,
        }
        let before = state.clone();
        assert!(advance_validated(&mut state, owned, predecessor_route, route).is_none());
        assert_eq!(state, before, "W/H/T overflow {slot}");
    }
}

#[test]
fn release_uses_cp441_committed_seal_and_rejects_public_else_entry() {
    let release = include_str!("release.rs");
    let seal = release
        .find("heating_outdoor_air_maximum_flow_continue_warning_timestamp_call_committed_latest_route(")
        .expect("CP441 committed seal");
    let reject = release
        .find("if route.entered")
        .expect("direct entry rejection");
    let clone = release
        .find("let mut next_state = unit")
        .expect("transactional clone");
    assert!(seal < reject && reject < clone);
    assert!(!release.contains("DirectZonePurchasedAirCouplingInput"));
    for forbidden in [
        "ShowRecurringWarningErrorAtEnd",
        "warning_index",
        "OAVolFlowRate",
    ] {
        assert!(!release.contains(forbidden), "{forbidden}");
    }
}

#[test]
fn cp442_subtree_is_twelve_files_and_every_file_is_bounded() {
    let files = [
        include_str!(
            "../heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry.rs"
        ),
        include_str!("release.rs"),
        include_str!("state.rs"),
        include_str!("tests.rs"),
        include_str!("transition.rs"),
        include_str!("release/error.rs"),
        include_str!("release/prefix.rs"),
        include_str!("release/runtime_validation.rs"),
        include_str!("release/snapshot_validation.rs"),
        include_str!("tests/schema_prefix.rs"),
        include_str!("transition/accounting.rs"),
        include_str!("transition/snapshot.rs"),
    ];
    assert_eq!(files.len(), 12);
    assert!(
        files
            .into_iter()
            .all(|source| source.lines().count() <= 500)
    );
}

#[allow(dead_code)]
pub(in crate::ideal_loads::calc) fn cp442_all_snapshots_for_successor_tests()
-> Vec<super::PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardElseBranchEntrySnapshot>
{
    cp441_all_snapshots_for_successor_tests()
        .into_iter()
        .map(|predecessor| {
            advance(&mut State::new(predecessor.system), predecessor).expect("CP442 snapshot")
        })
        .collect()
}

#[allow(dead_code)]
pub(in crate::ideal_loads::calc) fn cp442_fixture_unit_for_successor_tests() -> (
    Box<PurchasedAirUnitRuntimeState>,
    super::PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardElseBranchEntrySnapshot,
    Route,
) {
    let (mut unit, predecessor, predecessor_route) =
        cp441_guard_false_fixture_unit_for_successor_tests();
    let route = successor_route(predecessor, predecessor_route).expect("CP442 route");
    assert!(
        route.entered,
        "CP443 fixture requires the recurring-warning branch"
    );
    let mut state = State::new(predecessor.system);
    let snapshot = advance_validated(&mut state, predecessor, predecessor_route, route)
        .expect("CP442 fixture");
    assert!(snapshot.heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entered);
    unit.calc_heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry = state;
    (unit, snapshot, route)
}

fn guard_false_predecessor() -> Predecessor {
    cp441_all_snapshots_for_successor_tests()
        .into_iter()
        .find(|snapshot| {
            snapshot.heating_outdoor_air_maximum_flow_first_warning_guard_false_fallthrough
        })
        .expect("CP441 first-warning guard-false predecessor")
}

fn route_for(predecessor: Predecessor) -> Route {
    successor_route(predecessor, predecessor_route_for(predecessor)).expect("CP442 route")
}

fn predecessor_route_for(predecessor: Predecessor) -> PredecessorRoute {
    predecessor_route(predecessor).expect("CP441 route")
}

fn is_public_route(predecessor: Predecessor, route: Route) -> bool {
    matches!(route.logical_index, 0..=8 | 20 | 21 | 26 | 27)
        && !predecessor.single_cool_blocked
        && !route.predecessor_guard_body_entered
}

fn flip_route_component(route: &mut Route, component: usize) {
    match component {
        0 => route.logical_index = (route.logical_index + 1) % 36,
        1 => route.predecessor_guard_false_fallthrough ^= true,
        2 => route.predecessor_guard_body_entered ^= true,
        3 => route.predecessor_assignment_executed ^= true,
        4 => route.predecessor_first_warning_guard_evaluated ^= true,
        5 => route.predecessor_first_warning_branch_entered ^= true,
        6 => route.predecessor_first_warning_guard_false_fallthrough ^= true,
        7 => route.predecessor_counter_increment_executed ^= true,
        8 => route.predecessor_first_warning_call_site_reached ^= true,
        9 => route.predecessor_continue_warning_call_site_reached ^= true,
        10 => route.predecessor_continue_warning_timestamp_call_site_reached ^= true,
        _ => route.entered ^= true,
    }
}

fn flip_predecessor_route_component(route: &mut PredecessorRoute, component: usize) {
    match component {
        0 => route.logical_index = (route.logical_index + 1) % 36,
        1 => route.predecessor_guard_false_fallthrough ^= true,
        2 => route.predecessor_guard_body_entered ^= true,
        3 => route.predecessor_assignment_executed ^= true,
        4 => route.predecessor_first_warning_guard_evaluated ^= true,
        5 => route.predecessor_first_warning_branch_entered ^= true,
        6 => route.predecessor_first_warning_guard_false_fallthrough ^= true,
        7 => route.predecessor_counter_increment_executed ^= true,
        8 => route.predecessor_first_warning_call_site_reached ^= true,
        9 => route.predecessor_continue_warning_call_site_reached ^= true,
        _ => route.continue_warning_timestamp_call_site_reached ^= true,
    }
}

fn assert_bits(left: Option<f64>, right: Option<f64>) {
    assert_eq!(left.map(f64::to_bits), right.map(f64::to_bits));
}
