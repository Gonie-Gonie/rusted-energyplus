//! CP425 structural zero-flow supply-enthalpy assignment assertions.

use std::collections::BTreeSet;

use serde_json::{Map, Value, json};

#[path = "cp426_assertions.rs"]
mod cp426_assertions;

const CP424_KEY: &str =
    "purchased_air_calc_cooling_supply_mass_flow_positive_guard_else_branch_entry_lifecycle";
const CP425_KEY: &str = "purchased_air_calc_cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_lifecycle";
const ORDER: [&str; 2] = [
    "read-retained-mixed-air-enthalpy-for-zero-supply-mass-flow-supply-enthalpy-assignment",
    "assign-local-supply-enthalpy-from-mixed-air-enthalpy-for-zero-supply-mass-flow",
];
const PUBLIC: [usize; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 20, 21, 26, 27];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let lifecycle = &runtime[CP425_KEY];
    let predecessor = &runtime[CP424_KEY];
    assert_eq!(
        lifecycle["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2340"
    );
    assert_eq!(
        lifecycle["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2341"
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
        "zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_route_counts",
    ] {
        let values = array(lifecycle, field);
        assert_eq!(values.len(), 36, "CP425 {field} width");
        for (index, value) in values.iter().enumerate() {
            if !PUBLIC.contains(&index) {
                assert_eq!(count_value(value), 0, "CP425 {field} route {index}");
            }
        }
    }
    let predecessor_routes = array(lifecycle, "predecessor_route_counts");
    let assignment_routes = array(
        lifecycle,
        "zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_route_counts",
    );
    for index in 0..36 {
        assert_eq!(
            count_value(&assignment_routes[index]),
            if index == 2 {
                count_value(&predecessor_routes[index])
            } else {
                0
            },
            "CP425 sole assignment route {index}",
        );
    }
    let transitions = sum(predecessor_routes);
    let assignments = sum(assignment_routes);
    assert_eq!(count(lifecycle, "transition_count"), transitions);
    assert_eq!(
        count(lifecycle, "inactive_transition_count"),
        transitions - assignments
    );
    assert_eq!(
        count(
            lifecycle,
            "zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_count"
        ),
        assignments
    );
    assert_eq!(
        count(lifecycle, "source_site_execution_count"),
        assignments * 2
    );
    for (field, expected) in [
        ("cp425_supply_enthalpy_state_owner_count", assignments),
        (
            "cp329_retained_mixed_air_enthalpy_owned_read_count",
            assignments,
        ),
        (
            "mixed_air_enthalpy_for_zero_supply_mass_flow_supply_enthalpy_read_count",
            assignments,
        ),
        ("supply_enthalpy_assignment_write_count", assignments),
    ] {
        assert_eq!(count(lifecycle, field), expected, "CP425 {field}");
    }
    for (owner, unchanged, maximum) in [
        (
            "cp424_supply_humidity_ratio_state_owner_count",
            "unchanged_supply_humidity_ratio_preservation_count",
            36,
        ),
        (
            "cp424_supply_enthalpy_state_owner_count",
            "unchanged_supply_enthalpy_preservation_count",
            41,
        ),
        (
            "cp424_supply_temperature_state_owner_count",
            "unchanged_supply_temperature_preservation_count",
            56,
        ),
    ] {
        assert_eq!(count(lifecycle, unchanged), count(lifecycle, owner));
        assert!(count(lifecycle, owner) <= maximum);
    }

    let latest = lifecycle["latest"]
        .as_object()
        .expect("CP425 latest object");
    let predecessor_latest = predecessor["latest"]
        .as_object()
        .expect("CP424 latest object");
    assert_eq!(latest.len(), 374);
    assert_eq!(predecessor_latest.len(), 357);
    let keys = latest.keys().map(String::as_str).collect::<Vec<_>>();
    let predecessor_keys = predecessor_latest
        .keys()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    let tail = literal_keys(include_str!(
        "../../src/pipeline/purchased_air_cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment/serialization/snapshot.rs"
    ));
    assert_eq!(tail.len(), 24);
    let mut expected = predecessor_keys;
    for generic_terminal_key in [
        "resulting_supply_humidity_ratio",
        "resulting_supply_humidity_ratio_ieee_bits",
        "resulting_supply_enthalpy_j_per_kg",
        "resulting_supply_enthalpy_j_per_kg_ieee_bits",
        "resulting_supply_temperature_c",
        "resulting_supply_temperature_c_ieee_bits",
    ] {
        assert!(expected.remove(generic_terminal_key));
    }
    expected.extend(tail);
    assert_eq!(keys.iter().copied().collect::<BTreeSet<_>>(), expected);
    assert_eq!(
        keys.iter()
            .filter(|key| key.ends_with("_ieee_bits"))
            .count(),
        99
    );
    for (predecessor_field, cp424_field) in [
        (
            "predecessor_cp424_resulting_supply_humidity_ratio_ieee_bits",
            "resulting_supply_humidity_ratio_ieee_bits",
        ),
        (
            "predecessor_cp424_resulting_supply_enthalpy_j_per_kg_ieee_bits",
            "resulting_supply_enthalpy_j_per_kg_ieee_bits",
        ),
        (
            "predecessor_cp424_resulting_supply_temperature_c_ieee_bits",
            "resulting_supply_temperature_c_ieee_bits",
        ),
    ] {
        assert_eq!(latest[predecessor_field], predecessor_latest[cp424_field]);
    }
    let assignment =
        latest["cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_executed"]
            .as_bool()
            .expect("CP425 assignment marker");
    assert_eq!(
        assignment,
        latest["cooling_supply_mass_flow_positive_guard_else_branch_entered"]
            .as_bool()
            .expect("CP424 entry marker")
    );
    if assignment {
        assert_eq!(
            latest["assigned_supply_enthalpy_from_mixed_air_j_per_kg_ieee_bits"],
            latest["mixed_air_enthalpy_for_zero_supply_mass_flow_supply_enthalpy_j_per_kg_ieee_bits"]
        );
        assert_eq!(
            latest["resulting_supply_enthalpy_j_per_kg_ieee_bits"],
            latest["assigned_supply_enthalpy_from_mixed_air_j_per_kg_ieee_bits"]
        );
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
    assert!(!results.to_string().contains(CP425_KEY));
    cp426_assertions::assert_direct(runtime, results);
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP425_KEY));
    assert!(
        runtime[CP425_KEY].is_null(),
        "non-direct runtime must not publish CP425 evidence"
    );
    cp426_assertions::assert_non_direct(runtime);
}

