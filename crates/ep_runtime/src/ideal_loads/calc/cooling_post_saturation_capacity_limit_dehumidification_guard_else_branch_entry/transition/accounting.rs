//! CP418 checked counter preparation and mutation.

use super::super::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchEntryRuntimeState as State;
use super::RetainedRoute;
use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyEnthalpyAssignmentSnapshot as Predecessor;

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
    let route_counters = [
        (
            route.predecessor_guard_false_fallthrough,
            state.predecessor_guard_false_fallthrough_route_counts[index],
        ),
        (
            route.predecessor_guard_body_entered,
            state.predecessor_guard_body_entry_route_counts[index],
        ),
        (
            route.predecessor_saturation_temperature_assignment_executed,
            state.predecessor_supply_temperature_saturation_assignment_route_counts[index],
        ),
        (
            route.predecessor_saturation_temperature_mixed_air_limit_executed,
            state.predecessor_supply_temperature_mixed_air_limit_route_counts[index],
        ),
        (
            route.predecessor_supply_humidity_ratio_assignment_executed,
            state.predecessor_supply_humidity_ratio_assignment_route_counts[index],
        ),
        (
            route.predecessor_supply_enthalpy_assignment_executed,
            state.predecessor_supply_enthalpy_assignment_route_counts[index],
        ),
        (
            route.active,
            state.dehumidification_guard_else_branch_entry_route_counts[index],
        ),
    ];
    if route_counters
        .into_iter()
        .any(|(active, count)| active && count.checked_add(1).is_none())
    {
        return false;
    }
    let scalar_counters = [
        (
            route.predecessor_saturation_temperature_assignment_executed,
            state.predecessor_supply_temperature_saturation_assignment_count,
        ),
        (
            route.predecessor_saturation_temperature_mixed_air_limit_executed,
            state.predecessor_supply_temperature_saturation_mixed_air_limit_count,
        ),
        (
            route.predecessor_supply_humidity_ratio_assignment_executed,
            state.predecessor_supply_humidity_ratio_assignment_count,
        ),
        (
            route.predecessor_supply_enthalpy_assignment_executed,
            state.predecessor_supply_enthalpy_assignment_count,
        ),
        (
            route.active,
            state.dehumidification_guard_else_branch_entry_count,
        ),
        (route.active, state.source_site_execution_count),
        (!route.active, state.inactive_transition_count),
        (
            predecessor.resulting_supply_humidity_ratio.is_some(),
            state.cp417_supply_humidity_ratio_state_owner_count,
        ),
        (
            predecessor.resulting_supply_humidity_ratio.is_some(),
            state.unchanged_supply_humidity_ratio_preservation_count,
        ),
        (
            predecessor.resulting_supply_enthalpy_j_per_kg.is_some(),
            state.cp417_supply_enthalpy_state_owner_count,
        ),
        (
            predecessor.resulting_supply_enthalpy_j_per_kg.is_some(),
            state.unchanged_supply_enthalpy_preservation_count,
        ),
        (
            predecessor.resulting_supply_temperature_c.is_some(),
            state.cp417_supply_temperature_state_owner_count,
        ),
        (
            predecessor.resulting_supply_temperature_c.is_some(),
            state.unchanged_supply_temperature_preservation_count,
        ),
    ];
    scalar_counters
        .into_iter()
        .all(|(active, count)| !active || count.checked_add(1).is_some())
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
    if route.predecessor_supply_enthalpy_assignment_executed {
        state.predecessor_supply_enthalpy_assignment_count += 1;
        state.predecessor_supply_enthalpy_assignment_route_counts[index] += 1;
    }
    if predecessor.resulting_supply_humidity_ratio.is_some() {
        state.cp417_supply_humidity_ratio_state_owner_count += 1;
        state.unchanged_supply_humidity_ratio_preservation_count += 1;
    }
    if predecessor.resulting_supply_enthalpy_j_per_kg.is_some() {
        state.cp417_supply_enthalpy_state_owner_count += 1;
        state.unchanged_supply_enthalpy_preservation_count += 1;
    }
    if predecessor.resulting_supply_temperature_c.is_some() {
        state.cp417_supply_temperature_state_owner_count += 1;
        state.unchanged_supply_temperature_preservation_count += 1;
    }
    if route.active {
        state.dehumidification_guard_else_branch_entry_count += 1;
        state.dehumidification_guard_else_branch_entry_route_counts[index] += 1;
        state.source_site_execution_count += 1;
    } else {
        state.inactive_transition_count += 1;
    }
}
