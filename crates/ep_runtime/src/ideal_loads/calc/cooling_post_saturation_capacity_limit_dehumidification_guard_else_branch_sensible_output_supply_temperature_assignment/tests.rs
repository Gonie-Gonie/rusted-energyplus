//! CP423 boundary, exhaustive route, ownership, forgery, and hot-path tests.

mod overflow;
mod schema_ieee;
mod committed_seal;
pub(in crate::ideal_loads::calc) use committed_seal::cp423_fixture_unit_for_successor_tests;

use super::transition::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputSupplyTemperatureAssignmentRetainedRoute as Route,
    calculate_supply_temperature,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment_route_from_committed_predecessor as successor_route,
};
use super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputSupplyTemperatureAssignmentActiveInput as ActiveInput,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputSupplyTemperatureAssignmentRuntimeState as State,
    advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment_state as advance,
    advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment_state_with_validated_route as advance_validated,
};
use crate::ideal_loads::calc::{
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_snapshot_route as cp422_route,
    cp422_all_snapshots_for_successor_tests,
};
use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputMaximumCapacityAssignmentSnapshot as Predecessor;

#[test]
fn cp423_boundary_and_eight_sites_are_exact() {
    assert_eq!(
        super::PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2334",
    );
    assert_eq!(
        super::PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2340",
    );
    assert_eq!(
        super::PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER.len(),
        8,
    );
    assert_eq!(
        super::PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER[7],
        "assign-purchased-air-supply-temperature-for-post-saturation-capacity-limit-dehumidification-guard-else-branch-sensible-output-case",
    );
}

#[test]
fn exhaustive_59_routes_have_exact_assignment_owner_and_preservation_accounting() {
    let predecessors = cp422_all_snapshots_for_successor_tests();
    assert_eq!(predecessors.len(), 59);
    let mut state = State::new(predecessors[0].system);
    let mut routes = [0usize; 36];
    let mut false_routes = [0usize; 36];
    let mut assignment_routes = [0usize; 36];
    for predecessor in predecessors {
        let route = successor_route_for(predecessor);
        let snapshot = advance_validated(&mut state, predecessor, route, active_input(predecessor))
            .expect("CP423");
        routes[route.logical_index] += 1;
        false_routes[route.logical_index] += usize::from(route.active && !route.assignment_executed);
        assignment_routes[route.logical_index] += usize::from(route.assignment_executed);
        assert_eq!(
            snapshot.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment_executed,
            route.assignment_executed,
        );
        assert_eq!(
            snapshot.resulting_supply_humidity_ratio.map(f64::to_bits),
            predecessor.resulting_supply_humidity_ratio.map(f64::to_bits),
        );
        assert_eq!(
            snapshot.resulting_supply_enthalpy_j_per_kg.map(f64::to_bits),
            predecessor.resulting_supply_enthalpy_j_per_kg.map(f64::to_bits),
        );
        if route.assignment_executed {
            let flow = snapshot.supply_mass_flow_rate_for_sensible_output_supply_temperature_kg_per_s.expect("flow");
            let cp_air = snapshot.cp_air_for_sensible_output_supply_temperature_j_per_kg_k.expect("CpAir");
            let output = snapshot.cooling_sensible_output_for_supply_temperature_w.expect("output");
            let mixed = snapshot.mixed_air_temperature_for_sensible_output_supply_temperature_c.expect("mixed");
            let denominator = flow * cp_air;
            let drop = output / denominator;
            let calculated = mixed - drop;
            assert_eq!(snapshot.supply_mass_flow_rate_times_cp_air_for_sensible_output_supply_temperature_w_per_k.map(f64::to_bits), Some(denominator.to_bits()));
            assert_eq!(snapshot.cooling_sensible_output_over_air_capacity_rate_k.map(f64::to_bits), Some(drop.to_bits()));
            assert_eq!(snapshot.calculated_sensible_output_supply_temperature_c.map(f64::to_bits), Some(calculated.to_bits()));
            assert_eq!(snapshot.resulting_supply_temperature_c.map(f64::to_bits), Some(calculated.to_bits()));
        } else {
            assert_eq!(snapshot.resulting_supply_temperature_c.map(f64::to_bits), predecessor.resulting_supply_temperature_c.map(f64::to_bits));
        }
    }
    assert_eq!(state.transition_count, 59);
    assert_eq!(state.inactive_transition_count, 49);
    assert_eq!(state.predecessor_guard_false_fallthrough_count, 5);
    assert_eq!(state.cooling_sensible_output_supply_temperature_assignment_count, 5);
    assert_eq!(state.source_site_execution_count, 40);
    assert_eq!(state.cp422_supply_humidity_ratio_state_owner_count, 36);
    assert_eq!(state.unchanged_supply_humidity_ratio_preservation_count, 36);
    assert_eq!(state.cp422_supply_enthalpy_state_owner_count, 41);
    assert_eq!(state.unchanged_supply_enthalpy_preservation_count, 41);
    assert_eq!(state.cp422_supply_temperature_state_owner_count, 56);
    assert_eq!(state.unchanged_supply_temperature_preservation_count, 51);
    assert_eq!(state.cp423_sensible_output_supply_temperature_state_owner_count, 5);
    assert_eq!(state.predecessor_route_counts, routes);
    assert_eq!(state.predecessor_guard_false_fallthrough_route_counts, false_routes);
    assert_eq!(state.cooling_sensible_output_supply_temperature_assignment_route_counts, assignment_routes);
    assert_eq!(nonzero_indices(&false_routes), [4, 7, 10, 13, 16]);
    assert_eq!(nonzero_indices(&assignment_routes), [4, 7, 10, 13, 16]);
    assert!(super::release::state_counts_are_consistent_for_test(&state));
}

