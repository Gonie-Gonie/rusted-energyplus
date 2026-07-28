//! Exact CP336 snapshot validation.

use super::super::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot,
};
use crate::psychrometrics::energyplus_psy_h_fn_tdb_w;

pub(in crate::ideal_loads) fn cooling_positive_supply_enthalpy_assignment_snapshot_is_exact_direct_release(
    snapshot: PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot,
) -> bool {
    let provenance = snapshot.source
        == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE
        && snapshot.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        && snapshot.source_order
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER;
    let unit_off = snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && !snapshot.unit_body_entered
        && !snapshot.predecessor_cooling_body_entered
        && !snapshot.predecessor_no_outdoor_air_fallback_entered
        && !snapshot.predecessor_positive_supply_mass_flow_body_entered
        && !snapshot.predecessor_active_guard_false_fallthrough
        && !snapshot.positive_guard_false_fallthrough_skipped
        && !snapshot.supply_enthalpy_assignment_executed;
    let non_cooling = !snapshot.unit_off_skipped
        && snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && !snapshot.predecessor_cooling_body_entered
        && !snapshot.predecessor_no_outdoor_air_fallback_entered
        && !snapshot.predecessor_positive_supply_mass_flow_body_entered
        && !snapshot.predecessor_active_guard_false_fallthrough
        && !snapshot.positive_guard_false_fallthrough_skipped
        && !snapshot.supply_enthalpy_assignment_executed;
    let guard_false = !snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && snapshot.predecessor_cooling_body_entered
        && snapshot.predecessor_no_outdoor_air_fallback_entered
        && !snapshot.predecessor_positive_supply_mass_flow_body_entered
        && snapshot.predecessor_active_guard_false_fallthrough
        && snapshot.positive_guard_false_fallthrough_skipped
        && !snapshot.supply_enthalpy_assignment_executed;
    let assigned = !snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && snapshot.predecessor_cooling_body_entered
        && snapshot.predecessor_no_outdoor_air_fallback_entered
        && snapshot.predecessor_positive_supply_mass_flow_body_entered
        && !snapshot.predecessor_active_guard_false_fallthrough
        && !snapshot.positive_guard_false_fallthrough_skipped
        && snapshot.supply_enthalpy_assignment_executed;

    provenance
        && usize::from(unit_off)
            + usize::from(non_cooling)
            + usize::from(guard_false)
            + usize::from(assigned)
            == 1
        && if assigned {
            assigned_snapshot_is_exact(snapshot)
        } else {
            skipped_snapshot_is_exact(snapshot)
        }
}

fn assigned_snapshot_is_exact(
    snapshot: PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot,
) -> bool {
    let Some(supply_temperature_c) = snapshot.supply_temperature_c else {
        return false;
    };
    let Some(supply_humidity_ratio) = snapshot.supply_humidity_ratio else {
        return false;
    };
    let Some(psychrometric_result) =
        snapshot.psychrometric_supply_enthalpy_result_j_per_kg
    else {
        return false;
    };
    let Some(assigned) = snapshot.supply_enthalpy_j_per_kg else {
        return false;
    };
    let expected = energyplus_psy_h_fn_tdb_w(supply_temperature_c, supply_humidity_ratio);

    snapshot.supply_temperature_for_enthalpy_read
        && supply_temperature_c.is_finite()
        && snapshot.supply_humidity_ratio_for_enthalpy_read
        && supply_humidity_ratio.is_finite()
        && supply_humidity_ratio >= 0.0
        && snapshot.psychrometric_supply_enthalpy_evaluated
        && psychrometric_result.is_finite()
        && psychrometric_result.to_bits() == expected.to_bits()
        && snapshot.supply_enthalpy_assigned
        && assigned.to_bits() == psychrometric_result.to_bits()
}

fn skipped_snapshot_is_exact(
    snapshot: PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot,
) -> bool {
    !snapshot.supply_temperature_for_enthalpy_read
        && snapshot.supply_temperature_c.is_none()
        && !snapshot.supply_humidity_ratio_for_enthalpy_read
        && snapshot.supply_humidity_ratio.is_none()
        && !snapshot.psychrometric_supply_enthalpy_evaluated
        && snapshot
            .psychrometric_supply_enthalpy_result_j_per_kg
            .is_none()
        && !snapshot.supply_enthalpy_assigned
        && snapshot.supply_enthalpy_j_per_kg.is_none()
}

pub(in crate::ideal_loads::calc) fn snapshots_match_bit_exact(
    mut left: PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot,
    mut right: PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot,
) -> bool {
    let values_match = option_bits_match(left.supply_temperature_c, right.supply_temperature_c)
        && option_bits_match(left.supply_humidity_ratio, right.supply_humidity_ratio)
        && option_bits_match(
            left.psychrometric_supply_enthalpy_result_j_per_kg,
            right.psychrometric_supply_enthalpy_result_j_per_kg,
        )
        && option_bits_match(
            left.supply_enthalpy_j_per_kg,
            right.supply_enthalpy_j_per_kg,
        );
    for snapshot in [&mut left, &mut right] {
        snapshot.supply_temperature_c = None;
        snapshot.supply_humidity_ratio = None;
        snapshot.psychrometric_supply_enthalpy_result_j_per_kg = None;
        snapshot.supply_enthalpy_j_per_kg = None;
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
