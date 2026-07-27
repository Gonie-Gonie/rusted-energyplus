//! Completed CP318 retained-state validation for its CP319 consumer.

use super::runtime_validation::{
    source_counter_relationships_are_consistent, transition_partition_is_consistent,
};
use super::snapshot_validation::{
    cooling_sensible_flow_snapshot_is_exact_direct_release, cooling_sensible_flow_snapshot_route,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingEconomizerBodySnapshot, PurchasedAirCalcCoolingSensibleFlowSnapshot,
    PurchasedAirUnitRuntimeState,
};

pub(super) fn completed_sensible_flow_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor_body: PurchasedAirCalcCoolingEconomizerBodySnapshot,
    predecessor_flow: PurchasedAirCalcCoolingSensibleFlowSnapshot,
    flow_consumer_latest_witness: Option<PurchasedAirCalcCoolingSensibleFlowSnapshot>,
) -> bool {
    let state = &unit.calc_cooling_sensible_flow;
    let body = &unit.calc_cooling_economizer_body;
    let latest_is_valid = match (
        state.transition_count,
        state.latest,
        state.latest_route,
        state.latest_transition_ordinal,
        flow_consumer_latest_witness,
    ) {
        (
            count,
            Some(latest),
            Some(retained_route),
            Some(latest_transition_ordinal),
            Some(consumer_witness),
        ) if count > 0 => {
            latest == predecessor_flow
                && latest_transition_ordinal == count
                && consumer_witness == latest
                && latest.parent_call_ordinal == count
                && latest.system == state.system
                && unit.controlled_zone == Some(latest.controlled_zone)
                && cooling_sensible_flow_snapshot_is_exact_direct_release(latest)
                && cooling_sensible_flow_snapshot_route(latest) == Some(retained_route)
        }
        _ => false,
    };
    let Some(completed_cooling_count) = body
        .transition_count
        .checked_sub(body.unit_off_skip_count)
        .and_then(|count| count.checked_sub(body.non_cooling_skip_count))
    else {
        return false;
    };
    let histories_link = state.transition_count == body.transition_count
        && state.unit_off_skip_count == body.unit_off_skip_count
        && state.non_cooling_skip_count == body.non_cooling_skip_count
        && state.cooling_body_entry_count == completed_cooling_count;

    transition_partition_is_consistent(state)
        && latest_is_valid
        && histories_link
        && source_counter_relationships_are_consistent(state)
        && sensible_flow_links_to_body(predecessor_flow, predecessor_body)
}

fn sensible_flow_links_to_body(
    flow: PurchasedAirCalcCoolingSensibleFlowSnapshot,
    body: PurchasedAirCalcCoolingEconomizerBodySnapshot,
) -> bool {
    flow.system == body.system
        && flow.parent_call_ordinal == body.parent_call_ordinal
        && flow.controlled_zone == body.controlled_zone
        && flow.unit_body_entered == body.unit_body_entered
        && flow.predecessor_cooling_body_entered == body.predecessor_cooling_body_entered
        && flow.predecessor_maximum_cooling_flow_body_sibling_skipped
            == body.maximum_cooling_flow_body_sibling_skipped
        && flow.predecessor_no_economizer_outer_guard_fallthrough_skipped
            == body.no_economizer_outer_guard_fallthrough_skipped
        && flow.predecessor_economizer_condition_fallthrough_skipped
            == body.economizer_condition_fallthrough_skipped
        && flow.predecessor_economizer_calculation_body_executed
            == body.economizer_calculation_body_executed
        && flow.unit_off_skipped == body.unit_off_skipped
        && flow.non_cooling_skipped == body.non_cooling_skipped
        && flow.cooling_body_entered == body.predecessor_cooling_body_entered
}
