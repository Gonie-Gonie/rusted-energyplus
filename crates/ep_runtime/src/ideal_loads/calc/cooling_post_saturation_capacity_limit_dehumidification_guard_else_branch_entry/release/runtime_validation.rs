//! Exact CP418 route, owner, and counter validation.

use super::super::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchEntryRuntimeState as State;
use super::super::transition::RetainedRoute;
use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyEnthalpyAssignmentRuntimeState as PredecessorState;

pub(super) fn state_counts_are_consistent(state: &State) -> bool {
    let Some(transitions) = checked_sum(&state.predecessor_route_counts) else {
        return false;
    };
    let Some(predecessor_saturation_assignments) =
        checked_sum(&state.predecessor_supply_temperature_saturation_assignment_route_counts)
    else {
        return false;
    };
    let Some(predecessor_limits) =
        checked_sum(&state.predecessor_supply_temperature_mixed_air_limit_route_counts)
    else {
        return false;
    };
    let Some(predecessor_humidity_assignments) =
        checked_sum(&state.predecessor_supply_humidity_ratio_assignment_route_counts)
    else {
        return false;
    };
    let Some(predecessor_enthalpy_assignments) =
        checked_sum(&state.predecessor_supply_enthalpy_assignment_route_counts)
    else {
        return false;
    };
    let Some(entries) = checked_sum(&state.dehumidification_guard_else_branch_entry_route_counts)
    else {
        return false;
    };
    let Some(inactive) = transitions.checked_sub(entries) else {
        return false;
    };
    let Some(humidity_ratio_owners) = checked_sum(&state.predecessor_route_counts[18..]) else {
        return false;
    };
    let Some(enthalpy_owners) = sum_predecessor_indices(&state.predecessor_route_counts, |index| {
        matches!(index, 5 | 8 | 11 | 14 | 17..=29)
    }) else {
        return false;
    };
    let Some(temperature_owners) =
        sum_predecessor_indices(&state.predecessor_route_counts, |index| index >= 3)
    else {
        return false;
    };
    for index in 0..36 {
        let Some(guard_outcomes) = state.predecessor_guard_false_fallthrough_route_counts[index]
            .checked_add(state.predecessor_guard_body_entry_route_counts[index])
        else {
            return false;
        };
        let expected_guard_outcomes = if predecessor_index_for_logical(index) >= 18 {
            state.predecessor_route_counts[index]
        } else {
            0
        };
        let expected_entry = if matches!(index, 4 | 7 | 10 | 13 | 16) {
            state.predecessor_route_counts[index]
        } else {
            0
        };
        if guard_outcomes != expected_guard_outcomes
            || state.predecessor_supply_temperature_saturation_assignment_route_counts[index]
                != state.predecessor_guard_body_entry_route_counts[index]
            || state.predecessor_supply_temperature_mixed_air_limit_route_counts[index]
                != state.predecessor_supply_temperature_saturation_assignment_route_counts[index]
            || state.predecessor_supply_humidity_ratio_assignment_route_counts[index]
                != state.predecessor_supply_temperature_mixed_air_limit_route_counts[index]
            || state.predecessor_supply_enthalpy_assignment_route_counts[index]
                != state.predecessor_supply_humidity_ratio_assignment_route_counts[index]
            || state.dehumidification_guard_else_branch_entry_route_counts[index] != expected_entry
        {
            return false;
        }
    }
    state.transition_count == transitions
        && state.inactive_transition_count == inactive
        && state.predecessor_supply_temperature_saturation_assignment_count
            == predecessor_saturation_assignments
        && state.predecessor_supply_temperature_saturation_mixed_air_limit_count
            == predecessor_limits
        && state.predecessor_supply_humidity_ratio_assignment_count
            == predecessor_humidity_assignments
        && state.predecessor_supply_enthalpy_assignment_count == predecessor_enthalpy_assignments
        && predecessor_saturation_assignments == predecessor_limits
        && predecessor_limits == predecessor_humidity_assignments
        && predecessor_humidity_assignments == predecessor_enthalpy_assignments
        && state.dehumidification_guard_else_branch_entry_count == entries
        && state.source_site_execution_count == entries
        && state.cp417_supply_humidity_ratio_state_owner_count == humidity_ratio_owners
        && state.unchanged_supply_humidity_ratio_preservation_count == humidity_ratio_owners
        && state.cp417_supply_enthalpy_state_owner_count == enthalpy_owners
        && state.unchanged_supply_enthalpy_preservation_count == enthalpy_owners
        && state.cp417_supply_temperature_state_owner_count == temperature_owners
        && state.unchanged_supply_temperature_preservation_count == temperature_owners
}

