//! Run-summary evidence for the bounded PurchasedAir cooling OA maximum-flow gate.

use ep_model::IdealLoadsLimit;
use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_SOURCE,
    PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_SOURCE,
    PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_SOURCE_ORDER,
    PurchasedAirCalcCoolingEntryGateLifecycleSummary, PurchasedAirCalcCoolingEntryGateSnapshot,
    PurchasedAirCalcCoolingOaMaxFlowGateLifecycleSummary,
    PurchasedAirCalcCoolingOaMaxFlowGateSnapshot, PurchasedAirInitLifecycleSummary,
};
use serde_json::{Value, json};

pub(super) fn lifecycle_json(
    lifecycle: &PurchasedAirCalcCoolingOaMaxFlowGateLifecycleSummary,
) -> Value {
    let latest = lifecycle.state.latest.map(snapshot_json);
    json!({
        "source": lifecycle.source,
        "first_excluded_source": lifecycle.first_excluded_source,
        "system": lifecycle.state.system.0,
        "transition_count": lifecycle.state.transition_count,
        "source_execution_count": lifecycle.state.source_execution_count,
        "unit_off_skip_count": lifecycle.state.unit_off_skip_count,
        "non_cooling_skip_count": lifecycle.state.non_cooling_skip_count,
        "cooling_limit_flow_rate_comparison_count":
            lifecycle.state.cooling_limit_flow_rate_comparison_count,
        "cooling_limit_flow_rate_match_count":
            lifecycle.state.cooling_limit_flow_rate_match_count,
        "cooling_limit_flow_rate_and_capacity_comparison_count":
            lifecycle.state.cooling_limit_flow_rate_and_capacity_comparison_count,
        "cooling_limit_flow_rate_and_capacity_match_count":
            lifecycle.state.cooling_limit_flow_rate_and_capacity_match_count,
        "outdoor_air_mass_flow_rate_read_count":
            lifecycle.state.outdoor_air_mass_flow_rate_read_count,
        "maximum_cooling_air_mass_flow_rate_read_count":
            lifecycle.state.maximum_cooling_air_mass_flow_rate_read_count,
        "strict_mass_flow_comparison_count":
            lifecycle.state.strict_mass_flow_comparison_count,
        "strict_mass_flow_comparison_satisfied_count":
            lifecycle.state.strict_mass_flow_comparison_satisfied_count,
        "maximum_cooling_flow_body_entry_count":
            lifecycle.state.maximum_cooling_flow_body_entry_count,
        "active_fallthrough_count": lifecycle.state.active_fallthrough_count,
        "latest": latest,
    })
}

