//! CP434 boundary, exhaustive route, forgery, overflow, and bounded-path tests.

mod schema_prefix;

use super::PurchasedAirCalcHeatingOperatingModeDeadbandAssignmentRuntimeState as State;
use super::transition::{
    PurchasedAirCalcHeatingOperatingModeDeadbandAssignmentRetainedRoute as Route,
    advance_heating_operating_mode_deadband_assignment_state as advance,
    advance_heating_operating_mode_deadband_assignment_state_with_validated_route as advance_validated,
    heating_operating_mode_deadband_assignment_route_from_committed_predecessor as successor_route,
};
use crate::ideal_loads::calc::{
    PurchasedAirCalcHeatingModeGuardElseBranchEntryRetainedRoute as PredecessorRoute,
    cp433_all_snapshots_for_successor_tests,
    heating_mode_guard_else_branch_entry_snapshot_route as predecessor_route,
};
use crate::ideal_loads::{
    IdealLoadsSensibleMode, PurchasedAirCalcHeatingModeGuardElseBranchEntrySnapshot as Predecessor,
};

#[test]
fn cp434_boundary_maps_deadband_assignment_2351_and_excludes_2361() {
    assert_eq!(
        super::PURCHASED_AIR_CALC_HEATING_OPERATING_MODE_DEADBAND_ASSIGNMENT_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2351",
    );
    assert_eq!(
        super::PURCHASED_AIR_CALC_HEATING_OPERATING_MODE_DEADBAND_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2361",
    );
    assert_eq!(
        super::PURCHASED_AIR_CALC_HEATING_OPERATING_MODE_DEADBAND_ASSIGNMENT_SOURCE_ORDER,
        &["assign-local-operating-mode-deadband"],
    );
}

