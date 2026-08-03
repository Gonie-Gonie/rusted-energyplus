//! Coupled-runtime validation for CP405 latent-output maximum-capacity assignment evidence.

use ep_model::DehumidificationControlType;

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputMaximumCapacityAssignmentLifecycleSummary as Lifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyHumidityRatioAssignmentLifecycleSummary as PredecessorLifecycle,
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_snapshot_is_exact_direct_release,
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_humidity_ratio_assignment_snapshot_is_exact_direct_release,
    private_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_characterization,
};

use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

mod snapshot;
use snapshot::same_snapshot;

const EXPECTED_SOURCE_ORDER: &[&str] = &[
    "read-retained-maximum-total-cooling-capacity-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-latent-output-assignment",
    "assign-local-cooling-latent-output-from-maximum-total-cooling-capacity",
];

pub(in crate::ideal_loads) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let predecessor = output
        .calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_humidity_ratio_assignment;
    let snapshot = output
        .calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment;
    let expected = private_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_characterization(
        predecessor,
    );

    snapshot.system == binding.ideal_loads_air_system
        && snapshot.controlled_zone == binding.zone
        && snapshot.parent_call_ordinal == call_ordinal
        && snapshot.source
            == PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE
        && snapshot.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        && snapshot.source_order == EXPECTED_SOURCE_ORDER
        && binding.system.dehumidification_control_type == DehumidificationControlType::None
        && cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_humidity_ratio_assignment_snapshot_is_exact_direct_release(predecessor)
        && cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_snapshot_is_exact_direct_release(snapshot)
        && expected.is_some_and(|expected| same_snapshot(snapshot, expected))
}

pub(in crate::ideal_loads) fn validate_lifecycle(
    lifecycle: &Lifecycle,
    predecessor_cp404: &PredecessorLifecycle,
    timestep_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), Error> {
    let state = &lifecycle.state;
    let predecessor = &predecessor_cp404.state;

    if lifecycle.source
        != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || predecessor_cp404.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE
        || predecessor_cp404.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE_ORDER
            != EXPECTED_SOURCE_ORDER
        || [state.system, predecessor.system]
            .into_iter()
            .any(|system| system != binding.ideal_loads_air_system)
        || state.predecessor_route_counts != predecessor.predecessor_route_counts
        || state.predecessor_guard_false_fallthrough_route_counts
            != predecessor.predecessor_guard_false_fallthrough_route_counts
        || state.cooling_latent_output_maximum_capacity_assignment_route_counts
            != predecessor.supply_humidity_ratio_assignment_route_counts
        || binding.system.dehumidification_control_type != DehumidificationControlType::None
    {
        return Err(violation("source_predecessor_route_and_system_identity", 1, 0));
    }
    ensure_public_routes_only(&state.predecessor_route_counts)?;

    let route_sum = checked_sum(&state.predecessor_route_counts, "route_partition_overflow")?;
    let guard_false = checked_sum(
        &state.predecessor_guard_false_fallthrough_route_counts,
        "guard_false_partition_overflow",
    )?;
    let assignments = checked_sum(
        &state.cooling_latent_output_maximum_capacity_assignment_route_counts,
        "assignment_partition_overflow",
    )?;
    let inactive = checked_selected_sum(
        &state.predecessor_route_counts,
        &[
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 22, 23, 26, 28,
        ],
        "inactive_partition_overflow",
    )?;
    for index in 0..state.predecessor_route_counts.len() {
        let successor_sum = state.predecessor_guard_false_fallthrough_route_counts[index]
            .checked_add(
                state.cooling_latent_output_maximum_capacity_assignment_route_counts[index],
            )
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
    let humidity_owners = predecessor
        .cp403_supply_humidity_ratio_state_owner_count
        .checked_add(predecessor.supply_humidity_ratio_assignment_count)
        .ok_or_else(|| violation("supply_humidity_ratio_owner_count_overflow", 0, usize::MAX))?;
    let enthalpy_owners = predecessor.cp403_supply_enthalpy_state_owner_count;
    let temperature_owners = predecessor.cp403_supply_temperature_state_owner_count;

    for (field, expected, actual) in [
        ("transition_count", timestep_count, state.transition_count),
        (
            "predecessor_transition_count",
            predecessor.transition_count,
            state.transition_count,
        ),
        ("route_partition", state.transition_count, route_sum),
        (
            "transition_partition",
            state.transition_count,
            transition_partition,
        ),
        (
            "inactive_transition_count",
            inactive,
            state.inactive_transition_count,
        ),
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
            "cp404_guard_false_fallthrough_count",
            predecessor.predecessor_guard_false_fallthrough_count,
            guard_false,
        ),
        (
            "cooling_latent_output_maximum_capacity_assignment_count",
            assignments,
            state.cooling_latent_output_maximum_capacity_assignment_count,
        ),
        (
            "cp404_supply_humidity_ratio_assignment_count",
            predecessor.supply_humidity_ratio_assignment_count,
            assignments,
        ),
        (
            "source_site_execution_count",
            sites,
            state.source_site_execution_count,
        ),
        (
            "cp404_supply_humidity_ratio_state_owner_count",
            humidity_owners,
            state.cp404_supply_humidity_ratio_state_owner_count,
        ),
        (
            "unchanged_supply_humidity_ratio_preservation_count",
            humidity_owners,
            state.unchanged_supply_humidity_ratio_preservation_count,
        ),
        (
            "cp404_supply_enthalpy_state_owner_count",
            enthalpy_owners,
            state.cp404_supply_enthalpy_state_owner_count,
        ),
        (
            "unchanged_supply_enthalpy_preservation_count",
            enthalpy_owners,
            state.unchanged_supply_enthalpy_preservation_count,
        ),
        (
            "cp404_supply_temperature_state_owner_count",
            temperature_owners,
            state.cp404_supply_temperature_state_owner_count,
        ),
        (
            "unchanged_supply_temperature_preservation_count",
            temperature_owners,
            state.unchanged_supply_temperature_preservation_count,
        ),
        (
            "cp404_retained_maximum_total_cooling_capacity_owned_read_count",
            assignments,
            state.cp404_retained_maximum_total_cooling_capacity_owned_read_count,
        ),
        (
            "maximum_total_cooling_capacity_read_count",
            assignments,
            state.maximum_total_cooling_capacity_read_count,
        ),
        (
            "cooling_latent_output_assignment_write_count",
            assignments,
            state.cooling_latent_output_assignment_write_count,
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
    let expected = private_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_characterization(
        predecessor_latest,
    )
    .ok_or_else(|| violation("latest_predecessor_lineage_ready", 1, 0))?;
    let latest_route_count = if latest
        .dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed
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
            latest_output.calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment,
        )
        || !snapshot_matches_release(latest_output, timestep_count, binding)
        || latest_route_count == 0
    {
        return Err(violation("latest_release_snapshot_ready", 1, 0));
    }
    Ok(())
}

fn ensure_public_routes_only(values: &[usize; 30]) -> Result<(), Error> {
    for (index, count) in values.iter().enumerate() {
        if !matches!(index, 0..=8 | 20 | 24) && *count != 0 {
            return Err(violation("non_direct_route_count", 0, *count));
        }
    }
    Ok(())
}

const fn predecessor_index_is_active(index: usize) -> bool {
    matches!(index, 20 | 21 | 24 | 25 | 27 | 29)
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
    Error::CalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputMaximumCapacityAssignmentLifecycleInvariant {
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
