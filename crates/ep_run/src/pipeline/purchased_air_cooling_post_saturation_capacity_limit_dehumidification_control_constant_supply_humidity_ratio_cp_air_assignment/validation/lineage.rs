//! Exact CP398-to-CP399 pipeline lineage and CP329 operand-owner validation.

use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_CHILD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_NO_OA_CHILD_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_ENTRY_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_ENTRY_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_ENTRY_SOURCE_ORDER,
    PurchasedAirCalcCoolingMixedAirCallSnapshot as Owner,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCaseEntrySnapshot as Predecessor,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCpAirAssignmentSnapshot as Snapshot,
    psychrometrics::energyplus_psy_cp_air_fn_w,
};

pub(super) fn links_to_predecessor(snapshot: Snapshot, predecessor: Predecessor) -> bool {
    predecessor.source
        == PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_ENTRY_SOURCE
        && predecessor.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_ENTRY_FIRST_EXCLUDED_SOURCE
        && predecessor.source_order
            == PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_ENTRY_SOURCE_ORDER
        && snapshot.system == predecessor.system
        && snapshot.parent_call_ordinal == predecessor.parent_call_ordinal
        && snapshot.controlled_zone == predecessor.controlled_zone
        && control_flags(snapshot) == predecessor_control_flags(predecessor)
        && snapshot.predecessor_dehumidification_control_type
            == predecessor.predecessor_dehumidification_control_type
        && snapshot
            .predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break
            == predecessor
                .predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break
        && snapshot.predecessor_dehumidification_control_humidistat_case_entered
            == predecessor.predecessor_dehumidification_control_humidistat_case_entered
        && snapshot
            .predecessor_dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed
            == predecessor
                .predecessor_dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed
        && snapshot
            .predecessor_dehumidification_control_humidistat_case_exited_via_break
            == predecessor.predecessor_dehumidification_control_humidistat_case_exited_via_break
        && snapshot.predecessor_dehumidification_control_none_case_entered
            == predecessor.predecessor_dehumidification_control_none_case_entered
        && snapshot
            .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered
            == predecessor
                .dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered
        && recursive_values(snapshot)
            .into_iter()
            .zip(predecessor_recursive_values(predecessor))
            .all(|(left, right)| option_bits_equal(left, right))
}

pub(super) fn assignment_shape_is_exact(
    snapshot: Snapshot,
    predecessor: Predecessor,
    owner: Owner,
) -> bool {
    let active = predecessor
        .dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered;
    if snapshot
        .dehumidification_control_none_or_constant_supply_humidity_ratio_cp_air_assignment_executed
        != active
        || snapshot.mixed_air_humidity_ratio_read != active
        || snapshot.psychrometric_cp_air_evaluated != active
        || snapshot.cp_air_assigned != active
    {
        return false;
    }
    if !active {
        return snapshot.mixed_air_humidity_ratio.is_none()
            && snapshot.psychrometric_cp_air_result_j_per_kg_k.is_none()
            && snapshot.cp_air_j_per_kg_k.is_none();
    }

    let Some(humidity_ratio) = owner.mixed_air_humidity_ratio else {
        return false;
    };
    let cp_air = energyplus_psy_cp_air_fn_w(humidity_ratio);
    owner_links_to_assignment(snapshot, owner)
        && humidity_ratio.is_finite()
        && humidity_ratio >= 0.0
        && cp_air.is_finite()
        && option_has_bits(owner.recirculation_humidity_ratio, humidity_ratio)
        && option_has_bits(snapshot.mixed_air_humidity_ratio, humidity_ratio)
        && option_has_bits(snapshot.psychrometric_cp_air_result_j_per_kg_k, cp_air)
        && option_has_bits(snapshot.cp_air_j_per_kg_k, cp_air)
}

pub(super) fn carriers_are_preserved(snapshot: Snapshot, predecessor: Predecessor) -> bool {
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

fn owner_links_to_assignment(snapshot: Snapshot, owner: Owner) -> bool {
    owner.source == PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE
        && owner.child_source == PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_CHILD_SOURCE
        && owner.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_FIRST_EXCLUDED_SOURCE
        && owner.source_order == PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE_ORDER
        && owner.no_oa_child_source_order
            == PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_NO_OA_CHILD_SOURCE_ORDER
        && owner.system == snapshot.system
        && owner.parent_call_ordinal == snapshot.parent_call_ordinal
        && owner.controlled_zone == snapshot.controlled_zone
        && owner.cooling_call_executed
        && owner.no_outdoor_air_fallback_entered
        && owner.recirculation_humidity_ratio_read
        && owner.mixed_air_humidity_ratio_assigned
}

fn recursive_values(snapshot: Snapshot) -> [Option<f64>; 6] {
    [
        snapshot.predecessor_cp397_resulting_supply_humidity_ratio,
        snapshot.predecessor_cp397_resulting_supply_enthalpy_j_per_kg,
        snapshot.predecessor_cp397_resulting_supply_temperature_c,
        snapshot.predecessor_cp398_resulting_supply_humidity_ratio,
        snapshot.predecessor_cp398_resulting_supply_enthalpy_j_per_kg,
        snapshot.predecessor_cp398_resulting_supply_temperature_c,
    ]
}

fn predecessor_recursive_values(snapshot: Predecessor) -> [Option<f64>; 6] {
    [
        snapshot.predecessor_cp397_resulting_supply_humidity_ratio,
        snapshot.predecessor_cp397_resulting_supply_enthalpy_j_per_kg,
        snapshot.predecessor_cp397_resulting_supply_temperature_c,
        snapshot.resulting_supply_humidity_ratio,
        snapshot.resulting_supply_enthalpy_j_per_kg,
        snapshot.resulting_supply_temperature_c,
    ]
}

fn control_flags(snapshot: Snapshot) -> [bool; 24] {
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
        snapshot.predecessor_supply_enthalpy_assignment_executed,
        snapshot.predecessor_dehumidification_control_type_read,
        snapshot.predecessor_dehumidification_control_switch_dispatched,
        snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered,
    ]
}

fn predecessor_control_flags(snapshot: Predecessor) -> [bool; 24] {
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
        snapshot.predecessor_supply_enthalpy_assignment_executed,
        snapshot.predecessor_dehumidification_control_type_read,
        snapshot.predecessor_dehumidification_control_switch_dispatched,
        snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered,
    ]
}

pub(super) fn option_bits_equal(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}

fn option_has_bits(value: Option<f64>, expected: f64) -> bool {
    value.is_some_and(|value| value.to_bits() == expected.to_bits())
}
