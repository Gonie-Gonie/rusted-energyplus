//! Release validation for the bounded cooling sensible-flow calculation.

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
    PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_SOURCE_ORDER,
    PurchasedAirCalcCoolingEconomizerBodyLifecycleSummary,
    PurchasedAirCalcCoolingSensibleFlowLifecycleSummary,
};

use super::super::calc::cooling_sensible_flow_snapshot_is_exact_direct_release;
use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

pub(super) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let predecessor = output.calculation_cooling_economizer_body;
    let flow = output.calculation_cooling_sensible_flow;

    flow.source == PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_SOURCE
        && flow.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_FIRST_EXCLUDED_SOURCE
        && flow.source_order == PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_SOURCE_ORDER
        && predecessor.system == binding.ideal_loads_air_system
        && predecessor.parent_call_ordinal == call_ordinal
        && predecessor.controlled_zone == binding.zone
        && flow.system == predecessor.system
        && flow.parent_call_ordinal == predecessor.parent_call_ordinal
        && flow.controlled_zone == predecessor.controlled_zone
        && flow.unit_body_entered == predecessor.unit_body_entered
        && flow.predecessor_cooling_body_entered == predecessor.predecessor_cooling_body_entered
        && flow.predecessor_maximum_cooling_flow_body_sibling_skipped
            == predecessor.maximum_cooling_flow_body_sibling_skipped
        && flow.predecessor_no_economizer_outer_guard_fallthrough_skipped
            == predecessor.no_economizer_outer_guard_fallthrough_skipped
        && flow.predecessor_economizer_condition_fallthrough_skipped
            == predecessor.economizer_condition_fallthrough_skipped
        && flow.predecessor_economizer_calculation_body_executed
            == predecessor.economizer_calculation_body_executed
        && flow.unit_off_skipped == predecessor.unit_off_skipped
        && flow.non_cooling_skipped == predecessor.non_cooling_skipped
        && flow.cooling_body_entered == predecessor.predecessor_cooling_body_entered
        && cooling_sensible_flow_snapshot_is_exact_direct_release(flow)
}

pub(super) fn validate_lifecycle(
    lifecycle: &PurchasedAirCalcCoolingSensibleFlowLifecycleSummary,
    predecessor_lifecycle: &PurchasedAirCalcCoolingEconomizerBodyLifecycleSummary,
    timestep_count: usize,
    numerical_cooling_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), Error> {
    let state = &lifecycle.state;
    let predecessor = &predecessor_lifecycle.state;
    let skipped_count = checked_add(
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        "skip_partition_overflow",
        timestep_count,
    )?;
    let transition_partition = checked_add(
        skipped_count,
        state.cooling_body_entry_count,
        "transition_partition_overflow",
        timestep_count,
    )?;
    let predecessor_skipped_count = checked_add(
        predecessor.unit_off_skip_count,
        predecessor.non_cooling_skip_count,
        "predecessor_skip_partition_overflow",
        timestep_count,
    )?;
    let predecessor_cooling_count = predecessor
        .transition_count
        .checked_sub(predecessor_skipped_count)
        .ok_or_else(|| {
            violation(
                "predecessor_skip_partition",
                predecessor.transition_count,
                predecessor_skipped_count,
            )
        })?;
    let delta_gate_partition = checked_add(
        state.delta_temperature_comparison_satisfied_count,
        state.delta_temperature_fallthrough_count,
        "delta_temperature_gate_partition_overflow",
        state.delta_temperature_comparison_count,
    )?;
    let cooling_on_partition = checked_add(
        state.cooling_on_body_entry_count,
        state.cooling_on_fallthrough_count,
        "cooling_on_partition_overflow",
        state.cooling_on_read_count,
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
        transition_partition,
        state.transition_count,
        "transition_partition"
    );
    count!(unit_off_skip_count, predecessor.unit_off_skip_count);
    count!(non_cooling_skip_count, predecessor.non_cooling_skip_count);
    count!(cooling_body_entry_count, predecessor_cooling_count);
    count!(cooling_body_entry_count, numerical_cooling_count);
    count!(
        supply_mass_flow_rate_for_cool_reset_assignment_count,
        state.cooling_body_entry_count
    );
    count!(cooling_on_read_count, state.cooling_body_entry_count);
    count!(
        cooling_on_partition,
        state.cooling_on_read_count,
        "cooling_on_partition"
    );
    count!(cooling_on_body_entry_count, state.cooling_body_entry_count);
    count!(cooling_on_fallthrough_count, 0);
    count!(
        zone_humidity_ratio_read_count,
        state.cooling_on_body_entry_count
    );
    count!(
        psychrometric_cp_air_evaluation_count,
        state.cooling_on_body_entry_count
    );
    count!(cp_air_assignment_count, state.cooling_on_body_entry_count);
    count!(
        minimum_cooling_supply_air_temperature_read_count,
        state.cooling_on_body_entry_count
    );
    count!(
        zone_temperature_read_count,
        state.cooling_on_body_entry_count
    );
    count!(
        delta_temperature_calculation_count,
        state.cooling_on_body_entry_count
    );
    count!(
        delta_temperature_assignment_count,
        state.cooling_on_body_entry_count
    );
    count!(
        delta_temperature_for_gate_read_count,
        state.cooling_on_body_entry_count
    );
    count!(
        delta_temperature_comparison_count,
        state.cooling_on_body_entry_count
    );
    count!(
        delta_gate_partition,
        state.delta_temperature_comparison_count,
        "delta_temperature_gate_partition"
    );
    count!(
        delta_temperature_body_entry_count,
        state.delta_temperature_comparison_satisfied_count
    );
    count!(
        zone_cooling_setpoint_load_read_count,
        state.delta_temperature_body_entry_count
    );
    count!(
        cp_air_for_first_division_read_count,
        state.delta_temperature_body_entry_count
    );
    count!(
        zone_cooling_setpoint_load_over_cp_air_calculation_count,
        state.delta_temperature_body_entry_count
    );
    count!(
        delta_temperature_for_second_division_read_count,
        state.delta_temperature_body_entry_count
    );
    count!(
        supply_mass_flow_rate_for_cool_calculation_count,
        state.delta_temperature_body_entry_count
    );
    count!(
        supply_mass_flow_rate_for_cool_assignment_count,
        state.delta_temperature_body_entry_count
    );

    let latest = state
        .latest
        .as_ref()
        .ok_or_else(|| violation("latest_release_snapshot_ready", 1, 0))?;
    if lifecycle.source != PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_FIRST_EXCLUDED_SOURCE
        || state.system != binding.ideal_loads_air_system
        || latest != &latest_output.calculation_cooling_sensible_flow
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
    Error::CalcCoolingSensibleFlowLifecycleInvariant {
        field,
        expected,
        actual,
    }
}
