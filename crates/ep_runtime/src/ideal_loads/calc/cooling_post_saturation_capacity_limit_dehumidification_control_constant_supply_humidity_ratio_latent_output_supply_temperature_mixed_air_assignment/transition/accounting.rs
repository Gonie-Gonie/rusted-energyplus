//! Checked CP403 route, owner, preservation, and source-site accounting.

use super::routes::RetainedRoute;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputGuardSnapshot as Predecessor,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureMixedAirAssignmentRuntimeState as State,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_ASSIGNMENT_SOURCE_ORDER as SOURCE_ORDER,
};

pub(super) fn next_transition_fits(
    state: &State,
    predecessor: Predecessor,
    route: RetainedRoute,
) -> bool {
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
        (
            route.assignment_executed,
            state.supply_temperature_mixed_air_assignment_count,
        ),
        (
            route.assignment_executed,
            state.supply_temperature_mixed_air_assignment_route_counts[index],
        ),
    ]
    .into_iter()
    .any(|(used, count)| used && count.checked_add(1).is_none())
    {
        return false;
    }

    let humidity = predecessor.resulting_supply_humidity_ratio.is_some();
    let enthalpy = predecessor.resulting_supply_enthalpy_j_per_kg.is_some();
    let temperature = predecessor.resulting_supply_temperature_c.is_some();
    [
        (humidity, state.cp402_supply_humidity_ratio_state_owner_count),
        (humidity, state.unchanged_supply_humidity_ratio_preservation_count),
        (enthalpy, state.cp402_supply_enthalpy_state_owner_count),
        (enthalpy, state.unchanged_supply_enthalpy_preservation_count),
        (temperature, state.cp402_supply_temperature_state_owner_count),
        (
            temperature && !route.assignment_executed,
            state.unchanged_supply_temperature_preservation_count,
        ),
    ]
    .into_iter()
    .all(|(used, count)| !used || count.checked_add(1).is_some())
        && (!route.assignment_executed
            || (state
                .source_site_execution_count
                .checked_add(SOURCE_ORDER.len())
                .is_some()
                && [
                    state.cp329_mixed_air_temperature_owned_read_count,
                    state.cp402_same_call_mixed_air_temperature_bit_corroboration_count,
                    state.mixed_air_temperature_read_count,
                    state.supply_temperature_assignment_write_count,
                ]
                .into_iter()
                .all(|count| count.checked_add(1).is_some())))
}

pub(super) fn increment_counts(
    state: &mut State,
    predecessor: Predecessor,
    route: RetainedRoute,
) {
    let index = route.predecessor_index;
    state.transition_count += 1;
    state.predecessor_route_counts[index] += 1;
    if predecessor.resulting_supply_humidity_ratio.is_some() {
        state.cp402_supply_humidity_ratio_state_owner_count += 1;
        state.unchanged_supply_humidity_ratio_preservation_count += 1;
    }
    if predecessor.resulting_supply_enthalpy_j_per_kg.is_some() {
        state.cp402_supply_enthalpy_state_owner_count += 1;
        state.unchanged_supply_enthalpy_preservation_count += 1;
    }
    if predecessor.resulting_supply_temperature_c.is_some() {
        state.cp402_supply_temperature_state_owner_count += 1;
        if !route.assignment_executed {
            state.unchanged_supply_temperature_preservation_count += 1;
        }
    }
    if !route.guard_evaluated {
        state.inactive_transition_count += 1;
    } else if !route.assignment_executed {
        state.predecessor_guard_false_fallthrough_count += 1;
        state.predecessor_guard_false_fallthrough_route_counts[index] += 1;
    } else {
        state.supply_temperature_mixed_air_assignment_count += 1;
        state.supply_temperature_mixed_air_assignment_route_counts[index] += 1;
        state.source_site_execution_count += SOURCE_ORDER.len();
        state.cp329_mixed_air_temperature_owned_read_count += 1;
        state.cp402_same_call_mixed_air_temperature_bit_corroboration_count += 1;
        state.mixed_air_temperature_read_count += 1;
        state.supply_temperature_assignment_write_count += 1;
    }
}
