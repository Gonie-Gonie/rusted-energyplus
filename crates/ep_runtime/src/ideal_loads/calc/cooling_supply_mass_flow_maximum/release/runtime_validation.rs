use ep_model::{AutosizeOrNumber, IdealLoadsAirSystem, IdealLoadsAirSystemId, ZoneId};

use super::snapshot_validation::{
    cooling_supply_mass_flow_maximum_snapshot_is_exact_direct_release,
    cooling_supply_mass_flow_maximum_snapshot_route,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowMaximumRuntimeState,
    PurchasedAirCalcCoolingSupplyMassFlowMaximumSnapshot, PurchasedAirUnitRuntimeState,
};

pub(super) fn calc_state_identities_match(
    unit: &PurchasedAirUnitRuntimeState,
    selected: IdealLoadsAirSystemId,
) -> bool {
    unit.system == selected
        && unit.calc_entry.system == selected
        && unit.calc_minimum_oa_prefix.system == selected
        && unit.calc_cooling_capacity_zero_flow_reset.system == selected
        && unit.calc_cooling_supply_mass_flow_maximum.system == selected
}

pub(super) fn call_order_is_pending_supply_maximum(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot,
) -> bool {
    unit.init_call_count == unit.calc_entry.call_count
        && unit.calc_cooling_capacity_zero_flow_reset.transition_count == unit.calc_entry.call_count
        && unit
            .calc_cooling_supply_mass_flow_maximum
            .transition_count
            .checked_add(1)
            == Some(unit.calc_cooling_capacity_zero_flow_reset.transition_count)
        && predecessor.parent_call_ordinal
            == unit.calc_cooling_capacity_zero_flow_reset.transition_count
}

pub(super) fn completed_capacity_zero_reset_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot,
    witness: Option<PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot>,
) -> bool {
    let state = &unit.calc_cooling_capacity_zero_flow_reset;
    let cooling_count = state.cooling_body_entry_count;
    let capacity_count =
        usize::from(system.cooling_limit == ep_model::IdealLoadsLimit::LimitCapacity)
            * cooling_count;
    let combined_count =
        usize::from(system.cooling_limit == ep_model::IdealLoadsLimit::LimitFlowRateAndCapacity)
            * cooling_count;
    let selected_count = capacity_count + combined_count;
    let selected_capacity_is_zero = selected_count > 0
        && matches!(
            system.maximum_total_cooling_capacity_w,
            Some(AutosizeOrNumber::Value(value)) if value == 0.0
        );
    let expected_zero_count = usize::from(selected_capacity_is_zero) * cooling_count;
    state.latest == Some(predecessor)
        && witness == Some(predecessor)
        && super::super::super::cooling_capacity_zero_flow_reset::
            cooling_capacity_zero_flow_reset_snapshot_is_exact_direct_release(predecessor)
        && partition_is_consistent(
            state.transition_count,
            state.unit_off_skip_count,
            state.non_cooling_skip_count,
            cooling_count,
        )
        && state.transition_count == unit.calc_entry.call_count
        && state.first_cooling_limit_read_count == cooling_count
        && state.cooling_limit_capacity_count == capacity_count
        && state.second_cooling_limit_read_count == cooling_count - capacity_count
        && state.cooling_limit_flow_rate_and_capacity_count == combined_count
        && state.cooling_limit_rejected_count == cooling_count - selected_count
        && state.maximum_total_cooling_capacity_read_count == selected_count
        && state.maximum_total_cooling_capacity_comparison_count == selected_count
        && state.maximum_total_cooling_capacity_zero_count == expected_zero_count
        && state.maximum_total_cooling_capacity_nonzero_count
            == selected_count - expected_zero_count
        && state.zero_cooling_capacity_body_entry_count
            == state.maximum_total_cooling_capacity_zero_count
        && state.supply_mass_flow_rate_for_cool_zero_assignment_count
            == state.zero_cooling_capacity_body_entry_count
        && state.supply_mass_flow_rate_for_dehumidification_zero_assignment_count
            == state.zero_cooling_capacity_body_entry_count
        && state.supply_mass_flow_rate_for_humidification_zero_assignment_count
            == state.zero_cooling_capacity_body_entry_count
}

pub(super) fn pending_supply_maximum_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot,
    witness: Option<PurchasedAirCalcCoolingSupplyMassFlowMaximumSnapshot>,
) -> bool {
    let state = &unit.calc_cooling_supply_mass_flow_maximum;
    partition_is_consistent(
        state.transition_count,
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        state.cooling_body_entry_count,
    ) && latest_is_valid(state, unit.controlled_zone, witness)
        && state.transition_count.checked_add(1)
            == Some(unit.calc_cooling_capacity_zero_flow_reset.transition_count)
        && state
            .unit_off_skip_count
            .checked_add(usize::from(predecessor.unit_off_skipped))
            == Some(
                unit.calc_cooling_capacity_zero_flow_reset
                    .unit_off_skip_count,
            )
        && state
            .non_cooling_skip_count
            .checked_add(usize::from(predecessor.non_cooling_skipped))
            == Some(
                unit.calc_cooling_capacity_zero_flow_reset
                    .non_cooling_skip_count,
            )
        && source_counters_are_consistent(state)
}

fn partition_is_consistent(
    transition_count: usize,
    unit_off_count: usize,
    non_cooling_count: usize,
    cooling_count: usize,
) -> bool {
    unit_off_count
        .checked_add(non_cooling_count)
        .and_then(|count| count.checked_add(cooling_count))
        == Some(transition_count)
}

fn latest_is_valid(
    state: &PurchasedAirCalcCoolingSupplyMassFlowMaximumRuntimeState,
    controlled_zone: Option<ZoneId>,
    witness: Option<PurchasedAirCalcCoolingSupplyMassFlowMaximumSnapshot>,
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
                && cooling_supply_mass_flow_maximum_snapshot_is_exact_direct_release(latest)
                && cooling_supply_mass_flow_maximum_snapshot_route(latest) == Some(route)
        }
        _ => false,
    }
}

fn source_counters_are_consistent(
    state: &PurchasedAirCalcCoolingSupplyMassFlowMaximumRuntimeState,
) -> bool {
    [
        state.outdoor_air_mass_flow_rate_read_count,
        state.supply_mass_flow_rate_for_cool_read_count,
        state.supply_mass_flow_rate_for_dehumidification_read_count,
        state.supply_mass_flow_rate_for_humidification_read_count,
        state.positive_zero_vs_outdoor_air_comparison_count,
        state.cooling_vs_dehumidification_comparison_count,
        state.leading_vs_candidate_pair_comparison_count,
        state.leading_vs_humidification_comparison_count,
        state.maximum_evaluation_count,
        state.supply_mass_flow_rate_assignment_count,
    ]
    .into_iter()
    .all(|count| count == state.cooling_body_entry_count)
}
