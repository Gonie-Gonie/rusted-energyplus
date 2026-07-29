//! Release validation for the bounded Humidistat case break.

use ep_model::DehumidificationControlType;

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_CASE_BREAK_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_CASE_BREAK_SOURCE,
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_CASE_BREAK_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_SOURCE,
    PurchasedAirCalcCoolingHumidistatCaseBreakLifecycleSummary,
    PurchasedAirCalcCoolingHumidistatCaseBreakRuntimeState,
    PurchasedAirCalcCoolingHumidistatCaseBreakSnapshot,
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitLifecycleSummary,
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitRuntimeState,
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitSnapshot,
    cooling_humidistat_case_break_latest_metadata_is_consistent,
    cooling_humidistat_case_break_snapshot_is_exact_direct_release,
    cooling_humidistat_supply_humidity_ratio_mixed_air_limit_snapshot_is_exact_direct_release,
    cooling_humidistat_supply_humidity_ratio_mixed_air_limit_snapshots_match_bit_exact,
};

use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

pub(super) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let predecessor = output.calculation_cooling_humidistat_supply_humidity_ratio_mixed_air_limit;
    let snapshot = output.calculation_cooling_humidistat_case_break;
    snapshot.system == binding.ideal_loads_air_system
        && snapshot.parent_call_ordinal == call_ordinal
        && snapshot.controlled_zone == binding.zone
        && cooling_humidistat_case_break_snapshot_is_exact_direct_release(snapshot)
        && snapshots_match_exact(&snapshot, &expected_snapshot(predecessor))
}

pub(super) fn validate_lifecycle(
    lifecycle: &PurchasedAirCalcCoolingHumidistatCaseBreakLifecycleSummary,
    predecessor_lifecycle:
        &PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitLifecycleSummary,
    timestep_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), Error> {
    let state = &lifecycle.state;
    let predecessor = &predecessor_lifecycle.state;
    validate_counts(state, predecessor, timestep_count)?;

    let latest = state
        .latest
        .as_ref()
        .ok_or_else(|| violation("latest_release_snapshot_ready", 1, 0))?;
    let predecessor_latest = predecessor
        .latest
        .as_ref()
        .ok_or_else(|| violation("predecessor_latest_release_snapshot_ready", 1, 0))?;
    // The caller validates CP362 recursively first. CP363 compares that full
    // latest snapshot to the scheduled CP362 witness without re-querying an
    // earlier numeric owner.
    if binding.system.dehumidification_control_type != DehumidificationControlType::None
        || lifecycle.source != PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_CASE_BREAK_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_CASE_BREAK_FIRST_EXCLUDED_SOURCE
        || predecessor_lifecycle.source
            != PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_SOURCE
        || predecessor_lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_CASE_BREAK_SOURCE_ORDER.len() != 1
        || state.system != binding.ideal_loads_air_system
        || predecessor.system != binding.ideal_loads_air_system
        || !cooling_humidistat_case_break_latest_metadata_is_consistent(
            state,
            timestep_count,
        )
        || !cooling_humidistat_supply_humidity_ratio_mixed_air_limit_snapshot_is_exact_direct_release(
            *predecessor_latest,
        )
        || !cooling_humidistat_supply_humidity_ratio_mixed_air_limit_snapshots_match_bit_exact(
            *predecessor_latest,
            latest_output
                .calculation_cooling_humidistat_supply_humidity_ratio_mixed_air_limit,
        )
        || !cooling_humidistat_case_break_snapshot_is_exact_direct_release(*latest)
        || !snapshots_match_exact(latest, &expected_snapshot(*predecessor_latest))
        || !snapshots_match_exact(
            latest,
            &latest_output.calculation_cooling_humidistat_case_break,
        )
        || !snapshot_matches_release(latest_output, timestep_count, binding)
    {
        return Err(violation("latest_release_snapshot_ready", 1, 0));
    }
    Ok(())
}

fn validate_counts(
    state: &PurchasedAirCalcCoolingHumidistatCaseBreakRuntimeState,
    predecessor: &PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitRuntimeState,
    timestep_count: usize,
) -> Result<(), Error> {
    let executed = state.dehumidification_control_humidistat_case_break_count;
    validate_route_partition(state)?;
    validate_source_counters(state)?;
    for (field, expected, actual) in [
        ("transition_count", timestep_count, state.transition_count),
        (
            "predecessor_transition_count",
            predecessor.transition_count,
            state.transition_count,
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
            "positive_guard_false_fallthrough_skip_count",
            predecessor.positive_guard_false_fallthrough_skip_count,
            state.positive_guard_false_fallthrough_skip_count,
        ),
        (
            "none_case_completed_skip_count",
            predecessor.dehumidification_control_none_case_completed_skip_count,
            state.dehumidification_control_none_case_completed_skip_count,
        ),
        (
            "constant_sensible_heat_ratio_case_completed_skip_count",
            predecessor
                .dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
            state.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
        ),
        (
            "humidistat_case_break_count",
            predecessor
                .dehumidification_control_humidistat_supply_humidity_ratio_mixed_air_limit_count,
            executed,
        ),
        (
            "constant_supply_humidity_ratio_case_selected_skip_count",
            predecessor
                .dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
            state.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
        ),
        (
            "direct_constant_sensible_heat_ratio_case_completed_skip_count",
            0,
            state.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
        ),
        ("direct_humidistat_case_break_count", 0, executed),
        (
            "direct_constant_supply_humidity_ratio_case_selected_skip_count",
            0,
            state.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }
    Ok(())
}

fn validate_route_partition(
    state: &PurchasedAirCalcCoolingHumidistatCaseBreakRuntimeState,
) -> Result<(), Error> {
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
) -> Result<(), Error> {
    let executed = state.dehumidification_control_humidistat_case_break_count;
    let source_sites = executed
        .checked_mul(PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_CASE_BREAK_SOURCE_ORDER.len())
        .ok_or_else(|| violation("source_site_execution_count_overflow", usize::MAX, executed))?;
    ensure_count(
        state.source_site_execution_count,
        source_sites,
        "source_site_execution_count",
    )
}

pub(super) fn expected_snapshot(
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
        dehumidification_control_constant_supply_humidity_ratio_case_selected_skip:
            predecessor
                .dehumidification_control_constant_supply_humidity_ratio_case_selected_skip,
    }
}

pub(super) fn snapshots_match_exact(
    left: &PurchasedAirCalcCoolingHumidistatCaseBreakSnapshot,
    right: &PurchasedAirCalcCoolingHumidistatCaseBreakSnapshot,
) -> bool {
    left == right
}

fn checked_sum(values: &[usize]) -> Result<usize, Error> {
    values.iter().try_fold(0usize, |sum, value| {
        sum.checked_add(*value)
            .ok_or_else(|| violation("transition_partition_overflow", usize::MAX, *value))
    })
}

fn ensure_count(actual: usize, expected: usize, field: &'static str) -> Result<(), Error> {
    if actual == expected {
        Ok(())
    } else {
        Err(violation(field, expected, actual))
    }
}

fn violation(field: &'static str, expected: usize, actual: usize) -> Error {
    Error::CalcCoolingHumidistatCaseBreakLifecycleInvariant {
        field,
        expected,
        actual,
    }
}
