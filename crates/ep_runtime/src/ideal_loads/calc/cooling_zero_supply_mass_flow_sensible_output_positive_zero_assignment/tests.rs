//! CP428 boundary, exhaustive route, forgery, preservation, and overflow tests.

mod committed_seal;
mod schema_prefix;

pub(in crate::ideal_loads::calc) use committed_seal::cp428_fixture_unit_for_successor_tests;

use super::transition::{
    PurchasedAirCalcCoolingZeroSupplyMassFlowSensibleOutputPositiveZeroAssignmentRetainedRoute as Route,
    cooling_zero_supply_mass_flow_sensible_output_positive_zero_assignment_route_from_committed_predecessor as successor_route,
};
use super::{
    PurchasedAirCalcCoolingZeroSupplyMassFlowSensibleOutputPositiveZeroAssignmentRuntimeState as State,
    advance_cooling_zero_supply_mass_flow_sensible_output_positive_zero_assignment_state as advance,
    advance_cooling_zero_supply_mass_flow_sensible_output_positive_zero_assignment_state_with_validated_route as advance_validated,
};
use crate::ideal_loads::calc::{
    cooling_zero_supply_mass_flow_supply_temperature_mixed_air_assignment_snapshot_route as cp427_route,
    cp427_all_snapshots_for_successor_tests,
};
use crate::ideal_loads::PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyTemperatureMixedAirAssignmentSnapshot as Predecessor;

#[test]
fn cp428_boundary_and_single_site_are_exact() {
    assert_eq!(
        super::PURCHASED_AIR_CALC_COOLING_ZERO_SUPPLY_MASS_FLOW_SENSIBLE_OUTPUT_POSITIVE_ZERO_ASSIGNMENT_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2343",
    );
    assert_eq!(
        super::PURCHASED_AIR_CALC_COOLING_ZERO_SUPPLY_MASS_FLOW_SENSIBLE_OUTPUT_POSITIVE_ZERO_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2344",
    );
    assert_eq!(
        super::PURCHASED_AIR_CALC_COOLING_ZERO_SUPPLY_MASS_FLOW_SENSIBLE_OUTPUT_POSITIVE_ZERO_ASSIGNMENT_SOURCE_ORDER,
        &["assign-local-cooling-sensible-output-positive-zero-for-zero-supply-mass-flow"],
    );
}

#[test]
fn exhaustive_59_routes_assign_only_index_two_and_preserve_every_route() {
    let predecessors = cp427_all_snapshots_for_successor_tests();
    assert_eq!(predecessors.len(), 59);
    let mut state = State::new(predecessors[0].system);
    let mut predecessor_counts = [0usize; 36];
    let mut assignment_counts = [0usize; 36];
    for predecessor in predecessors {
        let predecessor_route = cp427_route(predecessor).expect("CP427 route");
        let route = successor_route(predecessor, predecessor_route).expect("CP428 route");
        let snapshot = advance_validated(&mut state, predecessor, route).expect("CP428");
        predecessor_counts[route.logical_index] += 1;
        assignment_counts[route.logical_index] += usize::from(route.assignment_executed);
        assert_eq!(route.logical_index, predecessor_route.logical_index);
        assert_eq!(
            route.predecessor_assignment_executed,
            predecessor_route.predecessor_assignment_executed
        );
        assert_eq!(route.predecessor_entered, predecessor_route.predecessor_entered);
        assert_eq!(route.active, route.logical_index == 2);
        assert_eq!(route.assignment_executed, route.logical_index == 2);
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
        assert_eq!(
            snapshot.assigned_cooling_sensible_output_w.map(f64::to_bits),
            route.assignment_executed.then_some(0),
        );
    }
    assert_eq!(state.transition_count, 59);
    assert_eq!(state.inactive_transition_count, 58);
    assert_eq!(
        state.zero_supply_mass_flow_sensible_output_positive_zero_assignment_count,
        1
    );
    assert_eq!(state.source_site_execution_count, 1);
    assert_eq!(state.predecessor_route_counts, predecessor_counts);
    assert_eq!(
        state.zero_supply_mass_flow_sensible_output_positive_zero_assignment_route_counts,
        assignment_counts
    );
    assert_eq!(nonzero_indices(&assignment_counts), [2]);
    assert_eq!(state.cp427_supply_humidity_ratio_state_owner_count, 37);
    assert_eq!(state.unchanged_supply_humidity_ratio_preservation_count, 37);
    assert_eq!(state.cp427_supply_enthalpy_state_owner_count, 42);
    assert_eq!(state.unchanged_supply_enthalpy_preservation_count, 42);
    assert_eq!(state.cp427_supply_temperature_state_owner_count, 57);
    assert_eq!(state.unchanged_supply_temperature_preservation_count, 57);
    assert_eq!(state.cp428_cooling_sensible_output_state_owner_count, 1);
    assert_eq!(state.cooling_sensible_output_assignment_write_count, 1);
    assert!(super::release::state_counts_are_consistent_for_test(&state));
}

#[test]
fn every_route_component_forgery_is_transactional() {
    let predecessor = active_predecessor();
    let route = route_for(predecessor);
    let mut forged = [route; 5];
    forged[0].logical_index = 3;
    forged[1].active = false;
    forged[2].predecessor_assignment_executed ^= true;
    forged[3].predecessor_entered = false;
    forged[4].assignment_executed = false;
    for route in forged {
        let mut state = State::new(predecessor.system);
        let before = state.clone();
        assert!(advance_validated(&mut state, predecessor, route).is_none());
        assert_eq!(state, before);
    }
}

