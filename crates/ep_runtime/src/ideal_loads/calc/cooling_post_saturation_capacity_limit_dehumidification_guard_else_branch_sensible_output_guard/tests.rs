//! CP421 boundary, exhaustive routes, ownership, and hot-path tests.

mod overflow;
mod schema_ieee;

use super::transition::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardRetainedRoute as Route,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_route_from_committed_predecessor as successor_route,
};
use super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardActiveInput as ActiveInput,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardRuntimeState as State,
    advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_state as advance,
    advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_state_with_validated_route as advance_validated,
};
use crate::ideal_loads::calc::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputAssignmentActiveInput as Cp420Input,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputAssignmentRuntimeState as Cp420State,
    advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_state as advance_cp420,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_snapshot_route as cp420_route,
    cp419_all_snapshots_for_successor_tests,
};
use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputAssignmentSnapshot as Predecessor;

#[test]
fn cp421_boundary_and_four_sites_are_exact() {
    assert_eq!(
        super::PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_GUARD_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2332",
    );
    assert_eq!(
        super::PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2333",
    );
    assert_eq!(
        super::PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_GUARD_SOURCE_ORDER,
        &[
            "read-retained-cooling-sensible-output-for-post-saturation-capacity-limit-dehumidification-guard-else-branch-maximum-capacity-comparison",
            "read-retained-maximum-total-cooling-capacity-for-post-saturation-capacity-limit-dehumidification-guard-else-branch-sensible-output-comparison",
            "compare-post-saturation-capacity-limit-dehumidification-guard-else-branch-cooling-sensible-output-greater-than-or-equal-to-maximum-total-cooling-capacity",
            "enter-post-saturation-capacity-limit-dehumidification-guard-else-branch-sensible-output-capacity-adjustment-body-if-comparison-satisfied",
        ],
    );
}

#[test]
fn exhaustive_54_predecessors_refine_to_59_successors_with_exact_accounting() {
    let predecessors = cp420_predecessors();
    assert_eq!(predecessors.len(), 54);
    let mut state = State::new(predecessors[0].system);
    let mut predecessor_counts = [0usize; 36];
    let mut false_counts = [0usize; 36];
    let mut body_counts = [0usize; 36];
    let mut public = 0;
    let mut private = 0;

    for predecessor in predecessors {
        let committed = cp420_route(predecessor).expect("CP420 route");
        let base = successor_route(predecessor, committed).expect("CP421 route");
        let outputs: &[bool] = if base.active { &[false, true] } else { &[false] };
        for &body in outputs {
            let input = active_input(predecessor, body);
            let snapshot =
                advance_validated(&mut state, predecessor, base, input).expect("CP421");
            predecessor_counts[base.logical_index] += 1;
            false_counts[base.logical_index] += usize::from(base.active && !body);
            body_counts[base.logical_index] += usize::from(body);
            if is_public_logical_index(base.logical_index) {
                public += 1;
            } else {
                private += 1;
            }
            assert_eq!(
                snapshot.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_adjustment_body_entered,
                body,
            );
        }
    }

    assert_eq!(state.transition_count, 59);
    assert_eq!(state.inactive_transition_count, 49);
    assert_eq!(
        state.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_guard_evaluation_count,
        10,
    );
    assert_eq!(
        state.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_guard_false_fallthrough_count,
        5,
    );
    assert_eq!(
        state.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_adjustment_body_entry_count,
        5,
    );
    assert_eq!(state.source_site_execution_count, 35);
    assert_eq!(public, 19);
    assert_eq!(private, 40);
    assert_eq!(state.cp420_supply_humidity_ratio_state_owner_count, 36);
    assert_eq!(state.cp420_supply_enthalpy_state_owner_count, 41);
    assert_eq!(state.cp420_supply_temperature_state_owner_count, 56);
    assert_eq!(state.predecessor_route_counts, predecessor_counts);
    assert_eq!(state.guard_false_fallthrough_route_counts, false_counts);
    assert_eq!(state.adjustment_body_entry_route_counts, body_counts);
    assert_eq!(
        nonzero_indices(&state.guard_false_fallthrough_route_counts),
        [4, 7, 10, 13, 16],
    );
    assert_eq!(
        nonzero_indices(&state.adjustment_body_entry_route_counts),
        [4, 7, 10, 13, 16],
    );
    assert!(super::release::state_counts_are_consistent_for_test(&state));
}

