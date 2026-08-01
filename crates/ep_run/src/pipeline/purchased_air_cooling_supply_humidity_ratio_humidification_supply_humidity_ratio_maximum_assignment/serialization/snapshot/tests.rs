use ep_model::{IdealLoadsAirSystemId, ZoneId};
use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_MAXIMUM_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_MAXIMUM_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_MAXIMUM_ASSIGNMENT_SOURCE_ORDER,
};

use super::*;

#[test]
fn direct_release_serializes_complete_null_values_and_ieee_sidecars() {
    let value = snapshot_json(snapshot(None, false));
    for field in [
        "dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_executed",
        "dehumidification_control_none_supply_humidity_ratio_maximum_assignment_executed",
        "purchased_air_supply_humidity_ratio_for_humidification_supply_maximum_read",
        "supply_humidity_ratio_for_humidification_for_supply_maximum_read",
        "source_shaped_two_argument_maximum_evaluated",
        "purchased_air_supply_humidity_ratio_assignment_performed",
    ] {
        assert_eq!(value[field], false, "{field}");
    }
    for field in numeric_fields() {
        assert!(value[field].is_null(), "{field} value");
        assert!(
            value[format!("{field}_ieee_bits")].is_null(),
            "{field} bits"
        );
    }
}

#[test]
fn finite_and_nonfinite_characterization_preserves_authoritative_bits() {
    for scalar in [
        -0.0,
        f64::from_bits(1),
        f64::from_bits(0x7ff8_0000_0000_0374),
        f64::INFINITY,
    ] {
        let value = snapshot_json(snapshot(Some(scalar), true));
        let expected_bits = format!("0x{:016x}", scalar.to_bits());
        for field in numeric_fields() {
            assert_eq!(value[format!("{field}_ieee_bits")], expected_bits);
            if scalar.is_finite() {
                assert_eq!(value[field], json!(scalar));
            } else {
                assert!(value[field].is_null(), "{field} JSON number");
            }
        }
    }
}

fn numeric_fields() -> [&'static str; 6] {
    [
        "predecessor_resulting_supply_humidity_ratio_for_humidification",
        "purchased_air_supply_humidity_ratio_before_humidification_supply_maximum",
        "supply_humidity_ratio_for_humidification_for_supply_maximum",
        "maximum_supply_humidity_ratio",
        "assigned_supply_humidity_ratio",
        "resulting_supply_humidity_ratio",
    ]
}

fn snapshot(
    scalar: Option<f64>,
    active: bool,
) -> PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentSnapshot{
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_MAXIMUM_ASSIGNMENT_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_MAXIMUM_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_MAXIMUM_ASSIGNMENT_SOURCE_ORDER,
        system: IdealLoadsAirSystemId(0),
        parent_call_ordinal: 1,
        controlled_zone: ZoneId(0),
        unit_body_entered: true,
        predecessor_cooling_body_entered: true,
        predecessor_no_outdoor_air_fallback_entered: true,
        predecessor_positive_supply_mass_flow_body_entered: true,
        unit_off_skipped: false,
        non_cooling_skipped: false,
        positive_guard_false_fallthrough_skipped: false,
        predecessor_dehumidification_control_type: Some(DehumidificationControlType::None),
        predecessor_dehumidification_control_none_case_completed_skip: true,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip: false,
        predecessor_dehumidification_control_humidistat_case_completed_skip: false,
        predecessor_dehumidification_control_constant_supply_humidity_ratio_case_completed_skip: false,
        predecessor_dehumidification_control_default_supply_humidity_ratio_case_exited_via_break: false,
        dehumidification_control_none_case_completed_skip: true,
        dehumidification_control_constant_sensible_heat_ratio_case_completed_skip: false,
        dehumidification_control_humidistat_case_completed_skip: false,
        dehumidification_control_constant_supply_humidity_ratio_case_completed_skip: false,
        predecessor_heating_on_read: true,
        predecessor_heating_on: Some(true),
        predecessor_cooling_supply_humidity_ratio_humidification_body_entered: true,
        predecessor_heating_on_guard_false_fallthrough: false,
        predecessor_humidification_control_type_read: true,
        predecessor_humidification_control_type: Some(if active {
            HumidificationControlType::Humidistat
        } else {
            HumidificationControlType::None
        }),
        predecessor_humidification_control_type_humidistat: Some(active),
        predecessor_humidification_control_body_entered: active,
        predecessor_humidification_control_guard_false_fallthrough: !active,
        predecessor_dehumidification_control_type_first_read: active,
        predecessor_first_dehumidification_control_type: active
            .then_some(DehumidificationControlType::None),
        predecessor_dehumidification_control_type_humidistat: active.then_some(false),
        predecessor_dehumidification_control_type_second_read: active,
        predecessor_second_dehumidification_control_type: active
            .then_some(DehumidificationControlType::None),
        predecessor_dehumidification_control_type_none: active.then_some(true),
        predecessor_dehumidification_control_body_entered: active,
        predecessor_dehumidification_control_guard_false_fallthrough: false,
        predecessor_dehumidification_control_humidistat_supply_humidity_ratio_for_humidification_maximum_limit_executed: false,
        predecessor_dehumidification_control_none_supply_humidity_ratio_for_humidification_maximum_limit_executed: active,
        predecessor_resulting_supply_humidity_ratio_for_humidification: scalar,
        dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_executed: false,
        dehumidification_control_none_supply_humidity_ratio_maximum_assignment_executed: active,
        purchased_air_supply_humidity_ratio_for_humidification_supply_maximum_read: active,
        purchased_air_supply_humidity_ratio_before_humidification_supply_maximum: scalar,
        supply_humidity_ratio_for_humidification_for_supply_maximum_read: active,
        supply_humidity_ratio_for_humidification_for_supply_maximum: scalar,
        source_shaped_two_argument_maximum_evaluated: active,
        maximum_supply_humidity_ratio: scalar,
        purchased_air_supply_humidity_ratio_assignment_performed: active,
        assigned_supply_humidity_ratio: scalar,
        resulting_supply_humidity_ratio: scalar,
    }
}
