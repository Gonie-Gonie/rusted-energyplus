//! Completed CP317 retained-state validation for its CP318 consumer.

use super::runtime_validation::{
    body_snapshot_is_exact_direct_release, body_snapshot_route, body_source_counters_are_zero,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingEconomizerBodySnapshot,
    PurchasedAirCalcCoolingEconomizerConditionSnapshot, PurchasedAirUnitRuntimeState,
};

pub(super) fn completed_body_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor_condition: PurchasedAirCalcCoolingEconomizerConditionSnapshot,
    predecessor_body: PurchasedAirCalcCoolingEconomizerBodySnapshot,
    body_consumer_latest_witness: Option<PurchasedAirCalcCoolingEconomizerBodySnapshot>,
) -> bool {
    let state = &unit.calc_cooling_economizer_body;
    let condition = &unit.calc_cooling_economizer_condition;
    let transition_partition = state
        .body_execution_count
        .checked_add(state.unit_off_skip_count)
        .and_then(|count| count.checked_add(state.non_cooling_skip_count))
        .and_then(|count| count.checked_add(state.maximum_cooling_flow_body_sibling_skip_count))
        .and_then(|count| count.checked_add(state.no_economizer_outer_guard_fallthrough_skip_count))
        .and_then(|count| count.checked_add(state.economizer_condition_fallthrough_skip_count))
        == Some(state.transition_count);
    let latest_is_valid = match (
        state.transition_count,
        state.latest,
        state.latest_route,
        state.latest_transition_ordinal,
        body_consumer_latest_witness,
    ) {
        (
            count,
            Some(latest),
            Some(retained_route),
            Some(latest_transition_ordinal),
            Some(consumer_witness),
        ) if count > 0 => {
            latest == predecessor_body
                && latest_transition_ordinal == count
                && consumer_witness == latest
                && latest.parent_call_ordinal == count
                && latest.system == state.system
                && unit.controlled_zone == Some(latest.controlled_zone)
                && body_snapshot_is_exact_direct_release(latest)
                && body_snapshot_route(latest) == Some(retained_route)
        }
        _ => false,
    };
    let histories_link = state.transition_count == condition.transition_count
        && state.unit_off_skip_count == condition.unit_off_skip_count
        && state.non_cooling_skip_count == condition.non_cooling_skip_count
        && state.maximum_cooling_flow_body_sibling_skip_count
            == condition.maximum_cooling_flow_body_sibling_skip_count
        && state.no_economizer_outer_guard_fallthrough_skip_count
            == condition.no_economizer_outer_guard_fallthrough_skip_count
        && state.body_execution_count == condition.economizer_calculation_body_entry_count
        && state
            .economizer_condition_fallthrough_skip_count
            .checked_add(state.body_execution_count)
            == Some(condition.condition_evaluation_count);

    transition_partition
        && latest_is_valid
        && histories_link
        && body_links_to_condition(predecessor_body, predecessor_condition)
        && state.body_execution_count == 0
        && state.maximum_cooling_flow_body_sibling_skip_count == 0
        && state.economizer_condition_fallthrough_skip_count == 0
        && body_source_counters_are_zero(state)
}

pub(super) fn body_links_to_condition(
    body: PurchasedAirCalcCoolingEconomizerBodySnapshot,
    condition: PurchasedAirCalcCoolingEconomizerConditionSnapshot,
) -> bool {
    body.system == condition.system
        && body.parent_call_ordinal == condition.parent_call_ordinal
        && body.controlled_zone == condition.controlled_zone
        && body.unit_body_entered == condition.unit_body_entered
        && body.predecessor_cooling_body_entered == condition.predecessor_cooling_body_entered
        && body.predecessor_maximum_cooling_flow_body_entered
            == condition.predecessor_maximum_cooling_flow_body_entered
        && body.predecessor_active_guard_false_economizer_fallthrough
            == condition.predecessor_active_guard_false_economizer_fallthrough
        && body.predecessor_economizer_guard_evaluated
            == condition.predecessor_economizer_guard_evaluated
        && body.predecessor_economizer_body_entered == condition.predecessor_economizer_body_entered
        && body.predecessor_no_economizer_fallthrough
            == condition.predecessor_no_economizer_fallthrough
        && body.predecessor_economizer_condition_evaluated
            == condition.economizer_condition_evaluated
        && body.predecessor_economizer_condition_satisfied
            == condition.economizer_condition_satisfied
        && body.predecessor_economizer_calculation_body_entered
            == condition.economizer_calculation_body_entered
        && body.unit_off_skipped == condition.unit_off_skipped
        && body.non_cooling_skipped == condition.non_cooling_skipped
        && body.maximum_cooling_flow_body_sibling_skipped
            == condition.maximum_cooling_flow_body_sibling_skipped
        && body.no_economizer_outer_guard_fallthrough_skipped
            == condition.no_economizer_outer_guard_fallthrough_skipped
        && body.economizer_condition_fallthrough_skipped
            == condition.economizer_condition_fallthrough
        && body.economizer_calculation_body_executed
            == condition.economizer_calculation_body_entered
}
