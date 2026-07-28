//! Exact CP344 snapshot validation.

use super::super::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitRetainedRoute as Route,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot,
};
use crate::ideal_loads::calc::cooling_positive_supply_temperature_mixed_air_limit::source_shaped_two_argument_minimum;

pub(in crate::ideal_loads) fn cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_snapshot_is_exact_direct_release(
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot,
) -> bool {
    let provenance = snapshot.source
        == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE
        && snapshot.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE
        && snapshot.source_order
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER;
    let unit_off = snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && !snapshot.unit_body_entered
        && inactive_before_capacity_limit(snapshot)
        && !snapshot.capacity_limit_sensible_output_guard_false_fallthrough
        && !snapshot
            .predecessor_capacity_limit_sensible_output_supply_temperature_assignment_executed
        && !snapshot
            .capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed;
    let non_cooling = !snapshot.unit_off_skipped
        && snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && inactive_before_capacity_limit(snapshot)
        && !snapshot.capacity_limit_sensible_output_guard_false_fallthrough
        && !snapshot
            .predecessor_capacity_limit_sensible_output_supply_temperature_assignment_executed
        && !snapshot
            .capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed;
    let positive_false = !snapshot.unit_off_skipped
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
        && inactive_sensible_output_prefix(snapshot)
        && !snapshot.capacity_limit_sensible_output_guard_false_fallthrough
        && !snapshot
            .predecessor_capacity_limit_sensible_output_supply_temperature_assignment_executed
        && !snapshot
            .capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed;
    let capacity_false = !snapshot.unit_off_skipped
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
        && inactive_sensible_output_prefix(snapshot)
        && !snapshot.capacity_limit_sensible_output_guard_false_fallthrough
        && !snapshot
            .predecessor_capacity_limit_sensible_output_supply_temperature_assignment_executed
        && !snapshot
            .capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed;
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
        && !snapshot
            .predecessor_capacity_limit_sensible_output_maximum_capacity_assignment_executed
        && !snapshot.predecessor_capacity_limit_sensible_output_supply_enthalpy_assignment_executed
        && !snapshot
            .predecessor_capacity_limit_sensible_output_supply_temperature_assignment_executed
        && snapshot.capacity_limit_sensible_output_guard_false_fallthrough
        && !snapshot
            .capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed;
    let limited = active_prefix
        && !snapshot.predecessor_capacity_limit_sensible_output_guard_false_fallthrough
        && snapshot.predecessor_capacity_limit_sensible_output_adjustment_body_entered
        && snapshot
            .predecessor_capacity_limit_sensible_output_maximum_capacity_assignment_executed
        && snapshot.predecessor_capacity_limit_sensible_output_supply_enthalpy_assignment_executed
        && snapshot
            .predecessor_capacity_limit_sensible_output_supply_temperature_assignment_executed
        && !snapshot.capacity_limit_sensible_output_guard_false_fallthrough
        && snapshot
            .capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed;

    provenance
        && usize::from(unit_off)
            + usize::from(non_cooling)
            + usize::from(positive_false)
            + usize::from(capacity_false)
            + usize::from(guard_false)
            + usize::from(limited)
            == 1
        && if guard_false {
            false_fallthrough_snapshot_is_exact(snapshot)
        } else if limited {
            limited_snapshot_is_exact(snapshot)
        } else {
            skipped_snapshot_is_exact(snapshot)
        }
}

