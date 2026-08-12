//! Cheap CP418-to-CP419 lineage and CP329 operand-owner validation.

use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_CHILD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE,
    PurchasedAirCalcCoolingMixedAirCallSnapshot as Owner,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchCpAirAssignmentSnapshot as Snapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchEntrySnapshot as Predecessor,
    psychrometrics::energyplus_psy_cp_air_fn_w,
};

use crate::pipeline::{
    purchased_air_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment::serialization::snapshot::snapshot_json,
    purchased_air_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry::serialization::snapshot::snapshot_json as predecessor_json,
};

const CP418_TERMINAL_KEYS: [&str; 7] = [
    "resulting_supply_humidity_ratio",
    "resulting_supply_humidity_ratio_ieee_bits",
    "resulting_supply_enthalpy_j_per_kg",
    "resulting_supply_enthalpy_j_per_kg_ieee_bits",
    "resulting_supply_temperature_c",
    "resulting_supply_temperature_c_ieee_bits",
    "post_saturation_capacity_limit_dehumidification_guard_else_branch_entered",
];

pub(super) fn lineage_is_exact(snapshot: Snapshot, predecessor: Predecessor, owner: Owner) -> bool {
    cp418_prefix_is_exact(snapshot, predecessor)
        && predecessor_tail_is_exact(snapshot, predecessor)
        && local_assignment_is_exact(snapshot, predecessor, owner)
        && carriers_are_preserved(snapshot, predecessor)
}

fn cp418_prefix_is_exact(snapshot: Snapshot, predecessor: Predecessor) -> bool {
    let snapshot_json = snapshot_json(snapshot);
    let predecessor_json = predecessor_json(predecessor);
    let (Some(snapshot), Some(predecessor)) =
        (snapshot_json.as_object(), predecessor_json.as_object())
    else {
        return false;
    };
    predecessor.iter().all(|(key, expected)| {
        matches!(
            key.as_str(),
            "source" | "first_excluded_source" | "source_order"
        ) || CP418_TERMINAL_KEYS.contains(&key.as_str())
            || snapshot.get(key) == Some(expected)
    })
}

fn predecessor_tail_is_exact(snapshot: Snapshot, predecessor: Predecessor) -> bool {
    option_bits_equal(
        snapshot.predecessor_cp418_resulting_supply_humidity_ratio,
        predecessor.resulting_supply_humidity_ratio,
    ) && option_bits_equal(
        snapshot.predecessor_cp418_resulting_supply_enthalpy_j_per_kg,
        predecessor.resulting_supply_enthalpy_j_per_kg,
    ) && option_bits_equal(
        snapshot.predecessor_cp418_resulting_supply_temperature_c,
        predecessor.resulting_supply_temperature_c,
    ) && snapshot
        .predecessor_post_saturation_capacity_limit_dehumidification_guard_else_branch_entered
        == predecessor.post_saturation_capacity_limit_dehumidification_guard_else_branch_entered
}

fn local_assignment_is_exact(snapshot: Snapshot, predecessor: Predecessor, owner: Owner) -> bool {
    let active =
        predecessor.post_saturation_capacity_limit_dehumidification_guard_else_branch_entered;
    if snapshot
        .post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_executed
        != active
        || snapshot.cp329_retained_mixed_air_humidity_ratio_owned_read != active
        || snapshot.mixed_air_humidity_ratio_for_cp_air_read != active
        || snapshot.psychrometric_cp_air_evaluated != active
        || snapshot.cp_air_assigned != active
    {
        return false;
    }
    if !active {
        return snapshot.mixed_air_humidity_ratio_for_cp_air.is_none()
            && snapshot.psychrometric_cp_air_result_j_per_kg_k.is_none()
            && snapshot.cp_air_j_per_kg_k.is_none();
    }

    let Some(humidity_ratio) = owner.mixed_air_humidity_ratio else {
        return false;
    };
    let cp_air = energyplus_psy_cp_air_fn_w(humidity_ratio);
    owner.source == PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE
        && owner.child_source == PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_CHILD_SOURCE
        && owner.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_FIRST_EXCLUDED_SOURCE
        && owner.system == snapshot.system
        && owner.parent_call_ordinal == snapshot.parent_call_ordinal
        && owner.controlled_zone == snapshot.controlled_zone
        && owner.cooling_call_executed
        && owner.no_outdoor_air_fallback_entered
        && owner.mixed_air_humidity_ratio_assigned
        && humidity_ratio.is_finite()
        && humidity_ratio >= 0.0
        && cp_air.is_finite()
        && option_has_bits(snapshot.mixed_air_humidity_ratio_for_cp_air, humidity_ratio)
        && option_has_bits(snapshot.psychrometric_cp_air_result_j_per_kg_k, cp_air)
        && option_has_bits(snapshot.cp_air_j_per_kg_k, cp_air)
}

fn carriers_are_preserved(snapshot: Snapshot, predecessor: Predecessor) -> bool {
    option_bits_equal(
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
