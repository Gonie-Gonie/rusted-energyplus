//! Transactional CP423 route, owner, arithmetic, and source-site accounting.

use super::{Predecessor, Route, State};

pub(super) fn next_transition_fits(state: &State, predecessor: Predecessor, route: Route) -> bool {
    let index = route.logical_index;
    if index >= 36
        || state.transition_count.checked_add(1).is_none()
        || state.predecessor_route_counts[index].checked_add(1).is_none()
    {
        return false;
    }
    let owner_pairs = [
        (predecessor.resulting_supply_humidity_ratio.is_some(), state.cp422_supply_humidity_ratio_state_owner_count),
        (predecessor.resulting_supply_enthalpy_j_per_kg.is_some(), state.cp422_supply_enthalpy_state_owner_count),
        (predecessor.resulting_supply_temperature_c.is_some(), state.cp422_supply_temperature_state_owner_count),
    ];
    if owner_pairs.into_iter().any(|(present, count)| present && count.checked_add(1).is_none()) {
        return false;
    }
    if predecessor.resulting_supply_humidity_ratio.is_some()
        && state.unchanged_supply_humidity_ratio_preservation_count.checked_add(1).is_none()
        || predecessor.resulting_supply_enthalpy_j_per_kg.is_some()
            && state.unchanged_supply_enthalpy_preservation_count.checked_add(1).is_none()
        || !route.assignment_executed
            && predecessor.resulting_supply_temperature_c.is_some()
            && state.unchanged_supply_temperature_preservation_count.checked_add(1).is_none()
    {
        return false;
    }
    if !route.active {
        return state.inactive_transition_count.checked_add(1).is_some();
    }
    if !route.assignment_executed {
        return state.predecessor_guard_false_fallthrough_count.checked_add(1).is_some()
            && state.predecessor_guard_false_fallthrough_route_counts[index].checked_add(1).is_some();
    }
    let scalar_counts = [
        state.cooling_sensible_output_supply_temperature_assignment_count,
        state.cp423_sensible_output_supply_temperature_state_owner_count,
        state.cp329_retained_mixed_air_temperature_for_sensible_output_supply_temperature_owned_read_count,
        state.mixed_air_temperature_for_sensible_output_supply_temperature_read_count,
        state.cp422_retained_cooling_sensible_output_owned_read_count,
        state.cooling_sensible_output_for_supply_temperature_read_count,
        state.cp330_retained_supply_mass_flow_rate_for_sensible_output_supply_temperature_owned_read_count,
        state.cp329_supply_mass_flow_rate_for_sensible_output_supply_temperature_bit_corroboration_count,
        state.supply_mass_flow_rate_for_sensible_output_supply_temperature_read_count,
        state.cp419_retained_cp_air_for_sensible_output_supply_temperature_owned_read_count,
        state.cp_air_for_sensible_output_supply_temperature_read_count,
        state.supply_mass_flow_rate_times_cp_air_for_sensible_output_supply_temperature_calculation_count,
        state.cooling_sensible_output_over_air_capacity_rate_calculation_count,
        state.sensible_output_supply_temperature_calculation_count,
        state.sensible_output_supply_temperature_assignment_write_count,
    ];
    scalar_counts.into_iter().all(|count| count.checked_add(1).is_some())
        && state.cooling_sensible_output_supply_temperature_assignment_route_counts[index].checked_add(1).is_some()
        && state.source_site_execution_count.checked_add(8).is_some()
}

pub(super) fn increment_counts(state: &mut State, predecessor: Predecessor, route: Route) {
    let index = route.logical_index;
    state.predecessor_route_counts[index] += 1;
    if predecessor.resulting_supply_humidity_ratio.is_some() {
        state.cp422_supply_humidity_ratio_state_owner_count += 1;
        state.unchanged_supply_humidity_ratio_preservation_count += 1;
    }
    if predecessor.resulting_supply_enthalpy_j_per_kg.is_some() {
        state.cp422_supply_enthalpy_state_owner_count += 1;
        state.unchanged_supply_enthalpy_preservation_count += 1;
    }
    if predecessor.resulting_supply_temperature_c.is_some() {
        state.cp422_supply_temperature_state_owner_count += 1;
        if !route.assignment_executed {
            state.unchanged_supply_temperature_preservation_count += 1;
        }
    }
    if !route.active {
        state.inactive_transition_count += 1;
    } else if !route.assignment_executed {
        state.predecessor_guard_false_fallthrough_count += 1;
        state.predecessor_guard_false_fallthrough_route_counts[index] += 1;
    } else {
        state.cooling_sensible_output_supply_temperature_assignment_count += 1;
        state.cooling_sensible_output_supply_temperature_assignment_route_counts[index] += 1;
        state.source_site_execution_count += 8;
        state.cp423_sensible_output_supply_temperature_state_owner_count += 1;
        state.cp329_retained_mixed_air_temperature_for_sensible_output_supply_temperature_owned_read_count += 1;
        state.mixed_air_temperature_for_sensible_output_supply_temperature_read_count += 1;
        state.cp422_retained_cooling_sensible_output_owned_read_count += 1;
        state.cooling_sensible_output_for_supply_temperature_read_count += 1;
        state.cp330_retained_supply_mass_flow_rate_for_sensible_output_supply_temperature_owned_read_count += 1;
        state.cp329_supply_mass_flow_rate_for_sensible_output_supply_temperature_bit_corroboration_count += 1;
        state.supply_mass_flow_rate_for_sensible_output_supply_temperature_read_count += 1;
        state.cp419_retained_cp_air_for_sensible_output_supply_temperature_owned_read_count += 1;
        state.cp_air_for_sensible_output_supply_temperature_read_count += 1;
        state.supply_mass_flow_rate_times_cp_air_for_sensible_output_supply_temperature_calculation_count += 1;
        state.cooling_sensible_output_over_air_capacity_rate_calculation_count += 1;
        state.sensible_output_supply_temperature_calculation_count += 1;
        state.sensible_output_supply_temperature_assignment_write_count += 1;
    }
}