pub(super) fn validate_direct_lifecycle(
    lifecycle: Option<&PurchasedAirCalcCoolingOaMaxFlowGateLifecycleSummary>,
    predecessor_lifecycle: Option<&PurchasedAirCalcCoolingEntryGateLifecycleSummary>,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle = lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads runtime did not expose cooling OA maximum-flow gate evidence"
            .to_string()
    })?;
    let predecessor_lifecycle = predecessor_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads cooling OA maximum-flow gate has no cooling-entry evidence"
            .to_string()
    })?;
    let init_lifecycle = init_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads cooling OA maximum-flow gate has no initialization evidence"
            .to_string()
    })?;
    let coupling_call_count = coupling_call_count.ok_or_else(|| {
        "direct-zone IdealLoads cooling OA maximum-flow gate has no coupling call count".to_string()
    })?;
    let state = &lifecycle.state;
    let predecessor = &predecessor_lifecycle.state;
    let skip_count = checked_add(
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        "skip partition",
    )?;
    let transition_partition = checked_add(
        state.source_execution_count,
        skip_count,
        "transition partition",
    )?;
    let expected_second_comparisons = state
        .source_execution_count
        .checked_sub(state.cooling_limit_flow_rate_match_count)
        .ok_or_else(|| {
            "direct-zone IdealLoads cooling OA maximum-flow second selector underflowed".to_string()
        })?;
    let selected_flow_count = checked_add(
        state.cooling_limit_flow_rate_match_count,
        state.cooling_limit_flow_rate_and_capacity_match_count,
        "selected-flow partition",
    )?;
    let active_partition = checked_add(
        state.maximum_cooling_flow_body_entry_count,
        state.active_fallthrough_count,
        "active partition",
    )?;
    if coupling_call_count == 0
        || lifecycle.source != PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_FIRST_EXCLUDED_SOURCE
        || predecessor_lifecycle.source != PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_SOURCE
        || predecessor_lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_FIRST_EXCLUDED_SOURCE
    {
        return Err(
            "direct-zone IdealLoads cooling OA maximum-flow provenance is invalid".to_string(),
        );
    }
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
            "source_execution_count",
            predecessor.cooling_body_entry_count,
            state.source_execution_count,
        ),
        (
            "unit_off_skip_count",
            predecessor.unit_off_skip_count,
            state.unit_off_skip_count,
        ),
        (
            "non_cooling_skip_count",
            predecessor.active_fallthrough_count,
            state.non_cooling_skip_count,
        ),
        (
            "flow_rate_comparison_count",
            state.source_execution_count,
            state.cooling_limit_flow_rate_comparison_count,
        ),
        (
            "second_selector_comparison_count",
            expected_second_comparisons,
            state.cooling_limit_flow_rate_and_capacity_comparison_count,
        ),
        (
            "outdoor_air_mass_flow_rate_read_count",
            selected_flow_count,
            state.outdoor_air_mass_flow_rate_read_count,
        ),
        (
            "maximum_cooling_air_mass_flow_rate_read_count",
            selected_flow_count,
            state.maximum_cooling_air_mass_flow_rate_read_count,
        ),
        (
            "strict_mass_flow_comparison_count",
            selected_flow_count,
            state.strict_mass_flow_comparison_count,
        ),
        (
            "strict_mass_flow_comparison_satisfied_count",
            0,
            state.strict_mass_flow_comparison_satisfied_count,
        ),
        (
            "maximum_cooling_flow_body_entry_count",
            0,
            state.maximum_cooling_flow_body_entry_count,
        ),
        (
            "active_fallthrough_count",
            state.source_execution_count,
            state.active_fallthrough_count,
        ),
        (
            "transition_partition",
            coupling_call_count,
            transition_partition,
        ),
        (
            "active_partition",
            state.source_execution_count,
            active_partition,
        ),
    ] {
        if actual != expected {
            return Err(format!(
                "direct-zone IdealLoads cooling OA maximum-flow invariant {field} expected {expected}, got {actual}"
            ));
        }
    }
    if state.cooling_limit_flow_rate_match_count != 0
        && state.cooling_limit_flow_rate_match_count != state.source_execution_count
    {
        return Err(
            "direct-zone IdealLoads cooling OA maximum-flow first-selector history is inconsistent"
                .to_string(),
        );
    }
    if state.cooling_limit_flow_rate_and_capacity_match_count != 0
        && state.cooling_limit_flow_rate_and_capacity_match_count
            != state.cooling_limit_flow_rate_and_capacity_comparison_count
    {
        return Err(
            "direct-zone IdealLoads cooling OA maximum-flow second-selector history is inconsistent"
                .to_string(),
        );
    }
    let latest = state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads cooling OA maximum-flow gate has no latest snapshot".to_string()
    })?;
    let latest_predecessor = predecessor.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads cooling OA maximum-flow gate has no latest cooling-entry snapshot"
            .to_string()
    })?;
    let expected_system = init_lifecycle
        .declared_system_order
        .first()
        .copied()
        .ok_or_else(|| {
            "direct-zone IdealLoads cooling OA maximum-flow gate has no declared system".to_string()
        })?;
    let expected_zone = init_lifecycle.controlled_zone.ok_or_else(|| {
        "direct-zone IdealLoads cooling OA maximum-flow gate has no controlled Zone".to_string()
    })?;
    let maximum_cooling_mass_flow = init_lifecycle.maximum_cooling_air_mass_flow_rate_kg_per_s;
    if !maximum_cooling_mass_flow.is_finite()
        || maximum_cooling_mass_flow < 0.0
        || state.system != expected_system
        || predecessor.system != expected_system
        || !latest_matches_release(
            latest,
            latest_predecessor,
            expected_system,
            expected_zone,
            maximum_cooling_mass_flow,
            coupling_call_count,
        )
    {
        return Err(
            "direct-zone IdealLoads cooling OA maximum-flow latest state is not release-ready"
                .to_string(),
        );
    }
    Ok(())
}

