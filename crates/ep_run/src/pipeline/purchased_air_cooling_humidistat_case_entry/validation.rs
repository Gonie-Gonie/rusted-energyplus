//! Fail-closed validation for CP358 direct-release evidence.

use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_CASE_BREAK_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_CASE_BREAK_SOURCE,
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_CASE_ENTRY_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_CASE_ENTRY_SOURCE,
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_CASE_ENTRY_SOURCE_ORDER,
    PurchasedAirCalcCoolingConstantShrCaseBreakLifecycleSummary,
    PurchasedAirCalcCoolingConstantShrCaseBreakSnapshot,
    PurchasedAirCalcCoolingHumidistatCaseEntryLifecycleSummary,
    PurchasedAirCalcCoolingHumidistatCaseEntryRuntimeState,
    PurchasedAirCalcCoolingHumidistatCaseEntrySnapshot, PurchasedAirInitLifecycleSummary,
};

pub(in crate::pipeline) struct DirectLifecyclePredecessors<'a> {
    pub(in crate::pipeline) case_break_cp357:
        Option<&'a PurchasedAirCalcCoolingConstantShrCaseBreakLifecycleSummary>,
}

pub(in crate::pipeline) fn validate_direct_lifecycle(
    lifecycle: Option<&PurchasedAirCalcCoolingHumidistatCaseEntryLifecycleSummary>,
    predecessors: DirectLifecyclePredecessors<'_>,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle = lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads runtime did not expose Humidistat case-entry evidence".to_string()
    })?;
    let predecessor = predecessors.case_break_cp357.ok_or_else(|| {
        "direct-zone IdealLoads Humidistat case entry has no CP357 evidence".to_string()
    })?;
    let init = init_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads Humidistat case entry has no initialization evidence".to_string()
    })?;
    let calls = coupling_call_count.ok_or_else(|| {
        "direct-zone IdealLoads Humidistat case entry has no coupling call count".to_string()
    })?;
    if calls == 0
        || lifecycle.source != PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_CASE_ENTRY_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_CASE_ENTRY_FIRST_EXCLUDED_SOURCE
        || predecessor.source != PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_CASE_BREAK_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_CASE_BREAK_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_CASE_ENTRY_SOURCE_ORDER.len() != 1
    {
        return Err("direct-zone IdealLoads Humidistat case-entry provenance is invalid".into());
    }

    let state = &lifecycle.state;
    let predecessor_state = &predecessor.state;
    let constant_shr =
        state.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count;
    let humidistat = state.dehumidification_control_humidistat_case_entry_count;
    validate_route_partition(state)?;
    validate_source_counters(state)?;
    for (field, expected, actual) in [
        ("transition_count", calls, state.transition_count),
        (
            "predecessor_transition_count",
            predecessor_state.transition_count,
            state.transition_count,
        ),
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
            "positive_guard_false_fallthrough_skip_count",
            predecessor_state.positive_guard_false_fallthrough_skip_count,
            state.positive_guard_false_fallthrough_skip_count,
        ),
        (
            "none_case_completed_skip_count",
            predecessor_state.dehumidification_control_none_case_completed_skip_count,
            state.dehumidification_control_none_case_completed_skip_count,
        ),
        (
            "constant_shr_case_completed_skip_count",
            predecessor_state
                .dehumidification_control_constant_sensible_heat_ratio_case_break_count,
            constant_shr,
        ),
        (
            "humidistat_case_entry_count",
            predecessor_state.dehumidification_control_humidistat_case_selected_skip_count,
            humidistat,
        ),
        (
            "constant_supply_humidity_ratio_case_selected_skip_count",
            predecessor_state
                .dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
            state.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
        ),
        (
            "direct_constant_shr_case_completed_skip_count",
            0,
            constant_shr,
        ),
        ("direct_humidistat_case_entry_count", 0, humidistat),
        (
            "direct_constant_supply_humidity_ratio_case_selected_skip_count",
            0,
            state.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }

    let latest = state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads Humidistat case entry has no latest snapshot".to_string()
    })?;
    let predecessor_latest = predecessor_state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads Humidistat case entry has no latest CP357 snapshot".to_string()
    })?;
    let expected_system = init.declared_system_order.first().copied().ok_or_else(|| {
        "direct-zone IdealLoads Humidistat case entry has no declared system".to_string()
    })?;
    let expected_zone = init.controlled_zone.ok_or_else(|| {
        "direct-zone IdealLoads Humidistat case entry has no controlled Zone".to_string()
    })?;
    if state.system != expected_system
        || predecessor_state.system != expected_system
        || latest.system != expected_system
        || predecessor_latest.system != expected_system
        || latest.parent_call_ordinal != calls
        || predecessor_latest.parent_call_ordinal != calls
        || latest.controlled_zone != expected_zone
        || predecessor_latest.controlled_zone != expected_zone
        || *latest != expected_snapshot(*predecessor_latest)
    {
        return Err(
            "direct-zone IdealLoads Humidistat case-entry latest state is not release-ready".into(),
        );
    }
    Ok(())
}

