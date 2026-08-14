//! CP425 boundary, exhaustive route, forgery, preservation, and overflow tests.

mod schema_prefix;

use super::transition::{
    PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyEnthalpyMixedAirAssignmentRetainedRoute as Route,
    cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_route_from_committed_predecessor as successor_route,
};
use super::{
    PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyEnthalpyMixedAirAssignmentRuntimeState as State,
    advance_cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_state as advance,
    advance_cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_state_with_validated_route as advance_validated,
};
use crate::ideal_loads::calc::{
    cooling_supply_mass_flow_positive_guard_else_branch_entry_snapshot_route as cp424_route,
    cp424_all_snapshots_for_successor_tests,
};
use crate::ideal_loads::PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardElseBranchEntrySnapshot as Predecessor;

#[test]
fn cp425_boundary_and_two_sites_are_exact() {
    assert_eq!(
        super::PURCHASED_AIR_CALC_COOLING_ZERO_SUPPLY_MASS_FLOW_SUPPLY_ENTHALPY_MIXED_AIR_ASSIGNMENT_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2340",
    );
    assert_eq!(
        super::PURCHASED_AIR_CALC_COOLING_ZERO_SUPPLY_MASS_FLOW_SUPPLY_ENTHALPY_MIXED_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2341",
    );
    assert_eq!(
        super::PURCHASED_AIR_CALC_COOLING_ZERO_SUPPLY_MASS_FLOW_SUPPLY_ENTHALPY_MIXED_AIR_ASSIGNMENT_SOURCE_ORDER,
        &[
            "read-retained-mixed-air-enthalpy-for-zero-supply-mass-flow-supply-enthalpy-assignment",
            "assign-local-supply-enthalpy-from-mixed-air-enthalpy-for-zero-supply-mass-flow",
        ],
    );
}

#[test]
fn exhaustive_59_routes_assign_only_index_two_and_preserve_every_route() {
    let predecessors = cp424_all_snapshots_for_successor_tests();
    assert_eq!(predecessors.len(), 59);
    let mut state = State::new(predecessors[0].system);
    let mut predecessor_counts = [0usize; 36];
    let mut assignment_counts = [0usize; 36];
    for predecessor in predecessors {
        let predecessor_route = cp424_route(predecessor).expect("CP424 route");
        let route = successor_route(predecessor, predecessor_route).expect("CP425 route");
        let rhs = route.assignment_executed.then_some(-0.0);
        let snapshot =
            advance_validated(&mut state, predecessor, route, rhs).expect("CP425");
        predecessor_counts[route.logical_index] += 1;
        assignment_counts[route.logical_index] += usize::from(route.assignment_executed);
        assert_eq!(route.logical_index, predecessor_route.logical_index);
        assert_eq!(
            route.predecessor_assignment_executed,
            predecessor_route.assignment_executed
        );
        assert_eq!(route.predecessor_entered, predecessor_route.entered);
        assert_eq!(route.active, route.logical_index == 2);
        assert_eq!(route.assignment_executed, route.logical_index == 2);
        assert_bits(
            snapshot.resulting_supply_humidity_ratio,
            predecessor.resulting_supply_humidity_ratio,
        );
        assert_bits(
            snapshot.resulting_supply_enthalpy_j_per_kg,
            rhs.or(predecessor.resulting_supply_enthalpy_j_per_kg),
        );
        assert_bits(
            snapshot.resulting_supply_temperature_c,
            predecessor.resulting_supply_temperature_c,
        );
    }
    assert_eq!(state.transition_count, 59);
    assert_eq!(state.inactive_transition_count, 58);
    assert_eq!(
        state.zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_count,
        1
    );
    assert_eq!(state.source_site_execution_count, 2);
    assert_eq!(state.predecessor_route_counts, predecessor_counts);
    assert_eq!(
        state.zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_route_counts,
        assignment_counts
    );
    assert_eq!(nonzero_indices(&assignment_counts), [2]);
    assert_eq!(state.cp424_supply_humidity_ratio_state_owner_count, 36);
    assert_eq!(state.unchanged_supply_humidity_ratio_preservation_count, 36);
    assert_eq!(state.cp424_supply_enthalpy_state_owner_count, 41);
    assert_eq!(state.unchanged_supply_enthalpy_preservation_count, 41);
    assert_eq!(state.cp424_supply_temperature_state_owner_count, 56);
    assert_eq!(state.unchanged_supply_temperature_preservation_count, 56);
    assert_eq!(state.cp425_supply_enthalpy_state_owner_count, 1);
    assert_eq!(state.cp329_retained_mixed_air_enthalpy_owned_read_count, 1);
    assert_eq!(
        state.mixed_air_enthalpy_for_zero_supply_mass_flow_supply_enthalpy_read_count,
        1
    );
    assert_eq!(state.supply_enthalpy_assignment_write_count, 1);
    assert!(super::release::state_counts_are_consistent_for_test(&state));
}

