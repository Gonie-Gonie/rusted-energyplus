//! Exact CP345 snapshot validation.

use super::super::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentRetainedRoute as Route,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot,
};

pub(in crate::ideal_loads) fn cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_snapshot_is_exact_direct_release(
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot,
) -> bool {
    let provenance = snapshot.source
        == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE
        && snapshot.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        && snapshot.source_order
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE_ORDER;
    let unit_off = snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && !snapshot.unit_body_entered
        && inactive_before_capacity_limit(snapshot);
    let non_cooling = !snapshot.unit_off_skipped
        && snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && inactive_before_capacity_limit(snapshot);
    let positive_false = !snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && snapshot.predecessor_cooling_body_entered
        && snapshot.predecessor_no_outdoor_air_fallback_entered
        && !snapshot.predecessor_positive_supply_mass_flow_body_entered
        && snapshot.predecessor_active_guard_false_fallthrough
        && snapshot.positive_guard_false_fallthrough_skipped
        && inactive_capacity_limit_prefix(snapshot);
    let capacity_false = active_positive_prefix(snapshot)
        && snapshot.predecessor_capacity_limit_guard_evaluated
        && !snapshot.predecessor_capacity_limit_body_entered
        && snapshot.predecessor_active_capacity_limit_guard_false_fallthrough
        && snapshot.capacity_limit_guard_false_fallthrough_skipped
        && inactive_sensible_output_prefix(snapshot);
    let sensible_false = active_positive_prefix(snapshot)
        && snapshot.predecessor_capacity_limit_guard_evaluated
        && snapshot.predecessor_capacity_limit_body_entered
        && !snapshot.predecessor_active_capacity_limit_guard_false_fallthrough
        && !snapshot.capacity_limit_guard_false_fallthrough_skipped
        && active_sensible_output_prefix(snapshot)
        && snapshot.predecessor_capacity_limit_sensible_output_guard_false_fallthrough
        && !snapshot.predecessor_capacity_limit_sensible_output_adjustment_body_entered
        && !snapshot
            .predecessor_capacity_limit_sensible_output_maximum_capacity_assignment_executed
        && !snapshot.predecessor_capacity_limit_sensible_output_supply_enthalpy_assignment_executed
        && !snapshot
            .predecessor_capacity_limit_sensible_output_supply_temperature_assignment_executed
        && snapshot.capacity_limit_sensible_output_guard_false_fallthrough
        && !snapshot.capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed;
    let temperature_limited = active_positive_prefix(snapshot)
        && snapshot.predecessor_capacity_limit_guard_evaluated
        && snapshot.predecessor_capacity_limit_body_entered
        && !snapshot.predecessor_active_capacity_limit_guard_false_fallthrough
        && !snapshot.capacity_limit_guard_false_fallthrough_skipped
        && active_sensible_output_prefix(snapshot)
        && !snapshot.predecessor_capacity_limit_sensible_output_guard_false_fallthrough
        && snapshot.predecessor_capacity_limit_sensible_output_adjustment_body_entered
        && snapshot.predecessor_capacity_limit_sensible_output_maximum_capacity_assignment_executed
        && snapshot.predecessor_capacity_limit_sensible_output_supply_enthalpy_assignment_executed
        && snapshot
            .predecessor_capacity_limit_sensible_output_supply_temperature_assignment_executed
        && !snapshot.capacity_limit_sensible_output_guard_false_fallthrough
        && snapshot.capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed;
    let active = capacity_false || sensible_false || temperature_limited;

    provenance
        && usize::from(unit_off)
            + usize::from(non_cooling)
            + usize::from(positive_false)
            + usize::from(capacity_false)
            + usize::from(sensible_false)
            + usize::from(temperature_limited)
            == 1
        && snapshot.post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_executed
            == active
        && if active {
            active_values_are_exact(snapshot)
        } else {
            inactive_values_are_exact(snapshot)
        }
}

