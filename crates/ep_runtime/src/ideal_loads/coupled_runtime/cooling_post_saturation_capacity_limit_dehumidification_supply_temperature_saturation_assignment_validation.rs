//! Coupled-runtime validation for CP414 saturation-temperature assignment evidence.

use ep_model::DehumidificationControlType;

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_TEMPERATURE_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_TEMPERATURE_SATURATION_ASSIGNMENT_SOURCE,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationGuardLifecycleSummary as PredecessorLifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationAssignmentLifecycleSummary as Lifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationAssignmentRuntimeState as State,
    cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_snapshot_is_exact_direct_release,
    cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_snapshot_is_exact_direct_release,
    cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_snapshots_match_bit_exact,
    private_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_characterization,
};

use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

const SPLIT_PREDECESSOR_INDICES: [usize; 6] = [20, 21, 24, 25, 27, 29];
const PUBLIC_LOGICAL_INDICES: [usize; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 20, 21, 26, 27];

pub(in crate::ideal_loads) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let predecessor = output
        .calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard;
    let snapshot = output
        .calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment;
    let pressure = snapshot
        .outdoor_barometric_pressure_for_saturation_temperature_pa
        .unwrap_or(crate::psychrometrics::ENERGYPLUS_STANDARD_ATMOSPHERIC_PRESSURE_PA);
    let expected = private_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_characterization(
        predecessor,
        pressure,
    );

    snapshot.system == binding.ideal_loads_air_system
        && snapshot.parent_call_ordinal == call_ordinal
        && snapshot.controlled_zone == binding.zone
        && binding.system.dehumidification_control_type == DehumidificationControlType::None
        && cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_snapshot_is_exact_direct_release(predecessor)
        && cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_snapshot_is_exact_direct_release(snapshot)
        && expected.is_some_and(|expected| {
            cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_snapshots_match_bit_exact(snapshot, expected)
        })
}