fn assert_schema_and_binding_cardinalities() {
    let core = include_str!(
        "../../../ep_runtime/src/ideal_loads/calc/cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment.rs"
    );
    let snapshot = core
        .split_once("pub struct PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyEnthalpyMixedAirAssignmentSnapshot")
        .and_then(|(_, tail)| tail.split_once("/// Final selected-unit CP425"))
        .map(|(snapshot, _)| snapshot)
        .expect("CP425 snapshot declaration");
    let fields = snapshot
        .lines()
        .filter(|line| line.trim_start().starts_with("pub "))
        .collect::<Vec<_>>();
    assert_eq!(fields.len(), 275);
    let names = fields
        .iter()
        .filter_map(|line| line.trim().strip_prefix("pub "))
        .filter_map(|line| line.split_once(':').map(|(name, _)| name))
        .collect::<BTreeSet<_>>();
    assert_eq!(names.len(), 275);
    assert_eq!(snapshot.matches("Option<f64>").count(), 99);
    assert_eq!(snapshot.matches("Option<bool>").count(), 2);
    assert_eq!(snapshot.matches("Option<").count() - 99 - 2, 1);
    let binding = include_str!("../../../ep_runtime/src/ideal_loads/binding/scheduled_output.rs");
    assert_eq!(binding.matches("    pub calculation_").count(), 123);
}

fn literal_keys(source: &'static str) -> Vec<&'static str> {
    source
        .lines()
        .filter_map(|line| {
            line.trim_start()
                .strip_prefix('"')
                .and_then(|line| line.split_once("\":").map(|(key, _)| key))
        })
        .collect()
}

fn count(value: &Value, field: &str) -> u64 {
    count_value(&value[field])
}
fn count_value(value: &Value) -> u64 {
    value.as_u64().expect("CP425 unsigned count")
}
fn array<'a>(value: &'a Value, field: &str) -> &'a [Value] {
    value[field]
        .as_array()
        .map(Vec::as_slice)
        .unwrap_or_default()
}
fn sum(values: &[Value]) -> u64 {
    values.iter().map(count_value).sum()
}
