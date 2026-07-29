//! Release validation for the bounded constant-supply-humidity-ratio assignment.

use ep_model::DehumidificationControlType;

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_ENTRY_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_ENTRY_SOURCE,
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentRuntimeState,
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentSnapshot,
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntryLifecycleSummary,
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntryRuntimeState,
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntrySnapshot,
    cooling_constant_supply_humidity_ratio_assignment_latest_metadata_is_consistent,
    cooling_constant_supply_humidity_ratio_assignment_snapshot_is_exact_direct_release,
    cooling_constant_supply_humidity_ratio_assignment_snapshot_links_to_predecessor,
    cooling_constant_supply_humidity_ratio_case_entry_snapshot_is_exact_direct_release,
};

use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

pub(super) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let predecessor = output.calculation_cooling_constant_supply_humidity_ratio_case_entry;
    let snapshot = output.calculation_cooling_constant_supply_humidity_ratio_assignment;
    snapshot.system == binding.ideal_loads_air_system
        && snapshot.parent_call_ordinal == call_ordinal
        && snapshot.controlled_zone == binding.zone
        && cooling_constant_supply_humidity_ratio_assignment_snapshot_is_exact_direct_release(
            snapshot,
        )
        && cooling_constant_supply_humidity_ratio_assignment_snapshot_links_to_predecessor(
            snapshot,
            predecessor,
        )
        && snapshots_match_bit_exact(&snapshot, &expected_snapshot(predecessor))
}

pub(super) fn validate_lifecycle(
    lifecycle: &PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentLifecycleSummary,
    predecessor_lifecycle: &PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntryLifecycleSummary,
    timestep_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), Error> {
    let state = &lifecycle.state;
    let predecessor = &predecessor_lifecycle.state;
    validate_counts(state, predecessor, timestep_count)?;

    let latest = state
        .latest
        .as_ref()
        .ok_or_else(|| violation("latest_release_snapshot_ready", 1, 0))?;
    let predecessor_latest = predecessor
        .latest
        .as_ref()
        .ok_or_else(|| violation("predecessor_latest_release_snapshot_ready", 1, 0))?;
    if binding.system.dehumidification_control_type != DehumidificationControlType::None
        || lifecycle.source
            != PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || predecessor_lifecycle.source
            != PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_ENTRY_SOURCE
        || predecessor_lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_ENTRY_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE_ORDER.len()
            != 2
        || state.system != binding.ideal_loads_air_system
        || predecessor.system != binding.ideal_loads_air_system
        || !cooling_constant_supply_humidity_ratio_assignment_latest_metadata_is_consistent(
            state,
            timestep_count,
        )
        || !cooling_constant_supply_humidity_ratio_case_entry_snapshot_is_exact_direct_release(
            *predecessor_latest,
        )
        || *predecessor_latest
            != latest_output.calculation_cooling_constant_supply_humidity_ratio_case_entry
        || !cooling_constant_supply_humidity_ratio_assignment_snapshot_is_exact_direct_release(
            *latest,
        )
        || !cooling_constant_supply_humidity_ratio_assignment_snapshot_links_to_predecessor(
            *latest,
            *predecessor_latest,
        )
        || !snapshots_match_bit_exact(latest, &expected_snapshot(*predecessor_latest))
        || !snapshots_match_bit_exact(
            latest,
            &latest_output.calculation_cooling_constant_supply_humidity_ratio_assignment,
        )
        || !snapshot_matches_release(latest_output, timestep_count, binding)
    {
        return Err(violation("latest_release_snapshot_ready", 1, 0));
    }
    Ok(())
}

