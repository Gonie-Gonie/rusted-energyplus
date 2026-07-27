//! Completed CP319 retained-state validation for its CP320 consumer.

use super::runtime_validation::{
    source_counter_relationships_are_consistent, transition_partition_is_consistent,
};
use super::snapshot_validation::{
    cooling_dehumidification_flow_snapshot_is_exact_direct_release,
    cooling_dehumidification_flow_snapshot_route,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingDehumidificationFlowSnapshot,
    PurchasedAirCalcCoolingSensibleFlowSnapshot, PurchasedAirUnitRuntimeState,
};

pub(super) fn completed_dehumidification_flow_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: PurchasedAirCalcCoolingSensibleFlowSnapshot,
    completed: PurchasedAirCalcCoolingDehumidificationFlowSnapshot,
    consumer_witness: Option<PurchasedAirCalcCoolingDehumidificationFlowSnapshot>,
) -> bool {
    let state = &unit.calc_cooling_dehumidification_flow;
    let latest_is_valid = match (
        state.transition_count,
        state.latest,
        state.latest_route,
        state.latest_transition_ordinal,
        consumer_witness,
    ) {
        (count, Some(latest), Some(route), Some(ordinal), Some(witness)) if count > 0 => {
            latest == completed
                && witness == latest
                && ordinal == count
                && latest.parent_call_ordinal == count
                && latest.system == state.system
                && unit.controlled_zone == Some(latest.controlled_zone)
                && cooling_dehumidification_flow_snapshot_is_exact_direct_release(latest)
                && cooling_dehumidification_flow_snapshot_route(latest) == Some(route)
        }
        _ => false,
    };
    let sensible = &unit.calc_cooling_sensible_flow;
    let histories_link = state.transition_count == sensible.transition_count
        && state.unit_off_skip_count == sensible.unit_off_skip_count
        && state.non_cooling_skip_count == sensible.non_cooling_skip_count
        && completed.system == predecessor.system
        && completed.parent_call_ordinal == predecessor.parent_call_ordinal
        && completed.controlled_zone == predecessor.controlled_zone
        && completed.unit_off_skipped == predecessor.unit_off_skipped
        && completed.non_cooling_skipped == predecessor.non_cooling_skipped
        && completed.cooling_body_entered == predecessor.cooling_body_entered;

    transition_partition_is_consistent(state)
        && source_counter_relationships_are_consistent(state)
        && latest_is_valid
        && histories_link
}
