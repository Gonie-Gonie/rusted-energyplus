use ep_model::{DehumidificationControlType, IdealLoadsAirSystemId, ZoneId};
use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE_ORDER,
};

use super::*;

#[test]
fn compact_snapshot_has_exact_71_key_and_twelve_sidecar_schema() {
    let value = snapshot_json(snapshot(Some(-0.0), true));
    assert_eq!(value.as_object().map(|object| object.len()), Some(71));
    assert_eq!(
        value.as_object().map(|object| {
            object
                .keys()
                .filter(|key| key.ends_with("_ieee_bits"))
                .count()
        }),
        Some(12)
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
fn compact_nonfinite_values_project_null_and_preserve_payload_bits() {
    let nan = f64::from_bits(0x7ff8_0000_0000_0411);
    let value = snapshot_json(snapshot(Some(nan), true));
    for field in numeric_fields() {
        assert!(value[field].is_null(), "{field}");
        assert_eq!(
            value[format!("{field}_ieee_bits")],
            "0x7ff8000000000411",
            "{field} bits"
        );
    }
}

#[test]
fn inactive_snapshot_retains_predecessor_carriers_but_omits_local_values() {
    let value = snapshot_json(snapshot(Some(-0.0), false));
    for field in [
        "purchased_air_supply_humidity_ratio_before_saturation_check",
        "assigned_supply_humidity_ratio_original",
        "resulting_supply_humidity_ratio_original",
    ] {
        assert!(value[field].is_null(), "{field}");
        assert!(
            value[format!("{field}_ieee_bits")].is_null(),
            "{field} bits"
        );
    }
    for field in [
        "predecessor_cp410_resulting_supply_humidity_ratio",
        "resulting_supply_humidity_ratio",
    ] {
        assert_eq!(value[format!("{field}_ieee_bits")], "0x8000000000000000");
    }
}

fn numeric_fields() -> [&'static str; 12] {
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
        "resulting_supply_humidity_ratio",
        "resulting_supply_enthalpy_j_per_kg",
        "resulting_supply_temperature_c",
    ]
}

pub(in crate::pipeline) fn snapshot(
    value: Option<f64>,
    active: bool,
) -> PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioPreSaturationOriginalAssignmentSnapshot
{
    let local = active.then_some(value).flatten();
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioPreSaturationOriginalAssignmentSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE_ORDER,
        system: IdealLoadsAirSystemId(0),
        parent_call_ordinal: 1,
        controlled_zone: ZoneId(0),
        unit_off_skipped: !active,
        non_cooling_skipped: false,
        positive_guard_false_fallthrough_skipped: false,
        heating_availability_guard_false_fallthrough: false,
        humidification_control_guard_false_fallthrough: false,
        dehumidification_control_humidistat_maximum_assignment_executed: false,
        dehumidification_control_none_maximum_assignment_executed: false,
        dehumidification_control_guard_false_fallthrough: false,
        predecessor_capacity_limit_guard_evaluated: active,
        predecessor_capacity_limit_body_entered: active,
        predecessor_active_capacity_limit_guard_false_fallthrough: false,
        predecessor_dehumidification_guard_evaluated: active,
        predecessor_dehumidification_body_entered: active,
        predecessor_dehumidification_guard_false_fallthrough: false,
        predecessor_dehumidification_total_output_assignment_executed: active,
        predecessor_dehumidification_total_output_capacity_guard_evaluated: active,
        predecessor_dehumidification_total_output_capacity_adjustment_body_entered: active,
        predecessor_dehumidification_total_output_capacity_guard_false_fallthrough: false,
        dehumidification_total_output_capacity_guard_false_fallthrough: false,
        dehumidification_total_output_maximum_capacity_assignment_executed: active,
        predecessor_supply_enthalpy_assignment_executed: active,
        predecessor_dehumidification_control_type_read: active,
        predecessor_dehumidification_control_type: active
            .then_some(DehumidificationControlType::None),
        predecessor_dehumidification_control_switch_dispatched: active,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered: false,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break:
            false,
        predecessor_dehumidification_control_humidistat_case_entered: false,
        predecessor_dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed:
            false,
        predecessor_dehumidification_control_humidistat_case_exited_via_break: false,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered:
            active,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough:
            active,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed:
            false,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_exited_via_break:
            active,
        predecessor_cp409_resulting_supply_humidity_ratio: value,
        predecessor_cp409_resulting_supply_enthalpy_j_per_kg: value,
        predecessor_cp409_resulting_supply_temperature_c: value,
        predecessor_dehumidification_control_default_case_exited_via_break: false,
        predecessor_cp410_resulting_supply_humidity_ratio: value,
        predecessor_cp410_resulting_supply_enthalpy_j_per_kg: value,
        predecessor_cp410_resulting_supply_temperature_c: value,
        post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment_executed: active,
        cp410_retained_supply_humidity_ratio_state_owned: value.is_some(),
        cp410_retained_supply_enthalpy_state_owned: value.is_some(),
        cp410_retained_supply_temperature_state_owned: value.is_some(),
        cp410_retained_supply_humidity_ratio_owned_read: active,
        purchased_air_supply_humidity_ratio_read: active,
        purchased_air_supply_humidity_ratio_before_saturation_check: local,
        local_supply_humidity_ratio_original_assignment_performed: active,
        assigned_supply_humidity_ratio_original: local,
        resulting_supply_humidity_ratio_original: local,
        resulting_supply_humidity_ratio: value,
        resulting_supply_enthalpy_j_per_kg: value,
        resulting_supply_temperature_c: value,
    }
}
