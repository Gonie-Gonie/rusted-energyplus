//! CP434 structural heating operating-mode Deadband-assignment assertions.

use std::collections::BTreeSet;

use serde_json::{Map, Value, json};

#[path = "cp435_assertions.rs"]
mod cp435_assertions;

const CP433_KEY: &str = "purchased_air_calc_heating_mode_guard_else_branch_entry_lifecycle";
const CP434_KEY: &str = "purchased_air_calc_heating_operating_mode_deadband_assignment_lifecycle";
const ORDER: [&str; 1] = ["assign-local-operating-mode-deadband"];
const PUBLIC: [usize; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 20, 21, 26, 27];
const REPLACED_PREDECESSOR_TAIL: [&str; 7] = [
    "resulting_supply_humidity_ratio",
    "resulting_supply_humidity_ratio_ieee_bits",
    "resulting_supply_enthalpy_j_per_kg",
    "resulting_supply_enthalpy_j_per_kg_ieee_bits",
    "resulting_supply_temperature_c",
    "resulting_supply_temperature_c_ieee_bits",
    "heating_mode_guard_else_branch_entered",
];
const TAIL: [&str; 19] = [
    "predecessor_cp433_resulting_supply_humidity_ratio",
    "predecessor_cp433_resulting_supply_humidity_ratio_ieee_bits",
    "predecessor_cp433_resulting_supply_enthalpy_j_per_kg",
    "predecessor_cp433_resulting_supply_enthalpy_j_per_kg_ieee_bits",
    "predecessor_cp433_resulting_supply_temperature_c",
    "predecessor_cp433_resulting_supply_temperature_c_ieee_bits",
    "heating_mode_guard_else_branch_entered",
    "heating_operating_mode_deadband_assignment_executed",
    "cp433_retained_supply_humidity_ratio_state_owned",
    "cp433_retained_supply_enthalpy_state_owned",
    "cp433_retained_supply_temperature_state_owned",
    "heating_operating_mode_deadband_assignment_performed",
    "assigned_heating_operating_mode_deadband",
    "resulting_supply_humidity_ratio",
    "resulting_supply_humidity_ratio_ieee_bits",
    "resulting_supply_enthalpy_j_per_kg",
    "resulting_supply_enthalpy_j_per_kg_ieee_bits",
    "resulting_supply_temperature_c",
    "resulting_supply_temperature_c_ieee_bits",
];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let lifecycle = &runtime[CP434_KEY];
    let predecessor = &runtime[CP433_KEY];
    assert_eq!(
        lifecycle["source"],
        json!("EnergyPlus 26.1 PurchasedAirManager.cc:2351")
    );
    assert_eq!(
        lifecycle["first_excluded_source"],
        json!("EnergyPlus 26.1 PurchasedAirManager.cc:2361")
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
        lifecycle["heating_operating_mode_deadband_assignment_route_counts"],
        predecessor["heating_mode_guard_else_branch_entry_route_counts"]
    );
    assert_eq!(
        lifecycle
            .as_object()
            .expect("CP434 lifecycle object")
            .keys()
            .filter(|key| key.ends_with("_route_counts"))
            .count(),
        2
    );
    for field in [
        "predecessor_route_counts",
        "heating_operating_mode_deadband_assignment_route_counts",
    ] {
        let values = array(lifecycle, field);
        assert_eq!(values.len(), 36, "CP434 {field} width");
        for (index, value) in values.iter().enumerate() {
            if !PUBLIC.contains(&index) {
                assert_eq!(count_value(value), 0, "CP434 {field} route {index}");
            }
        }
    }
    let transitions = sum(array(lifecycle, "predecessor_route_counts"));
    let assignments = sum(array(
        lifecycle,
        "heating_operating_mode_deadband_assignment_route_counts",
    ));
    assert!(assignments <= transitions);
    assert_eq!(count(lifecycle, "transition_count"), transitions);
    assert_eq!(
        count(lifecycle, "inactive_transition_count"),
        transitions - assignments
    );
    for field in [
        "heating_operating_mode_deadband_assignment_count",
        "source_site_execution_count",
        "cp434_heating_operating_mode_state_owner_count",
        "heating_operating_mode_assignment_write_count",
    ] {
        assert_eq!(count(lifecycle, field), assignments, "CP434 {field}");
    }
    for (owner, preserved, predecessor_owner) in [
        (
            "cp433_supply_humidity_ratio_state_owner_count",
            "unchanged_supply_humidity_ratio_preservation_count",
            "cp432_supply_humidity_ratio_state_owner_count",
        ),
        (
            "cp433_supply_enthalpy_state_owner_count",
            "unchanged_supply_enthalpy_preservation_count",
            "cp432_supply_enthalpy_state_owner_count",
        ),
        (
            "cp433_supply_temperature_state_owner_count",
            "unchanged_supply_temperature_preservation_count",
            "cp432_supply_temperature_state_owner_count",
        ),
    ] {
        let expected = count(predecessor, predecessor_owner);
        assert_eq!(count(lifecycle, owner), expected, "CP434 {owner}");
        assert_eq!(count(lifecycle, preserved), expected, "CP434 {preserved}");
    }

    let latest = lifecycle["latest"]
        .as_object()
        .expect("CP434 latest object");
    let predecessor_latest = predecessor["latest"]
        .as_object()
        .expect("CP433 latest object");
    assert_actual_json_key_set(latest, predecessor_latest);
    assert_eq!(latest.len(), 489);
    assert_eq!(
        latest
            .keys()
            .filter(|key| key.ends_with("_ieee_bits"))
            .count(),
        128
    );
    for (key, expected) in predecessor_latest {
        if !["source", "first_excluded_source", "source_order"].contains(&key.as_str())
            && !REPLACED_PREDECESSOR_TAIL.contains(&key.as_str())
        {
            assert_eq!(
                latest.get(key).expect("CP434 retained predecessor key"),
                expected,
                "CP434 predecessor key {key}"
            );
        }
    }
    let executed = predecessor_latest["heating_mode_guard_else_branch_entered"]
        .as_bool()
        .expect("CP433 else-entry marker");
    assert_eq!(
        latest["heating_mode_guard_else_branch_entered"],
        json!(executed)
    );
    assert_eq!(
        latest["heating_operating_mode_deadband_assignment_executed"],
        json!(executed)
    );
    assert_eq!(
        latest["heating_operating_mode_deadband_assignment_performed"],
        json!(executed)
    );
    assert_eq!(
        latest["assigned_heating_operating_mode_deadband"],
        if executed {
            json!("Deadband")
        } else {
            Value::Null
        }
    );
    for (cp434_field, cp433_field) in [
        (
            "predecessor_cp433_resulting_supply_humidity_ratio_ieee_bits",
            "resulting_supply_humidity_ratio_ieee_bits",
        ),
        (
            "predecessor_cp433_resulting_supply_enthalpy_j_per_kg_ieee_bits",
            "resulting_supply_enthalpy_j_per_kg_ieee_bits",
        ),
        (
            "predecessor_cp433_resulting_supply_temperature_c_ieee_bits",
            "resulting_supply_temperature_c_ieee_bits",
        ),
    ] {
        assert_eq!(latest[cp434_field], predecessor_latest[cp433_field]);
    }
    for (owner, predecessor_bits) in [
        (
            "cp433_retained_supply_humidity_ratio_state_owned",
            "resulting_supply_humidity_ratio_ieee_bits",
        ),
        (
            "cp433_retained_supply_enthalpy_state_owned",
            "resulting_supply_enthalpy_j_per_kg_ieee_bits",
        ),
        (
            "cp433_retained_supply_temperature_state_owned",
            "resulting_supply_temperature_c_ieee_bits",
        ),
    ] {
        assert_eq!(
            latest[owner],
            json!(!predecessor_latest[predecessor_bits].is_null()),
            "CP434 {owner}"
        );
    }
    for field in [
        "resulting_supply_humidity_ratio_ieee_bits",
        "resulting_supply_enthalpy_j_per_kg_ieee_bits",
        "resulting_supply_temperature_c_ieee_bits",
    ] {
        assert_eq!(latest[field], predecessor_latest[field], "CP434 {field}");
    }
    assert_schema_and_binding_cardinalities();
    for forbidden in [
        "numerical_dto",
        "prediction",
        "feedback",
        "nodes",
        "loads",
        "reports",
        "calculation.mode",
    ] {
        assert!(
            !lifecycle
                .to_string()
                .to_ascii_lowercase()
                .contains(forbidden)
        );
    }
    assert!(!results.to_string().contains(CP434_KEY));
    cp435_assertions::assert_direct(runtime, results);
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP434_KEY));
    assert!(
        runtime[CP434_KEY].is_null(),
        "non-direct runtime must not publish CP434 evidence"
    );
    cp435_assertions::assert_non_direct(runtime);
}

