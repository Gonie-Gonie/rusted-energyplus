//! Coupled-runtime validation for CP400 shared sensible-output evidence.

use ep_model::DehumidificationControlType;

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_CP_AIR_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingMixedAirCallLifecycleSummary as MixedAirLifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCpAirAssignmentLifecycleSummary as PredecessorLifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioSensibleOutputAssignmentLifecycleSummary as Lifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioSensibleOutputAssignmentSnapshot as Snapshot,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardLifecycleSummary as SupplyFlowLifecycle,
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment_snapshot_is_exact_direct_release,
    private_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment_characterization,
};

use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

pub(in crate::ideal_loads) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let predecessor = output
        .calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment;
    let active = predecessor
        .dehumidification_control_none_or_constant_supply_humidity_ratio_cp_air_assignment_executed;
    let expected = private_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment_characterization(
        predecessor,
        active.then_some(output.calculation_cooling_mixed_air_call),
        active.then_some(output.calculation_cooling_supply_mass_flow_positive_guard),
    );
    let snapshot = output
        .calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment;

    expected.is_some_and(|expected| {
        snapshot.system == binding.ideal_loads_air_system
            && snapshot.parent_call_ordinal == call_ordinal
            && snapshot.controlled_zone == binding.zone
            && binding.system.dehumidification_control_type == DehumidificationControlType::None
            && cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment_snapshot_is_exact_direct_release(snapshot)
            && same_snapshot(snapshot, expected)
    })
}