#[test]
fn exhaustive_61_routes_have_exact_deadband_partition_and_accounting() {
    let predecessors = cp433_all_snapshots_for_successor_tests();
    assert_eq!(predecessors.len(), 61);
    let mut state = State::new(predecessors[0].system);
    let mut expected = [[0usize; 36]; 2];
    let mut public = 0usize;
    let mut public_assignments = 0usize;
    let mut private_assignments = 0usize;
    let mut heat_assignments = 0usize;
    let mut guard_evaluations = 0usize;
    for predecessor in predecessors {
        let predecessor_route = predecessor_route_for(predecessor);
        let route = route_for(predecessor);
        let snapshot =
            advance_validated(&mut state, predecessor, predecessor_route, route).expect("CP434");
        expected[0][route.logical_index] += 1;
        if route.assignment_executed {
            expected[1][route.logical_index] += 1;
        }
        let is_public =
            is_public_logical_index(route.logical_index) && !route.predecessor_single_cool_blocked;
        public += usize::from(is_public);
        public_assignments += usize::from(is_public && route.assignment_executed);
        private_assignments += usize::from(!is_public && route.assignment_executed);
        heat_assignments +=
            usize::from(predecessor.heating_operating_mode_heat_assignment_executed);
        guard_evaluations += usize::from(predecessor.heating_mode_guard_evaluated);

        assert_eq!(
            route.assignment_executed,
            predecessor.heating_mode_guard_else_branch_entered,
        );
        assert!(
            !(route.assignment_executed
                && predecessor.heating_operating_mode_heat_assignment_executed)
        );
        assert_eq!(
            snapshot.heating_operating_mode_deadband_assignment_executed,
            route.assignment_executed,
        );
        assert_eq!(
            snapshot.assigned_heating_operating_mode_deadband,
            route
                .assignment_executed
                .then_some(IdealLoadsSensibleMode::Deadband),
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
        assert!(super::heating_operating_mode_deadband_assignment_snapshot_is_exact(snapshot));
    }
    assert_eq!((public, 61 - public), (20, 41));
    assert_eq!((public_assignments, private_assignments), (1, 1));
    assert_eq!(
        (
            heat_assignments,
            state.heating_operating_mode_deadband_assignment_count
        ),
        (1, 2)
    );
    assert_eq!(
        heat_assignments + state.heating_operating_mode_deadband_assignment_count,
        guard_evaluations
    );
    assert_eq!(state.transition_count, 61);
    assert_eq!(state.inactive_transition_count, 59);
    assert_eq!(state.heating_operating_mode_deadband_assignment_count, 2);
    assert_eq!(state.source_site_execution_count, 2);
    assert_eq!(state.predecessor_route_counts, expected[0]);
    assert_eq!(
        state.heating_operating_mode_deadband_assignment_route_counts,
        expected[1],
    );
    assert_eq!(expected[0][1], 3);
    assert_eq!(expected[1][1], 2);
    assert_eq!(state.cp433_supply_humidity_ratio_state_owner_count, 37);
    assert_eq!(state.unchanged_supply_humidity_ratio_preservation_count, 37);
    assert_eq!(state.cp433_supply_enthalpy_state_owner_count, 42);
    assert_eq!(state.unchanged_supply_enthalpy_preservation_count, 42);
    assert_eq!(state.cp433_supply_temperature_state_owner_count, 57);
    assert_eq!(state.unchanged_supply_temperature_preservation_count, 57);
    assert_eq!(state.cp434_heating_operating_mode_state_owner_count, 2);
    assert_eq!(state.heating_operating_mode_assignment_write_count, 2);
    assert!(super::release::state_counts_are_consistent_for_test(&state));
}

#[test]
fn cp434_new_state_has_exactly_two_zeroed_lossless_route_partitions() {
    let state = State::new(cp433_all_snapshots_for_successor_tests()[0].system);
    let arrays = [
        state.predecessor_route_counts,
        state.heating_operating_mode_deadband_assignment_route_counts,
    ];
    assert_eq!(arrays.len(), 2);
    assert!(arrays.into_iter().flatten().all(|count| count == 0));
    assert_eq!(state.source_site_execution_count, 0);
    assert!(state.latest.is_none());
}

#[test]
fn every_cp434_route_component_forgery_is_transactional() {
    let predecessor = active_predecessor();
    let predecessor_route = predecessor_route_for(predecessor);
    let exact = route_for(predecessor);
    for component in 0..14 {
        let mut forged = exact;
        flip_route_component(&mut forged, component);
        let mut state = State::new(predecessor.system);
        let before = state.clone();
        assert!(
            advance_validated(&mut state, predecessor, predecessor_route, forged).is_none(),
            "component {component}",
        );
        assert_eq!(state, before, "component {component}");
    }
}

#[test]
fn every_supplied_cp433_route_component_forgery_is_transactional() {
    let predecessor = active_predecessor();
    let exact_predecessor_route = predecessor_route_for(predecessor);
    let exact = route_for(predecessor);
    for component in 0..13 {
        let mut forged = exact_predecessor_route;
        flip_predecessor_route_component(&mut forged, component);
        let mut state = State::new(predecessor.system);
        let before = state.clone();
        assert!(
            advance_validated(&mut state, predecessor, forged, exact).is_none(),
            "CP433 component {component}",
        );
        assert_eq!(state, before, "CP433 component {component}");
    }
}

#[test]
fn every_independent_cp434_counter_overflow_is_transactional() {
    let predecessors = cp433_all_snapshots_for_successor_tests();
    let active = predecessors
        .iter()
        .copied()
        .find(|snapshot| route_for(*snapshot).assignment_executed)
        .expect("active");
    let inactive = predecessors
        .iter()
        .copied()
        .find(|snapshot| !route_for(*snapshot).assignment_executed)
        .expect("inactive");
    let owned = predecessors
        .iter()
        .copied()
        .find(|snapshot| {
            snapshot.resulting_supply_humidity_ratio.is_some()
                && snapshot.resulting_supply_enthalpy_j_per_kg.is_some()
                && snapshot.resulting_supply_temperature_c.is_some()
        })
        .expect("W/H/T owner");

    for slot in 0..14 {
        let predecessor = match slot {
            2 => inactive,
            6..=11 => owned,
            _ => active,
        };
        let predecessor_route = predecessor_route_for(predecessor);
        let route = route_for(predecessor);
        let mut state = State::new(predecessor.system);
        match slot {
            0 => state.transition_count = usize::MAX,
            1 => state.predecessor_route_counts[route.logical_index] = usize::MAX,
            2 => state.inactive_transition_count = usize::MAX,
            3 => state.heating_operating_mode_deadband_assignment_count = usize::MAX,
            4 => {
                state.heating_operating_mode_deadband_assignment_route_counts[route.logical_index] =
                    usize::MAX
            }
            5 => state.source_site_execution_count = usize::MAX,
            6 => state.cp433_supply_humidity_ratio_state_owner_count = usize::MAX,
            7 => state.unchanged_supply_humidity_ratio_preservation_count = usize::MAX,
            8 => state.cp433_supply_enthalpy_state_owner_count = usize::MAX,
            9 => state.unchanged_supply_enthalpy_preservation_count = usize::MAX,
            10 => state.cp433_supply_temperature_state_owner_count = usize::MAX,
            11 => state.unchanged_supply_temperature_preservation_count = usize::MAX,
            12 => state.cp434_heating_operating_mode_state_owner_count = usize::MAX,
            _ => state.heating_operating_mode_assignment_write_count = usize::MAX,
        }
        let before = state.clone();
        assert!(
            advance_validated(&mut state, predecessor, predecessor_route, route).is_none(),
            "overflow {slot}",
        );
        assert_eq!(state, before, "overflow {slot}");
    }
}

#[test]
fn cp434_hot_release_and_retained_validation_are_bounded() {
    let release = include_str!("release.rs");
    let runtime = include_str!("release/runtime_validation.rs");
    let hot = release
        .split_once("pub fn advance_direct_no_oa_calc_heating_operating_mode_deadband_assignment")
        .and_then(|(_, tail)| tail.split_once("#[allow(dead_code)]"))
        .map(|(hot, _)| hot)
        .expect("CP434 hot release");
    for source in [hot, runtime] {
        for forbidden in [
            "heating_mode_guard_else_branch_entry_snapshot_route(",
            "private_characterization",
            "DirectZonePurchasedAirCouplingInput",
            "numerical_dto",
            "numerical_feed",
        ] {
            assert!(!source.contains(forbidden), "{forbidden}");
        }
    }
    assert_eq!(
        hot.matches("heating_mode_guard_else_branch_entry_committed_latest_route(")
            .count(),
        1,
    );
}

#[test]
fn cp434_subtree_is_exactly_eleven_files_and_each_core_file_is_bounded() {
    let files = [
        include_str!("../heating_operating_mode_deadband_assignment.rs"),
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
    assert_eq!(files.len() - 1, 11);
    assert!(
        files
            .into_iter()
            .all(|source| source.lines().count() <= 500)
    );
}

#[allow(dead_code)]
pub(in crate::ideal_loads::calc) fn cp434_all_snapshots_for_successor_tests()
-> Vec<super::PurchasedAirCalcHeatingOperatingModeDeadbandAssignmentSnapshot> {
    let predecessors = cp433_all_snapshots_for_successor_tests();
    let mut state = State::new(predecessors[0].system);
    predecessors
        .into_iter()
        .map(|predecessor| advance(&mut state, predecessor).expect("CP434 snapshot"))
        .collect()
}

fn active_predecessor() -> Predecessor {
    cp433_all_snapshots_for_successor_tests()
        .into_iter()
        .find(|snapshot| route_for(*snapshot).assignment_executed)
        .expect("Deadband predecessor")
}

fn route_for(predecessor: Predecessor) -> Route {
    let predecessor_route = predecessor_route_for(predecessor);
    successor_route(predecessor, predecessor_route).expect("CP434 route")
}

fn predecessor_route_for(predecessor: Predecessor) -> PredecessorRoute {
    predecessor_route(predecessor).expect("CP433 route")
}

fn flip_route_component(route: &mut Route, component: usize) {
    match component {
        0 => route.logical_index = (route.logical_index + 1) % 36,
        1 => route.predecessor_active ^= true,
        2 => route.predecessor_assignment_executed ^= true,
        3 => route.predecessor_entered ^= true,
        4 => route.predecessor_total_output_assignment_executed ^= true,
        5 => route.predecessor_heating_or_no_load_case_entered ^= true,
        6 => route.predecessor_heating_mode_guard_evaluated ^= true,
        7 => route.predecessor_sensible_comparison_satisfied ^= true,
        8 => route.predecessor_single_cool_blocked ^= true,
        9 => route.predecessor_heating_operating_mode_body_entered ^= true,
        10 => route.predecessor_heating_mode_guard_false_fallthrough ^= true,
        11 => route.predecessor_heating_operating_mode_heat_assignment_executed ^= true,
        12 => route.predecessor_heating_mode_guard_else_branch_entered ^= true,
        _ => route.assignment_executed ^= true,
    }
}

fn flip_predecessor_route_component(route: &mut PredecessorRoute, component: usize) {
    match component {
        0 => route.logical_index = (route.logical_index + 1) % 36,
        1 => route.predecessor_active ^= true,
        2 => route.predecessor_assignment_executed ^= true,
        3 => route.predecessor_entered ^= true,
        4 => route.predecessor_total_output_assignment_executed ^= true,
        5 => route.predecessor_heating_or_no_load_case_entered ^= true,
        6 => route.predecessor_heating_mode_guard_evaluated ^= true,
        7 => route.predecessor_sensible_comparison_satisfied ^= true,
        8 => route.predecessor_single_cool_blocked ^= true,
        9 => route.predecessor_heating_operating_mode_body_entered ^= true,
        10 => route.predecessor_heating_mode_guard_false_fallthrough ^= true,
        11 => route.assignment_executed ^= true,
        _ => route.entered ^= true,
    }
}

fn is_public_logical_index(index: usize) -> bool {
    matches!(index, 0..=8 | 20 | 21 | 26 | 27)
}

fn assert_bits(left: Option<f64>, right: Option<f64>) {
    assert_eq!(left.map(f64::to_bits), right.map(f64::to_bits));
}