fn snapshot_json(snapshot: PurchasedAirCalcCoolingOaMaxFlowGateSnapshot) -> Value {
    json!({
        "source": snapshot.source,
        "first_excluded_source": snapshot.first_excluded_source,
        "system": snapshot.system.0,
        "parent_call_ordinal": snapshot.parent_call_ordinal,
        "source_order": snapshot.source_order,
        "controlled_zone": snapshot.controlled_zone.0,
        "unit_body_entered": snapshot.unit_body_entered,
        "predecessor_cooling_body_entered": snapshot.predecessor_cooling_body_entered,
        "unit_off_skipped": snapshot.unit_off_skipped,
        "non_cooling_skipped": snapshot.non_cooling_skipped,
        "cooling_limit_flow_rate_comparison_evaluated":
            snapshot.cooling_limit_flow_rate_comparison_evaluated,
        "cooling_limit_flow_rate_read": snapshot.cooling_limit_flow_rate_read,
        "cooling_limit_flow_rate_value": snapshot
            .cooling_limit_flow_rate_value
            .map(limit_name),
        "cooling_limit_flow_rate_comparison_satisfied":
            snapshot.cooling_limit_flow_rate_comparison_satisfied,
        "cooling_limit_flow_rate_and_capacity_comparison_evaluated":
            snapshot.cooling_limit_flow_rate_and_capacity_comparison_evaluated,
        "cooling_limit_flow_rate_and_capacity_read":
            snapshot.cooling_limit_flow_rate_and_capacity_read,
        "cooling_limit_flow_rate_and_capacity_value": snapshot
            .cooling_limit_flow_rate_and_capacity_value
            .map(limit_name),
        "cooling_limit_flow_rate_and_capacity_comparison_satisfied":
            snapshot.cooling_limit_flow_rate_and_capacity_comparison_satisfied,
        "cooling_flow_limit_active": snapshot.cooling_flow_limit_active,
        "outdoor_air_mass_flow_rate_read": snapshot.outdoor_air_mass_flow_rate_read,
        "outdoor_air_mass_flow_rate_kg_per_s":
            snapshot.outdoor_air_mass_flow_rate_kg_per_s,
        "maximum_cooling_air_mass_flow_rate_read":
            snapshot.maximum_cooling_air_mass_flow_rate_read,
        "maximum_cooling_air_mass_flow_rate_kg_per_s":
            snapshot.maximum_cooling_air_mass_flow_rate_kg_per_s,
        "strict_mass_flow_comparison_evaluated":
            snapshot.strict_mass_flow_comparison_evaluated,
        "outdoor_air_mass_flow_above_maximum":
            snapshot.outdoor_air_mass_flow_above_maximum,
        "maximum_cooling_flow_body_entered":
            snapshot.maximum_cooling_flow_body_entered,
    })
}

fn latest_matches_release(
    gate: &PurchasedAirCalcCoolingOaMaxFlowGateSnapshot,
    predecessor: &PurchasedAirCalcCoolingEntryGateSnapshot,
    expected_system: ep_model::IdealLoadsAirSystemId,
    expected_zone: ep_model::ZoneId,
    maximum_cooling_mass_flow: f64,
    call_count: usize,
) -> bool {
    let expected_unit_off = !predecessor.unit_body_entered;
    let expected_non_cooling = predecessor.unit_body_entered && !predecessor.cooling_body_entered;
    let common = gate.source == PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_SOURCE
        && gate.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_FIRST_EXCLUDED_SOURCE
        && gate.source_order == PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_SOURCE_ORDER
        && predecessor.source == PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_SOURCE
        && predecessor.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_FIRST_EXCLUDED_SOURCE
        && predecessor.source_order == PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_SOURCE_ORDER
        && gate.system == expected_system
        && gate.system == predecessor.system
        && gate.parent_call_ordinal == call_count
        && gate.parent_call_ordinal == predecessor.parent_call_ordinal
        && gate.controlled_zone == expected_zone
        && gate.controlled_zone == predecessor.controlled_zone
        && gate.unit_body_entered == predecessor.unit_body_entered
        && gate.predecessor_cooling_body_entered == predecessor.cooling_body_entered
        && gate.unit_off_skipped == expected_unit_off
        && gate.non_cooling_skipped == expected_non_cooling;
    if !common {
        return false;
    }
    if !predecessor.cooling_body_entered {
        return skipped_shape(gate);
    }
    active_shape_matches(gate, maximum_cooling_mass_flow)
}

