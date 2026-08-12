//! Coupled-runtime validation for CP417 psychrometric supply-enthalpy assignment evidence.

use ep_model::DehumidificationControlType;

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyEnthalpyAssignmentLifecycleSummary as Lifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioAssignmentLifecycleSummary as PredecessorLifecycle,
    cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment_snapshot_is_exact_direct_release,
    cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment_snapshots_match_bit_exact,
    cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment_snapshot_is_exact_direct_release,
    private_cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment_characterization,
};

use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

const EXPECTED_SOURCE_ORDER: &[&str] = &[
    "read-purchased-air-supply-temperature-for-post-saturation-capacity-limit-dehumidification-enthalpy",
    "read-purchased-air-supply-humidity-ratio-for-post-saturation-capacity-limit-dehumidification-enthalpy",
    "evaluate-psy-h-fn-tdb-w-for-post-saturation-capacity-limit-dehumidification",
    "assign-local-supply-enthalpy-after-post-saturation-capacity-limit-dehumidification-humidity-ratio-assignment",
];
const PUBLIC_LOGICAL_INDICES: [usize; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 20, 21, 26, 27];

pub(in crate::ideal_loads) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let predecessor = output
        .calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment;
    let snapshot = output
        .calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment;
    let expected = private_cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment_characterization(predecessor);

    snapshot.system == binding.ideal_loads_air_system
        && snapshot.parent_call_ordinal == call_ordinal
        && snapshot.controlled_zone == binding.zone
        && binding.system.dehumidification_control_type == DehumidificationControlType::None
        && cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment_snapshot_is_exact_direct_release(predecessor)
        && cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment_snapshot_is_exact_direct_release(snapshot)
        && expected.is_some_and(|expected| {
            cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment_snapshots_match_bit_exact(snapshot, expected)
        })
}

