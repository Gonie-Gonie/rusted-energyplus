//! Persistent CP439 state and CP438-accounting validation.

use super::super::transition::PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCallRetainedRoute as Route;
use super::super::{
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCallRuntimeState as State,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCallSnapshot as Snapshot,
};
use super::snapshot_validation::{
    retained_route_matches_prior_snapshot_bounded, retained_route_matches_snapshot_bounded,
    snapshots_match_bit_exact,
};
use crate::ideal_loads::{
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCounterIncrementRuntimeState as PredecessorState,
    PurchasedAirUnitRuntimeState,
};

pub(super) fn pending_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    witness: Option<Snapshot>,
) -> bool {
    let state = &unit.calc_heating_outdoor_air_maximum_flow_first_warning_call;
    state.system == unit.system
        && state_counts_are_consistent(state)
        && latest_is_consistent(state, witness)
        && state.transition_count.checked_add(1)
            == Some(
                unit.calc_heating_outdoor_air_maximum_flow_first_warning_counter_increment
                    .transition_count,
            )
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
        && retained_route_matches_snapshot_bounded(snapshot, route)
        && state_counts_are_consistent(state)
        && predecessor_counts_match(state, predecessor)
}

pub(super) fn predecessor_counts_match(state: &State, predecessor: &PredecessorState) -> bool {
    state.transition_count == predecessor.transition_count
        && state.predecessor_route_counts == predecessor.predecessor_route_counts
        && state.predecessor_guard_false_fallthrough_route_counts
            == predecessor.predecessor_guard_false_fallthrough_route_counts
        && state.predecessor_guard_body_entry_route_counts
            == predecessor.predecessor_guard_body_entry_route_counts
        && state.predecessor_volume_flow_assignment_route_counts
            == predecessor.predecessor_volume_flow_assignment_route_counts
        && state.predecessor_first_warning_guard_false_fallthrough_route_counts
            == predecessor.predecessor_first_warning_guard_false_fallthrough_route_counts
        && state.predecessor_first_warning_branch_entry_route_counts
            == predecessor.predecessor_first_warning_branch_entry_route_counts
        && state.predecessor_first_warning_counter_increment_route_counts
            == predecessor
                .heating_outdoor_air_maximum_flow_first_warning_counter_increment_route_counts
        && state.heating_outdoor_air_maximum_flow_first_warning_call_route_counts
            == predecessor
                .heating_outdoor_air_maximum_flow_first_warning_counter_increment_route_counts
        && state.cp438_supply_humidity_ratio_state_owner_count
            == predecessor.unchanged_supply_humidity_ratio_preservation_count
        && state.cp438_supply_enthalpy_state_owner_count
            == predecessor.unchanged_supply_enthalpy_preservation_count
        && state.cp438_supply_temperature_state_owner_count
            == predecessor.unchanged_supply_temperature_preservation_count
}

pub(super) fn state_counts_are_consistent(state: &State) -> bool {
    let Some(transitions) = checked_sum(&state.predecessor_route_counts) else {
        return false;
    };
    let Some(inherited_false) = checked_sum(&state.predecessor_guard_false_fallthrough_route_counts)
    else {
        return false;
    };
    let Some(inherited_body) = checked_sum(&state.predecessor_guard_body_entry_route_counts) else {
        return false;
    };
    let Some(assignments) = checked_sum(&state.predecessor_volume_flow_assignment_route_counts)
    else {
        return false;
    };
    let Some(guard_false) =
        checked_sum(&state.predecessor_first_warning_guard_false_fallthrough_route_counts)
    else {
        return false;
    };
    let Some(branches) = checked_sum(&state.predecessor_first_warning_branch_entry_route_counts)
    else {
        return false;
    };
    let Some(increments) =
        checked_sum(&state.predecessor_first_warning_counter_increment_route_counts)
    else {
        return false;
    };
    let Some(calls) =
        checked_sum(&state.heating_outdoor_air_maximum_flow_first_warning_call_route_counts)
    else {
        return false;
    };
    for index in 0..36 {
        let false_route = state.predecessor_guard_false_fallthrough_route_counts[index];
        let body = state.predecessor_guard_body_entry_route_counts[index];
        let assignment = state.predecessor_volume_flow_assignment_route_counts[index];
        let first_false =
            state.predecessor_first_warning_guard_false_fallthrough_route_counts[index];
        let first_entry = state.predecessor_first_warning_branch_entry_route_counts[index];
        let increment = state.predecessor_first_warning_counter_increment_route_counts[index];
        let call = state.heating_outdoor_air_maximum_flow_first_warning_call_route_counts[index];
        let predecessor_exact = if index == 1 {
            false_route.checked_add(body) == Some(state.predecessor_route_counts[index])
                && assignment == body
        } else {
            false_route == 0 && body == 0 && assignment == 0
        };
        if !predecessor_exact
            || first_false.checked_add(first_entry) != Some(assignment)
            || increment != first_entry
            || call != increment
        {
            return false;
        }
    }
    let Some(inactive) = transitions.checked_sub(calls) else {
        return false;
    };
    state.transition_count == transitions
        && inherited_false.checked_add(inherited_body) == Some(state.predecessor_route_counts[1])
        && assignments == inherited_body
        && guard_false.checked_add(branches) == Some(assignments)
        && increments == branches
        && calls == increments
        && state.inactive_transition_count == inactive
        && state.heating_outdoor_air_maximum_flow_first_warning_call_site_count == calls
        && state.source_site_execution_count == calls
        && state.cp438_outdoor_air_flow_maximum_heating_output_error_count_state_owner_count
            == calls
        && state
            .unchanged_outdoor_air_flow_maximum_heating_output_error_count_preservation_count
            == calls
        && state.cp438_supply_humidity_ratio_state_owner_count
            == state.unchanged_supply_humidity_ratio_preservation_count
        && state.cp438_supply_enthalpy_state_owner_count
            == state.unchanged_supply_enthalpy_preservation_count
        && state.cp438_supply_temperature_state_owner_count
            == state.unchanged_supply_temperature_preservation_count
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
