//! Checked CP436 route and source-site accounting.

use super::{Predecessor, Route, State};

fn fits(value: usize, increment: bool) -> bool {
    !increment || value.checked_add(1).is_some()
}

pub(super) fn next_transition_fits(
    state: &State,
    predecessor: Predecessor,
    route: Route,
) -> bool {
    let index = route.logical_index;
    if index >= 36
        || state.transition_count.checked_add(1).is_none()
        || state.predecessor_route_counts[index].checked_add(1).is_none()
        || state
            .source_site_execution_count
            .checked_add(if route.assignment_executed { 4 } else { 0 })
            .is_none()
        || !fits(
            state.predecessor_guard_false_fallthrough_route_counts[index],
            route.predecessor_guard_false_fallthrough,
        )
        || !fits(
            state.predecessor_guard_body_entry_route_counts[index],
            route.predecessor_guard_body_entered,
        )
        || !fits(
            state.heating_outdoor_air_volume_flow_assignment_route_counts[index],
            route.assignment_executed,
        )
        || !fits(state.inactive_transition_count, !route.assignment_executed)
        || !fits(
            state.outdoor_air_volume_flow_assignment_count,
            route.assignment_executed,
        )
    {
        return false;
    }
    let assignment_counts = [
        state.cp435_outdoor_air_mass_flow_rate_owned_read_count,
        state.outdoor_air_mass_flow_rate_for_volume_flow_division_read_count,
        state.begin_environment_standard_air_density_owner_count,
        state.standard_air_density_for_volume_flow_division_read_count,
        state.outdoor_air_mass_flow_rate_standard_air_density_division_count,
        state.local_outdoor_air_volume_flow_rate_assignment_write_count,
    ];
    assignment_counts
        .into_iter()
        .all(|count| fits(count, route.assignment_executed))
        && [
            (
                predecessor.resulting_supply_humidity_ratio.is_some(),
                state.cp435_supply_humidity_ratio_state_owner_count,
                state.unchanged_supply_humidity_ratio_preservation_count,
            ),
            (
                predecessor.resulting_supply_enthalpy_j_per_kg.is_some(),
                state.cp435_supply_enthalpy_state_owner_count,
                state.unchanged_supply_enthalpy_preservation_count,
            ),
            (
                predecessor.resulting_supply_temperature_c.is_some(),
                state.cp435_supply_temperature_state_owner_count,
                state.unchanged_supply_temperature_preservation_count,
            ),
        ]
        .into_iter()
        .all(|(present, owner, unchanged)| {
            fits(owner, present) && fits(unchanged, present)
        })
        && (!route.assignment_executed
            || (predecessor
                .outdoor_air_mass_flow_rate_for_heating_maximum_flow_guard_kg_per_s
                .is_some()
                && route.predecessor_guard_body_entered))
}

pub(super) fn increment_counts(state: &mut State, predecessor: Predecessor, route: Route) {
    let index = route.logical_index;
    state.predecessor_route_counts[index] += 1;
    if route.predecessor_guard_false_fallthrough {
        state.predecessor_guard_false_fallthrough_route_counts[index] += 1;
    }
    if route.predecessor_guard_body_entered {
        state.predecessor_guard_body_entry_route_counts[index] += 1;
    }
    if route.assignment_executed {
        state.heating_outdoor_air_volume_flow_assignment_route_counts[index] += 1;
        state.outdoor_air_volume_flow_assignment_count += 1;
        state.cp435_outdoor_air_mass_flow_rate_owned_read_count += 1;
        state.outdoor_air_mass_flow_rate_for_volume_flow_division_read_count += 1;
        state.begin_environment_standard_air_density_owner_count += 1;
        state.standard_air_density_for_volume_flow_division_read_count += 1;
        state.outdoor_air_mass_flow_rate_standard_air_density_division_count += 1;
        state.local_outdoor_air_volume_flow_rate_assignment_write_count += 1;
        state.source_site_execution_count += 4;
    } else {
        state.inactive_transition_count += 1;
    }
    if predecessor.resulting_supply_humidity_ratio.is_some() {
        state.cp435_supply_humidity_ratio_state_owner_count += 1;
        state.unchanged_supply_humidity_ratio_preservation_count += 1;
    }
    if predecessor.resulting_supply_enthalpy_j_per_kg.is_some() {
        state.cp435_supply_enthalpy_state_owner_count += 1;
        state.unchanged_supply_enthalpy_preservation_count += 1;
    }
    if predecessor.resulting_supply_temperature_c.is_some() {
        state.cp435_supply_temperature_state_owner_count += 1;
        state.unchanged_supply_temperature_preservation_count += 1;
    }
}
