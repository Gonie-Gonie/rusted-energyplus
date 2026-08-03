//! Coupled-runtime validation for CP408 supply-temperature mixed-air-limit evidence.

use ep_model::DehumidificationControlType;

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_CHILD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER,
    PurchasedAirCalcCoolingMixedAirCallLifecycleSummary as MixedAirLifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureAssignmentLifecycleSummary as PredecessorLifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureMixedAirLimitLifecycleSummary as Lifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureMixedAirLimitSnapshot as Snapshot,
    cooling_mixed_air_call_snapshot_is_exact_direct_release,
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_snapshot_is_exact_direct_release,
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit_snapshot_is_exact_direct_release,
    private_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit_characterization,
};

use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

const EXPECTED_SOURCE_ORDER: &[&str] = &[
    "read-purchased-air-supply-temperature-for-minimum",
    "read-purchased-air-mixed-air-temperature-for-minimum",
    "apply-source-shaped-two-argument-minimum",
    "assign-purchased-air-supply-temperature",
];

pub(in crate::ideal_loads) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let predecessor = output
        .calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment;
    let mixed_air_owner = output.calculation_cooling_mixed_air_call;
    let snapshot = output
        .calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit;
    let active = predecessor
        .dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_executed;
    let expected = private_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit_characterization(
        predecessor,
        active.then_some(mixed_air_owner),
    );

    snapshot.system == binding.ideal_loads_air_system
        && snapshot.controlled_zone == binding.zone
        && snapshot.parent_call_ordinal == call_ordinal
        && binding.system.dehumidification_control_type == DehumidificationControlType::None
        && cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_snapshot_is_exact_direct_release(predecessor)
        && cooling_mixed_air_call_snapshot_is_exact_direct_release(mixed_air_owner)
        && cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit_snapshot_is_exact_direct_release(snapshot)
        && expected.is_some_and(|expected| same_snapshot(snapshot, expected))
}

