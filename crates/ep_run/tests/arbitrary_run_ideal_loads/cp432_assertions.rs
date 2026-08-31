//! CP432 structural heating operating-mode Heat-assignment assertions.

use std::collections::BTreeSet;

use serde_json::{Map, Value, json};

#[path = "cp433_assertions.rs"]
mod cp433_assertions;

const CP431_KEY: &str = "purchased_air_calc_heating_mode_guard_lifecycle";
const CP432_KEY: &str = "purchased_air_calc_heating_operating_mode_heat_assignment_lifecycle";
const ORDER: [&str; 1] = ["assign-local-operating-mode-heat"];
const PUBLIC: [usize; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 20, 21, 26, 27];
const TAIL: [&str; 18] = [
    "predecessor_cp431_resulting_supply_humidity_ratio",
    "predecessor_cp431_resulting_supply_humidity_ratio_ieee_bits",
    "predecessor_cp431_resulting_supply_enthalpy_j_per_kg",
    "predecessor_cp431_resulting_supply_enthalpy_j_per_kg_ieee_bits",
    "predecessor_cp431_resulting_supply_temperature_c",
    "predecessor_cp431_resulting_supply_temperature_c_ieee_bits",
    "heating_operating_mode_heat_assignment_executed",
    "cp431_retained_supply_humidity_ratio_state_owned",
    "cp431_retained_supply_enthalpy_state_owned",
    "cp431_retained_supply_temperature_state_owned",
    "heating_operating_mode_heat_assignment_performed",
    "assigned_heating_operating_mode",
    "resulting_supply_humidity_ratio",
    "resulting_supply_humidity_ratio_ieee_bits",
    "resulting_supply_enthalpy_j_per_kg",
    "resulting_supply_enthalpy_j_per_kg_ieee_bits",
    "resulting_supply_temperature_c",
    "resulting_supply_temperature_c_ieee_bits",
];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let lifecycle = &runtime[CP432_KEY];
    let predecessor = &runtime[CP431_KEY];
    assert_eq!(
        lifecycle["source"],
        json!("EnergyPlus 26.1 PurchasedAirManager.cc:2349")
    );
    assert_eq!(
        lifecycle["first_excluded_source"],
        json!("EnergyPlus 26.1 PurchasedAirManager.cc:2351")
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
        lifecycle["predecessor_heating_mode_guard_evaluation_route_counts"],
        predecessor["heating_mode_guard_evaluation_route_counts"]
    );
    assert_eq!(
        lifecycle["predecessor_heating_mode_guard_false_fallthrough_route_counts"],
        predecessor["heating_mode_guard_false_fallthrough_route_counts"]
    );
    assert_eq!(
        lifecycle["heating_operating_mode_heat_assignment_route_counts"],
        predecessor["heating_operating_mode_body_entry_route_counts"]
    );
    for field in [
        "predecessor_route_counts",
        "predecessor_heating_mode_guard_evaluation_route_counts",
        "predecessor_heating_mode_guard_false_fallthrough_route_counts",
        "heating_operating_mode_heat_assignment_route_counts",
    ] {
        let values = array(lifecycle, field);
        assert_eq!(values.len(), 36, "CP432 {field} width");
        for (index, value) in values.iter().enumerate() {
            if !PUBLIC.contains(&index) {
                assert_eq!(count_value(value), 0, "CP432 {field} route {index}");
            }
        }
    }
    let routes = array(lifecycle, "predecessor_route_counts");
    let evaluations = array(
        lifecycle,
        "predecessor_heating_mode_guard_evaluation_route_counts",
    );
    let false_fallthroughs = array(
        lifecycle,
        "predecessor_heating_mode_guard_false_fallthrough_route_counts",
    );
    let assignments = array(
        lifecycle,
        "heating_operating_mode_heat_assignment_route_counts",
    );
    for index in 0..36 {
        assert_eq!(
            count_value(&false_fallthroughs[index]) + count_value(&assignments[index]),
            count_value(&evaluations[index]),
            "CP432 active partition route {index}"
        );
    }
    let transition_count = sum(routes);
    let evaluation_count = sum(evaluations);
    let false_fallthrough_count = sum(false_fallthroughs);
    let assignment_count = sum(assignments);
    assert!(evaluation_count <= transition_count);
    assert_eq!(false_fallthrough_count + assignment_count, evaluation_count);
    assert_eq!(count(lifecycle, "transition_count"), transition_count);
    assert_eq!(
        count(lifecycle, "inactive_transition_count"),
        transition_count - evaluation_count
    );
    assert_eq!(
        count(lifecycle, "predecessor_heating_mode_guard_evaluation_count"),
        evaluation_count
    );
    assert_eq!(
        count(
            lifecycle,
            "predecessor_heating_mode_guard_false_fallthrough_count"
        ),
        false_fallthrough_count
    );
    assert_eq!(
        count(lifecycle, "heating_operating_mode_heat_assignment_count"),
        assignment_count
    );
    for field in [
        "source_site_execution_count",
        "cp432_heating_operating_mode_state_owner_count",
        "heating_operating_mode_assignment_write_count",
    ] {
        assert_eq!(count(lifecycle, field), assignment_count, "CP432 {field}");
    }
    for (owner, preserved, predecessor_owner) in [
        (
            "cp431_supply_humidity_ratio_state_owner_count",
            "unchanged_supply_humidity_ratio_preservation_count",
            "cp430_supply_humidity_ratio_state_owner_count",
        ),
        (
            "cp431_supply_enthalpy_state_owner_count",
            "unchanged_supply_enthalpy_preservation_count",
            "cp430_supply_enthalpy_state_owner_count",
        ),
        (
            "cp431_supply_temperature_state_owner_count",
            "unchanged_supply_temperature_preservation_count",
            "cp430_supply_temperature_state_owner_count",
        ),
    ] {
        let expected = count(predecessor, predecessor_owner);
        assert_eq!(count(lifecycle, owner), expected, "CP432 {owner}");
        assert_eq!(count(lifecycle, preserved), expected, "CP432 {preserved}");
    }

    let latest = lifecycle["latest"]
        .as_object()
        .expect("CP432 latest object");
    let predecessor_latest = predecessor["latest"]
        .as_object()
        .expect("CP431 latest object");
    assert_actual_json_key_set(latest, predecessor_latest);
    assert_eq!(latest.len(), 476);
    assert_eq!(
        latest
            .keys()
            .filter(|key| key.ends_with("_ieee_bits"))
            .count(),
        125
    );
    let executed = predecessor_latest["heating_operating_mode_body_entered"]
        .as_bool()
        .expect("CP431 body marker");
    assert_eq!(
        latest["heating_operating_mode_heat_assignment_executed"],
        json!(executed)
    );
    assert_eq!(
        latest["heating_operating_mode_heat_assignment_performed"],
        json!(executed)
    );
    assert_eq!(
        latest["assigned_heating_operating_mode"],
        if executed {
            json!("Heating")
        } else {
            Value::Null
        }
    );
    for (cp432_field, cp431_field) in [
        (
            "predecessor_cp431_resulting_supply_humidity_ratio_ieee_bits",
            "resulting_supply_humidity_ratio_ieee_bits",
        ),
        (
            "predecessor_cp431_resulting_supply_enthalpy_j_per_kg_ieee_bits",
            "resulting_supply_enthalpy_j_per_kg_ieee_bits",
        ),
        (
            "predecessor_cp431_resulting_supply_temperature_c_ieee_bits",
            "resulting_supply_temperature_c_ieee_bits",
        ),
    ] {
        assert_eq!(latest[cp432_field], predecessor_latest[cp431_field]);
    }
    for field in [
        "resulting_supply_humidity_ratio_ieee_bits",
        "resulting_supply_enthalpy_j_per_kg_ieee_bits",
        "resulting_supply_temperature_c_ieee_bits",
    ] {
        assert_eq!(latest[field], predecessor_latest[field], "CP432 {field}");
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
    assert!(!results.to_string().contains(CP432_KEY));
    cp433_assertions::assert_direct(runtime, results);
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP432_KEY));
    assert!(
        runtime[CP432_KEY].is_null(),
        "non-direct runtime must not publish CP432 evidence"
    );
    cp433_assertions::assert_non_direct(runtime);
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
    assert_eq!(tail.len(), 18);
    expected.extend(tail);
    let actual = latest.keys().map(String::as_str).collect::<BTreeSet<_>>();
    assert_eq!(actual, expected);
    assert_eq!(actual.len(), 476);
}

