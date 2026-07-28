//! CP321/CP339-to-CP340 lineage validation.

use crate::ideal_loads::{
    PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot,
    cooling_capacity_zero_flow_reset_snapshot_is_exact_direct_release,
    cooling_positive_supply_capacity_limit_sensible_output_assignment_snapshot_is_exact_direct_release,
};

pub(super) fn sensible_output_guard_links_to_assignment(
    guard: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot,
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot,
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
        && guard.predecessor_capacity_limit_guard_evaluated
            == predecessor.predecessor_capacity_limit_guard_evaluated
        && guard.predecessor_capacity_limit_body_entered
            == predecessor.predecessor_capacity_limit_body_entered
        && guard.predecessor_active_capacity_limit_guard_false_fallthrough
            == predecessor.predecessor_active_capacity_limit_guard_false_fallthrough
        && guard.predecessor_capacity_limit_cp_air_assignment_executed
            == predecessor.predecessor_capacity_limit_cp_air_assignment_executed
        && guard.predecessor_capacity_limit_sensible_output_assignment_executed
            == predecessor.capacity_limit_sensible_output_assignment_executed
        && guard.unit_off_skipped == predecessor.unit_off_skipped
        && guard.non_cooling_skipped == predecessor.non_cooling_skipped
        && guard.positive_guard_false_fallthrough_skipped
            == predecessor.positive_guard_false_fallthrough_skipped
        && guard.capacity_limit_guard_false_fallthrough_skipped
            == predecessor.capacity_limit_guard_false_fallthrough_skipped
        && guard.capacity_limit_sensible_output_guard_evaluated
            == predecessor.capacity_limit_sensible_output_assignment_executed
}

pub(super) fn active_operands_link_to_retained_prefix(
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot,
    capacity_reset: PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot,
    capacity_reset_witness: PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot,
    cooling_sensible_output_w: f64,
    maximum_total_cooling_capacity_w: f64,
) -> bool {
    let same_call = capacity_reset.system == predecessor.system
        && capacity_reset.parent_call_ordinal == predecessor.parent_call_ordinal
        && capacity_reset.controlled_zone == predecessor.controlled_zone;
    predecessor.capacity_limit_sensible_output_assignment_executed
        && predecessor.cooling_sensible_output_assigned
        && predecessor
            .cooling_sensible_output_w
            .is_some_and(|value| value.to_bits() == cooling_sensible_output_w.to_bits())
        && cooling_positive_supply_capacity_limit_sensible_output_assignment_snapshot_is_exact_direct_release(
            predecessor,
        )
        && same_call
        && capacity_reset.cooling_body_entered
        && capacity_reset.cooling_limit_condition_satisfied == Some(true)
        && capacity_reset.maximum_total_cooling_capacity_read
        && capacity_reset
            .maximum_total_cooling_capacity_w
            .is_some_and(|value| {
                value.to_bits() == maximum_total_cooling_capacity_w.to_bits()
            })
        && maximum_total_cooling_capacity_w.is_finite()
        && maximum_total_cooling_capacity_w >= 0.0
        && cooling_capacity_zero_flow_reset_snapshot_is_exact_direct_release(capacity_reset)
        && capacity_reset_snapshots_match_bit_exact(capacity_reset, capacity_reset_witness)
}

