//! CP432 bounded state and CP431-accounting validation.

use super::super::{
    PurchasedAirCalcHeatingOperatingModeHeatAssignmentRuntimeState as State,
    PurchasedAirCalcHeatingOperatingModeHeatAssignmentSnapshot as Snapshot,
};
use super::super::transition::PurchasedAirCalcHeatingOperatingModeHeatAssignmentRetainedRoute as Route;
use super::snapshot_validation::{
    retained_route_matches_prior_snapshot_bounded,
    retained_route_matches_snapshot_bounded,
    snapshots_match_bit_exact,
};
use crate::ideal_loads::calc::PurchasedAirCalcHeatingModeGuardRetainedRoute as PredecessorRoute;
use crate::ideal_loads::{
    PurchasedAirCalcHeatingModeGuardRuntimeState as PredecessorState,
    PurchasedAirUnitRuntimeState,
};

pub(super) fn pending_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    witness: Option<Snapshot>,
) -> bool {
    let state = &unit.calc_heating_operating_mode_heat_assignment;
    state.system == unit.system
        && state_counts_are_consistent(state)
        && latest_is_consistent(state, witness)
        && state.transition_count.checked_add(1)
            == Some(unit.calc_heating_mode_guard.transition_count)
}

pub(super) fn post_transition_state_is_consistent(
    state: &State,
    snapshot: Snapshot,
    predecessor_route: PredecessorRoute,
    route: Route,
    predecessor: &PredecessorState,
) -> bool {
    state
        .latest
        .is_some_and(|latest| snapshots_match_bit_exact(latest, snapshot))
        && state.latest_route == Some(route)
        && state.latest_transition_ordinal == Some(state.transition_count)
        && retained_route_matches_snapshot_bounded(snapshot, predecessor_route, route)
        && state_counts_are_consistent(state)
        && predecessor_counts_match(state, predecessor)
}

pub(super) fn predecessor_counts_match(state: &State, predecessor: &PredecessorState) -> bool {
    state.transition_count == predecessor.transition_count
        && state.predecessor_route_counts == predecessor.predecessor_route_counts
        && state.predecessor_heating_mode_guard_evaluation_route_counts
            == predecessor.heating_mode_guard_evaluation_route_counts
        && state.predecessor_heating_mode_guard_false_fallthrough_route_counts
            == predecessor.heating_mode_guard_false_fallthrough_route_counts
        && state.heating_operating_mode_heat_assignment_route_counts
            == predecessor.heating_operating_mode_body_entry_route_counts
}

pub(super) fn state_counts_are_consistent(state: &State) -> bool {
    let Some(transitions) = checked_sum(&state.predecessor_route_counts) else {
        return false;
    };
    let Some(evaluations) =
        checked_sum(&state.predecessor_heating_mode_guard_evaluation_route_counts)
    else {
        return false;
    };
    let Some(fallthroughs) =
        checked_sum(&state.predecessor_heating_mode_guard_false_fallthrough_route_counts)
    else {
        return false;
    };
    let Some(assignments) =
        checked_sum(&state.heating_operating_mode_heat_assignment_route_counts)
    else {
        return false;
    };
    let Some(inactive) = transitions.checked_sub(evaluations) else {
        return false;
    };
    for index in 0..36 {
        if index != 1
            && (state.predecessor_heating_mode_guard_evaluation_route_counts[index] != 0
                || state.predecessor_heating_mode_guard_false_fallthrough_route_counts[index] != 0
                || state.heating_operating_mode_heat_assignment_route_counts[index] != 0)
        {
            return false;
        }
        let Some(local_total) = state
            .predecessor_heating_mode_guard_false_fallthrough_route_counts[index]
            .checked_add(state.heating_operating_mode_heat_assignment_route_counts[index])
        else {
            return false;
        };
        if state.predecessor_heating_mode_guard_evaluation_route_counts[index] != local_total {
            return false;
        }
    }
    let Some(w_owners) = checked_sum(&state.predecessor_route_counts[18..])
        .and_then(|count| count.checked_add(state.predecessor_route_counts[2]))
    else {
        return false;
    };
    let Some(h_owners) = sum_predecessor_indices(
        &state.predecessor_route_counts,
        |index| index == 2 || matches!(index, 5 | 8 | 11 | 14 | 17..=29),
    ) else {
        return false;
    };
    let Some(t_owners) =
        sum_predecessor_indices(&state.predecessor_route_counts, |index| index >= 2)
    else {
        return false;
    };
    state.transition_count == transitions
        && state.inactive_transition_count == inactive
        && state.predecessor_heating_mode_guard_evaluation_count == evaluations
        && state.predecessor_heating_mode_guard_false_fallthrough_count == fallthroughs
        && state.heating_operating_mode_heat_assignment_count == assignments
        && fallthroughs.checked_add(assignments) == Some(evaluations)
        && state.source_site_execution_count == assignments
        && state.cp431_supply_humidity_ratio_state_owner_count == w_owners
        && state.unchanged_supply_humidity_ratio_preservation_count == w_owners
        && state.cp431_supply_enthalpy_state_owner_count == h_owners
        && state.unchanged_supply_enthalpy_preservation_count == h_owners
        && state.cp431_supply_temperature_state_owner_count == t_owners
        && state.unchanged_supply_temperature_preservation_count == t_owners
        && state.cp432_heating_operating_mode_state_owner_count == assignments
        && state.heating_operating_mode_assignment_write_count == assignments
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
                && retained_route_matches_prior_snapshot_bounded(latest, route)
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
