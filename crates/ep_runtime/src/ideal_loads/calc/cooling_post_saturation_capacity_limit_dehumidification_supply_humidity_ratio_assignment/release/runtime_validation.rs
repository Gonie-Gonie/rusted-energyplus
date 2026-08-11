//! Exact CP416 route, owner, and counter validation.

use super::super::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioAssignmentRuntimeState as State;
use super::super::transition::RetainedRoute;
use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationMixedAirLimitRuntimeState as PredecessorState;

pub(super) fn state_counts_are_consistent(state: &State) -> bool {
    let Some(transitions) = checked_sum(&state.predecessor_route_counts) else {
        return false;
    };
    let Some(predecessor_assignments) =
        checked_sum(&state.predecessor_supply_temperature_saturation_assignment_route_counts)
    else {
        return false;
    };
    let Some(predecessor_limits) =
        checked_sum(&state.predecessor_supply_temperature_mixed_air_limit_route_counts)
    else {
        return false;
    };
    let Some(assignments) = checked_sum(&state.supply_humidity_ratio_assignment_route_counts)
    else {
        return false;
    };
    let Some(inactive) = transitions.checked_sub(assignments) else {
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
    let Some(unchanged_humidity_ratio) = humidity_ratio_owners.checked_sub(assignments) else {
        return false;
    };
    let Some(source_sites) = assignments.checked_mul(4) else {
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
        if guard_outcomes != expected_guard_outcomes
            || state.predecessor_supply_temperature_saturation_assignment_route_counts[index]
                != state.predecessor_guard_body_entry_route_counts[index]
            || state.predecessor_supply_temperature_mixed_air_limit_route_counts[index]
                != state.predecessor_supply_temperature_saturation_assignment_route_counts[index]
            || state.supply_humidity_ratio_assignment_route_counts[index]
                != state.predecessor_supply_temperature_mixed_air_limit_route_counts[index]
        {
            return false;
        }
    }
    state.transition_count == transitions
        && state.inactive_transition_count == inactive
        && state.predecessor_supply_temperature_saturation_assignment_count
            == predecessor_assignments
        && state.predecessor_supply_temperature_saturation_mixed_air_limit_count
            == predecessor_limits
        && state.supply_humidity_ratio_assignment_count == assignments
        && predecessor_assignments == predecessor_limits
        && predecessor_limits == assignments
        && state.source_site_execution_count == source_sites
        && state.cp415_supply_humidity_ratio_state_owner_count == humidity_ratio_owners
        && state.unchanged_supply_humidity_ratio_preservation_count == unchanged_humidity_ratio
        && state.cp415_supply_enthalpy_state_owner_count == enthalpy_owners
        && state.unchanged_supply_enthalpy_preservation_count == enthalpy_owners
        && state.cp415_supply_temperature_state_owner_count == temperature_owners
        && state.unchanged_supply_temperature_preservation_count == temperature_owners
        && state.cp416_psychrometric_supply_humidity_ratio_state_owner_count == assignments
        && state.cp415_retained_supply_temperature_owned_read_count == assignments
        && state.supply_temperature_for_humidity_ratio_inversion_read_count == assignments
        && state.cp415_retained_supply_enthalpy_owned_read_count == assignments
        && state.supply_enthalpy_for_humidity_ratio_inversion_read_count == assignments
        && state.psychrometric_supply_humidity_ratio_evaluation_count == assignments
        && state.supply_humidity_ratio_assignment_write_count == assignments
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
    routes[index] = match routes[index].checked_add(1) {
        Some(next) => next,
        None => return false,
    };
    if route.predecessor_guard_false_fallthrough {
        guard_false[index] = match guard_false[index].checked_add(1) {
            Some(next) => next,
            None => return false,
        };
    }
    if route.predecessor_guard_body_entered {
        guard_body[index] = match guard_body[index].checked_add(1) {
            Some(next) => next,
            None => return false,
        };
    }
    if route.predecessor_saturation_temperature_assignment_executed {
        saturation_assignments[index] = match saturation_assignments[index].checked_add(1) {
            Some(next) => next,
            None => return false,
        };
    }
    if route.predecessor_saturation_temperature_mixed_air_limit_executed {
        mixed_air_limits[index] = match mixed_air_limits[index].checked_add(1) {
            Some(next) => next,
            None => return false,
        };
    }
    routes == predecessor.predecessor_route_counts
        && guard_false == predecessor.predecessor_guard_false_fallthrough_route_counts
        && guard_body == predecessor.predecessor_guard_body_entry_route_counts
        && saturation_assignments
            == predecessor.predecessor_supply_temperature_saturation_assignment_route_counts
        && mixed_air_limits == predecessor.supply_temperature_mixed_air_limit_route_counts
}

pub(super) fn completed_predecessor_counts_match(
    state: &State,
    predecessor: &PredecessorState,
) -> bool {
    state.predecessor_route_counts == predecessor.predecessor_route_counts
        && state.predecessor_guard_false_fallthrough_route_counts
            == predecessor.predecessor_guard_false_fallthrough_route_counts
        && state.predecessor_guard_body_entry_route_counts
            == predecessor.predecessor_guard_body_entry_route_counts
        && state.predecessor_supply_temperature_saturation_assignment_route_counts
            == predecessor.predecessor_supply_temperature_saturation_assignment_route_counts
        && state.predecessor_supply_temperature_mixed_air_limit_route_counts
            == predecessor.supply_temperature_mixed_air_limit_route_counts
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
