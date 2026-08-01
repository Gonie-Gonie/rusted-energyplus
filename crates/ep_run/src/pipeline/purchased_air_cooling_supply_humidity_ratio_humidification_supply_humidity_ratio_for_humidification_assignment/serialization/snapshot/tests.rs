use ep_model::{IdealLoadsAirSystemId, ZoneId};
use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_FOR_HUMIDIFICATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_FOR_HUMIDIFICATION_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_FOR_HUMIDIFICATION_ASSIGNMENT_SOURCE_ORDER,
};

use super::*;

#[test]
fn direct_release_serializes_complete_null_values_and_ieee_sidecars() {
    let value = snapshot_json(snapshot(None, false));
    for field in [
        "dehumidification_control_humidistat_supply_humidity_ratio_for_humidification_assignment_executed",
        "dehumidification_control_none_supply_humidity_ratio_for_humidification_assignment_executed",
        "zone_humidifying_setpoint_moisture_demand_read",
        "supply_mass_flow_rate_read",
        "moisture_demand_derived_supply_humidity_ratio_calculated",
        "zone_node_humidity_ratio_read",
        "supply_humidity_ratio_for_humidification_calculated",
        "supply_humidity_ratio_for_humidification_assigned",
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
        f64::from_bits(0x7ff8_0000_0000_0042),
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

fn numeric_fields() -> [&'static str; 10] {
    [
        "predecessor_zone_humidifying_setpoint_moisture_demand_kg_per_s",
        "predecessor_assigned_zone_humidifying_setpoint_moisture_demand_kg_per_s",
        "predecessor_resulting_zone_humidifying_setpoint_moisture_demand_kg_per_s",
        "zone_humidifying_setpoint_moisture_demand_kg_per_s",
        "supply_mass_flow_rate_kg_per_s",
        "moisture_demand_derived_supply_humidity_ratio",
        "zone_node_humidity_ratio",
        "calculated_supply_humidity_ratio_for_humidification",
        "assigned_supply_humidity_ratio_for_humidification",
        "resulting_supply_humidity_ratio_for_humidification",
    ]
}

fn snapshot(
    scalar: Option<f64>,
    active: bool,
) -> PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentSnapshot{
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_FOR_HUMIDIFICATION_ASSIGNMENT_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_FOR_HUMIDIFICATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_FOR_HUMIDIFICATION_ASSIGNMENT_SOURCE_ORDER,
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
        predecessor_humidification_moisture_demand_assignment_executed: active,
        predecessor_zone_humidifying_setpoint_moisture_demand_read: active,
        predecessor_zone_humidifying_setpoint_moisture_demand_kg_per_s: scalar,
        predecessor_zone_humidifying_setpoint_moisture_demand_assigned: active,
        predecessor_assigned_zone_humidifying_setpoint_moisture_demand_kg_per_s: scalar,
        predecessor_resulting_zone_humidifying_setpoint_moisture_demand_kg_per_s: scalar,
        dehumidification_control_humidistat_supply_humidity_ratio_for_humidification_assignment_executed: false,
        dehumidification_control_none_supply_humidity_ratio_for_humidification_assignment_executed: active,
        zone_humidifying_setpoint_moisture_demand_read: active,
        zone_humidifying_setpoint_moisture_demand_kg_per_s: scalar,
        supply_mass_flow_rate_read: active,
        supply_mass_flow_rate_kg_per_s: scalar,
        moisture_demand_derived_supply_humidity_ratio_calculated: active,
        moisture_demand_derived_supply_humidity_ratio: scalar,
        zone_node_humidity_ratio_read: active,
        zone_node_humidity_ratio: scalar,
        supply_humidity_ratio_for_humidification_calculated: active,
        calculated_supply_humidity_ratio_for_humidification: scalar,
        supply_humidity_ratio_for_humidification_assigned: active,
        assigned_supply_humidity_ratio_for_humidification: scalar,
        resulting_supply_humidity_ratio_for_humidification: scalar,
    }
}
