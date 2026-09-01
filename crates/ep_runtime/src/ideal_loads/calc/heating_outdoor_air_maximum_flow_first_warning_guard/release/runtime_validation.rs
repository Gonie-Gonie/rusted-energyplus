//! Persistent bounded state and CP436-accounting validation.

use super::super::transition::PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardRetainedRoute as Route;
use super::super::{
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardRuntimeState as State,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardSnapshot as Snapshot,
};
use super::snapshot_validation::{
    retained_route_matches_prior_snapshot_bounded, retained_route_matches_snapshot_bounded,
    snapshots_match_bit_exact,
};
use crate::ideal_loads::{
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowBodyVolumeFlowAssignmentRuntimeState as PredecessorState,
    PurchasedAirUnitRuntimeState,
};

pub(super) fn pending_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    witness: Option<Snapshot>,
) -> bool {
    let state = &unit.calc_heating_outdoor_air_maximum_flow_first_warning_guard;
    state.system == unit.system
        && state.outdoor_air_flow_maximum_heating_output_error_count == 0
        && state_counts_are_consistent(state)
        && latest_is_consistent(state, witness)
        && state.transition_count.checked_add(1)
            == Some(
                unit.calc_heating_outdoor_air_maximum_flow_body_volume_flow_assignment
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
        && state.outdoor_air_flow_maximum_heating_output_error_count == 0
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
            == predecessor.heating_outdoor_air_volume_flow_assignment_route_counts
        && state.cp436_supply_humidity_ratio_state_owner_count
            == predecessor.unchanged_supply_humidity_ratio_preservation_count
        && state.cp436_supply_enthalpy_state_owner_count
            == predecessor.unchanged_supply_enthalpy_preservation_count
        && state.cp436_supply_temperature_state_owner_count
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
    let Some(false_fallthroughs) = checked_sum(
        &state.heating_outdoor_air_maximum_flow_first_warning_guard_false_fallthrough_route_counts,
    ) else {
        return false;
    };
    let Some(branch_entries) = checked_sum(
        &state.heating_outdoor_air_maximum_flow_first_warning_branch_entry_route_counts,
    ) else {
        return false;
    };
    for index in 0..36 {
        let inherited_false = state.predecessor_guard_false_fallthrough_route_counts[index];
        let inherited_body = state.predecessor_guard_body_entry_route_counts[index];
        let assignment = state.predecessor_volume_flow_assignment_route_counts[index];
        let guard_false = state
            .heating_outdoor_air_maximum_flow_first_warning_guard_false_fallthrough_route_counts
            [index];
        let branch =
            state.heating_outdoor_air_maximum_flow_first_warning_branch_entry_route_counts[index];
        let predecessor_exact = if index == 1 {
            inherited_false.checked_add(inherited_body)
                == Some(state.predecessor_route_counts[index])
                && assignment == inherited_body
        } else {
            inherited_false == 0 && inherited_body == 0 && assignment == 0
        };
        if !predecessor_exact || guard_false.checked_add(branch) != Some(assignment) {
            return false;
        }
    }
    let Some(inactive) = transitions.checked_sub(assignments) else {
        return false;
    };
    let Some(source_sites) = assignments
        .checked_mul(2)
        .and_then(|reads_and_comparisons| reads_and_comparisons.checked_add(branch_entries))
    else {
        return false;
    };
    state.transition_count == transitions
        && inherited_fallthrough.checked_add(inherited_body)
            == Some(state.predecessor_route_counts[1])
        && assignments == inherited_body
        && state.inactive_transition_count == inactive
        && state.guard_evaluation_count == assignments
        && state.first_warning_branch_entry_count == branch_entries
        && state.guard_false_fallthrough_count == false_fallthroughs
        && branch_entries.checked_add(false_fallthroughs) == Some(assignments)
        && state.source_site_execution_count == source_sites
        && state.outdoor_air_flow_maximum_heating_output_error_count_state_owner_count
            == assignments
        && state.outdoor_air_flow_maximum_heating_output_error_count_read_count == assignments
        && state.outdoor_air_flow_maximum_heating_output_error_count_less_than_one_comparison_count
            == assignments
        && state.cp436_supply_humidity_ratio_state_owner_count
            == state.unchanged_supply_humidity_ratio_preservation_count
        && state.cp436_supply_enthalpy_state_owner_count
            == state.unchanged_supply_enthalpy_preservation_count
        && state.cp436_supply_temperature_state_owner_count
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
