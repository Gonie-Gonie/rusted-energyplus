//! Release validation for the bounded Humidistat local minimum limit.

use ep_model::DehumidificationControlType;

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_MINIMUM_LIMIT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_MINIMUM_LIMIT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_MINIMUM_LIMIT_SOURCE_ORDER,
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentRuntimeState,
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitLifecycleSummary,
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitRuntimeState,
    cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_snapshot_is_exact_direct_release,
    cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_snapshot_is_exact_direct_release,
};

use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

mod snapshot_validation;
pub(super) use snapshot_validation::{expected_snapshot, snapshots_match_bit_exact};

pub(super) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let predecessor =
        output.calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment;
    let snapshot = output
        .calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit;
    snapshot.system == binding.ideal_loads_air_system
        && snapshot.parent_call_ordinal == call_ordinal
        && snapshot.controlled_zone == binding.zone
        && cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_snapshot_is_exact_direct_release(
            snapshot,
        )
        && snapshots_match_bit_exact(&snapshot, &expected_snapshot(predecessor))
}

pub(super) fn validate_lifecycle(
    lifecycle: &PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitLifecycleSummary,
    predecessor_lifecycle: &PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentLifecycleSummary,
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
            != PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_MINIMUM_LIMIT_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_MINIMUM_LIMIT_FIRST_EXCLUDED_SOURCE
        || predecessor_lifecycle.source
            != PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_ASSIGNMENT_SOURCE
        || predecessor_lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_MINIMUM_LIMIT_SOURCE_ORDER.len()
            != 4
        || state.system != binding.ideal_loads_air_system
        || predecessor.system != binding.ideal_loads_air_system
        || !cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_snapshot_is_exact_direct_release(
            *predecessor_latest,
        )
        || !super::cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_validation::
            snapshots_match_bit_exact(
                predecessor_latest,
                &latest_output
                    .calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment,
            )
        || !cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_snapshot_is_exact_direct_release(
            *latest,
        )
        || !snapshots_match_bit_exact(latest, &expected_snapshot(*predecessor_latest))
        || !snapshots_match_bit_exact(
            latest,
            &latest_output
                .calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit,
        )
        || !snapshot_matches_release(latest_output, timestep_count, binding)
    {
        return Err(violation("latest_release_snapshot_ready", 1, 0));
    }
    Ok(())
}

fn validate_counts(
    state: &PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitRuntimeState,
    predecessor: &PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentRuntimeState,
    timestep_count: usize,
) -> Result<(), Error> {
    let humidistat = state
        .dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_count;
    validate_route_partition(state)?;
    validate_source_counters(state)?;
    for (field, expected, actual) in [
        ("transition_count", timestep_count, state.transition_count),
        (
            "predecessor_transition_count",
            predecessor.transition_count,
            state.transition_count,
        ),
        ("unit_off_skip_count", predecessor.unit_off_skip_count, state.unit_off_skip_count),
        ("non_cooling_skip_count", predecessor.non_cooling_skip_count, state.non_cooling_skip_count),
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
            "constant_shr_case_completed_skip_count",
            predecessor
                .dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
            state.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
        ),
        (
            "humidistat_minimum_limit_count",
            predecessor
                .dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_assignment_count,
            humidistat,
        ),
        (
            "constant_supply_humidity_ratio_case_selected_skip_count",
            predecessor
                .dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
            state.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
        ),
        (
            "direct_constant_shr_case_completed_skip_count",
            0,
            state.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
        ),
        ("direct_humidistat_minimum_limit_count", 0, humidistat),
        (
            "direct_constant_supply_humidity_ratio_case_selected_skip_count",
            0,
            state.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
        ),
        ("direct_source_site_execution_count", 0, state.source_site_execution_count),
    ] {
        ensure_count(actual, expected, field)?;
    }
    Ok(())
}

fn validate_route_partition(
    state: &PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitRuntimeState,
) -> Result<(), Error> {
    let partition = checked_sum(&[
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        state.positive_guard_false_fallthrough_skip_count,
        state.dehumidification_control_none_case_completed_skip_count,
        state.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
        state
            .dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_count,
        state.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
    ])?;
    ensure_count(partition, state.transition_count, "transition_partition")
}

fn validate_source_counters(
    state: &PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitRuntimeState,
) -> Result<(), Error> {
    let assignments = state
        .dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_count;
    let source_sites = assignments
        .checked_mul(
            PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_MINIMUM_LIMIT_SOURCE_ORDER
                .len(),
        )
        .ok_or_else(|| violation("source_site_execution_count_overflow", usize::MAX, assignments))?;
    for (field, expected, actual) in [
        (
            "source_site_execution_count",
            source_sites,
            state.source_site_execution_count,
        ),
        (
            "supply_humidity_ratio_for_dehumidification_for_minimum_limit_maximum_read_count",
            assignments,
            state.supply_humidity_ratio_for_dehumidification_for_minimum_limit_maximum_read_count,
        ),
        (
            "minimum_cooling_supply_air_humidity_ratio_for_maximum_read_count",
            assignments,
            state.minimum_cooling_supply_air_humidity_ratio_for_maximum_read_count,
        ),
        (
            "source_shaped_two_argument_maximum_evaluation_count",
            assignments,
            state.source_shaped_two_argument_maximum_evaluation_count,
        ),
        (
            "supply_humidity_ratio_for_dehumidification_assignment_count",
            assignments,
            state.supply_humidity_ratio_for_dehumidification_assignment_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }
    Ok(())
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
    Error::CalcCoolingHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitLifecycleInvariant {
        field,
        expected,
        actual,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ep_model::IdealLoadsAirSystemId;

    #[test]
    fn partition_overflow_and_four_site_counter_corruption_fail_closed() {
        let system = IdealLoadsAirSystemId(0);
        let mut state =
            PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitRuntimeState::new(system);
        state.unit_off_skip_count = usize::MAX;
        state.non_cooling_skip_count = 1;
        assert!(validate_route_partition(&state).is_err());

        let mut active =
            PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitRuntimeState::new(system);
        active.dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_count = 1;
        active.source_site_execution_count = 4;
        active.supply_humidity_ratio_for_dehumidification_for_minimum_limit_maximum_read_count = 1;
        active.minimum_cooling_supply_air_humidity_ratio_for_maximum_read_count = 1;
        active.source_shaped_two_argument_maximum_evaluation_count = 1;
        active.supply_humidity_ratio_for_dehumidification_assignment_count = 1;
        assert!(validate_source_counters(&active).is_ok());
        active.source_site_execution_count = 3;
        assert!(validate_source_counters(&active).is_err());
    }
}
