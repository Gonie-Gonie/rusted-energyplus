//! CP329/CP337-to-CP338 lineage validation.

use crate::ideal_loads::{
    PurchasedAirCalcCoolingMixedAirCallSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot,
    cooling_mixed_air_call_snapshots_match_bit_exact,
};

pub(super) fn cp_air_assignment_links_to_capacity_limit_guard(
    assignment: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot,
    predecessor: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot,
) -> bool {
    assignment.system == predecessor.system
        && assignment.parent_call_ordinal == predecessor.parent_call_ordinal
        && assignment.controlled_zone == predecessor.controlled_zone
        && assignment.unit_body_entered == predecessor.unit_body_entered
        && assignment.predecessor_cooling_body_entered
            == predecessor.predecessor_cooling_body_entered
        && assignment.predecessor_no_outdoor_air_fallback_entered
            == predecessor.predecessor_no_outdoor_air_fallback_entered
        && assignment.predecessor_positive_supply_mass_flow_body_entered
            == predecessor.predecessor_positive_supply_mass_flow_body_entered
        && assignment.predecessor_active_guard_false_fallthrough
            == predecessor.predecessor_active_guard_false_fallthrough
        && assignment.predecessor_capacity_limit_guard_evaluated
            == predecessor.capacity_limit_guard_evaluated
        && assignment.predecessor_capacity_limit_body_entered
            == predecessor.capacity_limit_body_entered
        && assignment.predecessor_active_capacity_limit_guard_false_fallthrough
            == predecessor.active_guard_false_fallthrough
        && assignment.unit_off_skipped == predecessor.unit_off_skipped
        && assignment.non_cooling_skipped == predecessor.non_cooling_skipped
        && assignment.positive_guard_false_fallthrough_skipped
            == predecessor.positive_guard_false_fallthrough_skipped
        && assignment.capacity_limit_guard_false_fallthrough_skipped
            == predecessor.active_guard_false_fallthrough
        && assignment.capacity_limit_cp_air_assignment_executed
            == predecessor.capacity_limit_body_entered
}

pub(super) fn active_operand_links_to_retained_prefix(
    predecessor: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot,
    mixed_air: PurchasedAirCalcCoolingMixedAirCallSnapshot,
    mixed_air_witness: PurchasedAirCalcCoolingMixedAirCallSnapshot,
    operand: Option<f64>,
) -> bool {
    let Some(operand) = operand else {
        return false;
    };
    #[rustfmt::skip]
    let operand_matches = mixed_air.mixed_air_humidity_ratio.is_some_and(|value| value.to_bits() == operand.to_bits());
    predecessor.capacity_limit_body_entered
        && mixed_air.system == predecessor.system
        && mixed_air.parent_call_ordinal == predecessor.parent_call_ordinal
        && mixed_air.controlled_zone == predecessor.controlled_zone
        && mixed_air.cooling_call_executed
        && mixed_air.no_outdoor_air_fallback_entered
        && mixed_air.mixed_air_humidity_ratio_assigned
        && cooling_mixed_air_call_snapshots_match_bit_exact(mixed_air, mixed_air_witness)
        && operand_matches
        && operand.is_finite()
        && operand >= 0.0
}

pub(super) fn capacity_limit_guard_snapshots_match_exact(
    left: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot,
    right: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot,
) -> bool {
    left == right
}