fn validate_route_partition(
    state: &PurchasedAirCalcCoolingHumidistatCaseEntryRuntimeState,
) -> Result<(), String> {
    let partition = checked_sum(&[
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        state.positive_guard_false_fallthrough_skip_count,
        state.dehumidification_control_none_case_completed_skip_count,
        state.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
        state.dehumidification_control_humidistat_case_entry_count,
        state.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
    ])?;
    ensure_count(partition, state.transition_count, "transition_partition")
}

fn validate_source_counters(
    state: &PurchasedAirCalcCoolingHumidistatCaseEntryRuntimeState,
) -> Result<(), String> {
    let entries = state.dehumidification_control_humidistat_case_entry_count;
    let source_sites = entries
        .checked_mul(PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_CASE_ENTRY_SOURCE_ORDER.len())
        .ok_or_else(|| "source_site_execution_count overflowed".to_string())?;
    ensure_count(
        state.source_site_execution_count,
        source_sites,
        "source_site_execution_count",
    )
}

fn expected_snapshot(
    predecessor: PurchasedAirCalcCoolingConstantShrCaseBreakSnapshot,
) -> PurchasedAirCalcCoolingHumidistatCaseEntrySnapshot {
    PurchasedAirCalcCoolingHumidistatCaseEntrySnapshot {
        source: PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_CASE_ENTRY_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_CASE_ENTRY_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_CASE_ENTRY_SOURCE_ORDER,
        system: predecessor.system,
        parent_call_ordinal: predecessor.parent_call_ordinal,
        controlled_zone: predecessor.controlled_zone,
        unit_body_entered: predecessor.unit_body_entered,
        predecessor_cooling_body_entered: predecessor.predecessor_cooling_body_entered,
        predecessor_no_outdoor_air_fallback_entered: predecessor
            .predecessor_no_outdoor_air_fallback_entered,
        predecessor_positive_supply_mass_flow_body_entered: predecessor
            .predecessor_positive_supply_mass_flow_body_entered,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        positive_guard_false_fallthrough_skipped: predecessor
            .positive_guard_false_fallthrough_skipped,
        predecessor_dehumidification_control_type: predecessor
            .predecessor_dehumidification_control_type,
        predecessor_dehumidification_control_none_case_completed_skip: predecessor
            .dehumidification_control_none_case_completed_skip,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break:
            predecessor.dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break,
        predecessor_dehumidification_control_humidistat_case_selected_skip: predecessor
            .dehumidification_control_humidistat_case_selected_skip,
        predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip:
            predecessor.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip,
        dehumidification_control_none_case_completed_skip: predecessor
            .dehumidification_control_none_case_completed_skip,
        dehumidification_control_constant_sensible_heat_ratio_case_completed_skip: predecessor
            .dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break,
        dehumidification_control_humidistat_case_entered: predecessor
            .dehumidification_control_humidistat_case_selected_skip,
        dehumidification_control_constant_supply_humidity_ratio_case_selected_skip: predecessor
            .dehumidification_control_constant_supply_humidity_ratio_case_selected_skip,
    }
}

fn checked_sum(values: &[usize]) -> Result<usize, String> {
    values.iter().try_fold(0usize, |sum, value| {
        sum.checked_add(*value)
            .ok_or_else(|| "transition_partition overflowed".to_string())
    })
}

fn ensure_count(actual: usize, expected: usize, field: &str) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "direct-zone IdealLoads Humidistat case entry {field} expected {expected}, got {actual}"
        ))
    }
}

#[cfg(test)]
mod tests;
