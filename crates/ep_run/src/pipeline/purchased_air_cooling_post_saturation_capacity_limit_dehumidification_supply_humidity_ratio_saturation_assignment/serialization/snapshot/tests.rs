use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationAssignmentSnapshot,
    psychrometrics::energyplus_psy_w_fn_tdb_rh_pb,
};

use super::*;
use crate::pipeline::purchased_air_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment::test_snapshot as cp411_snapshot;

#[test]
fn full_lossless_snapshot_has_exact_97_key_and_twenty_sidecar_schema() {
    let value = snapshot_json(snapshot(Some(20.0), true));
    assert_eq!(value.as_object().map(|object| object.len()), Some(97));
    assert_eq!(
        value.as_object().map(|object| {
            object
                .keys()
                .filter(|key| key.ends_with("_ieee_bits"))
                .count()
        }),
        Some(20)
    );
    for field in numeric_fields() {
        assert!(value.get(field).is_some(), "{field} value");
        assert!(
            value.get(format!("{field}_ieee_bits")).is_some(),
            "{field} bits"
        );
    }
}

#[test]
fn inactive_snapshot_omits_all_five_cp412_local_values() {
    let value = snapshot_json(snapshot(Some(-0.0), false));
    for field in local_numeric_fields() {
        assert!(value[field].is_null(), "{field}");
        assert!(
            value[format!("{field}_ieee_bits")].is_null(),
            "{field} bits"
        );
    }
    for field in [
        "predecessor_cp411_resulting_supply_temperature_c",
        "resulting_supply_temperature_c",
    ] {
        assert_eq!(value[format!("{field}_ieee_bits")], "0x8000000000000000");
    }
}

#[test]
fn nonfinite_predecessor_values_project_null_and_preserve_payload_bits() {
    let nan = f64::from_bits(0x7ff8_0000_0000_0412);
    let value = snapshot_json(snapshot(Some(nan), false));
    for field in retained_predecessor_and_carrier_numeric_fields() {
        assert!(value[field].is_null(), "{field}");
        assert_eq!(
            value[format!("{field}_ieee_bits")],
            "0x7ff8000000000412",
            "{field} bits"
        );
    }
    for field in cp411_inactive_local_numeric_fields() {
        assert!(value[field].is_null(), "{field}");
        assert!(
            value[format!("{field}_ieee_bits")].is_null(),
            "{field} bits"
        );
    }
}

fn numeric_fields() -> [&'static str; 20] {
    [
        "predecessor_cp409_resulting_supply_humidity_ratio",
        "predecessor_cp409_resulting_supply_enthalpy_j_per_kg",
        "predecessor_cp409_resulting_supply_temperature_c",
        "predecessor_cp410_resulting_supply_humidity_ratio",
        "predecessor_cp410_resulting_supply_enthalpy_j_per_kg",
        "predecessor_cp410_resulting_supply_temperature_c",
        "purchased_air_supply_humidity_ratio_before_saturation_check",
        "assigned_supply_humidity_ratio_original",
        "resulting_supply_humidity_ratio_original",
        "predecessor_cp411_resulting_supply_humidity_ratio",
        "predecessor_cp411_resulting_supply_enthalpy_j_per_kg",
        "predecessor_cp411_resulting_supply_temperature_c",
        "supply_temperature_for_saturation_humidity_ratio_c",
        "outdoor_barometric_pressure_pa",
        "saturation_supply_humidity_ratio",
        "assigned_saturation_supply_humidity_ratio",
        "resulting_saturation_supply_humidity_ratio",
        "resulting_supply_humidity_ratio",
        "resulting_supply_enthalpy_j_per_kg",
        "resulting_supply_temperature_c",
    ]
}

fn local_numeric_fields() -> [&'static str; 5] {
    [
        "supply_temperature_for_saturation_humidity_ratio_c",
        "outdoor_barometric_pressure_pa",
        "saturation_supply_humidity_ratio",
        "assigned_saturation_supply_humidity_ratio",
        "resulting_saturation_supply_humidity_ratio",
    ]
}

fn retained_predecessor_and_carrier_numeric_fields() -> [&'static str; 12] {
    [
        "predecessor_cp409_resulting_supply_humidity_ratio",
        "predecessor_cp409_resulting_supply_enthalpy_j_per_kg",
        "predecessor_cp409_resulting_supply_temperature_c",
        "predecessor_cp410_resulting_supply_humidity_ratio",
        "predecessor_cp410_resulting_supply_enthalpy_j_per_kg",
        "predecessor_cp410_resulting_supply_temperature_c",
        "predecessor_cp411_resulting_supply_humidity_ratio",
        "predecessor_cp411_resulting_supply_enthalpy_j_per_kg",
        "predecessor_cp411_resulting_supply_temperature_c",
        "resulting_supply_humidity_ratio",
        "resulting_supply_enthalpy_j_per_kg",
        "resulting_supply_temperature_c",
    ]
}

fn cp411_inactive_local_numeric_fields() -> [&'static str; 3] {
    [
        "purchased_air_supply_humidity_ratio_before_saturation_check",
        "assigned_supply_humidity_ratio_original",
        "resulting_supply_humidity_ratio_original",
    ]
}

