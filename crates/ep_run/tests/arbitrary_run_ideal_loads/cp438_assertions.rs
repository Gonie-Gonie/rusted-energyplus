//! CP438 heating maximum-flow first-warning counter-increment assertions.

#[path = "cp439_assertions.rs"]
mod cp439_assertions;

use std::collections::BTreeSet;

use serde_json::{Map, Value, json};

const CP437_KEY: &str =
    "purchased_air_calc_heating_outdoor_air_maximum_flow_first_warning_guard_lifecycle";
const CP438_KEY: &str =
    "purchased_air_calc_heating_outdoor_air_maximum_flow_first_warning_counter_increment_lifecycle";
const ORDER: [&str; 1] =
    ["increment-state-owned-outdoor-air-flow-maximum-heating-output-error-count"];
const PUBLIC: [usize; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 20, 21, 26, 27];
const REPLACED_PREDECESSOR_TAIL: [&str; 6] = [
    "resulting_supply_humidity_ratio",
    "resulting_supply_humidity_ratio_ieee_bits",
    "resulting_supply_enthalpy_j_per_kg",
    "resulting_supply_enthalpy_j_per_kg_ieee_bits",
    "resulting_supply_temperature_c",
    "resulting_supply_temperature_c_ieee_bits",
];
const TAIL: [&str; 19] = [
    "predecessor_cp437_resulting_supply_humidity_ratio",
    "predecessor_cp437_resulting_supply_humidity_ratio_ieee_bits",
    "predecessor_cp437_resulting_supply_enthalpy_j_per_kg",
    "predecessor_cp437_resulting_supply_enthalpy_j_per_kg_ieee_bits",
    "predecessor_cp437_resulting_supply_temperature_c",
    "predecessor_cp437_resulting_supply_temperature_c_ieee_bits",
    "heating_outdoor_air_maximum_flow_first_warning_counter_increment_executed",
    "cp437_retained_supply_humidity_ratio_state_owned",
    "cp437_retained_supply_enthalpy_state_owned",
    "cp437_retained_supply_temperature_state_owned",
    "cp437_retained_outdoor_air_flow_maximum_heating_output_error_count_state_owned",
    "outdoor_air_flow_maximum_heating_output_error_count_increment_performed",
    "assigned_outdoor_air_flow_maximum_heating_output_error_count",
    "resulting_supply_humidity_ratio",
    "resulting_supply_humidity_ratio_ieee_bits",
    "resulting_supply_enthalpy_j_per_kg",
    "resulting_supply_enthalpy_j_per_kg_ieee_bits",
    "resulting_supply_temperature_c",
    "resulting_supply_temperature_c_ieee_bits",
];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let lifecycle = &runtime[CP438_KEY];
    let predecessor = &runtime[CP437_KEY];
    assert_eq!(
        lifecycle["source"],
        json!("EnergyPlus 26.1 PurchasedAirManager.cc:2365")
    );
    assert_eq!(
        lifecycle["first_excluded_source"],
        json!("EnergyPlus 26.1 PurchasedAirManager.cc:2366")
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
            "predecessor_volume_flow_assignment_route_counts",
        ),
        (
            "predecessor_first_warning_guard_false_fallthrough_route_counts",
            "heating_outdoor_air_maximum_flow_first_warning_guard_false_fallthrough_route_counts",
        ),
        (
            "predecessor_first_warning_branch_entry_route_counts",
            "heating_outdoor_air_maximum_flow_first_warning_branch_entry_route_counts",
        ),
    ] {
        assert_eq!(lifecycle[field], predecessor[predecessor_field], "{field}");
    }
    for field in [
        "predecessor_route_counts",
        "predecessor_guard_false_fallthrough_route_counts",
        "predecessor_guard_body_entry_route_counts",
        "predecessor_volume_flow_assignment_route_counts",
        "predecessor_first_warning_guard_false_fallthrough_route_counts",
        "predecessor_first_warning_branch_entry_route_counts",
        "heating_outdoor_air_maximum_flow_first_warning_counter_increment_route_counts",
    ] {
        let values = array(lifecycle, field);
        assert_eq!(values.len(), 36, "CP438 {field} width");
        for (index, value) in values.iter().enumerate() {
            if !PUBLIC.contains(&index) {
                assert_eq!(count_value(value), 0, "CP438 {field} route {index}");
            }
        }
    }
    let increments = sum(array(
        lifecycle,
        "heating_outdoor_air_maximum_flow_first_warning_counter_increment_route_counts",
    ));
    assert_eq!(increments, 0);
    assert_eq!(
        lifecycle["heating_outdoor_air_maximum_flow_first_warning_counter_increment_route_counts"],
        predecessor["heating_outdoor_air_maximum_flow_first_warning_branch_entry_route_counts"]
    );
    assert_eq!(
        count(lifecycle, "inactive_transition_count"),
        count(lifecycle, "transition_count")
    );
    for field in [
        "outdoor_air_flow_maximum_heating_output_error_count_increment_count",
        "source_site_execution_count",
        "cp437_outdoor_air_flow_maximum_heating_output_error_count_state_owner_count",
        "outdoor_air_flow_maximum_heating_output_error_count_increment_write_count",
    ] {
        assert_eq!(count(lifecycle, field), 0, "CP438 {field}");
    }
    for (owner, predecessor_preservation, preservation) in [
        (
            "cp437_supply_humidity_ratio_state_owner_count",
            "unchanged_supply_humidity_ratio_preservation_count",
            "unchanged_supply_humidity_ratio_preservation_count",
        ),
        (
            "cp437_supply_enthalpy_state_owner_count",
            "unchanged_supply_enthalpy_preservation_count",
            "unchanged_supply_enthalpy_preservation_count",
        ),
        (
            "cp437_supply_temperature_state_owner_count",
            "unchanged_supply_temperature_preservation_count",
            "unchanged_supply_temperature_preservation_count",
        ),
    ] {
        assert_eq!(lifecycle[owner], predecessor[predecessor_preservation]);
        assert_eq!(lifecycle[preservation], lifecycle[owner]);
    }
    let latest = lifecycle["latest"]
        .as_object()
        .expect("CP438 latest object");
    let predecessor_latest = predecessor["latest"]
        .as_object()
        .expect("CP437 latest object");
    assert_actual_json_key_set(latest, predecessor_latest);
    for (cp438_field, cp437_field) in [
        (
            "predecessor_cp437_resulting_supply_humidity_ratio",
            "resulting_supply_humidity_ratio",
        ),
        (
            "predecessor_cp437_resulting_supply_humidity_ratio_ieee_bits",
            "resulting_supply_humidity_ratio_ieee_bits",
        ),
        (
            "predecessor_cp437_resulting_supply_enthalpy_j_per_kg",
            "resulting_supply_enthalpy_j_per_kg",
        ),
        (
            "predecessor_cp437_resulting_supply_enthalpy_j_per_kg_ieee_bits",
            "resulting_supply_enthalpy_j_per_kg_ieee_bits",
        ),
        (
            "predecessor_cp437_resulting_supply_temperature_c",
            "resulting_supply_temperature_c",
        ),
        (
            "predecessor_cp437_resulting_supply_temperature_c_ieee_bits",
            "resulting_supply_temperature_c_ieee_bits",
        ),
    ] {
        assert_eq!(latest[cp438_field], predecessor_latest[cp437_field]);
        assert_eq!(latest[cp437_field], predecessor_latest[cp437_field]);
    }
    assert_public_skip_shape(latest);
    assert_schema_and_binding_cardinalities();
    assert!(!results.to_string().contains(CP438_KEY));
    cp439_assertions::assert_direct(runtime, results);
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP438_KEY));
    assert!(
        runtime[CP438_KEY].is_null(),
        "non-direct runtime must not publish CP438 evidence"
    );
    cp439_assertions::assert_non_direct(runtime);
}

