//! CP419 checked counter preparation and mutation.

use super::super::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchCpAirAssignmentRuntimeState as State;
use super::RetainedRoute;
use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchEntrySnapshot as Predecessor;

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
            state.predecessor_dehumidification_guard_else_branch_entry_route_counts[index],
        ),
        (
            route.active,
            state.dehumidification_guard_else_branch_cp_air_assignment_route_counts[index],
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
            state.predecessor_dehumidification_guard_else_branch_entry_count,
        ),
        (
            route.active,
            state.dehumidification_guard_else_branch_cp_air_assignment_count,
        ),
        (!route.active, state.inactive_transition_count),
        (
            predecessor.resulting_supply_humidity_ratio.is_some(),
            state.cp418_supply_humidity_ratio_state_owner_count,
        ),
        (
            predecessor.resulting_supply_humidity_ratio.is_some(),
            state.unchanged_supply_humidity_ratio_preservation_count,
        ),
        (
            predecessor.resulting_supply_enthalpy_j_per_kg.is_some(),
            state.cp418_supply_enthalpy_state_owner_count,
        ),
        (
            predecessor.resulting_supply_enthalpy_j_per_kg.is_some(),
            state.unchanged_supply_enthalpy_preservation_count,
        ),
        (
            predecessor.resulting_supply_temperature_c.is_some(),
            state.cp418_supply_temperature_state_owner_count,
        ),
        (
            predecessor.resulting_supply_temperature_c.is_some(),
            state.unchanged_supply_temperature_preservation_count,
        ),
        (
            route.active,
            state.cp419_psychrometric_cp_air_state_owner_count,
        ),
        (
            route.active,
            state.cp329_retained_mixed_air_humidity_ratio_owned_read_count,
        ),
        (
            route.active,
            state.mixed_air_humidity_ratio_for_cp_air_read_count,
        ),
        (route.active, state.psychrometric_cp_air_evaluation_count),
        (route.active, state.cp_air_assignment_write_count),
    ];
    let scalar_counters_fit = scalar_counters
        .into_iter()
        .all(|(active, count)| !active || count.checked_add(1).is_some());
    let sites_fit = !route.active || state.source_site_execution_count.checked_add(3).is_some();
    scalar_counters_fit && sites_fit
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
        state.cp418_supply_humidity_ratio_state_owner_count += 1;
        state.unchanged_supply_humidity_ratio_preservation_count += 1;
    }
    if predecessor.resulting_supply_enthalpy_j_per_kg.is_some() {
        state.cp418_supply_enthalpy_state_owner_count += 1;
        state.unchanged_supply_enthalpy_preservation_count += 1;
    }
    if predecessor.resulting_supply_temperature_c.is_some() {
        state.cp418_supply_temperature_state_owner_count += 1;
        state.unchanged_supply_temperature_preservation_count += 1;
    }
    if route.active {
        state.predecessor_dehumidification_guard_else_branch_entry_count += 1;
        state.dehumidification_guard_else_branch_cp_air_assignment_count += 1;
        state.predecessor_dehumidification_guard_else_branch_entry_route_counts[index] += 1;
        state.dehumidification_guard_else_branch_cp_air_assignment_route_counts[index] += 1;
        state.source_site_execution_count += 3;
        state.cp419_psychrometric_cp_air_state_owner_count += 1;
        state.cp329_retained_mixed_air_humidity_ratio_owned_read_count += 1;
        state.mixed_air_humidity_ratio_for_cp_air_read_count += 1;
        state.psychrometric_cp_air_evaluation_count += 1;
        state.cp_air_assignment_write_count += 1;
    } else {
        state.inactive_transition_count += 1;
    }
}
