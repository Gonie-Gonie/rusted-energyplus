//! CP430 boundary, exhaustive route, forgery, preservation, and overflow tests.

mod schema_prefix;

use super::transition::{
    PurchasedAirCalcHeatingOrNoLoadCaseEntryRetainedRoute as Route,
    heating_or_no_load_case_entry_route_from_committed_predecessor as successor_route,
};
use super::{
    PurchasedAirCalcHeatingOrNoLoadCaseEntryRuntimeState as State,
    advance_heating_or_no_load_case_entry_state as advance,
    advance_heating_or_no_load_case_entry_state_with_validated_route as advance_validated,
};
use crate::ideal_loads::calc::{
    cooling_zero_supply_mass_flow_total_output_positive_zero_assignment_snapshot_route as cp429_route,
    cp429_all_snapshots_for_successor_tests,
};
use crate::ideal_loads::PurchasedAirCalcCoolingZeroSupplyMassFlowTotalOutputPositiveZeroAssignmentSnapshot as Predecessor;

#[test]
fn cp430_boundary_and_single_structural_site_are_exact() {
    assert_eq!(
        super::PURCHASED_AIR_CALC_HEATING_OR_NO_LOAD_CASE_ENTRY_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2347",
    );
    assert_eq!(
        super::PURCHASED_AIR_CALC_HEATING_OR_NO_LOAD_CASE_ENTRY_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2348",
    );
    assert_eq!(
        super::PURCHASED_AIR_CALC_HEATING_OR_NO_LOAD_CASE_ENTRY_SOURCE_ORDER,
        &["enter-heating-or-no-load-case-after-cooling-entry-gate-false-fallthrough"],
    );
    let source_context = [
        (2346usize, "// Heating or no-load operation"),
        (2347usize, "} else { // Heating or no-load case"),
        (2348usize, "if ((MinOASensOutput < QZnHeatSP) && ..."),
    ];
    assert_eq!(source_context[0].0, 2346);
    assert_eq!(source_context[1], (2347, "} else { // Heating or no-load case"));
    assert_eq!(source_context[2].0, 2348);
}

#[test]
fn exhaustive_59_routes_enter_only_index_one_and_preserve_every_route() {
    let predecessors = cp429_all_snapshots_for_successor_tests();
    assert_eq!(predecessors.len(), 59);
    let mut state = State::new(predecessors[0].system);
    let mut predecessor_counts = [0usize; 36];
    let mut entry_counts = [0usize; 36];
    for predecessor in predecessors {
        let predecessor_route = cp429_route(predecessor).expect("CP429 route");
        let route = successor_route(predecessor, predecessor_route).expect("CP430 route");
        let snapshot = advance_validated(&mut state, predecessor, route).expect("CP430");
        predecessor_counts[route.logical_index] += 1;
        entry_counts[route.logical_index] += usize::from(route.entered);
        assert_eq!(route.logical_index, predecessor_route.logical_index);
        assert_eq!(route.active, predecessor_route.active);
        assert_eq!(
            route.predecessor_assignment_executed,
            predecessor_route.predecessor_assignment_executed
        );
        assert_eq!(route.predecessor_entered, predecessor_route.predecessor_entered);
        assert_eq!(route.assignment_executed, predecessor_route.assignment_executed);
        assert_eq!(route.entered, route.logical_index == 1);
        assert_eq!(route.entered, predecessor.non_cooling_skipped);
        assert_eq!(snapshot.heating_or_no_load_case_entered, route.entered);
        if route.entered {
            assert!(!predecessor.unit_off_skipped);
            assert!(predecessor.assigned_cooling_total_output_w.is_none());
            assert!(predecessor.resulting_supply_humidity_ratio.is_none());
            assert!(predecessor.resulting_supply_enthalpy_j_per_kg.is_none());
            assert!(predecessor.resulting_supply_temperature_c.is_none());
            assert!(snapshot.assigned_cooling_total_output_w.is_none());
            assert!(snapshot.resulting_supply_humidity_ratio.is_none());
            assert!(snapshot.resulting_supply_enthalpy_j_per_kg.is_none());
            assert!(snapshot.resulting_supply_temperature_c.is_none());
        }
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
    }
    assert_eq!(state.transition_count, 59);
    assert_eq!(state.inactive_transition_count, 58);
    assert_eq!(state.heating_or_no_load_case_entry_count, 1);
    assert_eq!(state.source_site_execution_count, 1);
    assert_eq!(state.predecessor_route_counts, predecessor_counts);
    assert_eq!(state.heating_or_no_load_case_entry_route_counts, entry_counts);
    assert_eq!(nonzero_indices(&entry_counts), [1]);
    assert_eq!(state.cp429_supply_humidity_ratio_state_owner_count, 37);
    assert_eq!(state.unchanged_supply_humidity_ratio_preservation_count, 37);
    assert_eq!(state.cp429_supply_enthalpy_state_owner_count, 42);
    assert_eq!(state.unchanged_supply_enthalpy_preservation_count, 42);
    assert_eq!(state.cp429_supply_temperature_state_owner_count, 57);
    assert_eq!(state.unchanged_supply_temperature_preservation_count, 57);
    assert!(super::release::state_counts_are_consistent_for_test(&state));
}

#[test]
fn every_retained_route_component_forgery_is_transactional() {
    let predecessor = entry_predecessor();
    let route = route_for(predecessor);
    for component in 0..6 {
        let mut forged = route;
        match component {
            0 => forged.logical_index = 2,
            1 => forged.active ^= true,
            2 => forged.predecessor_assignment_executed ^= true,
            3 => forged.predecessor_entered ^= true,
            4 => forged.assignment_executed ^= true,
            _ => forged.entered = false,
        }
        let mut state = State::new(predecessor.system);
        let before = state.clone();
        assert!(advance_validated(&mut state, predecessor, forged).is_none());
        assert_eq!(state, before, "route component {component}");
    }
}

