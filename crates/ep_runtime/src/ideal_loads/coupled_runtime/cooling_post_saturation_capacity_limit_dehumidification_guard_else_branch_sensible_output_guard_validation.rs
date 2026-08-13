//! Cheap coupled validation for CP421 sensible-output maximum-capacity guard evidence.

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
    PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingCapacityZeroFlowResetLifecycleSummary as CapacityLifecycle,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardLifecycleSummary as CorroboratorLifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputAssignmentLifecycleSummary as PredecessorLifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardLifecycleSummary as Lifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardRuntimeState as State,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_snapshots_match_bit_exact,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_predecessor_cp420_snapshot,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_snapshots_match_bit_exact,
};

use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

const PUBLIC_LOGICAL_INDICES: [usize; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 20, 21, 26, 27];
const ACTIVE_LOGICAL_INDICES: [usize; 5] = [4, 7, 10, 13, 16];

pub(in crate::ideal_loads) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let snapshot = output
        .calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard;
    let predecessor = output
        .calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment;
    snapshot.system == binding.ideal_loads_air_system
        && snapshot.parent_call_ordinal == call_ordinal
        && snapshot.controlled_zone == binding.zone
        && cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_snapshots_match_bit_exact(
            cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_predecessor_cp420_snapshot(snapshot),
            predecessor,
        )
        && local_guard_matches(
            snapshot,
            predecessor,
            output.calculation_cooling_capacity_zero_flow_reset,
            output.calculation_cooling_positive_supply_capacity_limit_sensible_output_guard,
        )
}

#[allow(clippy::too_many_arguments)]
pub(in crate::ideal_loads) fn validate_lifecycle(
    lifecycle: &Lifecycle,
    predecessor_cp420: &PredecessorLifecycle,
    capacity_cp321: &CapacityLifecycle,
    corroborator_cp340: &CorroboratorLifecycle,
    timestep_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), Error> {
    let state = &lifecycle.state;
    let predecessor = &predecessor_cp420.state;
    let capacity = &capacity_cp321.state;
    let corroborator = &corroborator_cp340.state;
    if lifecycle.source
        != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_GUARD_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE
        || predecessor_cp420.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE
        || predecessor_cp420.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || capacity_cp321.source != PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_SOURCE
        || capacity_cp321.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_FIRST_EXCLUDED_SOURCE
        || corroborator_cp340.source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_SOURCE
        || corroborator_cp340.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_GUARD_SOURCE_ORDER.len() != 4
        || [state.system, predecessor.system, capacity.system, corroborator.system]
            .into_iter()
            .any(|system| system != binding.ideal_loads_air_system)
        || [predecessor.transition_count, capacity.transition_count, corroborator.transition_count]
            .into_iter()
            .any(|count| count != state.transition_count)
        || state.predecessor_route_counts != predecessor.predecessor_route_counts
    {
        return Err(violation("source_owner_predecessor_route_and_system_identity", 1, 0));
    }
    validate_counts(state, predecessor, timestep_count)?;

    let latest = state
        .latest
        .ok_or_else(|| violation("latest_release_snapshot_ready", 1, 0))?;
    if !cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_snapshots_match_bit_exact(
        latest,
        latest_output.calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard,
    ) || !snapshot_matches_release(latest_output, timestep_count, binding)
    {
        return Err(violation("latest_release_snapshot_ready", 1, 0));
    }
    Ok(())
}

