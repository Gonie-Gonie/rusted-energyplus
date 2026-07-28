//! Exact CP333 snapshot validation.

use super::super::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MINIMUM_LIMIT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MINIMUM_LIMIT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MINIMUM_LIMIT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitSnapshot,
    source_shaped_two_argument_maximum,
};

pub(in crate::ideal_loads) fn cooling_positive_supply_temperature_minimum_limit_snapshot_is_exact_direct_release(
    snapshot: PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitSnapshot,
) -> bool {
    let provenance = snapshot.source
        == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MINIMUM_LIMIT_SOURCE
        && snapshot.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MINIMUM_LIMIT_FIRST_EXCLUDED_SOURCE
        && snapshot.source_order
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MINIMUM_LIMIT_SOURCE_ORDER;
    let unit_off = snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && !snapshot.unit_body_entered
        && !snapshot.predecessor_cooling_body_entered
        && !snapshot.predecessor_no_outdoor_air_fallback_entered
        && !snapshot.predecessor_positive_supply_mass_flow_body_entered
        && !snapshot.predecessor_active_guard_false_fallthrough
        && !snapshot.positive_guard_false_fallthrough_skipped
        && !snapshot.supply_temperature_minimum_limit_executed;
    let non_cooling = !snapshot.unit_off_skipped
        && snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && !snapshot.predecessor_cooling_body_entered
        && !snapshot.predecessor_no_outdoor_air_fallback_entered
        && !snapshot.predecessor_positive_supply_mass_flow_body_entered
        && !snapshot.predecessor_active_guard_false_fallthrough
        && !snapshot.positive_guard_false_fallthrough_skipped
        && !snapshot.supply_temperature_minimum_limit_executed;
    let guard_false = !snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && snapshot.predecessor_cooling_body_entered
        && snapshot.predecessor_no_outdoor_air_fallback_entered
        && !snapshot.predecessor_positive_supply_mass_flow_body_entered
        && snapshot.predecessor_active_guard_false_fallthrough
        && snapshot.positive_guard_false_fallthrough_skipped
        && !snapshot.supply_temperature_minimum_limit_executed;
    let limited = !snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && snapshot.predecessor_cooling_body_entered
        && snapshot.predecessor_no_outdoor_air_fallback_entered
        && snapshot.predecessor_positive_supply_mass_flow_body_entered
        && !snapshot.predecessor_active_guard_false_fallthrough
        && !snapshot.positive_guard_false_fallthrough_skipped
        && snapshot.supply_temperature_minimum_limit_executed;

    provenance
        && usize::from(unit_off)
            + usize::from(non_cooling)
            + usize::from(guard_false)
            + usize::from(limited)
            == 1
        && if limited {
            limited_snapshot_is_exact(snapshot)
        } else {
            skipped_snapshot_is_exact(snapshot)
        }
}

fn limited_snapshot_is_exact(
    snapshot: PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitSnapshot,
) -> bool {
    let Some(left) = snapshot.supply_temperature_before_minimum_limit_c else {
        return false;
    };
    let Some(right) = snapshot.minimum_cooling_supply_air_temperature_c else {
        return false;
    };
    let Some(maximum) = snapshot.maximum_supply_temperature_c else {
        return false;
    };
    let Some(assigned) = snapshot.assigned_supply_temperature_c else {
        return false;
    };
    let expected = source_shaped_two_argument_maximum(left, right);

    snapshot.supply_temperature_for_maximum_read
        && snapshot.minimum_cooling_supply_air_temperature_for_maximum_read
        && right.is_finite()
        && snapshot.source_shaped_two_argument_maximum_evaluated
        && maximum.to_bits() == expected.to_bits()
        && snapshot.supply_temperature_assignment_performed
        && assigned.to_bits() == maximum.to_bits()
}

fn skipped_snapshot_is_exact(
    snapshot: PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitSnapshot,
) -> bool {
    !snapshot.supply_temperature_for_maximum_read
        && snapshot
            .supply_temperature_before_minimum_limit_c
            .is_none()
        && !snapshot.minimum_cooling_supply_air_temperature_for_maximum_read
        && snapshot
            .minimum_cooling_supply_air_temperature_c
            .is_none()
        && !snapshot.source_shaped_two_argument_maximum_evaluated
        && snapshot.maximum_supply_temperature_c.is_none()
        && !snapshot.supply_temperature_assignment_performed
        && snapshot.assigned_supply_temperature_c.is_none()
}

pub(in crate::ideal_loads::calc) fn snapshots_match_bit_exact(
    mut left: PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitSnapshot,
    mut right: PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitSnapshot,
) -> bool {
    let values_match = option_bits_match(
        left.supply_temperature_before_minimum_limit_c,
        right.supply_temperature_before_minimum_limit_c,
    ) && option_bits_match(
        left.minimum_cooling_supply_air_temperature_c,
        right.minimum_cooling_supply_air_temperature_c,
    ) && option_bits_match(
        left.maximum_supply_temperature_c,
        right.maximum_supply_temperature_c,
    ) && option_bits_match(
        left.assigned_supply_temperature_c,
        right.assigned_supply_temperature_c,
    );
    for snapshot in [&mut left, &mut right] {
        snapshot.supply_temperature_before_minimum_limit_c = None;
        snapshot.minimum_cooling_supply_air_temperature_c = None;
        snapshot.maximum_supply_temperature_c = None;
        snapshot.assigned_supply_temperature_c = None;
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
