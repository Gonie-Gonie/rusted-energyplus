//! CP433 structural heating-mode guard else-branch-entry assertions.

use std::collections::BTreeSet;

use serde_json::{Map, Value, json};

#[path = "cp434_assertions.rs"]
mod cp434_assertions;

const CP432_KEY: &str = "purchased_air_calc_heating_operating_mode_heat_assignment_lifecycle";
const CP433_KEY: &str = "purchased_air_calc_heating_mode_guard_else_branch_entry_lifecycle";
const ORDER: [&str; 1] = ["enter-heating-mode-guard-else-branch-after-guard-false-fallthrough"];
const PUBLIC: [usize; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 20, 21, 26, 27];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let lifecycle = &runtime[CP433_KEY];
    let predecessor = &runtime[CP432_KEY];
    assert_eq!(
        lifecycle["source"],
        json!("EnergyPlus 26.1 PurchasedAirManager.cc:2350")
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
        lifecycle["heating_mode_guard_else_branch_entry_route_counts"],
        predecessor["predecessor_heating_mode_guard_false_fallthrough_route_counts"]
    );
    assert_eq!(
        lifecycle
            .as_object()
            .expect("CP433 lifecycle object")
            .keys()
            .filter(|key| key.ends_with("_route_counts"))
            .count(),
        2
    );
    for field in [
        "predecessor_route_counts",
        "heating_mode_guard_else_branch_entry_route_counts",
    ] {
        let values = array(lifecycle, field);
        assert_eq!(values.len(), 36, "CP433 {field} width");
        for (index, value) in values.iter().enumerate() {
            if !PUBLIC.contains(&index) {
                assert_eq!(count_value(value), 0, "CP433 {field} route {index}");
            }
        }
    }
    let transitions = sum(array(lifecycle, "predecessor_route_counts"));
    let entries = sum(array(
        lifecycle,
        "heating_mode_guard_else_branch_entry_route_counts",
    ));
    let inactive = count(predecessor, "inactive_transition_count")
        + count(predecessor, "heating_operating_mode_heat_assignment_count");
    assert_eq!(count(lifecycle, "transition_count"), transitions);
    assert_eq!(count(lifecycle, "inactive_transition_count"), inactive);
    assert_eq!(
        count(lifecycle, "heating_mode_guard_else_branch_entry_count"),
        entries
    );
    assert_eq!(count(lifecycle, "source_site_execution_count"), entries);
    for (owner, preserved, predecessor_owner) in [
        (
            "cp432_supply_humidity_ratio_state_owner_count",
            "unchanged_supply_humidity_ratio_preservation_count",
            "cp431_supply_humidity_ratio_state_owner_count",
        ),
        (
            "cp432_supply_enthalpy_state_owner_count",
            "unchanged_supply_enthalpy_preservation_count",
            "cp431_supply_enthalpy_state_owner_count",
        ),
        (
            "cp432_supply_temperature_state_owner_count",
            "unchanged_supply_temperature_preservation_count",
            "cp431_supply_temperature_state_owner_count",
        ),
    ] {
        let expected = count(predecessor, predecessor_owner);
        assert_eq!(count(lifecycle, owner), expected, "CP433 {owner}");
        assert_eq!(count(lifecycle, preserved), expected, "CP433 {preserved}");
    }

    let latest = lifecycle["latest"]
        .as_object()
        .expect("CP433 latest object");
    let predecessor_latest = predecessor["latest"]
        .as_object()
        .expect("CP432 latest object");
    let mut expected_keys = predecessor_latest
        .keys()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    assert!(expected_keys.insert("heating_mode_guard_else_branch_entered"));
    let actual_keys = latest.keys().map(String::as_str).collect::<BTreeSet<_>>();
    assert_eq!(actual_keys, expected_keys);
    assert_eq!(latest.len(), 477);
    assert_eq!(
        latest
            .keys()
            .filter(|key| key.ends_with("_ieee_bits"))
            .count(),
        125
    );
    for (key, expected) in predecessor_latest {
        if !["source", "first_excluded_source", "source_order"].contains(&key.as_str()) {
            assert_eq!(
                latest.get(key).expect("CP433 retained predecessor key"),
                expected,
                "CP433 predecessor key {key}"
            );
        }
    }
    assert_eq!(
        latest["heating_mode_guard_else_branch_entered"],
        predecessor_latest["heating_mode_guard_false_fallthrough"]
    );
    assert_schema_and_binding_cardinalities();
    for forbidden in [
        "numerical_dto",
        "prediction",
        "feedback",
        "nodes",
        "loads",
        "reports",
        "deadband",
        "calculation.mode",
    ] {
        assert!(
            !lifecycle
                .to_string()
                .to_ascii_lowercase()
                .contains(forbidden)
        );
    }
    assert!(!results.to_string().contains(CP433_KEY));
    cp434_assertions::assert_direct(runtime, results);
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP433_KEY));
    assert!(
        runtime[CP433_KEY].is_null(),
        "non-direct runtime must not publish CP433 evidence"
    );
    cp434_assertions::assert_non_direct(runtime);
}

fn assert_schema_and_binding_cardinalities() {
    let core = include_str!(
        "../../../ep_runtime/src/ideal_loads/calc/heating_mode_guard_else_branch_entry.rs"
    );
    let snapshot = core
        .split_once("pub struct PurchasedAirCalcHeatingModeGuardElseBranchEntrySnapshot")
        .and_then(|(_, tail)| tail.split_once("/// Final selected-unit CP433"))
        .map(|(snapshot, _)| snapshot)
        .expect("CP433 snapshot declaration");
    let fields = snapshot
        .lines()
        .filter(|line| line.trim_start().starts_with("pub "))
        .collect::<Vec<_>>();
    assert_eq!(fields.len(), 352);
    assert_eq!(
        fields
            .iter()
            .filter_map(|line| line.trim().strip_prefix("pub "))
            .filter_map(|line| line.split_once(':').map(|(name, _)| name))
            .collect::<BTreeSet<_>>()
            .len(),
        352
    );
    assert_eq!(snapshot.matches("Option<f64>").count(), 125);
    assert_eq!(snapshot.matches("Option<bool>").count(), 4);
    assert_eq!(snapshot.matches("Option<").count() - 125 - 4, 3);
    let binding = include_str!("../../../ep_runtime/src/ideal_loads/binding/scheduled_output.rs");
    assert_eq!(binding.matches("    pub calculation_").count(), 128);
}

fn array<'a>(lifecycle: &'a Value, field: &str) -> &'a Vec<Value> {
    lifecycle[field].as_array().expect("CP433 route array")
}

fn sum(values: &[Value]) -> u64 {
    values.iter().map(count_value).sum()
}

fn count(lifecycle: &Value, field: &str) -> u64 {
    count_value(&lifecycle[field])
}

fn count_value(value: &Value) -> u64 {
    value.as_u64().expect("CP433 unsigned count")
}
