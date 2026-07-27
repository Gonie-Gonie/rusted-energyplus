//! Run-summary evidence for the bounded PurchasedAir cooling sensible flow.

use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_SOURCE,
    PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_SOURCE_ORDER,
    PurchasedAirCalcCoolingEconomizerBodyLifecycleSummary,
    PurchasedAirCalcCoolingEconomizerBodySnapshot,
    PurchasedAirCalcCoolingSensibleFlowLifecycleSummary,
    PurchasedAirCalcCoolingSensibleFlowSnapshot, PurchasedAirInitLifecycleSummary,
};

mod serialization;
mod validation;

pub(super) use serialization::lifecycle_json;
use validation::{snapshot_shape, validate_source_counters};

pub(super) fn validate_direct_lifecycle(
    lifecycle: Option<&PurchasedAirCalcCoolingSensibleFlowLifecycleSummary>,
    predecessor_lifecycle: Option<&PurchasedAirCalcCoolingEconomizerBodyLifecycleSummary>,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle = lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads runtime did not expose cooling sensible-flow evidence".to_string()
    })?;
    let predecessor_lifecycle = predecessor_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads cooling sensible flow has no economizer-body evidence".to_string()
    })?;
    let init_lifecycle = init_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads cooling sensible flow has no initialization evidence".to_string()
    })?;
    let coupling_call_count = coupling_call_count.ok_or_else(|| {
        "direct-zone IdealLoads cooling sensible flow has no coupling call count".to_string()
    })?;
    let state = &lifecycle.state;
    let predecessor = &predecessor_lifecycle.state;

    if coupling_call_count == 0
        || lifecycle.source != PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_FIRST_EXCLUDED_SOURCE
        || predecessor_lifecycle.source != PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_SOURCE
        || predecessor_lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_FIRST_EXCLUDED_SOURCE
    {
        return Err(
            "direct-zone IdealLoads cooling sensible-flow provenance is invalid".to_string(),
        );
    }

    let skipped_count = checked_add(
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        "skip partition",
    )?;
    let transition_partition = checked_add(
        skipped_count,
        state.cooling_body_entry_count,
        "transition partition",
    )?;
    let predecessor_skipped_count = checked_add(
        predecessor.unit_off_skip_count,
        predecessor.non_cooling_skip_count,
        "predecessor skip partition",
    )?;
    let predecessor_cooling_count = predecessor
        .transition_count
        .checked_sub(predecessor_skipped_count)
        .ok_or_else(|| {
            "direct-zone IdealLoads cooling sensible-flow predecessor skip partition is invalid"
                .to_string()
        })?;
    for (field, expected, actual) in [
        (
            "transition_count",
            coupling_call_count,
            state.transition_count,
        ),
        (
            "predecessor_transition_count",
            predecessor.transition_count,
            state.transition_count,
        ),
        (
            "transition_partition",
            state.transition_count,
            transition_partition,
        ),
        (
            "unit_off_skip_count",
            predecessor.unit_off_skip_count,
            state.unit_off_skip_count,
        ),
        (
            "non_cooling_skip_count",
            predecessor.non_cooling_skip_count,
            state.non_cooling_skip_count,
        ),
        (
            "cooling_body_entry_count",
            predecessor_cooling_count,
            state.cooling_body_entry_count,
        ),
    ] {
        if actual != expected {
            return Err(format!(
                "direct-zone IdealLoads cooling sensible-flow invariant {field} expected {expected}, got {actual}"
            ));
        }
    }
    validate_source_counters(state)?;

    let latest = state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads cooling sensible flow has no latest snapshot".to_string()
    })?;
    let latest_predecessor = predecessor.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads cooling sensible flow has no latest economizer-body snapshot"
            .to_string()
    })?;
    let expected_system = init_lifecycle
        .declared_system_order
        .first()
        .copied()
        .ok_or_else(|| {
            "direct-zone IdealLoads cooling sensible flow has no declared system".to_string()
        })?;
    let expected_zone = init_lifecycle.controlled_zone.ok_or_else(|| {
        "direct-zone IdealLoads cooling sensible flow has no controlled Zone".to_string()
    })?;
    if state.system != expected_system
        || predecessor.system != expected_system
        || !latest_matches_release(
            latest,
            latest_predecessor,
            expected_system,
            expected_zone,
            coupling_call_count,
        )
    {
        return Err(
            "direct-zone IdealLoads cooling sensible-flow latest state is not release-ready"
                .to_string(),
        );
    }
    Ok(())
}

fn latest_matches_release(
    flow: &PurchasedAirCalcCoolingSensibleFlowSnapshot,
    predecessor: &PurchasedAirCalcCoolingEconomizerBodySnapshot,
    expected_system: ep_model::IdealLoadsAirSystemId,
    expected_zone: ep_model::ZoneId,
    call_count: usize,
) -> bool {
    flow.source == PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_SOURCE
        && flow.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_FIRST_EXCLUDED_SOURCE
        && flow.source_order == PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_SOURCE_ORDER
        && predecessor.source == PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_SOURCE
        && predecessor.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_FIRST_EXCLUDED_SOURCE
        && predecessor.source_order == PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_SOURCE_ORDER
        && flow.system == expected_system
        && flow.system == predecessor.system
        && flow.parent_call_ordinal == call_count
        && flow.parent_call_ordinal == predecessor.parent_call_ordinal
        && flow.controlled_zone == expected_zone
        && flow.controlled_zone == predecessor.controlled_zone
        && flow.unit_body_entered == predecessor.unit_body_entered
        && flow.predecessor_cooling_body_entered == predecessor.predecessor_cooling_body_entered
        && flow.predecessor_maximum_cooling_flow_body_sibling_skipped
            == predecessor.maximum_cooling_flow_body_sibling_skipped
        && flow.predecessor_no_economizer_outer_guard_fallthrough_skipped
            == predecessor.no_economizer_outer_guard_fallthrough_skipped
        && flow.predecessor_economizer_condition_fallthrough_skipped
            == predecessor.economizer_condition_fallthrough_skipped
        && flow.predecessor_economizer_calculation_body_executed
            == predecessor.economizer_calculation_body_executed
        && flow.unit_off_skipped == predecessor.unit_off_skipped
        && flow.non_cooling_skipped == predecessor.non_cooling_skipped
        && flow.cooling_body_entered == predecessor.predecessor_cooling_body_entered
        && snapshot_shape(flow)
}

fn checked_add(left: usize, right: usize, label: &str) -> Result<usize, String> {
    left.checked_add(right)
        .ok_or_else(|| format!("direct-zone IdealLoads cooling sensible-flow {label} overflowed"))
}
