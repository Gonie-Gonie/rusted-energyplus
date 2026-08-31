//! CP435 bounded state and CP434-accounting validation.

use super::super::transition::PurchasedAirCalcHeatingOutdoorAirMaximumFlowGuardRetainedRoute as Route;
use super::super::{
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowGuardRuntimeState as State,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowGuardSnapshot as Snapshot,
};
use super::snapshot_validation::{
    retained_route_matches_prior_snapshot_bounded, retained_route_matches_snapshot_bounded,
    snapshots_match_bit_exact,
};
use crate::ideal_loads::{
    PurchasedAirCalcHeatingOperatingModeDeadbandAssignmentRuntimeState as PredecessorState,
    PurchasedAirUnitRuntimeState,
};

pub(super) fn pending_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    witness: Option<Snapshot>,
) -> bool {
    let state = &unit.calc_heating_outdoor_air_maximum_flow_guard;
    state.system == unit.system
        && state_counts_are_consistent(state)
        && latest_is_consistent(state, witness)
        && state.transition_count.checked_add(1)
            == Some(unit.calc_heating_operating_mode_deadband_assignment.transition_count)
}

pub(super) fn post_transition_state_is_consistent(
    state: &State,
    snapshot: Snapshot,
    route: Route,
    predecessor: &PredecessorState,
) -> bool {
    state.latest.is_some_and(|latest| snapshots_match_bit_exact(latest, snapshot))
        && state.latest_route == Some(route)
        && state.latest_transition_ordinal == Some(state.transition_count)
        && retained_route_matches_snapshot_bounded(snapshot, route)
        && state_counts_are_consistent(state)
        && predecessor_counts_match(state, predecessor)
}

pub(super) fn predecessor_counts_match(state: &State, predecessor: &PredecessorState) -> bool {
    state.transition_count == predecessor.transition_count
        && state.predecessor_route_counts == predecessor.predecessor_route_counts
        && state.cp434_supply_humidity_ratio_state_owner_count
            == predecessor.cp433_supply_humidity_ratio_state_owner_count
        && state.cp434_supply_enthalpy_state_owner_count
            == predecessor.cp433_supply_enthalpy_state_owner_count
        && state.cp434_supply_temperature_state_owner_count
            == predecessor.cp433_supply_temperature_state_owner_count
}

pub(super) fn state_counts_are_consistent(state: &State) -> bool {
    let Some(transitions) = checked_sum(&state.predecessor_route_counts) else {
        return false;
    };
    let Some(body) = checked_sum(&state.maximum_heating_flow_body_entry_route_counts) else {
        return false;
    };
    let Some(fallthrough) =
        checked_sum(&state.heating_outdoor_air_maximum_flow_guard_false_fallthrough_route_counts)
    else {
        return false;
    };
    for index in 0..36 {
        let body = state.maximum_heating_flow_body_entry_route_counts[index];
        let fallthrough =
            state.heating_outdoor_air_maximum_flow_guard_false_fallthrough_route_counts[index];
        let exact = if index == 1 {
            body.checked_add(fallthrough) == Some(state.predecessor_route_counts[index])
        } else {
            body == 0 && fallthrough == 0
        };
        if !exact {
            return false;
        }
    }
    let Some(active) = transitions.checked_sub(state.inactive_transition_count) else {
        return false;
    };
    let Some(evaluated_branches) = body.checked_add(fallthrough) else {
        return false;
    };
    let Some(selector_matches) = state
        .heating_limit_flow_rate_match_count
        .checked_add(state.heating_limit_flow_rate_and_capacity_match_count)
    else {
        return false;
    };
    let Some(source_sites) = state
        .heating_limit_flow_rate_comparison_count
        .checked_add(state.heating_limit_flow_rate_and_capacity_comparison_count)
        .and_then(|count| {
            selector_matches
                .checked_mul(3)
                .and_then(|reads| count.checked_add(reads))
        })
        .and_then(|count| count.checked_add(body))
    else {
        return false;
    };
    state.transition_count == transitions
        && state.heating_outdoor_air_maximum_flow_guard_evaluation_count == active
        && active == state.predecessor_route_counts[1]
        && state.inactive_transition_count
            == transitions.saturating_sub(state.predecessor_route_counts[1])
        && state.heating_limit_flow_rate_comparison_count == active
        && state.heating_limit_flow_rate_match_count
            .checked_add(state.heating_limit_flow_rate_and_capacity_comparison_count)
            == Some(active)
        && state.heating_limit_flow_rate_and_capacity_match_count
            .checked_add(state.heating_flow_limit_selector_rejection_count)
            == Some(state.heating_limit_flow_rate_and_capacity_comparison_count)
        && evaluated_branches == active
        && selector_matches
            .checked_add(state.heating_flow_limit_selector_rejection_count)
            == Some(active)
        && state.cp311_same_call_outdoor_air_mass_flow_rate_bit_corroboration_count
            == selector_matches
        && state.outdoor_air_mass_flow_rate_read_after_heating_limit_short_circuit_count
            == selector_matches
        && state.maximum_heating_air_mass_flow_rate_read_after_heating_limit_short_circuit_count
            == selector_matches
        && state.outdoor_air_mass_flow_rate_maximum_heating_air_mass_flow_rate_comparison_count
            == selector_matches
        && state.outdoor_air_mass_flow_rate_strictly_greater_than_maximum_heating_air_mass_flow_rate_count
            == body
        && state.maximum_heating_flow_body_entry_count == body
        && state.heating_outdoor_air_maximum_flow_guard_false_fallthrough_count == fallthrough
        && state.source_site_execution_count == source_sites
        && state.cp434_supply_humidity_ratio_state_owner_count
            == state.unchanged_supply_humidity_ratio_preservation_count
        && state.cp434_supply_enthalpy_state_owner_count
            == state.unchanged_supply_enthalpy_preservation_count
        && state.cp434_supply_temperature_state_owner_count
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
    values.iter().try_fold(0usize, |sum, value| sum.checked_add(*value))
}
