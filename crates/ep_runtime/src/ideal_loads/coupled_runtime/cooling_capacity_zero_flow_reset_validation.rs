//! Release validation for the bounded cooling capacity-zero candidate reset.

use ep_model::IdealLoadsLimit;

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
    PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_SOURCE_ORDER,
    PurchasedAirCalcCoolingCapacityZeroFlowResetLifecycleSummary,
    PurchasedAirCalcCoolingHumidificationFlowLifecycleSummary,
};

use super::super::calc::cooling_capacity_zero_flow_reset_snapshot_is_exact_direct_release;
use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

pub(super) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let sensible = output.calculation_cooling_sensible_flow;
    let dehumidification = output.calculation_cooling_dehumidification_flow;
    let humidification = output.calculation_cooling_humidification_flow;
    let reset = output.calculation_cooling_capacity_zero_flow_reset;

    reset.source == PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_SOURCE
        && reset.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_FIRST_EXCLUDED_SOURCE
        && reset.source_order == PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_SOURCE_ORDER
        && reset.system == binding.ideal_loads_air_system
        && reset.parent_call_ordinal == call_ordinal
        && reset.controlled_zone == binding.zone
        && reset.system == humidification.system
        && reset.parent_call_ordinal == humidification.parent_call_ordinal
        && reset.controlled_zone == humidification.controlled_zone
        && reset.unit_body_entered == humidification.unit_body_entered
        && reset.predecessor_cooling_body_entered == humidification.cooling_body_entered
        && reset.unit_off_skipped == humidification.unit_off_skipped
        && reset.non_cooling_skipped == humidification.non_cooling_skipped
        && reset.cooling_body_entered == humidification.cooling_body_entered
        && same_option(
            reset.predecessor_supply_mass_flow_rate_for_cool_kg_per_s,
            sensible.resulting_supply_mass_flow_rate_for_cool_kg_per_s,
        )
        && same_option(
            reset.predecessor_supply_mass_flow_rate_for_dehumidification_kg_per_s,
            dehumidification.resulting_supply_mass_flow_rate_for_dehumidification_kg_per_s,
        )
        && same_option(
            reset.predecessor_supply_mass_flow_rate_for_humidification_kg_per_s,
            humidification.resulting_supply_mass_flow_rate_for_humidification_kg_per_s,
        )
        && cooling_capacity_zero_flow_reset_snapshot_is_exact_direct_release(reset)
}

pub(super) fn validate_lifecycle(
    lifecycle: &PurchasedAirCalcCoolingCapacityZeroFlowResetLifecycleSummary,
    predecessor_lifecycle: &PurchasedAirCalcCoolingHumidificationFlowLifecycleSummary,
    timestep_count: usize,
    numerical_cooling_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), Error> {
    let state = &lifecycle.state;
    let predecessor = &predecessor_lifecycle.state;
    let skipped = checked_add(
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        "skip_partition_overflow",
        timestep_count,
    )?;
    let partition = checked_add(
        skipped,
        state.cooling_body_entry_count,
        "transition_partition_overflow",
        timestep_count,
    )?;
    let selected_capacity_count = checked_add(
        state.cooling_limit_capacity_count,
        state.cooling_limit_flow_rate_and_capacity_count,
        "selected_capacity_partition_overflow",
        state.maximum_total_cooling_capacity_read_count,
    )?;
    let second_partition = checked_add(
        state.cooling_limit_flow_rate_and_capacity_count,
        state.cooling_limit_rejected_count,
        "second_selector_partition_overflow",
        state.second_cooling_limit_read_count,
    )?;
    let capacity_partition = checked_add(
        state.maximum_total_cooling_capacity_zero_count,
        state.maximum_total_cooling_capacity_nonzero_count,
        "capacity_result_partition_overflow",
        state.maximum_total_cooling_capacity_comparison_count,
    )?;

    let capacity_matches =
        usize::from(binding.system.cooling_limit == IdealLoadsLimit::LimitCapacity)
            * state.cooling_body_entry_count;
    let combined_matches =
        usize::from(binding.system.cooling_limit == IdealLoadsLimit::LimitFlowRateAndCapacity)
            * state.cooling_body_entry_count;

    for (field, expected, actual) in [
        ("transition_count", timestep_count, state.transition_count),
        (
            "predecessor_transition_count",
            predecessor.transition_count,
            state.transition_count,
        ),
        ("transition_partition", state.transition_count, partition),
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
            "cooling_body_entry_count",
            numerical_cooling_count,
            state.cooling_body_entry_count,
        ),
        (
            "first_cooling_limit_read_count",
            state.cooling_body_entry_count,
            state.first_cooling_limit_read_count,
        ),
        (
            "cooling_limit_capacity_count",
            capacity_matches,
            state.cooling_limit_capacity_count,
        ),
        (
            "second_cooling_limit_read_count",
            state.cooling_body_entry_count - capacity_matches,
            state.second_cooling_limit_read_count,
        ),
        (
            "cooling_limit_flow_rate_and_capacity_count",
            combined_matches,
            state.cooling_limit_flow_rate_and_capacity_count,
        ),
        (
            "second_selector_partition",
            state.second_cooling_limit_read_count,
            second_partition,
        ),
        (
            "selected_capacity_partition",
            state.maximum_total_cooling_capacity_read_count,
            selected_capacity_count,
        ),
        (
            "maximum_total_cooling_capacity_comparison_count",
            state.maximum_total_cooling_capacity_read_count,
            state.maximum_total_cooling_capacity_comparison_count,
        ),
        (
            "capacity_result_partition",
            state.maximum_total_cooling_capacity_comparison_count,
            capacity_partition,
        ),
        (
            "zero_cooling_capacity_body_entry_count",
            state.maximum_total_cooling_capacity_zero_count,
            state.zero_cooling_capacity_body_entry_count,
        ),
        (
            "cool_zero_assignment_count",
            state.zero_cooling_capacity_body_entry_count,
            state.supply_mass_flow_rate_for_cool_zero_assignment_count,
        ),
        (
            "dehumidification_zero_assignment_count",
            state.zero_cooling_capacity_body_entry_count,
            state.supply_mass_flow_rate_for_dehumidification_zero_assignment_count,
        ),
        (
            "humidification_zero_assignment_count",
            state.zero_cooling_capacity_body_entry_count,
            state.supply_mass_flow_rate_for_humidification_zero_assignment_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }

    let latest = state
        .latest
        .as_ref()
        .ok_or_else(|| violation("latest_release_snapshot_ready", 1, 0))?;
    if lifecycle.source != PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_FIRST_EXCLUDED_SOURCE
        || state.system != binding.ideal_loads_air_system
        || latest != &latest_output.calculation_cooling_capacity_zero_flow_reset
        || !snapshot_matches_release(latest_output, timestep_count, binding)
    {
        return Err(violation("latest_release_snapshot_ready", 1, 0));
    }
    Ok(())
}

pub(super) fn checked_add(
    left: usize,
    right: usize,
    field: &'static str,
    expected: usize,
) -> Result<usize, Error> {
    left.checked_add(right)
        .ok_or_else(|| violation(field, expected, usize::MAX))
}

fn same_option(left: Option<f64>, right: Option<f64>) -> bool {
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
    Error::CalcCoolingCapacityZeroFlowResetLifecycleInvariant {
        field,
        expected,
        actual,
    }
}
