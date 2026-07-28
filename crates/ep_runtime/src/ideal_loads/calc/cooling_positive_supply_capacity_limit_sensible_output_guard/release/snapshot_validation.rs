//! Exact CP340 snapshot validation.

use super::super::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardRetainedRoute as Route,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot,
};

pub(in crate::ideal_loads) fn cooling_positive_supply_capacity_limit_sensible_output_guard_snapshot_is_exact_direct_release(
    snapshot: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot,
) -> bool {
    let provenance = snapshot.source
        == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_SOURCE
        && snapshot.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE
        && snapshot.source_order
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_SOURCE_ORDER;
    let unit_off = snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && !snapshot.unit_body_entered
        && !snapshot.predecessor_cooling_body_entered
        && !snapshot.predecessor_no_outdoor_air_fallback_entered
        && !snapshot.predecessor_positive_supply_mass_flow_body_entered
        && !snapshot.predecessor_active_guard_false_fallthrough
        && !snapshot.positive_guard_false_fallthrough_skipped
        && !snapshot.predecessor_capacity_limit_guard_evaluated
        && !snapshot.predecessor_capacity_limit_body_entered
        && !snapshot.predecessor_active_capacity_limit_guard_false_fallthrough
        && !snapshot.capacity_limit_guard_false_fallthrough_skipped
        && !snapshot.predecessor_capacity_limit_cp_air_assignment_executed
        && !snapshot.predecessor_capacity_limit_sensible_output_assignment_executed
        && !snapshot.capacity_limit_sensible_output_guard_evaluated;
    let non_cooling = !snapshot.unit_off_skipped
        && snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && !snapshot.predecessor_cooling_body_entered
        && !snapshot.predecessor_no_outdoor_air_fallback_entered
        && !snapshot.predecessor_positive_supply_mass_flow_body_entered
        && !snapshot.predecessor_active_guard_false_fallthrough
        && !snapshot.positive_guard_false_fallthrough_skipped
        && !snapshot.predecessor_capacity_limit_guard_evaluated
        && !snapshot.predecessor_capacity_limit_body_entered
        && !snapshot.predecessor_active_capacity_limit_guard_false_fallthrough
        && !snapshot.capacity_limit_guard_false_fallthrough_skipped
        && !snapshot.predecessor_capacity_limit_cp_air_assignment_executed
        && !snapshot.predecessor_capacity_limit_sensible_output_assignment_executed
        && !snapshot.capacity_limit_sensible_output_guard_evaluated;
    let positive_guard_false = !snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && snapshot.predecessor_cooling_body_entered
        && snapshot.predecessor_no_outdoor_air_fallback_entered
        && !snapshot.predecessor_positive_supply_mass_flow_body_entered
        && snapshot.predecessor_active_guard_false_fallthrough
        && snapshot.positive_guard_false_fallthrough_skipped
        && !snapshot.predecessor_capacity_limit_guard_evaluated
        && !snapshot.predecessor_capacity_limit_body_entered
        && !snapshot.predecessor_active_capacity_limit_guard_false_fallthrough
        && !snapshot.capacity_limit_guard_false_fallthrough_skipped
        && !snapshot.predecessor_capacity_limit_cp_air_assignment_executed
        && !snapshot.predecessor_capacity_limit_sensible_output_assignment_executed
        && !snapshot.capacity_limit_sensible_output_guard_evaluated;
    let capacity_guard_false = !snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && snapshot.predecessor_cooling_body_entered
        && snapshot.predecessor_no_outdoor_air_fallback_entered
        && snapshot.predecessor_positive_supply_mass_flow_body_entered
        && !snapshot.predecessor_active_guard_false_fallthrough
        && !snapshot.positive_guard_false_fallthrough_skipped
        && snapshot.predecessor_capacity_limit_guard_evaluated
        && !snapshot.predecessor_capacity_limit_body_entered
        && snapshot.predecessor_active_capacity_limit_guard_false_fallthrough
        && snapshot.capacity_limit_guard_false_fallthrough_skipped
        && !snapshot.predecessor_capacity_limit_cp_air_assignment_executed
        && !snapshot.predecessor_capacity_limit_sensible_output_assignment_executed
        && !snapshot.capacity_limit_sensible_output_guard_evaluated;
    let active = !snapshot.unit_off_skipped
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
        && snapshot.capacity_limit_sensible_output_guard_evaluated;

    provenance
        && usize::from(unit_off)
            + usize::from(non_cooling)
            + usize::from(positive_guard_false)
            + usize::from(capacity_guard_false)
            + usize::from(active)
            == 1
        && if active {
            active_snapshot_is_exact(snapshot)
        } else {
            skipped_snapshot_is_exact(snapshot)
        }
}

