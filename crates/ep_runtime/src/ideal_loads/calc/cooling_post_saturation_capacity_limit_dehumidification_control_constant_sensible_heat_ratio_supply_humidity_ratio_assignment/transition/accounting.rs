//! Exact CP392 route, retained-state, and source-site accounting.

use super::State;
use super::routes::{
    RetainedRoute, predecessor_has_supply_enthalpy, predecessor_has_supply_temperature,
};
use crate::ideal_loads::PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE_ORDER;

pub(super) fn next_transition_fits(state: &State, route: RetainedRoute) -> bool {
    if state.transition_count.checked_add(1).is_none()
        || state.predecessor_route_counts[route.predecessor_index]
            .checked_add(1)
            .is_none()
    {
        return false;
    }
    if predecessor_has_supply_temperature(route.predecessor_index)
        && (state
            .cp391_supply_temperature_state_owner_count
            .checked_add(1)
            .is_none()
            || state
                .unchanged_supply_temperature_preservation_count
                .checked_add(1)
                .is_none())
    {
        return false;
    }
    if predecessor_has_supply_enthalpy(route.predecessor_index)
        && (state
            .cp391_supply_enthalpy_state_owner_count
            .checked_add(1)
            .is_none()
            || state
                .unchanged_supply_enthalpy_preservation_count
                .checked_add(1)
                .is_none())
    {
        return false;
    }
    if !route.active {
        return state.inactive_transition_count.checked_add(1).is_some();
    }
    [
        state
            .dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment_count,
        state.supply_temperature_owned_read_count,
        state.supply_temperature_for_humidity_ratio_inversion_read_count,
        state.supply_enthalpy_owned_read_count,
        state.supply_enthalpy_for_humidity_ratio_inversion_read_count,
        state.psychrometric_supply_humidity_ratio_evaluation_count,
        state.supply_humidity_ratio_assignment_write_count,
    ]
    .into_iter()
    .all(|count| count.checked_add(1).is_some())
        && state
            .source_site_execution_count
            .checked_add(
                PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE_ORDER.len(),
            )
            .is_some()
}

pub(super) fn increment_counts(state: &mut State, route: RetainedRoute) {
    state.predecessor_route_counts[route.predecessor_index] += 1;
    if predecessor_has_supply_temperature(route.predecessor_index) {
        state.cp391_supply_temperature_state_owner_count += 1;
        state.unchanged_supply_temperature_preservation_count += 1;
    }
    if predecessor_has_supply_enthalpy(route.predecessor_index) {
        state.cp391_supply_enthalpy_state_owner_count += 1;
        state.unchanged_supply_enthalpy_preservation_count += 1;
    }
    if !route.active {
        state.inactive_transition_count += 1;
        return;
    }
    state
        .dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment_count += 1;
    state.source_site_execution_count +=
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE_ORDER.len();
    state.supply_temperature_owned_read_count += 1;
    state.supply_temperature_for_humidity_ratio_inversion_read_count += 1;
    state.supply_enthalpy_owned_read_count += 1;
    state.supply_enthalpy_for_humidity_ratio_inversion_read_count += 1;
    state.psychrometric_supply_humidity_ratio_evaluation_count += 1;
    state.supply_humidity_ratio_assignment_write_count += 1;
}
