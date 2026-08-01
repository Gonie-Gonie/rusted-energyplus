//! Release validation for the CP371 nested dehumidification-control guard.

use ep_model::{DehumidificationControlType, HumidificationControlType};

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_CONTROL_HUMIDISTAT_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_CONTROL_HUMIDISTAT_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_CONTROL_HUMIDISTAT_GUARD_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardLifecycleSummary,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardRuntimeState,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardSnapshot,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardLifecycleSummary,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardRuntimeState,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardSnapshot,
    cooling_supply_humidity_ratio_humidification_control_humidistat_guard_snapshot_is_exact_direct_release,
    cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_latest_metadata_is_consistent,
    cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_snapshot_is_exact_direct_release,
};

use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

pub(super) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let predecessor =
        output.calculation_cooling_supply_humidity_ratio_humidification_control_humidistat_guard;
    let snapshot = output
        .calculation_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard;
    snapshot.system == binding.ideal_loads_air_system
        && snapshot.parent_call_ordinal == call_ordinal
        && snapshot.controlled_zone == binding.zone
        && cooling_supply_humidity_ratio_humidification_control_humidistat_guard_snapshot_is_exact_direct_release(
            predecessor,
        )
        && cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_snapshot_is_exact_direct_release(snapshot)
        && snapshot == expected_snapshot(predecessor)
}

pub(super) fn validate_lifecycle(
    lifecycle: &PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardLifecycleSummary,
    predecessor_lifecycle: &PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardLifecycleSummary,
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
    if binding.system.dehumidification_control_type != DehumidificationControlType::None
        || binding.system.humidification_control_type != HumidificationControlType::None
        || lifecycle.source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_FIRST_EXCLUDED_SOURCE
        || predecessor_lifecycle.source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_CONTROL_HUMIDISTAT_GUARD_SOURCE
        || predecessor_lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_CONTROL_HUMIDISTAT_GUARD_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_SOURCE_ORDER.len() != 5
        || PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_CONTROL_HUMIDISTAT_GUARD_SOURCE_ORDER.len() != 3
        || state.system != binding.ideal_loads_air_system
        || predecessor.system != binding.ideal_loads_air_system
        || !cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_latest_metadata_is_consistent(
            state,
            timestep_count,
        )
        || !cooling_supply_humidity_ratio_humidification_control_humidistat_guard_snapshot_is_exact_direct_release(
            *predecessor_latest,
        )
        || *predecessor_latest
            != latest_output
                .calculation_cooling_supply_humidity_ratio_humidification_control_humidistat_guard
        || !cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_snapshot_is_exact_direct_release(
            *latest,
        )
        || *latest != expected_snapshot(*predecessor_latest)
        || *latest
            != latest_output
                .calculation_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard
        || !snapshot_matches_release(latest_output, timestep_count, binding)
    {
        return Err(violation("latest_release_snapshot_ready", 1, 0));
    }
    Ok(())
}

