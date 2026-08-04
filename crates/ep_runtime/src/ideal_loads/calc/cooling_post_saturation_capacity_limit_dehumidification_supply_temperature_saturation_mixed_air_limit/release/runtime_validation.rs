//! Exact CP415 route, owner, and counter validation.

use super::super::transition::RetainedRoute;
use super::super::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationMixedAirLimitRuntimeState as State;
use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationAssignmentRuntimeState as PredecessorState;

pub(super) fn state_counts_are_consistent(state: &State) -> bool {
    let Some(transitions) = checked_sum(&state.predecessor_route_counts) else {
        return false;
    };
    let Some(predecessor_assignments) =
        checked_sum(&state.predecessor_supply_temperature_saturation_assignment_route_counts)
    else {
        return false;
    };
    let Some(executions) = checked_sum(&state.supply_temperature_mixed_air_limit_route_counts)
    else {
        return false;
    };
    let Some(inactive) = transitions.checked_sub(executions) else {
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
    let Some(unchanged_temperature) = temperature_owners.checked_sub(executions) else {
        return false;
    };
    let Some(source_sites) = executions.checked_mul(4) else {
        return false;
    };
    for index in 0..36 {
        let Some(guard_outcomes) = state.predecessor_guard_false_fallthrough_route_counts[index]
            .checked_add(state.predecessor_guard_body_entry_route_counts[index])
        else {
            return false;
        };
        let predecessor_index = predecessor_index_for_logical(index);
        let expected_guard_outcomes = if predecessor_index >= 18 {
            state.predecessor_route_counts[index]
        } else {
            0
        };
        if guard_outcomes != expected_guard_outcomes
            || state.predecessor_supply_temperature_saturation_assignment_route_counts[index]
                != state.predecessor_guard_body_entry_route_counts[index]
            || state.supply_temperature_mixed_air_limit_route_counts[index]
                != state.predecessor_supply_temperature_saturation_assignment_route_counts[index]
        {
            return false;
        }
    }
    state.transition_count == transitions
        && state.inactive_transition_count == inactive
        && state.predecessor_supply_temperature_saturation_assignment_count
            == predecessor_assignments
        && state.supply_temperature_saturation_mixed_air_limit_count == executions
        && predecessor_assignments == executions
        && state.source_site_execution_count == source_sites
        && state.cp414_supply_humidity_ratio_state_owner_count == humidity_ratio_owners
        && state.unchanged_supply_humidity_ratio_preservation_count == humidity_ratio_owners
        && state.cp414_supply_enthalpy_state_owner_count == enthalpy_owners
        && state.unchanged_supply_enthalpy_preservation_count == enthalpy_owners
        && state.cp414_supply_temperature_state_owner_count == temperature_owners
        && state.unchanged_supply_temperature_preservation_count == unchanged_temperature
        && state.cp415_mixed_air_limited_supply_temperature_state_owner_count == executions
        && state.cp414_retained_supply_temperature_owned_read_count == executions
        && state.supply_temperature_for_minimum_read_count == executions
        && state.cp329_retained_mixed_air_temperature_owned_read_count == executions
        && state.mixed_air_temperature_for_minimum_read_count == executions
        && state.source_shaped_two_argument_minimum_evaluation_count == executions
        && state.supply_temperature_assignment_write_count == executions
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
    let mut assignments = state.predecessor_supply_temperature_saturation_assignment_route_counts;
    let Some(next) = routes[index].checked_add(1) else {
        return false;
    };
    routes[index] = next;
    if route.predecessor_guard_false_fallthrough {
        let Some(next) = guard_false[index].checked_add(1) else {
            return false;
        };
        guard_false[index] = next;
    }
    if route.predecessor_guard_body_entered {
        let Some(next) = guard_body[index].checked_add(1) else {
            return false;
        };
        guard_body[index] = next;
    }
    if route.predecessor_assignment_executed {
        let Some(next) = assignments[index].checked_add(1) else {
            return false;
        };
        assignments[index] = next;
    }
    routes == predecessor.predecessor_route_counts
        && guard_false == predecessor.predecessor_guard_false_fallthrough_route_counts
        && guard_body == predecessor.predecessor_guard_body_entry_route_counts
        && assignments == predecessor.supply_temperature_saturation_assignment_route_counts
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
            == predecessor.supply_temperature_saturation_assignment_route_counts
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

#[cfg(test)]
mod tests {
    use super::*;
    use ep_model::IdealLoadsAirSystemId;

    #[test]
    fn source_site_multiplication_overflow_fails_closed() {
        let mut state = State::new(IdealLoadsAirSystemId(415));
        state.transition_count = usize::MAX;
        state.predecessor_supply_temperature_saturation_assignment_count = usize::MAX;
        state.supply_temperature_saturation_mixed_air_limit_count = usize::MAX;
        state.predecessor_route_counts[20] = usize::MAX;
        state.predecessor_guard_body_entry_route_counts[20] = usize::MAX;
        state.predecessor_supply_temperature_saturation_assignment_route_counts[20] = usize::MAX;
        state.supply_temperature_mixed_air_limit_route_counts[20] = usize::MAX;
        state.source_site_execution_count = usize::MAX;
        state.cp414_supply_humidity_ratio_state_owner_count = usize::MAX;
        state.unchanged_supply_humidity_ratio_preservation_count = usize::MAX;
        state.cp414_supply_enthalpy_state_owner_count = usize::MAX;
        state.unchanged_supply_enthalpy_preservation_count = usize::MAX;
        state.cp414_supply_temperature_state_owner_count = usize::MAX;
        state.cp415_mixed_air_limited_supply_temperature_state_owner_count = usize::MAX;
        state.cp414_retained_supply_temperature_owned_read_count = usize::MAX;
        state.supply_temperature_for_minimum_read_count = usize::MAX;
        state.cp329_retained_mixed_air_temperature_owned_read_count = usize::MAX;
        state.mixed_air_temperature_for_minimum_read_count = usize::MAX;
        state.source_shaped_two_argument_minimum_evaluation_count = usize::MAX;
        state.supply_temperature_assignment_write_count = usize::MAX;

        assert!(usize::MAX.checked_mul(4).is_none());
        assert!(!state_counts_are_consistent(&state));
    }
}
