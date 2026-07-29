//! Fail-closed validation for CP363 direct-release evidence.

use ep_model::{IdealLoadsAirSystemId, ZoneId};
use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_CASE_BREAK_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_CASE_BREAK_SOURCE,
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_CASE_BREAK_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_SOURCE_ORDER,
    PurchasedAirCalcCoolingHumidistatCaseBreakLifecycleSummary,
    PurchasedAirCalcCoolingHumidistatCaseBreakRuntimeState,
    PurchasedAirCalcCoolingHumidistatCaseBreakSnapshot,
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitLifecycleSummary,
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitRuntimeState,
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitSnapshot,
    PurchasedAirInitLifecycleSummary,
};

pub(in crate::pipeline) struct DirectLifecyclePredecessors<'a> {
    pub(in crate::pipeline) mixed_air_limit_cp362: Option<
        &'a PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitLifecycleSummary,
    >,
}

pub(in crate::pipeline) fn validate_direct_lifecycle(
    lifecycle: Option<&PurchasedAirCalcCoolingHumidistatCaseBreakLifecycleSummary>,
    predecessors: DirectLifecyclePredecessors<'_>,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle = lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads runtime did not expose Humidistat case-break evidence".to_string()
    })?;
    let predecessor = predecessors.mixed_air_limit_cp362.ok_or_else(|| {
        "direct-zone IdealLoads Humidistat case break has no CP362 evidence".to_string()
    })?;
    let init = init_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads Humidistat case break has no initialization evidence".to_string()
    })?;
    let calls = coupling_call_count.ok_or_else(|| {
        "direct-zone IdealLoads Humidistat case break has no coupling call count".to_string()
    })?;
    let expected_system = init.declared_system_order.first().copied().ok_or_else(|| {
        "direct-zone IdealLoads Humidistat case break has no declared system".to_string()
    })?;
    let expected_zone = init.controlled_zone.ok_or_else(|| {
        "direct-zone IdealLoads Humidistat case break has no controlled Zone".to_string()
    })?;
    validate_release_state(
        lifecycle,
        predecessor,
        expected_system,
        expected_zone,
        calls,
    )
}

fn validate_release_state(
    lifecycle: &PurchasedAirCalcCoolingHumidistatCaseBreakLifecycleSummary,
    predecessor: &PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitLifecycleSummary,
    expected_system: IdealLoadsAirSystemId,
    expected_zone: ZoneId,
    calls: usize,
) -> Result<(), String> {
    if calls == 0
        || lifecycle.source != PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_CASE_BREAK_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_CASE_BREAK_FIRST_EXCLUDED_SOURCE
        || predecessor.source
            != PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_CASE_BREAK_SOURCE_ORDER.len() != 1
        || PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_SOURCE_ORDER
            .len()
            != 4
    {
        return Err("direct-zone IdealLoads Humidistat case-break provenance is invalid".into());
    }

    let state = &lifecycle.state;
    let predecessor_state = &predecessor.state;
    let constant_shr =
        state.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count;
    let humidistat = state.dehumidification_control_humidistat_case_break_count;
    validate_route_partition(state)?;
    validate_source_counters(state)?;
    validate_predecessor_counters(predecessor_state)?;
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
                .dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
            constant_shr,
        ),
        (
            "humidistat_case_break_count",
            predecessor_state
                .dehumidification_control_humidistat_supply_humidity_ratio_mixed_air_limit_count,
            humidistat,
        ),
        (
            "constant_supply_humidity_ratio_case_selected_skip_count",
            predecessor_state
                .dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
            state.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
        ),
        ("direct_constant_shr_skip_count", 0, constant_shr),
        ("direct_humidistat_case_break_count", 0, humidistat),
        (
            "direct_constant_supply_humidity_ratio_skip_count",
            0,
            state.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }

    let latest = state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads Humidistat case break has no latest snapshot".to_string()
    })?;
    let predecessor_latest = predecessor_state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads Humidistat case break has no latest CP362 snapshot".to_string()
    })?;
    if state.system != expected_system
        || predecessor_state.system != expected_system
        || latest.system != expected_system
        || predecessor_latest.system != expected_system
        || latest.parent_call_ordinal != calls
        || predecessor_latest.parent_call_ordinal != calls
        || latest.controlled_zone != expected_zone
        || predecessor_latest.controlled_zone != expected_zone
        || !predecessor_latest_is_exact_direct_shape(predecessor_latest)
        || *latest != expected_snapshot(*predecessor_latest)
    {
        return Err(
            "direct-zone IdealLoads Humidistat case-break latest state is not release-ready".into(),
        );
    }
    Ok(())
}

fn validate_route_partition(
    state: &PurchasedAirCalcCoolingHumidistatCaseBreakRuntimeState,
) -> Result<(), String> {
    let partition = checked_sum(&[
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        state.positive_guard_false_fallthrough_skip_count,
        state.dehumidification_control_none_case_completed_skip_count,
        state.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
        state.dehumidification_control_humidistat_case_break_count,
        state.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
    ])?;
    ensure_count(partition, state.transition_count, "transition_partition")
}

fn validate_source_counters(
    state: &PurchasedAirCalcCoolingHumidistatCaseBreakRuntimeState,
) -> Result<(), String> {
    let executed = state.dehumidification_control_humidistat_case_break_count;
    let source_sites = executed
        .checked_mul(PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_CASE_BREAK_SOURCE_ORDER.len())
        .ok_or_else(|| {
            "direct-zone IdealLoads Humidistat case-break source counter overflow".to_string()
        })?;
    ensure_count(
        state.source_site_execution_count,
        source_sites,
        "source_site_execution_count",
    )
}