pub(super) fn snapshot_route(
    snapshot: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot,
) -> Option<Route> {
    if !cooling_positive_supply_capacity_limit_sensible_output_guard_snapshot_is_exact_direct_release(
        snapshot,
    ) {
        None
    } else if snapshot.unit_off_skipped {
        Some(Route::UnitOff)
    } else if snapshot.non_cooling_skipped {
        Some(Route::NonCooling)
    } else if snapshot.positive_guard_false_fallthrough_skipped {
        Some(Route::PositiveGuardFalseFallthrough)
    } else if snapshot.capacity_limit_guard_false_fallthrough_skipped {
        Some(Route::ActiveCapacityLimitGuardFalseFallthrough)
    } else if snapshot.capacity_limit_sensible_output_adjustment_body_entered {
        Some(Route::CapacityLimitSensibleOutputAdjustmentBodyEntered)
    } else {
        Some(Route::CapacityLimitSensibleOutputGuardFalseFallthrough)
    }
}

fn active_snapshot_is_exact(
    snapshot: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot,
) -> bool {
    let (Some(output), Some(maximum)) = (
        snapshot.cooling_sensible_output_w,
        snapshot.maximum_total_cooling_capacity_w,
    ) else {
        return false;
    };
    let satisfied = output >= maximum;
    snapshot.cooling_sensible_output_read
        && snapshot.maximum_total_cooling_capacity_read
        && maximum.is_finite()
        && maximum >= 0.0
        && snapshot.cooling_sensible_output_maximum_capacity_comparison_evaluated
        && snapshot.cooling_sensible_output_at_or_above_maximum_capacity == Some(satisfied)
        && snapshot.capacity_limit_sensible_output_guard_false_fallthrough != satisfied
        && snapshot.capacity_limit_sensible_output_adjustment_body_entered == satisfied
}

fn skipped_snapshot_is_exact(
    snapshot: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot,
) -> bool {
    !snapshot.cooling_sensible_output_read
        && snapshot.cooling_sensible_output_w.is_none()
        && !snapshot.maximum_total_cooling_capacity_read
        && snapshot.maximum_total_cooling_capacity_w.is_none()
        && !snapshot.cooling_sensible_output_maximum_capacity_comparison_evaluated
        && snapshot
            .cooling_sensible_output_at_or_above_maximum_capacity
            .is_none()
        && !snapshot.capacity_limit_sensible_output_guard_false_fallthrough
        && !snapshot.capacity_limit_sensible_output_adjustment_body_entered
}

pub(in crate::ideal_loads::calc) fn snapshots_match_bit_exact(
    mut left: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot,
    mut right: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot,
) -> bool {
    let values_match = option_bits_match(
        left.cooling_sensible_output_w,
        right.cooling_sensible_output_w,
    ) && option_bits_match(
        left.maximum_total_cooling_capacity_w,
        right.maximum_total_cooling_capacity_w,
    );
    for snapshot in [&mut left, &mut right] {
        snapshot.cooling_sensible_output_w = None;
        snapshot.maximum_total_cooling_capacity_w = None;
    }
    values_match && left == right
}

fn option_bits_match(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
