//! Run-summary evidence for the bounded Cooling mixed-air call and no-OA child route.

use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_CHILD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_NO_OA_CHILD_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE_ORDER,
    PurchasedAirCalcCoolingMixedAirCallLifecycleSummary,
    PurchasedAirCalcCoolingMixedAirCallSnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyLifecycleSummary,
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot,
    PurchasedAirInitLifecycleSummary,
};

mod serialization;
mod validation;

pub(super) use serialization::lifecycle_json;
use validation::snapshot_shape;

pub(super) fn validate_direct_lifecycle(
    lifecycle: Option<&PurchasedAirCalcCoolingMixedAirCallLifecycleSummary>,
    predecessor_cp328: Option<
        &PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyLifecycleSummary,
    >,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle = lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads runtime did not expose cooling mixed-air call evidence".to_string()
    })?;
    let predecessor = predecessor_cp328.ok_or_else(|| {
        "direct-zone IdealLoads cooling mixed-air call has no CP328 evidence".to_string()
    })?;
    let init = init_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads cooling mixed-air call has no initialization evidence".to_string()
    })?;
    let calls = coupling_call_count.ok_or_else(|| {
        "direct-zone IdealLoads cooling mixed-air call has no coupling call count".to_string()
    })?;

    if calls == 0
        || lifecycle.source != PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE
        || lifecycle.child_source != PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_CHILD_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_FIRST_EXCLUDED_SOURCE
    {
        return Err(
            "direct-zone IdealLoads cooling mixed-air call provenance is invalid".to_string(),
        );
    }

    let state = &lifecycle.state;
    let predecessor_state = &predecessor.state;
    let partition = checked_sum(
        &[
            state.unit_off_skip_count,
            state.non_cooling_skip_count,
            state.cooling_call_count,
        ],
        "transition partition",
    )?;
    for (field, expected, actual) in [
        ("transition_count", calls, state.transition_count),
        (
            "predecessor_transition_count",
            predecessor_state.transition_count,
            state.transition_count,
        ),
        ("transition_partition", state.transition_count, partition),
        (
            "unit_off_skip_count",
            predecessor_state.unit_off_skip_count,
            state.unit_off_skip_count,
        ),
        (
            "non_cooling_skip_count",
            predecessor_state.non_cooling_skip_count,
            state.non_cooling_skip_count,
        ),
        (
            "cooling_call_count",
            predecessor_state.cooling_body_entry_count,
            state.cooling_call_count,
        ),
        (
            "caller_source_site_execution_count",
            checked_product(
                state.cooling_call_count,
                PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE_ORDER.len(),
                "caller source-site count",
            )?,
            state.caller_source_site_execution_count,
        ),
        (
            "child_source_site_execution_count",
            checked_product(
                state.cooling_call_count,
                PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_NO_OA_CHILD_SOURCE_ORDER.len(),
                "child source-site count",
            )?,
            state.child_source_site_execution_count,
        ),
        (
            "state_reference_bind_count",
            state.cooling_call_count,
            state.state_reference_bind_count,
        ),
        (
            "purchased_air_number_read_count",
            state.cooling_call_count,
            state.purchased_air_number_read_count,
        ),
        (
            "outdoor_air_mass_flow_rate_read_count",
            state.cooling_call_count,
            state.outdoor_air_mass_flow_rate_read_count,
        ),
        (
            "supply_mass_flow_rate_read_count",
            state.cooling_call_count,
            state.supply_mass_flow_rate_read_count,
        ),
        (
            "mixed_air_output_reference_bind_count",
            checked_product(state.cooling_call_count, 3, "output-reference count")?,
            state.mixed_air_output_reference_bind_count,
        ),
        (
            "operating_mode_read_count",
            state.cooling_call_count,
            state.operating_mode_read_count,
        ),
        (
            "mixed_air_child_call_count",
            state.cooling_call_count,
            state.mixed_air_child_call_count,
        ),
        (
            "no_outdoor_air_fallback_count",
            state.cooling_call_count,
            state.no_outdoor_air_fallback_count,
        ),
        (
            "recirculation_enthalpy_projection_count",
            state.cooling_call_count,
            state.recirculation_enthalpy_projection_count,
        ),
        (
            "mixed_air_output_assignment_count",
            checked_product(state.cooling_call_count, 3, "mixed-output assignment count")?,
            state.mixed_air_output_assignment_count,
        ),
        (
            "heat_recovery_output_positive_zero_assignment_count",
            checked_product(
                state.cooling_call_count,
                2,
                "recovery-zero assignment count",
            )?,
            state.heat_recovery_output_positive_zero_assignment_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }

    let latest = state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads cooling mixed-air call has no latest snapshot".to_string()
    })?;
    let predecessor_latest = predecessor_state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads cooling mixed-air call has no latest CP328 snapshot".to_string()
    })?;
    let expected_system = init.declared_system_order.first().copied().ok_or_else(|| {
        "direct-zone IdealLoads cooling mixed-air call has no declared system".to_string()
    })?;
    let expected_zone = init.controlled_zone.ok_or_else(|| {
        "direct-zone IdealLoads cooling mixed-air call has no controlled Zone".to_string()
    })?;
    if state.system != expected_system
        || predecessor_state.system != expected_system
        || !latest_matches_release(
            latest,
            predecessor_latest,
            expected_system,
            expected_zone,
            init.recirculation_node,
            calls,
        )
    {
        return Err(
            "direct-zone IdealLoads cooling mixed-air call latest state is not release-ready"
                .to_string(),
        );
    }
    Ok(())
}

