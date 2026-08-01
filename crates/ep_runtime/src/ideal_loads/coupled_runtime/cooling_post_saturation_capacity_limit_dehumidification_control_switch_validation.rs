//! Coupled-runtime validation for CP386 switch-dispatch evidence.

use ep_model::DehumidificationControlType;

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_FIRST_EXCLUDED_LEXICAL_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlSwitchLifecycleSummary as Lifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlSwitchSnapshot as Snapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentLifecycleSummary as PredecessorLifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentRuntimeState as PredecessorState,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentSnapshot as PredecessorSnapshot,
    cooling_post_saturation_capacity_limit_dehumidification_control_switch_snapshot_is_exact_direct_release,
    cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_snapshot_is_exact_direct_release,
};

use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

pub(in crate::ideal_loads) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let predecessor = output
        .calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment;
    let snapshot =
        output.calculation_cooling_post_saturation_capacity_limit_dehumidification_control_switch;
    snapshot.system == binding.ideal_loads_air_system
        && snapshot.parent_call_ordinal == call_ordinal
        && snapshot.controlled_zone == binding.zone
        && binding.system.dehumidification_control_type == DehumidificationControlType::None
        && cooling_post_saturation_capacity_limit_dehumidification_control_switch_snapshot_is_exact_direct_release(snapshot)
        && links_to_predecessor(snapshot, predecessor, DehumidificationControlType::None)
}

