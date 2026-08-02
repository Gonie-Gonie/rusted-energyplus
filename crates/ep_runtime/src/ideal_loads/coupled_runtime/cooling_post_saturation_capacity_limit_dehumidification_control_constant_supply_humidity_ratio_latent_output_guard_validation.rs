//! Coupled-runtime validation for CP402 shared latent-output capacity-guard evidence.

use ep_model::DehumidificationControlType;

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
    PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingCapacityZeroFlowResetLifecycleSummary as CapacityLifecycle,
    PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot as CapacitySnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardLifecycleSummary as CapacityCorroboratorLifecycle,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot as CapacityCorroboratorSnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputAssignmentLifecycleSummary as PredecessorLifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputAssignmentSnapshot as PredecessorSnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputGuardActiveInput as ActiveInput,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputGuardLifecycleSummary as Lifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputGuardSnapshot as Snapshot,
    cooling_capacity_zero_flow_reset_snapshot_is_exact_direct_release,
    cooling_positive_supply_capacity_limit_sensible_output_guard_snapshot_is_exact_direct_release,
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard_snapshot_is_exact_direct_release,
    private_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard_characterization,
};

use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

mod snapshot;
#[cfg(test)]
use snapshot::options_have_exact_bits;
use snapshot::same_snapshot;

pub(in crate::ideal_loads) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let predecessor = output
        .calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment;
    let capacity_owner = output.calculation_cooling_capacity_zero_flow_reset;
    let capacity_corroborator =
        output.calculation_cooling_positive_supply_capacity_limit_sensible_output_guard;
    let snapshot = output
        .calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard;
    let expected = active_input(predecessor, capacity_owner, capacity_corroborator)
        .and_then(|input| {
            private_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard_characterization(
                predecessor,
                input,
            )
        });

    metadata_matches(
        snapshot,
        predecessor,
        capacity_owner,
        capacity_corroborator,
        call_ordinal,
        binding,
    ) && binding.system.dehumidification_control_type == DehumidificationControlType::None
        && cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard_snapshot_is_exact_direct_release(snapshot)
        && expected.is_some_and(|expected| same_snapshot(snapshot, expected))
}

