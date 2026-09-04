//! CP437 heating maximum-flow first-warning-guard assertions.

#[path = "cp438_assertions.rs"]
mod cp438_assertions;

use std::collections::BTreeSet;

use serde_json::{Map, Value, json};

const CP436_KEY: &str =
    "purchased_air_calc_heating_outdoor_air_maximum_flow_body_volume_flow_assignment_lifecycle";
const CP437_KEY: &str =
    "purchased_air_calc_heating_outdoor_air_maximum_flow_first_warning_guard_lifecycle";
const ORDER: [&str; 3] = [
    "read-state-owned-outdoor-air-flow-maximum-heating-output-error-count",
    "compare-outdoor-air-flow-maximum-heating-output-error-count-less-than-one",
    "enter-heating-outdoor-air-maximum-flow-first-warning-branch-if-satisfied",
];
const PUBLIC: [usize; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 20, 21, 26, 27];
const REPLACED_PREDECESSOR_TAIL: [&str; 6] = [
    "resulting_supply_humidity_ratio",
    "resulting_supply_humidity_ratio_ieee_bits",
    "resulting_supply_enthalpy_j_per_kg",
    "resulting_supply_enthalpy_j_per_kg_ieee_bits",
    "resulting_supply_temperature_c",
    "resulting_supply_temperature_c_ieee_bits",
];
const TAIL: [&str; 23] = [
    "predecessor_cp436_resulting_supply_humidity_ratio",
    "predecessor_cp436_resulting_supply_humidity_ratio_ieee_bits",
    "predecessor_cp436_resulting_supply_enthalpy_j_per_kg",
    "predecessor_cp436_resulting_supply_enthalpy_j_per_kg_ieee_bits",
    "predecessor_cp436_resulting_supply_temperature_c",
    "predecessor_cp436_resulting_supply_temperature_c_ieee_bits",
    "heating_outdoor_air_maximum_flow_first_warning_guard_evaluated",
    "cp436_retained_supply_humidity_ratio_state_owned",
    "cp436_retained_supply_enthalpy_state_owned",
    "cp436_retained_supply_temperature_state_owned",
    "outdoor_air_flow_maximum_heating_output_error_count_state_owned",
    "outdoor_air_flow_maximum_heating_output_error_count_read",
    "outdoor_air_flow_maximum_heating_output_error_count_before",
    "outdoor_air_flow_maximum_heating_output_error_count_less_than_one_comparison_evaluated",
    "outdoor_air_flow_maximum_heating_output_error_count_less_than_one",
    "heating_outdoor_air_maximum_flow_first_warning_branch_entered",
    "heating_outdoor_air_maximum_flow_first_warning_guard_false_fallthrough",
    "resulting_supply_humidity_ratio",
    "resulting_supply_humidity_ratio_ieee_bits",
    "resulting_supply_enthalpy_j_per_kg",
    "resulting_supply_enthalpy_j_per_kg_ieee_bits",
    "resulting_supply_temperature_c",
    "resulting_supply_temperature_c_ieee_bits",
];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let lifecycle = &runtime[CP437_KEY];
    let predecessor = &runtime[CP436_KEY];
    assert_eq!(
        lifecycle["source"],
        json!("EnergyPlus 26.1 PurchasedAirManager.cc:2364")
    );
    assert_eq!(
        lifecycle["first_excluded_source"],
        json!("EnergyPlus 26.1 PurchasedAirManager.cc:2365")
    );
    assert_eq!(lifecycle["latest"]["source_order"], json!(ORDER));
    assert_eq!(lifecycle["system"], predecessor["system"]);
    assert_eq!(
        lifecycle["transition_count"],
        predecessor["transition_count"]
    );
    for (field, predecessor_field) in [
        ("predecessor_route_counts", "predecessor_route_counts"),
        (
            "predecessor_guard_false_fallthrough_route_counts",
            "predecessor_guard_false_fallthrough_route_counts",
        ),
        (
            "predecessor_guard_body_entry_route_counts",
            "predecessor_guard_body_entry_route_counts",
        ),
        (
            "predecessor_volume_flow_assignment_route_counts",
            "heating_outdoor_air_volume_flow_assignment_route_counts",
        ),
    ] {
        assert_eq!(lifecycle[field], predecessor[predecessor_field]);
    }
    let route_fields = [
        "predecessor_route_counts",
        "predecessor_guard_false_fallthrough_route_counts",
        "predecessor_guard_body_entry_route_counts",
        "predecessor_volume_flow_assignment_route_counts",
        "heating_outdoor_air_maximum_flow_first_warning_guard_false_fallthrough_route_counts",
        "heating_outdoor_air_maximum_flow_first_warning_branch_entry_route_counts",
    ];
    assert_eq!(
        lifecycle
            .as_object()
            .expect("CP437 lifecycle object")
            .keys()
            .filter(|key| key.ends_with("_route_counts"))
            .count(),
        6
    );
    for field in route_fields {
        let values = array(lifecycle, field);
        assert_eq!(values.len(), 36, "CP437 {field} width");
        for (index, value) in values.iter().enumerate() {
            if !PUBLIC.contains(&index) {
                assert_eq!(count_value(value), 0, "CP437 {field} route {index}");
            }
        }
    }
    let transitions = sum(array(lifecycle, "predecessor_route_counts"));
    let assignments = sum(array(
        lifecycle,
        "predecessor_volume_flow_assignment_route_counts",
    ));
    let false_fallthroughs = sum(array(
        lifecycle,
        "heating_outdoor_air_maximum_flow_first_warning_guard_false_fallthrough_route_counts",
    ));
    let branch_entries = sum(array(
        lifecycle,
        "heating_outdoor_air_maximum_flow_first_warning_branch_entry_route_counts",
    ));
    assert_eq!(assignments, 0, "exact public release admits no CP437 guard");
    assert_eq!(false_fallthroughs, 0);
    assert_eq!(branch_entries, 0);
    assert_eq!(count(lifecycle, "transition_count"), transitions);
    assert_eq!(count(lifecycle, "inactive_transition_count"), transitions);
    for field in [
        "guard_evaluation_count",
        "first_warning_branch_entry_count",
        "guard_false_fallthrough_count",
        "source_site_execution_count",
        "outdoor_air_flow_maximum_heating_output_error_count",
        "outdoor_air_flow_maximum_heating_output_error_count_state_owner_count",
        "outdoor_air_flow_maximum_heating_output_error_count_read_count",
        "outdoor_air_flow_maximum_heating_output_error_count_less_than_one_comparison_count",
    ] {
        assert_eq!(count(lifecycle, field), 0, "CP437 {field}");
    }
    for (owner, preserved, predecessor_owner) in [
        (
            "cp436_supply_humidity_ratio_state_owner_count",
            "unchanged_supply_humidity_ratio_preservation_count",
            "cp435_supply_humidity_ratio_state_owner_count",
        ),
        (
            "cp436_supply_enthalpy_state_owner_count",
            "unchanged_supply_enthalpy_preservation_count",
            "cp435_supply_enthalpy_state_owner_count",
        ),
        (
            "cp436_supply_temperature_state_owner_count",
            "unchanged_supply_temperature_preservation_count",
            "cp435_supply_temperature_state_owner_count",
        ),
    ] {
        let expected = count(predecessor, predecessor_owner);
        assert_eq!(count(lifecycle, owner), expected, "CP437 {owner}");
        assert_eq!(count(lifecycle, preserved), expected, "CP437 {preserved}");
    }

    let latest = lifecycle["latest"]
        .as_object()
        .expect("CP437 latest object");
    let predecessor_latest = predecessor["latest"]
        .as_object()
        .expect("CP436 latest object");
    assert_actual_json_key_set(latest, predecessor_latest);
    assert_eq!(latest.len(), 559);
    assert_eq!(
        latest
            .keys()
            .filter(|key| key.ends_with("_ieee_bits"))
            .count(),
        143
    );
    for (key, expected) in predecessor_latest {
        if !["source", "first_excluded_source", "source_order"].contains(&key.as_str())
            && !REPLACED_PREDECESSOR_TAIL.contains(&key.as_str())
        {
            assert_eq!(latest.get(key).expect("CP437 predecessor key"), expected);
        }
    }
    for (cp437_field, cp436_field) in [
        (
            "predecessor_cp436_resulting_supply_humidity_ratio_ieee_bits",
            "resulting_supply_humidity_ratio_ieee_bits",
        ),
        (
            "predecessor_cp436_resulting_supply_enthalpy_j_per_kg_ieee_bits",
            "resulting_supply_enthalpy_j_per_kg_ieee_bits",
        ),
        (
            "predecessor_cp436_resulting_supply_temperature_c_ieee_bits",
            "resulting_supply_temperature_c_ieee_bits",
        ),
    ] {
        assert_eq!(latest[cp437_field], predecessor_latest[cp436_field]);
        assert_eq!(latest[cp436_field], predecessor_latest[cp436_field]);
    }
    assert_public_skip_shape(latest);
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
    assert!(!results.to_string().contains(CP437_KEY));
    cp438_assertions::assert_direct(runtime, results);
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP437_KEY));
    assert!(
        runtime[CP437_KEY].is_null(),
        "non-direct runtime must not publish CP437 evidence"
    );
    cp438_assertions::assert_non_direct(runtime);
}

