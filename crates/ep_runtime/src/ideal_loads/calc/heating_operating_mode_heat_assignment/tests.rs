//! CP432 boundary, exhaustive route, forgery, overflow, and bounded-path tests.

mod committed_seal;
mod schema_prefix;

use super::transition::{
    PurchasedAirCalcHeatingOperatingModeHeatAssignmentRetainedRoute as Route,
    advance_heating_operating_mode_heat_assignment_state as advance,
    advance_heating_operating_mode_heat_assignment_state_with_validated_route as advance_validated,
    heating_operating_mode_heat_assignment_route_from_committed_predecessor as successor_route,
};
use super::PurchasedAirCalcHeatingOperatingModeHeatAssignmentRuntimeState as State;
use crate::ideal_loads::calc::{
    IdealLoadsSensibleMode,
    cp431_all_snapshots_for_successor_tests,
    cp431_committed_fixture_for_successor_tests,
    heating_mode_guard_snapshot_route,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingMixedAirCallSnapshot as Cp329Snapshot,
    PurchasedAirCalcHeatingModeGuardSnapshot as Predecessor,
    PurchasedAirCalcHeatingOperatingModeHeatAssignmentSnapshot as Snapshot,
    PurchasedAirUnitRuntimeState,
};

#[test]
fn cp432_boundary_skips_structural_2350_and_constant_heat_site_is_exact() {
    assert_eq!(
        super::PURCHASED_AIR_CALC_HEATING_OPERATING_MODE_HEAT_ASSIGNMENT_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2349",
    );
    let structural_only = "EnergyPlus 26.1 PurchasedAirManager.cc:2350";
    assert_ne!(
        super::PURCHASED_AIR_CALC_HEATING_OPERATING_MODE_HEAT_ASSIGNMENT_SOURCE,
        structural_only,
    );
    assert_ne!(
        super::PURCHASED_AIR_CALC_HEATING_OPERATING_MODE_HEAT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        structural_only,
    );
    assert_eq!(
        super::PURCHASED_AIR_CALC_HEATING_OPERATING_MODE_HEAT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2351",
    );
    assert_eq!(
        super::PURCHASED_AIR_CALC_HEATING_OPERATING_MODE_HEAT_ASSIGNMENT_SOURCE_ORDER,
        &["assign-local-operating-mode-heat"],
    );
}

