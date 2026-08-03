//! Checked CP404 route, owner, preservation, and source-site accounting.

use super::routes::{
    RetainedRoute, predecessor_has_supply_enthalpy, predecessor_has_supply_humidity_ratio,
    predecessor_has_supply_temperature,
};
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE_ORDER as SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyHumidityRatioAssignmentRuntimeState as State,
};

pub(super) fn next_transition_fits(state: &State, route: RetainedRoute) -> bool {
    let index = route.predecessor_index;
    if index >= state.predecessor_route_counts.len()
        || state.transition_count.checked_add(1).is_none()
        || state.predecessor_route_counts[index].checked_add(1).is_none()
    {
        return false;
    }
    if [
        (!route.guard_evaluated, state.inactive_transition_count),
        (
            route.guard_evaluated && !route.assignment_executed,
            state.predecessor_guard_false_fallthrough_count,
        ),
        (
            route.guard_evaluated && !route.assignment_executed,
            state.predecessor_guard_false_fallthrough_route_counts[index],
        ),
        (route.assignment_executed, state.supply_humidity_ratio_assignment_count),
        (
            route.assignment_executed,
            state.supply_humidity_ratio_assignment_route_counts[index],
        ),
    ]
    .into_iter()
    .any(|(used, count)| used && count.checked_add(1).is_none())
    {
        return false;
    }
    let owner_pairs = [
        (
            predecessor_has_supply_humidity_ratio(index),
            state.cp403_supply_humidity_ratio_state_owner_count,
            state.unchanged_supply_humidity_ratio_preservation_count,
        ),
        (
            predecessor_has_supply_enthalpy(index),
            state.cp403_supply_enthalpy_state_owner_count,
            state.unchanged_supply_enthalpy_preservation_count,
        ),
        (
            predecessor_has_supply_temperature(index),
            state.cp403_supply_temperature_state_owner_count,
            state.unchanged_supply_temperature_preservation_count,
        ),
    ];
    if owner_pairs.into_iter().any(|(present, owners, preserved)| {
        present && (owners.checked_add(1).is_none() || preserved.checked_add(1).is_none())
    }) {
        return false;
    }
    !route.assignment_executed
        || (state
            .source_site_execution_count
            .checked_add(SOURCE_ORDER.len())
            .is_some()
            && [
                state.supply_temperature_owned_read_count,
                state.supply_temperature_for_humidity_ratio_inversion_read_count,
                state.supply_enthalpy_owned_read_count,
                state.cp385_same_call_supply_enthalpy_bit_corroboration_count,
                state.supply_enthalpy_for_humidity_ratio_inversion_read_count,
                state.psychrometric_supply_humidity_ratio_evaluation_count,
                state.supply_humidity_ratio_assignment_write_count,
            ]
            .into_iter()
            .all(|count| count.checked_add(1).is_some()))
}

pub(super) fn increment_counts(state: &mut State, route: RetainedRoute) {
    let index = route.predecessor_index;
    state.transition_count += 1;
    state.predecessor_route_counts[index] += 1;
    if predecessor_has_supply_humidity_ratio(index) {
        state.cp403_supply_humidity_ratio_state_owner_count += 1;
        state.unchanged_supply_humidity_ratio_preservation_count += 1;
    }
    if predecessor_has_supply_enthalpy(index) {
        state.cp403_supply_enthalpy_state_owner_count += 1;
        state.unchanged_supply_enthalpy_preservation_count += 1;
    }
    if predecessor_has_supply_temperature(index) {
        state.cp403_supply_temperature_state_owner_count += 1;
        state.unchanged_supply_temperature_preservation_count += 1;
    }
    if !route.guard_evaluated {
        state.inactive_transition_count += 1;
    } else if !route.assignment_executed {
        state.predecessor_guard_false_fallthrough_count += 1;
        state.predecessor_guard_false_fallthrough_route_counts[index] += 1;
    } else {
        state.supply_humidity_ratio_assignment_count += 1;
        state.supply_humidity_ratio_assignment_route_counts[index] += 1;
        state.source_site_execution_count += SOURCE_ORDER.len();
        state.supply_temperature_owned_read_count += 1;
        state.supply_temperature_for_humidity_ratio_inversion_read_count += 1;
        state.supply_enthalpy_owned_read_count += 1;
        state.cp385_same_call_supply_enthalpy_bit_corroboration_count += 1;
        state.supply_enthalpy_for_humidity_ratio_inversion_read_count += 1;
        state.psychrometric_supply_humidity_ratio_evaluation_count += 1;
        state.supply_humidity_ratio_assignment_write_count += 1;
    }
}
