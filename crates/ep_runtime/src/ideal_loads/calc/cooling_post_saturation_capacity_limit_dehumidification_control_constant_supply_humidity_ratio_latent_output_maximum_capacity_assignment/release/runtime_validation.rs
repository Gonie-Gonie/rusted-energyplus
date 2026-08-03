//! Persistent CP405 runtime-state and latest-witness validation.

use ep_model::IdealLoadsAirSystemId;

use super::super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputMaximumCapacityAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputMaximumCapacityAssignmentSnapshot as Snapshot,
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_state as advance,
};
use super::snapshot_validation::{snapshot_route, snapshots_match_bit_exact};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyHumidityRatioAssignmentRuntimeState as PredecessorState,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyHumidityRatioAssignmentSnapshot as Predecessor,
    PurchasedAirUnitRuntimeState,
};

pub(super) fn calc_state_identities_match(
    unit: &PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> bool {
    unit.system == system
        && unit.calc_entry.system == system
        && unit
            .calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_humidity_ratio_assignment
            .system
            == system
        && unit
            .calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment
            .system
            == system
}

pub(super) fn call_order_is_pending(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: Predecessor,
) -> bool {
    let ordinal = predecessor.parent_call_ordinal;
    unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment
        .transition_count
        .checked_add(1)
        == Some(ordinal)
        && unit.calc_entry.call_count == ordinal
        && unit
            .calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_humidity_ratio_assignment
            .transition_count
            == ordinal
}

pub(super) fn pending_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: Predecessor,
    witness: Option<Snapshot>,
) -> bool {
    let state = &unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment;
    state_is_consistent(state, witness, predecessor.system)
        && state.transition_count.checked_add(1) == Some(predecessor.parent_call_ordinal)
}

pub(super) fn prepare_next_transition(
    state: &State,
    predecessor: Predecessor,
) -> Option<(State, Snapshot)> {
    let mut next = state.clone();
    let snapshot = advance(&mut next, predecessor)?;
    Some((next, snapshot))
}

pub(super) fn prepared_completed_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    state: &State,
    snapshot: Snapshot,
) -> bool {
    let predecessor = &unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_humidity_ratio_assignment;
    state.transition_count == predecessor.transition_count
        && state_is_consistent(state, Some(snapshot), snapshot.system)
        && predecessor_counts_match(state, predecessor)
}

pub(super) fn completed_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    snapshot: Snapshot,
    witness: Option<Snapshot>,
) -> bool {
    let state = &unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment;
    let predecessor = &unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_humidity_ratio_assignment;
    state.transition_count == predecessor.transition_count
        && state_is_consistent(state, witness, snapshot.system)
        && predecessor_counts_match(state, predecessor)
        && state
            .latest
            .is_some_and(|latest| snapshots_match_bit_exact(latest, snapshot))
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_latest_metadata_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    expected_transition_count: usize,
) -> bool {
    let state = &unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment;
    state.transition_count == expected_transition_count
        && state_is_consistent(state, state.latest, state.system)
        && predecessor_counts_match(
            state,
            &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_humidity_ratio_assignment,
        )
}

fn predecessor_counts_match(state: &State, predecessor: &PredecessorState) -> bool {
    state.transition_count == predecessor.transition_count
        && state.predecessor_route_counts == predecessor.predecessor_route_counts
        && state.predecessor_guard_false_fallthrough_route_counts
            == predecessor.predecessor_guard_false_fallthrough_route_counts
        && state.cooling_latent_output_maximum_capacity_assignment_route_counts
            == predecessor.supply_humidity_ratio_assignment_route_counts
        && state.inactive_transition_count == predecessor.inactive_transition_count
        && state.predecessor_guard_false_fallthrough_count
            == predecessor.predecessor_guard_false_fallthrough_count
        && state.cooling_latent_output_maximum_capacity_assignment_count
            == predecessor.supply_humidity_ratio_assignment_count
}

