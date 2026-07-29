//! Fail-closed validation for CP356 direct-release evidence.

use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MINIMUM_LIMIT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MINIMUM_LIMIT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_SOURCE_ORDER,
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMinimumLimitLifecycleSummary,
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMinimumLimitSnapshot,
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitLifecycleSummary,
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitRuntimeState,
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitSnapshot,
    PurchasedAirInitLifecycleSummary,
};

pub(in crate::pipeline) struct DirectLifecyclePredecessors<'a> {
    pub(in crate::pipeline) minimum_limit_cp355: Option<
        &'a PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMinimumLimitLifecycleSummary,
    >,
}

pub(in crate::pipeline) fn validate_direct_lifecycle(
    lifecycle: Option<
        &PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitLifecycleSummary,
    >,
    predecessors: DirectLifecyclePredecessors<'_>,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle = lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads runtime did not expose constant-SHR supply-humidity-ratio mixed-air-limit evidence"
            .to_string()
    })?;
    let predecessor = predecessors.minimum_limit_cp355.ok_or_else(|| {
        "direct-zone IdealLoads supply-humidity-ratio mixed-air limit has no CP355 evidence"
            .to_string()
    })?;
    let init = init_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads supply-humidity-ratio mixed-air limit has no initialization evidence"
            .to_string()
    })?;
    let calls = coupling_call_count.ok_or_else(|| {
        "direct-zone IdealLoads supply-humidity-ratio mixed-air limit has no coupling call count"
            .to_string()
    })?;
    if calls == 0
        || lifecycle.source
            != PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE
        || predecessor.source
            != PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MINIMUM_LIMIT_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MINIMUM_LIMIT_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_SOURCE_ORDER
            .len()
            != 4
    {
        return Err(
            "direct-zone IdealLoads supply-humidity-ratio mixed-air-limit provenance is invalid"
                .to_string(),
        );
    }

    let state = &lifecycle.state;
    let predecessor_state = &predecessor.state;
    let executed = state
        .dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_mixed_air_limit_count;
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
            "constant_shr_supply_humidity_ratio_mixed_air_limit_count",
            predecessor_state
                .dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_minimum_limit_count,
            executed,
        ),
        (
            "humidistat_case_selected_skip_count",
            predecessor_state.dehumidification_control_humidistat_case_selected_skip_count,
            state.dehumidification_control_humidistat_case_selected_skip_count,
        ),
        (
            "constant_supply_humidity_ratio_case_selected_skip_count",
            predecessor_state
                .dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
            state.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
        ),
        (
            "direct_constant_shr_supply_humidity_ratio_mixed_air_limit_count",
            0,
            executed,
        ),
        (
            "direct_humidistat_case_selected_skip_count",
            0,
            state.dehumidification_control_humidistat_case_selected_skip_count,
        ),
        (
            "direct_constant_supply_humidity_ratio_case_selected_skip_count",
            0,
            state.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }

    let latest = state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads supply-humidity-ratio mixed-air limit has no latest snapshot"
            .to_string()
    })?;
    let predecessor_latest = predecessor_state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads supply-humidity-ratio mixed-air limit has no latest CP355 snapshot"
            .to_string()
    })?;
    let expected_system = init.declared_system_order.first().copied().ok_or_else(|| {
        "direct-zone IdealLoads supply-humidity-ratio mixed-air limit has no declared system"
            .to_string()
    })?;
    let expected_zone = init.controlled_zone.ok_or_else(|| {
        "direct-zone IdealLoads supply-humidity-ratio mixed-air limit has no controlled Zone"
            .to_string()
    })?;
    if state.system != expected_system
        || predecessor_state.system != expected_system
        || latest.system != expected_system
        || predecessor_latest.system != expected_system
        || latest.parent_call_ordinal != calls
        || predecessor_latest.parent_call_ordinal != calls
        || latest.controlled_zone != expected_zone
        || predecessor_latest.controlled_zone != expected_zone
        || !snapshots_match_exact_bits(latest, &expected_snapshot(*predecessor_latest))
    {
        return Err(
            "direct-zone IdealLoads supply-humidity-ratio mixed-air-limit latest state is not release-ready"
                .to_string(),
        );
    }
    Ok(())
}

fn validate_route_partition(
    state: &PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitRuntimeState,
) -> Result<(), String> {
    let partition = checked_sum(&[
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        state.positive_guard_false_fallthrough_skip_count,
        state.dehumidification_control_none_case_completed_skip_count,
        state
            .dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_mixed_air_limit_count,
        state.dehumidification_control_humidistat_case_selected_skip_count,
        state.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
    ])?;
    ensure_count(partition, state.transition_count, "transition_partition")
}

