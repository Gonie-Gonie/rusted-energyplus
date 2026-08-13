//! CP422 bounded state and predecessor-accounting validation.

use super::super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputMaximumCapacityAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputMaximumCapacityAssignmentSnapshot as Snapshot,
};
use super::super::transition::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputMaximumCapacityAssignmentRetainedRoute as Route;
use super::snapshot_validation::{
    retained_route_matches_snapshot_bounded, snapshots_match_bit_exact,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardRuntimeState as PredecessorState,
    PurchasedAirUnitRuntimeState,
};

pub(super) fn pending_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    witness: Option<Snapshot>,
) -> bool {
    let state = &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment;
    state.system == unit.system
        && state_counts_are_consistent(state)
        && latest_is_consistent(state, witness)
        && state.transition_count.checked_add(1)
            == Some(unit.calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard.transition_count)
}

pub(super) fn post_transition_state_is_consistent(
    state: &State,
    snapshot: Snapshot,
    route: Route,
    predecessor: &PredecessorState,
) -> bool {
    state
        .latest
        .is_some_and(|latest| snapshots_match_bit_exact(latest, snapshot))
        && state.latest_route == Some(route)
        && state.latest_transition_ordinal == Some(state.transition_count)
        && state_counts_are_consistent(state)
        && predecessor_counts_match(state, predecessor)
}

pub(super) fn predecessor_counts_match(
    state: &State,
    predecessor: &PredecessorState,
) -> bool {
    state.transition_count == predecessor.transition_count
        && state.predecessor_route_counts == predecessor.predecessor_route_counts
        && state.predecessor_guard_false_fallthrough_route_counts
            == predecessor.guard_false_fallthrough_route_counts
        && state.cooling_sensible_output_maximum_capacity_assignment_route_counts
            == predecessor.adjustment_body_entry_route_counts
}

pub(super) fn state_counts_are_consistent(state: &State) -> bool {
    let Some(transitions) = checked_sum(&state.predecessor_route_counts) else {
        return false;
    };
    let Some(false_count) = checked_sum(&state.predecessor_guard_false_fallthrough_route_counts)
    else {
        return false;
    };
    let Some(assignment_count) =
        checked_sum(&state.cooling_sensible_output_maximum_capacity_assignment_route_counts)
    else {
        return false;
    };
    let Some(active) = false_count.checked_add(assignment_count) else {
        return false;
    };
    let Some(inactive) = transitions.checked_sub(active) else {
        return false;
    };
    let Some(sites) = assignment_count.checked_mul(2) else {
        return false;
    };
    for index in 0..36 {
        let expected_active = if matches!(index, 4 | 7 | 10 | 13 | 16) {
            state.predecessor_route_counts[index]
        } else {
            0
        };
        if state.predecessor_guard_false_fallthrough_route_counts[index]
            .checked_add(
                state.cooling_sensible_output_maximum_capacity_assignment_route_counts[index],
            ) != Some(expected_active)
        {
            return false;
        }
    }
    let Some(w_owners) = checked_sum(&state.predecessor_route_counts[18..]) else {
        return false;
    };
    let Some(h_owners) = sum_predecessor_indices(
        &state.predecessor_route_counts,
        |index| matches!(index, 5 | 8 | 11 | 14 | 17..=29),
    ) else {
        return false;
    };
    let Some(t_owners) =
        sum_predecessor_indices(&state.predecessor_route_counts, |index| index >= 3)
    else {
        return false;
    };
    let assignment_scalars = [
        state.cp421_retained_maximum_total_cooling_capacity_owned_read_count,
        state.maximum_total_cooling_capacity_for_sensible_output_assignment_read_count,
        state.cooling_sensible_output_maximum_capacity_assignment_write_count,
    ];
    state.transition_count == transitions
        && state.inactive_transition_count == inactive
        && state.predecessor_guard_false_fallthrough_count == false_count
        && state.cooling_sensible_output_maximum_capacity_assignment_count == assignment_count
        && state.source_site_execution_count == sites
        && state.cp421_supply_humidity_ratio_state_owner_count == w_owners
        && state.unchanged_supply_humidity_ratio_preservation_count == w_owners
        && state.cp421_supply_enthalpy_state_owner_count == h_owners
        && state.unchanged_supply_enthalpy_preservation_count == h_owners
        && state.cp421_supply_temperature_state_owner_count == t_owners
        && state.unchanged_supply_temperature_preservation_count == t_owners
        && assignment_scalars
            .into_iter()
            .all(|count| count == assignment_count)
}

fn latest_is_consistent(state: &State, witness: Option<Snapshot>) -> bool {
    match (
        state.transition_count,
        state.latest,
        state.latest_route,
        state.latest_transition_ordinal,
        witness,
    ) {
        (0, None, None, None, None) => true,
        (count, Some(latest), Some(route), Some(ordinal), Some(witness)) => {
            count > 0
                && ordinal == count
                && retained_route_matches_snapshot_bounded(latest, route)
                && snapshots_match_bit_exact(latest, witness)
        }
        _ => false,
    }
}

fn checked_sum(values: &[usize]) -> Option<usize> {
    values
        .iter()
        .try_fold(0usize, |sum, value| sum.checked_add(*value))
}

fn sum_predecessor_indices(
    values: &[usize; 36],
    include: impl Fn(usize) -> bool,
) -> Option<usize> {
    let mut logical_index = 0;
    let mut total = 0usize;
    for predecessor_index in 0..30 {
        let width =
            1 + usize::from(matches!(predecessor_index, 20 | 21 | 24 | 25 | 27 | 29));
        if include(predecessor_index) {
            total = values[logical_index..logical_index + width]
                .iter()
                .try_fold(total, |sum, value| sum.checked_add(*value))?;
        }
        logical_index += width;
    }
    (logical_index == 36).then_some(total)
}
