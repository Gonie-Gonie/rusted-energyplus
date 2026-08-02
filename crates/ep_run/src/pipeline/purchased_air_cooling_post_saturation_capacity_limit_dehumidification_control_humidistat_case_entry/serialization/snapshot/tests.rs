use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_CASE_ENTRY_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_CASE_ENTRY_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_CASE_ENTRY_SOURCE_ORDER,
};

use super::*;

#[test]
fn six_compact_carriers_serialize_with_adjacent_ieee_sidecars() {
    let value = snapshot_json(snapshot(Some(-0.0), false));
    for field in [
        "predecessor_cp393_resulting_supply_enthalpy_j_per_kg_ieee_bits",
        "predecessor_cp393_resulting_supply_temperature_c_ieee_bits",
        "resulting_supply_enthalpy_j_per_kg_ieee_bits",
        "resulting_supply_temperature_c_ieee_bits",
    ] {
        assert_eq!(value[field], "0x8000000000000000", "{field}");
    }
    assert!(value["predecessor_cp393_resulting_supply_humidity_ratio"].is_null());
    assert!(value["resulting_supply_humidity_ratio"].is_null());
    assert_eq!(
        value["predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break"],
        false
    );
    assert_eq!(
        value["dehumidification_control_humidistat_case_entered"],
        false
    );
}

#[test]
fn six_nonfinite_carriers_project_null_and_preserve_nan_payload_bits() {
    let nan = f64::from_bits(0x7ff8_0000_0000_0394);
    let value = snapshot_json(snapshot(Some(nan), true));
    for field in [
        "predecessor_cp393_resulting_supply_humidity_ratio",
        "predecessor_cp393_resulting_supply_enthalpy_j_per_kg",
        "predecessor_cp393_resulting_supply_temperature_c",
        "resulting_supply_humidity_ratio",
        "resulting_supply_enthalpy_j_per_kg",
        "resulting_supply_temperature_c",
    ] {
        assert!(value[field].is_null(), "{field}");
        assert_eq!(
            value[format!("{field}_ieee_bits")],
            "0x7ff8000000000394",
            "{field} bits"
        );
    }
    assert_eq!(
        value["predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break"],
        true
    );
    assert_eq!(
        value["dehumidification_control_humidistat_case_entered"],
        false
    );
}

pub(in crate::pipeline) fn snapshot(
    value: Option<f64>,
    predecessor_break_active: bool,
) -> PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatCaseEntrySnapshot{
    let predecessor = crate::pipeline::purchased_air_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_break::test_snapshot(
        value,
        predecessor_break_active,
    );
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatCaseEntrySnapshot {
        source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_CASE_ENTRY_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_CASE_ENTRY_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_CASE_ENTRY_SOURCE_ORDER,
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
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break: predecessor.dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break,
        predecessor_cp393_resulting_supply_humidity_ratio: predecessor.resulting_supply_humidity_ratio,
        predecessor_cp393_resulting_supply_enthalpy_j_per_kg: predecessor.resulting_supply_enthalpy_j_per_kg,
        predecessor_cp393_resulting_supply_temperature_c: predecessor.resulting_supply_temperature_c,
        dehumidification_control_humidistat_case_entered: false,
        resulting_supply_humidity_ratio: predecessor.resulting_supply_humidity_ratio,
        resulting_supply_enthalpy_j_per_kg: predecessor.resulting_supply_enthalpy_j_per_kg,
        resulting_supply_temperature_c: predecessor.resulting_supply_temperature_c,
    }
}
