//! Coupled-runtime validation for CP415 saturation-temperature mixed-air-limit evidence.

use ep_model::DehumidificationControlType;

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_CHILD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_TEMPERATURE_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_TEMPERATURE_SATURATION_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_TEMPERATURE_SATURATION_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_TEMPERATURE_SATURATION_MIXED_AIR_LIMIT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_TEMPERATURE_SATURATION_MIXED_AIR_LIMIT_SOURCE_ORDER,
    PurchasedAirCalcCoolingMixedAirCallLifecycleSummary as MixedAirLifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationAssignmentLifecycleSummary as PredecessorLifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationMixedAirLimitLifecycleSummary as Lifecycle,
    cooling_mixed_air_call_snapshot_is_exact_direct_release,
    cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_snapshot_is_exact_direct_release,
    cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_snapshot_is_exact_direct_release,
    cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_snapshots_match_bit_exact,
    private_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_characterization,
};

use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

const EXPECTED_SOURCE_ORDER: &[&str] = &[
    "read-purchased-air-supply-temperature-for-minimum",
    "read-purchased-air-mixed-air-temperature-for-minimum",
    "apply-source-shaped-two-argument-minimum",
    "assign-purchased-air-supply-temperature",
];
const PUBLIC_LOGICAL_INDICES: [usize; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 20, 21, 26, 27];

pub(in crate::ideal_loads) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let predecessor = output
        .calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment;
    let mixed_air_owner = output.calculation_cooling_mixed_air_call;
    let snapshot = output
        .calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit;
    let active = predecessor
        .post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_executed;
    let expected = private_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_characterization(
        predecessor,
        active.then_some(mixed_air_owner),
    );

    snapshot.system == binding.ideal_loads_air_system
        && snapshot.parent_call_ordinal == call_ordinal
        && snapshot.controlled_zone == binding.zone
        && binding.system.dehumidification_control_type == DehumidificationControlType::None
        && cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_snapshot_is_exact_direct_release(predecessor)
        && cooling_mixed_air_call_snapshot_is_exact_direct_release(mixed_air_owner)
        && cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_snapshot_is_exact_direct_release(snapshot)
        && expected.is_some_and(|expected| {
            cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_snapshots_match_bit_exact(snapshot, expected)
        })
}