fn assert_public_skip_shape(latest: &Map<String, Value>) {
    for (owner, predecessor_bits) in [
        (
            "cp437_retained_supply_humidity_ratio_state_owned",
            "predecessor_cp437_resulting_supply_humidity_ratio_ieee_bits",
        ),
        (
            "cp437_retained_supply_enthalpy_state_owned",
            "predecessor_cp437_resulting_supply_enthalpy_j_per_kg_ieee_bits",
        ),
        (
            "cp437_retained_supply_temperature_state_owned",
            "predecessor_cp437_resulting_supply_temperature_c_ieee_bits",
        ),
    ] {
        assert_eq!(latest[owner], json!(!latest[predecessor_bits].is_null()));
    }
    for field in [
        "heating_outdoor_air_maximum_flow_first_warning_counter_increment_executed",
        "cp437_retained_outdoor_air_flow_maximum_heating_output_error_count_state_owned",
        "outdoor_air_flow_maximum_heating_output_error_count_increment_performed",
    ] {
        assert_eq!(latest[field], json!(false), "CP438 skipped {field}");
    }
    assert!(latest["assigned_outdoor_air_flow_maximum_heating_output_error_count"].is_null());
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
    assert_eq!(actual.len(), 572);
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
        "../../../ep_runtime/src/ideal_loads/calc/heating_outdoor_air_maximum_flow_first_warning_counter_increment.rs"
    );
    let snapshot = core
        .split_once(
            "pub struct PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCounterIncrementSnapshot",
        )
        .and_then(|(_, tail)| tail.split_once("/// Final selected-unit CP438"))
        .map(|(snapshot, _)| snapshot)
        .expect("CP438 snapshot declaration");
    let fields = snapshot
        .lines()
        .filter(|line| line.trim_start().starts_with("pub "))
        .collect::<Vec<_>>();
    assert_eq!(fields.len(), 426);
    assert_eq!(
        fields
            .iter()
            .filter_map(|line| line.trim().strip_prefix("pub "))
            .filter_map(|line| line.split_once(':').map(|(name, _)| name))
            .collect::<BTreeSet<_>>()
            .len(),
        426
    );
    assert_eq!(snapshot.matches("Option<f64>").count(), 146);
    assert_eq!(snapshot.matches("Option<bool>").count(), 9);
    assert_eq!(snapshot.matches("Option<usize>").count(), 2);
    assert_eq!(snapshot.matches("Option<").count() - 146 - 9 - 2, 6);
    let binding = include_str!("../../../ep_runtime/src/ideal_loads/binding/scheduled_output.rs");
    let fields = binding
        .lines()
        .filter(|line| line.starts_with("    pub calculation_"))
        .collect::<Vec<_>>();
    assert_eq!(fields.len(), 130);
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
    lifecycle[field].as_array().expect("CP438 route array")
}

fn sum(values: &[Value]) -> u64 {
    values.iter().map(count_value).sum()
}

fn count(lifecycle: &Value, field: &str) -> u64 {
    count_value(&lifecycle[field])
}

fn count_value(value: &Value) -> u64 {
    value.as_u64().expect("CP438 unsigned count")
}
