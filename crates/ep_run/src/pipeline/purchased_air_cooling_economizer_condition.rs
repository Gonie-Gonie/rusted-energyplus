//! Run-summary evidence for the bounded PurchasedAir cooling economizer condition.

use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_ECONOMIZER_CONDITION_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_ECONOMIZER_CONDITION_SOURCE,
    PURCHASED_AIR_CALC_COOLING_ECONOMIZER_CONDITION_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingEconomizerConditionLifecycleSummary,
    PurchasedAirCalcCoolingEconomizerConditionSnapshot,
    PurchasedAirCalcCoolingEconomizerGuardLifecycleSummary,
    PurchasedAirCalcCoolingEconomizerGuardSnapshot, PurchasedAirInitLifecycleSummary,
};

mod serialization;

pub(super) use serialization::lifecycle_json;

pub(super) fn validate_direct_lifecycle(
    lifecycle: Option<&PurchasedAirCalcCoolingEconomizerConditionLifecycleSummary>,
    predecessor_lifecycle: Option<&PurchasedAirCalcCoolingEconomizerGuardLifecycleSummary>,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle = lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads runtime did not expose cooling economizer condition evidence"
            .to_string()
    })?;
    let predecessor_lifecycle = predecessor_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads cooling economizer condition has no outer-guard evidence"
            .to_string()
    })?;
    let init_lifecycle = init_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads cooling economizer condition has no initialization evidence"
            .to_string()
    })?;
    let coupling_call_count = coupling_call_count.ok_or_else(|| {
        "direct-zone IdealLoads cooling economizer condition has no coupling call count".to_string()
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
    })
    .and_then(|partial| {
        checked_add(
            partial,
            state.no_economizer_outer_guard_fallthrough_skip_count,
            "skip partition",
        )
    })?;
    let transition_partition = checked_add(
        state.condition_evaluation_count,
        skip_partition,
        "transition partition",
    )?;
    let result_partition = checked_add(
        state.economizer_calculation_body_entry_count,
        state.economizer_condition_fallthrough_count,
        "condition-result partition",
    )?;

    if coupling_call_count == 0
        || lifecycle.source != PURCHASED_AIR_CALC_COOLING_ECONOMIZER_CONDITION_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_ECONOMIZER_CONDITION_FIRST_EXCLUDED_SOURCE
        || predecessor_lifecycle.source != PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_SOURCE
        || predecessor_lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_FIRST_EXCLUDED_SOURCE
    {
        return Err(
            "direct-zone IdealLoads cooling economizer condition provenance is invalid".to_string(),
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
            "condition_evaluation_count",
            predecessor.economizer_body_entry_count,
            state.condition_evaluation_count,
        ),
        (
            "direct_condition_evaluation_count",
            0,
            state.condition_evaluation_count,
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
            predecessor.maximum_cooling_flow_body_sibling_skip_count,
            state.maximum_cooling_flow_body_sibling_skip_count,
        ),
        (
            "direct_sibling_skip_count",
            0,
            state.maximum_cooling_flow_body_sibling_skip_count,
        ),
        (
            "no_economizer_outer_guard_fallthrough_skip_count",
            predecessor.no_economizer_fallthrough_count,
            state.no_economizer_outer_guard_fallthrough_skip_count,
        ),
        (
            "differential_dry_bulb_economizer_type_read_count",
            0,
            state.differential_dry_bulb_economizer_type_read_count,
        ),
        (
            "differential_dry_bulb_selector_comparison_count",
            0,
            state.differential_dry_bulb_selector_comparison_count,
        ),
        (
            "differential_dry_bulb_selector_match_count",
            0,
            state.differential_dry_bulb_selector_match_count,
        ),
        (
            "outdoor_air_temperature_read_count",
            0,
            state.outdoor_air_temperature_read_count,
        ),
        (
            "recirculation_air_temperature_read_count",
            0,
            state.recirculation_air_temperature_read_count,
        ),
        (
            "dry_bulb_temperature_comparison_count",
            0,
            state.dry_bulb_temperature_comparison_count,
        ),
        (
            "dry_bulb_temperature_comparison_satisfied_count",
            0,
            state.dry_bulb_temperature_comparison_satisfied_count,
        ),
        (
            "differential_enthalpy_economizer_type_read_count",
            0,
            state.differential_enthalpy_economizer_type_read_count,
        ),
        (
            "differential_enthalpy_selector_comparison_count",
            0,
            state.differential_enthalpy_selector_comparison_count,
        ),
        (
            "differential_enthalpy_selector_match_count",
            0,
            state.differential_enthalpy_selector_match_count,
        ),
        (
            "outdoor_air_enthalpy_read_count",
            0,
            state.outdoor_air_enthalpy_read_count,
        ),
        (
            "recirculation_air_enthalpy_read_count",
            0,
            state.recirculation_air_enthalpy_read_count,
        ),
        (
            "enthalpy_comparison_count",
            0,
            state.enthalpy_comparison_count,
        ),
        (
            "enthalpy_comparison_satisfied_count",
            0,
            state.enthalpy_comparison_satisfied_count,
        ),
        (
            "economizer_calculation_body_entry_count",
            0,
            state.economizer_calculation_body_entry_count,
        ),
        (
            "economizer_condition_fallthrough_count",
            0,
            state.economizer_condition_fallthrough_count,
        ),
        ("skip_partition", state.transition_count, skip_partition),
        (
            "transition_partition",
            state.transition_count,
            transition_partition,
        ),
        (
            "condition_result_partition",
            state.condition_evaluation_count,
            result_partition,
        ),
    ] {
        if actual != expected {
            return Err(format!(
                "direct-zone IdealLoads cooling economizer condition invariant {field} expected {expected}, got {actual}"
            ));
        }
    }

    let latest = state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads cooling economizer condition has no latest snapshot".to_string()
    })?;
    let latest_predecessor = predecessor.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads cooling economizer condition has no latest outer-guard snapshot"
            .to_string()
    })?;
    let expected_system = init_lifecycle
        .declared_system_order
        .first()
        .copied()
        .ok_or_else(|| {
            "direct-zone IdealLoads cooling economizer condition has no declared system".to_string()
        })?;
    let expected_zone = init_lifecycle.controlled_zone.ok_or_else(|| {
        "direct-zone IdealLoads cooling economizer condition has no controlled Zone".to_string()
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
            "direct-zone IdealLoads cooling economizer condition latest state is not release-ready"
                .to_string(),
        );
    }
    Ok(())
}

