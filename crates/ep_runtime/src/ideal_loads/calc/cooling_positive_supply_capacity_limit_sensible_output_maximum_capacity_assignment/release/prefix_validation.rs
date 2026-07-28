//! CP340-to-CP341 retained-lineage validation.

use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot,
};

pub(super) fn maximum_capacity_assignment_links_to_guard(
    assignment:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot,
    predecessor: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot,
) -> bool {
    let active = predecessor.capacity_limit_sensible_output_guard_evaluated;
    let assigned = predecessor.capacity_limit_sensible_output_adjustment_body_entered;
    let inherited = assignment.system == predecessor.system
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
            == predecessor.predecessor_capacity_limit_guard_evaluated
        && assignment.predecessor_capacity_limit_body_entered
            == predecessor.predecessor_capacity_limit_body_entered
        && assignment.predecessor_active_capacity_limit_guard_false_fallthrough
            == predecessor.predecessor_active_capacity_limit_guard_false_fallthrough
        && assignment.predecessor_capacity_limit_cp_air_assignment_executed
            == predecessor.predecessor_capacity_limit_cp_air_assignment_executed
        && assignment.predecessor_capacity_limit_sensible_output_assignment_executed
            == predecessor.predecessor_capacity_limit_sensible_output_assignment_executed
        && assignment.predecessor_capacity_limit_sensible_output_guard_evaluated == active
        && assignment.predecessor_capacity_limit_sensible_output_guard_false_fallthrough
            == predecessor.capacity_limit_sensible_output_guard_false_fallthrough
        && assignment.predecessor_capacity_limit_sensible_output_adjustment_body_entered
            == assigned
        && assignment.unit_off_skipped == predecessor.unit_off_skipped
        && assignment.non_cooling_skipped == predecessor.non_cooling_skipped
        && assignment.positive_guard_false_fallthrough_skipped
            == predecessor.positive_guard_false_fallthrough_skipped
        && assignment.capacity_limit_guard_false_fallthrough_skipped
            == predecessor.capacity_limit_guard_false_fallthrough_skipped
        && assignment.capacity_limit_sensible_output_guard_false_fallthrough
            == predecessor.capacity_limit_sensible_output_guard_false_fallthrough
        && assignment.capacity_limit_sensible_output_maximum_capacity_assignment_executed
            == assigned;
    if !inherited {
        return false;
    }

    if active {
        let (Some(predecessor_output), Some(preexisting), Some(result)) = (
            predecessor.cooling_sensible_output_w,
            assignment.preexisting_cooling_sensible_output_w,
            assignment.resulting_cooling_sensible_output_w,
        ) else {
            return false;
        };
        if predecessor_output.to_bits() != preexisting.to_bits() {
            return false;
        }
        if assigned {
            let (Some(predecessor_maximum), Some(maximum), Some(assigned_value)) = (
                predecessor.maximum_total_cooling_capacity_w,
                assignment.maximum_total_cooling_capacity_w,
                assignment.assigned_cooling_sensible_output_w,
            ) else {
                return false;
            };
            assignment.maximum_total_cooling_capacity_read
                && assignment.cooling_sensible_output_assigned
                && predecessor_maximum.to_bits() == maximum.to_bits()
                && maximum.to_bits() == assigned_value.to_bits()
                && assigned_value.to_bits() == result.to_bits()
        } else {
            !assignment.maximum_total_cooling_capacity_read
                && assignment.maximum_total_cooling_capacity_w.is_none()
                && !assignment.cooling_sensible_output_assigned
                && assignment.assigned_cooling_sensible_output_w.is_none()
                && preexisting.to_bits() == result.to_bits()
        }
    } else {
        assignment.preexisting_cooling_sensible_output_w.is_none()
            && !assignment.maximum_total_cooling_capacity_read
            && assignment.maximum_total_cooling_capacity_w.is_none()
            && !assignment.cooling_sensible_output_assigned
            && assignment.assigned_cooling_sensible_output_w.is_none()
            && assignment.resulting_cooling_sensible_output_w.is_none()
    }
}

pub(super) fn retained_guard_active_values_are_release_reachable(
    predecessor: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot,
) -> bool {
    if !predecessor.capacity_limit_sensible_output_guard_evaluated {
        return true;
    }
    let (Some(_output), Some(maximum)) = (
        predecessor.cooling_sensible_output_w,
        predecessor.maximum_total_cooling_capacity_w,
    ) else {
        return false;
    };
    maximum.is_finite() && maximum > 0.0
}

pub(super) fn sensible_output_guard_snapshots_match_bit_exact(
    mut left: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot,
    mut right: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot,
) -> bool {
    let values_match = option_bits_match(
        left.cooling_sensible_output_w,
        right.cooling_sensible_output_w,
    ) && option_bits_match(
        left.maximum_total_cooling_capacity_w,
        right.maximum_total_cooling_capacity_w,
    );
    for snapshot in [&mut left, &mut right] {
        snapshot.cooling_sensible_output_w = None;
        snapshot.maximum_total_cooling_capacity_w = None;
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
