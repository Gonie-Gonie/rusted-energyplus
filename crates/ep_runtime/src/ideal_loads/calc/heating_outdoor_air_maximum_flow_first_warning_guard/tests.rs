//! CP437 boundary, exhaustive guard, state-ownership, forgery, and topology tests.

mod committed_seal;
mod schema_prefix;

use super::PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardRuntimeState as State;
use super::transition::{
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardRetainedRoute as Route,
    advance_heating_outdoor_air_maximum_flow_first_warning_guard_state as advance,
    advance_heating_outdoor_air_maximum_flow_first_warning_guard_state_with_validated_route as advance_validated,
    heating_outdoor_air_maximum_flow_first_warning_guard_route_from_committed_predecessor as successor_route,
};
use crate::ideal_loads::calc::{
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowBodyVolumeFlowAssignmentRetainedRoute as PredecessorRoute,
    cp436_all_snapshots_for_successor_tests,
    heating_outdoor_air_maximum_flow_body_volume_flow_assignment_snapshot_route as predecessor_route,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingMixedAirCallSnapshot as Cp329Snapshot,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowBodyVolumeFlowAssignmentSnapshot as Predecessor,
    PurchasedAirUnitRuntimeState,
};

#[test]
fn cp437_boundary_maps_first_warning_guard_2364_and_excludes_increment_2365() {
    assert_eq!(
        super::PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_GUARD_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2364",
    );
    assert_eq!(
        super::PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_GUARD_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2365",
    );
    assert_eq!(
        super::PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_GUARD_SOURCE_ORDER,
        &[
            "read-state-owned-outdoor-air-flow-maximum-heating-output-error-count",
            "compare-outdoor-air-flow-maximum-heating-output-error-count-less-than-one",
            "enter-heating-outdoor-air-maximum-flow-first-warning-branch-if-satisfied",
        ],
    );
}

#[test]
fn exhaustive_67_routes_preserve_20_47_visibility_and_split_six_private_evaluations() {
    let predecessors = cp436_all_snapshots_for_successor_tests();
    assert_eq!(predecessors.len(), 64);
    let mut state = State::new(predecessors[0].system);
    let mut expected = [[0usize; 36]; 6];
    let mut public = 0usize;
    let mut public_evaluations = 0usize;
    let mut private_evaluations = 0usize;

    for predecessor in predecessors.iter().copied() {
        let predecessor_route = predecessor_route_for(predecessor);
        let route = route_for(predecessor, 0);
        let snapshot = advance_validated(&mut state, predecessor, predecessor_route, route)
            .expect("CP437 counter-zero route");
        record(&mut expected, route);
        let is_public = is_public_route(predecessor, predecessor_route, route);
        public += usize::from(is_public);
        public_evaluations += usize::from(is_public && route.guard_evaluated);
        private_evaluations += usize::from(!is_public && route.guard_evaluated);
        assert_snapshot(snapshot, predecessor, route, 0);
    }

    state.outdoor_air_flow_maximum_heating_output_error_count = 1;
    for predecessor in predecessors
        .iter()
        .copied()
        .filter(|predecessor| predecessor_route_for(*predecessor).assignment_executed)
    {
        let predecessor_route = predecessor_route_for(predecessor);
        let route = route_for(predecessor, 1);
        let snapshot = advance_validated(&mut state, predecessor, predecessor_route, route)
            .expect("CP437 counter-one route");
        record(&mut expected, route);
        let is_public = is_public_route(predecessor, predecessor_route, route);
        public += usize::from(is_public);
        public_evaluations += usize::from(is_public && route.guard_evaluated);
        private_evaluations += usize::from(!is_public && route.guard_evaluated);
        assert_snapshot(snapshot, predecessor, route, 1);
    }

    assert_eq!((public, 67 - public), (20, 47));
    assert_eq!((public_evaluations, private_evaluations), (0, 6));
    assert_eq!(state.transition_count, 67);
    assert_eq!(state.inactive_transition_count, 61);
    assert_eq!(state.guard_evaluation_count, 6);
    assert_eq!(state.first_warning_branch_entry_count, 3);
    assert_eq!(state.guard_false_fallthrough_count, 3);
    assert_eq!(state.source_site_execution_count, 15);
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
        state.heating_outdoor_air_maximum_flow_first_warning_guard_false_fallthrough_route_counts,
        expected[4]
    );
    assert_eq!(
        state.heating_outdoor_air_maximum_flow_first_warning_branch_entry_route_counts,
        expected[5]
    );
    assert_eq!(expected[0][1], 9);
    assert_eq!(expected[1][1], 3);
    assert_eq!(expected[2][1], 6);
    assert_eq!(expected[3][1], 6);
    assert_eq!(expected[4][1], 3);
    assert_eq!(expected[5][1], 3);
    assert_eq!(state.cp436_supply_humidity_ratio_state_owner_count, 37);
    assert_eq!(state.unchanged_supply_humidity_ratio_preservation_count, 37);
    assert_eq!(state.cp436_supply_enthalpy_state_owner_count, 42);
    assert_eq!(state.unchanged_supply_enthalpy_preservation_count, 42);
    assert_eq!(state.cp436_supply_temperature_state_owner_count, 57);
    assert_eq!(state.unchanged_supply_temperature_preservation_count, 57);
    assert_eq!(
        state.outdoor_air_flow_maximum_heating_output_error_count_state_owner_count,
        6
    );
    assert_eq!(
        state.outdoor_air_flow_maximum_heating_output_error_count_read_count,
        6
    );
    assert_eq!(
        state.outdoor_air_flow_maximum_heating_output_error_count_less_than_one_comparison_count,
        6
    );
    assert_eq!(state.outdoor_air_flow_maximum_heating_output_error_count, 1);
    assert!(super::release::state_counts_are_consistent_for_test(&state));
}

