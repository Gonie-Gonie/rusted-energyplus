//! Release validation for the bounded cooling supply mass-flow maximum.

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_MAXIMUM_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_MAXIMUM_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_MAXIMUM_SOURCE_ORDER,
    PurchasedAirCalcCoolingCapacityZeroFlowResetLifecycleSummary,
    PurchasedAirCalcCoolingSupplyMassFlowMaximumLifecycleSummary,
    PurchasedAirCalcMinimumOaPrefixLifecycleSummary,
};

use super::super::calc::cooling_supply_mass_flow_maximum_snapshot_is_exact_direct_release;
use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

pub(super) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let minimum_oa = output.calculation_minimum_outdoor_air;
    let predecessor = output.calculation_cooling_capacity_zero_flow_reset;
    let maximum = output.calculation_cooling_supply_mass_flow_maximum;

    maximum.source == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_MAXIMUM_SOURCE
        && maximum.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_MAXIMUM_FIRST_EXCLUDED_SOURCE
        && maximum.source_order == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_MAXIMUM_SOURCE_ORDER
        && maximum.system == binding.ideal_loads_air_system
        && maximum.parent_call_ordinal == call_ordinal
        && maximum.controlled_zone == binding.zone
        && maximum.system == predecessor.system
        && maximum.parent_call_ordinal == predecessor.parent_call_ordinal
        && maximum.controlled_zone == predecessor.controlled_zone
        && maximum.unit_body_entered == predecessor.unit_body_entered
        && maximum.predecessor_cooling_body_entered == predecessor.cooling_body_entered
        && maximum.unit_off_skipped == predecessor.unit_off_skipped
        && maximum.non_cooling_skipped == predecessor.non_cooling_skipped
        && maximum.cooling_body_entered == predecessor.cooling_body_entered
        && same_option(
            maximum.outdoor_air_mass_flow_rate_kg_per_s,
            minimum_oa
                .working_outdoor_air_mass_flow_rate_kg_per_s
                .filter(|_| maximum.cooling_body_entered),
        )
        && same_option(
            maximum.supply_mass_flow_rate_for_cool_kg_per_s,
            predecessor.resulting_supply_mass_flow_rate_for_cool_kg_per_s,
        )
        && same_option(
            maximum.supply_mass_flow_rate_for_dehumidification_kg_per_s,
            predecessor.resulting_supply_mass_flow_rate_for_dehumidification_kg_per_s,
        )
        && same_option(
            maximum.supply_mass_flow_rate_for_humidification_kg_per_s,
            predecessor.resulting_supply_mass_flow_rate_for_humidification_kg_per_s,
        )
        && cooling_supply_mass_flow_maximum_snapshot_is_exact_direct_release(maximum)
}

#[allow(clippy::too_many_arguments)]
pub(super) fn validate_lifecycle(
    lifecycle: &PurchasedAirCalcCoolingSupplyMassFlowMaximumLifecycleSummary,
    predecessor_lifecycle: &PurchasedAirCalcCoolingCapacityZeroFlowResetLifecycleSummary,
    minimum_oa_lifecycle: &PurchasedAirCalcMinimumOaPrefixLifecycleSummary,
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

    for (field, expected, actual) in [
        ("transition_count", timestep_count, state.transition_count),
        (
            "predecessor_transition_count",
            predecessor.transition_count,
            state.transition_count,
        ),
        (
            "minimum_oa_transition_count",
            minimum_oa_lifecycle.state.transition_count,
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
            "outdoor_air_mass_flow_rate_read_count",
            state.cooling_body_entry_count,
            state.outdoor_air_mass_flow_rate_read_count,
        ),
        (
            "supply_mass_flow_rate_for_cool_read_count",
            state.cooling_body_entry_count,
            state.supply_mass_flow_rate_for_cool_read_count,
        ),
        (
            "supply_mass_flow_rate_for_dehumidification_read_count",
            state.cooling_body_entry_count,
            state.supply_mass_flow_rate_for_dehumidification_read_count,
        ),
        (
            "supply_mass_flow_rate_for_humidification_read_count",
            state.cooling_body_entry_count,
            state.supply_mass_flow_rate_for_humidification_read_count,
        ),
        (
            "positive_zero_vs_outdoor_air_comparison_count",
            state.cooling_body_entry_count,
            state.positive_zero_vs_outdoor_air_comparison_count,
        ),
        (
            "cooling_vs_dehumidification_comparison_count",
            state.cooling_body_entry_count,
            state.cooling_vs_dehumidification_comparison_count,
        ),
        (
            "leading_vs_candidate_pair_comparison_count",
            state.cooling_body_entry_count,
            state.leading_vs_candidate_pair_comparison_count,
        ),
        (
            "leading_vs_humidification_comparison_count",
            state.cooling_body_entry_count,
            state.leading_vs_humidification_comparison_count,
        ),
        (
            "maximum_evaluation_count",
            state.cooling_body_entry_count,
            state.maximum_evaluation_count,
        ),
        (
            "supply_mass_flow_rate_assignment_count",
            state.cooling_body_entry_count,
            state.supply_mass_flow_rate_assignment_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }

    let latest = state
        .latest
        .as_ref()
        .ok_or_else(|| violation("latest_release_snapshot_ready", 1, 0))?;
    if lifecycle.source != PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_MAXIMUM_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_MAXIMUM_FIRST_EXCLUDED_SOURCE
        || state.system != binding.ideal_loads_air_system
        || latest != &latest_output.calculation_cooling_supply_mass_flow_maximum
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
    Error::CalcCoolingSupplyMassFlowMaximumLifecycleInvariant {
        field,
        expected,
        actual,
    }
}
