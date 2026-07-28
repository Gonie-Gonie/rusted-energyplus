//! Exact CP343 snapshot validation.

use super::super::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentRetainedRoute as Route,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentSnapshot,
};
use crate::psychrometrics::energyplus_psy_tdb_fn_h_w;

pub(in crate::ideal_loads) fn cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_snapshot_is_exact_direct_release(
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentSnapshot,
) -> bool {
    let provenance = snapshot.source
        == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE
        && snapshot.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        && snapshot.source_order
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER;
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
        && !snapshot.predecessor_capacity_limit_sensible_output_guard_evaluated
        && !snapshot.predecessor_capacity_limit_sensible_output_guard_false_fallthrough
        && !snapshot.predecessor_capacity_limit_sensible_output_adjustment_body_entered
        && !snapshot
            .predecessor_capacity_limit_sensible_output_maximum_capacity_assignment_executed
        && !snapshot.predecessor_capacity_limit_sensible_output_supply_enthalpy_assignment_executed
        && !snapshot.capacity_limit_sensible_output_guard_false_fallthrough
        && !snapshot.capacity_limit_sensible_output_supply_temperature_assignment_executed;
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
        && !snapshot.predecessor_capacity_limit_sensible_output_guard_evaluated
        && !snapshot.predecessor_capacity_limit_sensible_output_guard_false_fallthrough
        && !snapshot.predecessor_capacity_limit_sensible_output_adjustment_body_entered
        && !snapshot
            .predecessor_capacity_limit_sensible_output_maximum_capacity_assignment_executed
        && !snapshot.predecessor_capacity_limit_sensible_output_supply_enthalpy_assignment_executed
        && !snapshot.capacity_limit_sensible_output_guard_false_fallthrough
        && !snapshot.capacity_limit_sensible_output_supply_temperature_assignment_executed;
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
        && !snapshot.predecessor_capacity_limit_cp_air_assignment_executed
        && !snapshot.predecessor_capacity_limit_sensible_output_assignment_executed
        && !snapshot.predecessor_capacity_limit_sensible_output_guard_evaluated
        && !snapshot.predecessor_capacity_limit_sensible_output_guard_false_fallthrough
        && !snapshot.predecessor_capacity_limit_sensible_output_adjustment_body_entered
        && !snapshot
            .predecessor_capacity_limit_sensible_output_maximum_capacity_assignment_executed
        && !snapshot.predecessor_capacity_limit_sensible_output_supply_enthalpy_assignment_executed
        && !snapshot.capacity_limit_sensible_output_guard_false_fallthrough
        && !snapshot.capacity_limit_sensible_output_supply_temperature_assignment_executed;
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
        && !snapshot.predecessor_capacity_limit_cp_air_assignment_executed
        && !snapshot.predecessor_capacity_limit_sensible_output_assignment_executed
        && !snapshot.predecessor_capacity_limit_sensible_output_guard_evaluated
        && !snapshot.predecessor_capacity_limit_sensible_output_guard_false_fallthrough
        && !snapshot.predecessor_capacity_limit_sensible_output_adjustment_body_entered
        && !snapshot
            .predecessor_capacity_limit_sensible_output_maximum_capacity_assignment_executed
        && !snapshot.predecessor_capacity_limit_sensible_output_supply_enthalpy_assignment_executed
        && !snapshot.capacity_limit_sensible_output_guard_false_fallthrough
        && !snapshot.capacity_limit_sensible_output_supply_temperature_assignment_executed;
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
        && snapshot.capacity_limit_sensible_output_guard_false_fallthrough
        && !snapshot.capacity_limit_sensible_output_supply_temperature_assignment_executed;
    let assigned = active_prefix
        && !snapshot.predecessor_capacity_limit_sensible_output_guard_false_fallthrough
        && snapshot.predecessor_capacity_limit_sensible_output_adjustment_body_entered
        && snapshot.predecessor_capacity_limit_sensible_output_maximum_capacity_assignment_executed
        && snapshot.predecessor_capacity_limit_sensible_output_supply_enthalpy_assignment_executed
        && !snapshot.capacity_limit_sensible_output_guard_false_fallthrough
        && snapshot.capacity_limit_sensible_output_supply_temperature_assignment_executed;
    provenance
        && usize::from(unit_off)
            + usize::from(non_cooling)
            + usize::from(positive_false)
            + usize::from(capacity_false)
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
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentSnapshot,
) -> Option<Route> {
    if !cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_snapshot_is_exact_direct_release(
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
        .capacity_limit_sensible_output_supply_temperature_assignment_executed
    {
        Some(Route::CapacityLimitSensibleOutputSupplyTemperatureAssigned)
    } else {
        Some(Route::CapacityLimitSensibleOutputGuardFalseFallthrough)
    }
}

fn false_fallthrough_snapshot_is_exact(
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentSnapshot,
) -> bool {
    let (Some(preexisting), Some(resulting)) = (
        snapshot.preexisting_supply_temperature_c,
        snapshot.resulting_supply_temperature_c,
    ) else {
        return false;
    };
    preexisting.is_finite()
        && preexisting.to_bits() == resulting.to_bits()
        && source_values_are_none(snapshot)
}

fn assigned_snapshot_is_exact(
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentSnapshot,
) -> bool {
    let (
        Some(preexisting),
        Some(enthalpy),
        Some(humidity),
        Some(psychrometric),
        Some(assigned),
        Some(resulting),
    ) = (
        snapshot.preexisting_supply_temperature_c,
        snapshot.supply_enthalpy_j_per_kg,
        snapshot.supply_humidity_ratio,
        snapshot.psychrometric_supply_temperature_result_c,
        snapshot.assigned_supply_temperature_c,
        snapshot.resulting_supply_temperature_c,
    )
    else {
        return false;
    };
    let expected = energyplus_psy_tdb_fn_h_w(enthalpy, humidity);
    preexisting.is_finite()
        && snapshot.supply_enthalpy_for_dry_bulb_inversion_read
        && snapshot.supply_humidity_ratio_for_dry_bulb_inversion_read
        && humidity.is_finite()
        && humidity >= 0.0
        && snapshot.psychrometric_supply_temperature_evaluated
        && psychrometric.to_bits() == expected.to_bits()
        && snapshot.supply_temperature_assigned
        && assigned.to_bits() == psychrometric.to_bits()
        && resulting.to_bits() == assigned.to_bits()
}

fn skipped_snapshot_is_exact(
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentSnapshot,
) -> bool {
    !snapshot.capacity_limit_sensible_output_guard_false_fallthrough
        && !snapshot.capacity_limit_sensible_output_supply_temperature_assignment_executed
        && snapshot.preexisting_supply_temperature_c.is_none()
        && snapshot.resulting_supply_temperature_c.is_none()
        && source_values_are_none(snapshot)
}

fn source_values_are_none(
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentSnapshot,
) -> bool {
    !snapshot.supply_enthalpy_for_dry_bulb_inversion_read
        && snapshot.supply_enthalpy_j_per_kg.is_none()
        && !snapshot.supply_humidity_ratio_for_dry_bulb_inversion_read
        && snapshot.supply_humidity_ratio.is_none()
        && !snapshot.psychrometric_supply_temperature_evaluated
        && snapshot.psychrometric_supply_temperature_result_c.is_none()
        && !snapshot.supply_temperature_assigned
        && snapshot.assigned_supply_temperature_c.is_none()
}

pub(in crate::ideal_loads::calc) fn snapshots_match_bit_exact(
    mut left:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentSnapshot,
    mut right:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentSnapshot,
) -> bool {
    let values_match = [
        option_bits_match(
            left.preexisting_supply_temperature_c,
            right.preexisting_supply_temperature_c,
        ),
        option_bits_match(
            left.supply_enthalpy_j_per_kg,
            right.supply_enthalpy_j_per_kg,
        ),
        option_bits_match(left.supply_humidity_ratio, right.supply_humidity_ratio),
        option_bits_match(
            left.psychrometric_supply_temperature_result_c,
            right.psychrometric_supply_temperature_result_c,
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
        snapshot.supply_enthalpy_j_per_kg = None;
        snapshot.supply_humidity_ratio = None;
        snapshot.psychrometric_supply_temperature_result_c = None;
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
