//! Run-summary evidence for the bounded PurchasedAir cooling economizer guard.

use ep_model::OutdoorAirEconomizerType;
use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_RECURRING_WARNING_CHILD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_SOURCE,
    PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_SOURCE_ORDER,
    PurchasedAirCalcCoolingEconomizerGuardLifecycleSummary,
    PurchasedAirCalcCoolingEconomizerGuardSnapshot,
    PurchasedAirCalcCoolingOaMaxFlowBodyLifecycleSummary,
    PurchasedAirCalcCoolingOaMaxFlowBodySnapshot, PurchasedAirInitLifecycleSummary,
};
use serde_json::{Value, json};

pub(super) fn lifecycle_json(
    lifecycle: &PurchasedAirCalcCoolingEconomizerGuardLifecycleSummary,
) -> Value {
    let state = &lifecycle.state;
    json!({
        "source": lifecycle.source,
        "first_excluded_source": lifecycle.first_excluded_source,
        "system": state.system.0,
        "transition_count": state.transition_count,
        "guard_evaluation_count": state.guard_evaluation_count,
        "unit_off_skip_count": state.unit_off_skip_count,
        "non_cooling_skip_count": state.non_cooling_skip_count,
        "maximum_cooling_flow_body_sibling_skip_count":
            state.maximum_cooling_flow_body_sibling_skip_count,
        "economizer_type_read_count": state.economizer_type_read_count,
        "no_economizer_comparison_count": state.no_economizer_comparison_count,
        "economizer_body_entry_count": state.economizer_body_entry_count,
        "no_economizer_fallthrough_count": state.no_economizer_fallthrough_count,
        "latest": state.latest.map(snapshot_json),
    })
}

fn snapshot_json(snapshot: PurchasedAirCalcCoolingEconomizerGuardSnapshot) -> Value {
    json!({
        "source": snapshot.source,
        "first_excluded_source": snapshot.first_excluded_source,
        "system": snapshot.system.0,
        "parent_call_ordinal": snapshot.parent_call_ordinal,
        "source_order": snapshot.source_order,
        "controlled_zone": snapshot.controlled_zone.0,
        "unit_body_entered": snapshot.unit_body_entered,
        "predecessor_cooling_body_entered": snapshot.predecessor_cooling_body_entered,
        "predecessor_maximum_cooling_flow_body_entered":
            snapshot.predecessor_maximum_cooling_flow_body_entered,
        "predecessor_active_guard_false_economizer_fallthrough":
            snapshot.predecessor_active_guard_false_economizer_fallthrough,
        "unit_off_skipped": snapshot.unit_off_skipped,
        "non_cooling_skipped": snapshot.non_cooling_skipped,
        "maximum_cooling_flow_body_sibling_skipped":
            snapshot.maximum_cooling_flow_body_sibling_skipped,
        "economizer_guard_evaluated": snapshot.economizer_guard_evaluated,
        "economizer_type_read": snapshot.economizer_type_read,
        "economizer_type": snapshot.economizer_type.map(economizer_type_name),
        "no_economizer_comparison_evaluated":
            snapshot.no_economizer_comparison_evaluated,
        "economizer_not_no_economizer": snapshot.economizer_not_no_economizer,
        "economizer_body_entered": snapshot.economizer_body_entered,
        "no_economizer_fallthrough": snapshot.no_economizer_fallthrough,
    })
}

