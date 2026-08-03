//! Persistent CP408 runtime-state and latest-witness validation.

use ep_model::IdealLoadsAirSystemId;

use super::super::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureMixedAirLimitRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureMixedAirLimitSnapshot as Snapshot,
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit_state as advance,
};
use super::snapshot_validation::{snapshot_route, snapshots_match_bit_exact};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingMixedAirCallSnapshot as MixedAirOwner,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureAssignmentRuntimeState as PredecessorState,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureAssignmentSnapshot as Predecessor,
    PurchasedAirUnitRuntimeState,
};

pub(super) fn calc_state_identities_match(
    unit: &PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> bool {
    unit.system == system
        && unit.calc_entry.system == system
        && unit
            .calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment
            .system
            == system
        && unit
            .calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit
            .system
            == system
}

pub(super) fn call_order_is_pending(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: Predecessor,
) -> bool {
    let ordinal = predecessor.parent_call_ordinal;
    unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit
        .transition_count
        .checked_add(1)
        == Some(ordinal)
        && unit.calc_entry.call_count == ordinal
        && unit
            .calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment
            .transition_count
            == ordinal
}

pub(super) fn pending_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: Predecessor,
    witness: Option<Snapshot>,
) -> bool {
    let state = &unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit;
    state_is_consistent(state, witness, predecessor.system)
        && state.transition_count.checked_add(1) == Some(predecessor.parent_call_ordinal)
}

pub(super) fn prepare_next_transition(
    state: &State,
    predecessor: Predecessor,
    owner: Option<MixedAirOwner>,
) -> Option<(State, Snapshot)> {
    let mut next = state.clone();
    let snapshot = advance(&mut next, predecessor, owner)?;
    Some((next, snapshot))
}

pub(super) fn prepared_completed_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    state: &State,
    snapshot: Snapshot,
) -> bool {
    let predecessor = &unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment;
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
        .calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit;
    let predecessor = &unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment;
    state.transition_count == predecessor.transition_count
        && state_is_consistent(state, witness, snapshot.system)
        && predecessor_counts_match(state, predecessor)
        && state
            .latest
            .is_some_and(|latest| snapshots_match_bit_exact(latest, snapshot))
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit_latest_metadata_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    expected_transition_count: usize,
) -> bool {
    let state = &unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit;
    state.transition_count == expected_transition_count
        && state_is_consistent(state, state.latest, state.system)
        && predecessor_counts_match(
            state,
            &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment,
        )
}

