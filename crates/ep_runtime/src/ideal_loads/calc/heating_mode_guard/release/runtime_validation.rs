//! CP431 bounded state and CP430-accounting validation.

use super::super::{
    PurchasedAirCalcHeatingModeGuardRuntimeState as State,
    PurchasedAirCalcHeatingModeGuardSnapshot as Snapshot,
};
use super::super::transition::PurchasedAirCalcHeatingModeGuardRetainedRoute as Route;
use super::snapshot_validation::{
    retained_route_matches_prior_snapshot_bounded,
    retained_route_matches_snapshot_bounded, snapshots_match_bit_exact,
};
use crate::ideal_loads::calc::PurchasedAirCalcHeatingOrNoLoadCaseEntryRetainedRoute as PredecessorRoute;
use crate::ideal_loads::{
    PurchasedAirCalcHeatingOrNoLoadCaseEntryRuntimeState as PredecessorState,
    PurchasedAirUnitRuntimeState,
};

pub(super) fn pending_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    witness: Option<Snapshot>,
) -> bool {
    let state = &unit.calc_heating_mode_guard;
    state.system == unit.system
        && state_counts_are_consistent(state)
        && latest_is_consistent(state, witness)
        && state.transition_count.checked_add(1)
            == Some(unit.calc_heating_or_no_load_case_entry.transition_count)
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

pub(super) fn predecessor_counts_match(
    state: &State,
    predecessor: &PredecessorState,
) -> bool {
    state.transition_count == predecessor.transition_count
        && state.predecessor_route_counts == predecessor.predecessor_route_counts
}

pub(super) fn state_counts_are_consistent(state: &State) -> bool {
    let Some(transitions) = checked_sum(&state.predecessor_route_counts) else {
        return false;
    };
    let Some(evaluations) = checked_sum(&state.heating_mode_guard_evaluation_route_counts) else {
        return false;
    };
    let Some(body_entries) = checked_sum(&state.heating_operating_mode_body_entry_route_counts)
    else {
        return false;
    };
    let Some(fallthroughs) = checked_sum(&state.heating_mode_guard_false_fallthrough_route_counts)
    else {
        return false;
    };
    let Some(inactive) = transitions.checked_sub(evaluations) else {
        return false;
    };
    for index in 0..36 {
        if index != 1
            && (state.heating_mode_guard_evaluation_route_counts[index] != 0
                || state.heating_operating_mode_body_entry_route_counts[index] != 0
                || state.heating_mode_guard_false_fallthrough_route_counts[index] != 0)
        {
            return false;
        }
        let Some(local_total) = state.heating_operating_mode_body_entry_route_counts[index]
            .checked_add(state.heating_mode_guard_false_fallthrough_route_counts[index])
        else {
            return false;
        };
        if state.heating_mode_guard_evaluation_route_counts[index] != local_total {
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
    let active_owners_match = [
        state.cp311_retained_minimum_outdoor_air_sensible_output_owner_read_count,
        state.cp312_same_call_minimum_outdoor_air_sensible_output_bit_corroboration_count,
        state.minimum_outdoor_air_sensible_output_for_heating_mode_guard_read_count,
        state.cp310_retained_heating_setpoint_demand_owner_read_count,
        state.heating_setpoint_demand_for_heating_mode_guard_read_count,
        state.minimum_outdoor_air_sensible_output_heating_setpoint_demand_comparison_count,
    ]
    .into_iter()
    .all(|count| count == evaluations);
    let short_circuit =
        state.minimum_outdoor_air_sensible_output_strictly_less_than_heating_setpoint_demand_count;
    let short_circuit_counts_match = [
        state.prevalidated_temperature_control_type_owner_read_count,
        state.temperature_control_type_read_after_sensible_comparison_short_circuit_count,
        state.temperature_control_type_single_cool_comparison_count,
    ]
    .into_iter()
    .all(|count| count == short_circuit);
    let Some(expected_sites) = evaluations
        .checked_mul(3)
        .and_then(|count| short_circuit.checked_mul(2).and_then(|extra| count.checked_add(extra)))
        .and_then(|count| count.checked_add(body_entries))
    else {
        return false;
    };
    state.transition_count == transitions
        && state.inactive_transition_count == inactive
        && state.heating_mode_guard_evaluation_count == evaluations
        && state.heating_operating_mode_body_entry_count == body_entries
        && state.heating_mode_guard_false_fallthrough_count == fallthroughs
        && body_entries.checked_add(fallthroughs) == Some(evaluations)
        && state.source_site_execution_count == expected_sites
        && active_owners_match
        && short_circuit_counts_match
        && state.temperature_control_type_permits_heating_count == body_entries
        && state.single_cool_block_count.checked_add(body_entries) == Some(short_circuit)
        && state.cp430_supply_humidity_ratio_state_owner_count == w_owners
        && state.unchanged_supply_humidity_ratio_preservation_count == w_owners
        && state.cp430_supply_enthalpy_state_owner_count == h_owners
        && state.unchanged_supply_enthalpy_preservation_count == h_owners
        && state.cp430_supply_temperature_state_owner_count == t_owners
        && state.unchanged_supply_temperature_preservation_count == t_owners
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