fn validate_counts(
    state: &State,
    predecessor: &crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputAssignmentRuntimeState,
    timestep_count: usize,
) -> Result<(), Error> {
    for values in [
        &state.predecessor_route_counts,
        &state.guard_false_fallthrough_route_counts,
        &state.adjustment_body_entry_route_counts,
    ] {
        for (index, count) in values.iter().enumerate() {
            if !PUBLIC_LOGICAL_INDICES.contains(&index) && *count != 0 {
                return Err(violation("non_direct_route_count", 0, *count));
            }
        }
    }
    for index in 0..36 {
        let actual = checked_add(
            state.guard_false_fallthrough_route_counts[index],
            state.adjustment_body_entry_route_counts[index],
            "route_partition_overflow",
        )?;
        let expected = usize::from(ACTIVE_LOGICAL_INDICES.contains(&index))
            .checked_mul(state.predecessor_route_counts[index])
            .ok_or_else(|| violation("route_partition_overflow", 0, usize::MAX))?;
        ensure_count(actual, expected, "route_partition")?;
    }
    let transitions = checked_sum(
        &state.predecessor_route_counts,
        "transition_partition_overflow",
    )?;
    let false_fallthroughs = checked_sum(
        &state.guard_false_fallthrough_route_counts,
        "guard_partition_overflow",
    )?;
    let body_entries = checked_sum(
        &state.adjustment_body_entry_route_counts,
        "guard_partition_overflow",
    )?;
    let evaluations = checked_add(false_fallthroughs, body_entries, "guard_partition_overflow")?;
    let inactive = transitions
        .checked_sub(evaluations)
        .ok_or_else(|| violation("transition_partition_underflow", evaluations, transitions))?;
    let sites = evaluations
        .checked_mul(3)
        .and_then(|value| value.checked_add(body_entries))
        .ok_or_else(|| violation("site_count_overflow", 0, usize::MAX))?;
    for (field, expected, actual) in [
        ("transition_count", timestep_count, state.transition_count),
        ("route_partition", state.transition_count, transitions),
        ("inactive_transition_count", inactive, state.inactive_transition_count),
        ("guard_evaluation_count", evaluations, state.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_guard_evaluation_count),
        ("source_site_execution_count", sites, state.source_site_execution_count),
        ("humidity_owner_count", predecessor.cp419_supply_humidity_ratio_state_owner_count, state.cp420_supply_humidity_ratio_state_owner_count),
        ("humidity_preservation_count", predecessor.unchanged_supply_humidity_ratio_preservation_count, state.unchanged_supply_humidity_ratio_preservation_count),
        ("enthalpy_owner_count", predecessor.cp419_supply_enthalpy_state_owner_count, state.cp420_supply_enthalpy_state_owner_count),
        ("enthalpy_preservation_count", predecessor.unchanged_supply_enthalpy_preservation_count, state.unchanged_supply_enthalpy_preservation_count),
        ("temperature_owner_count", predecessor.cp419_supply_temperature_state_owner_count, state.cp420_supply_temperature_state_owner_count),
        ("temperature_preservation_count", predecessor.unchanged_supply_temperature_preservation_count, state.unchanged_supply_temperature_preservation_count),
        ("comparison_true_count", body_entries, state.cooling_sensible_output_greater_than_or_equal_to_maximum_total_cooling_capacity_count),
        ("body_entry_count", body_entries, state.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_adjustment_body_entry_count),
        ("false_fallthrough_count", false_fallthroughs, state.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_guard_false_fallthrough_count),
    ] {
        ensure_count(actual, expected, field)?;
    }
    for (field, actual) in [
        (
            "cooling_output_owned_read",
            state.cp420_cooling_sensible_output_owned_read_count,
        ),
        (
            "cooling_output_read",
            state.cooling_sensible_output_read_count,
        ),
        (
            "capacity_owned_read",
            state.cp321_maximum_total_cooling_capacity_owned_read_count,
        ),
        (
            "capacity_corroboration",
            state.cp340_same_call_maximum_total_cooling_capacity_bit_corroboration_count,
        ),
        (
            "capacity_read",
            state.maximum_total_cooling_capacity_read_count,
        ),
        (
            "comparison",
            state.cooling_sensible_output_maximum_total_cooling_capacity_comparison_count,
        ),
    ] {
        ensure_count(actual, evaluations, field)?;
    }
    Ok(())
}