pub(in crate::pipeline) fn snapshot(
    value: Option<f64>,
    active: bool,
) -> PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationAssignmentSnapshot
{
    let predecessor = cp411_snapshot(value, active);
    let pressure = active.then_some(101_325.0);
    let saturation = match (predecessor.resulting_supply_temperature_c, pressure) {
        (Some(temperature), Some(pressure)) => {
            Some(energyplus_psy_w_fn_tdb_rh_pb(temperature, 1.0, pressure))
        }
        _ => None,
    };
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationAssignmentSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE_ORDER,
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
        predecessor_capacity_limit_body_entered: predecessor
            .predecessor_capacity_limit_body_entered,
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
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered: predecessor
            .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough: predecessor
            .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed: predecessor
            .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_exited_via_break: predecessor
            .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_exited_via_break,
        predecessor_cp409_resulting_supply_humidity_ratio: predecessor
            .predecessor_cp409_resulting_supply_humidity_ratio,
        predecessor_cp409_resulting_supply_enthalpy_j_per_kg: predecessor
            .predecessor_cp409_resulting_supply_enthalpy_j_per_kg,
        predecessor_cp409_resulting_supply_temperature_c: predecessor
            .predecessor_cp409_resulting_supply_temperature_c,
        predecessor_dehumidification_control_default_case_exited_via_break: predecessor
            .predecessor_dehumidification_control_default_case_exited_via_break,
        predecessor_cp410_resulting_supply_humidity_ratio: predecessor
            .predecessor_cp410_resulting_supply_humidity_ratio,
        predecessor_cp410_resulting_supply_enthalpy_j_per_kg: predecessor
            .predecessor_cp410_resulting_supply_enthalpy_j_per_kg,
        predecessor_cp410_resulting_supply_temperature_c: predecessor
            .predecessor_cp410_resulting_supply_temperature_c,
        post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment_executed: predecessor
            .post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment_executed,
        cp410_retained_supply_humidity_ratio_state_owned: predecessor
            .cp410_retained_supply_humidity_ratio_state_owned,
        cp410_retained_supply_enthalpy_state_owned: predecessor
            .cp410_retained_supply_enthalpy_state_owned,
        cp410_retained_supply_temperature_state_owned: predecessor
            .cp410_retained_supply_temperature_state_owned,
        cp410_retained_supply_humidity_ratio_owned_read: predecessor
            .cp410_retained_supply_humidity_ratio_owned_read,
        purchased_air_supply_humidity_ratio_read: predecessor
            .purchased_air_supply_humidity_ratio_read,
        purchased_air_supply_humidity_ratio_before_saturation_check: predecessor
            .purchased_air_supply_humidity_ratio_before_saturation_check,
        local_supply_humidity_ratio_original_assignment_performed: predecessor
            .local_supply_humidity_ratio_original_assignment_performed,
        assigned_supply_humidity_ratio_original: predecessor.assigned_supply_humidity_ratio_original,
        resulting_supply_humidity_ratio_original: predecessor
            .resulting_supply_humidity_ratio_original,
        predecessor_cp411_resulting_supply_humidity_ratio: predecessor
            .resulting_supply_humidity_ratio,
        predecessor_cp411_resulting_supply_enthalpy_j_per_kg: predecessor
            .resulting_supply_enthalpy_j_per_kg,
        predecessor_cp411_resulting_supply_temperature_c: predecessor
            .resulting_supply_temperature_c,
        post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_executed: active,
        cp411_retained_supply_humidity_ratio_state_owned: predecessor
            .resulting_supply_humidity_ratio
            .is_some(),
        cp411_retained_supply_enthalpy_state_owned: predecessor
            .resulting_supply_enthalpy_j_per_kg
            .is_some(),
        cp411_retained_supply_temperature_state_owned: predecessor
            .resulting_supply_temperature_c
            .is_some(),
        cp411_retained_supply_temperature_owned_read: active,
        purchased_air_supply_temperature_for_saturation_humidity_ratio_read: active,
        supply_temperature_for_saturation_humidity_ratio_c: active
            .then_some(predecessor.resulting_supply_temperature_c)
            .flatten(),
        environment_outdoor_barometric_pressure_owned_read: active,
        environment_outdoor_barometric_pressure_for_saturation_humidity_ratio_read: active,
        outdoor_barometric_pressure_pa: pressure,
        psy_w_fn_tdb_rh_pb_at_unity_relative_humidity_evaluated: active,
        saturation_supply_humidity_ratio: saturation,
        local_saturation_supply_humidity_ratio_assignment_performed: active,
        assigned_saturation_supply_humidity_ratio: saturation,
        resulting_saturation_supply_humidity_ratio: saturation,
        resulting_supply_humidity_ratio: predecessor.resulting_supply_humidity_ratio,
        resulting_supply_enthalpy_j_per_kg: predecessor.resulting_supply_enthalpy_j_per_kg,
        resulting_supply_temperature_c: predecessor.resulting_supply_temperature_c,
    }
}
