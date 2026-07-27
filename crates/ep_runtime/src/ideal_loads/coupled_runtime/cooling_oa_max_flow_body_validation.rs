//! Release validation for the bounded cooling OA maximum-flow body.

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
    PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_RECURRING_WARNING_CHILD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_SOURCE,
    PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_SOURCE_ORDER,
    PurchasedAirCalcCoolingOaMaxFlowBodyLifecycleSummary,
    PurchasedAirCalcCoolingOaMaxFlowBodySnapshot,
    PurchasedAirCalcCoolingOaMaxFlowGateLifecycleSummary,
    PurchasedAirCalcCoolingOaMaxFlowGateSnapshot,
};

use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

pub(super) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let predecessor = output.calculation_cooling_oa_max_flow_gate;
    let body = output.calculation_cooling_oa_max_flow_body;
    let linked = predecessor.system == binding.ideal_loads_air_system
        && predecessor.parent_call_ordinal == call_ordinal
        && predecessor.controlled_zone == binding.zone
        && output.initialization.system == predecessor.system
        && output.initialization.controlled_zone == predecessor.controlled_zone
        && !predecessor.maximum_cooling_flow_body_entered;

    linked && body == expected_snapshot(predecessor, call_ordinal, binding)
}

fn expected_snapshot(
    predecessor: PurchasedAirCalcCoolingOaMaxFlowGateSnapshot,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> PurchasedAirCalcCoolingOaMaxFlowBodySnapshot {
    let unit_off_skipped = !predecessor.unit_body_entered;
    let non_cooling_skipped =
        predecessor.unit_body_entered && !predecessor.predecessor_cooling_body_entered;
    let active_guard_false_economizer_fallthrough =
        predecessor.unit_body_entered && predecessor.predecessor_cooling_body_entered;
    PurchasedAirCalcCoolingOaMaxFlowBodySnapshot {
        source: PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_FIRST_EXCLUDED_SOURCE,
        recurring_warning_child_source:
            PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_RECURRING_WARNING_CHILD_SOURCE,
        system: binding.ideal_loads_air_system,
        parent_call_ordinal: call_ordinal,
        source_order: PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_SOURCE_ORDER,
        controlled_zone: binding.zone,
        unit_body_entered: predecessor.unit_body_entered,
        predecessor_cooling_body_entered: predecessor.predecessor_cooling_body_entered,
        predecessor_maximum_cooling_flow_body_entered: predecessor
            .maximum_cooling_flow_body_entered,
        body_skipped: true,
        unit_off_skipped,
        non_cooling_skipped,
        active_guard_false_economizer_fallthrough,
        outdoor_air_mass_flow_rate_read: false,
        outdoor_air_mass_flow_rate_before_clamp_kg_per_s: None,
        standard_air_density_read: false,
        standard_air_density_kg_per_m3: None,
        outdoor_air_volume_flow_rate_calculated: false,
        outdoor_air_volume_flow_rate_m3_per_s: None,
        warning_counter_read: false,
        warning_counter_before: None,
        first_warning_predicate_satisfied: None,
        first_warning_branch_entered: false,
        warning_counter_incremented: false,
        warning_counter_after: None,
        first_warning_call_site_reached: false,
        maximum_cooling_air_volume_flow_rate_read: false,
        maximum_cooling_air_volume_flow_rate_m3_per_s: None,
        continue_warning_call_site_reached: false,
        continue_warning_timestamp_call_site_reached: false,
        recurring_warning_branch_entered: false,
        recurring_warning_call_site_reached: false,
        recurring_warning_report_maximum_input_m3_per_s: None,
        characterized_recurring_warning_index_allocated_on_call: false,
        characterized_recurring_warning_index_reused_on_call: false,
        characterized_recurring_warning_index_before: None,
        characterized_recurring_warning_index_after: None,
        characterized_recurring_warning_occurrence_ordinal: None,
        characterized_recurring_warning_report_maximum_m3_per_s: None,
        characterized_total_warning_error_incremented: false,
        maximum_cooling_air_mass_flow_rate_read: false,
        maximum_cooling_air_mass_flow_rate_kg_per_s: None,
        outdoor_air_mass_flow_clamp_assignment_performed: false,
        outdoor_air_mass_flow_rate_after_clamp_kg_per_s: None,
    }
}

pub(super) fn validate_lifecycle(
    lifecycle: &PurchasedAirCalcCoolingOaMaxFlowBodyLifecycleSummary,
    predecessor_lifecycle: &PurchasedAirCalcCoolingOaMaxFlowGateLifecycleSummary,
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
        state.body_skip_count,
    )
    .and_then(|partial| {
        checked_add(
            partial,
            state.active_guard_false_economizer_fallthrough_count,
            "skip_partition_overflow",
            state.body_skip_count,
        )
    })?;
    let transition_partition = checked_add(
        state.body_entry_count,
        state.body_skip_count,
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
        body_entry_count,
        predecessor.maximum_cooling_flow_body_entry_count
    );
    count!(body_entry_count, 0);
    count!(body_skip_count, timestep_count);
    count!(unit_off_skip_count, predecessor.unit_off_skip_count);
    count!(non_cooling_skip_count, predecessor.non_cooling_skip_count);
    count!(
        active_guard_false_economizer_fallthrough_count,
        predecessor.active_fallthrough_count
    );
    count!(skip_partition, state.body_skip_count, "skip_partition");
    count!(
        transition_partition,
        state.transition_count,
        "transition_partition"
    );
    count!(outdoor_air_mass_flow_rate_read_count, 0);
    count!(standard_air_density_read_count, 0);
    count!(outdoor_air_volume_flow_calculation_count, 0);
    count!(warning_counter_read_count, 0);
    count!(outdoor_air_flow_max_cooling_output_error_count, 0);
    count!(first_warning_branch_count, 0);
    count!(warning_counter_increment_count, 0);
    count!(first_warning_call_site_count, 0);
    count!(maximum_cooling_air_volume_flow_rate_read_count, 0);
    count!(continue_warning_call_site_count, 0);
    count!(continue_warning_timestamp_call_site_count, 0);
    count!(recurring_warning_branch_count, 0);
    count!(recurring_warning_call_site_count, 0);
    count!(characterized_recurring_warning_index_allocation_count, 0);
    count!(characterized_recurring_warning_index_reuse_count, 0);
    count!(characterized_recurring_warning_occurrence_count, 0);
    count!(
        usize::from(state.characterized_recurring_warning_index_allocated),
        0,
        "characterized_recurring_warning_index_allocated"
    );
    count!(outdoor_air_flow_max_cooling_output_index, 0);
    count!(characterized_total_warning_error_increment_count, 0);
    count!(maximum_cooling_air_mass_flow_rate_read_count, 0);
    count!(outdoor_air_mass_flow_clamp_assignment_count, 0);

    let latest = state
        .latest
        .as_ref()
        .ok_or_else(|| violation("latest_release_snapshot_ready", 1, 0))?;
    if lifecycle.source != PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_FIRST_EXCLUDED_SOURCE
        || lifecycle.recurring_warning_child_source
            != PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_RECURRING_WARNING_CHILD_SOURCE
        || state.system != binding.ideal_loads_air_system
        || state
            .characterized_recurring_warning_report_maximum_m3_per_s
            .is_some()
        || latest != &latest_output.calculation_cooling_oa_max_flow_body
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
    Error::CalcCoolingOaMaxFlowBodyLifecycleInvariant {
        field,
        expected,
        actual,
    }
}
