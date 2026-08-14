//! CP424 boundary, exhaustive route, forgery, preservation, and overflow tests.

mod schema_prefix;
mod committed_seal;

use super::transition::{
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardElseBranchEntryRetainedRoute as Route,
    cooling_supply_mass_flow_positive_guard_else_branch_entry_route_from_committed_predecessor as successor_route,
};
use super::{
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardElseBranchEntryRuntimeState as State,
    advance_cooling_supply_mass_flow_positive_guard_else_branch_entry_state as advance,
    advance_cooling_supply_mass_flow_positive_guard_else_branch_entry_state_with_validated_route as advance_validated,
};
use crate::ideal_loads::calc::{
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment_snapshot_route as cp423_route,
    cp423_all_snapshots_for_successor_tests,
};
use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputSupplyTemperatureAssignmentSnapshot as Predecessor;

#[test]
fn cp424_boundary_and_sole_site_are_exact() {
    assert_eq!(
        super::PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_ELSE_BRANCH_ENTRY_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2339",
    );
    assert_eq!(
        super::PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_ELSE_BRANCH_ENTRY_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2340",
    );
    assert_eq!(
        super::PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_ELSE_BRANCH_ENTRY_SOURCE_ORDER,
        &["enter-cooling-supply-mass-flow-positive-guard-else-branch-after-guard-false-fallthrough"],
    );
}

#[test]
fn exhaustive_59_routes_preserve_every_cp423_route_and_enter_only_index_two() {
    let predecessors = cp423_all_snapshots_for_successor_tests();
    assert_eq!(predecessors.len(), 59);
    let mut state = State::new(predecessors[0].system);
    let mut predecessor_counts = [0usize; 36];
    let mut entry_counts = [0usize; 36];
    let mut route_tuples = Vec::new();
    for predecessor in predecessors {
        let predecessor_route = cp423_route(predecessor).expect("CP423 route");
        let route = successor_route(predecessor, predecessor_route).expect("CP424 route");
        let snapshot = advance_validated(&mut state, predecessor, route).expect("CP424");
        predecessor_counts[route.logical_index] += 1;
        entry_counts[route.logical_index] += usize::from(route.entered);
        route_tuples.push((route.logical_index, route.active, route.assignment_executed, route.entered));
        assert_eq!(route.logical_index, predecessor_route.logical_index);
        assert_eq!(route.active, predecessor_route.active);
        assert_eq!(route.assignment_executed, predecessor_route.assignment_executed);
        assert_eq!(route.entered, route.logical_index == 2);
        assert_eq!(snapshot.cooling_supply_mass_flow_positive_guard_else_branch_entered, route.entered);
        assert_bits(snapshot.resulting_supply_humidity_ratio, predecessor.resulting_supply_humidity_ratio);
        assert_bits(snapshot.resulting_supply_enthalpy_j_per_kg, predecessor.resulting_supply_enthalpy_j_per_kg);
        assert_bits(snapshot.resulting_supply_temperature_c, predecessor.resulting_supply_temperature_c);
    }
    assert_eq!(route_tuples.len(), 59);
    assert_eq!(state.transition_count, 59);
    assert_eq!(state.inactive_transition_count, 58);
    assert_eq!(state.positive_supply_mass_flow_guard_else_branch_entry_count, 1);
    assert_eq!(state.source_site_execution_count, 1);
    assert_eq!(state.predecessor_route_counts, predecessor_counts);
    assert_eq!(state.positive_supply_mass_flow_guard_else_branch_entry_route_counts, entry_counts);
    assert_eq!(nonzero_indices(&entry_counts), [2]);
    assert_eq!(state.cp423_supply_humidity_ratio_state_owner_count, 36);
    assert_eq!(state.unchanged_supply_humidity_ratio_preservation_count, 36);
    assert_eq!(state.cp423_supply_enthalpy_state_owner_count, 41);
    assert_eq!(state.unchanged_supply_enthalpy_preservation_count, 41);
    assert_eq!(state.cp423_supply_temperature_state_owner_count, 56);
    assert_eq!(state.unchanged_supply_temperature_preservation_count, 56);
    assert!(super::release::state_counts_are_consistent_for_test(&state));
}

#[test]
fn every_route_component_forgery_is_transactional() {
    let predecessor = entry_predecessor();
    let route = route_for(predecessor);
    let mut forged = [route; 4];
    forged[0].logical_index = 3;
    forged[1].active = true;
    forged[2].assignment_executed = true;
    forged[3].entered = false;
    for route in forged {
        let mut state = State::new(predecessor.system);
        let before = state.clone();
        assert!(advance_validated(&mut state, predecessor, route).is_none());
        assert_eq!(state, before);
    }
}

