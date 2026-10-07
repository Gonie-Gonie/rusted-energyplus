//! CP442 heating maximum-flow first-warning guard else-entry assertions.

use std::collections::BTreeSet;

use serde_json::{Map, Value, json};

const CP441_KEY: &str =
    "purchased_air_calc_heating_outdoor_air_maximum_flow_continue_warning_timestamp_call_lifecycle";
const CP442_KEY: &str = "purchased_air_calc_heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry_lifecycle";
const MARKER: &str = "heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entered";
const ORDER: [&str; 1] = [
    "enter-heating-outdoor-air-maximum-flow-first-warning-guard-else-branch-after-guard-false-fallthrough",
];
const PUBLIC: [usize; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 20, 21, 26, 27];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let lifecycle = &runtime[CP442_KEY];
    let predecessor = &runtime[CP441_KEY];
    assert_eq!(
        lifecycle["source"],
        json!("EnergyPlus 26.1 PurchasedAirManager.cc:2375")
    );
    assert_eq!(
        lifecycle["first_excluded_source"],
        json!("EnergyPlus 26.1 PurchasedAirManager.cc:2376")
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
        lifecycle["heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry_route_counts"],
        predecessor["predecessor_first_warning_guard_false_fallthrough_route_counts"]
    );

    let route_fields = [
        "predecessor_route_counts",
        "heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry_route_counts",
    ];
    assert_eq!(
        lifecycle
            .as_object()
            .expect("CP442 lifecycle object")
            .keys()
            .filter(|key| key.ends_with("_route_counts"))
            .count(),
        2
    );
    for field in route_fields {
        let values = array(lifecycle, field);
        assert_eq!(values.len(), 36, "CP442 {field} width");
        for (index, value) in values.iter().enumerate() {
            if !PUBLIC.contains(&index) {
                assert_eq!(count_value(value), 0, "CP442 {field} route {index}");
            }
        }
    }
    let entries = sum(array(
        lifecycle,
        "heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry_route_counts",
    ));
    assert_eq!(entries, 0);
    assert_eq!(
        count(lifecycle, "inactive_transition_count"),
        count(lifecycle, "transition_count")
    );
    for field in [
        "heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry_count",
        "source_site_execution_count",
    ] {
        assert_eq!(count(lifecycle, field), 0, "CP442 {field}");
    }
    for (owner, predecessor_preservation, preservation) in [
        (
            "cp441_supply_humidity_ratio_state_owner_count",
            "unchanged_supply_humidity_ratio_preservation_count",
            "unchanged_supply_humidity_ratio_preservation_count",
        ),
        (
            "cp441_supply_enthalpy_state_owner_count",
            "unchanged_supply_enthalpy_preservation_count",
            "unchanged_supply_enthalpy_preservation_count",
        ),
        (
            "cp441_supply_temperature_state_owner_count",
            "unchanged_supply_temperature_preservation_count",
            "unchanged_supply_temperature_preservation_count",
        ),
    ] {
        assert_eq!(lifecycle[owner], predecessor[predecessor_preservation]);
        assert_eq!(lifecycle[preservation], lifecycle[owner]);
    }
    let lifecycle_object = lifecycle.as_object().expect("CP442 lifecycle object");
    for forbidden in [
        "cp441_outdoor_air_flow_maximum_heating_output_error_count_state_owner_count",
        "unchanged_outdoor_air_flow_maximum_heating_output_error_count_preservation_count",
    ] {
        assert!(
            !lifecycle_object.contains_key(forbidden),
            "CP442 {forbidden}"
        );
    }

    let latest = lifecycle["latest"]
        .as_object()
        .expect("CP442 latest object");
    let predecessor_latest = predecessor["latest"]
        .as_object()
        .expect("CP441 latest object");
    assert_actual_json_key_set(latest, predecessor_latest);
    for (key, value) in predecessor_latest {
        if !["source", "first_excluded_source", "source_order"].contains(&key.as_str()) {
            assert_eq!(
                latest.get(key),
                Some(value),
                "CP442 changed CP441 field {key}"
            );
        }
    }
    assert_eq!(latest[MARKER], json!(false));
    assert_schema_and_binding_cardinalities();
    assert!(!results.to_string().contains(CP442_KEY));
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP442_KEY));
    assert!(
        runtime[CP442_KEY].is_null(),
        "non-direct runtime must not publish CP442 evidence"
    );
}

fn assert_actual_json_key_set(
    latest: &Map<String, Value>,
    predecessor_latest: &Map<String, Value>,
) {
    let mut expected = predecessor_latest.keys().cloned().collect::<BTreeSet<_>>();
    assert!(expected.insert(MARKER.to_string()));
    let actual = latest.keys().cloned().collect::<BTreeSet<_>>();
    assert_eq!(actual, expected);
    assert_eq!(actual.len(), 576);
    assert_eq!(
        actual
            .iter()
            .filter(|key| key.ends_with("_ieee_bits"))
            .count(),
        146
    );
}

fn assert_schema_and_binding_cardinalities() {
    let core = include_str!(
        "../../../ep_runtime/src/ideal_loads/calc/heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry.rs"
    );
    let snapshot = core
        .split_once(
            "pub struct PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardElseBranchEntrySnapshot",
        )
        .and_then(|(_, tail)| tail.split_once("/// Final selected-unit CP442"))
        .map(|(snapshot, _)| snapshot)
        .expect("CP442 snapshot declaration");
    let fields = snapshot
        .lines()
        .filter(|line| line.trim_start().starts_with("pub "))
        .collect::<Vec<_>>();
    assert_eq!(fields.len(), 430);
    assert_eq!(
        fields
            .iter()
            .filter_map(|line| line.trim().strip_prefix("pub "))
            .filter_map(|line| line.split_once(':').map(|(name, _)| name))
            .collect::<BTreeSet<_>>()
            .len(),
        430
    );
    assert_eq!(snapshot.matches("Option<f64>").count(), 146);
    assert_eq!(snapshot.matches("Option<bool>").count(), 9);
    assert_eq!(snapshot.matches("Option<usize>").count(), 2);
    assert_eq!(snapshot.matches("Option<").count() - 146 - 9 - 2, 6);
    let binding = include_str!("../../../ep_runtime/src/ideal_loads/binding/scheduled_output.rs");
    let fields = binding
        .lines()
        .filter(|line| line.contains("pub calculation_"))
        .collect::<Vec<_>>();
    assert_eq!(fields.len(), 133);
    assert!(
        fields[131].contains(
            "calculation_heating_outdoor_air_maximum_flow_continue_warning_timestamp_call"
        )
    );
    assert!(fields[132].contains(
        "calculation_heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry"
    ));
}

fn array<'a>(lifecycle: &'a Value, field: &str) -> &'a Vec<Value> {
    lifecycle[field].as_array().expect("CP442 route array")
}

fn sum(values: &[Value]) -> u64 {
    values.iter().map(count_value).sum()
}

fn count(lifecycle: &Value, field: &str) -> u64 {
    count_value(&lifecycle[field])
}

fn count_value(value: &Value) -> u64 {
    value.as_u64().expect("CP442 unsigned count")
}
