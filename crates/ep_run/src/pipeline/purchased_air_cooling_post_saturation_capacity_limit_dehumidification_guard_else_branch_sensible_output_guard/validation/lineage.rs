//! Lossless CP420-prefix and CP321/CP340 operand-owner validation for CP421.

use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot as CapacityOwner,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot as CapacityCorroborator,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputAssignmentSnapshot as Predecessor,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardSnapshot as Snapshot,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_predecessor_cp420_snapshot,
};

use crate::pipeline::purchased_air_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment::serialization::snapshot::snapshot_json as predecessor_json;

pub(super) fn lineage_is_exact(
    snapshot: Snapshot,
    predecessor: Predecessor,
    capacity: CapacityOwner,
    corroborator: CapacityCorroborator,
) -> bool {
    predecessor_json(
        cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_predecessor_cp420_snapshot(snapshot),
    ) == predecessor_json(predecessor)
        && local_guard_is_exact(snapshot, predecessor, capacity, corroborator)
        && carriers_are_preserved(snapshot, predecessor)
}

fn local_guard_is_exact(
    snapshot: Snapshot,
    predecessor: Predecessor,
    capacity: CapacityOwner,
    corroborator: CapacityCorroborator,
) -> bool {
    let active = predecessor
        .post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_executed;
    if snapshot.source
        != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_GUARD_SOURCE
        || snapshot.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE
        || snapshot.source_order
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_GUARD_SOURCE_ORDER
        || [predecessor.system, capacity.system, corroborator.system]
        .into_iter()
        .any(|system| system != snapshot.system)
        || [
            predecessor.parent_call_ordinal,
            capacity.parent_call_ordinal,
            corroborator.parent_call_ordinal,
        ]
        .into_iter()
        .any(|ordinal| ordinal != snapshot.parent_call_ordinal)
        || [
            predecessor.controlled_zone,
            capacity.controlled_zone,
            corroborator.controlled_zone,
        ]
        .into_iter()
        .any(|zone| zone != snapshot.controlled_zone)
    {
        return false;
    }
    for flag in [
        snapshot.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_guard_evaluated,
        snapshot.cp420_retained_cooling_sensible_output_owned_read,
        snapshot.cooling_sensible_output_read,
        snapshot.cp321_maximum_total_cooling_capacity_owned_read,
        snapshot.cp340_same_call_maximum_total_cooling_capacity_bit_corroborated,
        snapshot.maximum_total_cooling_capacity_read,
        snapshot.cooling_sensible_output_maximum_total_cooling_capacity_comparison_evaluated,
    ] {
        if flag != active {
            return false;
        }
    }
    if snapshot.cp420_retained_supply_humidity_ratio_state_owned
        != predecessor.resulting_supply_humidity_ratio.is_some()
        || snapshot.cp420_retained_supply_enthalpy_state_owned
            != predecessor.resulting_supply_enthalpy_j_per_kg.is_some()
        || snapshot.cp420_retained_supply_temperature_state_owned
            != predecessor.resulting_supply_temperature_c.is_some()
    {
        return false;
    }
    if !active {
        return snapshot
            .cp420_cooling_sensible_output_for_capacity_guard_w
            .is_none()
            && snapshot.maximum_total_cooling_capacity_w.is_none()
            && snapshot
                .cooling_sensible_output_greater_than_or_equal_to_maximum_total_cooling_capacity
                .is_none()
            && !snapshot.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_adjustment_body_entered
            && !snapshot.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_guard_false_fallthrough;
    }
    let (Some(cooling), Some(maximum), Some(corroborating_maximum)) = (
        predecessor.cooling_sensible_output_w,
        capacity.maximum_total_cooling_capacity_w,
        corroborator.maximum_total_cooling_capacity_w,
    ) else {
        return false;
    };
    let comparison = cooling >= maximum;
    capacity.maximum_total_cooling_capacity_read
        && corroborator.maximum_total_cooling_capacity_read
        && option_has_bits(snapshot.cp420_cooling_sensible_output_for_capacity_guard_w, cooling)
        && maximum.to_bits() == corroborating_maximum.to_bits()
        && option_has_bits(snapshot.maximum_total_cooling_capacity_w, maximum)
        && snapshot.cooling_sensible_output_greater_than_or_equal_to_maximum_total_cooling_capacity
            == Some(comparison)
        && snapshot.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_adjustment_body_entered
            == comparison
        && snapshot.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_guard_false_fallthrough
            != comparison
}

fn carriers_are_preserved(snapshot: Snapshot, predecessor: Predecessor) -> bool {
    option_bits_equal(
        snapshot.predecessor_cp420_resulting_supply_humidity_ratio,
        predecessor.resulting_supply_humidity_ratio,
    ) && option_bits_equal(
        snapshot.predecessor_cp420_resulting_supply_enthalpy_j_per_kg,
        predecessor.resulting_supply_enthalpy_j_per_kg,
    ) && option_bits_equal(
        snapshot.predecessor_cp420_resulting_supply_temperature_c,
        predecessor.resulting_supply_temperature_c,
    ) && option_bits_equal(
        snapshot.resulting_supply_humidity_ratio,
        predecessor.resulting_supply_humidity_ratio,
    ) && option_bits_equal(
        snapshot.resulting_supply_enthalpy_j_per_kg,
        predecessor.resulting_supply_enthalpy_j_per_kg,
    ) && option_bits_equal(
        snapshot.resulting_supply_temperature_c,
        predecessor.resulting_supply_temperature_c,
    )
}

fn option_bits_equal(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}

fn option_has_bits(value: Option<f64>, expected: f64) -> bool {
    value.is_some_and(|value| value.to_bits() == expected.to_bits())
}