pub(in crate::ideal_loads) fn validate_lifecycle(
    lifecycle: &Lifecycle,
    predecessor_cp416: &PredecessorLifecycle,
    timestep_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), Error> {
    let state = &lifecycle.state;
    let predecessor = &predecessor_cp416.state;
    if lifecycle.source
        != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || predecessor_cp416.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE
        || predecessor_cp416.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER
            != EXPECTED_SOURCE_ORDER
        || [state.system, predecessor.system]
            .into_iter()
            .any(|system| system != binding.ideal_loads_air_system)
        || binding.system.dehumidification_control_type != DehumidificationControlType::None
        || state.predecessor_route_counts != predecessor.predecessor_route_counts
        || state.predecessor_guard_false_fallthrough_route_counts
            != predecessor.predecessor_guard_false_fallthrough_route_counts
        || state.predecessor_guard_body_entry_route_counts
            != predecessor.predecessor_guard_body_entry_route_counts
        || state.predecessor_supply_temperature_saturation_assignment_route_counts
            != predecessor.predecessor_supply_temperature_saturation_assignment_route_counts
        || state.predecessor_supply_temperature_mixed_air_limit_route_counts
            != predecessor.predecessor_supply_temperature_mixed_air_limit_route_counts
        || state.predecessor_supply_humidity_ratio_assignment_route_counts
            != predecessor.supply_humidity_ratio_assignment_route_counts
        || state.supply_enthalpy_assignment_route_counts
            != predecessor.supply_humidity_ratio_assignment_route_counts
    {
        return Err(violation(
            "source_owner_predecessor_route_and_system_identity",
            1,
            0,
        ));
    }
    for values in [
        &state.predecessor_route_counts,
        &state.predecessor_guard_false_fallthrough_route_counts,
        &state.predecessor_guard_body_entry_route_counts,
        &state.predecessor_supply_temperature_saturation_assignment_route_counts,
        &state.predecessor_supply_temperature_mixed_air_limit_route_counts,
        &state.predecessor_supply_humidity_ratio_assignment_route_counts,
        &state.supply_enthalpy_assignment_route_counts,
    ] {
        ensure_public_routes_only(values)?;
    }

    let transitions = checked_sum(&state.predecessor_route_counts, "route_partition_overflow")?;
    let predecessor_assignments = checked_sum(
        &state.predecessor_supply_temperature_saturation_assignment_route_counts,
        "predecessor_assignment_partition_overflow",
    )?;
    let predecessor_limits = checked_sum(
        &state.predecessor_supply_temperature_mixed_air_limit_route_counts,
        "predecessor_limit_partition_overflow",
    )?;
    let predecessor_humidity_ratio_assignments = checked_sum(
        &state.predecessor_supply_humidity_ratio_assignment_route_counts,
        "predecessor_humidity_ratio_assignment_partition_overflow",
    )?;
    let assignments = checked_sum(
        &state.supply_enthalpy_assignment_route_counts,
        "assignment_partition_overflow",
    )?;
    let inactive = transitions
        .checked_sub(assignments)
        .ok_or_else(|| violation("transition_partition_underflow", assignments, transitions))?;
    let humidity_ratio_owners = checked_sum(
        &state.predecessor_route_counts[18..],
        "humidity_ratio_owner_partition_overflow",
    )?;
    let enthalpy_owners = sum_predecessor_indices(&state.predecessor_route_counts, |index| {
        matches!(index, 5 | 8 | 11 | 14 | 17..=29)
    })?;
    let temperature_owners =
        sum_predecessor_indices(&state.predecessor_route_counts, |index| index >= 3)?;
    let unchanged_enthalpy = enthalpy_owners.checked_sub(assignments).ok_or_else(|| {
        violation(
            "enthalpy_preservation_partition_underflow",
            assignments,
            enthalpy_owners,
        )
    })?;
    let expected_sites = assignments
        .checked_mul(EXPECTED_SOURCE_ORDER.len())
        .ok_or_else(|| violation("source_site_execution_count_overflow", 0, usize::MAX))?;

    for (field, expected, actual) in [
        ("transition_count", timestep_count, state.transition_count),
        (
            "predecessor_transition_count",
            predecessor.transition_count,
            state.transition_count,
        ),
        ("route_partition", state.transition_count, transitions),
        (
            "inactive_transition_count",
            inactive,
            state.inactive_transition_count,
        ),
        (
            "predecessor_inactive_transition_count",
            predecessor.inactive_transition_count,
            state.inactive_transition_count,
        ),
        (
            "predecessor_supply_temperature_saturation_assignment_count",
            predecessor_assignments,
            state.predecessor_supply_temperature_saturation_assignment_count,
        ),
        (
            "predecessor_supply_temperature_saturation_mixed_air_limit_count",
            predecessor_limits,
            state.predecessor_supply_temperature_saturation_mixed_air_limit_count,
        ),
        (
            "predecessor_supply_humidity_ratio_assignment_count",
            predecessor_humidity_ratio_assignments,
            state.predecessor_supply_humidity_ratio_assignment_count,
        ),
        (
            "cp416_predecessor_supply_humidity_ratio_assignment_count",
            predecessor.supply_humidity_ratio_assignment_count,
            state.predecessor_supply_humidity_ratio_assignment_count,
        ),
        (
            "supply_enthalpy_assignment_count",
            assignments,
            state.supply_enthalpy_assignment_count,
        ),
        (
            "cp416_assignment_equals_cp417_assignment",
            predecessor.supply_humidity_ratio_assignment_count,
            assignments,
        ),
        (
            "source_site_execution_count",
            expected_sites,
            state.source_site_execution_count,
        ),
        (
            "cp416_supply_humidity_ratio_state_owner_count",
            humidity_ratio_owners,
            state.cp416_supply_humidity_ratio_state_owner_count,
        ),
        (
            "cp416_humidity_ratio_owner_lineage",
            predecessor.cp415_supply_humidity_ratio_state_owner_count,
            state.cp416_supply_humidity_ratio_state_owner_count,
        ),
        (
            "unchanged_supply_humidity_ratio_preservation_count",
            humidity_ratio_owners,
            state.unchanged_supply_humidity_ratio_preservation_count,
        ),
        (
            "cp416_supply_enthalpy_state_owner_count",
            enthalpy_owners,
            state.cp416_supply_enthalpy_state_owner_count,
        ),
        (
            "cp416_enthalpy_owner_lineage",
            predecessor.cp415_supply_enthalpy_state_owner_count,
            state.cp416_supply_enthalpy_state_owner_count,
        ),
        (
            "unchanged_supply_enthalpy_preservation_count",
            unchanged_enthalpy,
            state.unchanged_supply_enthalpy_preservation_count,
        ),
        (
            "cp416_supply_temperature_state_owner_count",
            temperature_owners,
            state.cp416_supply_temperature_state_owner_count,
        ),
        (
            "cp416_temperature_owner_lineage",
            predecessor.cp415_supply_temperature_state_owner_count,
            state.cp416_supply_temperature_state_owner_count,
        ),
        (
            "unchanged_supply_temperature_preservation_count",
            temperature_owners,
            state.unchanged_supply_temperature_preservation_count,
        ),
        (
            "cp417_psychrometric_supply_enthalpy_state_owner_count",
            assignments,
            state.cp417_psychrometric_supply_enthalpy_state_owner_count,
        ),
        (
            "cp416_retained_supply_temperature_owned_read_count",
            assignments,
            state.cp416_retained_supply_temperature_owned_read_count,
        ),
        (
            "supply_temperature_for_enthalpy_read_count",
            assignments,
            state.supply_temperature_for_enthalpy_read_count,
        ),
        (
            "cp416_retained_supply_humidity_ratio_owned_read_count",
            assignments,
            state.cp416_retained_supply_humidity_ratio_owned_read_count,
        ),
        (
            "supply_humidity_ratio_for_enthalpy_read_count",
            assignments,
            state.supply_humidity_ratio_for_enthalpy_read_count,
        ),
        (
            "psychrometric_supply_enthalpy_evaluation_count",
            assignments,
            state.psychrometric_supply_enthalpy_evaluation_count,
        ),
        (
            "supply_enthalpy_assignment_write_count",
            assignments,
            state.supply_enthalpy_assignment_write_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }

    let latest = state
        .latest
        .ok_or_else(|| violation("latest_release_snapshot_ready", 1, 0))?;
    let predecessor_latest = predecessor
        .latest
        .ok_or_else(|| violation("predecessor_latest_release_snapshot_ready", 1, 0))?;
    let expected = private_cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment_characterization(predecessor_latest)
        .ok_or_else(|| violation("latest_owner_lineage_ready", 1, 0))?;
    let output_latest = latest_output
        .calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment;
    if !cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment_snapshots_match_bit_exact(latest, expected)
        || !cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment_snapshots_match_bit_exact(latest, output_latest)
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

fn sum_predecessor_indices(
    values: &[usize; 36],
    include: impl Fn(usize) -> bool,
) -> Result<usize, Error> {
    let mut logical_index = 0usize;
    let mut total = 0usize;
    for predecessor_index in 0..30 {
        let width = 1 + usize::from(matches!(predecessor_index, 20 | 21 | 24 | 25 | 27 | 29));
        if include(predecessor_index) {
            total = values[logical_index..logical_index + width]
                .iter()
                .try_fold(total, |sum, value| sum.checked_add(*value))
                .ok_or_else(|| violation("owner_partition_overflow", 0, usize::MAX))?;
        }
        logical_index += width;
    }
    if logical_index == 36 {
        Ok(total)
    } else {
        Err(violation("logical_partition_width", 36, logical_index))
    }
}

fn ensure_count(actual: usize, expected: usize, field: &'static str) -> Result<(), Error> {
    if actual == expected {
        Ok(())
    } else {
        Err(violation(field, expected, actual))
    }
}

fn violation(field: &'static str, expected: usize, actual: usize) -> Error {
    Error::CalcCoolingPostSaturationCapacityLimitDehumidificationSupplyEnthalpyAssignmentLifecycleInvariant {
        field,
        expected,
        actual,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conceptual_cp417_contract_is_54_routes_18_assignments_and_72_sites() {
        assert_eq!((54 - 18, 18 * EXPECTED_SOURCE_ORDER.len()), (36, 72));
    }

    #[test]
    fn overflow_helper_fails_closed() {
        assert!(checked_sum(&[usize::MAX, 1], "overflow").is_err());
    }
}
