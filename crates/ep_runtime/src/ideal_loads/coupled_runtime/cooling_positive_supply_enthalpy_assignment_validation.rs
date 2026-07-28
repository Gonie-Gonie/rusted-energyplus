//! Release validation for the bounded cooling positive-supply enthalpy assignment.

use crate::{
    ideal_loads::{
        DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE,
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER,
        PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentLifecycleSummary,
        PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot,
        PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentLifecycleSummary,
        PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot,
        PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitLifecycleSummary,
        PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot,
    },
    psychrometrics::energyplus_psy_h_fn_tdb_w,
};

use super::super::calc::cooling_positive_supply_enthalpy_assignment_snapshot_is_exact_direct_release;
use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

pub(super) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let predecessor =
        output.calculation_cooling_positive_supply_humidity_ratio_mixed_air_assignment;
    let temperature = output.calculation_cooling_positive_supply_temperature_mixed_air_limit;
    let snapshot = output.calculation_cooling_positive_supply_enthalpy_assignment;
    let expected = expected_snapshot(predecessor, temperature);

    snapshot.system == binding.ideal_loads_air_system
        && snapshot.parent_call_ordinal == call_ordinal
        && snapshot.controlled_zone == binding.zone
        && cooling_positive_supply_enthalpy_assignment_snapshot_is_exact_direct_release(snapshot)
        && source_lineage_matches(output, &snapshot)
        && snapshots_match_exact_bits(&snapshot, &expected)
}

fn expected_snapshot(
    predecessor: PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot,
    temperature: PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot,
) -> PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot {
    let assignment_executed = predecessor.supply_humidity_ratio_mixed_air_assignment_executed;
    let supply_temperature_c = assignment_executed
        .then_some(temperature.assigned_supply_temperature_c)
        .flatten();
    let supply_humidity_ratio = assignment_executed
        .then_some(predecessor.assigned_supply_humidity_ratio)
        .flatten();
    let psychrometric_supply_enthalpy_result_j_per_kg = supply_temperature_c
        .zip(supply_humidity_ratio)
        .map(|(supply_temperature_c, supply_humidity_ratio)| {
            energyplus_psy_h_fn_tdb_w(supply_temperature_c, supply_humidity_ratio)
        });

    PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER,
        system: predecessor.system,
        parent_call_ordinal: predecessor.parent_call_ordinal,
        controlled_zone: predecessor.controlled_zone,
        unit_body_entered: predecessor.unit_body_entered,
        predecessor_cooling_body_entered: predecessor.predecessor_cooling_body_entered,
        predecessor_no_outdoor_air_fallback_entered: predecessor
            .predecessor_no_outdoor_air_fallback_entered,
        predecessor_positive_supply_mass_flow_body_entered: predecessor
            .predecessor_positive_supply_mass_flow_body_entered,
        predecessor_active_guard_false_fallthrough: predecessor
            .predecessor_active_guard_false_fallthrough,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        positive_guard_false_fallthrough_skipped: predecessor
            .positive_guard_false_fallthrough_skipped,
        supply_enthalpy_assignment_executed: assignment_executed,
        supply_temperature_for_enthalpy_read: assignment_executed,
        supply_temperature_c,
        supply_humidity_ratio_for_enthalpy_read: assignment_executed,
        supply_humidity_ratio,
        psychrometric_supply_enthalpy_evaluated: assignment_executed,
        psychrometric_supply_enthalpy_result_j_per_kg,
        supply_enthalpy_assigned: assignment_executed,
        supply_enthalpy_j_per_kg: psychrometric_supply_enthalpy_result_j_per_kg,
    }
}

fn source_lineage_matches(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    snapshot: &PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot,
) -> bool {
    if !snapshot.supply_enthalpy_assignment_executed {
        return true;
    }

    let expected = snapshot
        .supply_temperature_c
        .zip(snapshot.supply_humidity_ratio)
        .map(|(temperature, humidity_ratio)| {
            energyplus_psy_h_fn_tdb_w(temperature, humidity_ratio)
        });
    options_have_exact_bits(
        snapshot.supply_temperature_c,
        output
            .calculation_cooling_positive_supply_temperature_mixed_air_limit
            .assigned_supply_temperature_c,
    ) && options_have_exact_bits(
        snapshot.supply_humidity_ratio,
        output
            .calculation_cooling_positive_supply_humidity_ratio_mixed_air_assignment
            .assigned_supply_humidity_ratio,
    ) && options_have_exact_bits(
        snapshot.psychrometric_supply_enthalpy_result_j_per_kg,
        expected,
    ) && options_have_exact_bits(
        snapshot.supply_enthalpy_j_per_kg,
        snapshot.psychrometric_supply_enthalpy_result_j_per_kg,
    )
}

