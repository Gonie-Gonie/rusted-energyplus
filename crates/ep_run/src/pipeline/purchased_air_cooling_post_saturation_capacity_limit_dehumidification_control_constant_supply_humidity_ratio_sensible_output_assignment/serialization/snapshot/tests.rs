use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER,
};

use super::*;

#[test]
fn twenty_three_lossless_values_serialize_with_exact_ieee_sidecars() {
    let value = snapshot_json(snapshot(Some(-0.0), true));
    let object = value.as_object().expect("CP400 object");
    assert_eq!(
        object
            .keys()
            .filter(|key| key.ends_with("_ieee_bits"))
            .count(),
        23
    );
    for field in numeric_fields() {
        assert!(value[field].is_number(), "{field} finite JSON projection");
        assert_eq!(
            value[format!("{field}_ieee_bits")],
            "0x8000000000000000",
            "{field} bits"
        );
    }
}

#[test]
fn twenty_three_nonfinite_values_project_null_and_keep_payload_bits() {
    let nan = f64::from_bits(0x7ff8_0000_0000_0400);
    let value = snapshot_json(snapshot(Some(nan), true));
    for field in numeric_fields() {
        assert!(value[field].is_null(), "{field}");
        assert_eq!(
            value[format!("{field}_ieee_bits")],
            "0x7ff8000000000400",
            "{field} bits"
        );
    }
}

fn numeric_fields() -> [&'static str; 23] {
    [
        "predecessor_cp397_resulting_supply_humidity_ratio",
        "predecessor_cp397_resulting_supply_enthalpy_j_per_kg",
        "predecessor_cp397_resulting_supply_temperature_c",
        "predecessor_cp398_resulting_supply_humidity_ratio",
        "predecessor_cp398_resulting_supply_enthalpy_j_per_kg",
        "predecessor_cp398_resulting_supply_temperature_c",
        "predecessor_mixed_air_humidity_ratio",
        "predecessor_psychrometric_cp_air_result_j_per_kg_k",
        "predecessor_cp_air_j_per_kg_k",
        "predecessor_cp399_resulting_supply_humidity_ratio",
        "predecessor_cp399_resulting_supply_enthalpy_j_per_kg",
        "predecessor_cp399_resulting_supply_temperature_c",
        "supply_mass_flow_rate_kg_per_s",
        "cp_air_j_per_kg_k",
        "supply_mass_flow_rate_times_cp_air_w_per_k",
        "mixed_air_temperature_c",
        "supply_temperature_c",
        "mixed_air_minus_supply_temperature_k",
        "calculated_cooling_sensible_output_w",
        "cooling_sensible_output_w",
        "resulting_supply_humidity_ratio",
        "resulting_supply_enthalpy_j_per_kg",
        "resulting_supply_temperature_c",
    ]
}

