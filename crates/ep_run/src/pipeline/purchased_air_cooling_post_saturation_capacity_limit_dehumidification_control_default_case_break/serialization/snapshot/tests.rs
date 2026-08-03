use ep_model::{DehumidificationControlType, IdealLoadsAirSystemId, ZoneId};
use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_DEFAULT_CASE_BREAK_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_DEFAULT_CASE_BREAK_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_DEFAULT_CASE_BREAK_SOURCE_ORDER,
};

use super::*;

#[test]
fn compact_snapshot_has_exact_52_key_and_six_sidecar_schema() {
    let value = snapshot_json(snapshot(Some(-0.0), true));
    assert_eq!(value.as_object().map(|object| object.len()), Some(52));
    assert_eq!(
        value.as_object().map(|object| {
            object
                .keys()
                .filter(|key| key.ends_with("_ieee_bits"))
                .count()
        }),
        Some(6)
    );
    for field in numeric_fields() {
        assert!(value[field].is_number(), "{field} finite JSON projection");
        assert_eq!(
            value[format!("{field}_ieee_bits")],
            "0x8000000000000000",
            "{field} bits"
        );
    }
    assert_eq!(
        value["predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_exited_via_break"],
        true
    );
    assert_eq!(
        value["dehumidification_control_default_case_exited_via_break"],
        false
    );
}

#[test]
fn compact_nonfinite_carriers_project_null_and_preserve_payload_bits() {
    let nan = f64::from_bits(0x7ff8_0000_0000_0410);
    let value = snapshot_json(snapshot(Some(nan), false));
    for field in numeric_fields() {
        assert!(value[field].is_null(), "{field}");
        assert_eq!(
            value[format!("{field}_ieee_bits")],
            "0x7ff8000000000410",
            "{field} bits"
        );
    }
}

fn numeric_fields() -> [&'static str; 6] {
    [
        "predecessor_cp409_resulting_supply_humidity_ratio",
        "predecessor_cp409_resulting_supply_enthalpy_j_per_kg",
        "predecessor_cp409_resulting_supply_temperature_c",
        "resulting_supply_humidity_ratio",
        "resulting_supply_enthalpy_j_per_kg",
        "resulting_supply_temperature_c",
    ]
}

pub(in crate::pipeline) fn snapshot(
    value: Option<f64>,
    predecessor_shared_break: bool,
) -> PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlDefaultCaseBreakSnapshot
{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlDefaultCaseBreakSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_DEFAULT_CASE_BREAK_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_DEFAULT_CASE_BREAK_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_DEFAULT_CASE_BREAK_SOURCE_ORDER,
        system: IdealLoadsAirSystemId(0),
        parent_call_ordinal: 1,
        controlled_zone: ZoneId(0),
        unit_off_skipped: !predecessor_shared_break,
        non_cooling_skipped: false,
        positive_guard_false_fallthrough_skipped: false,
        heating_availability_guard_false_fallthrough: predecessor_shared_break,
        humidification_control_guard_false_fallthrough: false,
        dehumidification_control_humidistat_maximum_assignment_executed: false,
        dehumidification_control_none_maximum_assignment_executed: false,
        dehumidification_control_guard_false_fallthrough: false,
        predecessor_capacity_limit_guard_evaluated: predecessor_shared_break,
        predecessor_capacity_limit_body_entered: predecessor_shared_break,
        predecessor_active_capacity_limit_guard_false_fallthrough: false,
        predecessor_dehumidification_guard_evaluated: predecessor_shared_break,
        predecessor_dehumidification_body_entered: predecessor_shared_break,
        predecessor_dehumidification_guard_false_fallthrough: false,
        predecessor_dehumidification_total_output_assignment_executed: predecessor_shared_break,
        predecessor_dehumidification_total_output_capacity_guard_evaluated:
            predecessor_shared_break,
        predecessor_dehumidification_total_output_capacity_adjustment_body_entered:
            predecessor_shared_break,
        predecessor_dehumidification_total_output_capacity_guard_false_fallthrough: false,
        dehumidification_total_output_capacity_guard_false_fallthrough: false,
        dehumidification_total_output_maximum_capacity_assignment_executed:
            predecessor_shared_break,
        predecessor_supply_enthalpy_assignment_executed: predecessor_shared_break,
        predecessor_dehumidification_control_type_read: predecessor_shared_break,
        predecessor_dehumidification_control_type: predecessor_shared_break
            .then_some(DehumidificationControlType::None),
        predecessor_dehumidification_control_switch_dispatched: predecessor_shared_break,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered: false,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break:
            false,
        predecessor_dehumidification_control_humidistat_case_entered: false,
        predecessor_dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed:
            false,
        predecessor_dehumidification_control_humidistat_case_exited_via_break: false,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered:
            predecessor_shared_break,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough:
            predecessor_shared_break,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed:
            false,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_exited_via_break:
            predecessor_shared_break,
        predecessor_cp409_resulting_supply_humidity_ratio: value,
        predecessor_cp409_resulting_supply_enthalpy_j_per_kg: value,
        predecessor_cp409_resulting_supply_temperature_c: value,
        dehumidification_control_default_case_exited_via_break: false,
        resulting_supply_humidity_ratio: value,
        resulting_supply_enthalpy_j_per_kg: value,
        resulting_supply_temperature_c: value,
    }
}
