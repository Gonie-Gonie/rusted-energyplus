//! CP433 boundary, exhaustive route, forgery, overflow, and bounded-path tests.

mod committed_seal;
mod schema_prefix;

use super::transition::{
    PurchasedAirCalcHeatingModeGuardElseBranchEntryRetainedRoute as Route,
    advance_heating_mode_guard_else_branch_entry_state as advance,
    advance_heating_mode_guard_else_branch_entry_state_with_validated_route as advance_validated,
    heating_mode_guard_else_branch_entry_route_from_committed_predecessor as successor_route,
};
use super::PurchasedAirCalcHeatingModeGuardElseBranchEntryRuntimeState as State;
use crate::ideal_loads::calc::{
    PurchasedAirCalcHeatingOperatingModeHeatAssignmentRetainedRoute as PredecessorRoute,
    cp432_all_snapshots_for_successor_tests,
    cp432_fixture_unit_for_successor_tests,
    heating_operating_mode_heat_assignment_snapshot_route as predecessor_route,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingMixedAirCallSnapshot as Cp329Snapshot,
    PurchasedAirCalcHeatingOperatingModeHeatAssignmentSnapshot as Predecessor,
    PurchasedAirUnitRuntimeState,
};

#[test]
fn cp433_boundary_maps_structural_2350_and_excludes_deadband_assignment_2351() {
    assert_eq!(
        super::PURCHASED_AIR_CALC_HEATING_MODE_GUARD_ELSE_BRANCH_ENTRY_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2350",
    );
    assert_eq!(
        super::PURCHASED_AIR_CALC_HEATING_MODE_GUARD_ELSE_BRANCH_ENTRY_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2351",
    );
    assert_eq!(
        super::PURCHASED_AIR_CALC_HEATING_MODE_GUARD_ELSE_BRANCH_ENTRY_SOURCE_ORDER,
        &["enter-heating-mode-guard-else-branch-after-guard-false-fallthrough"],
    );
}

#[test]
fn exhaustive_61_routes_have_exact_two_entries_and_two_partition_accounting() {
    let predecessors = cp432_all_snapshots_for_successor_tests();
    assert_eq!(predecessors.len(), 61);
    let mut state = State::new(predecessors[0].system);
    let mut expected = [[0usize; 36]; 2];
    let mut public = 0usize;
    let mut public_entries = 0usize;
    let mut private_entries = 0usize;
    for predecessor in predecessors {
        let predecessor_route = predecessor_route_for(predecessor);
        let route = route_for(predecessor);
        let snapshot = advance_validated(&mut state, predecessor, predecessor_route, route)
            .expect("CP433");
        expected[0][route.logical_index] += 1;
        if route.entered {
            expected[1][route.logical_index] += 1;
        }
        let is_public = is_public_logical_index(route.logical_index)
            && !route.predecessor_single_cool_blocked;
        public += usize::from(is_public);
        public_entries += usize::from(is_public && route.entered);
        private_entries += usize::from(!is_public && route.entered);
        assert_eq!(route.entered, predecessor.heating_mode_guard_false_fallthrough);
        assert!(!(route.entered && route.assignment_executed));
        assert_eq!(snapshot.heating_mode_guard_else_branch_entered, route.entered);
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
        assert!(super::heating_mode_guard_else_branch_entry_snapshot_is_exact(snapshot));
    }
    assert_eq!((public, 61 - public), (20, 41));
    assert_eq!((public_entries, private_entries), (1, 1));
    assert_eq!(state.transition_count, 61);
    assert_eq!(state.inactive_transition_count, 59);
    assert_eq!(state.heating_mode_guard_else_branch_entry_count, 2);
    assert_eq!(state.source_site_execution_count, 2);
    assert_eq!(state.predecessor_route_counts, expected[0]);
    assert_eq!(state.heating_mode_guard_else_branch_entry_route_counts, expected[1]);
    assert_eq!(expected[0][1], 3);
    assert_eq!(expected[1][1], 2);
    assert_eq!(state.cp432_supply_humidity_ratio_state_owner_count, 37);
    assert_eq!(state.unchanged_supply_humidity_ratio_preservation_count, 37);
    assert_eq!(state.cp432_supply_enthalpy_state_owner_count, 42);
    assert_eq!(state.unchanged_supply_enthalpy_preservation_count, 42);
    assert_eq!(state.cp432_supply_temperature_state_owner_count, 57);
    assert_eq!(state.unchanged_supply_temperature_preservation_count, 57);
    assert!(super::release::state_counts_are_consistent_for_test(&state));
}

