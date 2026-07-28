//! Exact CP339 snapshot validation.

use super::super::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentRetainedRoute as Route,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot,
};

pub(in crate::ideal_loads) fn cooling_positive_supply_capacity_limit_sensible_output_assignment_snapshot_is_exact_direct_release(
    snapshot: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot,
) -> bool {
    let provenance = snapshot.source
        == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE
        && snapshot.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        && snapshot.source_order
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER;
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
        && !snapshot.capacity_limit_sensible_output_assignment_executed;
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
        && !snapshot.capacity_limit_sensible_output_assignment_executed;
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
        && !snapshot.capacity_limit_sensible_output_assignment_executed;
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
        && !snapshot.capacity_limit_sensible_output_assignment_executed;
    let assigned = !snapshot.unit_off_skipped
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
        && snapshot.capacity_limit_sensible_output_assignment_executed;

    provenance
        && usize::from(unit_off)
            + usize::from(non_cooling)
            + usize::from(positive_guard_false)
            + usize::from(capacity_guard_false)
            + usize::from(assigned)
            == 1
        && if assigned {
            assigned_snapshot_is_exact(snapshot)
        } else {
            skipped_snapshot_is_exact(snapshot)
        }
}

pub(super) fn snapshot_route(
    snapshot: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot,
) -> Option<Route> {
    if !cooling_positive_supply_capacity_limit_sensible_output_assignment_snapshot_is_exact_direct_release(
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
    } else {
        Some(Route::CapacityLimitSensibleOutputAssigned)
    }
}

fn assigned_snapshot_is_exact(
    snapshot: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot,
) -> bool {
    let (
        Some(supply_mass_flow_rate),
        Some(mixed_air_enthalpy),
        Some(supply_enthalpy),
        Some(enthalpy_difference),
        Some(calculated),
        Some(assigned),
    ) = (
        snapshot.supply_mass_flow_rate_kg_per_s,
        snapshot.mixed_air_enthalpy_j_per_kg,
        snapshot.supply_enthalpy_j_per_kg,
        snapshot.mixed_air_minus_supply_enthalpy_j_per_kg,
        snapshot.calculated_cooling_sensible_output_w,
        snapshot.cooling_sensible_output_w,
    )
    else {
        return false;
    };
    let expected_difference = mixed_air_enthalpy - supply_enthalpy;
    let expected_calculated = supply_mass_flow_rate * expected_difference;
    snapshot.supply_mass_flow_rate_read
        && supply_mass_flow_rate > 0.0
        && snapshot.mixed_air_enthalpy_read
        && mixed_air_enthalpy.is_finite()
        && snapshot.supply_enthalpy_read
        && supply_enthalpy.is_finite()
        && snapshot.enthalpy_difference_calculated
        && enthalpy_difference.to_bits() == expected_difference.to_bits()
        && snapshot.cooling_sensible_output_calculated
        && calculated.to_bits() == expected_calculated.to_bits()
        && snapshot.cooling_sensible_output_assigned
        && assigned.to_bits() == calculated.to_bits()
}

fn skipped_snapshot_is_exact(
    snapshot: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot,
) -> bool {
    !snapshot.supply_mass_flow_rate_read
        && snapshot.supply_mass_flow_rate_kg_per_s.is_none()
        && !snapshot.mixed_air_enthalpy_read
        && snapshot.mixed_air_enthalpy_j_per_kg.is_none()
        && !snapshot.supply_enthalpy_read
        && snapshot.supply_enthalpy_j_per_kg.is_none()
        && !snapshot.enthalpy_difference_calculated
        && snapshot
            .mixed_air_minus_supply_enthalpy_j_per_kg
            .is_none()
        && !snapshot.cooling_sensible_output_calculated
        && snapshot.calculated_cooling_sensible_output_w.is_none()
        && !snapshot.cooling_sensible_output_assigned
        && snapshot.cooling_sensible_output_w.is_none()
}

pub(in crate::ideal_loads::calc) fn snapshots_match_bit_exact(
    mut left: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot,
    mut right: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot,
) -> bool {
    let values_match = option_bits_match(
        left.supply_mass_flow_rate_kg_per_s,
        right.supply_mass_flow_rate_kg_per_s,
    ) && option_bits_match(
        left.mixed_air_enthalpy_j_per_kg,
        right.mixed_air_enthalpy_j_per_kg,
    ) && option_bits_match(
        left.supply_enthalpy_j_per_kg,
        right.supply_enthalpy_j_per_kg,
    ) && option_bits_match(
        left.mixed_air_minus_supply_enthalpy_j_per_kg,
        right.mixed_air_minus_supply_enthalpy_j_per_kg,
    ) && option_bits_match(
        left.calculated_cooling_sensible_output_w,
        right.calculated_cooling_sensible_output_w,
    ) && option_bits_match(
        left.cooling_sensible_output_w,
        right.cooling_sensible_output_w,
    );
    for snapshot in [&mut left, &mut right] {
        snapshot.supply_mass_flow_rate_kg_per_s = None;
        snapshot.mixed_air_enthalpy_j_per_kg = None;
        snapshot.supply_enthalpy_j_per_kg = None;
        snapshot.mixed_air_minus_supply_enthalpy_j_per_kg = None;
        snapshot.calculated_cooling_sensible_output_w = None;
        snapshot.cooling_sensible_output_w = None;
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