fn assert_actual_json_key_set(
    latest: &Map<String, Value>,
    predecessor_latest: &Map<String, Value>,
) {
    let mut expected = predecessor_latest
        .keys()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    for key in REPLACED_PREDECESSOR_TAIL {
        assert!(expected.remove(key));
    }
    let tail = TAIL.into_iter().collect::<BTreeSet<_>>();
    assert_eq!(tail.len(), 19);
    expected.extend(tail);
    let actual = latest.keys().map(String::as_str).collect::<BTreeSet<_>>();
    assert_eq!(actual, expected);
    assert_eq!(actual.len(), 489);
}

fn assert_schema_and_binding_cardinalities() {
    let core = include_str!(
        "../../../ep_runtime/src/ideal_loads/calc/heating_operating_mode_deadband_assignment.rs"
    );
    let snapshot = core
        .split_once("pub struct PurchasedAirCalcHeatingOperatingModeDeadbandAssignmentSnapshot")
        .and_then(|(_, tail)| tail.split_once("/// Final selected-unit CP434"))
        .map(|(snapshot, _)| snapshot)
        .expect("CP434 snapshot declaration");
    let fields = snapshot
        .lines()
        .filter(|line| line.trim_start().starts_with("pub "))
        .collect::<Vec<_>>();
    assert_eq!(fields.len(), 361);
    assert_eq!(
        fields
            .iter()
            .filter_map(|line| line.trim().strip_prefix("pub "))
            .filter_map(|line| line.split_once(':').map(|(name, _)| name))
            .collect::<BTreeSet<_>>()
            .len(),
        361
    );
    assert_eq!(snapshot.matches("Option<f64>").count(), 128);
    assert_eq!(snapshot.matches("Option<bool>").count(), 4);
    assert_eq!(snapshot.matches("Option<").count() - 128 - 4, 4);
    let binding = include_str!("../../../ep_runtime/src/ideal_loads/binding/scheduled_output.rs");
    let calculation_fields = binding
        .lines()
        .filter(|line| line.starts_with("    pub calculation_"))
        .collect::<Vec<_>>();
    assert_eq!(calculation_fields.len(), 132);
    assert!(calculation_fields[123].contains("calculation_heating_mode_guard_else_branch_entry"));
    assert!(
        calculation_fields[124].contains("calculation_heating_operating_mode_deadband_assignment")
    );
    assert!(calculation_fields[125].contains("calculation_heating_outdoor_air_maximum_flow_guard"));
    assert!(
        calculation_fields[126]
            .contains("calculation_heating_outdoor_air_maximum_flow_body_volume_flow_assignment")
    );
    assert!(
        calculation_fields[127]
            .contains("calculation_heating_outdoor_air_maximum_flow_first_warning_guard")
    );
    assert!(
        calculation_fields[128].contains(
            "calculation_heating_outdoor_air_maximum_flow_first_warning_counter_increment"
        )
    );
}

fn array<'a>(lifecycle: &'a Value, field: &str) -> &'a Vec<Value> {
    lifecycle[field].as_array().expect("CP434 route array")
}

fn sum(values: &[Value]) -> u64 {
    values.iter().map(count_value).sum()
}

fn count(lifecycle: &Value, field: &str) -> u64 {
    count_value(&lifecycle[field])
}

fn count_value(value: &Value) -> u64 {
    value.as_u64().expect("CP434 unsigned count")
}