#[test]
fn cp433_new_state_has_exactly_two_zeroed_lossless_route_partitions() {
    let state = State::new(cp432_all_snapshots_for_successor_tests()[0].system);
    let arrays = [
        state.predecessor_route_counts,
        state.heating_mode_guard_else_branch_entry_route_counts,
    ];
    assert_eq!(arrays.len(), 2);
    assert!(arrays.into_iter().flatten().all(|count| count == 0));
    assert_eq!(state.source_site_execution_count, 0);
    assert!(state.latest.is_none());
}

#[test]
fn every_cp433_route_component_forgery_is_transactional() {
    let predecessor = cp432_all_snapshots_for_successor_tests()
        .into_iter()
        .find(|snapshot| snapshot.heating_mode_guard_false_fallthrough)
        .expect("entry predecessor");
    let predecessor_route = predecessor_route_for(predecessor);
    let exact = route_for(predecessor);
    for component in 0..13 {
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
fn every_supplied_cp432_route_component_forgery_is_transactional() {
    let predecessor = cp432_all_snapshots_for_successor_tests()
        .into_iter()
        .find(|snapshot| snapshot.heating_mode_guard_false_fallthrough)
        .expect("entry predecessor");
    let exact_predecessor_route = predecessor_route_for(predecessor);
    let exact = route_for(predecessor);
    for component in 0..12 {
        let mut forged_predecessor_route = exact_predecessor_route;
        flip_predecessor_route_component(&mut forged_predecessor_route, component);
        let mut state = State::new(predecessor.system);
        let before = state.clone();
        assert!(
            advance_validated(
                &mut state,
                predecessor,
                forged_predecessor_route,
                exact,
            )
            .is_none(),
            "CP432 component {component}",
        );
        assert_eq!(state, before, "CP432 component {component}");
    }
}

#[test]
fn active_cp433_counter_and_route_overflows_are_transactional() {
    let predecessors = cp432_all_snapshots_for_successor_tests();
    let entry = predecessors
        .iter()
        .copied()
        .find(|snapshot| route_for(*snapshot).entered)
        .expect("entry");
    let inactive = predecessors
        .iter()
        .copied()
        .find(|snapshot| !route_for(*snapshot).entered)
        .expect("inactive");
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
            2 => state.heating_mode_guard_else_branch_entry_count = usize::MAX,
            3 => state.heating_mode_guard_else_branch_entry_route_counts[route.logical_index] = usize::MAX,
            4 => state.source_site_execution_count = usize::MAX,
            _ => state.inactive_transition_count = usize::MAX,
        }
        let before = state.clone();
        assert!(
            advance_validated(&mut state, predecessor, predecessor_route, route).is_none()
        );
        assert_eq!(state, before, "overflow {slot}");
    }

    let owned = predecessors
        .iter()
        .copied()
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
            0 => state.cp432_supply_humidity_ratio_state_owner_count = usize::MAX,
            1 => state.unchanged_supply_humidity_ratio_preservation_count = usize::MAX,
            2 => state.cp432_supply_enthalpy_state_owner_count = usize::MAX,
            3 => state.unchanged_supply_enthalpy_preservation_count = usize::MAX,
            4 => state.cp432_supply_temperature_state_owner_count = usize::MAX,
            _ => state.unchanged_supply_temperature_preservation_count = usize::MAX,
        }
        let before = state.clone();
        assert!(advance_validated(&mut state, owned, predecessor_route, route).is_none());
        assert_eq!(state, before, "W/H/T overflow {slot}");
    }
}

