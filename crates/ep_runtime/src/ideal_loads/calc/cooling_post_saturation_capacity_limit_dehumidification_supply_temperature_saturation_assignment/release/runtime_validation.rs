//! Exact CP414 route, owner, and counter validation.

use super::super::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationAssignmentRuntimeState as State;
use super::super::transition::RetainedRoute;
use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationGuardRuntimeState as PredecessorState;

pub(super) fn state_counts_are_consistent(state: &State) -> bool {
    let Some(transitions) = checked_sum(&state.predecessor_route_counts) else {
        return false;
    };
    let Some(assignments) = checked_sum(&state.supply_temperature_saturation_assignment_route_counts) else {
        return false;
    };
    let Some(inactive) = transitions.checked_sub(assignments) else {
        return false;
    };
    let Some(humidity_ratio_owners) = checked_sum(&state.predecessor_route_counts[18..]) else {
        return false;
    };
    let Some(enthalpy_owners) = sum_predecessor_indices(
        &state.predecessor_route_counts,
        |index| matches!(index, 5 | 8 | 11 | 14 | 17..=29),
    ) else {
        return false;
    };
    let Some(temperature_owners) = sum_predecessor_indices(
        &state.predecessor_route_counts,
        |index| index >= 3,
    ) else {
        return false;
    };
    let Some(unchanged_temperature) = temperature_owners.checked_sub(assignments) else {
        return false;
    };
    let Some(source_sites) = assignments.checked_mul(4) else {
        return false;
    };
    for index in 0..36 {
        let Some(predecessor_guard_outcomes) = state
            .predecessor_guard_false_fallthrough_route_counts[index]
            .checked_add(state.predecessor_guard_body_entry_route_counts[index])
        else {
            return false;
        };
        let expected_guard_outcomes = if index >= 18 {
            state.predecessor_route_counts[index]
        } else {
            0
        };
        if predecessor_guard_outcomes != expected_guard_outcomes
            || state.supply_temperature_saturation_assignment_route_counts[index]
                != state.predecessor_guard_body_entry_route_counts[index]
        {
            return false;
        }
    }
    state.transition_count == transitions
        && state.inactive_transition_count == inactive
        && state.saturation_supply_temperature_assignment_count == assignments
        && state.source_site_execution_count == source_sites
        && state.cp413_supply_humidity_ratio_state_owner_count == humidity_ratio_owners
        && state.unchanged_supply_humidity_ratio_preservation_count == humidity_ratio_owners
        && state.cp413_supply_enthalpy_state_owner_count == enthalpy_owners
        && state.unchanged_supply_enthalpy_preservation_count == enthalpy_owners
        && state.cp413_supply_temperature_state_owner_count == temperature_owners
        && state.unchanged_supply_temperature_preservation_count == unchanged_temperature
        && state.cp414_saturation_supply_temperature_state_owner_count == assignments
        && state.cp413_retained_supply_enthalpy_owned_read_count == assignments
        && state.supply_enthalpy_for_saturation_temperature_read_count == assignments
        && state.environment_outdoor_barometric_pressure_for_saturation_temperature_owner_count
            == assignments
        && state.environment_outdoor_barometric_pressure_for_saturation_temperature_read_count
            == assignments
        && state.psy_tsat_fn_h_pb_evaluation_count == assignments
        && state.purchased_air_supply_temperature_saturation_assignment_write_count == assignments
}

pub(super) fn pending_predecessor_counts_match(
    state: &State,
    predecessor: &PredecessorState,
    route: RetainedRoute,
) -> bool {
    let index = route.logical_index;
    let mut predecessor_routes = state.predecessor_route_counts;
    let mut guard_false_routes = state.predecessor_guard_false_fallthrough_route_counts;
    let mut guard_body_routes = state.predecessor_guard_body_entry_route_counts;
    let Some(next) = predecessor_routes[index].checked_add(1) else {
        return false;
    };
    predecessor_routes[index] = next;
    if route.predecessor_guard_false_fallthrough {
        let Some(next) = guard_false_routes[index].checked_add(1) else {
            return false;
        };
        guard_false_routes[index] = next;
    }
    if route.predecessor_guard_body_entered {
        let Some(next) = guard_body_routes[index].checked_add(1) else {
            return false;
        };
        guard_body_routes[index] = next;
    }
    predecessor_routes == predecessor.predecessor_route_counts
        && guard_false_routes == predecessor.guard_false_fallthrough_route_counts
        && guard_body_routes == predecessor.guard_body_entry_route_counts
}