fn validate_source_counters(
    state: &PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitRuntimeState,
) -> Result<(), String> {
    let executed = state
        .dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_mixed_air_limit_count;
    let source_sites = executed
        .checked_mul(
            PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_SOURCE_ORDER
                .len(),
        )
        .ok_or_else(|| {
            "direct-zone IdealLoads supply-humidity-ratio mixed-air-limit source counter overflow"
                .to_string()
        })?;
    for (field, expected, actual) in [
        (
            "source_site_execution_count",
            source_sites,
            state.source_site_execution_count,
        ),
        (
            "supply_humidity_ratio_for_mixed_air_limit_minimum_read_count",
            executed,
            state.supply_humidity_ratio_for_mixed_air_limit_minimum_read_count,
        ),
        (
            "mixed_air_humidity_ratio_for_minimum_read_count",
            executed,
            state.mixed_air_humidity_ratio_for_minimum_read_count,
        ),
        (
            "source_shaped_two_argument_minimum_evaluation_count",
            executed,
            state.source_shaped_two_argument_minimum_evaluation_count,
        ),
        (
            "supply_humidity_ratio_assignment_write_count",
            executed,
            state.supply_humidity_ratio_assignment_write_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }
    Ok(())
}

fn expected_snapshot(
    predecessor: PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMinimumLimitSnapshot,
) -> PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitSnapshot {
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitSnapshot {
        source:
            PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_SOURCE_ORDER,
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
        predecessor_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_minimum_limit_executed:
            predecessor
                .dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_minimum_limit_executed,
        predecessor_dehumidification_control_humidistat_case_selected_skip:
            predecessor.dehumidification_control_humidistat_case_selected_skip,
        predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip:
            predecessor
                .dehumidification_control_constant_supply_humidity_ratio_case_selected_skip,
        dehumidification_control_none_case_completed_skip:
            predecessor.dehumidification_control_none_case_completed_skip,
        dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_mixed_air_limit_executed:
            false,
        dehumidification_control_humidistat_case_selected_skip:
            predecessor.dehumidification_control_humidistat_case_selected_skip,
        dehumidification_control_constant_supply_humidity_ratio_case_selected_skip:
            predecessor
                .dehumidification_control_constant_supply_humidity_ratio_case_selected_skip,
        supply_humidity_ratio_for_mixed_air_limit_minimum_read: false,
        supply_humidity_ratio_before_mixed_air_limit: None,
        mixed_air_humidity_ratio_for_minimum_read: false,
        mixed_air_humidity_ratio: None,
        source_shaped_two_argument_minimum_evaluated: false,
        minimum_supply_humidity_ratio: None,
        supply_humidity_ratio_assignment_performed: false,
        assigned_supply_humidity_ratio: None,
        resulting_supply_humidity_ratio: None,
    }
}

fn snapshots_match_exact_bits(
    left: &PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitSnapshot,
    right: &PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitSnapshot,
) -> bool {
    let values_match = [
        (
            left.supply_humidity_ratio_before_mixed_air_limit,
            right.supply_humidity_ratio_before_mixed_air_limit,
        ),
        (
            left.mixed_air_humidity_ratio,
            right.mixed_air_humidity_ratio,
        ),
        (
            left.minimum_supply_humidity_ratio,
            right.minimum_supply_humidity_ratio,
        ),
        (
            left.assigned_supply_humidity_ratio,
            right.assigned_supply_humidity_ratio,
        ),
        (
            left.resulting_supply_humidity_ratio,
            right.resulting_supply_humidity_ratio,
        ),
    ]
    .into_iter()
    .all(|(left, right)| option_bits_match(left, right));
    let mut left = *left;
    let mut right = *right;
    for snapshot in [&mut left, &mut right] {
        snapshot.supply_humidity_ratio_before_mixed_air_limit = None;
        snapshot.mixed_air_humidity_ratio = None;
        snapshot.minimum_supply_humidity_ratio = None;
        snapshot.assigned_supply_humidity_ratio = None;
        snapshot.resulting_supply_humidity_ratio = None;
    }
    values_match && left == right
}

fn option_bits_match(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
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
            "direct-zone IdealLoads supply-humidity-ratio mixed air limit {field} expected {expected}, got {actual}"
        ))
    }
}

#[cfg(test)]
mod tests;
