//! CP438 boundary, exhaustive increment, canonical-owner, forgery, and overflow tests.

mod schema_prefix;
mod committed_seal;

use super::PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCounterIncrementRuntimeState as State;
use super::transition::{
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCounterIncrementRetainedRoute as Route,
    advance_heating_outdoor_air_maximum_flow_first_warning_counter_increment_state as advance,
    advance_heating_outdoor_air_maximum_flow_first_warning_counter_increment_state_with_validated_route as advance_validated,
    heating_outdoor_air_maximum_flow_first_warning_counter_increment_route_from_committed_predecessor as successor_route,
};
use crate::ideal_loads::calc::{
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardRetainedRoute as PredecessorRoute,
    advance_heating_outdoor_air_maximum_flow_first_warning_guard_state as advance_cp437,
    cp437_all_snapshots_for_successor_tests,
    heating_outdoor_air_maximum_flow_first_warning_guard_snapshot_route as predecessor_route,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingMixedAirCallSnapshot as Cp329Snapshot,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardRuntimeState as CounterOwner,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardSnapshot as Predecessor,
    PurchasedAirUnitRuntimeState,
    heating_outdoor_air_maximum_flow_first_warning_guard_predecessor_cp436_snapshot,
    heating_outdoor_air_maximum_flow_first_warning_guard_snapshots_match_bit_exact,
};

#[test]
fn cp438_boundary_maps_increment_2365_and_excludes_warning_call_2366() {
    assert_eq!(
        super::PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_COUNTER_INCREMENT_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2365",
    );
    assert_eq!(
        super::PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_COUNTER_INCREMENT_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2366",
    );
    assert_eq!(
        super::PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_COUNTER_INCREMENT_SOURCE_ORDER,
        &["increment-state-owned-outdoor-air-flow-maximum-heating-output-error-count"],
    );
}

#[test]
fn exhaustive_67_routes_preserve_20_47_visibility_and_increment_three_private_alternatives() {
    let predecessors = cp437_all_snapshots_for_successor_tests();
    assert_eq!(predecessors.len(), 67);
    let mut state = State::new(predecessors[0].system);
    let mut expected = [[0usize; 36]; 7];
    let mut public = 0usize;
    let mut public_increments = 0usize;
    let mut private_increments = 0usize;

    for predecessor in predecessors {
        let predecessor_route = predecessor_route_for(predecessor);
        let route = route_for(predecessor);
        let mut owner = counter_owner_for(predecessor);
        let before = owner.outdoor_air_flow_maximum_heating_output_error_count;
        let snapshot = advance_validated(
            &mut state,
            &mut owner,
            predecessor,
            predecessor_route,
            route,
        )
        .expect("CP438 route");
        record(&mut expected, route);
        let is_public = is_public_route(predecessor, route);
        public += usize::from(is_public);
        public_increments += usize::from(is_public && route.counter_increment_executed);
        private_increments += usize::from(!is_public && route.counter_increment_executed);
        assert_snapshot(snapshot, predecessor, route);
        assert_eq!(
            owner.outdoor_air_flow_maximum_heating_output_error_count,
            if route.counter_increment_executed {
                1
            } else {
                before
            }
        );
    }

    assert_eq!((public, 67 - public), (20, 47));
    assert_eq!((public_increments, private_increments), (0, 3));
    assert_eq!(state.transition_count, 67);
    assert_eq!(state.inactive_transition_count, 64);
    assert_eq!(
        state.outdoor_air_flow_maximum_heating_output_error_count_increment_count,
        3
    );
    assert_eq!(state.source_site_execution_count, 3);
    assert_eq!(state.predecessor_route_counts, expected[0]);
    assert_eq!(
        state.predecessor_guard_false_fallthrough_route_counts,
        expected[1]
    );
    assert_eq!(state.predecessor_guard_body_entry_route_counts, expected[2]);
    assert_eq!(
        state.predecessor_volume_flow_assignment_route_counts,
        expected[3]
    );
    assert_eq!(
        state.predecessor_first_warning_guard_false_fallthrough_route_counts,
        expected[4]
    );
    assert_eq!(
        state.predecessor_first_warning_branch_entry_route_counts,
        expected[5]
    );
    assert_eq!(
        state.heating_outdoor_air_maximum_flow_first_warning_counter_increment_route_counts,
        expected[6]
    );
    assert_eq!(
        (
            expected[0][1],
            expected[1][1],
            expected[2][1],
            expected[3][1],
            expected[4][1],
            expected[5][1],
            expected[6][1],
        ),
        (9, 3, 6, 6, 3, 3, 3)
    );
    assert_eq!(
        (
            state.cp437_supply_humidity_ratio_state_owner_count,
            state.cp437_supply_enthalpy_state_owner_count,
            state.cp437_supply_temperature_state_owner_count,
        ),
        (37, 42, 57)
    );
    assert_eq!(
        state.cp437_supply_humidity_ratio_state_owner_count,
        state.unchanged_supply_humidity_ratio_preservation_count
    );
    assert_eq!(
        state.cp437_supply_enthalpy_state_owner_count,
        state.unchanged_supply_enthalpy_preservation_count
    );
    assert_eq!(
        state.cp437_supply_temperature_state_owner_count,
        state.unchanged_supply_temperature_preservation_count
    );
    assert_eq!(
        state.cp437_outdoor_air_flow_maximum_heating_output_error_count_state_owner_count,
        3
    );
    assert_eq!(
        state.outdoor_air_flow_maximum_heating_output_error_count_increment_write_count,
        3
    );
    assert!(super::release::state_counts_are_consistent_for_test(&state));
}

