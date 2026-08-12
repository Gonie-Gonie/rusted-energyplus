//! CP420 boundary, exhaustive-route, IEEE, and hot-path tests.

mod overflow;
mod schema_routes;

use super::transition::predecessor_route;
use super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputAssignmentActiveInput as ActiveInput,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputAssignmentRuntimeState as State,
    advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_state as advance,
};
use crate::ideal_loads::calc::cp419_all_snapshots_for_successor_tests;

#[test]
fn cp420_boundary_and_eight_sites_are_exact() {
    assert_eq!(
        super::PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2331",
    );
    assert_eq!(
        super::PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2332",
    );
    assert_eq!(
        super::PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER.len(),
        8,
    );
}

#[test]
fn exhaustive_54_outcomes_49_inactive_five_assignments_and_ten_arrays_are_exact() {
    let predecessors = cp419_all_snapshots_for_successor_tests();
    let system = predecessors[0].system;
    let mut state = State::new(system);
    let mut active_indices = Vec::new();
    let mut predecessor_routes = [0usize; 36];
    let mut guard_false = [0usize; 36];
    let mut guard_body = [0usize; 36];
    let mut saturation_assignment = [0usize; 36];
    let mut mixed_air_limit = [0usize; 36];
    let mut humidity_assignment = [0usize; 36];
    let mut enthalpy_assignment = [0usize; 36];
    let mut else_entry = [0usize; 36];
    for (conceptual_index, predecessor) in predecessors.into_iter().enumerate() {
        let active = predecessor
            .post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_executed;
        let route = predecessor_route(predecessor).expect("CP420 route");
        let input = active.then_some(ActiveInput {
            supply_mass_flow_rate_kg_per_s: 0.25,
            mixed_air_temperature_c: 17.0,
        });
        let snapshot = advance(&mut state, predecessor, input).expect("CP420 transition");
        let index = route.logical_index;
        predecessor_routes[index] += 1;
        guard_false[index] += usize::from(route.predecessor_guard_false_fallthrough);
        guard_body[index] += usize::from(route.predecessor_guard_body_entered);
        saturation_assignment[index] +=
            usize::from(route.predecessor_saturation_temperature_assignment_executed);
        mixed_air_limit[index] +=
            usize::from(route.predecessor_saturation_temperature_mixed_air_limit_executed);
        humidity_assignment[index] +=
            usize::from(route.predecessor_supply_humidity_ratio_assignment_executed);
        enthalpy_assignment[index] +=
            usize::from(route.predecessor_supply_enthalpy_assignment_executed);
        else_entry[index] += usize::from(route.active);
        if active {
            active_indices.push(conceptual_index);
            assert_eq!(route.logical_index, conceptual_index);
            assert_formula_bits(snapshot);
        } else {
            assert!(snapshot.cooling_sensible_output_w.is_none());
        }
    }
    assert_eq!(active_indices, [4, 7, 10, 13, 16]);
    assert_eq!(state.transition_count, 54);
    assert_eq!(state.inactive_transition_count, 49);
    assert_eq!(
        state.dehumidification_guard_else_branch_sensible_output_assignment_count,
        5,
    );
    assert_eq!(state.source_site_execution_count, 40);
    assert_eq!(state.cp419_supply_humidity_ratio_state_owner_count, 36);
    assert_eq!(state.cp419_supply_enthalpy_state_owner_count, 41);
    assert_eq!(state.cp419_supply_temperature_state_owner_count, 51);
    assert_eq!(state.predecessor_route_counts, predecessor_routes);
    assert_eq!(
        state.predecessor_guard_false_fallthrough_route_counts,
        guard_false
    );
    assert_eq!(state.predecessor_guard_body_entry_route_counts, guard_body);
    assert_eq!(
        state.predecessor_supply_temperature_saturation_assignment_route_counts,
        saturation_assignment,
    );
    assert_eq!(
        state.predecessor_supply_temperature_mixed_air_limit_route_counts,
        mixed_air_limit,
    );
    assert_eq!(
        state.predecessor_supply_humidity_ratio_assignment_route_counts,
        humidity_assignment,
    );
    assert_eq!(
        state.predecessor_supply_enthalpy_assignment_route_counts,
        enthalpy_assignment,
    );
    assert_eq!(
        state.predecessor_dehumidification_guard_else_branch_entry_route_counts,
        else_entry,
    );
    assert_eq!(
        state.predecessor_dehumidification_guard_else_branch_cp_air_assignment_route_counts,
        else_entry,
    );
    assert_eq!(
        state.dehumidification_guard_else_branch_sensible_output_assignment_route_counts,
        else_entry,
    );
    assert_eq!(
        nonzero_indices(
            &state.dehumidification_guard_else_branch_sensible_output_assignment_route_counts
        ),
        [4, 7, 10, 13, 16],
    );
    assert_eq!(
        state.predecessor_dehumidification_guard_else_branch_cp_air_assignment_route_counts,
        state.dehumidification_guard_else_branch_sensible_output_assignment_route_counts,
    );
    for count in [
        state.supply_mass_flow_rate_owned_read_count,
        state.supply_mass_flow_rate_bit_corroboration_count,
        state.supply_mass_flow_rate_read_count,
        state.cp_air_owned_read_count,
        state.cp_air_read_count,
        state.supply_mass_flow_rate_times_cp_air_calculation_count,
        state.mixed_air_temperature_owned_read_count,
        state.mixed_air_temperature_read_count,
        state.supply_temperature_owned_read_count,
        state.supply_temperature_read_count,
        state.mixed_air_minus_supply_temperature_calculation_count,
        state.cooling_sensible_output_calculation_count,
        state.cooling_sensible_output_assignment_write_count,
    ] {
        assert_eq!(count, 5);
    }
}

