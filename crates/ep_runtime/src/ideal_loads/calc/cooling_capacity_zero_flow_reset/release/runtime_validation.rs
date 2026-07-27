use ep_model::{IdealLoadsAirSystemId, ZoneId};

use super::snapshot_validation::{
    cooling_capacity_zero_flow_reset_snapshot_is_exact_direct_release,
    cooling_capacity_zero_flow_reset_snapshot_route,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingCapacityZeroFlowResetRuntimeState,
    PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot,
    PurchasedAirCalcCoolingHumidificationFlowSnapshot, PurchasedAirUnitRuntimeState,
};

pub(super) fn calc_state_identities_match(
    unit: &PurchasedAirUnitRuntimeState,
    selected: IdealLoadsAirSystemId,
) -> bool {
    unit.system == selected
        && unit.calc_entry.system == selected
        && unit.calc_cooling_sensible_flow.system == selected
        && unit.calc_cooling_dehumidification_flow.system == selected
        && unit.calc_cooling_humidification_flow.system == selected
        && unit.calc_cooling_capacity_zero_flow_reset.system == selected
}

pub(super) fn call_order_is_pending_capacity_zero_reset(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: PurchasedAirCalcCoolingHumidificationFlowSnapshot,
) -> bool {
    unit.init_call_count == unit.calc_entry.call_count
        && unit.calc_cooling_humidification_flow.transition_count == unit.calc_entry.call_count
        && unit
            .calc_cooling_capacity_zero_flow_reset
            .transition_count
            .checked_add(1)
            == Some(unit.calc_cooling_humidification_flow.transition_count)
        && predecessor.parent_call_ordinal == unit.calc_cooling_humidification_flow.transition_count
}

pub(super) fn pending_capacity_zero_reset_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: PurchasedAirCalcCoolingHumidificationFlowSnapshot,
    witness: Option<PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot>,
) -> bool {
    let state = &unit.calc_cooling_capacity_zero_flow_reset;
    partition_is_consistent(state)
        && latest_is_valid(state, unit.controlled_zone, witness)
        && state.transition_count.checked_add(1)
            == Some(unit.calc_cooling_humidification_flow.transition_count)
        && state
            .unit_off_skip_count
            .checked_add(usize::from(predecessor.unit_off_skipped))
            == Some(unit.calc_cooling_humidification_flow.unit_off_skip_count)
        && state
            .non_cooling_skip_count
            .checked_add(usize::from(predecessor.non_cooling_skipped))
            == Some(unit.calc_cooling_humidification_flow.non_cooling_skip_count)
        && source_counter_relationships_are_consistent(state)
}

fn partition_is_consistent(
    state: &PurchasedAirCalcCoolingCapacityZeroFlowResetRuntimeState,
) -> bool {
    state
        .unit_off_skip_count
        .checked_add(state.non_cooling_skip_count)
        .and_then(|count| count.checked_add(state.cooling_body_entry_count))
        == Some(state.transition_count)
}

fn latest_is_valid(
    state: &PurchasedAirCalcCoolingCapacityZeroFlowResetRuntimeState,
    controlled_zone: Option<ZoneId>,
    witness: Option<PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot>,
) -> bool {
    match (
        state.transition_count,
        state.latest,
        state.latest_route,
        state.latest_transition_ordinal,
        witness,
    ) {
        (0, None, None, None, None) => true,
        (count, Some(latest), Some(route), Some(ordinal), Some(witness)) if count > 0 => {
            latest == witness
                && ordinal == count
                && latest.parent_call_ordinal == count
                && latest.system == state.system
                && controlled_zone == Some(latest.controlled_zone)
                && cooling_capacity_zero_flow_reset_snapshot_is_exact_direct_release(latest)
                && cooling_capacity_zero_flow_reset_snapshot_route(latest) == Some(route)
        }
        _ => false,
    }
}

fn source_counter_relationships_are_consistent(
    state: &PurchasedAirCalcCoolingCapacityZeroFlowResetRuntimeState,
) -> bool {
    let limit_partition = state
        .cooling_limit_capacity_count
        .checked_add(state.cooling_limit_flow_rate_and_capacity_count)
        .and_then(|count| count.checked_add(state.cooling_limit_rejected_count));
    let capacity_partition = state
        .maximum_total_cooling_capacity_zero_count
        .checked_add(state.maximum_total_cooling_capacity_nonzero_count);
    let first_read_partition = state
        .second_cooling_limit_read_count
        .checked_add(state.cooling_limit_capacity_count);
    let selected_capacity_count = state
        .cooling_limit_capacity_count
        .checked_add(state.cooling_limit_flow_rate_and_capacity_count);
    state.first_cooling_limit_read_count == state.cooling_body_entry_count
        && first_read_partition == Some(state.cooling_body_entry_count)
        && limit_partition == Some(state.cooling_body_entry_count)
        && selected_capacity_count == Some(state.maximum_total_cooling_capacity_read_count)
        && state.maximum_total_cooling_capacity_comparison_count
            == state.maximum_total_cooling_capacity_read_count
        && capacity_partition == Some(state.maximum_total_cooling_capacity_read_count)
        && state.zero_cooling_capacity_body_entry_count
            == state.maximum_total_cooling_capacity_zero_count
        && state.supply_mass_flow_rate_for_cool_zero_assignment_count
            == state.zero_cooling_capacity_body_entry_count
        && state.supply_mass_flow_rate_for_dehumidification_zero_assignment_count
            == state.zero_cooling_capacity_body_entry_count
        && state.supply_mass_flow_rate_for_humidification_zero_assignment_count
            == state.zero_cooling_capacity_body_entry_count
}