#[test]
fn inactive_routes_leave_the_canonical_cp437_counter_unchanged() {
    let predecessors = cp437_all_snapshots_for_successor_tests();
    for predecessor in predecessors
        .into_iter()
        .filter(|snapshot| !route_for(*snapshot).counter_increment_executed)
    {
        let mut state = State::new(predecessor.system);
        let mut owner = counter_owner_for(predecessor);
        if !predecessor.heating_outdoor_air_maximum_flow_first_warning_guard_evaluated {
            owner.outdoor_air_flow_maximum_heating_output_error_count = usize::MAX;
        }
        let before = owner.clone();
        let snapshot = advance(&mut state, &mut owner, predecessor).expect("inactive CP438");
        assert!(
            !snapshot.heating_outdoor_air_maximum_flow_first_warning_counter_increment_executed
        );
        assert_eq!(
            snapshot.assigned_outdoor_air_flow_maximum_heating_output_error_count,
            None
        );
        assert_eq!(owner, before);
    }
}

#[test]
fn active_increment_is_exactly_zero_to_one_and_persists_into_the_next_guard() {
    let predecessor = active_predecessor();
    let cp436 = heating_outdoor_air_maximum_flow_first_warning_guard_predecessor_cp436_snapshot(
        predecessor,
    );
    let mut owner = CounterOwner::new(predecessor.system);
    let first = advance_cp437(&mut owner, cp436).expect("first CP437");
    assert!(first.heating_outdoor_air_maximum_flow_first_warning_branch_entered);
    let mut state = State::new(predecessor.system);
    let incremented = advance(&mut state, &mut owner, first).expect("CP438 increment");
    assert_eq!(
        incremented.assigned_outdoor_air_flow_maximum_heating_output_error_count,
        Some(1)
    );
    assert_eq!(owner.outdoor_air_flow_maximum_heating_output_error_count, 1);
    let second = advance_cp437(&mut owner, cp436).expect("next CP437");
    assert_eq!(
        second.outdoor_air_flow_maximum_heating_output_error_count_before,
        Some(1)
    );
    assert!(second.heating_outdoor_air_maximum_flow_first_warning_guard_false_fallthrough);
    assert!(!second.heating_outdoor_air_maximum_flow_first_warning_branch_entered);
}