pub(super) fn completed_predecessor_counts_match(
    state: &State,
    predecessor: &PredecessorState,
) -> bool {
    state.predecessor_route_counts == predecessor.predecessor_route_counts
        && state.predecessor_guard_false_fallthrough_route_counts
            == predecessor.guard_false_fallthrough_route_counts
        && state.predecessor_guard_body_entry_route_counts
            == predecessor.guard_body_entry_route_counts
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

#[cfg(test)]
mod tests {
    use super::*;
    use ep_model::IdealLoadsAirSystemId;

    #[test]
    fn self_consistent_cp414_counts_still_fail_when_predecessor_routes_disagree() {
        let system = IdealLoadsAirSystemId(414);
        let mut state = State::new(system);
        let mut predecessor = PredecessorState::new(system);
        let route = RetainedRoute {
            logical_index: 20,
            predecessor_guard_false_fallthrough: false,
            predecessor_guard_body_entered: true,
            assignment_executed: true,
        };
        predecessor.predecessor_route_counts[20] = 1;
        predecessor.guard_body_entry_route_counts[20] = 1;
        assert!(pending_predecessor_counts_match(&state, &predecessor, route));

        state.transition_count = 1;
        state.predecessor_route_counts[21] = 1;
        state.predecessor_guard_body_entry_route_counts[21] = 1;
        state.supply_temperature_saturation_assignment_route_counts[21] = 1;
        state.saturation_supply_temperature_assignment_count = 1;
        state.source_site_execution_count = 4;
        state.cp413_supply_humidity_ratio_state_owner_count = 1;
        state.unchanged_supply_humidity_ratio_preservation_count = 1;
        state.cp413_supply_enthalpy_state_owner_count = 1;
        state.unchanged_supply_enthalpy_preservation_count = 1;
        state.cp413_supply_temperature_state_owner_count = 1;
        state.cp414_saturation_supply_temperature_state_owner_count = 1;
        state.cp413_retained_supply_enthalpy_owned_read_count = 1;
        state.supply_enthalpy_for_saturation_temperature_read_count = 1;
        state.environment_outdoor_barometric_pressure_for_saturation_temperature_owner_count = 1;
        state.environment_outdoor_barometric_pressure_for_saturation_temperature_read_count = 1;
        state.psy_tsat_fn_h_pb_evaluation_count = 1;
        state.purchased_air_supply_temperature_saturation_assignment_write_count = 1;
        assert!(state_counts_are_consistent(&state));
        assert!(!completed_predecessor_counts_match(&state, &predecessor));
        assert!(!pending_predecessor_counts_match(&state, &predecessor, route));
    }

    #[test]
    fn source_site_multiplication_overflow_fails_closed_even_when_every_field_is_max() {
        let mut state = State::new(IdealLoadsAirSystemId(414));
        state.transition_count = usize::MAX;
        state.saturation_supply_temperature_assignment_count = usize::MAX;
        state.predecessor_route_counts[20] = usize::MAX;
        state.predecessor_guard_body_entry_route_counts[20] = usize::MAX;
        state.supply_temperature_saturation_assignment_route_counts[20] = usize::MAX;
        state.source_site_execution_count = usize::MAX;
        state.cp413_supply_humidity_ratio_state_owner_count = usize::MAX;
        state.unchanged_supply_humidity_ratio_preservation_count = usize::MAX;
        state.cp413_supply_enthalpy_state_owner_count = usize::MAX;
        state.unchanged_supply_enthalpy_preservation_count = usize::MAX;
        state.cp413_supply_temperature_state_owner_count = usize::MAX;
        state.cp414_saturation_supply_temperature_state_owner_count = usize::MAX;
        state.cp413_retained_supply_enthalpy_owned_read_count = usize::MAX;
        state.supply_enthalpy_for_saturation_temperature_read_count = usize::MAX;
        state.environment_outdoor_barometric_pressure_for_saturation_temperature_owner_count =
            usize::MAX;
        state.environment_outdoor_barometric_pressure_for_saturation_temperature_read_count =
            usize::MAX;
        state.psy_tsat_fn_h_pb_evaluation_count = usize::MAX;
        state.purchased_air_supply_temperature_saturation_assignment_write_count = usize::MAX;

        assert!(usize::MAX.checked_mul(4).is_none());
        assert!(!state_counts_are_consistent(&state));
    }
}
