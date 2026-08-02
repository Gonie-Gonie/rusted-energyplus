//! Bit-exact CP398/CP329-to-CP399 lineage reconstruction.

use crate::{
    ideal_loads::{
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_CP_AIR_ASSIGNMENT_SOURCE,
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_CP_AIR_ASSIGNMENT_SOURCE_ORDER,
        PurchasedAirCalcCoolingMixedAirCallSnapshot as MixedAir,
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCaseEntrySnapshot as Predecessor,
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCpAirAssignmentSnapshot as Snapshot,
        cooling_mixed_air_call_snapshot_is_exact_direct_release,
    },
    psychrometrics::energyplus_psy_cp_air_fn_w,
};

pub(super) fn expected_snapshot(predecessor: Predecessor, mixed_air: MixedAir) -> Option<Snapshot> {
    if !cooling_mixed_air_call_snapshot_is_exact_direct_release(mixed_air)
        || mixed_air.system != predecessor.system
        || mixed_air.parent_call_ordinal != predecessor.parent_call_ordinal
        || mixed_air.controlled_zone != predecessor.controlled_zone
    {
        return None;
    }
    let active = predecessor
        .dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered;
    let mixed_air_humidity_ratio = if active {
        let value = mixed_air.mixed_air_humidity_ratio?;
        if !value.is_finite() || value < 0.0 {
            return None;
        }
        Some(value)
    } else {
        None
    };
    let cp_air = mixed_air_humidity_ratio.map(energyplus_psy_cp_air_fn_w);
    if cp_air.is_some_and(|value| !value.is_finite()) {
        return None;
    }

    Some(Snapshot {
        source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_CP_AIR_ASSIGNMENT_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_CP_AIR_ASSIGNMENT_SOURCE_ORDER,
        system: predecessor.system,
        parent_call_ordinal: predecessor.parent_call_ordinal,
        controlled_zone: predecessor.controlled_zone,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        positive_guard_false_fallthrough_skipped: predecessor
            .positive_guard_false_fallthrough_skipped,
        heating_availability_guard_false_fallthrough: predecessor
            .heating_availability_guard_false_fallthrough,
        humidification_control_guard_false_fallthrough: predecessor
            .humidification_control_guard_false_fallthrough,
        dehumidification_control_humidistat_maximum_assignment_executed: predecessor
            .dehumidification_control_humidistat_maximum_assignment_executed,
        dehumidification_control_none_maximum_assignment_executed: predecessor
            .dehumidification_control_none_maximum_assignment_executed,
        dehumidification_control_guard_false_fallthrough: predecessor
            .dehumidification_control_guard_false_fallthrough,
        predecessor_capacity_limit_guard_evaluated: predecessor
            .predecessor_capacity_limit_guard_evaluated,
        predecessor_capacity_limit_body_entered: predecessor.predecessor_capacity_limit_body_entered,
        predecessor_active_capacity_limit_guard_false_fallthrough: predecessor
            .predecessor_active_capacity_limit_guard_false_fallthrough,
        predecessor_dehumidification_guard_evaluated: predecessor
            .predecessor_dehumidification_guard_evaluated,
        predecessor_dehumidification_body_entered: predecessor
            .predecessor_dehumidification_body_entered,
        predecessor_dehumidification_guard_false_fallthrough: predecessor
            .predecessor_dehumidification_guard_false_fallthrough,
        predecessor_dehumidification_total_output_assignment_executed: predecessor
            .predecessor_dehumidification_total_output_assignment_executed,
        predecessor_dehumidification_total_output_capacity_guard_evaluated: predecessor
            .predecessor_dehumidification_total_output_capacity_guard_evaluated,
        predecessor_dehumidification_total_output_capacity_adjustment_body_entered: predecessor
            .predecessor_dehumidification_total_output_capacity_adjustment_body_entered,
        predecessor_dehumidification_total_output_capacity_guard_false_fallthrough: predecessor
            .predecessor_dehumidification_total_output_capacity_guard_false_fallthrough,
        dehumidification_total_output_capacity_guard_false_fallthrough: predecessor
            .dehumidification_total_output_capacity_guard_false_fallthrough,
        dehumidification_total_output_maximum_capacity_assignment_executed: predecessor
            .dehumidification_total_output_maximum_capacity_assignment_executed,
        predecessor_supply_enthalpy_assignment_executed: predecessor
            .predecessor_supply_enthalpy_assignment_executed,
        predecessor_dehumidification_control_type_read: predecessor
            .predecessor_dehumidification_control_type_read,
        predecessor_dehumidification_control_type: predecessor
            .predecessor_dehumidification_control_type,
        predecessor_dehumidification_control_switch_dispatched: predecessor
            .predecessor_dehumidification_control_switch_dispatched,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered: predecessor
            .predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break: predecessor
            .predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break,
        predecessor_dehumidification_control_humidistat_case_entered: predecessor
            .predecessor_dehumidification_control_humidistat_case_entered,
        predecessor_dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed: predecessor
            .predecessor_dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed,
        predecessor_dehumidification_control_humidistat_case_exited_via_break: predecessor
            .predecessor_dehumidification_control_humidistat_case_exited_via_break,
        predecessor_cp397_resulting_supply_humidity_ratio: predecessor
            .predecessor_cp397_resulting_supply_humidity_ratio,
        predecessor_cp397_resulting_supply_enthalpy_j_per_kg: predecessor
            .predecessor_cp397_resulting_supply_enthalpy_j_per_kg,
        predecessor_cp397_resulting_supply_temperature_c: predecessor
            .predecessor_cp397_resulting_supply_temperature_c,
        predecessor_dehumidification_control_none_case_entered: predecessor
            .predecessor_dehumidification_control_none_case_entered,
        predecessor_cp398_resulting_supply_humidity_ratio: predecessor
            .resulting_supply_humidity_ratio,
        predecessor_cp398_resulting_supply_enthalpy_j_per_kg: predecessor
            .resulting_supply_enthalpy_j_per_kg,
        predecessor_cp398_resulting_supply_temperature_c: predecessor
            .resulting_supply_temperature_c,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered:
            active,
        dehumidification_control_none_or_constant_supply_humidity_ratio_cp_air_assignment_executed:
            active,
        mixed_air_humidity_ratio_read: active,
        mixed_air_humidity_ratio,
        psychrometric_cp_air_evaluated: active,
        psychrometric_cp_air_result_j_per_kg_k: cp_air,
        cp_air_assigned: active,
        cp_air_j_per_kg_k: cp_air,
        resulting_supply_humidity_ratio: predecessor.resulting_supply_humidity_ratio,
        resulting_supply_enthalpy_j_per_kg: predecessor.resulting_supply_enthalpy_j_per_kg,
        resulting_supply_temperature_c: predecessor.resulting_supply_temperature_c,
    })
}