#[test]
fn every_route_component_forgery_and_counter_owner_mismatch_is_transactional() {
    let predecessor = active_predecessor();
    let predecessor_route = predecessor_route_for(predecessor);
    let exact = route_for(predecessor);
    for component in 0..8 {
        let mut forged = exact;
        match component {
            0 => forged.logical_index = (forged.logical_index + 1) % 36,
            1 => forged.predecessor_guard_false_fallthrough ^= true,
            2 => forged.predecessor_guard_body_entered ^= true,
            3 => forged.predecessor_assignment_executed ^= true,
            4 => forged.predecessor_first_warning_guard_evaluated ^= true,
            5 => forged.predecessor_first_warning_branch_entered ^= true,
            6 => forged.predecessor_first_warning_guard_false_fallthrough ^= true,
            _ => forged.counter_increment_executed ^= true,
        }
        let mut state = State::new(predecessor.system);
        let mut owner = counter_owner_for(predecessor);
        let state_before = state.clone();
        let owner_before = owner.clone();
        assert!(
            advance_validated(
                &mut state,
                &mut owner,
                predecessor,
                predecessor_route,
                forged,
            )
            .is_none(),
            "component {component}"
        );
        assert_eq!(state, state_before);
        assert_eq!(owner, owner_before);
    }
    let mut state = State::new(predecessor.system);
    let mut owner = counter_owner_for(predecessor);
    owner.outdoor_air_flow_maximum_heating_output_error_count = 1;
    let state_before = state.clone();
    let owner_before = owner.clone();
    assert!(advance(&mut state, &mut owner, predecessor).is_none());
    assert_eq!(state, state_before);
    assert_eq!(owner, owner_before);
}

#[test]
fn cp438_new_state_has_seven_zeroed_width_36_arrays_and_no_counter_duplicate() {
    let state = State::new(cp437_all_snapshots_for_successor_tests()[0].system);
    let arrays = [
        state.predecessor_route_counts,
        state.predecessor_guard_false_fallthrough_route_counts,
        state.predecessor_guard_body_entry_route_counts,
        state.predecessor_volume_flow_assignment_route_counts,
        state.predecessor_first_warning_guard_false_fallthrough_route_counts,
        state.predecessor_first_warning_branch_entry_route_counts,
        state.heating_outdoor_air_maximum_flow_first_warning_counter_increment_route_counts,
    ];
    assert_eq!(arrays.len(), 7);
    assert!(arrays.into_iter().flatten().all(|count| count == 0));
    assert!(
        !include_str!("state.rs")
            .contains("pub outdoor_air_flow_maximum_heating_output_error_count: usize")
    );
    assert!(state.latest.is_none());
}

#[test]
fn public_release_uses_cp437_committed_counter_seal_before_transactional_clones() {
    let release = include_str!("release.rs");
    let seal = release
        .find("heating_outdoor_air_maximum_flow_first_warning_guard_committed_latest_route_and_outdoor_air_flow_maximum_heating_output_error_count(")
        .expect("CP437 seal");
    let reject = release
        .find("return Err(Error::ExactReleaseReductionViolated")
        .expect("private increment rejection");
    let clone = release
        .find("let mut next_state = unit")
        .expect("transactional clone");
    assert!(seal < reject && reject < clone);
    assert!(release.contains("let mut next_counter_owner = unit"));
    assert!(!release.contains("DirectZonePurchasedAirCouplingInput"));
    assert!(!release.contains("ShowWarningError"));
}

#[test]
fn cp438_sealed_subtree_is_fourteen_files_and_every_file_is_bounded() {
    let files = [
        include_str!("../heating_outdoor_air_maximum_flow_first_warning_counter_increment.rs"),
        include_str!("release.rs"),
        include_str!("state.rs"),
        include_str!("tests.rs"),
        include_str!("transition.rs"),
        include_str!("release/error.rs"),
        include_str!("release/committed.rs"),
        include_str!("release/prefix.rs"),
        include_str!("release/runtime_validation.rs"),
        include_str!("release/snapshot_validation.rs"),
        include_str!("tests/schema_prefix.rs"),
        include_str!("tests/committed_seal.rs"),
        include_str!("transition/accounting.rs"),
        include_str!("transition/snapshot.rs"),
    ];
    assert_eq!(files.len(), 14);
    assert!(
        files
            .into_iter()
            .all(|source| source.lines().count() <= 500)
    );
}

#[allow(dead_code)]
pub(in crate::ideal_loads::calc) fn cp438_all_snapshots_for_successor_tests()
-> Vec<super::PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCounterIncrementSnapshot> {
    cp437_all_snapshots_for_successor_tests()
        .into_iter()
        .map(|predecessor| {
            let mut state = State::new(predecessor.system);
            let mut owner = counter_owner_for(predecessor);
            advance(&mut state, &mut owner, predecessor).expect("CP438 snapshot")
        })
        .collect()
}

