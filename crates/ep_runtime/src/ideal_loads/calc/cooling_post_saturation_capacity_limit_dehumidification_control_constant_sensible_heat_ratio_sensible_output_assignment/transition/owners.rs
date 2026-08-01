//! CP384-owned total output and CP385 bridge validation for CP388.

use crate::ideal_loads::calc::{
    cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment_snapshot_is_exact,
    cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_snapshot_is_exact,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentSnapshot as Predecessor,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputMaximumCapacityAssignmentSnapshot as Owner,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentSnapshot as Corroborator,
};

pub(super) fn cooling_total_output_from_exact_owner(
    predecessor: Predecessor,
    owner: Owner,
    corroborator: Corroborator,
) -> Option<f64> {
    if !cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment_snapshot_is_exact(owner)
        || !cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_snapshot_is_exact(corroborator)
        || !same_identity(predecessor, owner, corroborator)
        || !same_control_shape(predecessor, owner, corroborator)
        || !owner.dehumidification_total_output_maximum_capacity_assignment_executed
        || !owner.cp383_retained_maximum_total_cooling_capacity_owned_read
        || !owner.cooling_total_output_assigned
        || !corroborator.supply_enthalpy_assignment_executed
        || !corroborator.cp384_retained_cooling_total_output_owned_read
        || !corroborator.cooling_total_output_read
    {
        return None;
    }
    let total = owner.resulting_cooling_total_output_w?;
    if !option_bits_match(owner.assigned_cooling_total_output_w, Some(total))
        || !option_bits_match(corroborator.cooling_total_output_w, Some(total))
        || !option_bits_match(
            corroborator.resulting_supply_enthalpy_j_per_kg,
            predecessor.resulting_supply_enthalpy_j_per_kg,
        )
    {
        return None;
    }
    Some(total)
}

fn same_identity(predecessor: Predecessor, owner: Owner, corroborator: Corroborator) -> bool {
    predecessor.system == owner.system
        && predecessor.system == corroborator.system
        && predecessor.parent_call_ordinal == owner.parent_call_ordinal
        && predecessor.parent_call_ordinal == corroborator.parent_call_ordinal
        && predecessor.controlled_zone == owner.controlled_zone
        && predecessor.controlled_zone == corroborator.controlled_zone
}

fn same_control_shape(
    predecessor: Predecessor,
    owner: Owner,
    corroborator: Corroborator,
) -> bool {
    let predecessor_flags = [
        predecessor.unit_off_skipped,
        predecessor.non_cooling_skipped,
        predecessor.positive_guard_false_fallthrough_skipped,
        predecessor.heating_availability_guard_false_fallthrough,
        predecessor.humidification_control_guard_false_fallthrough,
        predecessor.dehumidification_control_humidistat_maximum_assignment_executed,
        predecessor.dehumidification_control_none_maximum_assignment_executed,
        predecessor.dehumidification_control_guard_false_fallthrough,
        predecessor.predecessor_capacity_limit_guard_evaluated,
        predecessor.predecessor_capacity_limit_body_entered,
        predecessor.predecessor_active_capacity_limit_guard_false_fallthrough,
        predecessor.predecessor_dehumidification_guard_evaluated,
        predecessor.predecessor_dehumidification_body_entered,
        predecessor.predecessor_dehumidification_guard_false_fallthrough,
        predecessor.predecessor_dehumidification_total_output_assignment_executed,
        predecessor.predecessor_dehumidification_total_output_capacity_guard_evaluated,
        predecessor.predecessor_dehumidification_total_output_capacity_adjustment_body_entered,
        predecessor.predecessor_dehumidification_total_output_capacity_guard_false_fallthrough,
        predecessor.dehumidification_total_output_capacity_guard_false_fallthrough,
        predecessor.dehumidification_total_output_maximum_capacity_assignment_executed,
    ];
    predecessor_flags == owner_flags(owner)
        && predecessor_flags == corroborator_flags(corroborator)
        && predecessor.predecessor_supply_enthalpy_assignment_executed
            == corroborator.supply_enthalpy_assignment_executed
}

fn owner_flags(snapshot: Owner) -> [bool; 20] {
    [
        snapshot.unit_off_skipped,
        snapshot.non_cooling_skipped,
        snapshot.positive_guard_false_fallthrough_skipped,
        snapshot.heating_availability_guard_false_fallthrough,
        snapshot.humidification_control_guard_false_fallthrough,
        snapshot.dehumidification_control_humidistat_maximum_assignment_executed,
        snapshot.dehumidification_control_none_maximum_assignment_executed,
        snapshot.dehumidification_control_guard_false_fallthrough,
        snapshot.predecessor_capacity_limit_guard_evaluated,
        snapshot.predecessor_capacity_limit_body_entered,
        snapshot.predecessor_active_capacity_limit_guard_false_fallthrough,
        snapshot.predecessor_dehumidification_guard_evaluated,
        snapshot.predecessor_dehumidification_body_entered,
        snapshot.predecessor_dehumidification_guard_false_fallthrough,
        snapshot.predecessor_dehumidification_total_output_assignment_executed,
        snapshot.predecessor_dehumidification_total_output_capacity_guard_evaluated,
        snapshot.predecessor_dehumidification_total_output_capacity_adjustment_body_entered,
        snapshot.predecessor_dehumidification_total_output_capacity_guard_false_fallthrough,
        snapshot.dehumidification_total_output_capacity_guard_false_fallthrough,
        snapshot.dehumidification_total_output_maximum_capacity_assignment_executed,
    ]
}

fn corroborator_flags(snapshot: Corroborator) -> [bool; 20] {
    [
        snapshot.unit_off_skipped,
        snapshot.non_cooling_skipped,
        snapshot.positive_guard_false_fallthrough_skipped,
        snapshot.heating_availability_guard_false_fallthrough,
        snapshot.humidification_control_guard_false_fallthrough,
        snapshot.dehumidification_control_humidistat_maximum_assignment_executed,
        snapshot.dehumidification_control_none_maximum_assignment_executed,
        snapshot.dehumidification_control_guard_false_fallthrough,
        snapshot.predecessor_capacity_limit_guard_evaluated,
        snapshot.predecessor_capacity_limit_body_entered,
        snapshot.predecessor_active_capacity_limit_guard_false_fallthrough,
        snapshot.predecessor_dehumidification_guard_evaluated,
        snapshot.predecessor_dehumidification_body_entered,
        snapshot.predecessor_dehumidification_guard_false_fallthrough,
        snapshot.predecessor_dehumidification_total_output_assignment_executed,
        snapshot.predecessor_dehumidification_total_output_capacity_guard_evaluated,
        snapshot.predecessor_dehumidification_total_output_capacity_adjustment_body_entered,
        snapshot.predecessor_dehumidification_total_output_capacity_guard_false_fallthrough,
        snapshot.dehumidification_total_output_capacity_guard_false_fallthrough,
        snapshot.dehumidification_total_output_maximum_capacity_assignment_executed,
    ]
}

fn option_bits_match(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
