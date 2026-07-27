//! Run-summary evidence for the bounded PurchasedAir cooling economizer true body.

use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_SOURCE,
    PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_ECONOMIZER_CONDITION_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_ECONOMIZER_CONDITION_SOURCE,
    PURCHASED_AIR_CALC_COOLING_ECONOMIZER_CONDITION_SOURCE_ORDER,
    PurchasedAirCalcCoolingEconomizerBodyLifecycleSummary,
    PurchasedAirCalcCoolingEconomizerBodySnapshot,
    PurchasedAirCalcCoolingEconomizerConditionLifecycleSummary,
    PurchasedAirCalcCoolingEconomizerConditionSnapshot, PurchasedAirInitLifecycleSummary,
};

mod serialization;
mod validation;

pub(super) use serialization::lifecycle_json;
use validation::{skipped_shape, validate_zero_source_counters};

pub(super) fn validate_direct_lifecycle(
    lifecycle: Option<&PurchasedAirCalcCoolingEconomizerBodyLifecycleSummary>,
    predecessor_lifecycle: Option<&PurchasedAirCalcCoolingEconomizerConditionLifecycleSummary>,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle = lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads runtime did not expose cooling economizer body evidence".to_string()
    })?;
    let predecessor_lifecycle = predecessor_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads cooling economizer body has no condition evidence".to_string()
    })?;
    let init_lifecycle = init_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads cooling economizer body has no initialization evidence".to_string()
    })?;
    let coupling_call_count = coupling_call_count.ok_or_else(|| {
        "direct-zone IdealLoads cooling economizer body has no coupling call count".to_string()
    })?;
    let state = &lifecycle.state;
    let predecessor = &predecessor_lifecycle.state;
    let skip_partition = [
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        state.maximum_cooling_flow_body_sibling_skip_count,
        state.no_economizer_outer_guard_fallthrough_skip_count,
        state.economizer_condition_fallthrough_skip_count,
    ]
    .into_iter()
    .try_fold(0usize, |partial, value| {
        checked_add(partial, value, "skip partition")
    })?;
    let transition_partition = checked_add(
        state.body_execution_count,
        skip_partition,
        "transition partition",
    )?;

    if coupling_call_count == 0
        || lifecycle.source != PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_FIRST_EXCLUDED_SOURCE
        || predecessor_lifecycle.source != PURCHASED_AIR_CALC_COOLING_ECONOMIZER_CONDITION_SOURCE
        || predecessor_lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_ECONOMIZER_CONDITION_FIRST_EXCLUDED_SOURCE
    {
        return Err(
            "direct-zone IdealLoads cooling economizer body provenance is invalid".to_string(),
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
            "body_execution_count",
            predecessor.economizer_calculation_body_entry_count,
            state.body_execution_count,
        ),
        ("direct_body_execution_count", 0, state.body_execution_count),
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
            "maximum_cooling_flow_body_sibling_skip_count",
            predecessor.maximum_cooling_flow_body_sibling_skip_count,
            state.maximum_cooling_flow_body_sibling_skip_count,
        ),
        (
            "direct_maximum_cooling_flow_body_sibling_skip_count",
            0,
            state.maximum_cooling_flow_body_sibling_skip_count,
        ),
        (
            "no_economizer_outer_guard_fallthrough_skip_count",
            predecessor.no_economizer_outer_guard_fallthrough_skip_count,
            state.no_economizer_outer_guard_fallthrough_skip_count,
        ),
        (
            "economizer_condition_fallthrough_skip_count",
            predecessor.economizer_condition_fallthrough_count,
            state.economizer_condition_fallthrough_skip_count,
        ),
        (
            "direct_economizer_condition_fallthrough_skip_count",
            0,
            state.economizer_condition_fallthrough_skip_count,
        ),
        ("skip_partition", state.transition_count, skip_partition),
        (
            "transition_partition",
            state.transition_count,
            transition_partition,
        ),
    ] {
        if actual != expected {
            return Err(format!(
                "direct-zone IdealLoads cooling economizer body invariant {field} expected {expected}, got {actual}"
            ));
        }
    }
    validate_zero_source_counters(state)?;

    let latest = state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads cooling economizer body has no latest snapshot".to_string()
    })?;
    let latest_predecessor = predecessor.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads cooling economizer body has no latest condition snapshot"
            .to_string()
    })?;
    let expected_system = init_lifecycle
        .declared_system_order
        .first()
        .copied()
        .ok_or_else(|| {
            "direct-zone IdealLoads cooling economizer body has no declared system".to_string()
        })?;
    let expected_zone = init_lifecycle.controlled_zone.ok_or_else(|| {
        "direct-zone IdealLoads cooling economizer body has no controlled Zone".to_string()
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
            "direct-zone IdealLoads cooling economizer body latest state is not release-ready"
                .to_string(),
        );
    }
    Ok(())
}

