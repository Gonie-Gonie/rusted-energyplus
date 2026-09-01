//! CP430 structural heating-or-no-load case-entry assertions.

use std::collections::BTreeSet;

use serde_json::{Map, Value, json};

#[path = "cp431_assertions.rs"]
mod cp431_assertions;

const CP429_KEY: &str = "purchased_air_calc_cooling_zero_supply_mass_flow_total_output_positive_zero_assignment_lifecycle";
const CP430_KEY: &str = "purchased_air_calc_heating_or_no_load_case_entry_lifecycle";
const MARKER: &str = "heating_or_no_load_case_entered";
const ORDER: [&str; 1] =
    ["enter-heating-or-no-load-case-after-cooling-entry-gate-false-fallthrough"];
const PUBLIC: [usize; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 20, 21, 26, 27];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let lifecycle = &runtime[CP430_KEY];
    let predecessor = &runtime[CP429_KEY];
    assert_eq!(
        lifecycle["source"],
        json!("EnergyPlus 26.1 PurchasedAirManager.cc:2347")
    );
    assert_eq!(
        lifecycle["first_excluded_source"],
        json!("EnergyPlus 26.1 PurchasedAirManager.cc:2348")
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
    for field in [
        "predecessor_route_counts",
        "heating_or_no_load_case_entry_route_counts",
    ] {
        let values = array(lifecycle, field);
        assert_eq!(values.len(), 36, "CP430 {field} width");
        for (index, value) in values.iter().enumerate() {
            if !PUBLIC.contains(&index) {
                assert_eq!(count_value(value), 0, "CP430 {field} route {index}");
            }
        }
    }
    let routes = array(lifecycle, "predecessor_route_counts");
    let entries = array(lifecycle, "heating_or_no_load_case_entry_route_counts");
    for index in 0..36 {
        assert_eq!(
            count_value(&entries[index]),
            if index == 1 {
                count_value(&routes[index])
            } else {
                0
            },
            "CP430 sole entry route {index}",
        );
    }
    let transitions = sum(routes);
    let entry_count = sum(entries);
    assert_eq!(count(lifecycle, "transition_count"), transitions);
    assert_eq!(
        count(lifecycle, "inactive_transition_count"),
        transitions - entry_count
    );
    assert_eq!(
        count(lifecycle, "heating_or_no_load_case_entry_count"),
        entry_count
    );
    assert_eq!(count(lifecycle, "source_site_execution_count"), entry_count);
    for (owner, preserved, predecessor_owner) in [
        (
            "cp429_supply_humidity_ratio_state_owner_count",
            "unchanged_supply_humidity_ratio_preservation_count",
            "cp428_supply_humidity_ratio_state_owner_count",
        ),
        (
            "cp429_supply_enthalpy_state_owner_count",
            "unchanged_supply_enthalpy_preservation_count",
            "cp428_supply_enthalpy_state_owner_count",
        ),
        (
            "cp429_supply_temperature_state_owner_count",
            "unchanged_supply_temperature_preservation_count",
            "cp428_supply_temperature_state_owner_count",
        ),
    ] {
        let expected = count(predecessor, predecessor_owner);
        assert_eq!(count(lifecycle, owner), expected, "CP430 {owner}");
        assert_eq!(count(lifecycle, preserved), expected, "CP430 {preserved}");
    }

    let latest = lifecycle["latest"]
        .as_object()
        .expect("CP430 latest object");
    let predecessor_latest = predecessor["latest"]
        .as_object()
        .expect("CP429 latest object");
    let mut successor_keys = latest.keys().map(String::as_str).collect::<BTreeSet<_>>();
    assert!(successor_keys.remove(MARKER));
    assert_eq!(
        successor_keys,
        predecessor_latest
            .keys()
            .map(String::as_str)
            .collect::<BTreeSet<_>>()
    );
    assert_eq!(latest.len(), 435);
    assert_eq!(
        latest
            .keys()
            .filter(|key| key.ends_with("_ieee_bits"))
            .count(),
        117
    );
    assert_eq!(latest[MARKER], predecessor_latest["non_cooling_skipped"]);
    for field in [
        "resulting_supply_humidity_ratio_ieee_bits",
        "resulting_supply_enthalpy_j_per_kg_ieee_bits",
        "resulting_supply_temperature_c_ieee_bits",
    ] {
        assert_eq!(latest[field], predecessor_latest[field], "CP430 {field}");
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
                .contains(forbidden),
            "{forbidden}"
        );
    }
    assert!(!results.to_string().contains(CP430_KEY));
    cp431_assertions::assert_direct(runtime, results);
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP430_KEY));
    assert!(
        runtime[CP430_KEY].is_null(),
        "non-direct runtime must not publish CP430 evidence"
    );
    cp431_assertions::assert_non_direct(runtime);
}

fn assert_schema_and_binding_cardinalities() {
    let core =
        include_str!("../../../ep_runtime/src/ideal_loads/calc/heating_or_no_load_case_entry.rs");
    let snapshot = core
        .split_once("pub struct PurchasedAirCalcHeatingOrNoLoadCaseEntrySnapshot")
        .and_then(|(_, tail)| tail.split_once("/// Final selected-unit CP430"))
        .map(|(snapshot, _)| snapshot)
        .expect("CP430 snapshot declaration");
    let fields = snapshot
        .lines()
        .filter(|line| line.trim_start().starts_with("pub "))
        .collect::<Vec<_>>();
    assert_eq!(fields.len(), 318);
    assert_eq!(
        fields
            .iter()
            .filter_map(|line| line.trim().strip_prefix("pub "))
            .filter_map(|line| line.split_once(':').map(|(name, _)| name))
            .collect::<BTreeSet<_>>()
            .len(),
        318
    );
    assert_eq!(snapshot.matches("Option<f64>").count(), 117);
    assert_eq!(snapshot.matches("Option<bool>").count(), 2);
    assert_eq!(snapshot.matches("Option<").count() - 117 - 2, 1);
    let binding = include_str!("../../../ep_runtime/src/ideal_loads/binding/scheduled_output.rs");
    assert_eq!(binding.matches("    pub calculation_").count(), 127);
}

fn array<'a>(lifecycle: &'a Value, field: &str) -> &'a Vec<Value> {
    lifecycle[field].as_array().expect("CP430 route array")
}

fn sum(values: &[Value]) -> u64 {
    values.iter().map(count_value).sum()
}

fn count(lifecycle: &Value, field: &str) -> u64 {
    count_value(&lifecycle[field])
}

fn count_value(value: &Value) -> u64 {
    value.as_u64().expect("CP430 unsigned count")
}
