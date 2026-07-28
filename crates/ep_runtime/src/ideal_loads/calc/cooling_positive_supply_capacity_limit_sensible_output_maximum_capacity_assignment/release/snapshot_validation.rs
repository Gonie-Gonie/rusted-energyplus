//! Exact CP341 snapshot validation.

use super::super::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentRetainedRoute as Route,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot,
};

pub(in crate::ideal_loads) fn cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_snapshot_is_exact_direct_release(
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot,
) -> bool {
    let provenance = snapshot.source
        == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE
        && snapshot.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        && snapshot.source_order
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE_ORDER;
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
        && !snapshot.predecessor_capacity_limit_sensible_output_guard_evaluated;
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
        && !snapshot.predecessor_capacity_limit_sensible_output_guard_evaluated;
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
        && !snapshot.predecessor_capacity_limit_sensible_output_guard_evaluated;
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
        && !snapshot.predecessor_capacity_limit_sensible_output_guard_evaluated;
    let active_prefix = !snapshot.unit_off_skipped
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
        && snapshot.predecessor_capacity_limit_sensible_output_guard_evaluated;
    let guard_false = active_prefix
        && snapshot.predecessor_capacity_limit_sensible_output_guard_false_fallthrough
        && !snapshot.predecessor_capacity_limit_sensible_output_adjustment_body_entered
        && snapshot.capacity_limit_sensible_output_guard_false_fallthrough
        && !snapshot
            .capacity_limit_sensible_output_maximum_capacity_assignment_executed;
    let assigned = active_prefix
        && !snapshot.predecessor_capacity_limit_sensible_output_guard_false_fallthrough
        && snapshot.predecessor_capacity_limit_sensible_output_adjustment_body_entered
        && !snapshot.capacity_limit_sensible_output_guard_false_fallthrough
        && snapshot
            .capacity_limit_sensible_output_maximum_capacity_assignment_executed;

    provenance
        && usize::from(unit_off)
            + usize::from(non_cooling)
            + usize::from(positive_guard_false)
            + usize::from(capacity_guard_false)
            + usize::from(guard_false)
            + usize::from(assigned)
            == 1
        && if guard_false {
            false_fallthrough_snapshot_is_exact(snapshot)
        } else if assigned {
            assigned_snapshot_is_exact(snapshot)
        } else {
            skipped_snapshot_is_exact(snapshot)
        }
}

pub(super) fn snapshot_route(
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot,
) -> Option<Route> {
    if !cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_snapshot_is_exact_direct_release(
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
    } else if snapshot
        .capacity_limit_sensible_output_maximum_capacity_assignment_executed
    {
        Some(Route::CapacityLimitSensibleOutputMaximumCapacityAssigned)
    } else {
        Some(Route::CapacityLimitSensibleOutputGuardFalseFallthrough)
    }
}

fn false_fallthrough_snapshot_is_exact(
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot,
) -> bool {
    let (Some(preexisting), Some(resulting)) = (
        snapshot.preexisting_cooling_sensible_output_w,
        snapshot.resulting_cooling_sensible_output_w,
    ) else {
        return false;
    };
    !snapshot.maximum_total_cooling_capacity_read
        && snapshot.maximum_total_cooling_capacity_w.is_none()
        && !snapshot.cooling_sensible_output_assigned
        && snapshot.assigned_cooling_sensible_output_w.is_none()
        && preexisting.to_bits() == resulting.to_bits()
}

fn assigned_snapshot_is_exact(
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot,
) -> bool {
    let (Some(_preexisting), Some(maximum), Some(assigned), Some(resulting)) = (
        snapshot.preexisting_cooling_sensible_output_w,
        snapshot.maximum_total_cooling_capacity_w,
        snapshot.assigned_cooling_sensible_output_w,
        snapshot.resulting_cooling_sensible_output_w,
    ) else {
        return false;
    };
    snapshot.maximum_total_cooling_capacity_read
        && maximum.is_finite()
        && maximum > 0.0
        && snapshot.cooling_sensible_output_assigned
        && maximum.to_bits() == assigned.to_bits()
        && assigned.to_bits() == resulting.to_bits()
}

fn skipped_snapshot_is_exact(
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot,
) -> bool {
    !snapshot.predecessor_capacity_limit_sensible_output_guard_false_fallthrough
        && !snapshot.predecessor_capacity_limit_sensible_output_adjustment_body_entered
        && !snapshot.capacity_limit_sensible_output_guard_false_fallthrough
        && !snapshot
            .capacity_limit_sensible_output_maximum_capacity_assignment_executed
        && snapshot.preexisting_cooling_sensible_output_w.is_none()
        && !snapshot.maximum_total_cooling_capacity_read
        && snapshot.maximum_total_cooling_capacity_w.is_none()
        && !snapshot.cooling_sensible_output_assigned
        && snapshot.assigned_cooling_sensible_output_w.is_none()
        && snapshot.resulting_cooling_sensible_output_w.is_none()
}

pub(in crate::ideal_loads::calc) fn snapshots_match_bit_exact(
    mut left:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot,
    mut right:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot,
) -> bool {
    let values_match = option_bits_match(
        left.preexisting_cooling_sensible_output_w,
        right.preexisting_cooling_sensible_output_w,
    ) && option_bits_match(
        left.maximum_total_cooling_capacity_w,
        right.maximum_total_cooling_capacity_w,
    ) && option_bits_match(
        left.assigned_cooling_sensible_output_w,
        right.assigned_cooling_sensible_output_w,
    ) && option_bits_match(
        left.resulting_cooling_sensible_output_w,
        right.resulting_cooling_sensible_output_w,
    );
    for snapshot in [&mut left, &mut right] {
        snapshot.preexisting_cooling_sensible_output_w = None;
        snapshot.maximum_total_cooling_capacity_w = None;
        snapshot.assigned_cooling_sensible_output_w = None;
        snapshot.resulting_cooling_sensible_output_w = None;
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
