//! CP431 structural heating-mode-guard assertions.

use std::collections::BTreeSet;

use serde_json::{Map, Value, json};

#[path = "cp432_assertions.rs"]
mod cp432_assertions;

const CP430_KEY: &str = "purchased_air_calc_heating_or_no_load_case_entry_lifecycle";
const CP431_KEY: &str = "purchased_air_calc_heating_mode_guard_lifecycle";
const ORDER: [&str; 6] = [
    "read-minimum-outdoor-air-sensible-output",
    "read-heating-setpoint-demand",
    "compare-strict-less-than",
    "read-zone-temperature-control-type-after-short-circuit",
    "exclude-exact-single-cooling-control",
    "enter-heating-mode-body-if-admitted",
];
const PUBLIC: [usize; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 20, 21, 26, 27];
const TAIL: [&str; 36] = [
    "predecessor_cp430_resulting_supply_humidity_ratio",
    "predecessor_cp430_resulting_supply_humidity_ratio_ieee_bits",
    "predecessor_cp430_resulting_supply_enthalpy_j_per_kg",
    "predecessor_cp430_resulting_supply_enthalpy_j_per_kg_ieee_bits",
    "predecessor_cp430_resulting_supply_temperature_c",
    "predecessor_cp430_resulting_supply_temperature_c_ieee_bits",
    "heating_or_no_load_case_entered",
    "heating_mode_guard_evaluated",
    "cp311_retained_minimum_outdoor_air_sensible_output_owned_read",
    "cp312_same_call_minimum_outdoor_air_sensible_output_bit_corroborated",
    "minimum_outdoor_air_sensible_output_for_heating_mode_guard_read",
    "minimum_outdoor_air_sensible_output_for_heating_mode_guard_w",
    "minimum_outdoor_air_sensible_output_for_heating_mode_guard_w_ieee_bits",
    "cp310_retained_heating_setpoint_demand_owned_read",
    "heating_setpoint_demand_for_heating_mode_guard_read",
    "heating_setpoint_demand_for_heating_mode_guard_w",
    "heating_setpoint_demand_for_heating_mode_guard_w_ieee_bits",
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
    "resulting_supply_humidity_ratio_ieee_bits",
    "resulting_supply_enthalpy_j_per_kg",
    "resulting_supply_enthalpy_j_per_kg_ieee_bits",
    "resulting_supply_temperature_c",
    "resulting_supply_temperature_c_ieee_bits",
];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let lifecycle = &runtime[CP431_KEY];
    let predecessor = &runtime[CP430_KEY];
    assert_eq!(
        lifecycle["source"],
        json!("EnergyPlus 26.1 PurchasedAirManager.cc:2348")
    );
    assert_eq!(
        lifecycle["first_excluded_source"],
        json!("EnergyPlus 26.1 PurchasedAirManager.cc:2349")
    );
    assert_eq!(lifecycle["latest"]["source_order"], json!(ORDER));
    assert_eq!(lifecycle["system"], predecessor["system"]);
    assert_eq!(
        lifecycle["transition_count"],
        predecessor["transition_count"]
    );
    assert_eq!(
        lifecycle["predecessor_route_counts"],
        predecessor["predecessor_route_counts"]
    );
    assert_eq!(
        lifecycle["heating_mode_guard_evaluation_route_counts"],
        predecessor["heating_or_no_load_case_entry_route_counts"]
    );
    for field in [
        "predecessor_route_counts",
        "heating_mode_guard_evaluation_route_counts",
        "heating_operating_mode_body_entry_route_counts",
        "heating_mode_guard_false_fallthrough_route_counts",
    ] {
        let values = array(lifecycle, field);
        assert_eq!(values.len(), 36, "CP431 {field} width");
        for (index, value) in values.iter().enumerate() {
            if !PUBLIC.contains(&index) {
                assert_eq!(count_value(value), 0, "CP431 {field} route {index}");
            }
        }
    }
    let routes = array(lifecycle, "predecessor_route_counts");
    let evaluations = array(lifecycle, "heating_mode_guard_evaluation_route_counts");
    let bodies = array(lifecycle, "heating_operating_mode_body_entry_route_counts");
    let fallthroughs = array(
        lifecycle,
        "heating_mode_guard_false_fallthrough_route_counts",
    );
    for index in 0..36 {
        assert_eq!(
            count_value(&bodies[index]) + count_value(&fallthroughs[index]),
            if index == 1 {
                count_value(&evaluations[index])
            } else {
                0
            },
            "CP431 active partition route {index}"
        );
    }
    let transitions = sum(routes);
    let evaluation_count = sum(evaluations);
    let body_count = sum(bodies);
    let fallthrough_count = sum(fallthroughs);
    assert_eq!(count(lifecycle, "transition_count"), transitions);
    assert_eq!(
        count(lifecycle, "inactive_transition_count"),
        transitions - evaluation_count
    );
    assert_eq!(
        count(lifecycle, "heating_mode_guard_evaluation_count"),
        evaluation_count
    );
    assert_eq!(
        count(lifecycle, "heating_operating_mode_body_entry_count"),
        body_count
    );
    assert_eq!(
        count(lifecycle, "heating_mode_guard_false_fallthrough_count"),
        fallthrough_count
    );
    assert_eq!(
        count(lifecycle, "source_site_execution_count"),
        3 * (evaluation_count + body_count)
    );
    for field in [
        "cp311_retained_minimum_outdoor_air_sensible_output_owner_read_count",
        "cp312_same_call_minimum_outdoor_air_sensible_output_bit_corroboration_count",
        "minimum_outdoor_air_sensible_output_for_heating_mode_guard_read_count",
        "cp310_retained_heating_setpoint_demand_owner_read_count",
        "heating_setpoint_demand_for_heating_mode_guard_read_count",
        "minimum_outdoor_air_sensible_output_heating_setpoint_demand_comparison_count",
    ] {
        assert_eq!(count(lifecycle, field), evaluation_count, "CP431 {field}");
    }
    for field in [
        "minimum_outdoor_air_sensible_output_strictly_less_than_heating_setpoint_demand_count",
        "prevalidated_temperature_control_type_owner_read_count",
        "temperature_control_type_read_after_sensible_comparison_short_circuit_count",
        "temperature_control_type_single_cool_comparison_count",
        "temperature_control_type_permits_heating_count",
    ] {
        assert_eq!(count(lifecycle, field), body_count, "CP431 {field}");
    }
    assert_eq!(count(lifecycle, "single_cool_block_count"), 0);
    for (owner, preserved, predecessor_owner) in [
        (
            "cp430_supply_humidity_ratio_state_owner_count",
            "unchanged_supply_humidity_ratio_preservation_count",
            "cp429_supply_humidity_ratio_state_owner_count",
        ),
        (
            "cp430_supply_enthalpy_state_owner_count",
            "unchanged_supply_enthalpy_preservation_count",
            "cp429_supply_enthalpy_state_owner_count",
        ),
        (
            "cp430_supply_temperature_state_owner_count",
            "unchanged_supply_temperature_preservation_count",
            "cp429_supply_temperature_state_owner_count",
        ),
    ] {
        let expected = count(predecessor, predecessor_owner);
        assert_eq!(count(lifecycle, owner), expected, "CP431 {owner}");
        assert_eq!(count(lifecycle, preserved), expected, "CP431 {preserved}");
    }

    let latest = lifecycle["latest"]
        .as_object()
        .expect("CP431 latest object");
    let predecessor_latest = predecessor["latest"]
        .as_object()
        .expect("CP430 latest object");
    assert_actual_json_key_set(latest, predecessor_latest);
    assert_eq!(latest.len(), 464);
    assert_eq!(
        latest
            .keys()
            .filter(|key| key.ends_with("_ieee_bits"))
            .count(),
        122
    );
    assert_eq!(
        latest["heating_or_no_load_case_entered"],
        predecessor_latest["heating_or_no_load_case_entered"]
    );
    for (cp431_field, cp430_field) in [
        (
            "predecessor_cp430_resulting_supply_humidity_ratio_ieee_bits",
            "resulting_supply_humidity_ratio_ieee_bits",
        ),
        (
            "predecessor_cp430_resulting_supply_enthalpy_j_per_kg_ieee_bits",
            "resulting_supply_enthalpy_j_per_kg_ieee_bits",
        ),
        (
            "predecessor_cp430_resulting_supply_temperature_c_ieee_bits",
            "resulting_supply_temperature_c_ieee_bits",
        ),
    ] {
        assert_eq!(latest[cp431_field], predecessor_latest[cp430_field]);
    }
    for field in [
        "resulting_supply_humidity_ratio_ieee_bits",
        "resulting_supply_enthalpy_j_per_kg_ieee_bits",
        "resulting_supply_temperature_c_ieee_bits",
    ] {
        assert_eq!(latest[field], predecessor_latest[field], "CP431 {field}");
    }
    assert_schema_and_binding_cardinalities();
    for forbidden in [
        "numerical_dto",
        "prediction",
        "feedback",
        "nodes",
        "loads",
        "reports",
    ] {
        assert!(
            !lifecycle
                .to_string()
                .to_ascii_lowercase()
                .contains(forbidden)
        );
    }
    assert!(!results.to_string().contains(CP431_KEY));
    cp432_assertions::assert_direct(runtime, results);
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP431_KEY));
    assert!(
        runtime[CP431_KEY].is_null(),
        "non-direct runtime must not publish CP431 evidence"
    );
    cp432_assertions::assert_non_direct(runtime);
}