pub(super) fn pending_predecessor_counts_match(
    state: &State,
    predecessor: &PredecessorState,
    route: RetainedRoute,
) -> bool {
    let index = route.logical_index;
    if index >= 36 {
        return false;
    }
    let mut routes = state.predecessor_route_counts;
    let mut guard_false = state.predecessor_guard_false_fallthrough_route_counts;
    let mut guard_body = state.predecessor_guard_body_entry_route_counts;
    let mut saturation_assignments =
        state.predecessor_supply_temperature_saturation_assignment_route_counts;
    let mut mixed_air_limits = state.predecessor_supply_temperature_mixed_air_limit_route_counts;
    let mut humidity_assignments = state.predecessor_supply_humidity_ratio_assignment_route_counts;
    let mut enthalpy_assignments = state.predecessor_supply_enthalpy_assignment_route_counts;
    routes[index] = match routes[index].checked_add(1) {
        Some(next) => next,
        None => return false,
    };
    if route.predecessor_guard_false_fallthrough && !checked_increment(&mut guard_false[index]) {
        return false;
    }
    if route.predecessor_guard_body_entered && !checked_increment(&mut guard_body[index]) {
        return false;
    }
    if route.predecessor_saturation_temperature_assignment_executed
        && !checked_increment(&mut saturation_assignments[index])
    {
        return false;
    }
    if route.predecessor_saturation_temperature_mixed_air_limit_executed
        && !checked_increment(&mut mixed_air_limits[index])
    {
        return false;
    }
    if route.predecessor_supply_humidity_ratio_assignment_executed
        && !checked_increment(&mut humidity_assignments[index])
    {
        return false;
    }
    if route.predecessor_supply_enthalpy_assignment_executed
        && !checked_increment(&mut enthalpy_assignments[index])
    {
        return false;
    }
    routes == predecessor.predecessor_route_counts
        && guard_false == predecessor.predecessor_guard_false_fallthrough_route_counts
        && guard_body == predecessor.predecessor_guard_body_entry_route_counts
        && saturation_assignments
            == predecessor.predecessor_supply_temperature_saturation_assignment_route_counts
        && mixed_air_limits
            == predecessor.predecessor_supply_temperature_mixed_air_limit_route_counts
        && humidity_assignments
            == predecessor.predecessor_supply_humidity_ratio_assignment_route_counts
        && enthalpy_assignments == predecessor.supply_enthalpy_assignment_route_counts
}