#[test]
fn inactive_routes_do_not_read_or_expose_the_state_owned_counter() {
    let predecessor = cp436_all_snapshots_for_successor_tests()
        .into_iter()
        .find(|snapshot| !predecessor_route_for(*snapshot).assignment_executed)
        .expect("inactive predecessor");
    let mut state = State::new(predecessor.system);
    state.outdoor_air_flow_maximum_heating_output_error_count = usize::MAX;
    let snapshot = advance(&mut state, predecessor).expect("inactive CP437");
    assert!(!snapshot.outdoor_air_flow_maximum_heating_output_error_count_state_owned);
    assert!(!snapshot.outdoor_air_flow_maximum_heating_output_error_count_read);
    assert_eq!(
        snapshot.outdoor_air_flow_maximum_heating_output_error_count_before,
        None
    );
    assert_eq!(
        snapshot.outdoor_air_flow_maximum_heating_output_error_count_less_than_one,
        None
    );
    assert_eq!(
        state.outdoor_air_flow_maximum_heating_output_error_count,
        usize::MAX
    );
}

#[test]
fn guard_reads_and_compares_without_incrementing_the_persistent_counter() {
    let predecessor = active_predecessor();
    for counter in [0, 1, usize::MAX] {
        let mut state = State::new(predecessor.system);
        state.outdoor_air_flow_maximum_heating_output_error_count = counter;
        let snapshot = advance(&mut state, predecessor).expect("active CP437");
        assert_eq!(
            snapshot.outdoor_air_flow_maximum_heating_output_error_count_before,
            Some(counter)
        );
        assert_eq!(
            snapshot.outdoor_air_flow_maximum_heating_output_error_count_less_than_one,
            Some(counter < 1)
        );
        assert_eq!(
            state.outdoor_air_flow_maximum_heating_output_error_count,
            counter
        );
    }
    let transition = include_str!("transition.rs");
    assert!(!transition.contains("output_error_count +="));
    assert!(!transition.contains("warning_call"));
    assert!(!transition.contains("recurring"));
    assert!(!transition.contains("clamp"));
}

#[test]
fn every_cp437_route_component_forgery_and_overflow_is_transactional() {
    let predecessor = active_predecessor();
    let predecessor_route = predecessor_route_for(predecessor);
    let exact = route_for(predecessor, 0);
    for component in 0..7 {
        let mut forged = exact;
        match component {
            0 => forged.logical_index = (forged.logical_index + 1) % 36,
            1 => forged.predecessor_guard_false_fallthrough ^= true,
            2 => forged.predecessor_guard_body_entered ^= true,
            3 => forged.predecessor_assignment_executed ^= true,
            4 => forged.guard_evaluated ^= true,
            5 => forged.first_warning_branch_entered ^= true,
            _ => forged.guard_false_fallthrough ^= true,
        }
        let mut state = State::new(predecessor.system);
        let before = state.clone();
        assert!(
            advance_validated(&mut state, predecessor, predecessor_route, forged).is_none(),
            "component {component}"
        );
        assert_eq!(state, before);
    }
    let mut state = State::new(predecessor.system);
    state.transition_count = usize::MAX;
    let before = state.clone();
    assert!(advance(&mut state, predecessor).is_none());
    assert_eq!(state, before);
}

#[test]
fn cp437_new_state_has_six_zeroed_width_36_arrays() {
    let state = State::new(cp436_all_snapshots_for_successor_tests()[0].system);
    let arrays = [
        state.predecessor_route_counts,
        state.predecessor_guard_false_fallthrough_route_counts,
        state.predecessor_guard_body_entry_route_counts,
        state.predecessor_volume_flow_assignment_route_counts,
        state.heating_outdoor_air_maximum_flow_first_warning_guard_false_fallthrough_route_counts,
        state.heating_outdoor_air_maximum_flow_first_warning_branch_entry_route_counts,
    ];
    assert_eq!(arrays.len(), 6);
    assert!(arrays.into_iter().flatten().all(|count| count == 0));
    assert_eq!(state.outdoor_air_flow_maximum_heating_output_error_count, 0);
    assert!(state.latest.is_none());
}

