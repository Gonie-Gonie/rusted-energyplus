//! Exact CP338 snapshot validation.

use super::super::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_CP_AIR_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_CP_AIR_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentRetainedRoute as Route,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot,
};
use crate::psychrometrics::energyplus_psy_cp_air_fn_w;

pub(in crate::ideal_loads) fn cooling_positive_supply_capacity_limit_cp_air_assignment_snapshot_is_exact_direct_release(
    snapshot: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot,
) -> bool {
    let provenance = snapshot.source
        == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_CP_AIR_ASSIGNMENT_SOURCE
        && snapshot.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        && snapshot.source_order
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_CP_AIR_ASSIGNMENT_SOURCE_ORDER;
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
        && !snapshot.capacity_limit_cp_air_assignment_executed;
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
        && !snapshot.capacity_limit_cp_air_assignment_executed;
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
        && !snapshot.capacity_limit_cp_air_assignment_executed;
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
        && !snapshot.capacity_limit_cp_air_assignment_executed;
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
        && snapshot.capacity_limit_cp_air_assignment_executed;

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
    snapshot: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot,
) -> Option<Route> {
    if !cooling_positive_supply_capacity_limit_cp_air_assignment_snapshot_is_exact_direct_release(
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
        Some(Route::CapacityLimitCpAirAssigned)
    }
}

fn assigned_snapshot_is_exact(
    snapshot: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot,
) -> bool {
    let Some(humidity_ratio) = snapshot.mixed_air_humidity_ratio else {
        return false;
    };
    let Some(result) = snapshot.psychrometric_cp_air_result_j_per_kg_k else {
        return false;
    };
    let Some(assigned) = snapshot.cp_air_j_per_kg_k else {
        return false;
    };
    let expected = energyplus_psy_cp_air_fn_w(humidity_ratio);
    snapshot.mixed_air_humidity_ratio_read
        && humidity_ratio.is_finite()
        && humidity_ratio >= 0.0
        && snapshot.psychrometric_cp_air_evaluated
        && result.is_finite()
        && result.to_bits() == expected.to_bits()
        && snapshot.cp_air_assigned
        && assigned.to_bits() == result.to_bits()
}

fn skipped_snapshot_is_exact(
    snapshot: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot,
) -> bool {
    !snapshot.mixed_air_humidity_ratio_read
        && snapshot.mixed_air_humidity_ratio.is_none()
        && !snapshot.psychrometric_cp_air_evaluated
        && snapshot
            .psychrometric_cp_air_result_j_per_kg_k
            .is_none()
        && !snapshot.cp_air_assigned
        && snapshot.cp_air_j_per_kg_k.is_none()
}

pub(in crate::ideal_loads::calc) fn snapshots_match_bit_exact(
    mut left: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot,
    mut right: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot,
) -> bool {
    let values_match = option_bits_match(
        left.mixed_air_humidity_ratio,
        right.mixed_air_humidity_ratio,
    ) && option_bits_match(
        left.psychrometric_cp_air_result_j_per_kg_k,
        right.psychrometric_cp_air_result_j_per_kg_k,
    ) && option_bits_match(left.cp_air_j_per_kg_k, right.cp_air_j_per_kg_k);
    for snapshot in [&mut left, &mut right] {
        snapshot.mixed_air_humidity_ratio = None;
        snapshot.psychrometric_cp_air_result_j_per_kg_k = None;
        snapshot.cp_air_j_per_kg_k = None;
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
