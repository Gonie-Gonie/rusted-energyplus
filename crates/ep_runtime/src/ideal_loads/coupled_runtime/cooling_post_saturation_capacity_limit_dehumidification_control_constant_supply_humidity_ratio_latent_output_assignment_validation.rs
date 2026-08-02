//! Coupled-runtime validation for CP401 shared latent-output evidence.

use ep_model::DehumidificationControlType;

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_ASSIGNMENT_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputAssignmentLifecycleSummary as Lifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputAssignmentSnapshot as Snapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioSensibleOutputAssignmentLifecycleSummary as PredecessorLifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputMaximumCapacityAssignmentLifecycleSummary as CoolingTotalOutputOwnerLifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentLifecycleSummary as CoolingTotalOutputCorroboratorLifecycle,
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment_snapshot_is_exact_direct_release,
    private_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment_characterization,
};

use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

pub(in crate::ideal_loads) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let predecessor = output
        .calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment;
    let active = predecessor
        .dehumidification_control_none_or_constant_supply_humidity_ratio_sensible_output_assignment_executed;
    let expected = private_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment_characterization(
        predecessor,
        active.then_some(
            output.calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment,
        ),
        active.then_some(
            output.calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment,
        ),
    );
    let snapshot = output
        .calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment;

    expected.is_some_and(|expected| {
        snapshot.system == binding.ideal_loads_air_system
            && snapshot.parent_call_ordinal == call_ordinal
            && snapshot.controlled_zone == binding.zone
            && binding.system.dehumidification_control_type == DehumidificationControlType::None
            && cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment_snapshot_is_exact_direct_release(snapshot)
            && same_snapshot(snapshot, expected)
    })
}

