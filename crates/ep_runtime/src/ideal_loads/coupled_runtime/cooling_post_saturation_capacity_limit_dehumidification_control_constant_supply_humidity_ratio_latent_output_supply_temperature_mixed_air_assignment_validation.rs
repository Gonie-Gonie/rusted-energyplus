//! Coupled-runtime validation for CP403 supply-temperature mixed-air assignment evidence.

use ep_model::DehumidificationControlType;

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_CHILD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingMixedAirCallLifecycleSummary as MixedAirLifecycle,
    PurchasedAirCalcCoolingMixedAirCallSnapshot as MixedAirSnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputGuardLifecycleSummary as PredecessorLifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputGuardSnapshot as PredecessorSnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureMixedAirAssignmentLifecycleSummary as Lifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureMixedAirAssignmentSnapshot as Snapshot,
    cooling_mixed_air_call_snapshot_is_exact_direct_release,
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_assignment_snapshot_is_exact_direct_release,
    private_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_assignment_characterization,
};

use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

mod snapshot;
use snapshot::same_snapshot;

const EXPECTED_SOURCE_ORDER: &[&str] = &[
    "read-retained-mixed-air-temperature-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-latent-output-capacity-adjustment-supply-temperature-assignment",
    "assign-purchased-air-supply-temperature-from-mixed-air-temperature-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-latent-output-capacity-adjustment",
];

pub(in crate::ideal_loads) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let predecessor = output
        .calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard;
    let mixed_air_owner = output.calculation_cooling_mixed_air_call;
    let snapshot = output
        .calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_assignment;
    let expected = private_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_assignment_characterization(
        predecessor,
    );

    metadata_matches(
        snapshot,
        predecessor,
        mixed_air_owner,
        call_ordinal,
        binding,
    ) && binding.system.dehumidification_control_type == DehumidificationControlType::None
        && cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_assignment_snapshot_is_exact_direct_release(snapshot)
        && mixed_air_owner_matches(predecessor, mixed_air_owner)
        && expected.is_some_and(|expected| same_snapshot(snapshot, expected))
}