pub(in crate::pipeline) fn snapshot(
    value: Option<f64>,
    active: bool,
) -> PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioSensibleOutputAssignmentSnapshot{
    let predecessor = crate::pipeline::purchased_air_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment::test_snapshot(value, active);
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioSensibleOutputAssignmentSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER,
        system: predecessor.system,
        parent_call_ordinal: predecessor.parent_call_ordinal,
        controlled_zone: predecessor.controlled_zone,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        positive_guard_false_fallthrough_skipped: predecessor.positive_guard_false_fallthrough_skipped,
        heating_availability_guard_false_fallthrough: predecessor.heating_availability_guard_false_fallthrough,
        humidification_control_guard_false_fallthrough: predecessor.humidification_control_guard_false_fallthrough,
        dehumidification_control_humidistat_maximum_assignment_executed: predecessor.dehumidification_control_humidistat_maximum_assignment_executed,
        dehumidification_control_none_maximum_assignment_executed: predecessor.dehumidification_control_none_maximum_assignment_executed,
        dehumidification_control_guard_false_fallthrough: predecessor.dehumidification_control_guard_false_fallthrough,
        predecessor_capacity_limit_guard_evaluated: predecessor.predecessor_capacity_limit_guard_evaluated,
        predecessor_capacity_limit_body_entered: predecessor.predecessor_capacity_limit_body_entered,
        predecessor_active_capacity_limit_guard_false_fallthrough: predecessor.predecessor_active_capacity_limit_guard_false_fallthrough,
        predecessor_dehumidification_guard_evaluated: predecessor.predecessor_dehumidification_guard_evaluated,
        predecessor_dehumidification_body_entered: predecessor.predecessor_dehumidification_body_entered,
        predecessor_dehumidification_guard_false_fallthrough: predecessor.predecessor_dehumidification_guard_false_fallthrough,
        predecessor_dehumidification_total_output_assignment_executed: predecessor.predecessor_dehumidification_total_output_assignment_executed,
        predecessor_dehumidification_total_output_capacity_guard_evaluated: predecessor.predecessor_dehumidification_total_output_capacity_guard_evaluated,
        predecessor_dehumidification_total_output_capacity_adjustment_body_entered: predecessor.predecessor_dehumidification_total_output_capacity_adjustment_body_entered,
        predecessor_dehumidification_total_output_capacity_guard_false_fallthrough: predecessor.predecessor_dehumidification_total_output_capacity_guard_false_fallthrough,
        dehumidification_total_output_capacity_guard_false_fallthrough: predecessor.dehumidification_total_output_capacity_guard_false_fallthrough,
        dehumidification_total_output_maximum_capacity_assignment_executed: predecessor.dehumidification_total_output_maximum_capacity_assignment_executed,
        predecessor_supply_enthalpy_assignment_executed: predecessor.predecessor_supply_enthalpy_assignment_executed,
        predecessor_dehumidification_control_type_read: predecessor.predecessor_dehumidification_control_type_read,
        predecessor_dehumidification_control_type: predecessor.predecessor_dehumidification_control_type,
        predecessor_dehumidification_control_switch_dispatched: predecessor.predecessor_dehumidification_control_switch_dispatched,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered: predecessor.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break: predecessor.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break,
        predecessor_dehumidification_control_humidistat_case_entered: predecessor.predecessor_dehumidification_control_humidistat_case_entered,
        predecessor_dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed: predecessor.predecessor_dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed,
        predecessor_dehumidification_control_humidistat_case_exited_via_break: predecessor.predecessor_dehumidification_control_humidistat_case_exited_via_break,
        predecessor_dehumidification_control_none_case_entered: predecessor.predecessor_dehumidification_control_none_case_entered,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered: predecessor.predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered,
        predecessor_cp397_resulting_supply_humidity_ratio: value,
        predecessor_cp397_resulting_supply_enthalpy_j_per_kg: value,
        predecessor_cp397_resulting_supply_temperature_c: value,
        predecessor_cp398_resulting_supply_humidity_ratio: value,
        predecessor_cp398_resulting_supply_enthalpy_j_per_kg: value,
        predecessor_cp398_resulting_supply_temperature_c: value,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_cp_air_assignment_executed: active,
        predecessor_mixed_air_humidity_ratio_read: active,
        predecessor_mixed_air_humidity_ratio: value,
        predecessor_psychrometric_cp_air_evaluated: active,
        predecessor_psychrometric_cp_air_result_j_per_kg_k: value,
        predecessor_cp_air_assigned: active,
        predecessor_cp_air_j_per_kg_k: value,
        predecessor_cp399_resulting_supply_humidity_ratio: value,
        predecessor_cp399_resulting_supply_enthalpy_j_per_kg: value,
        predecessor_cp399_resulting_supply_temperature_c: value,
        dehumidification_control_none_or_constant_supply_humidity_ratio_sensible_output_assignment_executed: active,
        cp399_retained_supply_humidity_ratio_state_owned: value.is_some(),
        cp399_retained_supply_enthalpy_state_owned: value.is_some(),
        cp399_retained_supply_temperature_state_owned: value.is_some(),
        cp330_retained_supply_mass_flow_rate_owned_read: active,
        cp329_supply_mass_flow_rate_bit_corroborated: active,
        supply_mass_flow_rate_read: active,
        supply_mass_flow_rate_kg_per_s: value,
        cp399_retained_cp_air_owned_read: active,
        cp_air_read: active,
        cp_air_j_per_kg_k: value,
        supply_mass_flow_rate_times_cp_air_calculated: active,
        supply_mass_flow_rate_times_cp_air_w_per_k: value,
        cp329_retained_mixed_air_temperature_owned_read: active,
        mixed_air_temperature_read: active,
        mixed_air_temperature_c: value,
        cp399_retained_supply_temperature_owned_read: active,
        supply_temperature_read: active,
        supply_temperature_c: value,
        mixed_air_minus_supply_temperature_calculated: active,
        mixed_air_minus_supply_temperature_k: value,
        cooling_sensible_output_calculated: active,
        calculated_cooling_sensible_output_w: value,
        cooling_sensible_output_assigned: active,
        cooling_sensible_output_w: value,
        resulting_supply_humidity_ratio: value,
        resulting_supply_enthalpy_j_per_kg: value,
        resulting_supply_temperature_c: value,
    }
}
