//! Coupled-runtime validation for CP407 psychrometric supply-temperature evidence.

use ep_model::DehumidificationControlType;

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_CAPACITY_GUARD_ELSE_BRANCH_ENTRY_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_CAPACITY_GUARD_ELSE_BRANCH_ENTRY_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_SOURCE,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputCapacityGuardElseBranchEntryLifecycleSummary as PredecessorLifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureAssignmentLifecycleSummary as Lifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureAssignmentSnapshot as Snapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentLifecycleSummary as EnthalpyLifecycle,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentLifecycleSummary as HumidityLifecycle,
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry_snapshot_is_exact_direct_release,
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_snapshot_is_exact_direct_release,
    cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_snapshot_is_exact_direct_release,
    cooling_supply_humidity_ratio_saturation_limit_assignment_snapshot_is_exact_direct_release,
    private_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_characterization,
};

use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

const EXPECTED_SOURCE_ORDER: &[&str] = &[
    "read-cp385-retained-supply-enthalpy-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-latent-output-supply-temperature-dry-bulb-inversion",
    "read-cp378-retained-supply-humidity-ratio-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-latent-output-supply-temperature-dry-bulb-inversion",
    "evaluate-psy-tdb-fn-h-w-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-latent-output-supply-temperature",
    "assign-purchased-air-supply-temperature-after-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-latent-output-guard-else-branch",
];

pub(in crate::ideal_loads) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let predecessor = output
        .calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry;
    let humidity_owner =
        output.calculation_cooling_supply_humidity_ratio_saturation_limit_assignment;
    let enthalpy_owner = output
        .calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment;
    let snapshot = output
        .calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment;
    let active = predecessor
        .dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entered;
    let expected = private_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_characterization(
        predecessor,
        active.then_some(humidity_owner),
        active.then_some(enthalpy_owner),
    );

    snapshot.system == binding.ideal_loads_air_system
        && snapshot.controlled_zone == binding.zone
        && snapshot.parent_call_ordinal == call_ordinal
        && binding.system.dehumidification_control_type == DehumidificationControlType::None
        && cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry_snapshot_is_exact_direct_release(predecessor)
        && cooling_supply_humidity_ratio_saturation_limit_assignment_snapshot_is_exact_direct_release(humidity_owner)
        && cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_snapshot_is_exact_direct_release(enthalpy_owner)
        && cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_snapshot_is_exact_direct_release(snapshot)
        && expected.is_some_and(|expected| same_snapshot(snapshot, expected))
}

