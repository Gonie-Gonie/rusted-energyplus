//! Completed CP316 retained-state validation for its CP317 consumer.

use super::super::PurchasedAirCalcCoolingEconomizerConditionSnapshot;
use super::runtime_validation::{
    condition_snapshot_is_exact_direct_release, condition_snapshot_route,
    condition_source_counters_are_zero,
};
use crate::ideal_loads::PurchasedAirUnitRuntimeState;

pub(super) fn completed_condition_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: PurchasedAirCalcCoolingEconomizerConditionSnapshot,
    condition_consumer_latest_witness: Option<PurchasedAirCalcCoolingEconomizerConditionSnapshot>,
) -> bool {
    let state = &unit.calc_cooling_economizer_condition;
    let guard = &unit.calc_cooling_economizer_guard;
    let transition_partition = state
        .condition_evaluation_count
        .checked_add(state.unit_off_skip_count)
        .and_then(|count| count.checked_add(state.non_cooling_skip_count))
        .and_then(|count| count.checked_add(state.maximum_cooling_flow_body_sibling_skip_count))
        .and_then(|count| {
            count.checked_add(state.no_economizer_outer_guard_fallthrough_skip_count)
        })
        == Some(state.transition_count);
    let latest_is_valid = match (
        state.transition_count,
        state.latest,
        state.latest_route,
        state.latest_transition_ordinal,
        condition_consumer_latest_witness,
    ) {
        (
            count,
            Some(latest),
            Some(retained_route),
            Some(latest_transition_ordinal),
            Some(consumer_witness),
        ) if count > 0 => {
            latest == predecessor
                && latest_transition_ordinal == count
                && consumer_witness == latest
                && latest.parent_call_ordinal == count
                && latest.system == state.system
                && unit.controlled_zone == Some(latest.controlled_zone)
                && condition_snapshot_is_exact_direct_release(latest)
                && condition_snapshot_route(latest) == Some(retained_route)
        }
        _ => false,
    };
    let histories_link = state.transition_count == guard.transition_count
        && state.unit_off_skip_count == guard.unit_off_skip_count
        && state.non_cooling_skip_count == guard.non_cooling_skip_count
        && state.maximum_cooling_flow_body_sibling_skip_count
            == guard.maximum_cooling_flow_body_sibling_skip_count
        && state.no_economizer_outer_guard_fallthrough_skip_count
            == guard.no_economizer_fallthrough_count
        && state.condition_evaluation_count == guard.economizer_body_entry_count;

    transition_partition
        && latest_is_valid
        && histories_link
        && state.condition_evaluation_count == 0
        && state.maximum_cooling_flow_body_sibling_skip_count == 0
        && condition_source_counters_are_zero(state)
}