#[test]
fn inactive_and_false_routes_are_owner_lazy_and_reject_supplied_values() {
    for predecessor in cp422_all_snapshots_for_successor_tests().into_iter().filter(|snapshot| {
        !snapshot.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_executed
    }).take(2) {
        let mut state = State::new(predecessor.system);
        let snapshot = advance(&mut state, predecessor, None).expect("lazy CP423");
        assert!(snapshot.mixed_air_temperature_for_sensible_output_supply_temperature_c.is_none());
        let before = state.clone();
        assert!(advance(&mut state, predecessor, Some(dummy_input())).is_none());
        assert_eq!(state, before);
    }
}

#[test]
fn route_and_each_owner_value_forgery_are_transactional() {
    let predecessor = assignment_predecessor();
    let route = successor_route_for(predecessor);
    let input = active_input(predecessor).expect("input");
    let mut forged_routes = [route, route, route];
    forged_routes[0].logical_index = (route.logical_index + 1) % 36;
    forged_routes[1].active = false;
    forged_routes[2].assignment_executed = false;
    for forged in forged_routes {
        let mut state = State::new(predecessor.system);
        let before = state.clone();
        assert!(advance_validated(&mut state, predecessor, forged, Some(input)).is_none());
        assert_eq!(state, before);
    }
    let mut forged_inputs = [input; 4];
    forged_inputs[0].mixed_air_temperature_c = flip(input.mixed_air_temperature_c);
    forged_inputs[1].cooling_sensible_output_w = flip(input.cooling_sensible_output_w);
    forged_inputs[2].supply_mass_flow_rate_kg_per_s = flip(input.supply_mass_flow_rate_kg_per_s);
    forged_inputs[3].cp_air_j_per_kg_k = flip(input.cp_air_j_per_kg_k);
    for forged in forged_inputs {
        let mut state = State::new(predecessor.system);
        let before = state.clone();
        assert!(advance_validated(&mut state, predecessor, route, Some(forged)).is_none());
        assert_eq!(state, before);
    }
}

#[test]
fn release_and_owner_capabilities_are_statically_bounded() {
    let release = include_str!("release.rs");
    let start = release.find("pub fn advance_direct_no_oa_calc_").expect("hot start");
    let end = release[start..].find("#[allow(dead_code)]").map(|offset| start + offset).expect("hot end");
    let hot = &release[start..end];
    for forbidden in ["completed_", "snapshot_is_exact", "private_characterization"] {
        assert!(!hot.contains(forbidden), "{forbidden}");
    }
    assert_eq!(hot.matches("maximum_capacity_assignment_committed_latest_route_and_assigned_cooling_sensible_output(").count(), 1);
    assert_eq!(hot.matches("cp_air_assignment_committed_latest_route_and_cp_air(").count(), 1);
    let cp422 = include_str!("../cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment/release/committed.rs");
    assert_eq!(cp422.split("#[cfg(test)]").next().expect("seal").matches("guard_committed_latest_route_and_assignment_values(").count(), 1);
}

pub(super) fn assignment_predecessor() -> Predecessor {
    cp422_all_snapshots_for_successor_tests().into_iter().find(|snapshot| {
        snapshot.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_executed
    }).expect("assignment predecessor")
}

pub(super) fn successor_route_for(predecessor: Predecessor) -> Route {
    let predecessor_route = cp422_route(predecessor).expect("CP422 route");
    successor_route(predecessor, predecessor_route).expect("CP423 route")
}

pub(super) fn active_input(predecessor: Predecessor) -> Option<ActiveInput> {
    if !predecessor.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_executed {
        return None;
    }
    Some(ActiveInput {
        mixed_air_temperature_c: predecessor.mixed_air_temperature_for_sensible_output_c?,
        cooling_sensible_output_w: predecessor.resulting_cooling_sensible_output_after_maximum_capacity_assignment_w?,
        supply_mass_flow_rate_kg_per_s: predecessor.supply_mass_flow_rate_kg_per_s?,
        cp_air_j_per_kg_k: predecessor.cp_air_j_per_kg_k?,
    })
}

fn dummy_input() -> ActiveInput {
    ActiveInput { mixed_air_temperature_c: 1.0, cooling_sensible_output_w: 2.0, supply_mass_flow_rate_kg_per_s: 3.0, cp_air_j_per_kg_k: 4.0 }
}

fn flip(value: f64) -> f64 { f64::from_bits(value.to_bits() ^ 1) }

fn nonzero_indices(values: &[usize; 36]) -> Vec<usize> {
    values.iter().enumerate().filter_map(|(index, count)| (*count != 0).then_some(index)).collect()
}

pub(in crate::ideal_loads::calc) fn cp423_all_snapshots_for_successor_tests() -> Vec<super::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputSupplyTemperatureAssignmentSnapshot> {
    let predecessors = cp422_all_snapshots_for_successor_tests();
    let mut state = State::new(predecessors[0].system);
    predecessors
        .into_iter()
        .map(|predecessor| {
            let route = successor_route_for(predecessor);
            advance_validated(&mut state, predecessor, route, active_input(predecessor))
                .expect("CP423 successor fixture")
        })
        .collect()
}
