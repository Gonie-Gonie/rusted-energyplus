//! Completed CP320 retained-state validation for its CP321 consumer.

use super::runtime_validation::{
    direct_counter_relationships_are_consistent, partition_is_consistent,
};
use super::snapshot_validation::{
    cooling_humidification_flow_snapshot_is_exact_direct_release,
    cooling_humidification_flow_snapshot_route,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingDehumidificationFlowSnapshot,
    PurchasedAirCalcCoolingHumidificationFlowSnapshot, PurchasedAirUnitRuntimeState,
};

pub(super) fn completed_humidification_flow_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: PurchasedAirCalcCoolingDehumidificationFlowSnapshot,
    completed: PurchasedAirCalcCoolingHumidificationFlowSnapshot,
    consumer_witness: Option<PurchasedAirCalcCoolingHumidificationFlowSnapshot>,
) -> bool {
    let state = &unit.calc_cooling_humidification_flow;
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
                && cooling_humidification_flow_snapshot_is_exact_direct_release(latest)
                && cooling_humidification_flow_snapshot_route(latest) == Some(route)
        }
        _ => false,
    };
    let prior = &unit.calc_cooling_dehumidification_flow;
    let histories_link = state.transition_count == prior.transition_count
        && state.unit_off_skip_count == prior.unit_off_skip_count
        && state.non_cooling_skip_count == prior.non_cooling_skip_count
        && completed.system == predecessor.system
        && completed.parent_call_ordinal == predecessor.parent_call_ordinal
        && completed.controlled_zone == predecessor.controlled_zone
        && completed.unit_off_skipped == predecessor.unit_off_skipped
        && completed.non_cooling_skipped == predecessor.non_cooling_skipped
        && completed.predecessor_cooling_body_entered == predecessor.cooling_body_entered
        && completed.cooling_body_entered == predecessor.cooling_body_entered;

    partition_is_consistent(state)
        && direct_counter_relationships_are_consistent(state)
        && latest_is_valid
        && histories_link
}