#[test]
fn cp433_hot_release_and_retained_validation_are_bounded() {
    let release = include_str!("release.rs");
    let runtime = include_str!("release/runtime_validation.rs");
    let hot = release
        .split_once("pub fn advance_direct_no_oa_calc_heating_mode_guard_else_branch_entry")
        .and_then(|(_, tail)| tail.split_once("#[allow(dead_code)]"))
        .map(|(hot, _)| hot)
        .expect("CP433 hot release");
    for source in [hot, runtime] {
        for forbidden in [
            "heating_operating_mode_heat_assignment_snapshot_route(",
            "private_characterization",
            "DirectZonePurchasedAirCouplingInput",
            "numerical_dto",
        ] {
            assert!(!source.contains(forbidden), "{forbidden}");
        }
    }
    assert_eq!(
        hot
            .matches("heating_operating_mode_heat_assignment_committed_latest_route(")
            .count(),
        1,
    );
}

#[test]
fn cp433_subtree_is_thirteen_files_and_each_core_file_is_bounded() {
    let files = [
        include_str!("../heating_mode_guard_else_branch_entry.rs"),
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
        include_str!("release/committed.rs"),
        include_str!("tests/committed_seal.rs"),
    ];
    assert_eq!(files.len() - 1, 13);
    assert!(files.into_iter().all(|source| source.lines().count() <= 500));
}

fn route_for(predecessor: Predecessor) -> Route {
    let predecessor_route = predecessor_route_for(predecessor);
    successor_route(predecessor, predecessor_route).expect("CP433 route")
}

fn predecessor_route_for(predecessor: Predecessor) -> PredecessorRoute {
    predecessor_route(predecessor).expect("CP432 route")
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
        11 => route.assignment_executed ^= true,
        _ => route.entered ^= true,
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
        _ => route.assignment_executed ^= true,
    }
}

fn is_public_logical_index(index: usize) -> bool {
    matches!(index, 0..=8 | 20 | 21 | 26 | 27)
}

fn assert_bits(left: Option<f64>, right: Option<f64>) {
    assert_eq!(left.map(f64::to_bits), right.map(f64::to_bits));
}

pub(in crate::ideal_loads::calc) fn cp433_all_snapshots_for_successor_tests() -> Vec<
    super::PurchasedAirCalcHeatingModeGuardElseBranchEntrySnapshot,
> {
    cp432_all_snapshots_for_successor_tests()
        .into_iter()
        .map(|predecessor| {
            let predecessor_route = predecessor_route_for(predecessor);
            let route = successor_route(predecessor, predecessor_route).expect("CP433 route");
            advance_validated(
                &mut State::new(predecessor.system),
                predecessor,
                predecessor_route,
                route,
            )
            .expect("CP433")
        })
        .collect()
}

pub(in crate::ideal_loads::calc) fn cp433_fixture_unit_for_successor_tests() -> (
    PurchasedAirUnitRuntimeState,
    super::PurchasedAirCalcHeatingModeGuardElseBranchEntrySnapshot,
    Route,
    Option<Cp329Snapshot>,
) {
    let (mut unit, predecessor, predecessor_route, owner) =
        cp432_fixture_unit_for_successor_tests();
    let route = successor_route(predecessor, predecessor_route).expect("CP433 route");
    let mut state = State::new(predecessor.system);
    let snapshot = advance_validated(&mut state, predecessor, predecessor_route, route)
        .expect("CP433");
    unit.calc_heating_mode_guard_else_branch_entry = state;
    (unit, snapshot, route, owner)
}