#[test]
fn public_release_uses_cp436_committed_seal_and_rejects_private_guard_evaluation() {
    let release = include_str!("release.rs");
    let seal = release
        .find(
            "heating_outdoor_air_maximum_flow_body_volume_flow_assignment_committed_latest_route(",
        )
        .expect("CP436 seal");
    let reject = release
        .find("return Err(Error::ExactReleaseReductionViolated")
        .expect("private guard rejection");
    let clone = release
        .find("let mut next_state = unit")
        .expect("transactional clone");
    assert!(seal < reject && reject < clone);
    assert!(!release.contains("standard_air_density_kg_per_m3"));
    assert!(!release.contains("DirectZonePurchasedAirCouplingInput"));
}

#[test]
fn cp437_subtree_is_twelve_files_and_every_file_is_bounded() {
    let files = [
        include_str!("../heating_outdoor_air_maximum_flow_first_warning_guard.rs"),
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
pub(in crate::ideal_loads::calc) fn cp437_all_snapshots_for_successor_tests()
-> Vec<super::PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardSnapshot> {
    let predecessors = cp436_all_snapshots_for_successor_tests();
    let mut state = State::new(predecessors[0].system);
    let mut snapshots = predecessors
        .iter()
        .copied()
        .map(|predecessor| advance(&mut state, predecessor).expect("CP437 counter-zero snapshot"))
        .collect::<Vec<_>>();
    state.outdoor_air_flow_maximum_heating_output_error_count = 1;
    snapshots.extend(
        predecessors
            .into_iter()
            .filter(|predecessor| predecessor_route_for(*predecessor).assignment_executed)
            .map(|predecessor| {
                advance(&mut state, predecessor).expect("CP437 counter-one snapshot")
            }),
    );
    snapshots
}

#[allow(dead_code)]
pub(in crate::ideal_loads::calc) fn cp437_fixture_unit_for_successor_tests() -> (
    PurchasedAirUnitRuntimeState,
    super::PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardSnapshot,
    Route,
    Option<Cp329Snapshot>,
) {
    let (mut unit, predecessor, predecessor_route, owner) =
        crate::ideal_loads::calc::cp436_fixture_unit_for_successor_tests();
    let route = route_for(predecessor, 0);
    let mut state = State::new(predecessor.system);
    let snapshot =
        advance_validated(&mut state, predecessor, predecessor_route, route).expect("CP437");
    unit.calc_heating_outdoor_air_maximum_flow_first_warning_guard = state;
    (unit, snapshot, route, owner)
}

fn active_predecessor() -> Predecessor {
    cp436_all_snapshots_for_successor_tests()
        .into_iter()
        .find(|snapshot| predecessor_route_for(*snapshot).assignment_executed)
        .expect("CP436 assignment predecessor")
}

fn route_for(predecessor: Predecessor, counter: usize) -> Route {
    successor_route(predecessor, predecessor_route_for(predecessor), counter).expect("CP437 route")
}

fn predecessor_route_for(predecessor: Predecessor) -> PredecessorRoute {
    predecessor_route(predecessor).expect("CP436 route")
}

fn is_public_route(
    predecessor: Predecessor,
    predecessor_route: PredecessorRoute,
    route: Route,
) -> bool {
    is_public_logical_index(route.logical_index)
        && !predecessor.single_cool_blocked
        && !predecessor_route.predecessor_guard_body_entered
}

fn is_public_logical_index(index: usize) -> bool {
    matches!(index, 0..=8 | 20 | 21 | 26 | 27)
}

fn record(expected: &mut [[usize; 36]; 6], route: Route) {
    expected[0][route.logical_index] += 1;
    expected[1][route.logical_index] += usize::from(route.predecessor_guard_false_fallthrough);
    expected[2][route.logical_index] += usize::from(route.predecessor_guard_body_entered);
    expected[3][route.logical_index] += usize::from(route.predecessor_assignment_executed);
    expected[4][route.logical_index] += usize::from(route.guard_false_fallthrough);
    expected[5][route.logical_index] += usize::from(route.first_warning_branch_entered);
}

fn assert_snapshot(
    snapshot: super::PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardSnapshot,
    predecessor: Predecessor,
    route: Route,
    counter: usize,
) {
    assert_eq!(
        snapshot.heating_outdoor_air_maximum_flow_first_warning_guard_evaluated,
        route.guard_evaluated
    );
    assert_eq!(
        snapshot.outdoor_air_flow_maximum_heating_output_error_count_before,
        route.guard_evaluated.then_some(counter)
    );
    assert_eq!(
        snapshot.outdoor_air_flow_maximum_heating_output_error_count_less_than_one,
        route.guard_evaluated.then_some(counter < 1)
    );
    assert_eq!(
        snapshot.heating_outdoor_air_maximum_flow_first_warning_branch_entered,
        route.first_warning_branch_entered
    );
    assert_eq!(
        snapshot.heating_outdoor_air_maximum_flow_first_warning_guard_false_fallthrough,
        route.guard_false_fallthrough
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
        super::heating_outdoor_air_maximum_flow_first_warning_guard_snapshot_is_exact(snapshot)
    );
}

fn assert_bits(left: Option<f64>, right: Option<f64>) {
    assert_eq!(left.map(f64::to_bits), right.map(f64::to_bits));
}