#[test]
fn prefix_and_marker_forgery_is_rejected_and_raw_ieee_bits_are_preserved() {
    let predecessor = cp423_all_snapshots_for_successor_tests()
        .into_iter()
        .find(|snapshot| snapshot.resulting_supply_temperature_c.is_some())
        .expect("numeric predecessor");
    let snapshot = advance(&mut State::new(predecessor.system), predecessor).expect("CP424");
    assert!(super::cooling_supply_mass_flow_positive_guard_else_branch_entry_snapshot_is_exact(snapshot));
    let mut marker = snapshot;
    marker.cooling_supply_mass_flow_positive_guard_else_branch_entered ^= true;
    assert!(!super::cooling_supply_mass_flow_positive_guard_else_branch_entry_snapshot_is_exact(marker));
    let mut bits = snapshot;
    bits.resulting_supply_temperature_c = bits.resulting_supply_temperature_c.map(flip);
    assert!(!super::cooling_supply_mass_flow_positive_guard_else_branch_entry_snapshot_is_exact(bits));
    assert_bits(snapshot.resulting_supply_temperature_c, predecessor.resulting_supply_temperature_c);
}

#[test]
fn every_incremented_counter_overflow_is_transactional() {
    let entry = entry_predecessor();
    let entry_route = route_for(entry);
    let mut states = Vec::new();
    macro_rules! scalar {
        ($field:ident) => {{
            let mut state = State::new(entry.system);
            state.$field = usize::MAX;
            states.push(state);
        }};
    }
    scalar!(transition_count);
    scalar!(positive_supply_mass_flow_guard_else_branch_entry_count);
    scalar!(source_site_execution_count);
    let mut route_count = State::new(entry.system);
    route_count.predecessor_route_counts[entry_route.logical_index] = usize::MAX;
    states.push(route_count);
    let mut entry_count = State::new(entry.system);
    entry_count.positive_supply_mass_flow_guard_else_branch_entry_route_counts[entry_route.logical_index] = usize::MAX;
    states.push(entry_count);
    for mut state in states {
        let before = state.clone();
        assert!(advance_validated(&mut state, entry, entry_route).is_none());
        assert_eq!(state, before);
    }

    let inactive = cp423_all_snapshots_for_successor_tests()
        .into_iter()
        .find(|snapshot| {
            let route = route_for(*snapshot);
            !route.entered
                && snapshot.resulting_supply_humidity_ratio.is_some()
                && snapshot.resulting_supply_enthalpy_j_per_kg.is_some()
                && snapshot.resulting_supply_temperature_c.is_some()
        })
        .expect("inactive all-owner route");
    let inactive_route = route_for(inactive);
    let mut inactive_fields = Vec::new();
    macro_rules! inactive_scalar {
        ($field:ident) => {{
            let mut state = State::new(inactive.system);
            state.$field = usize::MAX;
            inactive_fields.push(state);
        }};
    }
    inactive_scalar!(inactive_transition_count);
    inactive_scalar!(cp423_supply_humidity_ratio_state_owner_count);
    inactive_scalar!(unchanged_supply_humidity_ratio_preservation_count);
    inactive_scalar!(cp423_supply_enthalpy_state_owner_count);
    inactive_scalar!(unchanged_supply_enthalpy_preservation_count);
    inactive_scalar!(cp423_supply_temperature_state_owner_count);
    inactive_scalar!(unchanged_supply_temperature_preservation_count);
    for mut state in inactive_fields {
        let before = state.clone();
        assert!(advance_validated(&mut state, inactive, inactive_route).is_none());
        assert_eq!(state, before);
    }
}

#[test]
fn cp424_and_cp423_seal_hot_paths_are_statically_bounded() {
    let release = include_str!("release.rs");
    let start = release.find("pub fn advance_direct_no_oa_calc_").expect("hot start");
    let end = release[start..].find("#[allow(dead_code)]").map(|offset| start + offset).expect("hot end");
    let hot = &release[start..end];
    for forbidden in ["completed_", "snapshot_is_exact", "private_characterization", "snapshot_route("] {
        assert!(!hot.contains(forbidden), "{forbidden}");
    }
    assert_eq!(
        hot.matches("supply_temperature_assignment_committed_latest_route(").count(),
        1,
    );
    let seal = include_str!("../cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment/release/committed.rs");
    let seal = seal.split("#[cfg(test)]").next().expect("sealed hot path");
    assert!(!seal.contains("snapshot_is_exact"));
    assert!(!seal.contains("predecessor_route("));
}

fn entry_predecessor() -> Predecessor {
    cp423_all_snapshots_for_successor_tests()
        .into_iter()
        .find(|snapshot| snapshot.positive_guard_false_fallthrough_skipped)
        .expect("CP424 entry predecessor")
}

pub(super) fn route_for(predecessor: Predecessor) -> Route {
    successor_route(predecessor, cp423_route(predecessor).expect("CP423 route")).expect("CP424 route")
}

fn assert_bits(left: Option<f64>, right: Option<f64>) {
    assert_eq!(left.map(f64::to_bits), right.map(f64::to_bits));
}

fn flip(value: f64) -> f64 {
    f64::from_bits(value.to_bits() ^ 1)
}

fn nonzero_indices(values: &[usize; 36]) -> Vec<usize> {
    values.iter().enumerate().filter_map(|(index, count)| (*count != 0).then_some(index)).collect()
}

pub(in crate::ideal_loads::calc) fn cp424_all_snapshots_for_successor_tests(
) -> Vec<super::PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardElseBranchEntrySnapshot> {
    cp423_all_snapshots_for_successor_tests()
        .into_iter()
        .map(|predecessor| {
            advance(&mut State::new(predecessor.system), predecessor).expect("CP424 fixture")
        })
        .collect()
}