pub(super) fn snapshot_route(
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot,
) -> Option<Route> {
    if !cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_snapshot_is_exact_direct_release(
        snapshot,
    ) {
        None
    } else if snapshot.unit_off_skipped {
        Some(Route::UnitOff)
    } else if snapshot.non_cooling_skipped {
        Some(Route::NonCooling)
    } else if snapshot.positive_guard_false_fallthrough_skipped {
        Some(Route::PositiveGuardFalseFallthrough)
    } else {
        Some(Route::SupplyHumidityRatioMixedAirAssigned)
    }
}

fn active_positive_prefix(
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot,
) -> bool {
    !snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && snapshot.predecessor_cooling_body_entered
        && snapshot.predecessor_no_outdoor_air_fallback_entered
        && snapshot.predecessor_positive_supply_mass_flow_body_entered
        && !snapshot.predecessor_active_guard_false_fallthrough
        && !snapshot.positive_guard_false_fallthrough_skipped
}

fn inactive_before_capacity_limit(
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot,
) -> bool {
    !snapshot.predecessor_cooling_body_entered
        && !snapshot.predecessor_no_outdoor_air_fallback_entered
        && !snapshot.predecessor_positive_supply_mass_flow_body_entered
        && !snapshot.predecessor_active_guard_false_fallthrough
        && !snapshot.positive_guard_false_fallthrough_skipped
        && inactive_capacity_limit_prefix(snapshot)
}

fn inactive_capacity_limit_prefix(
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot,
) -> bool {
    !snapshot.predecessor_capacity_limit_guard_evaluated
        && !snapshot.predecessor_capacity_limit_body_entered
        && !snapshot.predecessor_active_capacity_limit_guard_false_fallthrough
        && !snapshot.capacity_limit_guard_false_fallthrough_skipped
        && inactive_sensible_output_prefix(snapshot)
}

fn inactive_sensible_output_prefix(
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot,
) -> bool {
    !snapshot.predecessor_capacity_limit_cp_air_assignment_executed
        && !snapshot.predecessor_capacity_limit_sensible_output_assignment_executed
        && !snapshot.predecessor_capacity_limit_sensible_output_guard_evaluated
        && !snapshot.predecessor_capacity_limit_sensible_output_guard_false_fallthrough
        && !snapshot.predecessor_capacity_limit_sensible_output_adjustment_body_entered
        && !snapshot.predecessor_capacity_limit_sensible_output_maximum_capacity_assignment_executed
        && !snapshot.predecessor_capacity_limit_sensible_output_supply_enthalpy_assignment_executed
        && !snapshot
            .predecessor_capacity_limit_sensible_output_supply_temperature_assignment_executed
        && !snapshot.capacity_limit_sensible_output_guard_false_fallthrough
        && !snapshot.capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed
}

fn active_sensible_output_prefix(
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot,
) -> bool {
    snapshot.predecessor_capacity_limit_cp_air_assignment_executed
        && snapshot.predecessor_capacity_limit_sensible_output_assignment_executed
        && snapshot.predecessor_capacity_limit_sensible_output_guard_evaluated
}

fn active_values_are_exact(
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot,
) -> bool {
    let (Some(mixed), Some(assigned)) = (
        snapshot.mixed_air_humidity_ratio,
        snapshot.assigned_supply_humidity_ratio,
    ) else {
        return false;
    };
    snapshot.mixed_air_humidity_ratio_read
        && snapshot.supply_humidity_ratio_assignment_performed
        && mixed.to_bits() == assigned.to_bits()
}

fn inactive_values_are_exact(
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot,
) -> bool {
    !snapshot.mixed_air_humidity_ratio_read
        && snapshot.mixed_air_humidity_ratio.is_none()
        && !snapshot.supply_humidity_ratio_assignment_performed
        && snapshot.assigned_supply_humidity_ratio.is_none()
}

pub(in crate::ideal_loads::calc) fn snapshots_match_bit_exact(
    mut left:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot,
    mut right:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot,
) -> bool {
    let values_match = option_bits_match(
        left.mixed_air_humidity_ratio,
        right.mixed_air_humidity_ratio,
    ) && option_bits_match(
        left.assigned_supply_humidity_ratio,
        right.assigned_supply_humidity_ratio,
    );
    for snapshot in [&mut left, &mut right] {
        snapshot.mixed_air_humidity_ratio = None;
        snapshot.assigned_supply_humidity_ratio = None;
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
