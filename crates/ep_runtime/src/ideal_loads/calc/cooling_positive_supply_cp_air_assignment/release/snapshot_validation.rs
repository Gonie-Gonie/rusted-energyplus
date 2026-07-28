//! Exact CP331 snapshot validation.

use super::super::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CP_AIR_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CP_AIR_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentSnapshot,
};
use crate::psychrometrics::energyplus_psy_cp_air_fn_w;

pub(in crate::ideal_loads) fn cooling_positive_supply_cp_air_assignment_snapshot_is_exact_direct_release(
    snapshot: PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentSnapshot,
) -> bool {
    let provenance = snapshot.source
        == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CP_AIR_ASSIGNMENT_SOURCE
        && snapshot.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        && snapshot.source_order
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CP_AIR_ASSIGNMENT_SOURCE_ORDER;
    let unit_off = snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && !snapshot.unit_body_entered
        && !snapshot.predecessor_cooling_body_entered
        && !snapshot.predecessor_no_outdoor_air_fallback_entered
        && !snapshot.predecessor_positive_supply_mass_flow_body_entered
        && !snapshot.predecessor_active_guard_false_fallthrough
        && !snapshot.positive_guard_false_fallthrough_skipped
        && !snapshot.cp_air_assignment_executed;
    let non_cooling = !snapshot.unit_off_skipped
        && snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && !snapshot.predecessor_cooling_body_entered
        && !snapshot.predecessor_no_outdoor_air_fallback_entered
        && !snapshot.predecessor_positive_supply_mass_flow_body_entered
        && !snapshot.predecessor_active_guard_false_fallthrough
        && !snapshot.positive_guard_false_fallthrough_skipped
        && !snapshot.cp_air_assignment_executed;
    let guard_false = !snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && snapshot.predecessor_cooling_body_entered
        && snapshot.predecessor_no_outdoor_air_fallback_entered
        && !snapshot.predecessor_positive_supply_mass_flow_body_entered
        && snapshot.predecessor_active_guard_false_fallthrough
        && snapshot.positive_guard_false_fallthrough_skipped
        && !snapshot.cp_air_assignment_executed;
    let assigned = !snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && snapshot.predecessor_cooling_body_entered
        && snapshot.predecessor_no_outdoor_air_fallback_entered
        && snapshot.predecessor_positive_supply_mass_flow_body_entered
        && !snapshot.predecessor_active_guard_false_fallthrough
        && !snapshot.positive_guard_false_fallthrough_skipped
        && snapshot.cp_air_assignment_executed;
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
    snapshot: PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentSnapshot,
) -> bool {
    let Some(humidity_ratio) = snapshot.zone_humidity_ratio else {
        return false;
    };
    let Some(result) = snapshot.psychrometric_cp_air_result_j_per_kg_k else {
        return false;
    };
    let Some(assigned) = snapshot.cp_air_j_per_kg_k else {
        return false;
    };
    let expected = energyplus_psy_cp_air_fn_w(humidity_ratio);
    snapshot.zone_humidity_ratio_read
        && humidity_ratio.is_finite()
        && humidity_ratio >= 0.0
        && snapshot.psychrometric_cp_air_evaluated
        && result.is_finite()
        && result.to_bits() == expected.to_bits()
        && snapshot.cp_air_assigned
        && assigned.to_bits() == result.to_bits()
}

fn skipped_snapshot_is_exact(
    snapshot: PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentSnapshot,
) -> bool {
    !snapshot.zone_humidity_ratio_read
        && snapshot.zone_humidity_ratio.is_none()
        && !snapshot.psychrometric_cp_air_evaluated
        && snapshot.psychrometric_cp_air_result_j_per_kg_k.is_none()
        && !snapshot.cp_air_assigned
        && snapshot.cp_air_j_per_kg_k.is_none()
}

pub(in crate::ideal_loads::calc) fn snapshots_match_bit_exact(
    mut left: PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentSnapshot,
    mut right: PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentSnapshot,
) -> bool {
    let values_match = option_bits_match(left.zone_humidity_ratio, right.zone_humidity_ratio)
        && option_bits_match(
            left.psychrometric_cp_air_result_j_per_kg_k,
            right.psychrometric_cp_air_result_j_per_kg_k,
        )
        && option_bits_match(left.cp_air_j_per_kg_k, right.cp_air_j_per_kg_k);
    for snapshot in [&mut left, &mut right] {
        snapshot.zone_humidity_ratio = None;
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
