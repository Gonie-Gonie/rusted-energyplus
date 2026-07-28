//! Exact CP330 snapshot validation.

use super::super::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,
};

pub(in crate::ideal_loads) fn cooling_supply_mass_flow_positive_guard_snapshot_is_exact_direct_release(
    snapshot: PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,
) -> bool {
    let provenance = snapshot.source
        == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_SOURCE
        && snapshot.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_FIRST_EXCLUDED_SOURCE
        && snapshot.source_order
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_SOURCE_ORDER;
    let unit_off = snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && !snapshot.unit_body_entered
        && !snapshot.predecessor_cooling_call_executed
        && !snapshot.predecessor_zero_flow_reset_body_entered
        && !snapshot.predecessor_active_guard_false_fallthrough
        && !snapshot.predecessor_no_outdoor_air_fallback_entered
        && !snapshot.cooling_body_entered;
    let non_cooling = !snapshot.unit_off_skipped
        && snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && !snapshot.predecessor_cooling_call_executed
        && !snapshot.predecessor_zero_flow_reset_body_entered
        && !snapshot.predecessor_active_guard_false_fallthrough
        && !snapshot.predecessor_no_outdoor_air_fallback_entered
        && !snapshot.cooling_body_entered;
    let active = !snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && snapshot.predecessor_cooling_call_executed
        && (snapshot.predecessor_zero_flow_reset_body_entered
            != snapshot.predecessor_active_guard_false_fallthrough)
        && snapshot.predecessor_no_outdoor_air_fallback_entered
        && snapshot.cooling_body_entered;
    provenance
        && usize::from(unit_off) + usize::from(non_cooling) + usize::from(active) == 1
        && if active {
            active_snapshot_is_exact(snapshot)
        } else {
            skipped_snapshot_is_exact(snapshot)
        }
}

fn active_snapshot_is_exact(
    snapshot: PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,
) -> bool {
    let Some(supply_mass_flow_rate_kg_per_s) = snapshot.supply_mass_flow_rate_kg_per_s else {
        return false;
    };
    let strictly_positive = supply_mass_flow_rate_kg_per_s > 0.0;
    snapshot.supply_mass_flow_rate_read
        && snapshot.supply_mass_flow_rate_strictly_positive_comparison_evaluated
        && snapshot.supply_mass_flow_rate_strictly_positive == Some(strictly_positive)
        && snapshot.positive_supply_mass_flow_body_entered == strictly_positive
        && snapshot.active_guard_false_fallthrough != strictly_positive
}

fn skipped_snapshot_is_exact(
    snapshot: PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,
) -> bool {
    !snapshot.supply_mass_flow_rate_read
        && snapshot.supply_mass_flow_rate_kg_per_s.is_none()
        && !snapshot.supply_mass_flow_rate_strictly_positive_comparison_evaluated
        && snapshot.supply_mass_flow_rate_strictly_positive.is_none()
        && !snapshot.positive_supply_mass_flow_body_entered
        && !snapshot.active_guard_false_fallthrough
}

pub(in crate::ideal_loads::calc) fn snapshots_match_bit_exact(
    mut left: PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,
    mut right: PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,
) -> bool {
    let supply_matches = option_bits_match(
        left.supply_mass_flow_rate_kg_per_s,
        right.supply_mass_flow_rate_kg_per_s,
    );
    left.supply_mass_flow_rate_kg_per_s = None;
    right.supply_mass_flow_rate_kg_per_s = None;
    supply_matches && left == right
}

fn option_bits_match(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