#[test]
fn every_route_component_and_rhs_presence_forgery_is_transactional() {
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
        assert!(advance_validated(&mut state, predecessor, route, Some(1.0)).is_none());
        assert_eq!(state, before);
    }
    let mut state = State::new(predecessor.system);
    let before = state.clone();
    assert!(advance_validated(&mut state, predecessor, route, None).is_none());
    assert_eq!(state, before);

    let inactive = cp424_all_snapshots_for_successor_tests()
        .into_iter()
        .find(|snapshot| !route_for(*snapshot).assignment_executed)
        .expect("inactive");
    let inactive_route = route_for(inactive);
    let mut state = State::new(inactive.system);
    let before = state.clone();
    assert!(advance_validated(&mut state, inactive, inactive_route, Some(1.0)).is_none());
    assert_eq!(state, before);
}

#[test]
fn pure_transition_copies_every_raw_binary64_rhs_bit_pattern() {
    let predecessor = active_predecessor();
    for bits in [
        (-0.0f64).to_bits(),
        1u64,
        0x7ff8_0000_0000_0042,
    ] {
        let value = f64::from_bits(bits);
        let snapshot =
            advance(&mut State::new(predecessor.system), predecessor, Some(value))
                .expect("CP425 raw bits");
        assert_eq!(
            snapshot
                .mixed_air_enthalpy_for_zero_supply_mass_flow_supply_enthalpy_j_per_kg
                .map(f64::to_bits),
            Some(bits),
        );
        assert_eq!(
            snapshot
                .assigned_supply_enthalpy_from_mixed_air_j_per_kg
                .map(f64::to_bits),
            Some(bits),
        );
        assert_eq!(
            snapshot.resulting_supply_enthalpy_j_per_kg.map(f64::to_bits),
            Some(bits),
        );
    }
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
    scalar!(zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_count);
    scalar!(source_site_execution_count);
    scalar!(cp425_supply_enthalpy_state_owner_count);
    scalar!(cp329_retained_mixed_air_enthalpy_owned_read_count);
    scalar!(mixed_air_enthalpy_for_zero_supply_mass_flow_supply_enthalpy_read_count);
    scalar!(supply_enthalpy_assignment_write_count);
    let mut predecessor_route = State::new(predecessor.system);
    predecessor_route.predecessor_route_counts[route.logical_index] = usize::MAX;
    states.push(predecessor_route);
    let mut assignment_route = State::new(predecessor.system);
    assignment_route
        .zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_route_counts
        [route.logical_index] = usize::MAX;
    states.push(assignment_route);
    for mut state in states {
        let before = state.clone();
        assert!(advance_validated(&mut state, predecessor, route, Some(1.0)).is_none());
        assert_eq!(state, before);
    }

    let inactive = cp424_all_snapshots_for_successor_tests()
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
    inactive_scalar!(cp424_supply_humidity_ratio_state_owner_count);
    inactive_scalar!(unchanged_supply_humidity_ratio_preservation_count);
    inactive_scalar!(cp424_supply_enthalpy_state_owner_count);
    inactive_scalar!(unchanged_supply_enthalpy_preservation_count);
    inactive_scalar!(cp424_supply_temperature_state_owner_count);
    inactive_scalar!(unchanged_supply_temperature_preservation_count);
    for mut state in fields {
        let before = state.clone();
        assert!(advance_validated(&mut state, inactive, inactive_route, None).is_none());
        assert_eq!(state, before);
    }
}

#[test]
fn cp425_cp424_and_cp329_hot_paths_are_statically_bounded_and_lazy() {
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
        hot.matches("else_branch_entry_committed_latest_route(").count(),
        1
    );
    assert_eq!(
        hot.matches("committed_latest_mixed_air_enthalpy(").count(),
        1
    );
    let owner_index = hot.find("if route.assignment_executed").expect("owner guard");
    let owner_call = hot
        .find("committed_latest_mixed_air_enthalpy(")
        .expect("owner call");
    assert!(owner_call > owner_index);
}

pub(super) fn active_predecessor() -> Predecessor {
    cp424_all_snapshots_for_successor_tests()
        .into_iter()
        .find(|snapshot| snapshot.cooling_supply_mass_flow_positive_guard_else_branch_entered)
        .expect("CP425 active predecessor")
}

pub(super) fn route_for(predecessor: Predecessor) -> Route {
    successor_route(
        predecessor,
        cp424_route(predecessor).expect("CP424 route"),
    )
    .expect("CP425 route")
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