pub(super) fn completed_predecessor_counts_match(
    state: &State,
    predecessor: &PredecessorState,
) -> bool {
    let Some(predecessor_inactive) = state
        .transition_count
        .checked_sub(state.predecessor_supply_enthalpy_assignment_count)
    else {
        return false;
    };
    let Some(predecessor_source_sites) = state
        .predecessor_supply_enthalpy_assignment_count
        .checked_mul(4)
    else {
        return false;
    };
    let Some(predecessor_unchanged_enthalpy) = state
        .cp417_supply_enthalpy_state_owner_count
        .checked_sub(state.predecessor_supply_enthalpy_assignment_count)
    else {
        return false;
    };
    state.transition_count == predecessor.transition_count
        && predecessor.inactive_transition_count == predecessor_inactive
        && state.predecessor_supply_temperature_saturation_assignment_count
            == predecessor.predecessor_supply_temperature_saturation_assignment_count
        && state.predecessor_supply_temperature_saturation_mixed_air_limit_count
            == predecessor.predecessor_supply_temperature_saturation_mixed_air_limit_count
        && state.predecessor_supply_humidity_ratio_assignment_count
            == predecessor.predecessor_supply_humidity_ratio_assignment_count
        && state.predecessor_supply_enthalpy_assignment_count
            == predecessor.supply_enthalpy_assignment_count
        && predecessor.source_site_execution_count == predecessor_source_sites
        && predecessor.cp416_supply_humidity_ratio_state_owner_count
            == state.cp417_supply_humidity_ratio_state_owner_count
        && predecessor.unchanged_supply_humidity_ratio_preservation_count
            == state.cp417_supply_humidity_ratio_state_owner_count
        && predecessor.cp416_supply_enthalpy_state_owner_count
            == state.cp417_supply_enthalpy_state_owner_count
        && predecessor.unchanged_supply_enthalpy_preservation_count
            == predecessor_unchanged_enthalpy
        && predecessor.cp416_supply_temperature_state_owner_count
            == state.cp417_supply_temperature_state_owner_count
        && predecessor.unchanged_supply_temperature_preservation_count
            == state.cp417_supply_temperature_state_owner_count
        && predecessor.cp417_psychrometric_supply_enthalpy_state_owner_count
            == state.predecessor_supply_enthalpy_assignment_count
        && predecessor.cp416_retained_supply_temperature_owned_read_count
            == state.predecessor_supply_enthalpy_assignment_count
        && predecessor.supply_temperature_for_enthalpy_read_count
            == state.predecessor_supply_enthalpy_assignment_count
        && predecessor.cp416_retained_supply_humidity_ratio_owned_read_count
            == state.predecessor_supply_enthalpy_assignment_count
        && predecessor.supply_humidity_ratio_for_enthalpy_read_count
            == state.predecessor_supply_enthalpy_assignment_count
        && predecessor.psychrometric_supply_enthalpy_evaluation_count
            == state.predecessor_supply_enthalpy_assignment_count
        && predecessor.supply_enthalpy_assignment_write_count
            == state.predecessor_supply_enthalpy_assignment_count
        && state.predecessor_route_counts == predecessor.predecessor_route_counts
        && state.predecessor_guard_false_fallthrough_route_counts
            == predecessor.predecessor_guard_false_fallthrough_route_counts
        && state.predecessor_guard_body_entry_route_counts
            == predecessor.predecessor_guard_body_entry_route_counts
        && state.predecessor_supply_temperature_saturation_assignment_route_counts
            == predecessor.predecessor_supply_temperature_saturation_assignment_route_counts
        && state.predecessor_supply_temperature_mixed_air_limit_route_counts
            == predecessor.predecessor_supply_temperature_mixed_air_limit_route_counts
        && state.predecessor_supply_humidity_ratio_assignment_route_counts
            == predecessor.predecessor_supply_humidity_ratio_assignment_route_counts
        && state.predecessor_supply_enthalpy_assignment_route_counts
            == predecessor.supply_enthalpy_assignment_route_counts
}

fn checked_increment(value: &mut usize) -> bool {
    match value.checked_add(1) {
        Some(next) => {
            *value = next;
            true
        }
        None => false,
    }
}

fn checked_sum(values: &[usize]) -> Option<usize> {
    values
        .iter()
        .try_fold(0usize, |sum, value| sum.checked_add(*value))
}

fn sum_predecessor_indices(values: &[usize; 36], include: impl Fn(usize) -> bool) -> Option<usize> {
    let mut logical_index = 0usize;
    let mut total = 0usize;
    for predecessor_index in 0..30 {
        let width = 1 + usize::from(matches!(predecessor_index, 20 | 21 | 24 | 25 | 27 | 29));
        if include(predecessor_index) {
            total = values[logical_index..logical_index + width]
                .iter()
                .try_fold(total, |sum, value| sum.checked_add(*value))?;
        }
        logical_index += width;
    }
    (logical_index == 36).then_some(total)
}

fn predecessor_index_for_logical(logical_index: usize) -> usize {
    let mut offset = 0usize;
    for predecessor_index in 0..30 {
        let width = 1 + usize::from(matches!(predecessor_index, 20 | 21 | 24 | 25 | 27 | 29));
        if logical_index < offset + width {
            return predecessor_index;
        }
        offset += width;
    }
    usize::MAX
}