pub(in crate::ideal_loads) fn validate_lifecycle(
    lifecycle: &Lifecycle,
    predecessor_cp385: &PredecessorLifecycle,
    timestep_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), Error> {
    let state = &lifecycle.state;
    let predecessor = &predecessor_cp385.state;
    let expected_routes = predecessor_route_counts(predecessor);
    let assignment_count = predecessor
        .post_saturation_capacity_limited_dehumidification_supply_enthalpy_assignment_count;
    let inactive_count = state
        .transition_count
        .checked_sub(assignment_count)
        .ok_or_else(|| {
            violation(
                "inactive_transition_count_underflow",
                assignment_count,
                state.transition_count,
            )
        })?;
    let route_sum = checked_sum(
        &state.predecessor_route_counts,
        "predecessor_route_partition_overflow",
    )?;
    let source_sites = assignment_count
        .checked_mul(PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_SOURCE_ORDER.len())
        .ok_or_else(|| violation("source_site_execution_count_overflow", usize::MAX, assignment_count))?;

    if lifecycle.source
        != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_SOURCE
        || lifecycle.first_excluded_lexical_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_FIRST_EXCLUDED_LEXICAL_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_FIRST_EXCLUDED_SOURCE
        || predecessor_cp385.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE
        || predecessor_cp385.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || state.system != binding.ideal_loads_air_system
        || predecessor.system != binding.ideal_loads_air_system
        || state.predecessor_route_counts != expected_routes
        || binding.system.dehumidification_control_type != DehumidificationControlType::None
    {
        return Err(violation("source_owner_route_and_system_identity", 1, 0));
    }

    for (field, expected, actual) in [
        ("transition_count", timestep_count, state.transition_count),
        (
            "predecessor_transition_count",
            predecessor.transition_count,
            state.transition_count,
        ),
        (
            "predecessor_route_partition",
            state.transition_count,
            route_sum,
        ),
        (
            "inactive_transition_count",
            inactive_count,
            state.inactive_transition_count,
        ),
        (
            "dehumidification_control_switch_count",
            assignment_count,
            state.dehumidification_control_switch_count,
        ),
        (
            "source_site_execution_count",
            source_sites,
            state.source_site_execution_count,
        ),
        (
            "dehumidification_control_type_read_count",
            assignment_count,
            state.dehumidification_control_type_read_count,
        ),
        (
            "dehumidification_control_switch_dispatch_count",
            assignment_count,
            state.dehumidification_control_switch_dispatch_count,
        ),
        (
            "dehumidification_control_none_case_selection_count",
            assignment_count,
            state.dehumidification_control_none_case_selection_count,
        ),
        (
            "dehumidification_control_constant_sensible_heat_ratio_case_selection_count",
            0,
            state.dehumidification_control_constant_sensible_heat_ratio_case_selection_count,
        ),
        (
            "dehumidification_control_humidistat_case_selection_count",
            0,
            state.dehumidification_control_humidistat_case_selection_count,
        ),
        (
            "dehumidification_control_constant_supply_humidity_ratio_case_selection_count",
            0,
            state.dehumidification_control_constant_supply_humidity_ratio_case_selection_count,
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
    if !same_snapshot(
        latest,
        latest_output
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_control_switch,
    ) || !links_to_predecessor(
        latest,
        predecessor_latest,
        DehumidificationControlType::None,
    ) || !snapshot_matches_release(latest_output, timestep_count, binding)
    {
        return Err(violation("latest_release_snapshot_ready", 1, 0));
    }
    Ok(())
}

fn links_to_predecessor(
    snapshot: Snapshot,
    predecessor: PredecessorSnapshot,
    selected_control: DehumidificationControlType,
) -> bool {
    let active = predecessor.supply_enthalpy_assignment_executed;
    cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_snapshot_is_exact_direct_release(predecessor)
        && snapshot.system == predecessor.system
        && snapshot.parent_call_ordinal == predecessor.parent_call_ordinal
        && snapshot.controlled_zone == predecessor.controlled_zone
        && inherited_flags(snapshot) == predecessor_flags(predecessor)
        && snapshot.predecessor_supply_enthalpy_assignment_executed == active
        && option_bits_equal(
            snapshot.predecessor_resulting_supply_enthalpy_j_per_kg,
            predecessor.resulting_supply_enthalpy_j_per_kg,
        )
        && option_bits_equal(
            snapshot.resulting_supply_enthalpy_j_per_kg,
            predecessor.resulting_supply_enthalpy_j_per_kg,
        )
        && snapshot.dehumidification_control_type_read == active
        && snapshot.dehumidification_control_type == active.then_some(selected_control)
        && snapshot.dehumidification_control_switch_dispatched == active
}

fn inherited_flags(snapshot: Snapshot) -> [bool; 20] {
    [
        snapshot.unit_off_skipped,
        snapshot.non_cooling_skipped,
        snapshot.positive_guard_false_fallthrough_skipped,
        snapshot.heating_availability_guard_false_fallthrough,
        snapshot.humidification_control_guard_false_fallthrough,
        snapshot.dehumidification_control_humidistat_maximum_assignment_executed,
        snapshot.dehumidification_control_none_maximum_assignment_executed,
        snapshot.dehumidification_control_guard_false_fallthrough,
        snapshot.predecessor_capacity_limit_guard_evaluated,
        snapshot.predecessor_capacity_limit_body_entered,
        snapshot.predecessor_active_capacity_limit_guard_false_fallthrough,
        snapshot.predecessor_dehumidification_guard_evaluated,
        snapshot.predecessor_dehumidification_body_entered,
        snapshot.predecessor_dehumidification_guard_false_fallthrough,
        snapshot.predecessor_dehumidification_total_output_assignment_executed,
        snapshot.predecessor_dehumidification_total_output_capacity_guard_evaluated,
        snapshot.predecessor_dehumidification_total_output_capacity_adjustment_body_entered,
        snapshot.predecessor_dehumidification_total_output_capacity_guard_false_fallthrough,
        snapshot.dehumidification_total_output_capacity_guard_false_fallthrough,
        snapshot.dehumidification_total_output_maximum_capacity_assignment_executed,
    ]
}

fn predecessor_flags(snapshot: PredecessorSnapshot) -> [bool; 20] {
    [
        snapshot.unit_off_skipped,
        snapshot.non_cooling_skipped,
        snapshot.positive_guard_false_fallthrough_skipped,
        snapshot.heating_availability_guard_false_fallthrough,
        snapshot.humidification_control_guard_false_fallthrough,
        snapshot.dehumidification_control_humidistat_maximum_assignment_executed,
        snapshot.dehumidification_control_none_maximum_assignment_executed,
        snapshot.dehumidification_control_guard_false_fallthrough,
        snapshot.predecessor_capacity_limit_guard_evaluated,
        snapshot.predecessor_capacity_limit_body_entered,
        snapshot.predecessor_active_capacity_limit_guard_false_fallthrough,
        snapshot.predecessor_dehumidification_guard_evaluated,
        snapshot.predecessor_dehumidification_body_entered,
        snapshot.predecessor_dehumidification_guard_false_fallthrough,
        snapshot.predecessor_dehumidification_total_output_assignment_executed,
        snapshot.predecessor_dehumidification_total_output_capacity_guard_evaluated,
        snapshot.predecessor_dehumidification_total_output_capacity_adjustment_body_entered,
        snapshot.predecessor_dehumidification_total_output_capacity_guard_false_fallthrough,
        snapshot.dehumidification_total_output_capacity_guard_false_fallthrough,
        snapshot.dehumidification_total_output_maximum_capacity_assignment_executed,
    ]
}

fn predecessor_route_counts(state: &PredecessorState) -> [usize; 23] {
    [
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        state.positive_guard_false_fallthrough_skip_count,
        state.heating_availability_guard_false_fallthrough_capacity_guard_false_count,
        state.heating_availability_guard_false_fallthrough_dehumidification_guard_false_count,
        state.heating_availability_guard_false_fallthrough_dehumidification_total_output_capacity_guard_false_fallthrough_count,
        state.heating_availability_guard_false_fallthrough_dehumidification_total_output_maximum_capacity_assignment_count,
        state.humidification_control_guard_false_fallthrough_capacity_guard_false_count,
        state.humidification_control_guard_false_fallthrough_dehumidification_guard_false_count,
        state.humidification_control_guard_false_fallthrough_dehumidification_total_output_capacity_guard_false_fallthrough_count,
        state.humidification_control_guard_false_fallthrough_dehumidification_total_output_maximum_capacity_assignment_count,
        state.dehumidification_control_humidistat_maximum_assignment_capacity_guard_false_count,
        state.dehumidification_control_humidistat_maximum_assignment_dehumidification_guard_false_count,
        state.dehumidification_control_humidistat_maximum_assignment_dehumidification_total_output_capacity_guard_false_fallthrough_count,
        state.dehumidification_control_humidistat_maximum_assignment_dehumidification_total_output_maximum_capacity_assignment_count,
        state.dehumidification_control_none_maximum_assignment_capacity_guard_false_count,
        state.dehumidification_control_none_maximum_assignment_dehumidification_guard_false_count,
        state.dehumidification_control_none_maximum_assignment_dehumidification_total_output_capacity_guard_false_fallthrough_count,
        state.dehumidification_control_none_maximum_assignment_dehumidification_total_output_maximum_capacity_assignment_count,
        state.dehumidification_control_guard_false_fallthrough_capacity_guard_false_count,
        state.dehumidification_control_guard_false_fallthrough_dehumidification_guard_false_count,
        state.dehumidification_control_guard_false_fallthrough_dehumidification_total_output_capacity_guard_false_fallthrough_count,
        state.dehumidification_control_guard_false_fallthrough_dehumidification_total_output_maximum_capacity_assignment_count,
    ]
}

fn same_snapshot(left: Snapshot, right: Snapshot) -> bool {
    option_bits_equal(
        left.predecessor_resulting_supply_enthalpy_j_per_kg,
        right.predecessor_resulting_supply_enthalpy_j_per_kg,
    ) && option_bits_equal(
        left.resulting_supply_enthalpy_j_per_kg,
        right.resulting_supply_enthalpy_j_per_kg,
    ) && {
        let mut left = left;
        let mut right = right;
        left.predecessor_resulting_supply_enthalpy_j_per_kg = None;
        right.predecessor_resulting_supply_enthalpy_j_per_kg = None;
        left.resulting_supply_enthalpy_j_per_kg = None;
        right.resulting_supply_enthalpy_j_per_kg = None;
        left == right
    }
}

fn option_bits_equal(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
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
    Error::CalcCoolingPostSaturationCapacityLimitDehumidificationControlSwitchLifecycleInvariant {
        field,
        expected,
        actual,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn route_partition_overflow_fails_closed() {
        assert!(checked_sum(&[usize::MAX, 1], "overflow").is_err());
    }

    #[test]
    fn bit_comparison_distinguishes_signed_zero() {
        assert!(option_bits_equal(Some(-0.0), Some(-0.0)));
        assert!(!option_bits_equal(Some(-0.0), Some(0.0)));
    }
}