fn assert_schema_and_binding_cardinalities() {
    let core = include_str!(
        "../../../ep_runtime/src/ideal_loads/calc/heating_operating_mode_heat_assignment.rs"
    );
    let snapshot = core
        .split_once("pub struct PurchasedAirCalcHeatingOperatingModeHeatAssignmentSnapshot")
        .and_then(|(_, tail)| tail.split_once("/// Final selected-unit CP432"))
        .map(|(snapshot, _)| snapshot)
        .expect("CP432 snapshot declaration");
    let fields = snapshot
        .lines()
        .filter(|line| line.trim_start().starts_with("pub "))
        .collect::<Vec<_>>();
    assert_eq!(fields.len(), 351);
    assert_eq!(
        fields
            .iter()
            .filter_map(|line| line.trim().strip_prefix("pub "))
            .filter_map(|line| line.split_once(':').map(|(name, _)| name))
            .collect::<BTreeSet<_>>()
            .len(),
        351
    );
    assert_eq!(snapshot.matches("Option<f64>").count(), 125);
    assert_eq!(snapshot.matches("Option<bool>").count(), 4);
    assert_eq!(snapshot.matches("Option<").count() - 125 - 4, 3);
    let binding = include_str!("../../../ep_runtime/src/ideal_loads/binding/scheduled_output.rs");
    assert_eq!(binding.matches("    pub calculation_").count(), 126);
}

fn array<'a>(lifecycle: &'a Value, field: &str) -> &'a Vec<Value> {
    lifecycle[field].as_array().expect("CP432 route array")
}

fn sum(values: &[Value]) -> u64 {
    values.iter().map(count_value).sum()
}

fn count(lifecycle: &Value, field: &str) -> u64 {
    count_value(&lifecycle[field])
}

fn count_value(value: &Value) -> u64 {
    value.as_u64().expect("CP432 unsigned count")
}