#[allow(clippy::too_many_arguments)]
pub(in crate::ideal_loads) fn validate_lifecycle(
    lifecycle: &Lifecycle,
    predecessor_cp399: &PredecessorLifecycle,
    supply_flow_cp330: &SupplyFlowLifecycle,
    mixed_air_cp329: &MixedAirLifecycle,
    timestep_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), Error> {
    let state = &lifecycle.state;
    let predecessor = &predecessor_cp399.state;
    let supply_flow = &supply_flow_cp330.state;
    let mixed_air = &mixed_air_cp329.state;
    let assignments = state
        .dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment_count;
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
            PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER.len(),
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
        != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || predecessor_cp399.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_CP_AIR_ASSIGNMENT_SOURCE
        || predecessor_cp399.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER.len()
            != 8
        || state.system != binding.ideal_loads_air_system
        || predecessor.system != binding.ideal_loads_air_system
        || supply_flow.system != binding.ideal_loads_air_system
        || mixed_air.system != binding.ideal_loads_air_system
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
            "supply_flow_transition_count",
            supply_flow.transition_count,
            state.transition_count,
        ),
        (
            "mixed_air_transition_count",
            mixed_air.transition_count,
            state.transition_count,
        ),
        (
            "predecessor_route_partition",
            state.transition_count,
            route_sum,
        ),
        (
            "inactive_transition_count",
            inactive,
            state.inactive_transition_count,
        ),
        ("active_route_count", expected_assignments, assignments),
        (
            "predecessor_cp_air_assignment_count",
            predecessor
                .dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment_count,
            assignments,
        ),
        (
            "source_site_execution_count",
            expected_sites,
            state.source_site_execution_count,
        ),
        (
            "cp399_supply_humidity_ratio_state_owner_count",
            expected_humidity_owners,
            state.cp399_supply_humidity_ratio_state_owner_count,
        ),
        (
            "unchanged_supply_humidity_ratio_preservation_count",
            expected_humidity_owners,
            state.unchanged_supply_humidity_ratio_preservation_count,
        ),
        (
            "cp399_supply_enthalpy_state_owner_count",
            expected_enthalpy_owners,
            state.cp399_supply_enthalpy_state_owner_count,
        ),
        (
            "unchanged_supply_enthalpy_preservation_count",
            expected_enthalpy_owners,
            state.unchanged_supply_enthalpy_preservation_count,
        ),
        (
            "cp399_supply_temperature_state_owner_count",
            expected_temperature_owners,
            state.cp399_supply_temperature_state_owner_count,
        ),
        (
            "unchanged_supply_temperature_preservation_count",
            expected_temperature_owners,
            state.unchanged_supply_temperature_preservation_count,
        ),
        (
            "supply_mass_flow_rate_owned_read_count",
            assignments,
            state.supply_mass_flow_rate_owned_read_count,
        ),
        (
            "supply_mass_flow_rate_bit_corroboration_count",
            assignments,
            state.supply_mass_flow_rate_bit_corroboration_count,
        ),
        (
            "supply_mass_flow_rate_read_count",
            assignments,
            state.supply_mass_flow_rate_read_count,
        ),
        (
            "cp_air_owned_read_count",
            assignments,
            state.cp_air_owned_read_count,
        ),
        ("cp_air_read_count", assignments, state.cp_air_read_count),
        (
            "supply_mass_flow_rate_times_cp_air_calculation_count",
            assignments,
            state.supply_mass_flow_rate_times_cp_air_calculation_count,
        ),
        (
            "mixed_air_temperature_owned_read_count",
            assignments,
            state.mixed_air_temperature_owned_read_count,
        ),
        (
            "mixed_air_temperature_read_count",
            assignments,
            state.mixed_air_temperature_read_count,
        ),
        (
            "supply_temperature_owned_read_count",
            assignments,
            state.supply_temperature_owned_read_count,
        ),
        (
            "supply_temperature_read_count",
            assignments,
            state.supply_temperature_read_count,
        ),
        (
            "mixed_air_minus_supply_temperature_calculation_count",
            assignments,
            state.mixed_air_minus_supply_temperature_calculation_count,
        ),
        (
            "cooling_sensible_output_calculation_count",
            assignments,
            state.cooling_sensible_output_calculation_count,
        ),
        (
            "cooling_sensible_output_assignment_write_count",
            assignments,
            state.cooling_sensible_output_assignment_write_count,
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
        .dehumidification_control_none_or_constant_supply_humidity_ratio_cp_air_assignment_executed;
    let mixed_air_latest = if active {
        Some(
            mixed_air
                .latest
                .ok_or_else(|| violation("active_mixed_air_owner_ready", 1, 0))?,
        )
    } else {
        None
    };
    let supply_flow_latest = if active {
        Some(
            supply_flow
                .latest
                .ok_or_else(|| violation("active_supply_flow_owner_ready", 1, 0))?,
        )
    } else {
        None
    };
    let expected = private_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment_characterization(
        predecessor_latest,
        mixed_air_latest,
        supply_flow_latest,
    )
    .ok_or_else(|| violation("active_operand_owners_ready", 1, 0))?;

    if !same_snapshot(latest, expected)
        || !same_snapshot(
            latest,
            latest_output.calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment,
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
            left.supply_mass_flow_rate_kg_per_s,
            right.supply_mass_flow_rate_kg_per_s,
        ),
        (left.cp_air_j_per_kg_k, right.cp_air_j_per_kg_k),
        (
            left.supply_mass_flow_rate_times_cp_air_w_per_k,
            right.supply_mass_flow_rate_times_cp_air_w_per_k,
        ),
        (left.mixed_air_temperature_c, right.mixed_air_temperature_c),
        (left.supply_temperature_c, right.supply_temperature_c),
        (
            left.mixed_air_minus_supply_temperature_k,
            right.mixed_air_minus_supply_temperature_k,
        ),
        (
            left.calculated_cooling_sensible_output_w,
            right.calculated_cooling_sensible_output_w,
        ),
        (
            left.cooling_sensible_output_w,
            right.cooling_sensible_output_w,
        ),
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
        snapshot.supply_mass_flow_rate_kg_per_s = None;
        snapshot.cp_air_j_per_kg_k = None;
        snapshot.supply_mass_flow_rate_times_cp_air_w_per_k = None;
        snapshot.mixed_air_temperature_c = None;
        snapshot.supply_temperature_c = None;
        snapshot.mixed_air_minus_supply_temperature_k = None;
        snapshot.calculated_cooling_sensible_output_w = None;
        snapshot.cooling_sensible_output_w = None;
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
    Error::CalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioSensibleOutputAssignmentLifecycleInvariant {
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
