//! Sealed CP340 same-call maximum-capacity corroborator.

use super::snapshot_validation::snapshots_match_bit_exact;
use super::super::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE as EXCLUDED,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_SOURCE as SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_SOURCE_ORDER as ORDER,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardRetainedRoute as Route,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardRuntimeState as State,
};
use crate::ideal_loads::calc::cooling_capacity_zero_flow_reset_committed_latest_maximum_total_cooling_capacity;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot as Cp321Snapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot as Snapshot,
    PurchasedAirUnitRuntimeState,
};

pub(in crate::ideal_loads::calc) fn cooling_positive_supply_capacity_limit_sensible_output_guard_committed_latest_maximum_total_cooling_capacity(
    unit: &PurchasedAirUnitRuntimeState,
    cp321_witness: Cp321Snapshot,
    cp340_witness: Snapshot,
) -> Option<f64> {
    let capacity = cooling_capacity_zero_flow_reset_committed_latest_maximum_total_cooling_capacity(
        unit,
        cp321_witness,
    )?;
    let state = &unit.calc_cooling_positive_supply_capacity_limit_sensible_output_guard;
    let latest = state.latest?;
    (state.system == unit.system
        && state.transition_count == unit.init_call_count
        && state.transition_count > 0
        && state.latest_transition_ordinal == Some(state.transition_count)
        && latest.system == unit.system
        && latest.parent_call_ordinal == state.transition_count
        && unit.controlled_zone == Some(latest.controlled_zone)
        && committed_state_shape(state)
        && snapshots_match_bit_exact(latest, cp340_witness)
        && committed_snapshot_shape(latest)
        && state.latest_route
            == Some(if latest.capacity_limit_sensible_output_adjustment_body_entered {
                Route::CapacityLimitSensibleOutputAdjustmentBodyEntered
            } else {
                Route::CapacityLimitSensibleOutputGuardFalseFallthrough
            })
        && latest.capacity_limit_sensible_output_guard_evaluated
        && latest.maximum_total_cooling_capacity_read
        && latest
            .maximum_total_cooling_capacity_w
            .is_some_and(|value| value.to_bits() == capacity.to_bits()))
        .then_some(capacity)
}

fn committed_snapshot_shape(snapshot: Snapshot) -> bool {
    let (Some(output), Some(capacity), Some(satisfied)) = (
        snapshot.cooling_sensible_output_w,
        snapshot.maximum_total_cooling_capacity_w,
        snapshot.cooling_sensible_output_at_or_above_maximum_capacity,
    ) else {
        return false;
    };
    snapshot.source == SOURCE
        && snapshot.first_excluded_source == EXCLUDED
        && snapshot.source_order == ORDER
        && !snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && snapshot.predecessor_cooling_body_entered
        && snapshot.predecessor_no_outdoor_air_fallback_entered
        && snapshot.predecessor_positive_supply_mass_flow_body_entered
        && !snapshot.predecessor_active_guard_false_fallthrough
        && !snapshot.positive_guard_false_fallthrough_skipped
        && snapshot.predecessor_capacity_limit_guard_evaluated
        && snapshot.predecessor_capacity_limit_body_entered
        && !snapshot.predecessor_active_capacity_limit_guard_false_fallthrough
        && !snapshot.capacity_limit_guard_false_fallthrough_skipped
        && snapshot.predecessor_capacity_limit_cp_air_assignment_executed
        && snapshot.predecessor_capacity_limit_sensible_output_assignment_executed
        && snapshot.capacity_limit_sensible_output_guard_evaluated
        && snapshot.cooling_sensible_output_read
        && snapshot.maximum_total_cooling_capacity_read
        && capacity.is_finite()
        && capacity >= 0.0
        && snapshot.cooling_sensible_output_maximum_capacity_comparison_evaluated
        && satisfied == (output >= capacity)
        && snapshot.capacity_limit_sensible_output_guard_false_fallthrough != satisfied
        && snapshot.capacity_limit_sensible_output_adjustment_body_entered == satisfied
}

fn committed_state_shape(state: &State) -> bool {
    let route_partition = state
        .unit_off_skip_count
        .checked_add(state.non_cooling_skip_count)
        .and_then(|count| count.checked_add(state.positive_guard_false_fallthrough_skip_count))
        .and_then(|count| {
            count.checked_add(state.capacity_limit_guard_false_fallthrough_skip_count)
        })
        .and_then(|count| {
            count.checked_add(
                state.capacity_limit_sensible_output_guard_false_fallthrough_count,
            )
        })
        .and_then(|count| {
            count.checked_add(
                state.capacity_limit_sensible_output_adjustment_body_entry_count,
            )
        });
    let active = state
        .capacity_limit_sensible_output_guard_false_fallthrough_count
        .checked_add(state.capacity_limit_sensible_output_adjustment_body_entry_count);
    let sites = state
        .capacity_limit_sensible_output_guard_evaluation_count
        .checked_mul(3)
        .and_then(|count| {
            count.checked_add(
                state.capacity_limit_sensible_output_adjustment_body_entry_count,
            )
        });
    route_partition == Some(state.transition_count)
        && active == Some(state.capacity_limit_sensible_output_guard_evaluation_count)
        && sites == Some(state.source_site_execution_count)
        && state.cooling_sensible_output_read_count
            == state.capacity_limit_sensible_output_guard_evaluation_count
        && state.maximum_total_cooling_capacity_read_count
            == state.capacity_limit_sensible_output_guard_evaluation_count
        && state.cooling_sensible_output_maximum_capacity_comparison_count
            == state.capacity_limit_sensible_output_guard_evaluation_count
        && state.witnessed_positive_guard_false_fallthrough_skip_count
            == state.positive_guard_false_fallthrough_skip_count
        && state.witnessed_capacity_limit_guard_false_fallthrough_skip_count
            == state.capacity_limit_guard_false_fallthrough_skip_count
        && state
            .witnessed_capacity_limit_sensible_output_guard_false_fallthrough_count
            == state.capacity_limit_sensible_output_guard_false_fallthrough_count
        && state
            .witnessed_capacity_limit_sensible_output_adjustment_body_entry_count
            == state.capacity_limit_sensible_output_adjustment_body_entry_count
        && matches!(
            state.latest_route,
            Some(
                Route::CapacityLimitSensibleOutputGuardFalseFallthrough
                    | Route::CapacityLimitSensibleOutputAdjustmentBodyEntered
            )
        )
}
