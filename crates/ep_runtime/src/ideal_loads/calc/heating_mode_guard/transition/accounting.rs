//! Transactional CP431 route, source-site, and owner accounting.

use super::{Predecessor, Route, State};

pub(super) fn next_transition_fits(
    state: &State,
    predecessor: Predecessor,
    route: Route,
) -> bool {
    let index = route.logical_index;
    if index >= 36
        || state.transition_count.checked_add(1).is_none()
        || state.predecessor_route_counts[index].checked_add(1).is_none()
    {
        return false;
    }
    let preserved = [
        (predecessor.resulting_supply_humidity_ratio.is_some(), state.cp430_supply_humidity_ratio_state_owner_count, state.unchanged_supply_humidity_ratio_preservation_count),
        (predecessor.resulting_supply_enthalpy_j_per_kg.is_some(), state.cp430_supply_enthalpy_state_owner_count, state.unchanged_supply_enthalpy_preservation_count),
        (predecessor.resulting_supply_temperature_c.is_some(), state.cp430_supply_temperature_state_owner_count, state.unchanged_supply_temperature_preservation_count),
    ];
    if preserved.into_iter().any(|(present, owner, unchanged)| {
        present && (owner.checked_add(1).is_none() || unchanged.checked_add(1).is_none())
    }) {
        return false;
    }
    if !route.guard_evaluated {
        return state.inactive_transition_count.checked_add(1).is_some();
    }
    let always = [
        state.heating_mode_guard_evaluation_count,
        state.heating_mode_guard_evaluation_route_counts[index],
        state.cp311_retained_minimum_outdoor_air_sensible_output_owner_read_count,
        state.cp312_same_call_minimum_outdoor_air_sensible_output_bit_corroboration_count,
        state.minimum_outdoor_air_sensible_output_for_heating_mode_guard_read_count,
        state.cp310_retained_heating_setpoint_demand_owner_read_count,
        state.heating_setpoint_demand_for_heating_mode_guard_read_count,
        state.minimum_outdoor_air_sensible_output_heating_setpoint_demand_comparison_count,
    ];
    if always.into_iter().any(|value| value.checked_add(1).is_none())
        || state
            .source_site_execution_count
            .checked_add(3 + 2 * usize::from(route.sensible_comparison_satisfied) + usize::from(route.body_entered))
            .is_none()
    {
        return false;
    }
    if route.sensible_comparison_satisfied {
        let short_circuit = [
            state.minimum_outdoor_air_sensible_output_strictly_less_than_heating_setpoint_demand_count,
            state.prevalidated_temperature_control_type_owner_read_count,
            state.temperature_control_type_read_after_sensible_comparison_short_circuit_count,
            state.temperature_control_type_single_cool_comparison_count,
        ];
        if short_circuit
            .into_iter()
            .any(|value| value.checked_add(1).is_none())
        {
            return false;
        }
    }
    if route.single_cool_blocked && state.single_cool_block_count.checked_add(1).is_none() {
        return false;
    }
    if route.body_entered {
        state.temperature_control_type_permits_heating_count.checked_add(1).is_some()
            && state.heating_operating_mode_body_entry_count.checked_add(1).is_some()
            && state.heating_operating_mode_body_entry_route_counts[index]
                .checked_add(1)
                .is_some()
    } else {
        state.heating_mode_guard_false_fallthrough_count.checked_add(1).is_some()
            && state.heating_mode_guard_false_fallthrough_route_counts[index]
                .checked_add(1)
                .is_some()
    }
}

pub(super) fn increment_counts(state: &mut State, predecessor: Predecessor, route: Route) {
    let index = route.logical_index;
    state.predecessor_route_counts[index] += 1;
    if predecessor.resulting_supply_humidity_ratio.is_some() {
        state.cp430_supply_humidity_ratio_state_owner_count += 1;
        state.unchanged_supply_humidity_ratio_preservation_count += 1;
    }
    if predecessor.resulting_supply_enthalpy_j_per_kg.is_some() {
        state.cp430_supply_enthalpy_state_owner_count += 1;
        state.unchanged_supply_enthalpy_preservation_count += 1;
    }
    if predecessor.resulting_supply_temperature_c.is_some() {
        state.cp430_supply_temperature_state_owner_count += 1;
        state.unchanged_supply_temperature_preservation_count += 1;
    }
    if !route.guard_evaluated {
        state.inactive_transition_count += 1;
        return;
    }
    state.heating_mode_guard_evaluation_count += 1;
    state.heating_mode_guard_evaluation_route_counts[index] += 1;
    state.cp311_retained_minimum_outdoor_air_sensible_output_owner_read_count += 1;
    state.cp312_same_call_minimum_outdoor_air_sensible_output_bit_corroboration_count += 1;
    state.minimum_outdoor_air_sensible_output_for_heating_mode_guard_read_count += 1;
    state.cp310_retained_heating_setpoint_demand_owner_read_count += 1;
    state.heating_setpoint_demand_for_heating_mode_guard_read_count += 1;
    state.minimum_outdoor_air_sensible_output_heating_setpoint_demand_comparison_count += 1;
    state.source_site_execution_count += 3;
    if route.sensible_comparison_satisfied {
        state.minimum_outdoor_air_sensible_output_strictly_less_than_heating_setpoint_demand_count += 1;
        state.prevalidated_temperature_control_type_owner_read_count += 1;
        state.temperature_control_type_read_after_sensible_comparison_short_circuit_count += 1;
        state.temperature_control_type_single_cool_comparison_count += 1;
        state.source_site_execution_count += 2;
    }
    if route.single_cool_blocked {
        state.single_cool_block_count += 1;
    }
    if route.body_entered {
        state.temperature_control_type_permits_heating_count += 1;
        state.heating_operating_mode_body_entry_count += 1;
        state.heating_operating_mode_body_entry_route_counts[index] += 1;
        state.source_site_execution_count += 1;
    } else {
        state.heating_mode_guard_false_fallthrough_count += 1;
        state.heating_mode_guard_false_fallthrough_route_counts[index] += 1;
    }
}
