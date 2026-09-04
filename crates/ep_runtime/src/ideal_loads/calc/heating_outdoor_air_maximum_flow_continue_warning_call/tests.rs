//! CP440 boundary, exhaustive call-site, prefix, and no-service tests.

mod schema_prefix;

use super::transition::{
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowContinueWarningCallRetainedRoute as Route,
    advance_heating_outdoor_air_maximum_flow_continue_warning_call_state as advance,
    advance_heating_outdoor_air_maximum_flow_continue_warning_call_state_with_validated_route as advance_validated,
    heating_outdoor_air_maximum_flow_continue_warning_call_route_from_committed_predecessor as successor_route,
};
use ep_model::IdealLoadsAirSystemId;
use super::{
    PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_CONTINUE_WARNING_CALL_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_CONTINUE_WARNING_CALL_SOURCE,
    PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_CONTINUE_WARNING_CALL_SOURCE_ORDER,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowContinueWarningCallRuntimeState as State,
    heating_outdoor_air_maximum_flow_continue_warning_call_predecessor_cp439_snapshot,
    heating_outdoor_air_maximum_flow_continue_warning_call_snapshot_is_exact,
};
use crate::ideal_loads::calc::{
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCallRetainedRoute as PredecessorRoute,
    cp439_all_snapshots_for_successor_tests, cp439_fixture_unit_for_successor_tests,
    heating_outdoor_air_maximum_flow_first_warning_call_snapshot_route as predecessor_route,
};
use crate::ideal_loads::{
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCallSnapshot as Predecessor,
    PurchasedAirUnitRuntimeState,
    heating_outdoor_air_maximum_flow_first_warning_call_snapshots_match_bit_exact,
};

#[test]
fn cp440_boundary_maps_complete_continue_warning_call_and_excludes_timestamp_call() {
    assert_eq!(
        PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_CONTINUE_WARNING_CALL_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2371-2373"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_CONTINUE_WARNING_CALL_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2374"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_CONTINUE_WARNING_CALL_SOURCE_ORDER,
        &["reach-heating-outdoor-air-maximum-flow-continue-warning-call-site"]
    );
}

#[test]
fn exhaustive_67_routes_preserve_20_47_visibility_and_reach_three_private_calls() {
    let predecessors = cp439_all_snapshots_for_successor_tests();
    assert_eq!(predecessors.len(), 67);
    let mut state = State::new(predecessors[0].system);
    let mut public = 0;
    let mut public_calls = 0;
    let mut private_calls = 0;
    let mut expected = [[0usize; 36]; 9];

    for predecessor in predecessors {
        let predecessor_route = route_for(predecessor);
        let route = successor_route(predecessor, predecessor_route).expect("CP440 route");
        let is_public = is_public_route(predecessor, route);
        public += usize::from(is_public);
        public_calls += usize::from(is_public && route.continue_warning_call_site_reached);
        private_calls += usize::from(!is_public && route.continue_warning_call_site_reached);
        record(&mut expected, route);
        let snapshot = advance(&mut state, predecessor).expect("CP440 snapshot");
        assert!(
            heating_outdoor_air_maximum_flow_first_warning_call_snapshots_match_bit_exact(
                heating_outdoor_air_maximum_flow_continue_warning_call_predecessor_cp439_snapshot(
                    snapshot,
                ),
                predecessor,
            )
        );
        assert_eq!(
            snapshot.heating_outdoor_air_maximum_flow_continue_warning_call_site_reached,
            route.continue_warning_call_site_reached
        );
        assert_eq!(
            snapshot.heating_outdoor_air_maximum_flow_continue_warning_call_site_reached,
            snapshot.heating_outdoor_air_maximum_flow_first_warning_call_site_reached
        );
        assert!(heating_outdoor_air_maximum_flow_continue_warning_call_snapshot_is_exact(snapshot));
    }

    assert_eq!((public, 67 - public), (20, 47));
    assert_eq!((public_calls, private_calls), (0, 3));
    assert_eq!(state.transition_count, 67);
    assert_eq!(state.inactive_transition_count, 64);
    assert_eq!(
        state.heating_outdoor_air_maximum_flow_continue_warning_call_site_count,
        3
    );
    assert_eq!(state.source_site_execution_count, 3);
    assert_eq!(
        [
            state.predecessor_route_counts,
            state.predecessor_guard_false_fallthrough_route_counts,
            state.predecessor_guard_body_entry_route_counts,
            state.predecessor_volume_flow_assignment_route_counts,
            state.predecessor_first_warning_guard_false_fallthrough_route_counts,
            state.predecessor_first_warning_branch_entry_route_counts,
            state.predecessor_first_warning_counter_increment_route_counts,
            state.predecessor_first_warning_call_route_counts,
            state.heating_outdoor_air_maximum_flow_continue_warning_call_route_counts,
        ],
        expected
    );
}