pub(super) fn snapshot_route(
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot,
) -> Option<Route> {
    if !cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_snapshot_is_exact_direct_release(
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
        .capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed
    {
        Some(Route::CapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitExecuted)
    } else {
        Some(Route::CapacityLimitSensibleOutputGuardFalseFallthrough)
    }
}

fn inactive_before_capacity_limit(
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot,
) -> bool {
    !snapshot.predecessor_cooling_body_entered
        && !snapshot.predecessor_no_outdoor_air_fallback_entered
        && !snapshot.predecessor_positive_supply_mass_flow_body_entered
        && !snapshot.predecessor_active_guard_false_fallthrough
        && !snapshot.positive_guard_false_fallthrough_skipped
        && !snapshot.predecessor_capacity_limit_guard_evaluated
        && !snapshot.predecessor_capacity_limit_body_entered
        && !snapshot.predecessor_active_capacity_limit_guard_false_fallthrough
        && !snapshot.capacity_limit_guard_false_fallthrough_skipped
        && inactive_sensible_output_prefix(snapshot)
}

fn inactive_sensible_output_prefix(
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot,
) -> bool {
    !snapshot.predecessor_capacity_limit_cp_air_assignment_executed
        && !snapshot.predecessor_capacity_limit_sensible_output_assignment_executed
        && !snapshot.predecessor_capacity_limit_sensible_output_guard_evaluated
        && !snapshot.predecessor_capacity_limit_sensible_output_guard_false_fallthrough
        && !snapshot.predecessor_capacity_limit_sensible_output_adjustment_body_entered
        && !snapshot
            .predecessor_capacity_limit_sensible_output_maximum_capacity_assignment_executed
        && !snapshot.predecessor_capacity_limit_sensible_output_supply_enthalpy_assignment_executed
}

fn false_fallthrough_snapshot_is_exact(
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot,
) -> bool {
    let (Some(preexisting), Some(resulting)) = (
        snapshot.preexisting_supply_temperature_c,
        snapshot.resulting_supply_temperature_c,
    ) else {
        return false;
    };
    preexisting.to_bits() == resulting.to_bits() && source_values_are_none(snapshot)
}

fn limited_snapshot_is_exact(
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot,
) -> bool {
    let (
        Some(preexisting),
        Some(left),
        Some(right),
        Some(minimum),
        Some(assigned),
        Some(resulting),
    ) = (
        snapshot.preexisting_supply_temperature_c,
        snapshot.supply_temperature_before_mixed_air_limit_c,
        snapshot.mixed_air_temperature_c,
        snapshot.minimum_supply_temperature_c,
        snapshot.assigned_supply_temperature_c,
        snapshot.resulting_supply_temperature_c,
    )
    else {
        return false;
    };
    let expected = source_shaped_two_argument_minimum(left, right);

    snapshot.supply_temperature_for_minimum_read
        && preexisting.to_bits() == left.to_bits()
        && snapshot.mixed_air_temperature_for_minimum_read
        && right.is_finite()
        && snapshot.source_shaped_two_argument_minimum_evaluated
        && minimum.to_bits() == expected.to_bits()
        && snapshot.supply_temperature_assignment_performed
        && assigned.to_bits() == minimum.to_bits()
        && resulting.to_bits() == assigned.to_bits()
}

fn skipped_snapshot_is_exact(
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot,
) -> bool {
    snapshot.preexisting_supply_temperature_c.is_none()
        && snapshot.resulting_supply_temperature_c.is_none()
        && source_values_are_none(snapshot)
}

fn source_values_are_none(
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot,
) -> bool {
    !snapshot.supply_temperature_for_minimum_read
        && snapshot
            .supply_temperature_before_mixed_air_limit_c
            .is_none()
        && !snapshot.mixed_air_temperature_for_minimum_read
        && snapshot.mixed_air_temperature_c.is_none()
        && !snapshot.source_shaped_two_argument_minimum_evaluated
        && snapshot.minimum_supply_temperature_c.is_none()
        && !snapshot.supply_temperature_assignment_performed
        && snapshot.assigned_supply_temperature_c.is_none()
}

pub(in crate::ideal_loads::calc) fn snapshots_match_bit_exact(
    mut left:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot,
    mut right:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot,
) -> bool {
    let values_match = [
        option_bits_match(
            left.preexisting_supply_temperature_c,
            right.preexisting_supply_temperature_c,
        ),
        option_bits_match(
            left.supply_temperature_before_mixed_air_limit_c,
            right.supply_temperature_before_mixed_air_limit_c,
        ),
        option_bits_match(left.mixed_air_temperature_c, right.mixed_air_temperature_c),
        option_bits_match(
            left.minimum_supply_temperature_c,
            right.minimum_supply_temperature_c,
        ),
        option_bits_match(
            left.assigned_supply_temperature_c,
            right.assigned_supply_temperature_c,
        ),
        option_bits_match(
            left.resulting_supply_temperature_c,
            right.resulting_supply_temperature_c,
        ),
    ]
    .into_iter()
    .all(|matches| matches);
    for snapshot in [&mut left, &mut right] {
        snapshot.preexisting_supply_temperature_c = None;
        snapshot.supply_temperature_before_mixed_air_limit_c = None;
        snapshot.mixed_air_temperature_c = None;
        snapshot.minimum_supply_temperature_c = None;
        snapshot.assigned_supply_temperature_c = None;
        snapshot.resulting_supply_temperature_c = None;
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