fn assert_actual_json_key_set(
    latest: &Map<String, Value>,
    predecessor_latest: &Map<String, Value>,
) {
    let mut expected = predecessor_latest
        .keys()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    for key in [
        "resulting_supply_humidity_ratio",
        "resulting_supply_humidity_ratio_ieee_bits",
        "resulting_supply_enthalpy_j_per_kg",
        "resulting_supply_enthalpy_j_per_kg_ieee_bits",
        "resulting_supply_temperature_c",
        "resulting_supply_temperature_c_ieee_bits",
    ] {
        assert!(expected.remove(key));
    }
    let tail = TAIL.into_iter().collect::<BTreeSet<_>>();
    assert_eq!(tail.len(), 36);
    expected.extend(tail);
    let actual = latest.keys().map(String::as_str).collect::<BTreeSet<_>>();
    assert_eq!(actual, expected);
    assert_eq!(actual.len(), 464);
}

fn assert_schema_and_binding_cardinalities() {
    let core = include_str!("../../../ep_runtime/src/ideal_loads/calc/heating_mode_guard.rs");
    let snapshot = core
        .split_once("pub struct PurchasedAirCalcHeatingModeGuardSnapshot")
        .and_then(|(_, tail)| tail.split_once("/// Final selected-unit CP431"))
        .map(|(snapshot, _)| snapshot)
        .expect("CP431 snapshot declaration");
    let fields = snapshot
        .lines()
        .filter(|line| line.trim_start().starts_with("pub "))
        .collect::<Vec<_>>();
    assert_eq!(fields.len(), 342);
    assert_eq!(
        fields
            .iter()
            .filter_map(|line| line.trim().strip_prefix("pub "))
            .filter_map(|line| line.split_once(':').map(|(name, _)| name))
            .collect::<BTreeSet<_>>()
            .len(),
        342
    );
    assert_eq!(snapshot.matches("Option<f64>").count(), 122);
    assert_eq!(snapshot.matches("Option<bool>").count(), 4);
    assert_eq!(snapshot.matches("Option<").count() - 122 - 4, 2);
    let binding = include_str!("../../../ep_runtime/src/ideal_loads/binding/scheduled_output.rs");
    assert_eq!(binding.matches("    pub calculation_").count(), 123);
}

fn array<'a>(lifecycle: &'a Value, field: &str) -> &'a Vec<Value> {
    lifecycle[field].as_array().expect("CP431 route array")
}

fn sum(values: &[Value]) -> u64 {
    values.iter().map(count_value).sum()
}

fn count(lifecycle: &Value, field: &str) -> u64 {
    count_value(&lifecycle[field])
}

fn count_value(value: &Value) -> u64 {
    value.as_u64().expect("CP431 unsigned count")
}