#[test]
fn marker_is_structural_and_exact_without_wht_mutation() {
    for predecessor in [entry_predecessor(), inactive_predecessor()] {
        let snapshot = advance(&mut State::new(predecessor.system), predecessor).expect("CP430");
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
        let mut forged = snapshot;
        forged.heating_or_no_load_case_entered ^= true;
        assert!(!super::heating_or_no_load_case_entry_snapshot_is_exact(forged));
        assert!(!super::heating_or_no_load_case_entry_snapshots_match_bit_exact(
            snapshot, forged,
        ));
    }
}

#[test]
fn every_incremented_counter_overflow_is_transactional() {
    let predecessor = entry_predecessor();
    let route = route_for(predecessor);
    for scalar in 0..4 {
        let mut state = State::new(predecessor.system);
        match scalar {
            0 => state.transition_count = usize::MAX,
            1 => state.heating_or_no_load_case_entry_count = usize::MAX,
            2 => state.source_site_execution_count = usize::MAX,
            _ => state.heating_or_no_load_case_entry_route_counts[route.logical_index] = usize::MAX,
        }
        let before = state.clone();
        assert!(advance_validated(&mut state, predecessor, route).is_none());
        assert_eq!(state, before, "entry scalar {scalar}");
    }
    let mut route_overflow = State::new(predecessor.system);
    route_overflow.predecessor_route_counts[route.logical_index] = usize::MAX;
    let before = route_overflow.clone();
    assert!(advance_validated(&mut route_overflow, predecessor, route).is_none());
    assert_eq!(route_overflow, before);

    let predecessor = all_owner_predecessor();
    let route = route_for(predecessor);
    for scalar in 0..7 {
        let mut state = State::new(predecessor.system);
        match scalar {
            0 => state.inactive_transition_count = usize::MAX,
            1 => state.cp429_supply_humidity_ratio_state_owner_count = usize::MAX,
            2 => state.unchanged_supply_humidity_ratio_preservation_count = usize::MAX,
            3 => state.cp429_supply_enthalpy_state_owner_count = usize::MAX,
            4 => state.unchanged_supply_enthalpy_preservation_count = usize::MAX,
            5 => state.cp429_supply_temperature_state_owner_count = usize::MAX,
            _ => state.unchanged_supply_temperature_preservation_count = usize::MAX,
        }
        let before = state.clone();
        assert!(advance_validated(&mut state, predecessor, route).is_none());
        assert_eq!(state, before, "inactive scalar {scalar}");
    }
}

#[test]
fn cp430_cp429_and_cp329_hot_paths_are_statically_bounded_and_lazy() {
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
        "snapshot_route(",
    ] {
        assert!(!hot.contains(forbidden), "{forbidden}");
    }
    assert_eq!(
        hot.matches("total_output_positive_zero_assignment_committed_latest_route(")
            .count(),
        1
    );
    let owner_guard = hot
        .find("if predecessor_cp429")
        .expect("lazy CP429 owner guard");
    let owner_call = hot
        .find("cooling_mixed_air_call_latest_witness")
        .expect("CP329 owner call");
    assert!(owner_call > owner_guard);
}

#[test]
fn cp430_pending_transition_advances_from_cp429_not_from_itself() {
    let source = include_str!("release/runtime_validation.rs");
    let start = source.find("fn pending_state_is_consistent").expect("pending start");
    let end = source[start..]
        .find("pub(super) fn post_transition_state_is_consistent")
        .map(|offset| start + offset)
        .expect("pending end");
    let pending = &source[start..end];
    assert_eq!(
        pending
            .matches("calc_cooling_zero_supply_mass_flow_total_output_positive_zero_assignment")
            .count(),
        1,
    );
    assert_eq!(pending.matches("calc_heating_or_no_load_case_entry").count(), 1);
}

fn entry_predecessor() -> Predecessor {
    cp429_all_snapshots_for_successor_tests()
        .into_iter()
        .find(|snapshot| snapshot.non_cooling_skipped)
        .expect("CP430 entry predecessor")
}

fn inactive_predecessor() -> Predecessor {
    cp429_all_snapshots_for_successor_tests()
        .into_iter()
        .find(|snapshot| !snapshot.non_cooling_skipped)
        .expect("CP430 inactive predecessor")
}

fn all_owner_predecessor() -> Predecessor {
    cp429_all_snapshots_for_successor_tests()
        .into_iter()
        .find(|snapshot| {
            !snapshot.non_cooling_skipped
                && snapshot.resulting_supply_humidity_ratio.is_some()
                && snapshot.resulting_supply_enthalpy_j_per_kg.is_some()
                && snapshot.resulting_supply_temperature_c.is_some()
        })
        .expect("CP430 inactive all-owner predecessor")
}

fn route_for(predecessor: Predecessor) -> Route {
    successor_route(
        predecessor,
        cp429_route(predecessor).expect("CP429 route"),
    )
    .expect("CP430 route")
}

fn assert_bits(left: Option<f64>, right: Option<f64>) {
    assert_eq!(left.map(f64::to_bits), right.map(f64::to_bits));
}

fn nonzero_indices(values: &[usize; 36]) -> Vec<usize> {
    values
        .iter()
        .enumerate()
        .filter_map(|(index, count)| (*count != 0).then_some(index))
        .collect()
}