fn assert_public_skip_shape(latest: &Map<String, Value>) {
    for field in [
        "heating_outdoor_air_maximum_flow_first_warning_guard_evaluated",
        "outdoor_air_flow_maximum_heating_output_error_count_state_owned",
        "outdoor_air_flow_maximum_heating_output_error_count_read",
        "outdoor_air_flow_maximum_heating_output_error_count_less_than_one_comparison_evaluated",
        "heating_outdoor_air_maximum_flow_first_warning_branch_entered",
        "heating_outdoor_air_maximum_flow_first_warning_guard_false_fallthrough",
    ] {
        assert_eq!(latest[field], json!(false), "CP437 skipped {field}");
    }
    for field in [
        "outdoor_air_flow_maximum_heating_output_error_count_before",
        "outdoor_air_flow_maximum_heating_output_error_count_less_than_one",
    ] {
        assert!(latest[field].is_null(), "CP437 skipped {field}");
    }
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
    assert_eq!(tail.len(), 23);
    expected.extend(tail);
    let actual = latest.keys().map(String::as_str).collect::<BTreeSet<_>>();
    assert_eq!(actual, expected);
    assert_eq!(actual.len(), 559);
}

fn assert_schema_and_binding_cardinalities() {
    let core = include_str!(
        "../../../ep_runtime/src/ideal_loads/calc/heating_outdoor_air_maximum_flow_first_warning_guard.rs"
    );
    let snapshot = core
        .split_once(
            "pub struct PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardSnapshot",
        )
        .and_then(|(_, tail)| tail.split_once("/// Final selected-unit CP437"))
        .map(|(snapshot, _)| snapshot)
        .expect("CP437 snapshot declaration");
    let fields = snapshot
        .lines()
        .filter(|line| line.trim_start().starts_with("pub "))
        .collect::<Vec<_>>();
    assert_eq!(fields.len(), 416);
    assert_eq!(
        fields
            .iter()
            .filter_map(|line| line.trim().strip_prefix("pub "))
            .filter_map(|line| line.split_once(':').map(|(name, _)| name))
            .collect::<BTreeSet<_>>()
            .len(),
        416
    );
    assert_eq!(snapshot.matches("Option<f64>").count(), 143);
    assert_eq!(snapshot.matches("Option<bool>").count(), 9);
    assert_eq!(snapshot.matches("Option<usize>").count(), 1);
    assert_eq!(snapshot.matches("Option<").count() - 143 - 9 - 1, 6);
    let binding = include_str!("../../../ep_runtime/src/ideal_loads/binding/scheduled_output.rs");
    let fields = binding
        .lines()
        .filter(|line| line.starts_with("    pub calculation_"))
        .collect::<Vec<_>>();
    assert_eq!(fields.len(), 132);
    assert!(
        fields[126]
            .contains("calculation_heating_outdoor_air_maximum_flow_body_volume_flow_assignment")
    );
    assert!(
        fields[127].contains("calculation_heating_outdoor_air_maximum_flow_first_warning_guard")
    );
    assert!(
        fields[128].contains(
            "calculation_heating_outdoor_air_maximum_flow_first_warning_counter_increment"
        )
    );
}

fn array<'a>(lifecycle: &'a Value, field: &str) -> &'a Vec<Value> {
    lifecycle[field].as_array().expect("CP437 route array")
}

fn sum(values: &[Value]) -> u64 {
    values.iter().map(count_value).sum()
}

fn count(lifecycle: &Value, field: &str) -> u64 {
    count_value(&lifecycle[field])
}

fn count_value(value: &Value) -> u64 {
    value.as_u64().expect("CP437 unsigned count")
}