pub(super) fn validate_direct_lifecycle(
    lifecycle: Option<&PurchasedAirCalcCoolingEconomizerGuardLifecycleSummary>,
    predecessor_lifecycle: Option<&PurchasedAirCalcCoolingOaMaxFlowBodyLifecycleSummary>,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle = lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads runtime did not expose cooling economizer guard evidence"
            .to_string()
    })?;
    let predecessor_lifecycle = predecessor_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads cooling economizer guard has no maximum-flow body evidence"
            .to_string()
    })?;
    let init_lifecycle = init_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads cooling economizer guard has no initialization evidence".to_string()
    })?;
    let coupling_call_count = coupling_call_count.ok_or_else(|| {
        "direct-zone IdealLoads cooling economizer guard has no coupling call count".to_string()
    })?;
    let state = &lifecycle.state;
    let predecessor = &predecessor_lifecycle.state;
    let skip_partition = checked_add(
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        "skip partition",
    )
    .and_then(|partial| {
        checked_add(
            partial,
            state.maximum_cooling_flow_body_sibling_skip_count,
            "skip partition",
        )
    })?;
    let transition_partition = checked_add(
        state.guard_evaluation_count,
        skip_partition,
        "transition partition",
    )?;
    let guard_result_partition = checked_add(
        state.economizer_body_entry_count,
        state.no_economizer_fallthrough_count,
        "guard-result partition",
    )?;

    if coupling_call_count == 0
        || lifecycle.source != PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_FIRST_EXCLUDED_SOURCE
        || predecessor_lifecycle.source != PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_SOURCE
        || predecessor_lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_FIRST_EXCLUDED_SOURCE
        || predecessor_lifecycle.recurring_warning_child_source
            != PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_RECURRING_WARNING_CHILD_SOURCE
    {
        return Err(
            "direct-zone IdealLoads cooling economizer guard provenance is invalid".to_string(),
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
            "guard_evaluation_count",
            predecessor.active_guard_false_economizer_fallthrough_count,
            state.guard_evaluation_count,
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
            "maximum_cooling_flow_body_sibling_skip_count",
            predecessor.body_entry_count,
            state.maximum_cooling_flow_body_sibling_skip_count,
        ),
        (
            "direct_sibling_skip_count",
            0,
            state.maximum_cooling_flow_body_sibling_skip_count,
        ),
        (
            "economizer_type_read_count",
            state.guard_evaluation_count,
            state.economizer_type_read_count,
        ),
        (
            "no_economizer_comparison_count",
            state.guard_evaluation_count,
            state.no_economizer_comparison_count,
        ),
        (
            "economizer_body_entry_count",
            0,
            state.economizer_body_entry_count,
        ),
        (
            "no_economizer_fallthrough_count",
            state.guard_evaluation_count,
            state.no_economizer_fallthrough_count,
        ),
        (
            "transition_partition",
            state.transition_count,
            transition_partition,
        ),
        (
            "guard_result_partition",
            state.guard_evaluation_count,
            guard_result_partition,
        ),
    ] {
        if actual != expected {
            return Err(format!(
                "direct-zone IdealLoads cooling economizer guard invariant {field} expected {expected}, got {actual}"
            ));
        }
    }

    let latest = state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads cooling economizer guard has no latest snapshot".to_string()
    })?;
    let latest_predecessor = predecessor.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads cooling economizer guard has no latest maximum-flow body snapshot"
            .to_string()
    })?;
    let expected_system = init_lifecycle
        .declared_system_order
        .first()
        .copied()
        .ok_or_else(|| {
            "direct-zone IdealLoads cooling economizer guard has no declared system".to_string()
        })?;
    let expected_zone = init_lifecycle.controlled_zone.ok_or_else(|| {
        "direct-zone IdealLoads cooling economizer guard has no controlled Zone".to_string()
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
            "direct-zone IdealLoads cooling economizer guard latest state is not release-ready"
                .to_string(),
        );
    }
    Ok(())
}

