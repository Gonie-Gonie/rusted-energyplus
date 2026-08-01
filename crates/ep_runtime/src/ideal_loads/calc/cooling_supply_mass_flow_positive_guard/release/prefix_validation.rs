//! CP329-to-CP330 lineage validation.

use crate::ideal_loads::{
    PurchasedAirCalcCoolingMixedAirCallSnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,
};

pub(in crate::ideal_loads::calc) fn positive_guard_links_to_mixed_air_call(
    guard: PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,
    predecessor: PurchasedAirCalcCoolingMixedAirCallSnapshot,
) -> bool {
    let common = guard.system == predecessor.system
        && guard.parent_call_ordinal == predecessor.parent_call_ordinal
        && guard.controlled_zone == predecessor.controlled_zone
        && guard.unit_body_entered == predecessor.unit_body_entered
        && guard.predecessor_cooling_call_executed == predecessor.cooling_call_executed
        && guard.predecessor_zero_flow_reset_body_entered
            == predecessor.predecessor_zero_flow_reset_body_entered
        && guard.predecessor_active_guard_false_fallthrough
            == predecessor.predecessor_active_guard_false_fallthrough
        && guard.predecessor_no_outdoor_air_fallback_entered
            == predecessor.no_outdoor_air_fallback_entered
        && guard.unit_off_skipped == predecessor.unit_off_skipped
        && guard.non_cooling_skipped == predecessor.non_cooling_skipped
        && guard.cooling_body_entered == predecessor.cooling_call_executed;
    if !common {
        return false;
    }
    if predecessor.cooling_call_executed {
        options_match_bits(
            guard.supply_mass_flow_rate_kg_per_s,
            predecessor.supply_mass_flow_rate_kg_per_s,
        ) && options_match_bits(
            guard.supply_mass_flow_rate_kg_per_s,
            predecessor.child_supply_mass_flow_rate_kg_per_s,
        ) && options_match_bits(
            guard.supply_mass_flow_rate_kg_per_s,
            predecessor.resulting_recirculation_mass_flow_rate_kg_per_s,
        )
    } else {
        guard.supply_mass_flow_rate_kg_per_s.is_none()
            && predecessor.supply_mass_flow_rate_kg_per_s.is_none()
            && predecessor.child_supply_mass_flow_rate_kg_per_s.is_none()
            && predecessor
                .resulting_recirculation_mass_flow_rate_kg_per_s
                .is_none()
    }
}

fn options_match_bits(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