fn validate_predecessor_counters(
    state: &PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitRuntimeState,
) -> Result<(), String> {
    let executed =
        state.dehumidification_control_humidistat_supply_humidity_ratio_mixed_air_limit_count;
    let partition = checked_sum(&[
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        state.positive_guard_false_fallthrough_skip_count,
        state.dehumidification_control_none_case_completed_skip_count,
        state.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
        executed,
        state.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
    ])?;
    let source_sites = executed
        .checked_mul(
            PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_SOURCE_ORDER
                .len(),
        )
        .ok_or_else(|| {
            "direct-zone IdealLoads CP362 predecessor source counter overflow".to_string()
        })?;
    for (field, expected, actual) in [
        (
            "predecessor_transition_partition",
            state.transition_count,
            partition,
        ),
        (
            "predecessor_source_site_execution_count",
            source_sites,
            state.source_site_execution_count,
        ),
        (
            "predecessor_mixed_air_read_count",
            executed,
            state.mixed_air_humidity_ratio_for_minimum_read_count,
        ),
        (
            "predecessor_local_read_count",
            executed,
            state.supply_humidity_ratio_for_dehumidification_for_mixed_air_limit_minimum_read_count,
        ),
        (
            "predecessor_minimum_count",
            executed,
            state.source_shaped_two_argument_minimum_evaluation_count,
        ),
        (
            "predecessor_assignment_count",
            executed,
            state.supply_humidity_ratio_assignment_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }
    Ok(())
}

fn predecessor_latest_is_exact_direct_shape(
    snapshot: &PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitSnapshot,
) -> bool {
    let route_count = [
        snapshot.unit_off_skipped,
        snapshot.non_cooling_skipped,
        snapshot.positive_guard_false_fallthrough_skipped,
        snapshot.dehumidification_control_none_case_completed_skip,
    ]
    .into_iter()
    .filter(|selected| *selected)
    .count();
    snapshot.source
        == PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_SOURCE
        && snapshot.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE
        && snapshot.source_order
            == PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_SOURCE_ORDER
        && route_count == 1
        && !snapshot.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip
        && !snapshot
            .dehumidification_control_humidistat_supply_humidity_ratio_mixed_air_limit_executed
        && !snapshot
            .dehumidification_control_constant_supply_humidity_ratio_case_selected_skip
        && !snapshot.mixed_air_humidity_ratio_for_minimum_read
        && !snapshot
            .supply_humidity_ratio_for_dehumidification_for_mixed_air_limit_minimum_read
        && !snapshot.source_shaped_two_argument_minimum_evaluated
        && !snapshot.supply_humidity_ratio_assignment_performed
        && [
            snapshot.predecessor_resulting_supply_humidity_ratio_for_dehumidification,
            snapshot.mixed_air_humidity_ratio,
            snapshot.supply_humidity_ratio_for_dehumidification_before_mixed_air_limit,
            snapshot.minimum_supply_humidity_ratio,
            snapshot.assigned_supply_humidity_ratio,
            snapshot.resulting_supply_humidity_ratio,
        ]
        .into_iter()
        .all(|value| value.is_none())
}

fn expected_snapshot(
    predecessor: PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitSnapshot,
) -> PurchasedAirCalcCoolingHumidistatCaseBreakSnapshot {
    PurchasedAirCalcCoolingHumidistatCaseBreakSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_CASE_BREAK_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_CASE_BREAK_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_CASE_BREAK_SOURCE_ORDER,
        system: predecessor.system,
        parent_call_ordinal: predecessor.parent_call_ordinal,
        controlled_zone: predecessor.controlled_zone,
        unit_body_entered: predecessor.unit_body_entered,
        predecessor_cooling_body_entered: predecessor.predecessor_cooling_body_entered,
        predecessor_no_outdoor_air_fallback_entered:
            predecessor.predecessor_no_outdoor_air_fallback_entered,
        predecessor_positive_supply_mass_flow_body_entered:
            predecessor.predecessor_positive_supply_mass_flow_body_entered,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        positive_guard_false_fallthrough_skipped:
            predecessor.positive_guard_false_fallthrough_skipped,
        predecessor_dehumidification_control_type:
            predecessor.predecessor_dehumidification_control_type,
        predecessor_dehumidification_control_none_case_completed_skip:
            predecessor.dehumidification_control_none_case_completed_skip,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip:
            predecessor.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
        predecessor_dehumidification_control_humidistat_supply_humidity_ratio_mixed_air_limit_executed:
            predecessor
                .dehumidification_control_humidistat_supply_humidity_ratio_mixed_air_limit_executed,
        predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip:
            predecessor
                .dehumidification_control_constant_supply_humidity_ratio_case_selected_skip,
        dehumidification_control_none_case_completed_skip:
            predecessor.dehumidification_control_none_case_completed_skip,
        dehumidification_control_constant_sensible_heat_ratio_case_completed_skip:
            predecessor.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
        dehumidification_control_humidistat_case_exited_via_break: predecessor
            .dehumidification_control_humidistat_supply_humidity_ratio_mixed_air_limit_executed,
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
            "direct-zone IdealLoads Humidistat case break {field} expected {expected}, got {actual}"
        ))
    }
}

#[cfg(test)]
mod tests;