fn skipped_shape(gate: &PurchasedAirCalcCoolingOaMaxFlowGateSnapshot) -> bool {
    !gate.cooling_limit_flow_rate_comparison_evaluated
        && !gate.cooling_limit_flow_rate_read
        && gate.cooling_limit_flow_rate_value.is_none()
        && gate.cooling_limit_flow_rate_comparison_satisfied.is_none()
        && !gate.cooling_limit_flow_rate_and_capacity_comparison_evaluated
        && !gate.cooling_limit_flow_rate_and_capacity_read
        && gate.cooling_limit_flow_rate_and_capacity_value.is_none()
        && gate
            .cooling_limit_flow_rate_and_capacity_comparison_satisfied
            .is_none()
        && gate.cooling_flow_limit_active.is_none()
        && !gate.outdoor_air_mass_flow_rate_read
        && gate.outdoor_air_mass_flow_rate_kg_per_s.is_none()
        && !gate.maximum_cooling_air_mass_flow_rate_read
        && gate.maximum_cooling_air_mass_flow_rate_kg_per_s.is_none()
        && !gate.strict_mass_flow_comparison_evaluated
        && gate.outdoor_air_mass_flow_above_maximum.is_none()
        && !gate.maximum_cooling_flow_body_entered
}

fn active_shape_matches(
    gate: &PurchasedAirCalcCoolingOaMaxFlowGateSnapshot,
    maximum_cooling_mass_flow: f64,
) -> bool {
    let Some(limit) = gate.cooling_limit_flow_rate_value else {
        return false;
    };
    let first_match = limit == IdealLoadsLimit::LimitFlowRate;
    let second_evaluated = !first_match;
    let second_match = limit == IdealLoadsLimit::LimitFlowRateAndCapacity;
    let flow_limit_active = first_match || second_match;
    let selectors_match = gate.cooling_limit_flow_rate_comparison_evaluated
        && gate.cooling_limit_flow_rate_read
        && gate.cooling_limit_flow_rate_comparison_satisfied == Some(first_match)
        && gate.cooling_limit_flow_rate_and_capacity_comparison_evaluated == second_evaluated
        && gate.cooling_limit_flow_rate_and_capacity_read == second_evaluated
        && gate.cooling_limit_flow_rate_and_capacity_value == second_evaluated.then_some(limit)
        && gate.cooling_limit_flow_rate_and_capacity_comparison_satisfied
            == second_evaluated.then_some(second_match)
        && gate.cooling_flow_limit_active == Some(flow_limit_active);
    if !selectors_match {
        return false;
    }
    if !flow_limit_active {
        return !gate.outdoor_air_mass_flow_rate_read
            && gate.outdoor_air_mass_flow_rate_kg_per_s.is_none()
            && !gate.maximum_cooling_air_mass_flow_rate_read
            && gate.maximum_cooling_air_mass_flow_rate_kg_per_s.is_none()
            && !gate.strict_mass_flow_comparison_evaluated
            && gate.outdoor_air_mass_flow_above_maximum.is_none()
            && !gate.maximum_cooling_flow_body_entered;
    }
    gate.outdoor_air_mass_flow_rate_read
        && option_has_bits(gate.outdoor_air_mass_flow_rate_kg_per_s, 0.0)
        && gate.maximum_cooling_air_mass_flow_rate_read
        && option_has_bits(
            gate.maximum_cooling_air_mass_flow_rate_kg_per_s,
            maximum_cooling_mass_flow,
        )
        && gate.strict_mass_flow_comparison_evaluated
        && gate.outdoor_air_mass_flow_above_maximum == Some(false)
        && !gate.maximum_cooling_flow_body_entered
}

fn option_has_bits(value: Option<f64>, expected: f64) -> bool {
    value.is_some_and(|value| value.to_bits() == expected.to_bits())
}

fn checked_add(left: usize, right: usize, label: &str) -> Result<usize, String> {
    left.checked_add(right)
        .ok_or_else(|| format!("direct-zone IdealLoads cooling OA maximum-flow {label} overflowed"))
}

fn limit_name(limit: IdealLoadsLimit) -> &'static str {
    match limit {
        IdealLoadsLimit::NoLimit => "NoLimit",
        IdealLoadsLimit::LimitFlowRate => "LimitFlowRate",
        IdealLoadsLimit::LimitCapacity => "LimitCapacity",
        IdealLoadsLimit::LimitFlowRateAndCapacity => "LimitFlowRateAndCapacity",
    }
}