#[test]
fn pure_transition_writes_only_exact_positive_zero_on_the_active_route() {
    let predecessor = active_predecessor();
    let snapshot = advance(&mut State::new(predecessor.system), predecessor).expect("CP428");
    assert_eq!(snapshot.assigned_cooling_sensible_output_w.map(f64::to_bits), Some(0));
    assert_ne!(snapshot.assigned_cooling_sensible_output_w.map(f64::to_bits), Some((-0.0_f64).to_bits()));
    let mut forged = snapshot;
    forged.assigned_cooling_sensible_output_w = Some(-0.0_f64);
    assert!(
        !super::cooling_zero_supply_mass_flow_sensible_output_positive_zero_assignment_snapshot_is_exact(
            forged,
        )
    );
    assert!(
        !super::cooling_zero_supply_mass_flow_sensible_output_positive_zero_assignment_snapshots_match_bit_exact(
            snapshot,
            forged,
        )
    );

    let inactive = cp427_all_snapshots_for_successor_tests()
        .into_iter()
        .find(|snapshot| !route_for(*snapshot).assignment_executed)
        .expect("inactive");
    let snapshot = advance(&mut State::new(inactive.system), inactive).expect("inactive CP428");
    assert!(snapshot.assigned_cooling_sensible_output_w.is_none());
}

#[test]
fn every_incremented_counter_overflow_is_transactional() {
    let predecessor = active_predecessor();
    let route = route_for(predecessor);
    let mut states = Vec::new();
    macro_rules! scalar {
        ($field:ident) => {{
            let mut state = State::new(predecessor.system);
            state.$field = usize::MAX;
            states.push(state);
        }};
    }
    scalar!(transition_count);
    scalar!(zero_supply_mass_flow_sensible_output_positive_zero_assignment_count);
    scalar!(source_site_execution_count);
    scalar!(cp428_cooling_sensible_output_state_owner_count);
    scalar!(cooling_sensible_output_assignment_write_count);
    let mut predecessor_route = State::new(predecessor.system);
    predecessor_route.predecessor_route_counts[route.logical_index] = usize::MAX;
    states.push(predecessor_route);
    let mut assignment_route = State::new(predecessor.system);
    assignment_route
        .zero_supply_mass_flow_sensible_output_positive_zero_assignment_route_counts
        [route.logical_index] = usize::MAX;
    states.push(assignment_route);
    for mut state in states {
        let before = state.clone();
        assert!(advance_validated(&mut state, predecessor, route).is_none());
        assert_eq!(state, before);
    }

    let inactive = cp427_all_snapshots_for_successor_tests()
        .into_iter()
        .find(|snapshot| {
            !route_for(*snapshot).active
                && snapshot.resulting_supply_humidity_ratio.is_some()
                && snapshot.resulting_supply_enthalpy_j_per_kg.is_some()
                && snapshot.resulting_supply_temperature_c.is_some()
        })
        .expect("inactive all-owner route");
    let inactive_route = route_for(inactive);
    let mut fields = Vec::new();
    macro_rules! inactive_scalar {
        ($field:ident) => {{
            let mut state = State::new(inactive.system);
            state.$field = usize::MAX;
            fields.push(state);
        }};
    }
    inactive_scalar!(inactive_transition_count);
    inactive_scalar!(cp427_supply_humidity_ratio_state_owner_count);
    inactive_scalar!(unchanged_supply_humidity_ratio_preservation_count);
    inactive_scalar!(cp427_supply_enthalpy_state_owner_count);
    inactive_scalar!(unchanged_supply_enthalpy_preservation_count);
    inactive_scalar!(cp427_supply_temperature_state_owner_count);
    inactive_scalar!(unchanged_supply_temperature_preservation_count);
    for mut state in fields {
        let before = state.clone();
        assert!(advance_validated(&mut state, inactive, inactive_route).is_none());
        assert_eq!(state, before);
    }
}

#[test]
fn cp428_cp427_and_cp329_hot_paths_are_statically_bounded_and_lazy() {
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
        hot.matches("supply_temperature_mixed_air_assignment_committed_latest_route(")
            .count(),
        1
    );
    assert_eq!(
        hot.matches("committed_latest_mixed_air_temperature(").count(),
        0
    );
    let owner_index = hot.find("if predecessor_cp427").expect("witness guard");
    let owner_call = hot.find("cooling_mixed_air_call_latest_witness").expect("witness call");
    assert!(owner_call > owner_index);
}

pub(super) fn active_predecessor() -> Predecessor {
    cp427_all_snapshots_for_successor_tests()
        .into_iter()
        .find(|snapshot| snapshot.cooling_zero_supply_mass_flow_supply_temperature_mixed_air_assignment_executed)
        .expect("CP428 active predecessor")
}

pub(in crate::ideal_loads::calc) fn cp428_all_snapshots_for_successor_tests() -> Vec<
    crate::ideal_loads::PurchasedAirCalcCoolingZeroSupplyMassFlowSensibleOutputPositiveZeroAssignmentSnapshot,
> {
    cp427_all_snapshots_for_successor_tests()
        .into_iter()
        .map(|predecessor| {
            let route = route_for(predecessor);
            advance_validated(&mut State::new(predecessor.system), predecessor, route)
                .expect("CP428 successor fixture")
        })
        .collect()
}

pub(super) fn route_for(predecessor: Predecessor) -> Route {
    successor_route(
        predecessor,
        cp427_route(predecessor).expect("CP427 route"),
    )
    .expect("CP428 route")
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
