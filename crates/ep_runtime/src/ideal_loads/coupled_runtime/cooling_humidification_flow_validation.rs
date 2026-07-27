//! Release validation for the bounded cooling humidification-flow calculation.

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
    PURCHASED_AIR_CALC_COOLING_HUMIDIFICATION_FLOW_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_HUMIDIFICATION_FLOW_SOURCE,
    PURCHASED_AIR_CALC_COOLING_HUMIDIFICATION_FLOW_SOURCE_ORDER,
    PurchasedAirCalcCoolingDehumidificationFlowLifecycleSummary,
    PurchasedAirCalcCoolingHumidificationFlowLifecycleSummary,
};

use super::super::calc::cooling_humidification_flow_snapshot_is_exact_direct_release;
use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

pub(super) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let predecessor = output.calculation_cooling_dehumidification_flow;
    let flow = output.calculation_cooling_humidification_flow;

    flow.source == PURCHASED_AIR_CALC_COOLING_HUMIDIFICATION_FLOW_SOURCE
        && flow.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_HUMIDIFICATION_FLOW_FIRST_EXCLUDED_SOURCE
        && flow.source_order == PURCHASED_AIR_CALC_COOLING_HUMIDIFICATION_FLOW_SOURCE_ORDER
        && predecessor.system == binding.ideal_loads_air_system
        && predecessor.parent_call_ordinal == call_ordinal
        && predecessor.controlled_zone == binding.zone
        && flow.system == predecessor.system
        && flow.parent_call_ordinal == predecessor.parent_call_ordinal
        && flow.controlled_zone == predecessor.controlled_zone
        && flow.unit_body_entered == predecessor.unit_body_entered
        && flow.predecessor_cooling_body_entered == predecessor.cooling_body_entered
        && flow.unit_off_skipped == predecessor.unit_off_skipped
        && flow.non_cooling_skipped == predecessor.non_cooling_skipped
        && flow.cooling_body_entered == predecessor.cooling_body_entered
        && cooling_humidification_flow_snapshot_is_exact_direct_release(flow)
}

pub(super) fn validate_lifecycle(
    lifecycle: &PurchasedAirCalcCoolingHumidificationFlowLifecycleSummary,
    predecessor_lifecycle: &PurchasedAirCalcCoolingDehumidificationFlowLifecycleSummary,
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
    let heating_on_partition = checked_add(
        state.heating_on_body_entry_count,
        state.heating_on_fallthrough_count,
        "heating_on_partition_overflow",
        state.heating_on_read_count,
    )?;
    let humidification_control_partition = checked_add(
        state.humidification_control_type_humidistat_count,
        state.humidification_control_type_fallthrough_count,
        "humidification_control_partition_overflow",
        state.humidification_control_type_read_count,
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
    count!(reset_assignment_count, state.cooling_body_entry_count);
    count!(heating_on_read_count, state.cooling_body_entry_count);
    count!(
        heating_on_partition,
        state.heating_on_read_count,
        "heating_on_partition"
    );
    count!(heating_on_body_entry_count, state.cooling_body_entry_count);
    count!(heating_on_fallthrough_count, 0);
    count!(
        humidification_control_type_read_count,
        state.heating_on_body_entry_count
    );
    count!(
        humidification_control_partition,
        state.humidification_control_type_read_count,
        "humidification_control_partition"
    );
    count!(humidification_control_type_humidistat_count, 0);
    count!(
        humidification_control_type_fallthrough_count,
        state.humidification_control_type_read_count
    );

    count!(humidification_control_body_entry_count, 0);
    count!(dehumidification_control_type_first_read_count, 0);
    count!(dehumidification_control_type_humidistat_count, 0);
    count!(dehumidification_control_type_second_read_count, 0);
    count!(dehumidification_control_type_none_count, 0);
    count!(dehumidification_control_type_rejected_count, 0);
    count!(admitted_control_body_entry_count, 0);
    count!(moisture_demand_read_count, 0);
    count!(moisture_demand_assignment_count, 0);
    count!(maximum_heating_supply_humidity_ratio_read_count, 0);
    count!(zone_humidity_ratio_read_count, 0);
    count!(delta_calculation_count, 0);
    count!(delta_assignment_count, 0);
    count!(delta_gate_read_count, 0);
    count!(delta_comparison_count, 0);
    count!(delta_comparison_satisfied_count, 0);
    count!(delta_fallthrough_count, 0);
    count!(moisture_demand_gate_read_count, 0);
    count!(moisture_demand_comparison_count, 0);
    count!(moisture_demand_comparison_satisfied_count, 0);
    count!(moisture_demand_fallthrough_count, 0);
    count!(humidification_flow_body_entry_count, 0);
    count!(moisture_demand_division_read_count, 0);
    count!(delta_division_read_count, 0);
    count!(calculation_count, 0);
    count!(assignment_count, 0);

    let latest = state
        .latest
        .as_ref()
        .ok_or_else(|| violation("latest_release_snapshot_ready", 1, 0))?;
    if lifecycle.source != PURCHASED_AIR_CALC_COOLING_HUMIDIFICATION_FLOW_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_HUMIDIFICATION_FLOW_FIRST_EXCLUDED_SOURCE
        || state.system != binding.ideal_loads_air_system
        || latest != &latest_output.calculation_cooling_humidification_flow
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
    Error::CalcCoolingHumidificationFlowLifecycleInvariant {
        field,
        expected,
        actual,
    }
}
