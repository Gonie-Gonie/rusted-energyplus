//! Exact CP402 route partitions and checked counter identities.

use super::State;

const ACTIVE: &[usize] = &[20, 21, 24, 25, 27, 29];
const HUMIDITY: &[usize] = &[18, 19, 22, 23, 26, 28];
const ENTHALPY: &[usize] = &[
    5, 8, 11, 14, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29,
];
const TEMPERATURE: &[usize] = &[
    3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
    24, 25, 26, 27, 28, 29,
];

pub(super) fn counts_are_exact(state: &State) -> bool {
    let evaluations = state
        .dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_evaluation_count;
    let Some(route_sum) = checked_sum(&state.predecessor_route_counts) else {
        return false;
    };
    let Some(active) = checked_selected_sum(&state.predecessor_route_counts, ACTIVE) else {
        return false;
    };
    let Some(false_count) = checked_sum(&state.guard_false_fallthrough_route_counts) else {
        return false;
    };
    let Some(body_count) = checked_sum(&state.adjustment_body_entry_route_counts) else {
        return false;
    };
    if route_sum != state.transition_count
        || active != evaluations
        || state.inactive_transition_count.checked_add(evaluations)
            != Some(state.transition_count)
        || false_count.checked_add(body_count) != Some(evaluations)
        || !per_lineage_partitions_are_exact(state)
    {
        return false;
    }
    let common = [
        state.cp401_cooling_latent_output_owned_read_count,
        state.cooling_latent_output_read_count,
        state.cp321_maximum_total_cooling_capacity_owned_read_count,
        state.cp340_same_call_maximum_total_cooling_capacity_bit_corroboration_count,
        state.maximum_total_cooling_capacity_read_count,
        state.cooling_latent_output_maximum_total_cooling_capacity_comparison_count,
    ];
    if common.into_iter().any(|count| count != evaluations)
        || state.cooling_latent_output_greater_than_or_equal_to_maximum_total_cooling_capacity_count
            != body_count
        || state.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_adjustment_body_entry_count
            != body_count
        || state.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough_count
            != false_count
        || evaluations
            .checked_mul(3)
            .and_then(|sites| sites.checked_add(body_count))
            != Some(state.source_site_execution_count)
    {
        return false;
    }
    owner_counts_are_exact(state)
}

fn per_lineage_partitions_are_exact(state: &State) -> bool {
    (0..30).all(|index| {
        let false_count = state.guard_false_fallthrough_route_counts[index];
        let body_count = state.adjustment_body_entry_route_counts[index];
        if ACTIVE.contains(&index) {
            false_count.checked_add(body_count) == Some(state.predecessor_route_counts[index])
        } else {
            false_count == 0 && body_count == 0
        }
    })
}

fn owner_counts_are_exact(state: &State) -> bool {
    let humidity = checked_selected_sum(&state.predecessor_route_counts, HUMIDITY);
    let enthalpy = checked_selected_sum(&state.predecessor_route_counts, ENTHALPY);
    let temperature = checked_selected_sum(&state.predecessor_route_counts, TEMPERATURE);
    humidity == Some(state.cp401_supply_humidity_ratio_state_owner_count)
        && humidity == Some(state.unchanged_supply_humidity_ratio_preservation_count)
        && enthalpy == Some(state.cp401_supply_enthalpy_state_owner_count)
        && enthalpy == Some(state.unchanged_supply_enthalpy_preservation_count)
        && temperature == Some(state.cp401_supply_temperature_state_owner_count)
        && temperature == Some(state.unchanged_supply_temperature_preservation_count)
}

fn checked_sum(values: &[usize]) -> Option<usize> {
    values
        .iter()
        .try_fold(0usize, |sum, value| sum.checked_add(*value))
}

pub(super) fn checked_selected_sum(
    route_counts: &[usize; 30],
    indices: &[usize],
) -> Option<usize> {
    indices
        .iter()
        .try_fold(0usize, |sum, index| sum.checked_add(route_counts[*index]))
}