pub(in crate::ideal_loads) fn validate_lifecycle(
    lifecycle: &Lifecycle,
    predecessor_cp402: &PredecessorLifecycle,
    mixed_air_owner_cp329: &MixedAirLifecycle,
    timestep_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), Error> {
    let state = &lifecycle.state;
    let predecessor = &predecessor_cp402.state;
    let mixed_air = &mixed_air_owner_cp329.state;

    if lifecycle.source
        != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_ASSIGNMENT_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || predecessor_cp402.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_GUARD_SOURCE
        || predecessor_cp402.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE
        || mixed_air_owner_cp329.source != PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE
        || mixed_air_owner_cp329.child_source
            != PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_CHILD_SOURCE
        || mixed_air_owner_cp329.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_ASSIGNMENT_SOURCE_ORDER
            != EXPECTED_SOURCE_ORDER
        || [state.system, predecessor.system, mixed_air.system]
            .into_iter()
            .any(|system| system != binding.ideal_loads_air_system)
        || state.predecessor_route_counts != predecessor.predecessor_route_counts
        || state.predecessor_guard_false_fallthrough_route_counts
            != predecessor.guard_false_fallthrough_route_counts
        || state.supply_temperature_mixed_air_assignment_route_counts
            != predecessor.adjustment_body_entry_route_counts
        || binding.system.dehumidification_control_type != DehumidificationControlType::None
    {
        return Err(violation("source_owner_route_and_system_identity", 1, 0));
    }
    ensure_public_routes_only(&state.predecessor_route_counts)?;

    let route_sum = checked_sum(
        &state.predecessor_route_counts,
        "predecessor_route_partition_overflow",
    )?;
    let guard_false = checked_sum(
        &state.predecessor_guard_false_fallthrough_route_counts,
        "guard_false_route_sum_overflow",
    )?;
    let assignments = checked_sum(
        &state.supply_temperature_mixed_air_assignment_route_counts,
        "assignment_route_sum_overflow",
    )?;
    let inactive = checked_selected_sum(
        &state.predecessor_route_counts,
        &[
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 22, 23, 26, 28,
        ],
        "inactive_route_sum_overflow",
    )?;
    for index in 0..state.predecessor_route_counts.len() {
        let successor_sum = state.predecessor_guard_false_fallthrough_route_counts[index]
            .checked_add(state.supply_temperature_mixed_air_assignment_route_counts[index])
            .ok_or_else(|| violation("successor_route_partition_overflow", 0, usize::MAX))?;
        let expected = if predecessor_index_is_active(index) {
            state.predecessor_route_counts[index]
        } else {
            0
        };
        ensure_count(successor_sum, expected, "successor_route_partition")?;
    }
    let transition_partition = inactive
        .checked_add(guard_false)
        .and_then(|count| count.checked_add(assignments))
        .ok_or_else(|| violation("transition_partition_overflow", 0, usize::MAX))?;
    let sites = assignments
        .checked_mul(EXPECTED_SOURCE_ORDER.len())
        .ok_or_else(|| violation("source_site_execution_count_overflow", 0, usize::MAX))?;
    let expected_humidity_owners = checked_selected_sum(
        &state.predecessor_route_counts,
        &[18, 19, 22, 23, 26, 28],
        "supply_humidity_ratio_owner_count_overflow",
    )?;
    let expected_enthalpy_owners = checked_sum(
        &[
            state.predecessor_route_counts[5],
            state.predecessor_route_counts[8],
            state.predecessor_route_counts[11],
            state.predecessor_route_counts[14],
            checked_sum(
                &state.predecessor_route_counts[17..=29],
                "supply_enthalpy_owner_count_overflow",
            )?,
        ],
        "supply_enthalpy_owner_count_overflow",
    )?;
    let expected_temperature_owners = checked_sum(
        &state.predecessor_route_counts[3..=29],
        "supply_temperature_owner_count_overflow",
    )?;
    let expected_temperature_preservations = expected_temperature_owners
        .checked_sub(assignments)
        .ok_or_else(|| {
        violation(
            "unchanged_supply_temperature_preservation_underflow",
            assignments,
            expected_temperature_owners,
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
        ("predecessor_route_partition", state.transition_count, route_sum),
        ("transition_partition", state.transition_count, transition_partition),
        ("inactive_transition_count", inactive, state.inactive_transition_count),
        (
            "predecessor_inactive_transition_count",
            predecessor.inactive_transition_count,
            inactive,
        ),
        (
            "predecessor_guard_false_fallthrough_count",
            guard_false,
            state.predecessor_guard_false_fallthrough_count,
        ),
        (
            "cp402_guard_false_fallthrough_count",
            predecessor.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough_count,
            guard_false,
        ),
        (
            "supply_temperature_mixed_air_assignment_count",
            assignments,
            state.supply_temperature_mixed_air_assignment_count,
        ),
        (
            "cp402_adjustment_body_entry_count",
            predecessor.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_adjustment_body_entry_count,
            assignments,
        ),
        ("source_site_execution_count", sites, state.source_site_execution_count),
        (
            "cp402_supply_humidity_ratio_state_owner_count",
            expected_humidity_owners,
            state.cp402_supply_humidity_ratio_state_owner_count,
        ),
        (
            "cp402_predecessor_supply_humidity_ratio_state_owner_count",
            predecessor.cp401_supply_humidity_ratio_state_owner_count,
            state.cp402_supply_humidity_ratio_state_owner_count,
        ),
        (
            "unchanged_supply_humidity_ratio_preservation_count",
            expected_humidity_owners,
            state.unchanged_supply_humidity_ratio_preservation_count,
        ),
        (
            "cp402_supply_enthalpy_state_owner_count",
            expected_enthalpy_owners,
            state.cp402_supply_enthalpy_state_owner_count,
        ),
        (
            "cp402_predecessor_supply_enthalpy_state_owner_count",
            predecessor.cp401_supply_enthalpy_state_owner_count,
            state.cp402_supply_enthalpy_state_owner_count,
        ),
        (
            "unchanged_supply_enthalpy_preservation_count",
            expected_enthalpy_owners,
            state.unchanged_supply_enthalpy_preservation_count,
        ),
        (
            "cp402_supply_temperature_state_owner_count",
            expected_temperature_owners,
            state.cp402_supply_temperature_state_owner_count,
        ),
        (
            "cp402_predecessor_supply_temperature_state_owner_count",
            predecessor.cp401_supply_temperature_state_owner_count,
            state.cp402_supply_temperature_state_owner_count,
        ),
        (
            "unchanged_supply_temperature_preservation_count",
            expected_temperature_preservations,
            state.unchanged_supply_temperature_preservation_count,
        ),
        (
            "cp329_mixed_air_temperature_owned_read_count",
            assignments,
            state.cp329_mixed_air_temperature_owned_read_count,
        ),
        (
            "cp402_same_call_mixed_air_temperature_bit_corroboration_count",
            assignments,
            state.cp402_same_call_mixed_air_temperature_bit_corroboration_count,
        ),
        (
            "mixed_air_temperature_read_count",
            assignments,
            state.mixed_air_temperature_read_count,
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
    let mixed_air_latest = mixed_air
        .latest
        .ok_or_else(|| violation("cp329_latest_owner_snapshot_ready", 1, 0))?;
    let expected = private_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_assignment_characterization(
        predecessor_latest,
    )
    .ok_or_else(|| violation("latest_predecessor_lineage_ready", 1, 0))?;
    let latest_route_count = if latest
        .dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_adjustment_supply_temperature_mixed_air_assignment_executed
    {
        assignments
    } else if latest
        .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough
    {
        guard_false
    } else {
        inactive
    };

    if !same_snapshot(latest, expected)
        || !same_snapshot(
            latest,
            latest_output.calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_assignment,
        )
        || !mixed_air_owner_matches(predecessor_latest, mixed_air_latest)
        || !snapshot_matches_release(latest_output, timestep_count, binding)
        || latest_route_count == 0
    {
        return Err(violation("latest_release_snapshot_ready", 1, 0));
    }
    Ok(())
}

fn metadata_matches(
    snapshot: Snapshot,
    predecessor: PredecessorSnapshot,
    mixed_air_owner: MixedAirSnapshot,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    snapshot.system == binding.ideal_loads_air_system
        && snapshot.parent_call_ordinal == call_ordinal
        && snapshot.controlled_zone == binding.zone
        && [predecessor.system, mixed_air_owner.system]
            .into_iter()
            .all(|system| system == snapshot.system)
        && [
            predecessor.parent_call_ordinal,
            mixed_air_owner.parent_call_ordinal,
        ]
        .into_iter()
        .all(|ordinal| ordinal == snapshot.parent_call_ordinal)
        && [predecessor.controlled_zone, mixed_air_owner.controlled_zone]
            .into_iter()
            .all(|zone| zone == snapshot.controlled_zone)
}

fn mixed_air_owner_matches(
    predecessor: PredecessorSnapshot,
    mixed_air_owner: MixedAirSnapshot,
) -> bool {
    let assignment = predecessor
        .dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_adjustment_body_entered;
    if !assignment {
        return true;
    }
    let (Some(carried), Some(owned)) = (
        predecessor.predecessor_mixed_air_temperature_c,
        mixed_air_owner.mixed_air_temperature_c,
    ) else {
        return false;
    };
    predecessor.predecessor_cp329_retained_mixed_air_temperature_owned_read
        && predecessor.predecessor_mixed_air_temperature_read
        && mixed_air_owner.cooling_call_executed
        && mixed_air_owner.no_outdoor_air_fallback_entered
        && mixed_air_owner.mixed_air_temperature_assigned
        && cooling_mixed_air_call_snapshot_is_exact_direct_release(mixed_air_owner)
        && carried.to_bits() == owned.to_bits()
}

const fn predecessor_index_is_active(index: usize) -> bool {
    matches!(index, 20 | 21 | 24 | 25 | 27 | 29)
}

fn ensure_public_routes_only(routes: &[usize; 30]) -> Result<(), Error> {
    for (index, count) in routes.iter().enumerate() {
        if !matches!(index, 0..=8 | 20 | 24) && *count != 0 {
            return Err(violation("non_direct_route_count", 0, *count));
        }
    }
    Ok(())
}

fn checked_selected_sum(
    values: &[usize; 30],
    indices: &[usize],
    field: &'static str,
) -> Result<usize, Error> {
    indices.iter().try_fold(0usize, |sum, index| {
        sum.checked_add(values[*index])
            .ok_or_else(|| violation(field, 0, usize::MAX))
    })
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
    Error::CalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureMixedAirAssignmentLifecycleInvariant {
        field,
        expected,
        actual,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn route_partition_overflow_fails_closed() {
        assert!(checked_sum(&[usize::MAX, 1], "overflow").is_err());
    }
}