pub(super) fn same_snapshot(left: Snapshot, right: Snapshot) -> bool {
    let values_match = [
        (
            left.predecessor_cp397_resulting_supply_humidity_ratio,
            right.predecessor_cp397_resulting_supply_humidity_ratio,
        ),
        (
            left.predecessor_cp397_resulting_supply_enthalpy_j_per_kg,
            right.predecessor_cp397_resulting_supply_enthalpy_j_per_kg,
        ),
        (
            left.predecessor_cp397_resulting_supply_temperature_c,
            right.predecessor_cp397_resulting_supply_temperature_c,
        ),
        (
            left.predecessor_cp398_resulting_supply_humidity_ratio,
            right.predecessor_cp398_resulting_supply_humidity_ratio,
        ),
        (
            left.predecessor_cp398_resulting_supply_enthalpy_j_per_kg,
            right.predecessor_cp398_resulting_supply_enthalpy_j_per_kg,
        ),
        (
            left.predecessor_cp398_resulting_supply_temperature_c,
            right.predecessor_cp398_resulting_supply_temperature_c,
        ),
        (
            left.mixed_air_humidity_ratio,
            right.mixed_air_humidity_ratio,
        ),
        (
            left.psychrometric_cp_air_result_j_per_kg_k,
            right.psychrometric_cp_air_result_j_per_kg_k,
        ),
        (left.cp_air_j_per_kg_k, right.cp_air_j_per_kg_k),
        (
            left.resulting_supply_humidity_ratio,
            right.resulting_supply_humidity_ratio,
        ),
        (
            left.resulting_supply_enthalpy_j_per_kg,
            right.resulting_supply_enthalpy_j_per_kg,
        ),
        (
            left.resulting_supply_temperature_c,
            right.resulting_supply_temperature_c,
        ),
    ]
    .into_iter()
    .all(|(left, right)| options_have_exact_bits(left, right));
    let mut left_without_values = left;
    let mut right_without_values = right;
    for snapshot in [&mut left_without_values, &mut right_without_values] {
        snapshot.predecessor_cp397_resulting_supply_humidity_ratio = None;
        snapshot.predecessor_cp397_resulting_supply_enthalpy_j_per_kg = None;
        snapshot.predecessor_cp397_resulting_supply_temperature_c = None;
        snapshot.predecessor_cp398_resulting_supply_humidity_ratio = None;
        snapshot.predecessor_cp398_resulting_supply_enthalpy_j_per_kg = None;
        snapshot.predecessor_cp398_resulting_supply_temperature_c = None;
        snapshot.mixed_air_humidity_ratio = None;
        snapshot.psychrometric_cp_air_result_j_per_kg_k = None;
        snapshot.cp_air_j_per_kg_k = None;
        snapshot.resulting_supply_humidity_ratio = None;
        snapshot.resulting_supply_enthalpy_j_per_kg = None;
        snapshot.resulting_supply_temperature_c = None;
    }
    values_match && left_without_values == right_without_values
}

fn options_have_exact_bits(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