fn local_guard_matches(
    snapshot: crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardSnapshot,
    predecessor: crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputAssignmentSnapshot,
    capacity: crate::ideal_loads::PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot,
    corroborator: crate::ideal_loads::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot,
) -> bool {
    let active = predecessor
        .post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_executed;
    if snapshot.source
        != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_GUARD_SOURCE
        || snapshot.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE
        || snapshot.source_order
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_GUARD_SOURCE_ORDER
        || [predecessor.system, capacity.system, corroborator.system]
        .into_iter()
        .any(|system| system != snapshot.system)
        || [
            predecessor.parent_call_ordinal,
            capacity.parent_call_ordinal,
            corroborator.parent_call_ordinal,
        ]
        .into_iter()
        .any(|ordinal| ordinal != snapshot.parent_call_ordinal)
        || [
            predecessor.controlled_zone,
            capacity.controlled_zone,
            corroborator.controlled_zone,
        ]
        .into_iter()
        .any(|zone| zone != snapshot.controlled_zone)
        || snapshot.cp420_retained_supply_humidity_ratio_state_owned
            != predecessor.resulting_supply_humidity_ratio.is_some()
        || snapshot.cp420_retained_supply_enthalpy_state_owned
            != predecessor.resulting_supply_enthalpy_j_per_kg.is_some()
        || snapshot.cp420_retained_supply_temperature_state_owned
            != predecessor.resulting_supply_temperature_c.is_some()
        || !option_bits_equal(
            snapshot.predecessor_cp420_resulting_supply_humidity_ratio,
            predecessor.resulting_supply_humidity_ratio,
        )
        || !option_bits_equal(
            snapshot.predecessor_cp420_resulting_supply_enthalpy_j_per_kg,
            predecessor.resulting_supply_enthalpy_j_per_kg,
        )
        || !option_bits_equal(
            snapshot.predecessor_cp420_resulting_supply_temperature_c,
            predecessor.resulting_supply_temperature_c,
        )
        || !option_bits_equal(
            snapshot.resulting_supply_humidity_ratio,
            predecessor.resulting_supply_humidity_ratio,
        )
        || !option_bits_equal(
            snapshot.resulting_supply_enthalpy_j_per_kg,
            predecessor.resulting_supply_enthalpy_j_per_kg,
        )
        || !option_bits_equal(
            snapshot.resulting_supply_temperature_c,
            predecessor.resulting_supply_temperature_c,
        )
    {
        return false;
    }
    let flags = [
        snapshot.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_guard_evaluated,
        snapshot.cp420_retained_cooling_sensible_output_owned_read,
        snapshot.cooling_sensible_output_read,
        snapshot.cp321_maximum_total_cooling_capacity_owned_read,
        snapshot.cp340_same_call_maximum_total_cooling_capacity_bit_corroborated,
        snapshot.maximum_total_cooling_capacity_read,
        snapshot.cooling_sensible_output_maximum_total_cooling_capacity_comparison_evaluated,
    ];
    if flags.into_iter().any(|flag| flag != active) {
        return false;
    }
    if !active {
        return snapshot.cp420_cooling_sensible_output_for_capacity_guard_w.is_none()
            && snapshot.maximum_total_cooling_capacity_w.is_none()
            && snapshot.cooling_sensible_output_greater_than_or_equal_to_maximum_total_cooling_capacity.is_none()
            && !snapshot.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_adjustment_body_entered
            && !snapshot.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_guard_false_fallthrough;
    }
    let (Some(cooling), Some(maximum), Some(corroborating_maximum)) = (
        predecessor.cooling_sensible_output_w,
        capacity.maximum_total_cooling_capacity_w,
        corroborator.maximum_total_cooling_capacity_w,
    ) else {
        return false;
    };
    let comparison = cooling >= maximum;
    capacity.maximum_total_cooling_capacity_read
        && corroborator.maximum_total_cooling_capacity_read
        && option_has_bits(snapshot.cp420_cooling_sensible_output_for_capacity_guard_w, cooling)
        && maximum.to_bits() == corroborating_maximum.to_bits()
        && option_has_bits(snapshot.maximum_total_cooling_capacity_w, maximum)
        && snapshot.cooling_sensible_output_greater_than_or_equal_to_maximum_total_cooling_capacity == Some(comparison)
        && snapshot.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_adjustment_body_entered == comparison
        && snapshot.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_guard_false_fallthrough != comparison
}

fn option_has_bits(value: Option<f64>, expected: f64) -> bool {
    value.is_some_and(|value| value.to_bits() == expected.to_bits())
}

fn option_bits_equal(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}

fn checked_add(left: usize, right: usize, field: &'static str) -> Result<usize, Error> {
    left.checked_add(right)
        .ok_or_else(|| violation(field, 0, usize::MAX))
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
    Error::CalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardLifecycleInvariant { field, expected, actual }
}

#[cfg(test)]
mod tests {
    #[test]
    fn hot_validator_uses_bounded_cp420_reconstruction_and_three_arrays() {
        let source = include_str!(
            "cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_validation.rs"
        );
        let production = source
            .split_once("#[cfg(test)]")
            .map_or(source, |(value, _)| value);
        assert!(production.contains("predecessor_cp420_snapshot"));
        for required in [
            "snapshot.source",
            "snapshot.first_excluded_source",
            "snapshot.source_order",
            "snapshot.predecessor_cp420_resulting_supply_humidity_ratio",
            "snapshot.resulting_supply_humidity_ratio",
            "snapshot.cp420_retained_supply_humidity_ratio_state_owned",
            "snapshot.cp420_cooling_sensible_output_for_capacity_guard_w.is_none()",
            "sensible_output_capacity_adjustment_body_entered",
            "sensible_output_capacity_guard_false_fallthrough",
        ] {
            assert!(production.contains(required), "{required}");
        }
        for forbidden in ["private_characterization", "predecessor_route("] {
            assert!(!production.contains(forbidden), "{forbidden}");
        }
    }
}
