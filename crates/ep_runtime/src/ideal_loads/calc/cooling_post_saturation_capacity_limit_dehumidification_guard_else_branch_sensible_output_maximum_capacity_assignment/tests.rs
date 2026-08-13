//! CP422 boundary, exhaustive routes, ownership, and hot-path tests.

mod overflow;
mod schema_ieee;

use super::transition::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputMaximumCapacityAssignmentRetainedRoute as Route,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_route_from_committed_predecessor as successor_route,
};
use super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputMaximumCapacityAssignmentActiveInput as ActiveInput,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputMaximumCapacityAssignmentRuntimeState as State,
    advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_state as advance,
    advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_state_with_validated_route as advance_validated,
};
use crate::ideal_loads::calc::{
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_snapshot_route as cp421_route,
    cp421_all_snapshots_for_successor_tests,
};
use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardSnapshot as Predecessor;

#[test]
fn cp422_boundary_and_two_sites_are_exact() {
    assert_eq!(
        super::PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2333",
    );
    assert_eq!(
        super::PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2334",
    );
    assert_eq!(
        super::PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE_ORDER,
        &[
            "read-retained-maximum-total-cooling-capacity-for-post-saturation-capacity-limit-dehumidification-guard-else-branch-sensible-output-assignment",
            "assign-local-cooling-sensible-output-from-maximum-total-cooling-capacity",
        ],
    );
}

#[test]
fn exhaustive_59_routes_have_exact_assignment_and_owner_accounting() {
    let predecessors = cp421_all_snapshots_for_successor_tests();
    assert_eq!(predecessors.len(), 59);
    let mut state = State::new(predecessors[0].system);
    let mut predecessor_counts = [0usize; 36];
    let mut false_counts = [0usize; 36];
    let mut assignment_counts = [0usize; 36];
    let mut public = 0;
    let mut private = 0;

    for predecessor in predecessors {
        let route = successor_route_for(predecessor);
        let snapshot = advance_validated(&mut state, predecessor, route, active_input(predecessor))
            .expect("CP422");
        predecessor_counts[route.logical_index] += 1;
        false_counts[route.logical_index] += usize::from(route.active && !route.assignment_executed);
        assignment_counts[route.logical_index] += usize::from(route.assignment_executed);
        if is_public_logical_index(route.logical_index) {
            public += 1;
        } else {
            private += 1;
        }
        assert_eq!(
            snapshot.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_executed,
            route.assignment_executed,
        );
        if route.assignment_executed {
            assert_eq!(
                snapshot
                    .assigned_cooling_sensible_output_from_maximum_capacity_w
                    .map(f64::to_bits),
                snapshot
                    .maximum_total_cooling_capacity_for_sensible_output_assignment_w
                    .map(f64::to_bits),
            );
        } else if route.active {
            assert_eq!(
                snapshot
                    .resulting_cooling_sensible_output_after_maximum_capacity_assignment_w
                    .map(f64::to_bits),
                snapshot
                    .preexisting_cooling_sensible_output_for_maximum_capacity_assignment_w
                    .map(f64::to_bits),
            );
        }
    }

    assert_eq!(state.transition_count, 59);
    assert_eq!(state.inactive_transition_count, 49);
    assert_eq!(state.predecessor_guard_false_fallthrough_count, 5);
    assert_eq!(state.cooling_sensible_output_maximum_capacity_assignment_count, 5);
    assert_eq!(state.source_site_execution_count, 10);
    assert_eq!(public, 19);
    assert_eq!(private, 40);
    assert_eq!(state.cp421_supply_humidity_ratio_state_owner_count, 36);
    assert_eq!(state.cp421_supply_enthalpy_state_owner_count, 41);
    assert_eq!(state.cp421_supply_temperature_state_owner_count, 56);
    assert_eq!(state.predecessor_route_counts, predecessor_counts);
    assert_eq!(state.predecessor_guard_false_fallthrough_route_counts, false_counts);
    assert_eq!(
        state.cooling_sensible_output_maximum_capacity_assignment_route_counts,
        assignment_counts,
    );
    assert_eq!(nonzero_indices(&false_counts), [4, 7, 10, 13, 16]);
    assert_eq!(nonzero_indices(&assignment_counts), [4, 7, 10, 13, 16]);
    assert!(super::release::state_counts_are_consistent_for_test(&state));
}

#[test]
fn inactive_route_is_owner_lazy_and_rejects_supplied_values_transactionally() {
    let predecessor = cp421_all_snapshots_for_successor_tests()
        .into_iter()
        .find(|snapshot| {
            !snapshot.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_guard_evaluated
        })
        .expect("inactive");
    let mut state = State::new(predecessor.system);
    let snapshot = advance(&mut state, predecessor, None).expect("inactive CP422");
    assert!(snapshot
        .preexisting_cooling_sensible_output_for_maximum_capacity_assignment_w
        .is_none());
    assert!(!snapshot.maximum_total_cooling_capacity_for_sensible_output_assignment_read);

    let before = state.clone();
    assert!(advance(
        &mut state,
        predecessor,
        Some(ActiveInput {
            preexisting_cooling_sensible_output_w: 1.0,
            maximum_total_cooling_capacity_w: 2.0,
            cp421_retained_maximum_total_cooling_capacity_owned_read: true,
        }),
    )
    .is_none());
    assert_eq!(state, before);
}