#[allow(dead_code)]
pub(in crate::ideal_loads::calc) fn cp438_fixture_unit_for_successor_tests() -> (
    PurchasedAirUnitRuntimeState,
    super::PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCounterIncrementSnapshot,
    Route,
    Option<Cp329Snapshot>,
) {
    let (mut unit, predecessor, predecessor_route, owner) =
        crate::ideal_loads::calc::cp437_fixture_unit_for_successor_tests();
    let route = successor_route(predecessor, predecessor_route).expect("CP438 route");
    let mut state = State::new(predecessor.system);
    let mut counter_owner = unit
        .calc_heating_outdoor_air_maximum_flow_first_warning_guard
        .clone();
    let snapshot = advance_validated(
        &mut state,
        &mut counter_owner,
        predecessor,
        predecessor_route,
        route,
    )
    .expect("CP438");
    unit.calc_heating_outdoor_air_maximum_flow_first_warning_guard = counter_owner;
    unit.calc_heating_outdoor_air_maximum_flow_first_warning_counter_increment = state;
    (unit, snapshot, route, owner)
}

fn active_predecessor() -> Predecessor {
    cp437_all_snapshots_for_successor_tests()
        .into_iter()
        .find(|snapshot| predecessor_route_for(*snapshot).first_warning_branch_entered)
        .expect("CP437 first-warning entry")
}

fn predecessor_matching(
    predicate: impl Fn(Predecessor) -> bool,
    description: &'static str,
) -> Predecessor {
    cp437_all_snapshots_for_successor_tests()
        .into_iter()
        .find(|snapshot| predicate(*snapshot))
        .expect(description)
}

fn route_for(predecessor: Predecessor) -> Route {
    successor_route(predecessor, predecessor_route_for(predecessor)).expect("CP438 route")
}

fn predecessor_route_for(predecessor: Predecessor) -> PredecessorRoute {
    predecessor_route(predecessor).expect("CP437 route")
}

fn counter_owner_for(predecessor: Predecessor) -> CounterOwner {
    let mut owner = CounterOwner::new(predecessor.system);
    owner.outdoor_air_flow_maximum_heating_output_error_count = predecessor
        .outdoor_air_flow_maximum_heating_output_error_count_before
        .unwrap_or(0);
    owner
}

fn is_public_route(predecessor: Predecessor, route: Route) -> bool {
    is_public_logical_index(route.logical_index)
        && !predecessor.single_cool_blocked
        && !route.predecessor_guard_body_entered
}

fn is_public_logical_index(index: usize) -> bool {
    matches!(index, 0..=8 | 20 | 21 | 26 | 27)
}

fn record(expected: &mut [[usize; 36]; 7], route: Route) {
    expected[0][route.logical_index] += 1;
    expected[1][route.logical_index] += usize::from(route.predecessor_guard_false_fallthrough);
    expected[2][route.logical_index] += usize::from(route.predecessor_guard_body_entered);
    expected[3][route.logical_index] += usize::from(route.predecessor_assignment_executed);
    expected[4][route.logical_index] +=
        usize::from(route.predecessor_first_warning_guard_false_fallthrough);
    expected[5][route.logical_index] += usize::from(route.predecessor_first_warning_branch_entered);
    expected[6][route.logical_index] += usize::from(route.counter_increment_executed);
}

fn assert_snapshot(
    snapshot: super::PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCounterIncrementSnapshot,
    predecessor: Predecessor,
    route: Route,
) {
    assert!(heating_outdoor_air_maximum_flow_first_warning_guard_snapshots_match_bit_exact(
        super::heating_outdoor_air_maximum_flow_first_warning_counter_increment_predecessor_cp437_snapshot(snapshot),
        predecessor,
    ));
    assert_eq!(
        snapshot.heating_outdoor_air_maximum_flow_first_warning_counter_increment_executed,
        route.counter_increment_executed
    );
    assert_eq!(
        snapshot.outdoor_air_flow_maximum_heating_output_error_count_increment_performed,
        route.counter_increment_executed
    );
    assert_eq!(
        snapshot.assigned_outdoor_air_flow_maximum_heating_output_error_count,
        route.counter_increment_executed.then_some(1)
    );
    assert!(
        super::heating_outdoor_air_maximum_flow_first_warning_counter_increment_snapshot_is_exact(
            snapshot
        )
    );
}