#[test]
fn exhaustive_61_routes_have_exact_assignment_owner_and_four_partition_accounting() {
    let predecessors = cp431_all_snapshots_for_successor_tests();
    assert_eq!(predecessors.len(), 61);
    let mut state = State::new(predecessors[0].system);
    let mut expected = [[0usize; 36]; 4];
    let mut public = 0usize;
    for predecessor in predecessors {
        let predecessor_route = heating_mode_guard_snapshot_route(predecessor).expect("CP431 route");
        let route = successor_route(predecessor, predecessor_route).expect("CP432 route");
        let snapshot = advance_validated(&mut state, predecessor, predecessor_route, route)
            .expect("CP432");
        expected[0][route.logical_index] += 1;
        if route.predecessor_heating_mode_guard_evaluated {
            expected[1][route.logical_index] += 1;
        }
        if route.predecessor_heating_mode_guard_false_fallthrough {
            expected[2][route.logical_index] += 1;
        }
        if route.assignment_executed {
            expected[3][route.logical_index] += 1;
        }
        if is_public_logical_index(route.logical_index) && !route.predecessor_single_cool_blocked {
            public += 1;
        }
        assert_eq!(route.assignment_executed, predecessor.heating_operating_mode_body_entered);
        assert_eq!(
            snapshot.assigned_heating_operating_mode,
            route.assignment_executed.then_some(IdealLoadsSensibleMode::Heating),
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
        assert!(super::heating_operating_mode_heat_assignment_snapshot_is_exact(snapshot));
    }
    assert_eq!((public, 61 - public), (20, 41));
    assert_eq!(state.transition_count, 61);
    assert_eq!(state.inactive_transition_count, 58);
    assert_eq!(state.predecessor_heating_mode_guard_evaluation_count, 3);
    assert_eq!(state.predecessor_heating_mode_guard_false_fallthrough_count, 2);
    assert_eq!(state.heating_operating_mode_heat_assignment_count, 1);
    assert_eq!(state.source_site_execution_count, 1);
    assert_eq!(state.predecessor_route_counts, expected[0]);
    assert_eq!(state.predecessor_heating_mode_guard_evaluation_route_counts, expected[1]);
    assert_eq!(state.predecessor_heating_mode_guard_false_fallthrough_route_counts, expected[2]);
    assert_eq!(state.heating_operating_mode_heat_assignment_route_counts, expected[3]);
    assert_eq!(state.cp431_supply_humidity_ratio_state_owner_count, 37);
    assert_eq!(state.unchanged_supply_humidity_ratio_preservation_count, 37);
    assert_eq!(state.cp431_supply_enthalpy_state_owner_count, 42);
    assert_eq!(state.unchanged_supply_enthalpy_preservation_count, 42);
    assert_eq!(state.cp431_supply_temperature_state_owner_count, 57);
    assert_eq!(state.unchanged_supply_temperature_preservation_count, 57);
    assert_eq!(state.cp432_heating_operating_mode_state_owner_count, 1);
    assert_eq!(state.heating_operating_mode_assignment_write_count, 1);
    assert!(super::release::state_counts_are_consistent_for_test(&state));
}

#[test]
fn cp432_new_state_has_exactly_four_zeroed_lossless_route_partitions() {
    let predecessor = assignment_predecessor();
    let state = State::new(predecessor.system);
    let arrays = [
        state.predecessor_route_counts,
        state.predecessor_heating_mode_guard_evaluation_route_counts,
        state.predecessor_heating_mode_guard_false_fallthrough_route_counts,
        state.heating_operating_mode_heat_assignment_route_counts,
    ];
    assert_eq!(arrays.len(), 4);
    assert!(arrays.into_iter().flatten().all(|count| count == 0));
}

#[test]
fn every_cp432_route_component_forgery_is_transactional() {
    let predecessor = assignment_predecessor();
    let predecessor_route = heating_mode_guard_snapshot_route(predecessor).expect("CP431 route");
    let route = successor_route(predecessor, predecessor_route).expect("CP432 route");
    for component in 0..12 {
        let mut forged = route;
        forge_route_component(&mut forged, component);
        let mut state = State::new(predecessor.system);
        let before = state.clone();
        assert!(advance_validated(&mut state, predecessor, predecessor_route, forged).is_none());
        assert_eq!(state, before, "component {component}");
    }
}

#[test]
fn active_cp432_counter_and_route_overflows_are_transactional() {
    let predecessor = assignment_predecessor();
    let predecessor_route = heating_mode_guard_snapshot_route(predecessor).expect("CP431 route");
    let route = successor_route(predecessor, predecessor_route).expect("CP432 route");
    for counter in 0..9 {
        let mut state = State::new(predecessor.system);
        match counter {
            0 => state.transition_count = usize::MAX,
            1 => state.predecessor_route_counts[1] = usize::MAX,
            2 => state.predecessor_heating_mode_guard_evaluation_count = usize::MAX,
            3 => state.predecessor_heating_mode_guard_evaluation_route_counts[1] = usize::MAX,
            4 => state.heating_operating_mode_heat_assignment_count = usize::MAX,
            5 => state.heating_operating_mode_heat_assignment_route_counts[1] = usize::MAX,
            6 => state.source_site_execution_count = usize::MAX,
            7 => state.cp432_heating_operating_mode_state_owner_count = usize::MAX,
            _ => state.heating_operating_mode_assignment_write_count = usize::MAX,
        }
        let before = state.clone();
        assert!(advance_validated(&mut state, predecessor, predecessor_route, route).is_none());
        assert_eq!(state, before, "counter {counter}");
    }
}

#[test]
fn cp432_hot_release_and_retained_validation_are_bounded() {
    let release = include_str!("release.rs");
    let start = release.find("pub fn advance_direct_no_oa_calc_").expect("hot start");
    let end = release[start..]
        .find("#[allow(dead_code)]")
        .map(|offset| start + offset)
        .expect("hot end");
    let hot = &release[start..end];
    for forbidden in [
        "completed_",
        "snapshot_is_exact",
        "private_characterization",
        "predecessor_route(",
        "_snapshot_route(",
    ] {
        assert!(!hot.contains(forbidden), "{forbidden}");
    }
    assert_eq!(hot.matches("heating_mode_guard_committed_latest_route(").count(), 1);
    assert_no_recursive_route_replay(include_str!("release/runtime_validation.rs"));
}

#[test]
fn cp432_subtree_is_eleven_files_and_each_core_file_is_bounded() {
    let files = [
        include_str!("../heating_operating_mode_heat_assignment.rs"),
        include_str!("release.rs"),
        include_str!("release/error.rs"),
        include_str!("release/prefix.rs"),
        include_str!("release/runtime_validation.rs"),
        include_str!("release/snapshot_validation.rs"),
        include_str!("state.rs"),
        include_str!("tests.rs"),
        include_str!("tests/schema_prefix.rs"),
        include_str!("transition.rs"),
        include_str!("transition/accounting.rs"),
        include_str!("transition/snapshot.rs"),
    ];
    assert_eq!(files.len() - 1, 11);
    assert!(files.into_iter().all(|source| source.lines().count() <= 500));
}

fn assignment_predecessor() -> Predecessor {
    cp431_all_snapshots_for_successor_tests()
        .into_iter()
        .find(|snapshot| snapshot.heating_operating_mode_body_entered)
        .expect("CP431 body route")
}

fn forge_route_component(route: &mut Route, component: usize) {
    match component {
        0 => route.logical_index = 2,
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

fn assert_no_recursive_route_replay(source: &str) {
    for forbidden in ["predecessor_route(", "_snapshot_route("] {
        assert!(!source.contains(forbidden), "{forbidden}");
    }
}

pub(in crate::ideal_loads::calc) fn cp432_all_snapshots_for_successor_tests() -> Vec<Snapshot> {
    cp431_all_snapshots_for_successor_tests()
        .into_iter()
        .map(|predecessor| {
            let predecessor_route =
                heating_mode_guard_snapshot_route(predecessor).expect("CP431 route");
            let route = successor_route(predecessor, predecessor_route).expect("CP432 route");
            advance_validated(
                &mut State::new(predecessor.system),
                predecessor,
                predecessor_route,
                route,
            )
            .expect("CP432")
        })
        .collect()
}

pub(in crate::ideal_loads::calc) fn cp432_fixture_unit_for_successor_tests() -> (
    PurchasedAirUnitRuntimeState,
    Snapshot,
    Route,
    Option<Cp329Snapshot>,
) {
    let (mut unit, predecessor, predecessor_route, owner) =
        cp431_committed_fixture_for_successor_tests();
    let route = successor_route(predecessor, predecessor_route).expect("CP432 route");
    let mut state = State::new(predecessor.system);
    let snapshot = advance_validated(
        &mut state,
        predecessor,
        predecessor_route,
        route,
    )
    .expect("CP432");
    unit.calc_heating_operating_mode_heat_assignment = state;
    (unit, snapshot, route, owner)
}
