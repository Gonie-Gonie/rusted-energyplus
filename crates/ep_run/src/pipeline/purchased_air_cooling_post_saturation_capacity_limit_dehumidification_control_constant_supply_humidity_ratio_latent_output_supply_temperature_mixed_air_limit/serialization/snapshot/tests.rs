use ep_model::{DehumidificationControlType, IdealLoadsAirSystemId, ZoneId};
use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER,
};

use super::*;

#[test]
fn active_snapshot_has_exact_95_key_and_19_sidecar_schema() {
    let value = snapshot_json(snapshot(Some(16.0), Some(13.0), true));
    let object = value.as_object();
    assert!(object.is_some(), "CP408 snapshot object");
    let Some(object) = object else {
        return;
    };
    assert_eq!(object.len(), 95);
    assert_eq!(
        object
            .keys()
            .filter(|key| key.ends_with("_ieee_bits"))
            .count(),
        19
    );
    assert_eq!(value["supply_temperature_before_mixed_air_limit_c"], 16.0);
    assert_eq!(value["mixed_air_temperature_c"], 13.0);
    assert_eq!(value["minimum_supply_temperature_c"], 13.0);
    assert_eq!(value["assigned_supply_temperature_c"], 13.0);
    assert_eq!(value["resulting_supply_temperature_c"], 13.0);
}

#[test]
fn source_shaped_tie_keeps_right_operand_ieee_bits() {
    let value = snapshot_json(snapshot(Some(0.0), Some(-0.0), true));
    for field in [
        "mixed_air_temperature_c",
        "minimum_supply_temperature_c",
        "assigned_supply_temperature_c",
        "resulting_supply_temperature_c",
    ] {
        assert_eq!(
            value[format!("{field}_ieee_bits")],
            "0x8000000000000000",
            "{field}"
        );
    }
}

#[test]
fn unordered_operands_serialize_null_numbers_and_right_payload_bits() {
    let left = f64::from_bits(0x7ff8_0000_0000_1408);
    let right = f64::from_bits(0x7ff8_0000_0000_2408);
    let value = snapshot_json(snapshot(Some(left), Some(right), true));
    for field in [
        "minimum_supply_temperature_c",
        "assigned_supply_temperature_c",
        "resulting_supply_temperature_c",
    ] {
        assert!(value[field].is_null(), "{field}");
        assert_eq!(
            value[format!("{field}_ieee_bits")],
            "0x7ff8000000002408",
            "{field}"
        );
    }
}

#[test]
fn inactive_snapshot_preserves_temperature_and_nulls_local_operands() {
    let value = snapshot_json(snapshot(Some(-0.0), None, false));
    assert_eq!(
        value["preexisting_supply_temperature_c_ieee_bits"],
        "0x8000000000000000"
    );
    assert_eq!(
        value["resulting_supply_temperature_c_ieee_bits"],
        "0x8000000000000000"
    );
    for field in [
        "supply_temperature_before_mixed_air_limit_c",
        "mixed_air_temperature_c",
        "minimum_supply_temperature_c",
        "assigned_supply_temperature_c",
    ] {
        assert!(value[field].is_null(), "{field}");
        assert!(
            value[format!("{field}_ieee_bits")].is_null(),
            "{field} bits"
        );
    }
}