fn latest_matches_release(
    call: &PurchasedAirCalcCoolingMixedAirCallSnapshot,
    predecessor: &PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot,
    expected_system: ep_model::IdealLoadsAirSystemId,
    expected_zone: ep_model::ZoneId,
    expected_recirculation_node: Option<ep_model::NodeId>,
    call_count: usize,
) -> bool {
    call.source == PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE
        && call.child_source == PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_CHILD_SOURCE
        && call.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_FIRST_EXCLUDED_SOURCE
        && call.source_order == PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE_ORDER
        && call.no_oa_child_source_order
            == PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_NO_OA_CHILD_SOURCE_ORDER
        && call.system == expected_system
        && predecessor.system == expected_system
        && call.parent_call_ordinal == call_count
        && predecessor.parent_call_ordinal == call_count
        && call.controlled_zone == expected_zone
        && predecessor.controlled_zone == expected_zone
        && call.unit_body_entered == predecessor.unit_body_entered
        && call.predecessor_cooling_body_entered == predecessor.cooling_body_entered
        && call.predecessor_zero_flow_reset_body_entered == predecessor.zero_flow_reset_body_entered
        && call.predecessor_active_guard_false_fallthrough
            == predecessor.active_guard_false_fallthrough
        && call.unit_off_skipped == predecessor.unit_off_skipped
        && call.non_cooling_skipped == predecessor.non_cooling_skipped
        && snapshot_shape(call, predecessor, expected_recirculation_node)
}

fn checked_sum(values: &[usize], label: &str) -> Result<usize, String> {
    values.iter().try_fold(0usize, |sum, value| {
        sum.checked_add(*value).ok_or_else(|| {
            format!("direct-zone IdealLoads cooling mixed-air call {label} overflowed")
        })
    })
}

fn checked_product(left: usize, right: usize, label: &str) -> Result<usize, String> {
    left.checked_mul(right)
        .ok_or_else(|| format!("direct-zone IdealLoads cooling mixed-air call {label} overflowed"))
}

fn ensure_count(actual: usize, expected: usize, field: &str) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "direct-zone IdealLoads cooling mixed-air call invariant {field} expected {expected}, got {actual}"
        ))
    }
}