#[allow(clippy::too_many_arguments)]
pub(in crate::ideal_loads) fn validate_lifecycle(
    lifecycle: &Lifecycle,
    predecessor_cp406: &PredecessorLifecycle,
    humidity_cp378: &HumidityLifecycle,
    enthalpy_cp385: &EnthalpyLifecycle,
    timestep_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), Error> {
    let state = &lifecycle.state;
    let predecessor = &predecessor_cp406.state;
    let humidity = &humidity_cp378.state;
    let enthalpy = &enthalpy_cp385.state;

    if lifecycle.source != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE
        || lifecycle.first_excluded_source != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || predecessor_cp406.source != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_CAPACITY_GUARD_ELSE_BRANCH_ENTRY_SOURCE
        || predecessor_cp406.first_excluded_source != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_CAPACITY_GUARD_ELSE_BRANCH_ENTRY_FIRST_EXCLUDED_SOURCE
        || humidity_cp378.source != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_SOURCE
        || humidity_cp378.first_excluded_source != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || enthalpy_cp385.source != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE
        || enthalpy_cp385.first_excluded_source != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER != EXPECTED_SOURCE_ORDER
        || [state.system, predecessor.system, humidity.system, enthalpy.system]
            .into_iter()
            .any(|system| system != binding.ideal_loads_air_system)
        || binding.system.dehumidification_control_type != DehumidificationControlType::None
        || state.predecessor_route_counts != predecessor.predecessor_route_counts
        || state.predecessor_guard_false_fallthrough_route_counts != predecessor.predecessor_guard_false_fallthrough_route_counts
        || state.predecessor_maximum_capacity_assignment_route_counts != predecessor.predecessor_maximum_capacity_assignment_route_counts
        || state.predecessor_else_branch_entry_route_counts != predecessor.else_branch_entry_route_counts
        || state.supply_temperature_assignment_route_counts != predecessor.else_branch_entry_route_counts
    {
        return Err(violation("source_owner_predecessor_route_and_system_identity", 1, 0));
    }
    ensure_public_routes_only(&state.predecessor_route_counts)?;

    let transitions = checked_sum(&state.predecessor_route_counts, "route_partition_overflow")?;
    let assignments = checked_sum(
        &state.supply_temperature_assignment_route_counts,
        "assignment_partition_overflow",
    )?;
    let expected_sites = assignments
        .checked_mul(EXPECTED_SOURCE_ORDER.len())
        .ok_or_else(|| violation("source_site_execution_count_overflow", 0, usize::MAX))?;
    let temperature_owners = selected_sum(
        &state.predecessor_route_counts,
        3..30,
        "temperature_owner_partition_overflow",
    )?;
    let enthalpy_preservations = selected_sum(
        &state.predecessor_route_counts,
        [5, 8, 11, 14].into_iter().chain(17..30),
        "enthalpy_preservation_partition_overflow",
    )?;
    let humidity_preservations = selected_sum(
        &state.predecessor_route_counts,
        [18, 19, 22, 23, 26, 28],
        "humidity_preservation_partition_overflow",
    )?
    .checked_add(state.predecessor_maximum_capacity_assignment_count)
    .and_then(|count| count.checked_add(assignments))
    .ok_or_else(|| violation("humidity_preservation_partition_overflow", 0, usize::MAX))?;
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
        ("humidity_owner_transition_count", humidity.transition_count, state.transition_count),
        ("enthalpy_owner_transition_count", enthalpy.transition_count, state.transition_count),
        ("route_partition", state.transition_count, transitions),
        ("inactive_transition_count", predecessor.inactive_transition_count, state.inactive_transition_count),
        ("transition_partition", state.transition_count, state.inactive_transition_count.checked_add(assignments).ok_or_else(|| violation("transition_partition_overflow", 0, usize::MAX))?),
        ("predecessor_guard_false_fallthrough_count", predecessor.predecessor_guard_false_fallthrough_count, state.predecessor_guard_false_fallthrough_count),
        ("predecessor_maximum_capacity_assignment_count", predecessor.predecessor_maximum_capacity_assignment_count, state.predecessor_maximum_capacity_assignment_count),
        ("predecessor_else_branch_entry_count", predecessor.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry_count, state.predecessor_else_branch_entry_count),
        ("supply_temperature_assignment_count", assignments, state.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_count),
        ("cp406_else_equals_assignment", predecessor.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry_count, assignments),
        ("source_site_execution_count", expected_sites, state.source_site_execution_count),
        ("cp385_retained_supply_enthalpy_owned_read_count", assignments, state.cp385_retained_supply_enthalpy_owned_read_count),
        ("cp406_same_call_supply_enthalpy_bit_corroboration_count", assignments, state.cp406_same_call_supply_enthalpy_bit_corroboration_count),
        ("supply_enthalpy_for_dry_bulb_inversion_read_count", assignments, state.supply_enthalpy_for_dry_bulb_inversion_read_count),
        ("cp378_retained_supply_humidity_ratio_owned_read_count", assignments, state.cp378_retained_supply_humidity_ratio_owned_read_count),
        ("supply_humidity_ratio_for_dry_bulb_inversion_read_count", assignments, state.supply_humidity_ratio_for_dry_bulb_inversion_read_count),
        ("psychrometric_supply_temperature_evaluation_count", assignments, state.psychrometric_supply_temperature_evaluation_count),
        ("supply_temperature_assignment_write_count", assignments, state.supply_temperature_assignment_write_count),
        ("cp406_preexisting_supply_temperature_state_owner_count", temperature_owners, state.cp406_preexisting_supply_temperature_state_owner_count),
        ("unchanged_supply_humidity_ratio_preservation_count", humidity_preservations, state.unchanged_supply_humidity_ratio_preservation_count),
        ("unchanged_supply_enthalpy_preservation_count", enthalpy_preservations, state.unchanged_supply_enthalpy_preservation_count),
        ("unchanged_supply_temperature_preservation_count", unchanged_temperatures, state.unchanged_supply_temperature_preservation_count),
    ] {
        ensure_count(actual, expected, field)?;
    }

    let latest = state
        .latest
        .ok_or_else(|| violation("latest_release_snapshot_ready", 1, 0))?;
    let predecessor_latest = predecessor
        .latest
        .ok_or_else(|| violation("predecessor_latest_release_snapshot_ready", 1, 0))?;
    let active = predecessor_latest.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entered;
    let humidity_latest = active.then_some(humidity.latest).flatten();
    let enthalpy_latest = active.then_some(enthalpy.latest).flatten();
    let expected = private_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_characterization(
        predecessor_latest,
        humidity_latest,
        enthalpy_latest,
    )
    .ok_or_else(|| violation("latest_owner_lineage_ready", 1, 0))?;
    if !same_snapshot(latest, expected)
        || !same_snapshot(
            latest,
            latest_output.calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment,
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
        && compare_clear!(supply_enthalpy_j_per_kg)
        && compare_clear!(supply_humidity_ratio)
        && compare_clear!(preexisting_supply_temperature_c)
        && compare_clear!(psychrometric_supply_temperature_result_c)
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
    Error::CalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureAssignmentLifecycleInvariant {
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