fn latest_matches_release(
    condition: &PurchasedAirCalcCoolingEconomizerConditionSnapshot,
    predecessor: &PurchasedAirCalcCoolingEconomizerGuardSnapshot,
    expected_system: ep_model::IdealLoadsAirSystemId,
    expected_zone: ep_model::ZoneId,
    call_count: usize,
) -> bool {
    let common = condition.source == PURCHASED_AIR_CALC_COOLING_ECONOMIZER_CONDITION_SOURCE
        && condition.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_ECONOMIZER_CONDITION_FIRST_EXCLUDED_SOURCE
        && condition.source_order == PURCHASED_AIR_CALC_COOLING_ECONOMIZER_CONDITION_SOURCE_ORDER
        && predecessor.source == PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_SOURCE
        && predecessor.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_FIRST_EXCLUDED_SOURCE
        && predecessor.source_order == PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_SOURCE_ORDER
        && condition.system == expected_system
        && condition.system == predecessor.system
        && condition.parent_call_ordinal == call_count
        && condition.parent_call_ordinal == predecessor.parent_call_ordinal
        && condition.controlled_zone == expected_zone
        && condition.controlled_zone == predecessor.controlled_zone
        && condition.unit_body_entered == predecessor.unit_body_entered
        && condition.predecessor_cooling_body_entered
            == predecessor.predecessor_cooling_body_entered
        && condition.predecessor_maximum_cooling_flow_body_entered
            == predecessor.predecessor_maximum_cooling_flow_body_entered
        && condition.predecessor_active_guard_false_economizer_fallthrough
            == predecessor.predecessor_active_guard_false_economizer_fallthrough
        && condition.predecessor_economizer_guard_evaluated
            == predecessor.economizer_guard_evaluated
        && condition.predecessor_economizer_body_entered == predecessor.economizer_body_entered
        && condition.predecessor_no_economizer_fallthrough == predecessor.no_economizer_fallthrough
        && condition.unit_off_skipped == predecessor.unit_off_skipped
        && condition.non_cooling_skipped == predecessor.non_cooling_skipped
        && condition.maximum_cooling_flow_body_sibling_skipped
            == predecessor.maximum_cooling_flow_body_sibling_skipped
        && condition.no_economizer_outer_guard_fallthrough_skipped
            == predecessor.no_economizer_fallthrough
        && !condition.economizer_condition_evaluated
        && usize::from(condition.unit_off_skipped)
            + usize::from(condition.non_cooling_skipped)
            + usize::from(condition.maximum_cooling_flow_body_sibling_skipped)
            + usize::from(condition.no_economizer_outer_guard_fallthrough_skipped)
            == 1;
    common && skipped_shape(condition)
}

fn skipped_shape(condition: &PurchasedAirCalcCoolingEconomizerConditionSnapshot) -> bool {
    !condition.differential_dry_bulb_economizer_type_read
        && condition.differential_dry_bulb_economizer_type.is_none()
        && !condition.differential_dry_bulb_selector_comparison_evaluated
        && condition.differential_dry_bulb_selector_matched.is_none()
        && !condition.outdoor_air_temperature_read
        && condition.outdoor_air_temperature_c.is_none()
        && !condition.recirculation_air_temperature_read
        && condition.recirculation_air_temperature_c.is_none()
        && !condition.dry_bulb_temperature_comparison_evaluated
        && condition
            .outdoor_air_temperature_below_recirculation_temperature
            .is_none()
        && !condition.differential_enthalpy_economizer_type_read
        && condition.differential_enthalpy_economizer_type.is_none()
        && !condition.differential_enthalpy_selector_comparison_evaluated
        && condition.differential_enthalpy_selector_matched.is_none()
        && !condition.outdoor_air_enthalpy_read
        && condition.outdoor_air_enthalpy_j_per_kg.is_none()
        && !condition.recirculation_air_enthalpy_read
        && condition.recirculation_air_enthalpy_j_per_kg.is_none()
        && !condition.enthalpy_comparison_evaluated
        && condition
            .outdoor_air_enthalpy_below_recirculation_enthalpy
            .is_none()
        && condition.economizer_condition_satisfied.is_none()
        && !condition.economizer_calculation_body_entered
        && !condition.economizer_condition_fallthrough
}

fn checked_add(left: usize, right: usize, label: &str) -> Result<usize, String> {
    left.checked_add(right).ok_or_else(|| {
        format!("direct-zone IdealLoads cooling economizer condition {label} overflowed")
    })
}