#[allow(clippy::too_many_arguments)]
pub(in crate::ideal_loads) fn validate_lifecycle(
    lifecycle: &Lifecycle,
    predecessor_cp400: &PredecessorLifecycle,
    cooling_total_output_owner_cp384: &CoolingTotalOutputOwnerLifecycle,
    cooling_total_output_corroborator_cp385: &CoolingTotalOutputCorroboratorLifecycle,
    timestep_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), Error> {
    let state = &lifecycle.state;
    let predecessor = &predecessor_cp400.state;
    let owner = &cooling_total_output_owner_cp384.state;
    let corroborator = &cooling_total_output_corroborator_cp385.state;
    let assignments = state
        .dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment_count;
    let inactive = state
        .transition_count
        .checked_sub(assignments)
        .ok_or_else(|| {
            violation(
                "inactive_transition_underflow",
                assignments,
                state.transition_count,
            )
        })?;
    let route_sum = checked_sum(
        &state.predecessor_route_counts,
        "predecessor_route_partition_overflow",
    )?;
    let expected_assignments = checked_sum(
        &[
            state.predecessor_route_counts[20],
            state.predecessor_route_counts[21],
            state.predecessor_route_counts[24],
            state.predecessor_route_counts[25],
            state.predecessor_route_counts[27],
            state.predecessor_route_counts[29],
        ],
        "active_route_count_overflow",
    )?;
    let expected_sites = assignments
        .checked_mul(
            PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_ASSIGNMENT_SOURCE_ORDER.len(),
        )
        .ok_or_else(|| violation("source_site_execution_count_overflow", 0, usize::MAX))?;
    let expected_humidity_owners = checked_sum(
        &[
            state.predecessor_route_counts[18],
            state.predecessor_route_counts[19],
            state.predecessor_route_counts[22],
            state.predecessor_route_counts[23],
            state.predecessor_route_counts[26],
            state.predecessor_route_counts[28],
        ],
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

    if lifecycle.source
        != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_ASSIGNMENT_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || predecessor_cp400.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE
        || predecessor_cp400.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || cooling_total_output_owner_cp384.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE
        || cooling_total_output_owner_cp384.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || cooling_total_output_corroborator_cp385.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE
        || cooling_total_output_corroborator_cp385.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_ASSIGNMENT_SOURCE_ORDER.len()
            != 4
        || state.system != binding.ideal_loads_air_system
        || predecessor.system != binding.ideal_loads_air_system
        || owner.system != binding.ideal_loads_air_system
        || corroborator.system != binding.ideal_loads_air_system
        || state.predecessor_route_counts != predecessor.predecessor_route_counts
        || binding.system.dehumidification_control_type != DehumidificationControlType::None
    {
        return Err(violation("source_owner_route_and_system_identity", 1, 0));
    }
    ensure_public_routes_only(&state.predecessor_route_counts)?;

    for (field, expected, actual) in [
        ("transition_count", timestep_count, state.transition_count),
        (
            "predecessor_transition_count",
            predecessor.transition_count,
            state.transition_count,
        ),
        (
            "cooling_total_output_owner_transition_count",
            owner.transition_count,
            state.transition_count,
        ),
        (
            "cooling_total_output_corroborator_transition_count",
            corroborator.transition_count,
            state.transition_count,
        ),
        ("predecessor_route_partition", state.transition_count, route_sum),
        ("inactive_transition_count", inactive, state.inactive_transition_count),
        ("active_route_count", expected_assignments, assignments),
        (
            "predecessor_sensible_output_assignment_count",
            predecessor
                .dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment_count,
            assignments,
        ),
        ("source_site_execution_count", expected_sites, state.source_site_execution_count),
        (
            "cp400_supply_humidity_ratio_state_owner_count",
            expected_humidity_owners,
            state.cp400_supply_humidity_ratio_state_owner_count,
        ),
        (
            "unchanged_supply_humidity_ratio_preservation_count",
            expected_humidity_owners,
            state.unchanged_supply_humidity_ratio_preservation_count,
        ),
        (
            "cp400_supply_enthalpy_state_owner_count",
            expected_enthalpy_owners,
            state.cp400_supply_enthalpy_state_owner_count,
        ),
        (
            "unchanged_supply_enthalpy_preservation_count",
            expected_enthalpy_owners,
            state.unchanged_supply_enthalpy_preservation_count,
        ),
        (
            "cp400_supply_temperature_state_owner_count",
            expected_temperature_owners,
            state.cp400_supply_temperature_state_owner_count,
        ),
        (
            "unchanged_supply_temperature_preservation_count",
            expected_temperature_owners,
            state.unchanged_supply_temperature_preservation_count,
        ),
        (
            "cooling_total_output_owned_read_count",
            assignments,
            state.cooling_total_output_owned_read_count,
        ),
        (
            "cooling_total_output_bit_corroboration_count",
            assignments,
            state.cooling_total_output_bit_corroboration_count,
        ),
        (
            "cooling_total_output_read_count",
            assignments,
            state.cooling_total_output_read_count,
        ),
        (
            "cooling_sensible_output_owned_read_count",
            assignments,
            state.cooling_sensible_output_owned_read_count,
        ),
        (
            "cooling_sensible_output_read_count",
            assignments,
            state.cooling_sensible_output_read_count,
        ),
        (
            "cooling_latent_output_calculation_count",
            assignments,
            state.cooling_latent_output_calculation_count,
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
    let active = predecessor_latest
        .dehumidification_control_none_or_constant_supply_humidity_ratio_sensible_output_assignment_executed;
    let owner_latest = if active {
        Some(
            owner
                .latest
                .ok_or_else(|| violation("active_cp384_owner_ready", 1, 0))?,
        )
    } else {
        None
    };
    let corroborator_latest = if active {
        Some(
            corroborator
                .latest
                .ok_or_else(|| violation("active_cp385_corroborator_ready", 1, 0))?,
        )
    } else {
        None
    };
    let expected = private_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment_characterization(
        predecessor_latest,
        owner_latest,
        corroborator_latest,
    )
    .ok_or_else(|| violation("active_operand_owners_ready", 1, 0))?;

    if !same_snapshot(latest, expected)
        || !same_snapshot(
            latest,
            latest_output.calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment,
        )
        || !snapshot_matches_release(latest_output, timestep_count, binding)
    {
        return Err(violation("latest_release_snapshot_ready", 1, 0));
    }
    Ok(())
}

fn ensure_public_routes_only(routes: &[usize; 30]) -> Result<(), Error> {
    for (index, count) in routes.iter().enumerate() {
        if !matches!(index, 0..=8 | 20 | 24) && *count != 0 {
            return Err(violation("non_direct_route_count", 0, *count));
        }
    }
    Ok(())
}

fn same_snapshot(mut left: Snapshot, mut right: Snapshot) -> bool {
    let values_match = [
        (
            left.predecessor_cp397_resulting_supply_humidity_ratio,
            right.predecessor_cp397_resulting_supply_humidity_ratio,
        ),
        (
            left.predecessor_cp397_resulting_supply_enthalpy_j_per_kg,
            right.predecessor_cp397_resulting_supply_enthalpy_j_per_kg,
        ),
        (
            left.predecessor_cp397_resulting_supply_temperature_c,
            right.predecessor_cp397_resulting_supply_temperature_c,
        ),
        (
            left.predecessor_cp398_resulting_supply_humidity_ratio,
            right.predecessor_cp398_resulting_supply_humidity_ratio,
        ),
        (
            left.predecessor_cp398_resulting_supply_enthalpy_j_per_kg,
            right.predecessor_cp398_resulting_supply_enthalpy_j_per_kg,
        ),
        (
            left.predecessor_cp398_resulting_supply_temperature_c,
            right.predecessor_cp398_resulting_supply_temperature_c,
        ),
        (
            left.predecessor_mixed_air_humidity_ratio,
            right.predecessor_mixed_air_humidity_ratio,
        ),
        (
            left.predecessor_psychrometric_cp_air_result_j_per_kg_k,
            right.predecessor_psychrometric_cp_air_result_j_per_kg_k,
        ),
        (
            left.predecessor_cp_air_j_per_kg_k,
            right.predecessor_cp_air_j_per_kg_k,
        ),
        (
            left.predecessor_cp399_resulting_supply_humidity_ratio,
            right.predecessor_cp399_resulting_supply_humidity_ratio,
        ),
        (
            left.predecessor_cp399_resulting_supply_enthalpy_j_per_kg,
            right.predecessor_cp399_resulting_supply_enthalpy_j_per_kg,
        ),
        (
            left.predecessor_cp399_resulting_supply_temperature_c,
            right.predecessor_cp399_resulting_supply_temperature_c,
        ),
        (
            left.predecessor_supply_mass_flow_rate_kg_per_s,
            right.predecessor_supply_mass_flow_rate_kg_per_s,
        ),
        (
            left.predecessor_cp400_cp_air_j_per_kg_k,
            right.predecessor_cp400_cp_air_j_per_kg_k,
        ),
        (
            left.predecessor_supply_mass_flow_rate_times_cp_air_w_per_k,
            right.predecessor_supply_mass_flow_rate_times_cp_air_w_per_k,
        ),
        (
            left.predecessor_mixed_air_temperature_c,
            right.predecessor_mixed_air_temperature_c,
        ),
        (
            left.predecessor_supply_temperature_c,
            right.predecessor_supply_temperature_c,
        ),
        (
            left.predecessor_mixed_air_minus_supply_temperature_k,
            right.predecessor_mixed_air_minus_supply_temperature_k,
        ),
        (
            left.predecessor_calculated_cooling_sensible_output_w,
            right.predecessor_calculated_cooling_sensible_output_w,
        ),
        (
            left.predecessor_cooling_sensible_output_w,
            right.predecessor_cooling_sensible_output_w,
        ),
        (
            left.predecessor_cp400_resulting_supply_humidity_ratio,
            right.predecessor_cp400_resulting_supply_humidity_ratio,
        ),
        (
            left.predecessor_cp400_resulting_supply_enthalpy_j_per_kg,
            right.predecessor_cp400_resulting_supply_enthalpy_j_per_kg,
        ),
        (
            left.predecessor_cp400_resulting_supply_temperature_c,
            right.predecessor_cp400_resulting_supply_temperature_c,
        ),
        (left.cooling_total_output_w, right.cooling_total_output_w),
        (
            left.cooling_sensible_output_w,
            right.cooling_sensible_output_w,
        ),
        (
            left.calculated_cooling_latent_output_w,
            right.calculated_cooling_latent_output_w,
        ),
        (left.cooling_latent_output_w, right.cooling_latent_output_w),
        (
            left.resulting_supply_humidity_ratio,
            right.resulting_supply_humidity_ratio,
        ),
        (
            left.resulting_supply_enthalpy_j_per_kg,
            right.resulting_supply_enthalpy_j_per_kg,
        ),
        (
            left.resulting_supply_temperature_c,
            right.resulting_supply_temperature_c,
        ),
    ]
    .into_iter()
    .all(|(left, right)| options_have_exact_bits(left, right));
    for snapshot in [&mut left, &mut right] {
        snapshot.predecessor_cp397_resulting_supply_humidity_ratio = None;
        snapshot.predecessor_cp397_resulting_supply_enthalpy_j_per_kg = None;
        snapshot.predecessor_cp397_resulting_supply_temperature_c = None;
        snapshot.predecessor_cp398_resulting_supply_humidity_ratio = None;
        snapshot.predecessor_cp398_resulting_supply_enthalpy_j_per_kg = None;
        snapshot.predecessor_cp398_resulting_supply_temperature_c = None;
        snapshot.predecessor_mixed_air_humidity_ratio = None;
        snapshot.predecessor_psychrometric_cp_air_result_j_per_kg_k = None;
        snapshot.predecessor_cp_air_j_per_kg_k = None;
        snapshot.predecessor_cp399_resulting_supply_humidity_ratio = None;
        snapshot.predecessor_cp399_resulting_supply_enthalpy_j_per_kg = None;
        snapshot.predecessor_cp399_resulting_supply_temperature_c = None;
        snapshot.predecessor_supply_mass_flow_rate_kg_per_s = None;
        snapshot.predecessor_cp400_cp_air_j_per_kg_k = None;
        snapshot.predecessor_supply_mass_flow_rate_times_cp_air_w_per_k = None;
        snapshot.predecessor_mixed_air_temperature_c = None;
        snapshot.predecessor_supply_temperature_c = None;
        snapshot.predecessor_mixed_air_minus_supply_temperature_k = None;
        snapshot.predecessor_calculated_cooling_sensible_output_w = None;
        snapshot.predecessor_cooling_sensible_output_w = None;
        snapshot.predecessor_cp400_resulting_supply_humidity_ratio = None;
        snapshot.predecessor_cp400_resulting_supply_enthalpy_j_per_kg = None;
        snapshot.predecessor_cp400_resulting_supply_temperature_c = None;
        snapshot.cooling_total_output_w = None;
        snapshot.cooling_sensible_output_w = None;
        snapshot.calculated_cooling_latent_output_w = None;
        snapshot.cooling_latent_output_w = None;
        snapshot.resulting_supply_humidity_ratio = None;
        snapshot.resulting_supply_enthalpy_j_per_kg = None;
        snapshot.resulting_supply_temperature_c = None;
    }
    values_match && left == right
}

fn options_have_exact_bits(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
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
    Error::CalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputAssignmentLifecycleInvariant {
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

    #[test]
    fn bit_comparison_distinguishes_signed_zero() {
        assert!(options_have_exact_bits(Some(-0.0), Some(-0.0)));
        assert!(!options_have_exact_bits(Some(-0.0), Some(0.0)));
    }
}
