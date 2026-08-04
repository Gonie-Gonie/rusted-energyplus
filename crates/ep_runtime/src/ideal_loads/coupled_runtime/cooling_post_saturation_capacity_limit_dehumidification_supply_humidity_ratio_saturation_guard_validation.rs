//! Coupled-runtime validation for CP413 saturation humidity-ratio guard evidence.

use ep_model::DehumidificationControlType;

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_GUARD_SOURCE,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationAssignmentLifecycleSummary as PredecessorLifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationAssignmentRuntimeState as PredecessorState,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationGuardLifecycleSummary as Lifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationGuardRuntimeState as State,
    cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_snapshot_is_exact_direct_release,
    cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_snapshot_is_exact_direct_release,
    private_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_characterization,
};

use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

const SPLIT_PREDECESSOR_INDICES: [usize; 6] = [20, 21, 24, 25, 27, 29];
const PUBLIC_LOGICAL_INDICES: [usize; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 20, 21, 26, 27];
const FIRST_ACTIVE_LOGICAL_INDEX: usize = 18;

pub(in crate::ideal_loads) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let predecessor = output
        .calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment;
    let snapshot = output
        .calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard;
    let expected =
        private_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_characterization(
            predecessor,
        );

    snapshot.system == binding.ideal_loads_air_system
        && snapshot.parent_call_ordinal == call_ordinal
        && snapshot.controlled_zone == binding.zone
        && binding.system.dehumidification_control_type == DehumidificationControlType::None
        && cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_snapshot_is_exact_direct_release(predecessor)
        && cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_snapshot_is_exact_direct_release(snapshot)
        && expected.is_some_and(|expected| lineage::same_snapshot(snapshot, expected))
}

pub(in crate::ideal_loads) fn validate_lifecycle(
    lifecycle: &Lifecycle,
    predecessor_cp412: &PredecessorLifecycle,
    timestep_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), Error> {
    let state = &lifecycle.state;
    let predecessor = &predecessor_cp412.state;
    if lifecycle.source
        != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_GUARD_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_GUARD_FIRST_EXCLUDED_SOURCE
        || predecessor_cp412.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE
        || predecessor_cp412.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || state.system != binding.ideal_loads_air_system
        || predecessor.system != binding.ideal_loads_air_system
        || binding.system.dehumidification_control_type != DehumidificationControlType::None
    {
        return Err(violation("source_predecessor_and_system_identity", 1, 0));
    }

    let expected_predecessor_routes = flatten_predecessor_routes(predecessor)?;
    if state.predecessor_route_counts != expected_predecessor_routes {
        return Err(violation("predecessor_route_lineage", 1, 0));
    }
    for values in [
        &state.predecessor_route_counts,
        &state.guard_false_fallthrough_route_counts,
        &state.guard_body_entry_route_counts,
    ] {
        ensure_public_routes_only(values)?;
    }
    validate_route_evidence(state)?;

    let transitions = checked_sum(&state.predecessor_route_counts, "route_partition_overflow")?;
    let evaluations = checked_sum(
        &state.predecessor_route_counts[FIRST_ACTIVE_LOGICAL_INDEX..],
        "active_route_partition_overflow",
    )?;
    let guard_false = checked_sum(
        &state.guard_false_fallthrough_route_counts,
        "guard_false_partition_overflow",
    )?;
    let body_entries = checked_sum(
        &state.guard_body_entry_route_counts,
        "guard_body_partition_overflow",
    )?;
    let guard_outcomes = checked_guard_outcome_partition(guard_false, body_entries)?;
    let inactive = transitions
        .checked_sub(evaluations)
        .ok_or_else(|| violation("inactive_transition_underflow", evaluations, transitions))?;
    let enthalpy_owners = sum_predecessor_indices(
        &state.predecessor_route_counts,
        |index| matches!(index, 5 | 8 | 11 | 14 | 17..=29),
        "enthalpy_owner_partition_overflow",
    )?;
    let temperature_owners = sum_predecessor_indices(
        &state.predecessor_route_counts,
        |index| index >= 3,
        "temperature_owner_partition_overflow",
    )?;
    let source_sites = evaluations
        .checked_mul(3)
        .and_then(|sites| sites.checked_add(body_entries))
        .ok_or_else(|| violation("source_site_execution_count_overflow", 0, usize::MAX))?;

    for (field, expected, actual) in [
        ("transition_count", timestep_count, state.transition_count),
        ("predecessor_transition_count", predecessor.transition_count, state.transition_count),
        ("route_partition", state.transition_count, transitions),
        ("inactive_transition_count", inactive, state.inactive_transition_count),
        ("guard_evaluation_count", evaluations, state.saturation_supply_humidity_ratio_guard_evaluation_count),
        ("guard_outcome_partition", evaluations, guard_outcomes),
        ("source_site_execution_count", source_sites, state.source_site_execution_count),
        ("cp412_supply_humidity_ratio_state_owner_count", evaluations, state.cp412_supply_humidity_ratio_state_owner_count),
        ("unchanged_supply_humidity_ratio_preservation_count", evaluations, state.unchanged_supply_humidity_ratio_preservation_count),
        ("cp412_supply_enthalpy_state_owner_count", enthalpy_owners, state.cp412_supply_enthalpy_state_owner_count),
        ("unchanged_supply_enthalpy_preservation_count", enthalpy_owners, state.unchanged_supply_enthalpy_preservation_count),
        ("cp412_supply_temperature_state_owner_count", temperature_owners, state.cp412_supply_temperature_state_owner_count),
        ("unchanged_supply_temperature_preservation_count", temperature_owners, state.unchanged_supply_temperature_preservation_count),
        ("cp412_saturation_supply_humidity_ratio_owned_read_count", evaluations, state.cp412_saturation_supply_humidity_ratio_owned_read_count),
        ("saturation_supply_humidity_ratio_for_guard_read_count", evaluations, state.saturation_supply_humidity_ratio_for_guard_read_count),
        ("cp411_original_supply_humidity_ratio_owned_read_count", evaluations, state.cp411_original_supply_humidity_ratio_owned_read_count),
        ("cp412_same_call_original_supply_humidity_ratio_bit_corroboration_count", evaluations, state.cp412_same_call_original_supply_humidity_ratio_bit_corroboration_count),
        ("original_supply_humidity_ratio_for_guard_read_count", evaluations, state.original_supply_humidity_ratio_for_guard_read_count),
        ("saturation_original_supply_humidity_ratio_comparison_count", evaluations, state.saturation_original_supply_humidity_ratio_comparison_count),
        ("strictly_less_than_count", body_entries, state.saturation_supply_humidity_ratio_strictly_less_than_original_supply_humidity_ratio_count),
        ("guard_body_entry_count", body_entries, state.saturation_supply_humidity_ratio_guard_body_entry_count),
        ("guard_false_fallthrough_count", guard_false, state.saturation_supply_humidity_ratio_guard_false_fallthrough_count),
    ] {
        ensure_count(actual, expected, field)?;
    }

    let latest = state
        .latest
        .ok_or_else(|| violation("latest_release_snapshot_ready", 1, 0))?;
    let predecessor_latest = predecessor
        .latest
        .ok_or_else(|| violation("predecessor_latest_release_snapshot_ready", 1, 0))?;
    let expected =
        private_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_characterization(
            predecessor_latest,
        )
        .ok_or_else(|| violation("latest_predecessor_lineage_ready", 1, 0))?;
    if !lineage::same_snapshot(latest, expected)
        || !lineage::same_snapshot(
            latest,
            latest_output
                .calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard,
        )
        || !snapshot_matches_release(latest_output, timestep_count, binding)
    {
        return Err(violation("latest_release_snapshot_ready", 1, 0));
    }
    Ok(())
}