fn validate_counts(
    state: &PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardRuntimeState,
    predecessor: &PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardRuntimeState,
    timestep_count: usize,
) -> Result<(), Error> {
    let upstream_partition = checked_sum(&[
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        state.positive_guard_false_fallthrough_skip_count,
        state.dehumidification_control_none_case_completed_skip_count,
        state.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
        state.dehumidification_control_humidistat_case_completed_skip_count,
        state.dehumidification_control_constant_supply_humidity_ratio_case_completed_skip_count,
    ])?;
    let active = checked_sum(&[
        state.dehumidification_control_none_case_completed_skip_count,
        state.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
        state.dehumidification_control_humidistat_case_completed_skip_count,
        state.dehumidification_control_constant_supply_humidity_ratio_case_completed_skip_count,
    ])?;
    let heating_partition = state
        .heating_on_body_entry_count
        .checked_add(state.heating_on_guard_false_fallthrough_count)
        .ok_or_else(|| violation("heating_partition_overflow", usize::MAX, 1))?;
    let humidification_partition = state
        .humidification_control_body_entry_count
        .checked_add(state.humidification_control_guard_false_fallthrough_count)
        .ok_or_else(|| violation("humidification_control_partition_overflow", usize::MAX, 1))?;
    let final_partition = checked_sum(&[
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        state.positive_guard_false_fallthrough_skip_count,
        state.heating_on_guard_false_fallthrough_count,
        state.humidification_control_guard_false_fallthrough_count,
        state.dehumidification_control_type_humidistat_match_count,
        state.dehumidification_control_type_none_match_count,
        state.dehumidification_control_guard_false_fallthrough_count,
    ])?;

    for (field, expected, actual) in [
        ("transition_count", timestep_count, state.transition_count),
        (
            "predecessor_transition_count",
            predecessor.transition_count,
            state.transition_count,
        ),
        (
            "upstream_transition_partition",
            state.transition_count,
            upstream_partition,
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
            "humidistat_case_completed_skip_count",
            predecessor.dehumidification_control_humidistat_case_completed_skip_count,
            state.dehumidification_control_humidistat_case_completed_skip_count,
        ),
        (
            "constant_supply_humidity_ratio_case_completed_skip_count",
            predecessor
                .dehumidification_control_constant_supply_humidity_ratio_case_completed_skip_count,
            state.dehumidification_control_constant_supply_humidity_ratio_case_completed_skip_count,
        ),
        (
            "heating_on_read_count",
            predecessor.heating_on_read_count,
            state.heating_on_read_count,
        ),
        (
            "heating_on_body_entry_count",
            predecessor.heating_on_body_entry_count,
            state.heating_on_body_entry_count,
        ),
        (
            "heating_on_guard_false_fallthrough_count",
            predecessor.heating_on_guard_false_fallthrough_count,
            state.heating_on_guard_false_fallthrough_count,
        ),
        ("active_selector_count", active, state.heating_on_read_count),
        ("heating_partition", active, heating_partition),
        (
            "humidification_control_type_read_count",
            predecessor.humidification_control_type_read_count,
            state.humidification_control_type_read_count,
        ),
        (
            "humidification_control_type_humidistat_comparison_count",
            predecessor.humidification_control_type_humidistat_comparison_count,
            state.humidification_control_type_humidistat_comparison_count,
        ),
        (
            "humidification_control_body_entry_count",
            predecessor.humidification_control_body_entry_count,
            state.humidification_control_body_entry_count,
        ),
        (
            "humidification_control_guard_false_fallthrough_count",
            predecessor.humidification_control_guard_false_fallthrough_count,
            state.humidification_control_guard_false_fallthrough_count,
        ),
        (
            "humidification_control_partition",
            state.humidification_control_type_read_count,
            humidification_partition,
        ),
        (
            "direct_dehumidification_control_type_first_read_count",
            0,
            state.dehumidification_control_type_first_read_count,
        ),
        (
            "direct_dehumidification_control_type_humidistat_comparison_count",
            0,
            state.dehumidification_control_type_humidistat_comparison_count,
        ),
        (
            "direct_dehumidification_control_type_humidistat_match_count",
            0,
            state.dehumidification_control_type_humidistat_match_count,
        ),
        (
            "direct_dehumidification_control_type_second_read_count",
            0,
            state.dehumidification_control_type_second_read_count,
        ),
        (
            "direct_dehumidification_control_type_none_comparison_count",
            0,
            state.dehumidification_control_type_none_comparison_count,
        ),
        (
            "direct_dehumidification_control_type_none_match_count",
            0,
            state.dehumidification_control_type_none_match_count,
        ),
        (
            "direct_dehumidification_control_body_entry_count",
            0,
            state.dehumidification_control_body_entry_count,
        ),
        (
            "direct_dehumidification_control_guard_false_fallthrough_count",
            0,
            state.dehumidification_control_guard_false_fallthrough_count,
        ),
        (
            "direct_source_site_execution_count",
            0,
            state.source_site_execution_count,
        ),
        (
            "final_transition_partition",
            state.transition_count,
            final_partition,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }
    Ok(())
}

pub(super) fn expected_snapshot(
    predecessor: PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardSnapshot,
) -> PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardSnapshot{
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_SOURCE_ORDER,
        system: predecessor.system,
        parent_call_ordinal: predecessor.parent_call_ordinal,
        controlled_zone: predecessor.controlled_zone,
        unit_body_entered: predecessor.unit_body_entered,
        predecessor_cooling_body_entered: predecessor.predecessor_cooling_body_entered,
        predecessor_no_outdoor_air_fallback_entered: predecessor.predecessor_no_outdoor_air_fallback_entered,
        predecessor_positive_supply_mass_flow_body_entered: predecessor.predecessor_positive_supply_mass_flow_body_entered,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        positive_guard_false_fallthrough_skipped: predecessor.positive_guard_false_fallthrough_skipped,
        predecessor_dehumidification_control_type: predecessor.predecessor_dehumidification_control_type,
        predecessor_dehumidification_control_none_case_completed_skip: predecessor.predecessor_dehumidification_control_none_case_completed_skip,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip: predecessor.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
        predecessor_dehumidification_control_humidistat_case_completed_skip: predecessor.predecessor_dehumidification_control_humidistat_case_completed_skip,
        predecessor_dehumidification_control_constant_supply_humidity_ratio_case_completed_skip: predecessor.dehumidification_control_constant_supply_humidity_ratio_case_completed_skip,
        predecessor_dehumidification_control_default_supply_humidity_ratio_case_exited_via_break: predecessor.predecessor_dehumidification_control_default_supply_humidity_ratio_case_exited_via_break,
        dehumidification_control_none_case_completed_skip: predecessor.dehumidification_control_none_case_completed_skip,
        dehumidification_control_constant_sensible_heat_ratio_case_completed_skip: predecessor.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
        dehumidification_control_humidistat_case_completed_skip: predecessor.dehumidification_control_humidistat_case_completed_skip,
        dehumidification_control_constant_supply_humidity_ratio_case_completed_skip: predecessor.dehumidification_control_constant_supply_humidity_ratio_case_completed_skip,
        predecessor_heating_on_read: predecessor.predecessor_heating_on_read,
        predecessor_heating_on: predecessor.predecessor_heating_on,
        predecessor_cooling_supply_humidity_ratio_humidification_body_entered: predecessor.predecessor_cooling_supply_humidity_ratio_humidification_body_entered,
        predecessor_heating_on_guard_false_fallthrough: predecessor.predecessor_heating_on_guard_false_fallthrough,
        predecessor_humidification_control_type_read: predecessor.humidification_control_type_read,
        predecessor_humidification_control_type: predecessor.humidification_control_type,
        predecessor_humidification_control_type_humidistat: predecessor.humidification_control_type_humidistat,
        predecessor_humidification_control_body_entered: predecessor.humidification_control_body_entered,
        predecessor_humidification_control_guard_false_fallthrough: predecessor.humidification_control_guard_false_fallthrough,
        dehumidification_control_type_first_read: false,
        first_dehumidification_control_type: None,
        dehumidification_control_type_humidistat: None,
        dehumidification_control_type_second_read: false,
        second_dehumidification_control_type: None,
        dehumidification_control_type_none: None,
        dehumidification_control_body_entered: false,
        dehumidification_control_guard_false_fallthrough: false,
    }
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
    Error::CalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardLifecycleInvariant {
        field,
        expected,
        actual,
    }
}