pub(super) fn sensible_output_assignment_snapshots_match_bit_exact(
    mut left:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot,
    mut right:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot,
) -> bool {
    let values_match = [
        option_bits_match(
            left.supply_mass_flow_rate_kg_per_s,
            right.supply_mass_flow_rate_kg_per_s,
        ),
        option_bits_match(
            left.mixed_air_enthalpy_j_per_kg,
            right.mixed_air_enthalpy_j_per_kg,
        ),
        option_bits_match(
            left.supply_enthalpy_j_per_kg,
            right.supply_enthalpy_j_per_kg,
        ),
        option_bits_match(
            left.mixed_air_minus_supply_enthalpy_j_per_kg,
            right.mixed_air_minus_supply_enthalpy_j_per_kg,
        ),
        option_bits_match(
            left.calculated_cooling_sensible_output_w,
            right.calculated_cooling_sensible_output_w,
        ),
        option_bits_match(
            left.cooling_sensible_output_w,
            right.cooling_sensible_output_w,
        ),
    ]
    .into_iter()
    .all(|matches| matches);
    for snapshot in [&mut left, &mut right] {
        snapshot.supply_mass_flow_rate_kg_per_s = None;
        snapshot.mixed_air_enthalpy_j_per_kg = None;
        snapshot.supply_enthalpy_j_per_kg = None;
        snapshot.mixed_air_minus_supply_enthalpy_j_per_kg = None;
        snapshot.calculated_cooling_sensible_output_w = None;
        snapshot.cooling_sensible_output_w = None;
    }
    values_match && left == right
}

fn capacity_reset_snapshots_match_bit_exact(
    mut left: PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot,
    mut right: PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot,
) -> bool {
    let values_match = [
        option_bits_match(
            left.maximum_total_cooling_capacity_w,
            right.maximum_total_cooling_capacity_w,
        ),
        option_bits_match(
            left.predecessor_supply_mass_flow_rate_for_cool_kg_per_s,
            right.predecessor_supply_mass_flow_rate_for_cool_kg_per_s,
        ),
        option_bits_match(
            left.predecessor_supply_mass_flow_rate_for_dehumidification_kg_per_s,
            right.predecessor_supply_mass_flow_rate_for_dehumidification_kg_per_s,
        ),
        option_bits_match(
            left.predecessor_supply_mass_flow_rate_for_humidification_kg_per_s,
            right.predecessor_supply_mass_flow_rate_for_humidification_kg_per_s,
        ),
        option_bits_match(
            left.assigned_supply_mass_flow_rate_for_cool_kg_per_s,
            right.assigned_supply_mass_flow_rate_for_cool_kg_per_s,
        ),
        option_bits_match(
            left.assigned_supply_mass_flow_rate_for_dehumidification_kg_per_s,
            right.assigned_supply_mass_flow_rate_for_dehumidification_kg_per_s,
        ),
        option_bits_match(
            left.assigned_supply_mass_flow_rate_for_humidification_kg_per_s,
            right.assigned_supply_mass_flow_rate_for_humidification_kg_per_s,
        ),
        option_bits_match(
            left.resulting_supply_mass_flow_rate_for_cool_kg_per_s,
            right.resulting_supply_mass_flow_rate_for_cool_kg_per_s,
        ),
        option_bits_match(
            left.resulting_supply_mass_flow_rate_for_dehumidification_kg_per_s,
            right.resulting_supply_mass_flow_rate_for_dehumidification_kg_per_s,
        ),
        option_bits_match(
            left.resulting_supply_mass_flow_rate_for_humidification_kg_per_s,
            right.resulting_supply_mass_flow_rate_for_humidification_kg_per_s,
        ),
    ]
    .into_iter()
    .all(|matches| matches);
    for snapshot in [&mut left, &mut right] {
        snapshot.maximum_total_cooling_capacity_w = None;
        snapshot.predecessor_supply_mass_flow_rate_for_cool_kg_per_s = None;
        snapshot.predecessor_supply_mass_flow_rate_for_dehumidification_kg_per_s = None;
        snapshot.predecessor_supply_mass_flow_rate_for_humidification_kg_per_s = None;
        snapshot.assigned_supply_mass_flow_rate_for_cool_kg_per_s = None;
        snapshot.assigned_supply_mass_flow_rate_for_dehumidification_kg_per_s = None;
        snapshot.assigned_supply_mass_flow_rate_for_humidification_kg_per_s = None;
        snapshot.resulting_supply_mass_flow_rate_for_cool_kg_per_s = None;
        snapshot.resulting_supply_mass_flow_rate_for_dehumidification_kg_per_s = None;
        snapshot.resulting_supply_mass_flow_rate_for_humidification_kg_per_s = None;
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