fn flatten_predecessor_routes(predecessor: &PredecessorState) -> Result<[usize; 36], Error> {
    let mut flattened = [0usize; 36];
    let mut logical = 0usize;
    for index in 0..30 {
        if SPLIT_PREDECESSOR_INDICES.contains(&index) {
            let guard_false = predecessor.predecessor_guard_false_fallthrough_route_counts[index];
            let maximum = predecessor.predecessor_maximum_capacity_assignment_route_counts[index];
            ensure_count(
                guard_false
                    .checked_add(maximum)
                    .ok_or_else(|| violation("predecessor_split_route_overflow", 0, usize::MAX))?,
                predecessor.predecessor_route_counts[index],
                "predecessor_split_route_partition",
            )?;
            flattened[logical] = guard_false;
            flattened[logical + 1] = maximum;
            logical += 2;
        } else {
            flattened[logical] = predecessor.predecessor_route_counts[index];
            logical += 1;
        }
    }
    ensure_count(logical, flattened.len(), "flattened_route_width")?;
    Ok(flattened)
}

fn validate_route_evidence(state: &State) -> Result<(), Error> {
    for index in 0..36 {
        let outcomes = state.guard_false_fallthrough_route_counts[index]
            .checked_add(state.guard_body_entry_route_counts[index])
            .ok_or_else(|| violation("guard_outcome_route_overflow", 0, usize::MAX))?;
        let expected = if index >= FIRST_ACTIVE_LOGICAL_INDEX {
            state.predecessor_route_counts[index]
        } else {
            0
        };
        ensure_count(outcomes, expected, "guard_outcome_route_partition")?;
    }
    Ok(())
}

fn sum_predecessor_indices(
    values: &[usize; 36],
    include: impl Fn(usize) -> bool,
    field: &'static str,
) -> Result<usize, Error> {
    let mut logical = 0usize;
    let mut total = 0usize;
    for predecessor_index in 0..30 {
        let width = 1 + usize::from(SPLIT_PREDECESSOR_INDICES.contains(&predecessor_index));
        if include(predecessor_index) {
            total = values[logical..logical + width]
                .iter()
                .try_fold(total, |sum, value| sum.checked_add(*value))
                .ok_or_else(|| violation(field, 0, usize::MAX))?;
        }
        logical += width;
    }
    Ok(total)
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

fn checked_guard_outcome_partition(
    guard_false: usize,
    body_entries: usize,
) -> Result<usize, Error> {
    guard_false
        .checked_add(body_entries)
        .ok_or_else(|| violation("guard_outcome_partition_overflow", 0, usize::MAX))
}

fn ensure_count(actual: usize, expected: usize, field: &'static str) -> Result<(), Error> {
    if actual == expected {
        Ok(())
    } else {
        Err(violation(field, expected, actual))
    }
}

fn violation(field: &'static str, expected: usize, actual: usize) -> Error {
    Error::CalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationGuardLifecycleInvariant {
        field,
        expected,
        actual,
    }
}

mod lineage;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conceptual_route_contract_is_54_with_18_inactive_and_two_active_outcomes() {
        let predecessor = 36;
        let active = predecessor - 18;
        assert_eq!((predecessor + active, 18, active, active), (54, 18, 18, 18));
    }

    #[test]
    fn conceptual_source_site_total_is_126() {
        assert_eq!(18 * 3 + 18 * 4, 126);
    }

    #[test]
    fn overflow_helpers_fail_closed() {
        assert!(checked_sum(&[usize::MAX, 1], "overflow").is_err());
        assert_eq!(
            checked_guard_outcome_partition(usize::MAX, 1),
            Err(violation("guard_outcome_partition_overflow", 0, usize::MAX,)),
        );
    }
}