#[test]
fn release_hot_path_uses_only_committed_owners_and_validated_route() {
    let source = include_str!("release.rs");
    let hot = source
        .split("/// Executes CP420")
        .nth(1)
        .expect("release")
        .split("#[allow(dead_code)]")
        .next()
        .expect("hot function");
    for required in [
        "cp_air_assignment_committed_latest_route",
        "committed_latest_sensible_output_inputs",
        "positive_guard_committed_latest_supply_mass_flow_rate",
        "advance_with_validated_route",
    ] {
        assert!(hot.contains(required), "missing {required}");
    }
    assert!(!hot.contains("snapshot_is_exact"));
    assert!(!hot.contains("predecessor_route("));
    assert!(!hot.contains("completed_direct"));
    assert!(!hot.contains("private_"));
}

fn assert_formula_bits(
    snapshot: crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputAssignmentSnapshot,
) {
    let flow = snapshot.supply_mass_flow_rate_kg_per_s.expect("flow");
    let cp_air = snapshot
        .cp419_cp_air_for_sensible_output_j_per_kg_k
        .expect("CpAir");
    let mixed = snapshot
        .mixed_air_temperature_for_sensible_output_c
        .expect("mixed T");
    let supply = snapshot
        .supply_temperature_for_sensible_output_c
        .expect("supply T");
    let first = flow * cp_air;
    let difference = mixed - supply;
    let output = first * difference;
    assert_eq!(
        snapshot
            .supply_mass_flow_rate_times_cp_air_w_per_k
            .expect("first")
            .to_bits(),
        first.to_bits(),
    );
    assert_eq!(
        snapshot
            .mixed_air_minus_supply_temperature_k
            .expect("difference")
            .to_bits(),
        difference.to_bits(),
    );
    assert_eq!(
        snapshot
            .cooling_sensible_output_w
            .expect("output")
            .to_bits(),
        output.to_bits(),
    );
}

fn nonzero_indices(values: &[usize; 36]) -> Vec<usize> {
    values
        .iter()
        .enumerate()
        .filter_map(|(index, count)| (*count != 0).then_some(index))
        .collect()
}
