//! CP420 checked counter preparation and mutation.

use super::super::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER as ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputAssignmentRuntimeState as State,
};
use super::RetainedRoute;
use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchCpAirAssignmentSnapshot as Predecessor;

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
            state.predecessor_dehumidification_guard_else_branch_cp_air_assignment_route_counts
                [index],
        ),
        (
            route.active,
            state.dehumidification_guard_else_branch_sensible_output_assignment_route_counts[index],
        ),
    ];
    if route_counters
        .into_iter()
        .any(|(used, count)| used && count.checked_add(1).is_none())
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
            state.predecessor_dehumidification_guard_else_branch_cp_air_assignment_count,
        ),
        (
            route.active,
            state.dehumidification_guard_else_branch_sensible_output_assignment_count,
        ),
        (!route.active, state.inactive_transition_count),
        (
            predecessor.resulting_supply_humidity_ratio.is_some(),
            state.cp419_supply_humidity_ratio_state_owner_count,
        ),
        (
            predecessor.resulting_supply_humidity_ratio.is_some(),
            state.unchanged_supply_humidity_ratio_preservation_count,
        ),
        (
            predecessor.resulting_supply_enthalpy_j_per_kg.is_some(),
            state.cp419_supply_enthalpy_state_owner_count,
        ),
        (
            predecessor.resulting_supply_enthalpy_j_per_kg.is_some(),
            state.unchanged_supply_enthalpy_preservation_count,
        ),
        (
            predecessor.resulting_supply_temperature_c.is_some(),
            state.cp419_supply_temperature_state_owner_count,
        ),
        (
            predecessor.resulting_supply_temperature_c.is_some(),
            state.unchanged_supply_temperature_preservation_count,
        ),
    ];
    let active_counters = [
        state.supply_mass_flow_rate_owned_read_count,
        state.supply_mass_flow_rate_bit_corroboration_count,
        state.supply_mass_flow_rate_read_count,
        state.cp_air_owned_read_count,
        state.cp_air_read_count,
        state.supply_mass_flow_rate_times_cp_air_calculation_count,
        state.mixed_air_temperature_owned_read_count,
        state.mixed_air_temperature_read_count,
        state.supply_temperature_owned_read_count,
        state.supply_temperature_read_count,
        state.mixed_air_minus_supply_temperature_calculation_count,
        state.cooling_sensible_output_calculation_count,
        state.cooling_sensible_output_assignment_write_count,
    ];
    scalar_counters
        .into_iter()
        .all(|(used, count)| !used || count.checked_add(1).is_some())
        && (!route.active
            || active_counters
                .into_iter()
                .all(|count| count.checked_add(1).is_some()))
        && (!route.active
            || state
                .source_site_execution_count
                .checked_add(ORDER.len())
                .is_some())
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
        state.cp419_supply_humidity_ratio_state_owner_count += 1;
        state.unchanged_supply_humidity_ratio_preservation_count += 1;
    }
    if predecessor.resulting_supply_enthalpy_j_per_kg.is_some() {
        state.cp419_supply_enthalpy_state_owner_count += 1;
        state.unchanged_supply_enthalpy_preservation_count += 1;
    }
    if predecessor.resulting_supply_temperature_c.is_some() {
        state.cp419_supply_temperature_state_owner_count += 1;
        state.unchanged_supply_temperature_preservation_count += 1;
    }
    if route.active {
        state.predecessor_dehumidification_guard_else_branch_entry_count += 1;
        state.predecessor_dehumidification_guard_else_branch_cp_air_assignment_count += 1;
        state.dehumidification_guard_else_branch_sensible_output_assignment_count += 1;
        state.predecessor_dehumidification_guard_else_branch_entry_route_counts[index] += 1;
        state.predecessor_dehumidification_guard_else_branch_cp_air_assignment_route_counts
            [index] += 1;
        state.dehumidification_guard_else_branch_sensible_output_assignment_route_counts[index] +=
            1;
        state.source_site_execution_count += ORDER.len();
        state.supply_mass_flow_rate_owned_read_count += 1;
        state.supply_mass_flow_rate_bit_corroboration_count += 1;
        state.supply_mass_flow_rate_read_count += 1;
        state.cp_air_owned_read_count += 1;
        state.cp_air_read_count += 1;
        state.supply_mass_flow_rate_times_cp_air_calculation_count += 1;
        state.mixed_air_temperature_owned_read_count += 1;
        state.mixed_air_temperature_read_count += 1;
        state.supply_temperature_owned_read_count += 1;
        state.supply_temperature_read_count += 1;
        state.mixed_air_minus_supply_temperature_calculation_count += 1;
        state.cooling_sensible_output_calculation_count += 1;
        state.cooling_sensible_output_assignment_write_count += 1;
    } else {
        state.inactive_transition_count += 1;
    }
}
