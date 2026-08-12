//! CP417 checked counter preparation and mutation.

use super::super::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyEnthalpyAssignmentRuntimeState as State;
use super::RetainedRoute;
use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioAssignmentSnapshot as Predecessor;

pub(super) fn next_transition_fits(
    state: &State,
    predecessor: Predecessor,
    route: RetainedRoute,
) -> bool {
    let index = route.logical_index;
    if index >= 36
        || state.transition_count.checked_add(1).is_none()
        || state.predecessor_route_counts[index]
            .checked_add(1)
            .is_none()
    {
        return false;
    }
    if route.predecessor_guard_false_fallthrough
        && state.predecessor_guard_false_fallthrough_route_counts[index]
            .checked_add(1)
            .is_none()
    {
        return false;
    }
    if route.predecessor_guard_body_entered
        && state.predecessor_guard_body_entry_route_counts[index]
            .checked_add(1)
            .is_none()
    {
        return false;
    }
    if route.predecessor_saturation_temperature_assignment_executed
        && (state
            .predecessor_supply_temperature_saturation_assignment_count
            .checked_add(1)
            .is_none()
            || state.predecessor_supply_temperature_saturation_assignment_route_counts[index]
                .checked_add(1)
                .is_none())
    {
        return false;
    }
    if route.predecessor_saturation_temperature_mixed_air_limit_executed
        && (state
            .predecessor_supply_temperature_saturation_mixed_air_limit_count
            .checked_add(1)
            .is_none()
            || state.predecessor_supply_temperature_mixed_air_limit_route_counts[index]
                .checked_add(1)
                .is_none())
    {
        return false;
    }
    if route.predecessor_supply_humidity_ratio_assignment_executed
        && (state
            .predecessor_supply_humidity_ratio_assignment_count
            .checked_add(1)
            .is_none()
            || state.predecessor_supply_humidity_ratio_assignment_route_counts[index]
                .checked_add(1)
                .is_none())
    {
        return false;
    }
    if predecessor.resulting_supply_humidity_ratio.is_some()
        && (state
            .cp416_supply_humidity_ratio_state_owner_count
            .checked_add(1)
            .is_none()
            || state
                .unchanged_supply_humidity_ratio_preservation_count
                .checked_add(1)
                .is_none())
    {
        return false;
    }
    if predecessor.resulting_supply_enthalpy_j_per_kg.is_some()
        && (state
            .cp416_supply_enthalpy_state_owner_count
            .checked_add(1)
            .is_none()
            || (!route.active
                && state
                    .unchanged_supply_enthalpy_preservation_count
                    .checked_add(1)
                    .is_none()))
    {
        return false;
    }
    if predecessor.resulting_supply_temperature_c.is_some()
        && (state
            .cp416_supply_temperature_state_owner_count
            .checked_add(1)
            .is_none()
            || state
                .unchanged_supply_temperature_preservation_count
                .checked_add(1)
                .is_none())
    {
        return false;
    }
    if route.active {
        let counters = [
            state.supply_enthalpy_assignment_count,
            state.supply_enthalpy_assignment_route_counts[index],
            state.cp417_psychrometric_supply_enthalpy_state_owner_count,
            state.cp416_retained_supply_temperature_owned_read_count,
            state.supply_temperature_for_enthalpy_read_count,
            state.cp416_retained_supply_humidity_ratio_owned_read_count,
            state.supply_humidity_ratio_for_enthalpy_read_count,
            state.psychrometric_supply_enthalpy_evaluation_count,
            state.supply_enthalpy_assignment_write_count,
        ];
        state.source_site_execution_count.checked_add(4).is_some()
            && counters
                .into_iter()
                .all(|count| count.checked_add(1).is_some())
    } else {
        state.inactive_transition_count.checked_add(1).is_some()
    }
}

pub(super) fn increment_counts(state: &mut State, predecessor: Predecessor, route: RetainedRoute) {
    let index = route.logical_index;
    state.predecessor_route_counts[index] += 1;
    if route.predecessor_guard_false_fallthrough {
        state.predecessor_guard_false_fallthrough_route_counts[index] += 1;
    }
    if route.predecessor_guard_body_entered {
        state.predecessor_guard_body_entry_route_counts[index] += 1;
    }
    if route.predecessor_saturation_temperature_assignment_executed {
        state.predecessor_supply_temperature_saturation_assignment_count += 1;
        state.predecessor_supply_temperature_saturation_assignment_route_counts[index] += 1;
    }
    if route.predecessor_saturation_temperature_mixed_air_limit_executed {
        state.predecessor_supply_temperature_saturation_mixed_air_limit_count += 1;
        state.predecessor_supply_temperature_mixed_air_limit_route_counts[index] += 1;
    }
    if route.predecessor_supply_humidity_ratio_assignment_executed {
        state.predecessor_supply_humidity_ratio_assignment_count += 1;
        state.predecessor_supply_humidity_ratio_assignment_route_counts[index] += 1;
    }
    if predecessor.resulting_supply_humidity_ratio.is_some() {
        state.cp416_supply_humidity_ratio_state_owner_count += 1;
        state.unchanged_supply_humidity_ratio_preservation_count += 1;
    }
    if predecessor.resulting_supply_enthalpy_j_per_kg.is_some() {
        state.cp416_supply_enthalpy_state_owner_count += 1;
        if !route.active {
            state.unchanged_supply_enthalpy_preservation_count += 1;
        }
    }
    if predecessor.resulting_supply_temperature_c.is_some() {
        state.cp416_supply_temperature_state_owner_count += 1;
        state.unchanged_supply_temperature_preservation_count += 1;
    }
    if !route.active {
        state.inactive_transition_count += 1;
        return;
    }
    state.supply_enthalpy_assignment_count += 1;
    state.supply_enthalpy_assignment_route_counts[index] += 1;
    state.source_site_execution_count += 4;
    state.cp417_psychrometric_supply_enthalpy_state_owner_count += 1;
    state.cp416_retained_supply_temperature_owned_read_count += 1;
    state.supply_temperature_for_enthalpy_read_count += 1;
    state.cp416_retained_supply_humidity_ratio_owned_read_count += 1;
    state.supply_humidity_ratio_for_enthalpy_read_count += 1;
    state.psychrometric_supply_enthalpy_evaluation_count += 1;
    state.supply_enthalpy_assignment_write_count += 1;
}
