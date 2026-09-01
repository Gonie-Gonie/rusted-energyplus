//! Persistent bounded state, canonical-counter mutation, and CP437-accounting validation.

use super::super::transition::PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCounterIncrementRetainedRoute as Route;
use super::super::{
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCounterIncrementRuntimeState as State,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCounterIncrementSnapshot as Snapshot,
};
use super::snapshot_validation::{
    retained_route_matches_prior_snapshot_bounded, retained_route_matches_snapshot_bounded,
    snapshots_match_bit_exact,
};
use crate::ideal_loads::{
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardRuntimeState as PredecessorState,
    PurchasedAirUnitRuntimeState,
};

pub(super) fn pending_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    witness: Option<Snapshot>,
) -> bool {
    let state = &unit.calc_heating_outdoor_air_maximum_flow_first_warning_counter_increment;
    state.system == unit.system
        && state_counts_are_consistent(state)
        && latest_is_consistent(state, witness)
        && state.transition_count.checked_add(1)
            == Some(
                unit.calc_heating_outdoor_air_maximum_flow_first_warning_guard
                    .transition_count,
            )
}

pub(super) fn post_transition_state_is_consistent(
    state: &State,
    snapshot: Snapshot,
    route: Route,
    next_counter_owner: &PredecessorState,
    previous_counter_owner: &PredecessorState,
) -> bool {
    state
        .latest
        .is_some_and(|latest| snapshots_match_bit_exact(latest, snapshot))
        && state.latest_route == Some(route)
        && state.latest_transition_ordinal == Some(state.transition_count)
        && retained_route_matches_snapshot_bounded(snapshot, route)
        && state_counts_are_consistent(state)
        && predecessor_counts_match(state, next_counter_owner)
        && canonical_counter_mutation_is_exact(
            snapshot,
            route,
            next_counter_owner,
            previous_counter_owner,
        )
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
            == predecessor
                .heating_outdoor_air_maximum_flow_first_warning_guard_false_fallthrough_route_counts
        && state.predecessor_first_warning_branch_entry_route_counts
            == predecessor.heating_outdoor_air_maximum_flow_first_warning_branch_entry_route_counts
        && state.heating_outdoor_air_maximum_flow_first_warning_counter_increment_route_counts
            == predecessor.heating_outdoor_air_maximum_flow_first_warning_branch_entry_route_counts
        && state.cp437_supply_humidity_ratio_state_owner_count
            == predecessor.unchanged_supply_humidity_ratio_preservation_count
        && state.cp437_supply_enthalpy_state_owner_count
            == predecessor.unchanged_supply_enthalpy_preservation_count
        && state.cp437_supply_temperature_state_owner_count
            == predecessor.unchanged_supply_temperature_preservation_count
}

pub(super) fn state_counts_are_consistent(state: &State) -> bool {
    let Some(transitions) = checked_sum(&state.predecessor_route_counts) else {
        return false;
    };
    let Some(inherited_fallthrough) =
        checked_sum(&state.predecessor_guard_false_fallthrough_route_counts)
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
    let Some(guard_false_fallthroughs) =
        checked_sum(&state.predecessor_first_warning_guard_false_fallthrough_route_counts)
    else {
        return false;
    };
    let Some(branch_entries) =
        checked_sum(&state.predecessor_first_warning_branch_entry_route_counts)
    else {
        return false;
    };
    let Some(increments) = checked_sum(
        &state.heating_outdoor_air_maximum_flow_first_warning_counter_increment_route_counts,
    ) else {
        return false;
    };
    for index in 0..36 {
        let inherited_false = state.predecessor_guard_false_fallthrough_route_counts[index];
        let inherited_body = state.predecessor_guard_body_entry_route_counts[index];
        let assignment = state.predecessor_volume_flow_assignment_route_counts[index];
        let guard_false =
            state.predecessor_first_warning_guard_false_fallthrough_route_counts[index];
        let branch = state.predecessor_first_warning_branch_entry_route_counts[index];
        let increment = state
            .heating_outdoor_air_maximum_flow_first_warning_counter_increment_route_counts[index];
        let predecessor_exact = if index == 1 {
            inherited_false.checked_add(inherited_body)
                == Some(state.predecessor_route_counts[index])
                && assignment == inherited_body
        } else {
            inherited_false == 0 && inherited_body == 0 && assignment == 0
        };
        if !predecessor_exact
            || guard_false.checked_add(branch) != Some(assignment)
            || increment != branch
        {
            return false;
        }
    }
    let Some(inactive) = transitions.checked_sub(increments) else {
        return false;
    };
    state.transition_count == transitions
        && inherited_fallthrough.checked_add(inherited_body)
            == Some(state.predecessor_route_counts[1])
        && assignments == inherited_body
        && branch_entries.checked_add(guard_false_fallthroughs) == Some(assignments)
        && increments == branch_entries
        && state.inactive_transition_count == inactive
        && state.outdoor_air_flow_maximum_heating_output_error_count_increment_count == increments
        && state.source_site_execution_count == increments
        && state.cp437_outdoor_air_flow_maximum_heating_output_error_count_state_owner_count
            == increments
        && state.outdoor_air_flow_maximum_heating_output_error_count_increment_write_count
            == increments
        && state.cp437_supply_humidity_ratio_state_owner_count
            == state.unchanged_supply_humidity_ratio_preservation_count
        && state.cp437_supply_enthalpy_state_owner_count
            == state.unchanged_supply_enthalpy_preservation_count
        && state.cp437_supply_temperature_state_owner_count
            == state.unchanged_supply_temperature_preservation_count
}

fn canonical_counter_mutation_is_exact(
    snapshot: Snapshot,
    route: Route,
    next: &PredecessorState,
    previous: &PredecessorState,
) -> bool {
    let mut expected = previous.clone();
    if route.counter_increment_executed {
        if previous.outdoor_air_flow_maximum_heating_output_error_count != 0
            || snapshot.assigned_outdoor_air_flow_maximum_heating_output_error_count != Some(1)
        {
            return false;
        }
        expected.outdoor_air_flow_maximum_heating_output_error_count = 1;
    } else if snapshot
        .assigned_outdoor_air_flow_maximum_heating_output_error_count
        .is_some()
    {
        return false;
    }
    next == &expected
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