fn latest_matches_release(
    body: &PurchasedAirCalcCoolingEconomizerBodySnapshot,
    predecessor: &PurchasedAirCalcCoolingEconomizerConditionSnapshot,
    expected_system: ep_model::IdealLoadsAirSystemId,
    expected_zone: ep_model::ZoneId,
    call_count: usize,
) -> bool {
    let common = body.source == PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_SOURCE
        && body.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_FIRST_EXCLUDED_SOURCE
        && body.source_order == PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_SOURCE_ORDER
        && predecessor.source == PURCHASED_AIR_CALC_COOLING_ECONOMIZER_CONDITION_SOURCE
        && predecessor.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_ECONOMIZER_CONDITION_FIRST_EXCLUDED_SOURCE
        && predecessor.source_order == PURCHASED_AIR_CALC_COOLING_ECONOMIZER_CONDITION_SOURCE_ORDER
        && body.system == expected_system
        && body.system == predecessor.system
        && body.parent_call_ordinal == call_count
        && body.parent_call_ordinal == predecessor.parent_call_ordinal
        && body.controlled_zone == expected_zone
        && body.controlled_zone == predecessor.controlled_zone
        && body.unit_body_entered == predecessor.unit_body_entered
        && body.predecessor_cooling_body_entered == predecessor.predecessor_cooling_body_entered
        && body.predecessor_maximum_cooling_flow_body_entered
            == predecessor.predecessor_maximum_cooling_flow_body_entered
        && body.predecessor_active_guard_false_economizer_fallthrough
            == predecessor.predecessor_active_guard_false_economizer_fallthrough
        && body.predecessor_economizer_guard_evaluated
            == predecessor.predecessor_economizer_guard_evaluated
        && body.predecessor_economizer_body_entered
            == predecessor.predecessor_economizer_body_entered
        && body.predecessor_no_economizer_fallthrough
            == predecessor.predecessor_no_economizer_fallthrough
        && body.predecessor_economizer_condition_evaluated
            == predecessor.economizer_condition_evaluated
        && body.predecessor_economizer_condition_satisfied
            == predecessor.economizer_condition_satisfied
        && body.predecessor_economizer_calculation_body_entered
            == predecessor.economizer_calculation_body_entered
        && body.unit_off_skipped == predecessor.unit_off_skipped
        && body.non_cooling_skipped == predecessor.non_cooling_skipped
        && body.maximum_cooling_flow_body_sibling_skipped
            == predecessor.maximum_cooling_flow_body_sibling_skipped
        && body.no_economizer_outer_guard_fallthrough_skipped
            == predecessor.no_economizer_outer_guard_fallthrough_skipped
        && body.economizer_condition_fallthrough_skipped
            == predecessor.economizer_condition_fallthrough
        && body.economizer_calculation_body_executed
            == predecessor.economizer_calculation_body_entered
        && !body.economizer_calculation_body_executed
        && !body.maximum_cooling_flow_body_sibling_skipped
        && !body.economizer_condition_fallthrough_skipped
        && usize::from(body.unit_off_skipped)
            + usize::from(body.non_cooling_skipped)
            + usize::from(body.no_economizer_outer_guard_fallthrough_skipped)
            == 1;
    common && skipped_shape(body)
}

fn checked_add(left: usize, right: usize, label: &str) -> Result<usize, String> {
    left.checked_add(right)
        .ok_or_else(|| format!("direct-zone IdealLoads cooling economizer body {label} overflowed"))
}
