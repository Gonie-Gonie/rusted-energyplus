//! Release validation for the bounded cooling economizer true body.

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
    PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_SOURCE,
    PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_SOURCE_ORDER,
    PurchasedAirCalcCoolingEconomizerBodyLifecycleSummary,
    PurchasedAirCalcCoolingEconomizerConditionLifecycleSummary,
};

use super::super::calc::cooling_economizer_body_snapshot_is_exact_direct_release;
use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

struct ExpectedBodyProvenance {
    source: &'static str,
    first_excluded_source: &'static str,
    source_order: &'static [&'static str],
}

pub(super) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let predecessor = output.calculation_cooling_economizer_condition;
    let body = output.calculation_cooling_economizer_body;
    let expected_provenance = ExpectedBodyProvenance {
        source: PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_SOURCE_ORDER,
    };

    body.source == expected_provenance.source
        && body.first_excluded_source == expected_provenance.first_excluded_source
        && body.source_order == expected_provenance.source_order
        && predecessor.system == binding.ideal_loads_air_system
        && predecessor.parent_call_ordinal == call_ordinal
        && predecessor.controlled_zone == binding.zone
        && body.system == predecessor.system
        && body.parent_call_ordinal == predecessor.parent_call_ordinal
        && body.controlled_zone == predecessor.controlled_zone
        && body.unit_body_entered == predecessor.unit_body_entered
        && body.predecessor_cooling_body_entered == predecessor.predecessor_cooling_body_entered
        && body.predecessor_maximum_cooling_flow_body_entered
            == predecessor.predecessor_maximum_cooling_flow_body_entered
        && body.predecessor_active_guard_false_economizer_fallthrough
            == predecessor.predecessor_active_guard_false_economizer_fallthrough
        && body.predecessor_economizer_guard_evaluated
            == predecessor.predecessor_economizer_guard_evaluated
        && body.predecessor_economizer_body_entered
            == predecessor.predecessor_economizer_body_entered
        && body.predecessor_no_economizer_fallthrough
            == predecessor.predecessor_no_economizer_fallthrough
        && body.predecessor_economizer_condition_evaluated
            == predecessor.economizer_condition_evaluated
        && body.predecessor_economizer_condition_satisfied
            == predecessor.economizer_condition_satisfied
        && body.predecessor_economizer_calculation_body_entered
            == predecessor.economizer_calculation_body_entered
        && body.unit_off_skipped == predecessor.unit_off_skipped
        && body.non_cooling_skipped == predecessor.non_cooling_skipped
        && body.maximum_cooling_flow_body_sibling_skipped
            == predecessor.maximum_cooling_flow_body_sibling_skipped
        && body.no_economizer_outer_guard_fallthrough_skipped
            == predecessor.no_economizer_outer_guard_fallthrough_skipped
        && body.economizer_condition_fallthrough_skipped
            == predecessor.economizer_condition_fallthrough
        && cooling_economizer_body_snapshot_is_exact_direct_release(body)
}

