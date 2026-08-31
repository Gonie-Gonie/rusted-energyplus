//! Transactional CP435 route and source-site accounting.

use super::{Predecessor, Route, State};

fn source_site_delta(route: Route) -> usize {
    usize::from(route.guard_evaluated)
        + usize::from(route.heating_limit_flow_rate_and_capacity_comparison_evaluated)
        + 3 * usize::from(route.strict_mass_flow_comparison_evaluated)
        + usize::from(route.body_entered)
}

fn fits(value: usize, increment: bool) -> bool {
    !increment || value.checked_add(1).is_some()
}

pub(super) fn next_transition_fits(state: &State, predecessor: Predecessor, route: Route) -> bool {
    let index = route.logical_index;
    if index >= 36
        || state.transition_count.checked_add(1).is_none()
        || state.predecessor_route_counts[index].checked_add(1).is_none()
        || state
            .source_site_execution_count
            .checked_add(source_site_delta(route))
            .is_none()
        || !fits(state.inactive_transition_count, !route.guard_evaluated)
        || !fits(state.heating_outdoor_air_maximum_flow_guard_evaluation_count, route.guard_evaluated)
        || !fits(state.heating_limit_flow_rate_comparison_count, route.guard_evaluated)
        || !fits(state.heating_limit_flow_rate_match_count, route.heating_limit_flow_rate_comparison_satisfied)
        || !fits(state.heating_limit_flow_rate_and_capacity_comparison_count, route.heating_limit_flow_rate_and_capacity_comparison_evaluated)
        || !fits(state.heating_limit_flow_rate_and_capacity_match_count, route.heating_limit_flow_rate_and_capacity_comparison_satisfied)
        || !fits(state.heating_flow_limit_selector_rejection_count, route.heating_flow_limit_selector_rejected)
        || !fits(state.cp311_same_call_outdoor_air_mass_flow_rate_bit_corroboration_count, route.strict_mass_flow_comparison_evaluated)
        || !fits(state.outdoor_air_mass_flow_rate_read_after_heating_limit_short_circuit_count, route.strict_mass_flow_comparison_evaluated)
        || !fits(state.maximum_heating_air_mass_flow_rate_read_after_heating_limit_short_circuit_count, route.strict_mass_flow_comparison_evaluated)
        || !fits(state.outdoor_air_mass_flow_rate_maximum_heating_air_mass_flow_rate_comparison_count, route.strict_mass_flow_comparison_evaluated)
        || !fits(state.outdoor_air_mass_flow_rate_strictly_greater_than_maximum_heating_air_mass_flow_rate_count, route.body_entered)
        || !fits(state.maximum_heating_flow_body_entry_count, route.body_entered)
        || !fits(state.heating_outdoor_air_maximum_flow_guard_false_fallthrough_count, route.false_fallthrough)
        || (route.body_entered && state.maximum_heating_flow_body_entry_route_counts[index].checked_add(1).is_none())
        || (route.false_fallthrough && state.heating_outdoor_air_maximum_flow_guard_false_fallthrough_route_counts[index].checked_add(1).is_none())
    {
        return false;
    }
    [
        (predecessor.resulting_supply_humidity_ratio.is_some(), state.cp434_supply_humidity_ratio_state_owner_count, state.unchanged_supply_humidity_ratio_preservation_count),
        (predecessor.resulting_supply_enthalpy_j_per_kg.is_some(), state.cp434_supply_enthalpy_state_owner_count, state.unchanged_supply_enthalpy_preservation_count),
        (predecessor.resulting_supply_temperature_c.is_some(), state.cp434_supply_temperature_state_owner_count, state.unchanged_supply_temperature_preservation_count),
    ]
    .into_iter()
    .all(|(present, owner, unchanged)| {
        !present || (owner.checked_add(1).is_some() && unchanged.checked_add(1).is_some())
    })
}

pub(super) fn increment_counts(state: &mut State, predecessor: Predecessor, route: Route) {
    let index = route.logical_index;
    state.predecessor_route_counts[index] += 1;
    state.source_site_execution_count += source_site_delta(route);
    if predecessor.resulting_supply_humidity_ratio.is_some() {
        state.cp434_supply_humidity_ratio_state_owner_count += 1;
        state.unchanged_supply_humidity_ratio_preservation_count += 1;
    }
    if predecessor.resulting_supply_enthalpy_j_per_kg.is_some() {
        state.cp434_supply_enthalpy_state_owner_count += 1;
        state.unchanged_supply_enthalpy_preservation_count += 1;
    }
    if predecessor.resulting_supply_temperature_c.is_some() {
        state.cp434_supply_temperature_state_owner_count += 1;
        state.unchanged_supply_temperature_preservation_count += 1;
    }
    if !route.guard_evaluated {
        state.inactive_transition_count += 1;
        return;
    }
    state.heating_outdoor_air_maximum_flow_guard_evaluation_count += 1;
    state.heating_limit_flow_rate_comparison_count += 1;
    state.heating_limit_flow_rate_match_count += usize::from(route.heating_limit_flow_rate_comparison_satisfied);
    state.heating_limit_flow_rate_and_capacity_comparison_count += usize::from(route.heating_limit_flow_rate_and_capacity_comparison_evaluated);
    state.heating_limit_flow_rate_and_capacity_match_count += usize::from(route.heating_limit_flow_rate_and_capacity_comparison_satisfied);
    state.heating_flow_limit_selector_rejection_count += usize::from(route.heating_flow_limit_selector_rejected);
    if route.strict_mass_flow_comparison_evaluated {
        state.cp311_same_call_outdoor_air_mass_flow_rate_bit_corroboration_count += 1;
        state.outdoor_air_mass_flow_rate_read_after_heating_limit_short_circuit_count += 1;
        state.maximum_heating_air_mass_flow_rate_read_after_heating_limit_short_circuit_count += 1;
        state.outdoor_air_mass_flow_rate_maximum_heating_air_mass_flow_rate_comparison_count += 1;
    }
    if route.body_entered {
        state.outdoor_air_mass_flow_rate_strictly_greater_than_maximum_heating_air_mass_flow_rate_count += 1;
        state.maximum_heating_flow_body_entry_count += 1;
        state.maximum_heating_flow_body_entry_route_counts[index] += 1;
    }
    if route.false_fallthrough {
        state.heating_outdoor_air_maximum_flow_guard_false_fallthrough_count += 1;
        state.heating_outdoor_air_maximum_flow_guard_false_fallthrough_route_counts[index] += 1;
    }
}