pub(super) fn validate_lifecycle(
    lifecycle: &PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentLifecycleSummary,
    predecessor_lifecycle: &PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentLifecycleSummary,
    temperature_lifecycle: &PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitLifecycleSummary,
    timestep_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), Error> {
    let state = &lifecycle.state;
    let predecessor = &predecessor_lifecycle.state;
    let temperature = &temperature_lifecycle.state;
    let skipped = checked_add(
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        "skip_partition_overflow",
        timestep_count,
    )?;
    let active_routes = checked_add(
        state.positive_guard_false_fallthrough_skip_count,
        state.supply_enthalpy_assignment_count,
        "active_route_partition_overflow",
        timestep_count,
    )?;
    let transition_partition = checked_add(
        skipped,
        active_routes,
        "transition_partition_overflow",
        timestep_count,
    )?;
    let source_sites = checked_mul(
        state.supply_enthalpy_assignment_count,
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER.len(),
        "source_site_execution_count_overflow",
        state.source_site_execution_count,
    )?;

    for (field, expected, actual) in [
        ("transition_count", timestep_count, state.transition_count),
        (
            "predecessor_transition_count",
            predecessor.transition_count,
            state.transition_count,
        ),
        (
            "temperature_transition_count",
            temperature.transition_count,
            state.transition_count,
        ),
        (
            "transition_partition",
            state.transition_count,
            transition_partition,
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
            "supply_enthalpy_assignment_count",
            predecessor.supply_humidity_ratio_mixed_air_assignment_count,
            state.supply_enthalpy_assignment_count,
        ),
        (
            "temperature_assignment_count",
            temperature.supply_temperature_mixed_air_limit_count,
            state.supply_enthalpy_assignment_count,
        ),
        (
            "source_site_execution_count",
            source_sites,
            state.source_site_execution_count,
        ),
        (
            "supply_temperature_for_enthalpy_read_count",
            state.supply_enthalpy_assignment_count,
            state.supply_temperature_for_enthalpy_read_count,
        ),
        (
            "supply_humidity_ratio_for_enthalpy_read_count",
            state.supply_enthalpy_assignment_count,
            state.supply_humidity_ratio_for_enthalpy_read_count,
        ),
        (
            "psychrometric_supply_enthalpy_evaluation_count",
            state.supply_enthalpy_assignment_count,
            state.psychrometric_supply_enthalpy_evaluation_count,
        ),
        (
            "supply_enthalpy_assignment_write_count",
            state.supply_enthalpy_assignment_count,
            state.supply_enthalpy_assignment_write_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }

    let latest = state
        .latest
        .as_ref()
        .ok_or_else(|| violation("latest_release_snapshot_ready", 1, 0))?;
    let predecessor_latest = predecessor
        .latest
        .as_ref()
        .ok_or_else(|| violation("predecessor_latest_release_snapshot_ready", 1, 0))?;
    let temperature_latest = temperature
        .latest
        .as_ref()
        .ok_or_else(|| violation("temperature_latest_release_snapshot_ready", 1, 0))?;
    let expected = expected_snapshot(*predecessor_latest, *temperature_latest);
    if lifecycle.source != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || state.system != binding.ideal_loads_air_system
        || predecessor.system != binding.ideal_loads_air_system
        || temperature.system != binding.ideal_loads_air_system
        || !humidity_assignment_snapshots_match_exact_bits(
            predecessor_latest,
            &latest_output.calculation_cooling_positive_supply_humidity_ratio_mixed_air_assignment,
        )
        || !temperature_snapshots_match_exact_bits(
            temperature_latest,
            &latest_output.calculation_cooling_positive_supply_temperature_mixed_air_limit,
        )
        || !snapshots_match_exact_bits(latest, &expected)
        || !snapshots_match_exact_bits(
            latest,
            &latest_output.calculation_cooling_positive_supply_enthalpy_assignment,
        )
        || !snapshot_matches_release(latest_output, timestep_count, binding)
    {
        return Err(violation("latest_release_snapshot_ready", 1, 0));
    }
    Ok(())
}

fn snapshots_match_exact_bits(
    left: &PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot,
    right: &PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot,
) -> bool {
    let values_match = [
        (left.supply_temperature_c, right.supply_temperature_c),
        (left.supply_humidity_ratio, right.supply_humidity_ratio),
        (
            left.psychrometric_supply_enthalpy_result_j_per_kg,
            right.psychrometric_supply_enthalpy_result_j_per_kg,
        ),
        (
            left.supply_enthalpy_j_per_kg,
            right.supply_enthalpy_j_per_kg,
        ),
    ]
    .into_iter()
    .all(|(left, right)| options_have_exact_bits(left, right));
    let mut left_without_values = *left;
    let mut right_without_values = *right;
    left_without_values.supply_temperature_c = None;
    right_without_values.supply_temperature_c = None;
    left_without_values.supply_humidity_ratio = None;
    right_without_values.supply_humidity_ratio = None;
    left_without_values.psychrometric_supply_enthalpy_result_j_per_kg = None;
    right_without_values.psychrometric_supply_enthalpy_result_j_per_kg = None;
    left_without_values.supply_enthalpy_j_per_kg = None;
    right_without_values.supply_enthalpy_j_per_kg = None;
    values_match && left_without_values == right_without_values
}

fn humidity_assignment_snapshots_match_exact_bits(
    left: &PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot,
    right: &PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot,
) -> bool {
    let values_match = [
        (
            left.mixed_air_humidity_ratio,
            right.mixed_air_humidity_ratio,
        ),
        (
            left.assigned_supply_humidity_ratio,
            right.assigned_supply_humidity_ratio,
        ),
    ]
    .into_iter()
    .all(|(left, right)| options_have_exact_bits(left, right));
    let mut left_without_values = *left;
    let mut right_without_values = *right;
    left_without_values.mixed_air_humidity_ratio = None;
    right_without_values.mixed_air_humidity_ratio = None;
    left_without_values.assigned_supply_humidity_ratio = None;
    right_without_values.assigned_supply_humidity_ratio = None;
    values_match && left_without_values == right_without_values
}

fn temperature_snapshots_match_exact_bits(
    left: &PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot,
    right: &PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot,
) -> bool {
    let values_match = [
        (
            left.supply_temperature_before_mixed_air_limit_c,
            right.supply_temperature_before_mixed_air_limit_c,
        ),
        (left.mixed_air_temperature_c, right.mixed_air_temperature_c),
        (
            left.minimum_supply_temperature_c,
            right.minimum_supply_temperature_c,
        ),
        (
            left.assigned_supply_temperature_c,
            right.assigned_supply_temperature_c,
        ),
    ]
    .into_iter()
    .all(|(left, right)| options_have_exact_bits(left, right));
    let mut left_without_values = *left;
    let mut right_without_values = *right;
    left_without_values.supply_temperature_before_mixed_air_limit_c = None;
    right_without_values.supply_temperature_before_mixed_air_limit_c = None;
    left_without_values.mixed_air_temperature_c = None;
    right_without_values.mixed_air_temperature_c = None;
    left_without_values.minimum_supply_temperature_c = None;
    right_without_values.minimum_supply_temperature_c = None;
    left_without_values.assigned_supply_temperature_c = None;
    right_without_values.assigned_supply_temperature_c = None;
    values_match && left_without_values == right_without_values
}

fn options_have_exact_bits(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}

pub(super) fn checked_add(
    left: usize,
    right: usize,
    field: &'static str,
    expected: usize,
) -> Result<usize, Error> {
    left.checked_add(right)
        .ok_or_else(|| violation(field, expected, usize::MAX))
}

pub(super) fn checked_mul(
    left: usize,
    right: usize,
    field: &'static str,
    expected: usize,
) -> Result<usize, Error> {
    left.checked_mul(right)
        .ok_or_else(|| violation(field, expected, usize::MAX))
}

fn ensure_count(actual: usize, expected: usize, field: &'static str) -> Result<(), Error> {
    if actual == expected {
        Ok(())
    } else {
        Err(violation(field, expected, actual))
    }
}

fn violation(field: &'static str, expected: usize, actual: usize) -> Error {
    Error::CalcCoolingPositiveSupplyEnthalpyAssignmentLifecycleInvariant {
        field,
        expected,
        actual,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transition_partition_addition_overflow_fails_closed() {
        let error = checked_add(
            usize::MAX,
            1,
            "test_transition_partition_overflow",
            usize::MAX,
        )
        .expect_err("transition partition overflow must fail closed");

        assert!(matches!(
            error,
            Error::CalcCoolingPositiveSupplyEnthalpyAssignmentLifecycleInvariant {
                field: "test_transition_partition_overflow",
                expected: usize::MAX,
                actual: usize::MAX,
            }
        ));
    }

    #[test]
    fn source_site_count_multiplication_overflow_fails_closed() {
        let error = checked_mul(
            usize::MAX,
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER.len(),
            "test_source_site_count_overflow",
            usize::MAX,
        )
        .expect_err("source-site multiplication overflow must fail closed");

        assert!(matches!(
            error,
            Error::CalcCoolingPositiveSupplyEnthalpyAssignmentLifecycleInvariant {
                field: "test_source_site_count_overflow",
                expected: usize::MAX,
                actual: usize::MAX,
            }
        ));
    }
}
