//! Coupled-runtime validation for CP362.

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_SOURCE_ORDER,
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitLifecycleSummary,
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitLifecycleSummary,
    cooling_humidistat_supply_humidity_ratio_mixed_air_limit_latest_metadata_is_consistent,
    cooling_humidistat_supply_humidity_ratio_mixed_air_limit_snapshot_is_exact_direct_release,
    cooling_humidistat_supply_humidity_ratio_mixed_air_limit_snapshot_links_to_predecessor,
    cooling_humidistat_supply_humidity_ratio_mixed_air_limit_snapshots_match_bit_exact,
};

use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

pub(super) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let predecessor = output
        .calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit;
    let snapshot = output.calculation_cooling_humidistat_supply_humidity_ratio_mixed_air_limit;
    snapshot.system == binding.ideal_loads_air_system
        && snapshot.parent_call_ordinal == call_ordinal
        && snapshot.controlled_zone == binding.zone
        && cooling_humidistat_supply_humidity_ratio_mixed_air_limit_snapshot_is_exact_direct_release(
            snapshot,
        )
        && cooling_humidistat_supply_humidity_ratio_mixed_air_limit_snapshot_links_to_predecessor(
            snapshot,
            predecessor,
        )
}

pub(super) fn validate_lifecycle(
    lifecycle: &PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitLifecycleSummary,
    predecessor: &PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitLifecycleSummary,
    timestep_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), Error> {
    let state = &lifecycle.state;
    let previous = &predecessor.state;
    let active =
        state.dehumidification_control_humidistat_supply_humidity_ratio_mixed_air_limit_count;
    let route_partition = checked_sum(&[
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        state.positive_guard_false_fallthrough_skip_count,
        state.dehumidification_control_none_case_completed_skip_count,
        state.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
        active,
        state.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
    ])?;
    let source_sites = active
        .checked_mul(
            PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_SOURCE_ORDER
                .len(),
        )
        .ok_or_else(|| violation("source_site_execution_count_overflow", usize::MAX, active))?;
    for (field, expected, actual) in [
        ("transition_count", timestep_count, state.transition_count),
        ("transition_partition", state.transition_count, route_partition),
        (
            "predecessor_transition_count",
            previous.transition_count,
            state.transition_count,
        ),
        (
            "unit_off_skip_count",
            previous.unit_off_skip_count,
            state.unit_off_skip_count,
        ),
        (
            "non_cooling_skip_count",
            previous.non_cooling_skip_count,
            state.non_cooling_skip_count,
        ),
        (
            "positive_guard_false_fallthrough_skip_count",
            previous.positive_guard_false_fallthrough_skip_count,
            state.positive_guard_false_fallthrough_skip_count,
        ),
        (
            "none_case_completed_skip_count",
            previous.dehumidification_control_none_case_completed_skip_count,
            state.dehumidification_control_none_case_completed_skip_count,
        ),
        (
            "constant_shr_case_completed_skip_count",
            previous.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
            state.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
        ),
        (
            "humidistat_mixed_air_limit_count",
            previous.dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_count,
            active,
        ),
        (
            "constant_supply_humidity_ratio_case_selected_skip_count",
            previous.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
            state.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
        ),
        (
            "direct_constant_shr_case_completed_skip_count",
            0,
            state.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
        ),
        ("direct_active_count", 0, active),
        (
            "direct_constant_supply_humidity_ratio_case_selected_skip_count",
            0,
            state.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
        ),
        (
            "source_site_execution_count",
            source_sites,
            state.source_site_execution_count,
        ),
        (
            "mixed_air_read_count",
            active,
            state.mixed_air_humidity_ratio_for_minimum_read_count,
        ),
        (
            "local_read_count",
            active,
            state.supply_humidity_ratio_for_dehumidification_for_mixed_air_limit_minimum_read_count,
        ),
        (
            "minimum_count",
            active,
            state.source_shaped_two_argument_minimum_evaluation_count,
        ),
        (
            "assignment_count",
            active,
            state.supply_humidity_ratio_assignment_count,
        ),
    ] {
        if expected != actual {
            return Err(
                Error::CalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitLifecycleInvariant {
                    field,
                    expected,
                    actual,
                },
            );
        }
    }
    let latest = state.latest.ok_or(
        Error::CalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitLifecycleInvariant {
            field: "latest_release_snapshot_ready",
            expected: 1,
            actual: 0,
        },
    )?;
    let predecessor_latest = previous.latest.ok_or(
        Error::CalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitLifecycleInvariant {
            field: "predecessor_latest_release_snapshot_ready",
            expected: 1,
            actual: 0,
        },
    )?;
    if lifecycle.source
        != PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE
        || state.system != binding.ideal_loads_air_system
        || previous.system != binding.ideal_loads_air_system
        || !cooling_humidistat_supply_humidity_ratio_mixed_air_limit_latest_metadata_is_consistent(
            state,
            timestep_count,
        )
        || !super::cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_validation::
            snapshots_match_bit_exact(
                &predecessor_latest,
                &latest_output
                    .calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit,
            )
        || !cooling_humidistat_supply_humidity_ratio_mixed_air_limit_snapshot_links_to_predecessor(
            latest,
            predecessor_latest,
        )
        || !cooling_humidistat_supply_humidity_ratio_mixed_air_limit_snapshots_match_bit_exact(
            latest,
            latest_output.calculation_cooling_humidistat_supply_humidity_ratio_mixed_air_limit,
        )
        || !snapshot_matches_release(latest_output, timestep_count, binding)
    {
        return Err(Error::CalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitLifecycleInvariant {
            field: "latest_release_snapshot_ready",
            expected: 1,
            actual: 0,
        });
    }
    Ok(())
}

fn checked_sum(values: &[usize]) -> Result<usize, Error> {
    values.iter().try_fold(0usize, |sum, value| {
        sum.checked_add(*value)
            .ok_or_else(|| violation("transition_partition_overflow", usize::MAX, *value))
    })
}

fn violation(field: &'static str, expected: usize, actual: usize) -> Error {
    Error::CalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitLifecycleInvariant {
        field,
        expected,
        actual,
    }
}