#[test]
fn validated_route_and_owner_value_forgeries_are_transactional() {
    let predecessor = cp421_all_snapshots_for_successor_tests()
        .into_iter()
        .find(|snapshot| {
            snapshot.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_adjustment_body_entered
        })
        .expect("assignment");
    let route = successor_route_for(predecessor);
    let input = active_input(predecessor).expect("input");

    let mut routes = [route, route, route];
    routes[0].logical_index = (route.logical_index + 1) % 36;
    routes[1].active = false;
    routes[2].assignment_executed = false;
    for forged in routes {
        let mut state = State::new(predecessor.system);
        let before = state.clone();
        assert!(advance_validated(&mut state, predecessor, forged, Some(input)).is_none());
        assert_eq!(state, before);
    }

    let mut inputs = [input, input, input];
    inputs[0].cp421_retained_maximum_total_cooling_capacity_owned_read = false;
    inputs[1].preexisting_cooling_sensible_output_w = f64::from_bits(
        inputs[1].preexisting_cooling_sensible_output_w.to_bits() ^ 1,
    );
    inputs[2].maximum_total_cooling_capacity_w =
        f64::from_bits(inputs[2].maximum_total_cooling_capacity_w.to_bits() ^ 1);
    for forged in inputs {
        let mut state = State::new(predecessor.system);
        let before = state.clone();
        assert!(advance_validated(&mut state, predecessor, route, Some(forged)).is_none());
        assert_eq!(state, before);
    }

    let mut coordinated = predecessor;
    coordinated.cp420_cooling_sensible_output_for_capacity_guard_w = Some(0.0);
    coordinated.maximum_total_cooling_capacity_w = Some(1.0);
    let coordinated_input = ActiveInput {
        preexisting_cooling_sensible_output_w: 0.0,
        maximum_total_cooling_capacity_w: 1.0,
        cp421_retained_maximum_total_cooling_capacity_owned_read: true,
    };
    let mut state = State::new(predecessor.system);
    let before = state.clone();
    assert!(advance_validated(
        &mut state,
        coordinated,
        route,
        Some(coordinated_input),
    )
    .is_none());
    assert_eq!(state, before);
}

#[test]
fn hot_release_and_cp421_committed_owner_are_bounded() {
    let release = include_str!("release.rs");
    let start = release
        .find("pub fn advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment")
        .expect("release");
    let end = release[start..]
        .find("#[allow(dead_code)]")
        .map(|offset| start + offset)
        .expect("hot end");
    let hot = &release[start..end];
    for forbidden in ["completed_", "snapshot_is_exact", "private_characterization"] {
        assert!(!hot.contains(forbidden), "forbidden {forbidden}");
    }
    assert_eq!(
        hot.matches("guard_committed_latest_route_and_assignment_values")
            .count(),
        1,
    );
    let committed = include_str!(
        "../cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard/release/committed.rs"
    );
    assert!(!committed.contains("snapshot_is_exact"));
    assert!(committed.contains("retained_route_matches_snapshot_bounded"));
}

pub(super) fn active_input(predecessor: Predecessor) -> Option<ActiveInput> {
    predecessor
        .post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_guard_evaluated
        .then_some(ActiveInput {
            preexisting_cooling_sensible_output_w: predecessor
                .cp420_cooling_sensible_output_for_capacity_guard_w?,
            maximum_total_cooling_capacity_w: predecessor.maximum_total_cooling_capacity_w?,
            cp421_retained_maximum_total_cooling_capacity_owned_read: true,
        })
}

pub(super) fn successor_route_for(predecessor: Predecessor) -> Route {
    let route = cp421_route(predecessor).expect("CP421 route");
    successor_route(predecessor, route).expect("CP422 route")
}

fn is_public_logical_index(index: usize) -> bool {
    matches!(index, 0..=8 | 20 | 21 | 26 | 27)
}

fn nonzero_indices(values: &[usize; 36]) -> Vec<usize> {
    values
        .iter()
        .enumerate()
        .filter_map(|(index, count)| (*count != 0).then_some(index))
        .collect()
}

pub(in crate::ideal_loads::calc) fn cp422_all_snapshots_for_successor_tests() -> Vec<super::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputMaximumCapacityAssignmentSnapshot> {
    let predecessors = cp421_all_snapshots_for_successor_tests();
    let mut state = State::new(predecessors[0].system);
    predecessors
        .into_iter()
        .map(|predecessor| {
            let route = successor_route_for(predecessor);
            advance_validated(&mut state, predecessor, route, active_input(predecessor))
                .expect("CP422 successor fixture")
        })
        .collect()
}