#[test]
fn active_call_aliases_cp439_first_warning_call_and_preserves_sealed_counter() {
    let predecessor = cp439_all_snapshots_for_successor_tests()
        .into_iter()
        .find(|snapshot| {
            snapshot.heating_outdoor_air_maximum_flow_first_warning_call_site_reached
        })
        .expect("active CP439 first-warning call");
    assert_eq!(
        predecessor.assigned_outdoor_air_flow_maximum_heating_output_error_count,
        Some(1)
    );
    let mut state = State::new(predecessor.system);
    let snapshot = advance(&mut state, predecessor).expect("CP440 call");
    assert!(snapshot.heating_outdoor_air_maximum_flow_continue_warning_call_site_reached);
    assert_eq!(
        heating_outdoor_air_maximum_flow_continue_warning_call_predecessor_cp439_snapshot(snapshot)
            .assigned_outdoor_air_flow_maximum_heating_output_error_count,
        Some(1)
    );
}

#[test]
fn cp440_new_state_has_nine_zeroed_width_36_arrays_and_no_service_state() {
    let state = State::new(IdealLoadsAirSystemId(0));
    let arrays = [
        state.predecessor_route_counts,
        state.predecessor_guard_false_fallthrough_route_counts,
        state.predecessor_guard_body_entry_route_counts,
        state.predecessor_volume_flow_assignment_route_counts,
        state.predecessor_first_warning_guard_false_fallthrough_route_counts,
        state.predecessor_first_warning_branch_entry_route_counts,
        state.predecessor_first_warning_counter_increment_route_counts,
        state.predecessor_first_warning_call_route_counts,
        state.heating_outdoor_air_maximum_flow_continue_warning_call_route_counts,
    ];
    assert!(arrays.into_iter().all(|array| array == [0; 36]));
    let source = include_str!("state.rs");
    for forbidden in ["message", "sink", "format", "sqlite", "callback"] {
        assert!(!source.to_ascii_lowercase().contains(forbidden), "{forbidden}");
    }
}

#[test]
fn cp440_transition_contains_no_sink_format_service_or_counter_mutation() {
    let source = include_str!("transition.rs");
    for forbidden in [
        "ShowWarningError",
        "ShowContinueError",
        "EnergyPlus::format",
        "TotalWarningErrors",
        "warning_index",
        "checked_add(1)?",
        "outdoor_air_flow_maximum_heating_output_error_count =",
    ] {
        assert!(!source.contains(forbidden), "{forbidden}");
    }
}

#[test]
fn cp440_subtree_is_twelve_files_and_every_file_is_bounded() {
    let files = [
        include_str!("../heating_outdoor_air_maximum_flow_continue_warning_call.rs"),
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
    assert!(files.into_iter().all(|source| source.lines().count() <= 500));
}

#[allow(dead_code)]
pub(in crate::ideal_loads::calc) fn cp440_all_snapshots_for_successor_tests()
-> Vec<super::PurchasedAirCalcHeatingOutdoorAirMaximumFlowContinueWarningCallSnapshot> {
    cp439_all_snapshots_for_successor_tests()
        .into_iter()
        .map(|predecessor| {
            let mut state = State::new(predecessor.system);
            advance(&mut state, predecessor).expect("CP440 snapshot")
        })
        .collect()
}

#[allow(dead_code)]
pub(in crate::ideal_loads::calc) fn cp440_fixture_unit_for_successor_tests() -> (
    PurchasedAirUnitRuntimeState,
    super::PurchasedAirCalcHeatingOutdoorAirMaximumFlowContinueWarningCallSnapshot,
    Route,
) {
    let (mut unit, predecessor, predecessor_route) = cp439_fixture_unit_for_successor_tests();
    let route = successor_route(predecessor, predecessor_route).expect("CP440 route");
    let mut state = State::new(predecessor.system);
    let snapshot = advance_validated(&mut state, predecessor, predecessor_route, route)
        .expect("CP440");
    unit.calc_heating_outdoor_air_maximum_flow_continue_warning_call = state;
    (unit, snapshot, route)
}

fn route_for(predecessor: Predecessor) -> PredecessorRoute {
    predecessor_route(predecessor).expect("CP439 route")
}

fn is_public_route(predecessor: Predecessor, route: Route) -> bool {
    matches!(route.logical_index, 0..=8 | 20 | 21 | 26 | 27)
        && !predecessor.single_cool_blocked
        && !route.predecessor_guard_body_entered
}

fn record(expected: &mut [[usize; 36]; 9], route: Route) {
    expected[0][route.logical_index] += 1;
    expected[1][route.logical_index] += usize::from(route.predecessor_guard_false_fallthrough);
    expected[2][route.logical_index] += usize::from(route.predecessor_guard_body_entered);
    expected[3][route.logical_index] += usize::from(route.predecessor_assignment_executed);
    expected[4][route.logical_index] +=
        usize::from(route.predecessor_first_warning_guard_false_fallthrough);
    expected[5][route.logical_index] += usize::from(route.predecessor_first_warning_branch_entered);
    expected[6][route.logical_index] += usize::from(route.predecessor_counter_increment_executed);
    expected[7][route.logical_index] +=
        usize::from(route.predecessor_first_warning_call_site_reached);
    expected[8][route.logical_index] += usize::from(route.continue_warning_call_site_reached);
}