#[test]
fn inactive_route_is_owner_lazy_and_rejects_supplied_operands_transactionally() {
    let predecessor = cp420_predecessors()
        .into_iter()
        .find(|snapshot| !snapshot.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_executed)
        .expect("inactive");
    let mut state = State::new(predecessor.system);
    let snapshot = advance(&mut state, predecessor, None).expect("owner-lazy inactive");
    assert!(!snapshot.cooling_sensible_output_read);
    assert!(!snapshot.maximum_total_cooling_capacity_read);

    let mut rejected = State::new(predecessor.system);
    let before = rejected.clone();
    let forged = ActiveInput {
        cooling_sensible_output_w: 1.0,
        maximum_total_cooling_capacity_w: 2.0,
        cp420_cooling_sensible_output_owned_read: true,
        cp321_maximum_total_cooling_capacity_owned_read: true,
        cp340_same_call_maximum_total_cooling_capacity_bit_corroborated: true,
    };
    assert!(advance(&mut rejected, predecessor, Some(forged)).is_none());
    assert_eq!(rejected, before);
}

#[test]
fn hot_release_and_pending_validation_have_no_recursive_lineage_calls() {
    let source = include_str!("release.rs");
    let start = source
        .find("pub fn advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard")
        .expect("public release");
    let end = source[start..]
        .find("#[allow(dead_code)]")
        .map(|offset| start + offset)
        .expect("hot end");
    let hot = &source[start..end];
    for forbidden in [
        "completed_",
        "snapshot_is_exact",
        "predecessor_route(",
        "private_characterization",
    ] {
        assert!(!hot.contains(forbidden), "forbidden {forbidden}");
    }
    for required in [
        "assignment_committed_latest_route_and_cooling_sensible_output",
        "guard_committed_latest_maximum_total_cooling_capacity",
        "advance_with_route",
    ] {
        assert!(hot.contains(required), "missing {required}");
    }

    let runtime = include_str!("release/runtime_validation.rs");
    let latest = runtime
        .split("fn latest_is_consistent")
        .nth(1)
        .expect("latest helper");
    assert!(!latest.contains("snapshot_route("));
    assert!(latest.contains("retained_route_matches_snapshot_bounded"));
}

pub(super) fn cp420_predecessors() -> Vec<Predecessor> {
    let cp419 = cp419_all_snapshots_for_successor_tests();
    let mut state = Cp420State::new(cp419[0].system);
    cp419
        .into_iter()
        .map(|predecessor| {
            let input = predecessor
                .post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_executed
                .then_some(Cp420Input {
                    supply_mass_flow_rate_kg_per_s: 0.25,
                    mixed_air_temperature_c: 17.0,
                });
            advance_cp420(&mut state, predecessor, input).expect("CP420")
        })
        .collect()
}

pub(super) fn active_input(predecessor: Predecessor, body: bool) -> Option<ActiveInput> {
    let output = predecessor.cooling_sensible_output_w?;
    Some(ActiveInput {
        cooling_sensible_output_w: output,
        maximum_total_cooling_capacity_w: if body { output } else { output + 1.0 },
        cp420_cooling_sensible_output_owned_read: true,
        cp321_maximum_total_cooling_capacity_owned_read: true,
        cp340_same_call_maximum_total_cooling_capacity_bit_corroborated: true,
    })
}

pub(super) fn successor_route_for(predecessor: Predecessor) -> Route {
    let route = cp420_route(predecessor).expect("CP420 route");
    successor_route(predecessor, route).expect("CP421 route")
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
