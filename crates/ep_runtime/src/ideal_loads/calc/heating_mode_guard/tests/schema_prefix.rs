//! Exact CP430 prefix and CP431 schema/order tests.

use super::*;

#[test]
fn cp431_schema_is_exact_342_122_4_2_with_locked_tail_order() {
    let cp430 = public_fields(include_str!("../../heating_or_no_load_case_entry.rs"));
    let cp431 = public_fields(include_str!("../../heating_mode_guard.rs"));
    assert_eq!(cp430.len(), 318);
    assert_eq!(cp431.len(), 342);
    assert_eq!(&cp431[..314], &cp430[..314]);
    assert_eq!(
        &cp431[314..],
        &[
            "predecessor_cp430_resulting_supply_humidity_ratio",
            "predecessor_cp430_resulting_supply_enthalpy_j_per_kg",
            "predecessor_cp430_resulting_supply_temperature_c",
            "heating_or_no_load_case_entered",
            "heating_mode_guard_evaluated",
            "cp311_retained_minimum_outdoor_air_sensible_output_owned_read",
            "cp312_same_call_minimum_outdoor_air_sensible_output_bit_corroborated",
            "minimum_outdoor_air_sensible_output_for_heating_mode_guard_read",
            "minimum_outdoor_air_sensible_output_for_heating_mode_guard_w",
            "cp310_retained_heating_setpoint_demand_owned_read",
            "heating_setpoint_demand_for_heating_mode_guard_read",
            "heating_setpoint_demand_for_heating_mode_guard_w",
            "minimum_outdoor_air_sensible_output_heating_setpoint_demand_comparison_evaluated",
            "minimum_outdoor_air_sensible_output_strictly_less_than_heating_setpoint_demand",
            "prevalidated_temperature_control_type_owned_read",
            "temperature_control_type_read_after_sensible_comparison_short_circuit",
            "temperature_control_type",
            "temperature_control_type_single_cool_comparison_evaluated",
            "temperature_control_type_permits_heating",
            "single_cool_blocked",
            "heating_operating_mode_body_entered",
            "heating_mode_guard_false_fallthrough",
            "cp430_retained_supply_humidity_ratio_state_owned",
            "cp430_retained_supply_enthalpy_state_owned",
            "cp430_retained_supply_temperature_state_owned",
            "resulting_supply_humidity_ratio",
            "resulting_supply_enthalpy_j_per_kg",
            "resulting_supply_temperature_c",
        ],
    );
    let source = snapshot_block(include_str!("../../heating_mode_guard.rs"));
    assert_eq!(source.matches("Option<f64>").count(), 122);
    assert_eq!(source.matches("Option<bool>").count(), 4);
    assert_eq!(source.matches("Option<DehumidificationControlType>").count(), 1);
    assert_eq!(source.matches("Option<PurchasedAirTemperatureControlType>").count(), 1);
}

#[test]
fn lossless_prefix_and_bitwise_match_cover_all_61_outcomes() {
    for snapshot in cp431_all_snapshots_for_successor_tests() {
        let predecessor = super::super::heating_mode_guard_predecessor_cp430_snapshot(snapshot);
        assert_eq!(predecessor.system, snapshot.system);
        assert_eq!(predecessor.parent_call_ordinal, snapshot.parent_call_ordinal);
        assert_eq!(
            predecessor.heating_or_no_load_case_entered,
            snapshot.heating_or_no_load_case_entered,
        );
        assert_bits(
            predecessor.resulting_supply_humidity_ratio,
            snapshot.predecessor_cp430_resulting_supply_humidity_ratio,
        );
        assert_bits(
            predecessor.resulting_supply_enthalpy_j_per_kg,
            snapshot.predecessor_cp430_resulting_supply_enthalpy_j_per_kg,
        );
        assert_bits(
            predecessor.resulting_supply_temperature_c,
            snapshot.predecessor_cp430_resulting_supply_temperature_c,
        );
        assert!(super::super::heating_mode_guard_snapshots_match_bit_exact(snapshot, snapshot));
    }
}

fn public_fields(source: &'static str) -> Vec<&'static str> {
    snapshot_block(source)
        .lines()
        .filter_map(|line| line.trim().strip_prefix("pub "))
        .filter_map(|line| line.split_once(':').map(|(field, _)| field))
        .collect()
}

fn snapshot_block(source: &'static str) -> &'static str {
    let start = source
        .find("pub struct PurchasedAirCalc")
        .expect("snapshot start");
    let tail = &source[start..];
    let end = tail.find("\n}\n").expect("snapshot end") + 3;
    &tail[..end]
}