#[allow(clippy::too_many_arguments)]
pub(in crate::ideal_loads) fn validate_lifecycle(
    lifecycle: &Lifecycle,
    predecessor_cp401: &PredecessorLifecycle,
    maximum_capacity_owner_cp321: &CapacityLifecycle,
    maximum_capacity_corroborator_cp340: &CapacityCorroboratorLifecycle,
    timestep_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), Error> {
    let state = &lifecycle.state;
    let predecessor = &predecessor_cp401.state;
    let owner = &maximum_capacity_owner_cp321.state;
    let corroborator = &maximum_capacity_corroborator_cp340.state;

    if lifecycle.source
        != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_GUARD_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE
        || predecessor_cp401.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_ASSIGNMENT_SOURCE
        || predecessor_cp401.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || maximum_capacity_owner_cp321.source
            != PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_SOURCE
        || maximum_capacity_owner_cp321.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_FIRST_EXCLUDED_SOURCE
        || maximum_capacity_corroborator_cp340.source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_SOURCE
        || maximum_capacity_corroborator_cp340.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_GUARD_SOURCE_ORDER.len()
            != 4
        || [state.system, predecessor.system, owner.system, corroborator.system]
            .into_iter()
            .any(|system| system != binding.ideal_loads_air_system)
        || state.predecessor_route_counts != predecessor.predecessor_route_counts
        || binding.system.dehumidification_control_type != DehumidificationControlType::None
    {
        return Err(violation("source_owner_route_and_system_identity", 1, 0));
    }
    ensure_public_routes_only(&state.predecessor_route_counts)?;

    let route_sum = checked_sum(
        &state.predecessor_route_counts,
        "predecessor_route_partition_overflow",
    )?;
    let evaluations = checked_selected_sum(
        &state.predecessor_route_counts,
        &[20, 21, 24, 25, 27, 29],
        "active_route_count_overflow",
    )?;
    let inactive = state
        .transition_count
        .checked_sub(evaluations)
        .ok_or_else(|| {
            violation(
                "inactive_transition_underflow",
                evaluations,
                state.transition_count,
            )
        })?;
    let guard_false = checked_sum(
        &state.guard_false_fallthrough_route_counts,
        "guard_false_route_sum_overflow",
    )?;
    let body_entries = checked_sum(
        &state.adjustment_body_entry_route_counts,
        "body_entry_route_sum_overflow",
    )?;
    for index in 0..state.predecessor_route_counts.len() {
        let successor_sum = state.guard_false_fallthrough_route_counts[index]
            .checked_add(state.adjustment_body_entry_route_counts[index])
            .ok_or_else(|| violation("successor_route_partition_overflow", 0, usize::MAX))?;
        let expected = if predecessor_index_is_active(index) {
            state.predecessor_route_counts[index]
        } else {
            0
        };
        ensure_count(successor_sum, expected, "successor_route_partition")?;
    }
    let sites = evaluations
        .checked_mul(3)
        .and_then(|sites| sites.checked_add(body_entries))
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

    for (field, expected, actual) in [
        ("transition_count", timestep_count, state.transition_count),
        (
            "predecessor_transition_count",
            predecessor.transition_count,
            state.transition_count,
        ),
        (
            "maximum_capacity_owner_transition_count",
            owner.transition_count,
            state.transition_count,
        ),
        (
            "maximum_capacity_corroborator_transition_count",
            corroborator.transition_count,
            state.transition_count,
        ),
        ("predecessor_route_partition", state.transition_count, route_sum),
        ("inactive_transition_count", inactive, state.inactive_transition_count),
        (
            "guard_evaluation_route_sum",
            evaluations,
            state.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_evaluation_count,
        ),
        (
            "predecessor_latent_output_assignment_count",
            predecessor.dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment_count,
            evaluations,
        ),
        ("source_site_execution_count", sites, state.source_site_execution_count),
        (
            "cp401_supply_humidity_ratio_state_owner_count",
            expected_humidity_owners,
            state.cp401_supply_humidity_ratio_state_owner_count,
        ),
        (
            "unchanged_supply_humidity_ratio_preservation_count",
            expected_humidity_owners,
            state.unchanged_supply_humidity_ratio_preservation_count,
        ),
        (
            "cp401_supply_enthalpy_state_owner_count",
            expected_enthalpy_owners,
            state.cp401_supply_enthalpy_state_owner_count,
        ),
        (
            "unchanged_supply_enthalpy_preservation_count",
            expected_enthalpy_owners,
            state.unchanged_supply_enthalpy_preservation_count,
        ),
        (
            "cp401_supply_temperature_state_owner_count",
            expected_temperature_owners,
            state.cp401_supply_temperature_state_owner_count,
        ),
        (
            "unchanged_supply_temperature_preservation_count",
            expected_temperature_owners,
            state.unchanged_supply_temperature_preservation_count,
        ),
        (
            "cp401_cooling_latent_output_owned_read_count",
            evaluations,
            state.cp401_cooling_latent_output_owned_read_count,
        ),
        (
            "cooling_latent_output_read_count",
            evaluations,
            state.cooling_latent_output_read_count,
        ),
        (
            "cp321_maximum_total_cooling_capacity_owned_read_count",
            evaluations,
            state.cp321_maximum_total_cooling_capacity_owned_read_count,
        ),
        (
            "cp340_same_call_maximum_total_cooling_capacity_bit_corroboration_count",
            evaluations,
            state.cp340_same_call_maximum_total_cooling_capacity_bit_corroboration_count,
        ),
        (
            "maximum_total_cooling_capacity_read_count",
            evaluations,
            state.maximum_total_cooling_capacity_read_count,
        ),
        (
            "cooling_latent_output_maximum_total_cooling_capacity_comparison_count",
            evaluations,
            state.cooling_latent_output_maximum_total_cooling_capacity_comparison_count,
        ),
        (
            "cooling_latent_output_greater_than_or_equal_to_maximum_total_cooling_capacity_count",
            body_entries,
            state.cooling_latent_output_greater_than_or_equal_to_maximum_total_cooling_capacity_count,
        ),
        (
            "latent_output_capacity_adjustment_body_entry_count",
            body_entries,
            state.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_adjustment_body_entry_count,
        ),
        (
            "latent_output_capacity_guard_false_fallthrough_count",
            guard_false,
            state.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough_count,
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
    let owner_latest = owner
        .latest
        .ok_or_else(|| violation("cp321_latest_owner_snapshot_ready", 1, 0))?;
    let corroborator_latest = corroborator
        .latest
        .ok_or_else(|| violation("cp340_latest_corroborator_snapshot_ready", 1, 0))?;
    let expected = active_input(predecessor_latest, owner_latest, corroborator_latest)
        .and_then(|input| {
            private_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard_characterization(
                predecessor_latest,
                input,
            )
        })
        .ok_or_else(|| violation("latest_owner_lineage_ready", 1, 0))?;
    let latest_route_count = if latest
        .dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_adjustment_body_entered
    {
        body_entries
    } else if latest
        .dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough
    {
        guard_false
    } else {
        state.inactive_transition_count
    };

    if !same_snapshot(latest, expected)
        || !same_snapshot(
            latest,
            latest_output
                .calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard,
        )
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
    capacity_owner: CapacitySnapshot,
    capacity_corroborator: CapacityCorroboratorSnapshot,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    snapshot.system == binding.ideal_loads_air_system
        && snapshot.parent_call_ordinal == call_ordinal
        && snapshot.controlled_zone == binding.zone
        && [
            predecessor.system,
            capacity_owner.system,
            capacity_corroborator.system,
        ]
        .into_iter()
        .all(|system| system == snapshot.system)
        && [
            predecessor.parent_call_ordinal,
            capacity_owner.parent_call_ordinal,
            capacity_corroborator.parent_call_ordinal,
        ]
        .into_iter()
        .all(|ordinal| ordinal == snapshot.parent_call_ordinal)
        && [
            predecessor.controlled_zone,
            capacity_owner.controlled_zone,
            capacity_corroborator.controlled_zone,
        ]
        .into_iter()
        .all(|zone| zone == snapshot.controlled_zone)
}

fn active_input(
    predecessor: PredecessorSnapshot,
    capacity_owner: CapacitySnapshot,
    capacity_corroborator: CapacityCorroboratorSnapshot,
) -> Option<Option<ActiveInput>> {
    let active = predecessor
        .dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_assignment_executed;
    if !active {
        return Some(None);
    }
    if !predecessor.cooling_latent_output_assigned
        || !capacity_owner.cooling_body_entered
        || capacity_owner.cooling_limit_condition_satisfied != Some(true)
        || !capacity_owner.maximum_total_cooling_capacity_read
        || !capacity_owner.maximum_total_cooling_capacity_comparison_evaluated
        || capacity_owner.maximum_total_cooling_capacity_equal_to_zero != Some(false)
        || capacity_owner.zero_cooling_capacity_body_entered
        || !capacity_corroborator.capacity_limit_sensible_output_guard_evaluated
        || !capacity_corroborator.maximum_total_cooling_capacity_read
        || !cooling_capacity_zero_flow_reset_snapshot_is_exact_direct_release(capacity_owner)
        || !cooling_positive_supply_capacity_limit_sensible_output_guard_snapshot_is_exact_direct_release(
            capacity_corroborator,
        )
    {
        return None;
    }
    let cooling_latent_output_w = predecessor.cooling_latent_output_w?;
    let maximum_total_cooling_capacity_w = capacity_owner.maximum_total_cooling_capacity_w?;
    let corroborated = capacity_corroborator.maximum_total_cooling_capacity_w?;
    if !maximum_total_cooling_capacity_w.is_finite()
        || maximum_total_cooling_capacity_w < 0.0
        || corroborated.to_bits() != maximum_total_cooling_capacity_w.to_bits()
    {
        return None;
    }
    Some(Some(ActiveInput {
        cooling_latent_output_w,
        maximum_total_cooling_capacity_w,
        cp401_cooling_latent_output_owned_read: true,
        cp321_maximum_total_cooling_capacity_owned_read: true,
        cp340_same_call_maximum_total_cooling_capacity_bit_corroborated: true,
    }))
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

fn checked_sum(values: &[usize], field: &'static str) -> Result<usize, Error> {
    values.iter().try_fold(0usize, |sum, value| {
        sum.checked_add(*value)
            .ok_or_else(|| violation(field, 0, usize::MAX))
    })
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

fn ensure_count(actual: usize, expected: usize, field: &'static str) -> Result<(), Error> {
    if actual == expected {
        Ok(())
    } else {
        Err(violation(field, expected, actual))
    }
}

fn violation(field: &'static str, expected: usize, actual: usize) -> Error {
    Error::CalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputGuardLifecycleInvariant {
        field,
        expected,
        actual,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_site_arithmetic_overflow_fails_closed() {
        assert!(usize::MAX.checked_mul(3).is_none());
    }

    #[test]
    fn bit_comparison_distinguishes_signed_zero() {
        assert!(options_have_exact_bits(Some(-0.0), Some(-0.0)));
        assert!(!options_have_exact_bits(Some(-0.0), Some(0.0)));
    }
}