fn validate_counts(
    state: &PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentRuntimeState,
    predecessor: &PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntryRuntimeState,
    timestep_count: usize,
) -> Result<(), Error> {
    let assigned = state.dehumidification_control_constant_supply_humidity_ratio_assignment_count;
    validate_route_partition(state)?;
    validate_source_counters(state)?;
    for (field, expected, actual) in [
        ("transition_count", timestep_count, state.transition_count),
        (
            "predecessor_transition_count",
            predecessor.transition_count,
            state.transition_count,
        ),
        (
            "unit_off_skip_count",
            predecessor.unit_off_skip_count,
            state.unit_off_skip_count,
        ),
        (
            "non_cooling_skip_count",
            predecessor.non_cooling_skip_count,
            state.non_cooling_skip_count,
        ),
        (
            "positive_guard_false_fallthrough_skip_count",
            predecessor.positive_guard_false_fallthrough_skip_count,
            state.positive_guard_false_fallthrough_skip_count,
        ),
        (
            "none_case_completed_skip_count",
            predecessor.dehumidification_control_none_case_completed_skip_count,
            state.dehumidification_control_none_case_completed_skip_count,
        ),
        (
            "constant_sensible_heat_ratio_case_completed_skip_count",
            predecessor
                .dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
            state.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
        ),
        (
            "humidistat_case_completed_skip_count",
            predecessor.dehumidification_control_humidistat_case_completed_skip_count,
            state.dehumidification_control_humidistat_case_completed_skip_count,
        ),
        (
            "constant_supply_humidity_ratio_assignment_count",
            predecessor.dehumidification_control_constant_supply_humidity_ratio_case_entry_count,
            assigned,
        ),
        (
            "direct_constant_sensible_heat_ratio_case_completed_skip_count",
            0,
            state.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
        ),
        (
            "direct_humidistat_case_completed_skip_count",
            0,
            state.dehumidification_control_humidistat_case_completed_skip_count,
        ),
        (
            "direct_constant_supply_humidity_ratio_assignment_count",
            0,
            assigned,
        ),
        (
            "direct_source_site_execution_count",
            0,
            state.source_site_execution_count,
        ),
        (
            "direct_minimum_cooling_supply_air_humidity_ratio_read_count",
            0,
            state.minimum_cooling_supply_air_humidity_ratio_read_count,
        ),
        (
            "direct_supply_humidity_ratio_assignment_count",
            0,
            state.supply_humidity_ratio_assignment_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }
    Ok(())
}

fn validate_route_partition(
    state: &PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentRuntimeState,
) -> Result<(), Error> {
    let partition = checked_sum(&[
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        state.positive_guard_false_fallthrough_skip_count,
        state.dehumidification_control_none_case_completed_skip_count,
        state.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
        state.dehumidification_control_humidistat_case_completed_skip_count,
        state.dehumidification_control_constant_supply_humidity_ratio_assignment_count,
    ])?;
    ensure_count(partition, state.transition_count, "transition_partition")
}

fn validate_source_counters(
    state: &PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentRuntimeState,
) -> Result<(), Error> {
    let assigned = state.dehumidification_control_constant_supply_humidity_ratio_assignment_count;
    let source_sites = assigned
        .checked_mul(
            PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE_ORDER.len(),
        )
        .ok_or_else(|| violation("source_site_execution_count_overflow", usize::MAX, assigned))?;
    for (field, expected, actual) in [
        (
            "source_site_execution_count",
            source_sites,
            state.source_site_execution_count,
        ),
        (
            "minimum_cooling_supply_air_humidity_ratio_read_count",
            assigned,
            state.minimum_cooling_supply_air_humidity_ratio_read_count,
        ),
        (
            "supply_humidity_ratio_assignment_count",
            assigned,
            state.supply_humidity_ratio_assignment_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }
    Ok(())
}

pub(super) fn expected_snapshot(
    predecessor: PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntrySnapshot,
) -> PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentSnapshot {
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE_ORDER,
        system: predecessor.system,
        parent_call_ordinal: predecessor.parent_call_ordinal,
        controlled_zone: predecessor.controlled_zone,
        unit_body_entered: predecessor.unit_body_entered,
        predecessor_cooling_body_entered: predecessor.predecessor_cooling_body_entered,
        predecessor_no_outdoor_air_fallback_entered:
            predecessor.predecessor_no_outdoor_air_fallback_entered,
        predecessor_positive_supply_mass_flow_body_entered:
            predecessor.predecessor_positive_supply_mass_flow_body_entered,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        positive_guard_false_fallthrough_skipped:
            predecessor.positive_guard_false_fallthrough_skipped,
        predecessor_dehumidification_control_type:
            predecessor.predecessor_dehumidification_control_type,
        predecessor_dehumidification_control_none_case_completed_skip:
            predecessor.dehumidification_control_none_case_completed_skip,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip:
            predecessor.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
        predecessor_dehumidification_control_humidistat_case_completed_skip:
            predecessor.dehumidification_control_humidistat_case_completed_skip,
        predecessor_dehumidification_control_constant_supply_humidity_ratio_case_entered:
            predecessor.dehumidification_control_constant_supply_humidity_ratio_case_entered,
        dehumidification_control_none_case_completed_skip:
            predecessor.dehumidification_control_none_case_completed_skip,
        dehumidification_control_constant_sensible_heat_ratio_case_completed_skip:
            predecessor.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
        dehumidification_control_humidistat_case_completed_skip:
            predecessor.dehumidification_control_humidistat_case_completed_skip,
        dehumidification_control_constant_supply_humidity_ratio_assignment_executed:
            predecessor.dehumidification_control_constant_supply_humidity_ratio_case_entered,
        minimum_cooling_supply_air_humidity_ratio_read: false,
        minimum_cooling_supply_air_humidity_ratio: None,
        supply_humidity_ratio_assigned: false,
        assigned_supply_humidity_ratio: None,
        resulting_supply_humidity_ratio: None,
    }
}

pub(super) fn snapshots_match_bit_exact(
    left: &PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentSnapshot,
    right: &PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentSnapshot,
) -> bool {
    let values_match = [
        (
            left.minimum_cooling_supply_air_humidity_ratio,
            right.minimum_cooling_supply_air_humidity_ratio,
        ),
        (
            left.assigned_supply_humidity_ratio,
            right.assigned_supply_humidity_ratio,
        ),
        (
            left.resulting_supply_humidity_ratio,
            right.resulting_supply_humidity_ratio,
        ),
    ]
    .into_iter()
    .all(|(left, right)| option_bits_eq(left, right));
    let mut left = *left;
    let mut right = *right;
    for snapshot in [&mut left, &mut right] {
        snapshot.minimum_cooling_supply_air_humidity_ratio = None;
        snapshot.assigned_supply_humidity_ratio = None;
        snapshot.resulting_supply_humidity_ratio = None;
    }
    values_match && left == right
}

fn option_bits_eq(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}

fn checked_sum(values: &[usize]) -> Result<usize, Error> {
    values.iter().try_fold(0usize, |sum, value| {
        sum.checked_add(*value)
            .ok_or_else(|| violation("transition_partition_overflow", usize::MAX, *value))
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
    Error::CalcCoolingConstantSupplyHumidityRatioAssignmentLifecycleInvariant {
        field,
        expected,
        actual,
    }
}