fn state_is_consistent(
    state: &State,
    witness: Option<Snapshot>,
    expected_system: IdealLoadsAirSystemId,
) -> bool {
    if state.system != expected_system || !counts_are_exact(state) {
        return false;
    }
    match (
        state.transition_count,
        state.latest,
        state.latest_route,
        state.latest_transition_ordinal,
        witness,
    ) {
        (0, None, None, None, None) => true,
        (count, Some(latest), Some(route), Some(ordinal), Some(witness)) => {
            let retained_count = if !route.guard_evaluated {
                state.predecessor_route_counts[route.predecessor_index]
            } else if route.assignment_executed {
                state.cooling_latent_output_maximum_capacity_assignment_route_counts
                    [route.predecessor_index]
            } else {
                state.predecessor_guard_false_fallthrough_route_counts[route.predecessor_index]
            };
            count > 0
                && ordinal == count
                && retained_count > 0
                && snapshot_route(latest) == Some(route)
                && snapshots_match_bit_exact(latest, witness)
        }
        _ => false,
    }
}

pub(in crate::ideal_loads::calc) fn counts_are_exact(state: &State) -> bool {
    let transition = checked_sum(state.predecessor_route_counts.iter().copied());
    let inactive = checked_selected_sum(
        &state.predecessor_route_counts,
        &[
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 22, 23, 26, 28,
        ],
    );
    let guard_false = checked_sum(
        state
            .predecessor_guard_false_fallthrough_route_counts
            .iter()
            .copied(),
    );
    let assignment = checked_sum(
        state
            .cooling_latent_output_maximum_capacity_assignment_route_counts
            .iter()
            .copied(),
    );
    let humidity_owner =
        checked_selected_sum(&state.predecessor_route_counts, &[18, 19, 22, 23, 26, 28])
            .and_then(|owner| assignment.and_then(|assigned| owner.checked_add(assigned)));
    let enthalpy_owner = checked_selected_sum(
        &state.predecessor_route_counts,
        &[
            5, 8, 11, 14, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29,
        ],
    );
    let temperature_owner = checked_selected_sum(
        &state.predecessor_route_counts,
        &[
            3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
            26, 27, 28, 29,
        ],
    );
    let partition_exact = (0..30).all(|index| {
        let active = matches!(index, 20 | 21 | 24 | 25 | 27 | 29);
        if active {
            state.predecessor_guard_false_fallthrough_route_counts[index].checked_add(
                state.cooling_latent_output_maximum_capacity_assignment_route_counts[index],
            ) == Some(state.predecessor_route_counts[index])
        } else {
            state.predecessor_guard_false_fallthrough_route_counts[index] == 0
                && state.cooling_latent_output_maximum_capacity_assignment_route_counts[index] == 0
        }
    });
    transition == Some(state.transition_count)
        && inactive == Some(state.inactive_transition_count)
        && guard_false == Some(state.predecessor_guard_false_fallthrough_count)
        && assignment == Some(state.cooling_latent_output_maximum_capacity_assignment_count)
        && partition_exact
        && state
            .cooling_latent_output_maximum_capacity_assignment_count
            .checked_mul(2)
            == Some(state.source_site_execution_count)
        && humidity_owner == Some(state.cp404_supply_humidity_ratio_state_owner_count)
        && state.cp404_supply_humidity_ratio_state_owner_count
            == state.unchanged_supply_humidity_ratio_preservation_count
        && enthalpy_owner == Some(state.cp404_supply_enthalpy_state_owner_count)
        && state.cp404_supply_enthalpy_state_owner_count
            == state.unchanged_supply_enthalpy_preservation_count
        && temperature_owner == Some(state.cp404_supply_temperature_state_owner_count)
        && state.cp404_supply_temperature_state_owner_count
            == state.unchanged_supply_temperature_preservation_count
        && [
            state.cp404_retained_maximum_total_cooling_capacity_owned_read_count,
            state.maximum_total_cooling_capacity_read_count,
            state.cooling_latent_output_assignment_write_count,
        ]
        .into_iter()
        .all(|count| count == state.cooling_latent_output_maximum_capacity_assignment_count)
}

fn checked_selected_sum(counts: &[usize; 30], indices: &[usize]) -> Option<usize> {
    checked_sum(indices.iter().map(|index| counts[*index]))
}

fn checked_sum(values: impl IntoIterator<Item = usize>) -> Option<usize> {
    values
        .into_iter()
        .try_fold(0usize, |sum, value| sum.checked_add(value))
}
