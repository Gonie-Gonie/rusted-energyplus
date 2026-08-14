//! Transactional CP425 route and source-site accounting.

use super::{Predecessor, Route, State};

pub(super) fn next_transition_fits(state: &State, predecessor: Predecessor, route: Route) -> bool {
    let index = route.logical_index;
    if index >= 36
        || state.transition_count.checked_add(1).is_none()
        || state.predecessor_route_counts[index].checked_add(1).is_none()
    {
        return false;
    }
    let preserved = [
        (
            predecessor.resulting_supply_humidity_ratio.is_some(),
            state.cp424_supply_humidity_ratio_state_owner_count,
            state.unchanged_supply_humidity_ratio_preservation_count,
        ),
        (
            predecessor.resulting_supply_enthalpy_j_per_kg.is_some(),
            state.cp424_supply_enthalpy_state_owner_count,
            state.unchanged_supply_enthalpy_preservation_count,
        ),
        (
            predecessor.resulting_supply_temperature_c.is_some(),
            state.cp424_supply_temperature_state_owner_count,
            state.unchanged_supply_temperature_preservation_count,
        ),
    ];
    if preserved.into_iter().any(|(present, owner, unchanged)| {
        present && (owner.checked_add(1).is_none() || unchanged.checked_add(1).is_none())
    }) {
        return false;
    }
    if route.assignment_executed {
        state
            .zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_count
            .checked_add(1)
            .is_some()
            && state.zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_route_counts[index]
                .checked_add(1)
                .is_some()
            && state.source_site_execution_count.checked_add(2).is_some()
            && state.cp425_supply_enthalpy_state_owner_count.checked_add(1).is_some()
            && state.cp329_retained_mixed_air_enthalpy_owned_read_count.checked_add(1).is_some()
            && state.mixed_air_enthalpy_for_zero_supply_mass_flow_supply_enthalpy_read_count.checked_add(1).is_some()
            && state.supply_enthalpy_assignment_write_count.checked_add(1).is_some()
    } else {
        state.inactive_transition_count.checked_add(1).is_some()
    }
}

pub(super) fn increment_counts(state: &mut State, predecessor: Predecessor, route: Route) {
    let index = route.logical_index;
    state.predecessor_route_counts[index] += 1;
    if predecessor.resulting_supply_humidity_ratio.is_some() {
        state.cp424_supply_humidity_ratio_state_owner_count += 1;
        state.unchanged_supply_humidity_ratio_preservation_count += 1;
    }
    if predecessor.resulting_supply_enthalpy_j_per_kg.is_some() {
        state.cp424_supply_enthalpy_state_owner_count += 1;
        state.unchanged_supply_enthalpy_preservation_count += 1;
    }
    if predecessor.resulting_supply_temperature_c.is_some() {
        state.cp424_supply_temperature_state_owner_count += 1;
        state.unchanged_supply_temperature_preservation_count += 1;
    }
    if route.assignment_executed {
        state.zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_count += 1;
        state.zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_route_counts[index] += 1;
        state.source_site_execution_count += 2;
        state.cp425_supply_enthalpy_state_owner_count += 1;
        state.cp329_retained_mixed_air_enthalpy_owned_read_count += 1;
        state.mixed_air_enthalpy_for_zero_supply_mass_flow_supply_enthalpy_read_count += 1;
        state.supply_enthalpy_assignment_write_count += 1;
    } else {
        state.inactive_transition_count += 1;
    }
}
