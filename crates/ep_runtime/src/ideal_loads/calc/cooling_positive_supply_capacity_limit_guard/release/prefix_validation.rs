//! CP321/CP325/CP336-to-CP337 lineage validation.

use ep_model::IdealLoadsLimit;

use crate::ideal_loads::{
    PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot,
};

pub(super) fn capacity_limit_guard_links_to_enthalpy_assignment(
    guard: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot,
    predecessor: PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot,
) -> bool {
    guard.system == predecessor.system
        && guard.parent_call_ordinal == predecessor.parent_call_ordinal
        && guard.controlled_zone == predecessor.controlled_zone
        && guard.unit_body_entered == predecessor.unit_body_entered
        && guard.predecessor_cooling_body_entered
            == predecessor.predecessor_cooling_body_entered
        && guard.predecessor_no_outdoor_air_fallback_entered
            == predecessor.predecessor_no_outdoor_air_fallback_entered
        && guard.predecessor_positive_supply_mass_flow_body_entered
            == predecessor.predecessor_positive_supply_mass_flow_body_entered
        && guard.predecessor_active_guard_false_fallthrough
            == predecessor.predecessor_active_guard_false_fallthrough
        && guard.unit_off_skipped == predecessor.unit_off_skipped
        && guard.non_cooling_skipped == predecessor.non_cooling_skipped
        && guard.positive_guard_false_fallthrough_skipped
            == predecessor.positive_guard_false_fallthrough_skipped
        && guard.capacity_limit_guard_evaluated
            == predecessor.supply_enthalpy_assignment_executed
}

pub(super) fn active_cooling_limit_links_to_retained_prefix(
    predecessor: PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot,
    capacity_reset: PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot,
    flow_limit_guard: PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot,
    cooling_limit: IdealLoadsLimit,
) -> bool {
    predecessor.supply_enthalpy_assignment_executed
        && capacity_reset.system == predecessor.system
        && capacity_reset.parent_call_ordinal == predecessor.parent_call_ordinal
        && capacity_reset.controlled_zone == predecessor.controlled_zone
        && capacity_reset.cooling_body_entered
        && capacity_reset.first_cooling_limit_read
        && capacity_reset.first_cooling_limit == Some(cooling_limit)
        && (!capacity_reset.second_cooling_limit_read
            || capacity_reset.second_cooling_limit == Some(cooling_limit))
        && flow_limit_guard.system == predecessor.system
        && flow_limit_guard.parent_call_ordinal == predecessor.parent_call_ordinal
        && flow_limit_guard.controlled_zone == predecessor.controlled_zone
        && flow_limit_guard.cooling_body_entered
        && flow_limit_guard.first_cooling_limit_read
        && flow_limit_guard.first_cooling_limit == Some(cooling_limit)
        && (!flow_limit_guard.second_cooling_limit_read
            || flow_limit_guard.second_cooling_limit == Some(cooling_limit))
}

pub(super) fn enthalpy_assignment_snapshots_match_bit_exact(
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
