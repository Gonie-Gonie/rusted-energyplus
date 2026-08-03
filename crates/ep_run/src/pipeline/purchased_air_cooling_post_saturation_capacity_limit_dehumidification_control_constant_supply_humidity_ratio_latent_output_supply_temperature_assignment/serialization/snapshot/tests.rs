use ep_model::{DehumidificationControlType, IdealLoadsAirSystemId, ZoneId};
use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER,
};

use super::*;

#[test]
fn active_assignment_serializes_exact_operands_result_and_width() {
    let enthalpy = 40_000.0;
    let humidity = 0.008;
    let expected = ep_runtime::psychrometrics::energyplus_psy_tdb_fn_h_w(enthalpy, humidity);
    let value = snapshot_json(snapshot(Some((enthalpy, humidity, expected))));
    let object = value.as_object();
    assert!(
        object.is_some(),
        "CP407 snapshot must serialize as an object"
    );
    let Some(object) = object else {
        return;
    };
    assert_eq!(object.len(), 71);
    assert_eq!(
        object
            .keys()
            .filter(|key| key.ends_with("_ieee_bits"))
            .count(),
        11
    );
    for (field, expected) in [
        ("supply_enthalpy_j_per_kg", enthalpy),
        ("supply_humidity_ratio", humidity),
        ("psychrometric_supply_temperature_result_c", expected),
        ("assigned_supply_temperature_c", expected),
        ("resulting_supply_temperature_c", expected),
    ] {
        assert_eq!(value[field], expected, "{field}");
        assert_eq!(
            value[format!("{field}_ieee_bits")],
            format!("0x{:016x}", expected.to_bits()),
            "{field} bits"
        );
    }
    assert!(value["predecessor_cp406_resulting_supply_humidity_ratio"].is_null());
    assert!(value["predecessor_cp406_resulting_supply_humidity_ratio_ieee_bits"].is_null());
}

#[test]
fn inactive_route_serializes_all_cp407_values_as_null() {
    let value = snapshot_json(snapshot(None));
    for field in [
        "supply_enthalpy_j_per_kg",
        "supply_humidity_ratio",
        "preexisting_supply_temperature_c",
        "psychrometric_supply_temperature_result_c",
        "assigned_supply_temperature_c",
        "resulting_supply_humidity_ratio",
        "resulting_supply_enthalpy_j_per_kg",
        "resulting_supply_temperature_c",
    ] {
        assert!(value[field].is_null(), "{field}");
        assert!(
            value[format!("{field}_ieee_bits")].is_null(),
            "{field} bits"
        );
    }
}

#[test]
fn characterized_nonfinite_result_serializes_null_number_and_exact_bits() {
    let enthalpy = f64::NEG_INFINITY;
    let humidity = 0.008;
    let result = ep_runtime::psychrometrics::energyplus_psy_tdb_fn_h_w(enthalpy, humidity);
    let value = snapshot_json(snapshot(Some((enthalpy, humidity, result))));
    for field in [
        "supply_enthalpy_j_per_kg",
        "psychrometric_supply_temperature_result_c",
        "assigned_supply_temperature_c",
        "resulting_supply_temperature_c",
    ] {
        assert!(value[field].is_null(), "{field}");
        assert_eq!(
            value[format!("{field}_ieee_bits")],
            "0xfff0000000000000",
            "{field} bits"
        );
    }
}

fn snapshot(
    active: Option<(f64, f64, f64)>,
) -> PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureAssignmentSnapshot{
    let executed = active.is_some();
    let enthalpy = active.map(|values| values.0);
    let humidity = active.map(|values| values.1);
    let result = active.map(|values| values.2);
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureAssignmentSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER,
        system: IdealLoadsAirSystemId(0),
        parent_call_ordinal: 1,
        controlled_zone: ZoneId(0),
        unit_off_skipped: !executed,
        non_cooling_skipped: false,
        positive_guard_false_fallthrough_skipped: false,
        heating_availability_guard_false_fallthrough: false,
        humidification_control_guard_false_fallthrough: false,
        dehumidification_control_humidistat_maximum_assignment_executed: false,
        dehumidification_control_none_maximum_assignment_executed: false,
        dehumidification_control_guard_false_fallthrough: false,
        predecessor_capacity_limit_guard_evaluated: executed,
        predecessor_capacity_limit_body_entered: executed,
        predecessor_active_capacity_limit_guard_false_fallthrough: false,
        predecessor_dehumidification_guard_evaluated: executed,
        predecessor_dehumidification_body_entered: executed,
        predecessor_dehumidification_guard_false_fallthrough: false,
        predecessor_dehumidification_total_output_assignment_executed: executed,
        predecessor_dehumidification_total_output_capacity_guard_evaluated: executed,
        predecessor_dehumidification_total_output_capacity_adjustment_body_entered: executed,
        predecessor_dehumidification_total_output_capacity_guard_false_fallthrough: false,
        dehumidification_total_output_capacity_guard_false_fallthrough: false,
        dehumidification_total_output_maximum_capacity_assignment_executed: executed,
        predecessor_supply_enthalpy_assignment_executed: executed,
        predecessor_dehumidification_control_type_read: executed,
        predecessor_dehumidification_control_type: executed
            .then_some(DehumidificationControlType::None),
        predecessor_dehumidification_control_switch_dispatched: executed,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered: false,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break:
            false,
        predecessor_dehumidification_control_humidistat_case_entered: false,
        predecessor_dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed:
            false,
        predecessor_dehumidification_control_humidistat_case_exited_via_break: false,
        predecessor_dehumidification_control_none_case_entered: executed,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered:
            executed,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough:
            executed,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed:
            false,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entered:
            executed,
        predecessor_cp406_resulting_supply_humidity_ratio: None,
        predecessor_cp406_resulting_supply_enthalpy_j_per_kg: enthalpy,
        predecessor_cp406_resulting_supply_temperature_c: executed.then_some(16.0),
        dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_executed:
            executed,
        cp385_retained_supply_enthalpy_owned_read: executed,
        cp406_same_call_supply_enthalpy_bit_corroborated: executed,
        supply_enthalpy_for_dry_bulb_inversion_read: executed,
        supply_enthalpy_j_per_kg: enthalpy,
        cp378_retained_supply_humidity_ratio_owned_read: executed,
        supply_humidity_ratio_for_dry_bulb_inversion_read: executed,
        supply_humidity_ratio: humidity,
        cp406_retained_supply_temperature_state_owned: executed,
        preexisting_supply_temperature_c: executed.then_some(16.0),
        psychrometric_supply_temperature_evaluated: executed,
        psychrometric_supply_temperature_result_c: result,
        supply_temperature_assigned: executed,
        assigned_supply_temperature_c: result,
        resulting_supply_humidity_ratio: humidity,
        resulting_supply_enthalpy_j_per_kg: enthalpy,
        resulting_supply_temperature_c: result,
    }
}
