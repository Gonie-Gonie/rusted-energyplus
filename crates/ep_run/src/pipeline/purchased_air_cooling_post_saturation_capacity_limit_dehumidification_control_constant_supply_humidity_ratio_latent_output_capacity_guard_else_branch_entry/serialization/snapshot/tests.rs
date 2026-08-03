use ep_model::{DehumidificationControlType, IdealLoadsAirSystemId, ZoneId};
use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_CAPACITY_GUARD_ELSE_BRANCH_ENTRY_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_CAPACITY_GUARD_ELSE_BRANCH_ENTRY_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_CAPACITY_GUARD_ELSE_BRANCH_ENTRY_SOURCE_ORDER,
};

use super::*;

fn snapshot(
    value: Option<f64>,
    else_entered: bool,
) -> PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputCapacityGuardElseBranchEntrySnapshot{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputCapacityGuardElseBranchEntrySnapshot {
        source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_CAPACITY_GUARD_ELSE_BRANCH_ENTRY_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_CAPACITY_GUARD_ELSE_BRANCH_ENTRY_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_CAPACITY_GUARD_ELSE_BRANCH_ENTRY_SOURCE_ORDER,
        system: IdealLoadsAirSystemId(0),
        parent_call_ordinal: 1,
        controlled_zone: ZoneId(0),
        unit_off_skipped: false,
        non_cooling_skipped: false,
        positive_guard_false_fallthrough_skipped: false,
        heating_availability_guard_false_fallthrough: true,
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
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough: else_entered,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed: false,
        predecessor_cp405_resulting_supply_humidity_ratio: value,
        predecessor_cp405_resulting_supply_enthalpy_j_per_kg: value,
        predecessor_cp405_resulting_supply_temperature_c: value,
        dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entered: else_entered,
        resulting_supply_humidity_ratio: value,
        resulting_supply_enthalpy_j_per_kg: value,
        resulting_supply_temperature_c: value,
    }
}

#[test]
fn compact_snapshot_has_fifty_two_keys_and_six_adjacent_ieee_sidecars() {
    let value = snapshot_json(snapshot(Some(-0.0), true));
    assert!(
        value.is_object(),
        "CP406 snapshot must serialize as an object"
    );
    let Some(object) = value.as_object() else {
        return;
    };
    assert_eq!(object.len(), 52);
    assert_eq!(
        object
            .keys()
            .filter(|key| key.ends_with("_ieee_bits"))
            .count(),
        6
    );
    for field in numeric_fields() {
        assert_eq!(value[format!("{field}_ieee_bits")], "0x8000000000000000");
    }
    assert_eq!(
        value["dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entered"],
        true
    );
}

#[test]
fn nonfinite_projection_is_null_while_payload_bits_remain_authoritative() {
    let nan = f64::from_bits(0x7ff8_0000_0000_0406);
    let value = snapshot_json(snapshot(Some(nan), false));
    for field in numeric_fields() {
        assert!(value[field].is_null(), "{field}");
        assert_eq!(value[format!("{field}_ieee_bits")], "0x7ff8000000000406");
    }
}

fn numeric_fields() -> [&'static str; 6] {
    [
        "predecessor_cp405_resulting_supply_humidity_ratio",
        "predecessor_cp405_resulting_supply_enthalpy_j_per_kg",
        "predecessor_cp405_resulting_supply_temperature_c",
        "resulting_supply_humidity_ratio",
        "resulting_supply_enthalpy_j_per_kg",
        "resulting_supply_temperature_c",
    ]
}