fn latest_matches_release(
    guard: &PurchasedAirCalcCoolingEconomizerGuardSnapshot,
    predecessor: &PurchasedAirCalcCoolingOaMaxFlowBodySnapshot,
    expected_system: ep_model::IdealLoadsAirSystemId,
    expected_zone: ep_model::ZoneId,
    call_count: usize,
) -> bool {
    let reached = predecessor.active_guard_false_economizer_fallthrough;
    let common = guard.source == PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_SOURCE
        && guard.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_FIRST_EXCLUDED_SOURCE
        && guard.source_order == PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_SOURCE_ORDER
        && predecessor.source == PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_SOURCE
        && predecessor.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_FIRST_EXCLUDED_SOURCE
        && predecessor.recurring_warning_child_source
            == PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_RECURRING_WARNING_CHILD_SOURCE
        && predecessor.source_order == PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_SOURCE_ORDER
        && guard.system == expected_system
        && guard.system == predecessor.system
        && guard.parent_call_ordinal == call_count
        && guard.parent_call_ordinal == predecessor.parent_call_ordinal
        && guard.controlled_zone == expected_zone
        && guard.controlled_zone == predecessor.controlled_zone
        && guard.unit_body_entered == predecessor.unit_body_entered
        && guard.predecessor_cooling_body_entered == predecessor.predecessor_cooling_body_entered
        && guard.predecessor_maximum_cooling_flow_body_entered
            == predecessor.predecessor_maximum_cooling_flow_body_entered
        && guard.predecessor_active_guard_false_economizer_fallthrough == reached
        && guard.unit_off_skipped == predecessor.unit_off_skipped
        && guard.non_cooling_skipped == predecessor.non_cooling_skipped
        && guard.maximum_cooling_flow_body_sibling_skipped
            == predecessor.predecessor_maximum_cooling_flow_body_entered
        && guard.economizer_guard_evaluated == reached;
    if !common {
        return false;
    }
    if reached {
        predecessor.body_skipped
            && predecessor.unit_body_entered
            && predecessor.predecessor_cooling_body_entered
            && !predecessor.predecessor_maximum_cooling_flow_body_entered
            && !predecessor.unit_off_skipped
            && !predecessor.non_cooling_skipped
            && !guard.unit_off_skipped
            && !guard.non_cooling_skipped
            && !guard.maximum_cooling_flow_body_sibling_skipped
            && guard.economizer_type_read
            && guard.economizer_type == Some(OutdoorAirEconomizerType::NoEconomizer)
            && guard.no_economizer_comparison_evaluated
            && guard.economizer_not_no_economizer == Some(false)
            && !guard.economizer_body_entered
            && guard.no_economizer_fallthrough
    } else {
        let unit_off_predecessor = predecessor.unit_off_skipped
            && !predecessor.unit_body_entered
            && !predecessor.predecessor_cooling_body_entered;
        let non_cooling_predecessor = predecessor.non_cooling_skipped
            && predecessor.unit_body_entered
            && !predecessor.predecessor_cooling_body_entered;
        predecessor.body_skipped
            && !predecessor.predecessor_maximum_cooling_flow_body_entered
            && (unit_off_predecessor || non_cooling_predecessor)
            && usize::from(guard.unit_off_skipped)
                + usize::from(guard.non_cooling_skipped)
                + usize::from(guard.maximum_cooling_flow_body_sibling_skipped)
                == 1
            && skipped_shape(guard)
    }
}

fn skipped_shape(guard: &PurchasedAirCalcCoolingEconomizerGuardSnapshot) -> bool {
    !guard.economizer_type_read
        && guard.economizer_type.is_none()
        && !guard.no_economizer_comparison_evaluated
        && guard.economizer_not_no_economizer.is_none()
        && !guard.economizer_body_entered
        && !guard.no_economizer_fallthrough
}

fn economizer_type_name(value: OutdoorAirEconomizerType) -> &'static str {
    match value {
        OutdoorAirEconomizerType::NoEconomizer => "NoEconomizer",
        OutdoorAirEconomizerType::DifferentialDryBulb => "DifferentialDryBulb",
        OutdoorAirEconomizerType::DifferentialEnthalpy => "DifferentialEnthalpy",
    }
}

fn checked_add(left: usize, right: usize, label: &str) -> Result<usize, String> {
    left.checked_add(right).ok_or_else(|| {
        format!("direct-zone IdealLoads cooling economizer guard {label} overflowed")
    })
}