pub(in crate::ideal_loads) fn validate_lifecycle(
    lifecycle: &Lifecycle,
    predecessor_cp414: &PredecessorLifecycle,
    mixed_air_owner_cp329: &MixedAirLifecycle,
    timestep_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), Error> {
    let state = &lifecycle.state;
    let predecessor = &predecessor_cp414.state;
    let mixed_air = &mixed_air_owner_cp329.state;
    if lifecycle.source
        != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_TEMPERATURE_SATURATION_MIXED_AIR_LIMIT_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_TEMPERATURE_SATURATION_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE
        || predecessor_cp414.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_TEMPERATURE_SATURATION_ASSIGNMENT_SOURCE
        || predecessor_cp414.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_TEMPERATURE_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || mixed_air_owner_cp329.source != PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE
        || mixed_air_owner_cp329.child_source != PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_CHILD_SOURCE
        || mixed_air_owner_cp329.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_TEMPERATURE_SATURATION_MIXED_AIR_LIMIT_SOURCE_ORDER
            != EXPECTED_SOURCE_ORDER
        || [state.system, predecessor.system, mixed_air.system]
            .into_iter()
            .any(|system| system != binding.ideal_loads_air_system)
        || binding.system.dehumidification_control_type != DehumidificationControlType::None
        || state.predecessor_route_counts != predecessor.predecessor_route_counts
        || state.predecessor_guard_false_fallthrough_route_counts
            != predecessor.predecessor_guard_false_fallthrough_route_counts
        || state.predecessor_guard_body_entry_route_counts
            != predecessor.predecessor_guard_body_entry_route_counts
        || state.predecessor_supply_temperature_saturation_assignment_route_counts
            != predecessor.supply_temperature_saturation_assignment_route_counts
        || state.supply_temperature_mixed_air_limit_route_counts
            != predecessor.supply_temperature_saturation_assignment_route_counts
    {
        return Err(violation("source_owner_predecessor_route_and_system_identity", 1, 0));
    }
    for values in [
        &state.predecessor_route_counts,
        &state.predecessor_guard_false_fallthrough_route_counts,
        &state.predecessor_guard_body_entry_route_counts,
        &state.predecessor_supply_temperature_saturation_assignment_route_counts,
        &state.supply_temperature_mixed_air_limit_route_counts,
    ] {
        ensure_public_routes_only(values)?;
    }

    let transitions = checked_sum(&state.predecessor_route_counts, "route_partition_overflow")?;
    let assignments = checked_sum(
        &state.supply_temperature_mixed_air_limit_route_counts,
        "mixed_air_limit_partition_overflow",
    )?;
    let expected_sites = assignments
        .checked_mul(EXPECTED_SOURCE_ORDER.len())
        .ok_or_else(|| violation("source_site_execution_count_overflow", 0, usize::MAX))?;
    let unchanged_temperatures = state
        .cp414_supply_temperature_state_owner_count
        .checked_sub(assignments)
        .ok_or_else(|| {
            violation(
                "temperature_preservation_partition_underflow",
                assignments,
                state.cp414_supply_temperature_state_owner_count,
            )
        })?;

    for (field, expected, actual) in [
        ("transition_count", timestep_count, state.transition_count),
        (
            "predecessor_transition_count",
            predecessor.transition_count,
            state.transition_count,
        ),
        (
            "mixed_air_owner_transition_count",
            mixed_air.transition_count,
            state.transition_count,
        ),
        ("route_partition", state.transition_count, transitions),
        (
            "inactive_transition_count",
            predecessor.inactive_transition_count,
            state.inactive_transition_count,
        ),
        (
            "transition_partition",
            state.transition_count,
            state
                .inactive_transition_count
                .checked_add(assignments)
                .ok_or_else(|| violation("transition_partition_overflow", 0, usize::MAX))?,
        ),
        (
            "predecessor_supply_temperature_saturation_assignment_count",
            predecessor.saturation_supply_temperature_assignment_count,
            state.predecessor_supply_temperature_saturation_assignment_count,
        ),
        (
            "supply_temperature_saturation_mixed_air_limit_count",
            assignments,
            state.supply_temperature_saturation_mixed_air_limit_count,
        ),
        (
            "cp414_assignment_equals_cp415_limit",
            predecessor.saturation_supply_temperature_assignment_count,
            assignments,
        ),
        (
            "source_site_execution_count",
            expected_sites,
            state.source_site_execution_count,
        ),
        (
            "cp414_supply_humidity_ratio_state_owner_count",
            predecessor.cp413_supply_humidity_ratio_state_owner_count,
            state.cp414_supply_humidity_ratio_state_owner_count,
        ),
        (
            "unchanged_supply_humidity_ratio_preservation_count",
            predecessor.unchanged_supply_humidity_ratio_preservation_count,
            state.unchanged_supply_humidity_ratio_preservation_count,
        ),
        (
            "cp414_supply_enthalpy_state_owner_count",
            predecessor.cp413_supply_enthalpy_state_owner_count,
            state.cp414_supply_enthalpy_state_owner_count,
        ),
        (
            "unchanged_supply_enthalpy_preservation_count",
            predecessor.unchanged_supply_enthalpy_preservation_count,
            state.unchanged_supply_enthalpy_preservation_count,
        ),
        (
            "cp414_supply_temperature_state_owner_count",
            predecessor.cp413_supply_temperature_state_owner_count,
            state.cp414_supply_temperature_state_owner_count,
        ),
        (
            "unchanged_supply_temperature_preservation_count",
            unchanged_temperatures,
            state.unchanged_supply_temperature_preservation_count,
        ),
        (
            "cp415_mixed_air_limited_supply_temperature_state_owner_count",
            assignments,
            state.cp415_mixed_air_limited_supply_temperature_state_owner_count,
        ),
        (
            "cp414_retained_supply_temperature_owned_read_count",
            assignments,
            state.cp414_retained_supply_temperature_owned_read_count,
        ),
        (
            "supply_temperature_for_minimum_read_count",
            assignments,
            state.supply_temperature_for_minimum_read_count,
        ),
        (
            "cp329_retained_mixed_air_temperature_owned_read_count",
            assignments,
            state.cp329_retained_mixed_air_temperature_owned_read_count,
        ),
        (
            "mixed_air_temperature_for_minimum_read_count",
            assignments,
            state.mixed_air_temperature_for_minimum_read_count,
        ),
        (
            "source_shaped_two_argument_minimum_evaluation_count",
            assignments,
            state.source_shaped_two_argument_minimum_evaluation_count,
        ),
        (
            "supply_temperature_assignment_write_count",
            assignments,
            state.supply_temperature_assignment_write_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }
    if mixed_air.cooling_call_count < assignments {
        return Err(violation(
            "cp329_cooling_call_owner_coverage",
            assignments,
            mixed_air.cooling_call_count,
        ));
    }

    let latest = state
        .latest
        .ok_or_else(|| violation("latest_release_snapshot_ready", 1, 0))?;
    let predecessor_latest = predecessor
        .latest
        .ok_or_else(|| violation("predecessor_latest_release_snapshot_ready", 1, 0))?;
    let active = predecessor_latest
        .post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_executed;
    let mixed_air_latest = active.then_some(mixed_air.latest).flatten();
    let expected = private_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_characterization(
        predecessor_latest,
        mixed_air_latest,
    )
    .ok_or_else(|| violation("latest_owner_lineage_ready", 1, 0))?;
    let output_latest = latest_output
        .calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit;
    if !cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_snapshots_match_bit_exact(latest, expected)
        || !cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_snapshots_match_bit_exact(latest, output_latest)
        || !snapshot_matches_release(latest_output, timestep_count, binding)
    {
        return Err(violation("latest_release_snapshot_ready", 1, 0));
    }
    Ok(())
}

fn ensure_public_routes_only(values: &[usize; 36]) -> Result<(), Error> {
    for (index, count) in values.iter().enumerate() {
        if !PUBLIC_LOGICAL_INDICES.contains(&index) && *count != 0 {
            return Err(violation("non_direct_route_count", 0, *count));
        }
    }
    Ok(())
}

fn checked_sum(values: &[usize], field: &'static str) -> Result<usize, Error> {
    values.iter().try_fold(0usize, |sum, value| {
        sum.checked_add(*value)
            .ok_or_else(|| violation(field, 0, usize::MAX))
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
    Error::CalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationMixedAirLimitLifecycleInvariant {
        field,
        expected,
        actual,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conceptual_cp415_contract_is_54_routes_18_limits_and_72_sites() {
        assert_eq!((54 - 18, 18 * EXPECTED_SOURCE_ORDER.len()), (36, 72));
    }

    #[test]
    fn overflow_helper_fails_closed() {
        assert!(checked_sum(&[usize::MAX, 1], "overflow").is_err());
    }
}