fn predecessor_counts_match(state: &State, predecessor: &PredecessorState) -> bool {
    state.transition_count == predecessor.transition_count
        && state.inactive_transition_count == predecessor.inactive_transition_count
        && state.predecessor_route_counts == predecessor.predecessor_route_counts
        && state.predecessor_guard_false_fallthrough_route_counts
            == predecessor.predecessor_guard_false_fallthrough_route_counts
        && state.predecessor_maximum_capacity_assignment_route_counts
            == predecessor.predecessor_maximum_capacity_assignment_route_counts
        && state.predecessor_else_branch_entry_route_counts
            == predecessor.predecessor_else_branch_entry_route_counts
        && state.predecessor_supply_temperature_assignment_route_counts
            == predecessor.supply_temperature_assignment_route_counts
        && state.supply_temperature_mixed_air_limit_route_counts
            == predecessor.supply_temperature_assignment_route_counts
        && state.predecessor_guard_false_fallthrough_count
            == predecessor.predecessor_guard_false_fallthrough_count
        && state.predecessor_maximum_capacity_assignment_count
            == predecessor.predecessor_maximum_capacity_assignment_count
        && state.predecessor_else_branch_entry_count
            == predecessor.predecessor_else_branch_entry_count
        && state.predecessor_supply_temperature_assignment_count
            == predecessor.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_count
        && state.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit_count
            == predecessor.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_count
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
            let retained_count = if route.active {
                state.supply_temperature_mixed_air_limit_route_counts[route.predecessor_index]
            } else if route.predecessor_maximum_capacity_assignment_executed {
                state.predecessor_maximum_capacity_assignment_route_counts[route.predecessor_index]
            } else {
                state.predecessor_route_counts[route.predecessor_index]
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
    let guard_false = checked_sum(
        state
            .predecessor_guard_false_fallthrough_route_counts
            .iter()
            .copied(),
    );
    let maximum_assignment = checked_sum(
        state
            .predecessor_maximum_capacity_assignment_route_counts
            .iter()
            .copied(),
    );
    let else_entry = checked_sum(
        state
            .predecessor_else_branch_entry_route_counts
            .iter()
            .copied(),
    );
    let predecessor_assignment = checked_sum(
        state
            .predecessor_supply_temperature_assignment_route_counts
            .iter()
            .copied(),
    );
    let limit = checked_sum(
        state
            .supply_temperature_mixed_air_limit_route_counts
            .iter()
            .copied(),
    );
    let temperature_owner = checked_selected_sum(
        &state.predecessor_route_counts,
        &(3..30).collect::<Vec<_>>(),
    );
    let humidity_preserved = checked_selected_sum(
        &state.predecessor_route_counts,
        &[18, 19, 22, 23, 26, 28],
    )
    .and_then(|count| {
        count.checked_add(state.predecessor_maximum_capacity_assignment_count)
    })
    .and_then(|count| {
        count.checked_add(
            state.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit_count,
        )
    });
    let enthalpy_preserved = checked_selected_sum(
        &state.predecessor_route_counts,
        &[
            5, 8, 11, 14, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29,
        ],
    );
    let partition_exact = (0..30).all(|index| {
        let active = matches!(index, 20 | 21 | 24 | 25 | 27 | 29);
        if active {
            state.predecessor_guard_false_fallthrough_route_counts[index]
                .checked_add(state.predecessor_maximum_capacity_assignment_route_counts[index])
                == Some(state.predecessor_route_counts[index])
        } else {
            state.predecessor_guard_false_fallthrough_route_counts[index] == 0
                && state.predecessor_maximum_capacity_assignment_route_counts[index] == 0
        }
    });
    let assigned = state
        .dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit_count;
    transition == Some(state.transition_count)
        && guard_false == Some(state.predecessor_guard_false_fallthrough_count)
        && maximum_assignment == Some(state.predecessor_maximum_capacity_assignment_count)
        && else_entry == Some(state.predecessor_else_branch_entry_count)
        && predecessor_assignment == Some(state.predecessor_supply_temperature_assignment_count)
        && limit == Some(assigned)
        && state.predecessor_else_branch_entry_route_counts
            == state.predecessor_guard_false_fallthrough_route_counts
        && state.predecessor_supply_temperature_assignment_route_counts
            == state.predecessor_else_branch_entry_route_counts
        && state.supply_temperature_mixed_air_limit_route_counts
            == state.predecessor_supply_temperature_assignment_route_counts
        && state.predecessor_else_branch_entry_count == assigned
        && state.predecessor_supply_temperature_assignment_count == assigned
        && state.inactive_transition_count.checked_add(assigned)
            == Some(state.transition_count)
        && partition_exact
        && assigned.checked_mul(PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER.len())
            == Some(state.source_site_execution_count)
        && [
            state.cp407_retained_supply_temperature_owned_read_count,
            state.supply_temperature_for_minimum_read_count,
            state.cp329_retained_mixed_air_temperature_owned_read_count,
            state.mixed_air_temperature_for_minimum_read_count,
            state.source_shaped_two_argument_minimum_evaluation_count,
            state.supply_temperature_assignment_write_count,
        ]
        .into_iter()
        .all(|count| count == assigned)
        && temperature_owner == Some(state.cp407_supply_temperature_state_owner_count)
        && temperature_owner.and_then(|count| count.checked_sub(assigned))
            == Some(state.unchanged_supply_temperature_preservation_count)
        && humidity_preserved == Some(state.unchanged_supply_humidity_ratio_preservation_count)
        && enthalpy_preserved == Some(state.unchanged_supply_enthalpy_preservation_count)
}

fn checked_selected_sum(counts: &[usize; 30], indices: &[usize]) -> Option<usize> {
    checked_sum(indices.iter().map(|index| counts[*index]))
}

fn checked_sum(values: impl IntoIterator<Item = usize>) -> Option<usize> {
    values
        .into_iter()
        .try_fold(0usize, |sum, value| sum.checked_add(value))
}