pub(super) fn validate_lifecycle(
    lifecycle: &PurchasedAirCalcCoolingEconomizerBodyLifecycleSummary,
    predecessor_lifecycle: &PurchasedAirCalcCoolingEconomizerConditionLifecycleSummary,
    timestep_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), Error> {
    let state = &lifecycle.state;
    let predecessor = &predecessor_lifecycle.state;
    let skip_partition = checked_add(
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        "skip_partition_overflow",
        timestep_count,
    )
    .and_then(|partial| {
        checked_add(
            partial,
            state.maximum_cooling_flow_body_sibling_skip_count,
            "skip_partition_overflow",
            timestep_count,
        )
    })
    .and_then(|partial| {
        checked_add(
            partial,
            state.no_economizer_outer_guard_fallthrough_skip_count,
            "skip_partition_overflow",
            timestep_count,
        )
    })
    .and_then(|partial| {
        checked_add(
            partial,
            state.economizer_condition_fallthrough_skip_count,
            "skip_partition_overflow",
            timestep_count,
        )
    })?;
    let transition_partition = checked_add(
        state.body_execution_count,
        skip_partition,
        "transition_partition_overflow",
        timestep_count,
    )?;

    macro_rules! count {
        ($field:ident, $expected:expr) => {
            ensure_count(state.$field, $expected, stringify!($field))?
        };
        ($actual:expr, $expected:expr, $field:literal) => {
            ensure_count($actual, $expected, $field)?
        };
    }

    count!(transition_count, timestep_count);
    count!(
        state.transition_count,
        predecessor.transition_count,
        "predecessor_transition_count"
    );
    count!(
        body_execution_count,
        predecessor.economizer_calculation_body_entry_count
    );
    count!(body_execution_count, 0);
    count!(unit_off_skip_count, predecessor.unit_off_skip_count);
    count!(non_cooling_skip_count, predecessor.non_cooling_skip_count);
    count!(
        maximum_cooling_flow_body_sibling_skip_count,
        predecessor.maximum_cooling_flow_body_sibling_skip_count
    );
    count!(maximum_cooling_flow_body_sibling_skip_count, 0);
    count!(
        no_economizer_outer_guard_fallthrough_skip_count,
        predecessor.no_economizer_outer_guard_fallthrough_skip_count
    );
    count!(
        economizer_condition_fallthrough_skip_count,
        predecessor.economizer_condition_fallthrough_count
    );
    count!(economizer_condition_fallthrough_skip_count, 0);
    count!(zone_humidity_ratio_read_count, 0);
    count!(psychrometric_cp_air_evaluation_count, 0);
    count!(cp_air_assignment_count, 0);
    count!(outdoor_air_temperature_read_count, 0);
    count!(zone_temperature_read_count, 0);
    count!(delta_temperature_calculation_count, 0);
    count!(delta_temperature_assignment_count, 0);
    count!(delta_temperature_for_gate_read_count, 0);
    count!(delta_temperature_comparison_count, 0);
    count!(delta_temperature_comparison_satisfied_count, 0);
    count!(delta_temperature_body_entry_count, 0);
    count!(delta_temperature_fallthrough_count, 0);
    count!(zone_cooling_setpoint_load_read_count, 0);
    count!(cp_air_for_first_division_read_count, 0);
    count!(zone_cooling_setpoint_load_over_cp_air_calculation_count, 0);
    count!(delta_temperature_for_second_division_read_count, 0);
    count!(supply_mass_flow_rate_calculation_count, 0);
    count!(initial_supply_mass_flow_rate_assignment_count, 0);
    count!(cooling_limit_flow_rate_read_count, 0);
    count!(cooling_limit_flow_rate_comparison_count, 0);
    count!(cooling_limit_flow_rate_match_count, 0);
    count!(cooling_limit_flow_rate_and_capacity_comparison_count, 0);
    count!(cooling_limit_flow_rate_and_capacity_read_count, 0);
    count!(cooling_limit_flow_rate_and_capacity_match_count, 0);
    count!(maximum_cooling_air_mass_flow_rate_read_count, 0);
    count!(
        maximum_cooling_air_mass_flow_rate_positive_comparison_count,
        0
    );
    count!(maximum_cooling_air_mass_flow_rate_positive_count, 0);
    count!(maximum_flow_clamp_body_entry_count, 0);
    count!(supply_mass_flow_rate_for_clamp_read_count, 0);
    count!(inner_max_evaluation_count, 0);
    count!(
        maximum_cooling_air_mass_flow_rate_clamp_upper_bound_read_count,
        0
    );
    count!(supply_mass_flow_rate_clamp_count, 0);
    count!(outer_min_evaluation_count, 0);
    count!(clamped_supply_mass_flow_rate_assignment_count, 0);
    count!(resulting_supply_mass_flow_rate_read_count, 0);
    count!(outdoor_air_mass_flow_rate_read_count, 0);
    count!(supply_above_outdoor_air_mass_flow_comparison_count, 0);
    count!(
        supply_above_outdoor_air_mass_flow_comparison_satisfied_count,
        0
    );
    count!(economizer_activation_body_entry_count, 0);
    count!(outdoor_air_mass_flow_comparison_fallthrough_count, 0);
    count!(economizer_on_assignment_count, 0);
    count!(
        supply_mass_flow_rate_for_outdoor_air_assignment_read_count,
        0
    );
    count!(outdoor_air_mass_flow_rate_assignment_count, 0);
    count!(system_time_step_read_count, 0);
    count!(economizer_active_time_assignment_count, 0);
    count!(skip_partition, state.transition_count, "skip_partition");
    count!(
        transition_partition,
        state.transition_count,
        "transition_partition"
    );

    let latest = state
        .latest
        .as_ref()
        .ok_or_else(|| violation("latest_release_snapshot_ready", 1, 0))?;
    if lifecycle.source != PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_FIRST_EXCLUDED_SOURCE
        || state.system != binding.ideal_loads_air_system
        || latest != &latest_output.calculation_cooling_economizer_body
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

fn ensure_count(actual: usize, expected: usize, field: &'static str) -> Result<(), Error> {
    if actual == expected {
        Ok(())
    } else {
        Err(violation(field, expected, actual))
    }
}

fn violation(field: &'static str, expected: usize, actual: usize) -> Error {
    Error::CalcCoolingEconomizerBodyLifecycleInvariant {
        field,
        expected,
        actual,
    }
}