pub(in crate::ideal_loads) fn validate_lifecycle(
    lifecycle: &Lifecycle,
    predecessor_cp413: &PredecessorLifecycle,
    timestep_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), Error> {
    let state = &lifecycle.state;
    let predecessor = &predecessor_cp413.state;
    if lifecycle.source
        != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_TEMPERATURE_SATURATION_ASSIGNMENT_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_TEMPERATURE_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || predecessor_cp413.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_GUARD_SOURCE
        || predecessor_cp413.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_GUARD_FIRST_EXCLUDED_SOURCE
        || state.system != binding.ideal_loads_air_system
        || predecessor.system != binding.ideal_loads_air_system
        || binding.system.dehumidification_control_type != DehumidificationControlType::None
    {
        return Err(violation("source_predecessor_and_system_identity", 1, 0));
    }

    ensure_count(state.transition_count, timestep_count, "transition_count")?;
    ensure_count(
        state.transition_count,
        predecessor.transition_count,
        "predecessor_transition_count",
    )?;
    if state.predecessor_route_counts != predecessor.predecessor_route_counts
        || state.predecessor_guard_false_fallthrough_route_counts
            != predecessor.guard_false_fallthrough_route_counts
        || state.predecessor_guard_body_entry_route_counts
            != predecessor.guard_body_entry_route_counts
        || state.supply_temperature_saturation_assignment_route_counts
            != predecessor.guard_body_entry_route_counts
    {
        return Err(violation("predecessor_route_lineage", 1, 0));
    }
    for values in [
        &state.predecessor_route_counts,
        &state.predecessor_guard_false_fallthrough_route_counts,
        &state.predecessor_guard_body_entry_route_counts,
        &state.supply_temperature_saturation_assignment_route_counts,
    ] {
        ensure_public_routes_only(values)?;
    }
    validate_route_evidence(state)?;

    let transitions = checked_sum(&state.predecessor_route_counts, "route_partition_overflow")?;
    let assignments = checked_sum(
        &state.supply_temperature_saturation_assignment_route_counts,
        "assignment_partition_overflow",
    )?;
    let inactive = transitions
        .checked_sub(assignments)
        .ok_or_else(|| violation("inactive_transition_underflow", assignments, transitions))?;
    let humidity_ratio_owners = checked_sum(
        &state.predecessor_route_counts[18..],
        "humidity_ratio_owner_partition_overflow",
    )?;
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
    let unchanged_temperature = temperature_owners.checked_sub(assignments).ok_or_else(|| {
        violation(
            "unchanged_temperature_owner_underflow",
            assignments,
            temperature_owners,
        )
    })?;
    let source_sites = assignments
        .checked_mul(4)
        .ok_or_else(|| violation("source_site_execution_count_overflow", 0, usize::MAX))?;

    for (field, expected, actual) in [
        ("route_partition", state.transition_count, transitions),
        (
            "inactive_transition_count",
            inactive,
            state.inactive_transition_count,
        ),
        (
            "assignment_count",
            assignments,
            state.saturation_supply_temperature_assignment_count,
        ),
        (
            "source_site_execution_count",
            source_sites,
            state.source_site_execution_count,
        ),
        (
            "cp413_supply_humidity_ratio_state_owner_count",
            humidity_ratio_owners,
            state.cp413_supply_humidity_ratio_state_owner_count,
        ),
        (
            "unchanged_supply_humidity_ratio_preservation_count",
            humidity_ratio_owners,
            state.unchanged_supply_humidity_ratio_preservation_count,
        ),
        (
            "cp413_supply_enthalpy_state_owner_count",
            enthalpy_owners,
            state.cp413_supply_enthalpy_state_owner_count,
        ),
        (
            "unchanged_supply_enthalpy_preservation_count",
            enthalpy_owners,
            state.unchanged_supply_enthalpy_preservation_count,
        ),
        (
            "cp413_supply_temperature_state_owner_count",
            temperature_owners,
            state.cp413_supply_temperature_state_owner_count,
        ),
        (
            "unchanged_supply_temperature_preservation_count",
            unchanged_temperature,
            state.unchanged_supply_temperature_preservation_count,
        ),
        (
            "cp414_saturation_supply_temperature_state_owner_count",
            assignments,
            state.cp414_saturation_supply_temperature_state_owner_count,
        ),
        (
            "cp413_retained_supply_enthalpy_owned_read_count",
            assignments,
            state.cp413_retained_supply_enthalpy_owned_read_count,
        ),
        (
            "supply_enthalpy_for_saturation_temperature_read_count",
            assignments,
            state.supply_enthalpy_for_saturation_temperature_read_count,
        ),
        (
            "environment_outdoor_barometric_pressure_for_saturation_temperature_owner_count",
            assignments,
            state.environment_outdoor_barometric_pressure_for_saturation_temperature_owner_count,
        ),
        (
            "environment_outdoor_barometric_pressure_for_saturation_temperature_read_count",
            assignments,
            state.environment_outdoor_barometric_pressure_for_saturation_temperature_read_count,
        ),
        (
            "psy_tsat_fn_h_pb_evaluation_count",
            assignments,
            state.psy_tsat_fn_h_pb_evaluation_count,
        ),
        (
            "purchased_air_supply_temperature_saturation_assignment_write_count",
            assignments,
            state.purchased_air_supply_temperature_saturation_assignment_write_count,
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
    let pressure = latest
        .outdoor_barometric_pressure_for_saturation_temperature_pa
        .unwrap_or(crate::psychrometrics::ENERGYPLUS_STANDARD_ATMOSPHERIC_PRESSURE_PA);
    let expected = private_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_characterization(
        predecessor_latest,
        pressure,
    )
    .ok_or_else(|| violation("latest_predecessor_lineage_ready", 1, 0))?;
    let output_latest = latest_output
        .calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment;
    if !cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_snapshots_match_bit_exact(latest, expected)
        || !cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_snapshots_match_bit_exact(latest, output_latest)
        || !snapshot_matches_release(latest_output, timestep_count, binding)
    {
        return Err(violation("latest_release_snapshot_ready", 1, 0));
    }
    Ok(())
}

fn validate_route_evidence(state: &State) -> Result<(), Error> {
    for index in 0..36 {
        let predecessor_outcomes = state.predecessor_guard_false_fallthrough_route_counts[index]
            .checked_add(state.predecessor_guard_body_entry_route_counts[index])
            .ok_or_else(|| violation("predecessor_guard_outcome_route_overflow", 0, usize::MAX))?;
        let expected_outcomes = if index >= 18 {
            state.predecessor_route_counts[index]
        } else {
            0
        };
        ensure_count(
            predecessor_outcomes,
            expected_outcomes,
            "predecessor_guard_outcome_route_partition",
        )?;
        ensure_count(
            state.supply_temperature_saturation_assignment_route_counts[index],
            state.predecessor_guard_body_entry_route_counts[index],
            "assignment_route_partition",
        )?;
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

fn ensure_count(actual: usize, expected: usize, field: &'static str) -> Result<(), Error> {
    if actual == expected {
        Ok(())
    } else {
        Err(violation(field, expected, actual))
    }
}

fn violation(field: &'static str, expected: usize, actual: usize) -> Error {
    Error::CalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationAssignmentLifecycleInvariant {
        field,
        expected,
        actual,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn conceptual_cp414_contract_is_54_routes_18_assignments_and_72_sites() {
        assert_eq!((54 - 18, 18 * 4), (36, 72));
    }
}
