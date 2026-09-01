//! CP436 bounded state and CP435-accounting validation.

use super::super::transition::PurchasedAirCalcHeatingOutdoorAirMaximumFlowBodyVolumeFlowAssignmentRetainedRoute as Route;
use super::super::{
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowBodyVolumeFlowAssignmentRuntimeState as State,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowBodyVolumeFlowAssignmentSnapshot as Snapshot,
};
use super::snapshot_validation::{
    retained_route_matches_prior_snapshot_bounded, retained_route_matches_snapshot_bounded,
    snapshots_match_bit_exact,
};
use crate::ideal_loads::{
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowGuardRuntimeState as PredecessorState,
    PurchasedAirUnitRuntimeState,
};

pub(super) fn pending_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    witness: Option<Snapshot>,
) -> bool {
    let state = &unit.calc_heating_outdoor_air_maximum_flow_body_volume_flow_assignment;
    state.system == unit.system
        && state_counts_are_consistent(state)
        && latest_is_consistent(state, witness)
        && state.transition_count.checked_add(1)
            == Some(unit.calc_heating_outdoor_air_maximum_flow_guard.transition_count)
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
            == predecessor.heating_outdoor_air_maximum_flow_guard_false_fallthrough_route_counts
        && state.predecessor_guard_body_entry_route_counts
            == predecessor.maximum_heating_flow_body_entry_route_counts
        && state.cp435_supply_humidity_ratio_state_owner_count
            == predecessor.unchanged_supply_humidity_ratio_preservation_count
        && state.cp435_supply_enthalpy_state_owner_count
            == predecessor.unchanged_supply_enthalpy_preservation_count
        && state.cp435_supply_temperature_state_owner_count
            == predecessor.unchanged_supply_temperature_preservation_count
}

pub(super) fn state_counts_are_consistent(state: &State) -> bool {
    let Some(transitions) = checked_sum(&state.predecessor_route_counts) else {
        return false;
    };
    let Some(fallthrough) = checked_sum(&state.predecessor_guard_false_fallthrough_route_counts)
    else {
        return false;
    };
    let Some(body) = checked_sum(&state.predecessor_guard_body_entry_route_counts) else {
        return false;
    };
    let Some(assignments) =
        checked_sum(&state.heating_outdoor_air_volume_flow_assignment_route_counts)
    else {
        return false;
    };
    for index in 0..36 {
        let false_count = state.predecessor_guard_false_fallthrough_route_counts[index];
        let body_count = state.predecessor_guard_body_entry_route_counts[index];
        let assignment = state.heating_outdoor_air_volume_flow_assignment_route_counts[index];
        let exact = if index == 1 {
            false_count.checked_add(body_count) == Some(state.predecessor_route_counts[index])
                && assignment == body_count
        } else {
            false_count == 0 && body_count == 0 && assignment == 0
        };
        if !exact {
            return false;
        }
    }
    let Some(inactive) = transitions.checked_sub(assignments) else {
        return false;
    };
    let Some(source_sites) = assignments.checked_mul(4) else {
        return false;
    };
    state.transition_count == transitions
        && fallthrough.checked_add(body) == Some(state.predecessor_route_counts[1])
        && assignments == body
        && state.inactive_transition_count == inactive
        && state.outdoor_air_volume_flow_assignment_count == assignments
        && state.source_site_execution_count == source_sites
        && state.cp435_outdoor_air_mass_flow_rate_owned_read_count == assignments
        && state.outdoor_air_mass_flow_rate_for_volume_flow_division_read_count == assignments
        && state.begin_environment_standard_air_density_owner_count == assignments
        && state.standard_air_density_for_volume_flow_division_read_count == assignments
        && state.outdoor_air_mass_flow_rate_standard_air_density_division_count == assignments
        && state.local_outdoor_air_volume_flow_rate_assignment_write_count == assignments
        && state.cp435_supply_humidity_ratio_state_owner_count
            == state.unchanged_supply_humidity_ratio_preservation_count
        && state.cp435_supply_enthalpy_state_owner_count
            == state.unchanged_supply_enthalpy_preservation_count
        && state.cp435_supply_temperature_state_owner_count
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