fn snapshot(
    supply_temperature: Option<f64>,
    mixed_air_temperature: Option<f64>,
    active: bool,
) -> PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureMixedAirLimitSnapshot{
    let local_supply = active.then_some(supply_temperature).flatten();
    let local_mixed = active.then_some(mixed_air_temperature).flatten();
    let minimum = match (local_supply, local_mixed) {
        (Some(left), Some(right)) => Some(if left < right { left } else { right }),
        _ => None,
    };
    let humidity = Some(0.008);
    let enthalpy = Some(40_000.0);
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureMixedAirLimitSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER,
        system: IdealLoadsAirSystemId(408),
        parent_call_ordinal: 1,
        controlled_zone: ZoneId(40),
        unit_off_skipped: false,
        non_cooling_skipped: false,
        positive_guard_false_fallthrough_skipped: false,
        heating_availability_guard_false_fallthrough: false,
        humidification_control_guard_false_fallthrough: false,
        dehumidification_control_humidistat_maximum_assignment_executed: false,
        dehumidification_control_none_maximum_assignment_executed: false,
        dehumidification_control_guard_false_fallthrough: false,
        predecessor_capacity_limit_guard_evaluated: true,
        predecessor_capacity_limit_body_entered: true,
        predecessor_active_capacity_limit_guard_false_fallthrough: false,
        predecessor_dehumidification_guard_evaluated: true,
        predecessor_dehumidification_body_entered: true,
        predecessor_dehumidification_guard_false_fallthrough: false,
        predecessor_dehumidification_total_output_assignment_executed: true,
        predecessor_dehumidification_total_output_capacity_guard_evaluated: true,
        predecessor_dehumidification_total_output_capacity_adjustment_body_entered: true,
        predecessor_dehumidification_total_output_capacity_guard_false_fallthrough: false,
        dehumidification_total_output_capacity_guard_false_fallthrough: false,
        dehumidification_total_output_maximum_capacity_assignment_executed: true,
        predecessor_supply_enthalpy_assignment_executed: true,
        predecessor_dehumidification_control_type_read: true,
        predecessor_dehumidification_control_type: Some(DehumidificationControlType::None),
        predecessor_dehumidification_control_switch_dispatched: true,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered: false,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break: false,
        predecessor_dehumidification_control_humidistat_case_entered: false,
        predecessor_dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed: false,
        predecessor_dehumidification_control_humidistat_case_exited_via_break: false,
        predecessor_dehumidification_control_none_case_entered: true,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered: true,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough: active,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed: false,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entered: active,
        predecessor_cp406_resulting_supply_humidity_ratio: humidity,
        predecessor_cp406_resulting_supply_enthalpy_j_per_kg: enthalpy,
        predecessor_cp406_resulting_supply_temperature_c: supply_temperature,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_executed: active,
        predecessor_cp385_retained_supply_enthalpy_owned_read: active,
        predecessor_cp406_same_call_supply_enthalpy_bit_corroborated: active,
        predecessor_supply_enthalpy_for_dry_bulb_inversion_read: active,
        predecessor_supply_enthalpy_j_per_kg: active.then_some(enthalpy).flatten(),
        predecessor_cp378_retained_supply_humidity_ratio_owned_read: active,
        predecessor_supply_humidity_ratio_for_dry_bulb_inversion_read: active,
        predecessor_supply_humidity_ratio: active.then_some(humidity).flatten(),
        predecessor_cp406_retained_supply_temperature_state_owned: supply_temperature.is_some(),
        predecessor_preexisting_supply_temperature_c: supply_temperature,
        predecessor_psychrometric_supply_temperature_evaluated: active,
        predecessor_psychrometric_supply_temperature_result_c: local_supply,
        predecessor_supply_temperature_assigned: active,
        predecessor_assigned_supply_temperature_c: local_supply,
        predecessor_resulting_supply_humidity_ratio: humidity,
        predecessor_resulting_supply_enthalpy_j_per_kg: enthalpy,
        predecessor_resulting_supply_temperature_c: supply_temperature,
        dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit_executed: active,
        cp407_retained_supply_temperature_state_owned: supply_temperature.is_some(),
        preexisting_supply_temperature_c: supply_temperature,
        cp407_retained_supply_temperature_owned_read: active,
        supply_temperature_for_minimum_read: active,
        supply_temperature_before_mixed_air_limit_c: local_supply,
        cp329_retained_mixed_air_temperature_owned_read: active,
        mixed_air_temperature_for_minimum_read: active,
        mixed_air_temperature_c: local_mixed,
        source_shaped_two_argument_minimum_evaluated: active,
        minimum_supply_temperature_c: minimum,
        supply_temperature_assignment_performed: active,
        assigned_supply_temperature_c: minimum,
        resulting_supply_humidity_ratio: humidity,
        resulting_supply_enthalpy_j_per_kg: enthalpy,
        resulting_supply_temperature_c: minimum.or(supply_temperature),
    }
}