pub(in crate::ideal_loads) fn validate_lifecycle(
    lifecycle: &Lifecycle,
    predecessor_cp407: &PredecessorLifecycle,
    mixed_air_owner_cp329: &MixedAirLifecycle,
    timestep_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), Error> {
    let state = &lifecycle.state;
    let predecessor = &predecessor_cp407.state;
    let mixed_air = &mixed_air_owner_cp329.state;

    if lifecycle.source
        != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE
        || predecessor_cp407.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE
        || predecessor_cp407.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || mixed_air_owner_cp329.source != PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE
        || mixed_air_owner_cp329.child_source != PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_CHILD_SOURCE
        || mixed_air_owner_cp329.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER
            != EXPECTED_SOURCE_ORDER
        || [state.system, predecessor.system, mixed_air.system]
            .into_iter()
            .any(|system| system != binding.ideal_loads_air_system)
        || binding.system.dehumidification_control_type != DehumidificationControlType::None
        || state.predecessor_route_counts != predecessor.predecessor_route_counts
        || state.predecessor_guard_false_fallthrough_route_counts
            != predecessor.predecessor_guard_false_fallthrough_route_counts
        || state.predecessor_maximum_capacity_assignment_route_counts
            != predecessor.predecessor_maximum_capacity_assignment_route_counts
        || state.predecessor_else_branch_entry_route_counts
            != predecessor.predecessor_else_branch_entry_route_counts
        || state.predecessor_supply_temperature_assignment_route_counts
            != predecessor.supply_temperature_assignment_route_counts
        || state.supply_temperature_mixed_air_limit_route_counts
            != predecessor.supply_temperature_assignment_route_counts
    {
        return Err(violation("source_owner_predecessor_route_and_system_identity", 1, 0));
    }
    ensure_public_routes_only(&state.predecessor_route_counts)?;

    let transitions = checked_sum(&state.predecessor_route_counts, "route_partition_overflow")?;
    let assignments = checked_sum(
        &state.supply_temperature_mixed_air_limit_route_counts,
        "mixed_air_limit_partition_overflow",
    )?;
    let expected_sites = assignments
        .checked_mul(EXPECTED_SOURCE_ORDER.len())
        .ok_or_else(|| violation("source_site_execution_count_overflow", 0, usize::MAX))?;
    let temperature_owners = selected_sum(
        &state.predecessor_route_counts,
        3..30,
        "temperature_owner_partition_overflow",
    )?;
    let unchanged_temperatures = temperature_owners.checked_sub(assignments).ok_or_else(|| {
        violation(
            "temperature_preservation_partition_underflow",
            assignments,
            temperature_owners,
        )
    })?;

    for (field, expected, actual) in [
        ("transition_count", timestep_count, state.transition_count),
        ("predecessor_transition_count", predecessor.transition_count, state.transition_count),
        ("mixed_air_owner_transition_count", mixed_air.transition_count, state.transition_count),
        ("route_partition", state.transition_count, transitions),
        ("inactive_transition_count", predecessor.inactive_transition_count, state.inactive_transition_count),
        ("transition_partition", state.transition_count, state.inactive_transition_count.checked_add(assignments).ok_or_else(|| violation("transition_partition_overflow", 0, usize::MAX))?),
        ("predecessor_guard_false_fallthrough_count", predecessor.predecessor_guard_false_fallthrough_count, state.predecessor_guard_false_fallthrough_count),
        ("predecessor_maximum_capacity_assignment_count", predecessor.predecessor_maximum_capacity_assignment_count, state.predecessor_maximum_capacity_assignment_count),
        ("predecessor_else_branch_entry_count", predecessor.predecessor_else_branch_entry_count, state.predecessor_else_branch_entry_count),
        ("predecessor_supply_temperature_assignment_count", predecessor.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_count, state.predecessor_supply_temperature_assignment_count),
        ("supply_temperature_mixed_air_limit_count", assignments, state.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit_count),
        ("cp407_assignment_equals_cp408_limit", predecessor.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_count, assignments),
        ("source_site_execution_count", expected_sites, state.source_site_execution_count),
        ("cp407_supply_temperature_state_owner_count", temperature_owners, state.cp407_supply_temperature_state_owner_count),
        ("predecessor_temperature_owner_count", predecessor.cp406_preexisting_supply_temperature_state_owner_count, state.cp407_supply_temperature_state_owner_count),
        ("unchanged_supply_humidity_ratio_preservation_count", predecessor.unchanged_supply_humidity_ratio_preservation_count, state.unchanged_supply_humidity_ratio_preservation_count),
        ("unchanged_supply_enthalpy_preservation_count", predecessor.unchanged_supply_enthalpy_preservation_count, state.unchanged_supply_enthalpy_preservation_count),
        ("unchanged_supply_temperature_preservation_count", unchanged_temperatures, state.unchanged_supply_temperature_preservation_count),
        ("cp407_retained_supply_temperature_owned_read_count", assignments, state.cp407_retained_supply_temperature_owned_read_count),
        ("supply_temperature_for_minimum_read_count", assignments, state.supply_temperature_for_minimum_read_count),
        ("cp329_retained_mixed_air_temperature_owned_read_count", assignments, state.cp329_retained_mixed_air_temperature_owned_read_count),
        ("mixed_air_temperature_for_minimum_read_count", assignments, state.mixed_air_temperature_for_minimum_read_count),
        ("source_shaped_two_argument_minimum_evaluation_count", assignments, state.source_shaped_two_argument_minimum_evaluation_count),
        ("supply_temperature_assignment_write_count", assignments, state.supply_temperature_assignment_write_count),
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
        .dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_executed;
    let mixed_air_latest = active.then_some(mixed_air.latest).flatten();
    let expected = private_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit_characterization(
        predecessor_latest,
        mixed_air_latest,
    )
    .ok_or_else(|| violation("latest_owner_lineage_ready", 1, 0))?;
    if !same_snapshot(latest, expected)
        || !same_snapshot(
            latest,
            latest_output.calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit,
        )
        || !snapshot_matches_release(latest_output, timestep_count, binding)
    {
        return Err(violation("latest_release_snapshot_ready", 1, 0));
    }
    Ok(())
}

fn same_snapshot(mut left: Snapshot, mut right: Snapshot) -> bool {
    macro_rules! compare_clear {
        ($field:ident) => {{
            let matches = option_bits_equal(left.$field, right.$field);
            left.$field = None;
            right.$field = None;
            matches
        }};
    }
    let values_match = compare_clear!(predecessor_cp406_resulting_supply_humidity_ratio)
        && compare_clear!(predecessor_cp406_resulting_supply_enthalpy_j_per_kg)
        && compare_clear!(predecessor_cp406_resulting_supply_temperature_c)
        && compare_clear!(predecessor_supply_enthalpy_j_per_kg)
        && compare_clear!(predecessor_supply_humidity_ratio)
        && compare_clear!(predecessor_preexisting_supply_temperature_c)
        && compare_clear!(predecessor_psychrometric_supply_temperature_result_c)
        && compare_clear!(predecessor_assigned_supply_temperature_c)
        && compare_clear!(predecessor_resulting_supply_humidity_ratio)
        && compare_clear!(predecessor_resulting_supply_enthalpy_j_per_kg)
        && compare_clear!(predecessor_resulting_supply_temperature_c)
        && compare_clear!(preexisting_supply_temperature_c)
        && compare_clear!(supply_temperature_before_mixed_air_limit_c)
        && compare_clear!(mixed_air_temperature_c)
        && compare_clear!(minimum_supply_temperature_c)
        && compare_clear!(assigned_supply_temperature_c)
        && compare_clear!(resulting_supply_humidity_ratio)
        && compare_clear!(resulting_supply_enthalpy_j_per_kg)
        && compare_clear!(resulting_supply_temperature_c);
    values_match && left == right
}

fn ensure_public_routes_only(values: &[usize; 30]) -> Result<(), Error> {
    for (index, count) in values.iter().enumerate() {
        if !matches!(index, 0..=8 | 20 | 24) && *count != 0 {
            return Err(violation("non_direct_route_count", 0, *count));
        }
    }
    Ok(())
}

fn selected_sum(
    values: &[usize; 30],
    indices: impl IntoIterator<Item = usize>,
    field: &'static str,
) -> Result<usize, Error> {
    indices.into_iter().try_fold(0usize, |sum, index| {
        sum.checked_add(values[index])
            .ok_or_else(|| violation(field, 0, usize::MAX))
    })
}

fn checked_sum(values: &[usize], field: &'static str) -> Result<usize, Error> {
    values.iter().try_fold(0usize, |sum, value| {
        sum.checked_add(*value)
            .ok_or_else(|| violation(field, 0, usize::MAX))
    })
}

fn option_bits_equal(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
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
    Error::CalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureMixedAirLimitLifecycleInvariant {
        field,
        expected,
        actual,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overflow_and_ieee_helpers_fail_closed() {
        assert!(checked_sum(&[usize::MAX, 1], "overflow").is_err());
        assert!(option_bits_equal(Some(-0.0), Some(-0.0)));
        assert!(!option_bits_equal(Some(-0.0), Some(0.0)));
    }
}
