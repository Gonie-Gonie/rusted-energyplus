//! CP329/CP330-to-CP331 lineage validation.

use crate::ideal_loads::{
    PurchasedAirCalcCoolingMixedAirCallSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentSnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,
};

pub(super) fn cp_air_assignment_links_to_positive_guard(
    assignment: PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentSnapshot,
    predecessor: PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,
) -> bool {
    assignment.system == predecessor.system
        && assignment.parent_call_ordinal == predecessor.parent_call_ordinal
        && assignment.controlled_zone == predecessor.controlled_zone
        && assignment.unit_body_entered == predecessor.unit_body_entered
        && assignment.predecessor_cooling_body_entered == predecessor.cooling_body_entered
        && assignment.predecessor_no_outdoor_air_fallback_entered
            == predecessor.predecessor_no_outdoor_air_fallback_entered
        && assignment.predecessor_positive_supply_mass_flow_body_entered
            == predecessor.positive_supply_mass_flow_body_entered
        && assignment.predecessor_active_guard_false_fallthrough
            == predecessor.active_guard_false_fallthrough
        && assignment.unit_off_skipped == predecessor.unit_off_skipped
        && assignment.non_cooling_skipped == predecessor.non_cooling_skipped
        && assignment.positive_guard_false_fallthrough_skipped
            == predecessor.active_guard_false_fallthrough
        && assignment.cp_air_assignment_executed
            == predecessor.positive_supply_mass_flow_body_entered
}

pub(super) fn cp_air_assignment_humidity_links_to_mixed_air(
    assignment: PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentSnapshot,
    mixed_air: PurchasedAirCalcCoolingMixedAirCallSnapshot,
) -> bool {
    if !assignment.cp_air_assignment_executed {
        return assignment.zone_humidity_ratio.is_none();
    }
    let Some(zone_humidity_ratio) = assignment.zone_humidity_ratio else {
        return false;
    };
    options_match_value_bits(mixed_air.recirculation_humidity_ratio, zone_humidity_ratio)
        && options_match_value_bits(mixed_air.mixed_air_humidity_ratio, zone_humidity_ratio)
}

pub(super) fn positive_guard_snapshots_match_bit_exact(
    mut left: PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,
    mut right: PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,
) -> bool {
    let supply_matches = options_match_bits(
        left.supply_mass_flow_rate_kg_per_s,
        right.supply_mass_flow_rate_kg_per_s,
    );
    left.supply_mass_flow_rate_kg_per_s = None;
    right.supply_mass_flow_rate_kg_per_s = None;
    supply_matches && left == right
}

fn options_match_value_bits(value: Option<f64>, expected: f64) -> bool {
    value.is_some_and(|value| value.to_bits() == expected.to_bits())
}

fn options_match_bits(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
