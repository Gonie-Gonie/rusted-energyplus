//! CP428 structural zero-flow sensible-output positive-zero assignment assertions.

use std::collections::BTreeSet;

use serde_json::{Map, Value, json};

const CP427_KEY: &str = "purchased_air_calc_cooling_zero_supply_mass_flow_supply_temperature_mixed_air_assignment_lifecycle";
const CP428_KEY: &str = "purchased_air_calc_cooling_zero_supply_mass_flow_sensible_output_positive_zero_assignment_lifecycle";
const ORDER: [&str; 1] =
    ["assign-local-cooling-sensible-output-positive-zero-for-zero-supply-mass-flow"];
const PUBLIC: [usize; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 20, 21, 26, 27];
const TAIL: [&str; 19] = [
    "predecessor_cp427_resulting_supply_humidity_ratio",
    "predecessor_cp427_resulting_supply_humidity_ratio_ieee_bits",
    "predecessor_cp427_resulting_supply_enthalpy_j_per_kg",
    "predecessor_cp427_resulting_supply_enthalpy_j_per_kg_ieee_bits",
    "predecessor_cp427_resulting_supply_temperature_c",
    "predecessor_cp427_resulting_supply_temperature_c_ieee_bits",
    "cooling_zero_supply_mass_flow_sensible_output_positive_zero_assignment_executed",
    "cp427_retained_supply_humidity_ratio_state_owned",
    "cp427_retained_supply_enthalpy_state_owned",
    "cp427_retained_supply_temperature_state_owned",
    "cooling_sensible_output_positive_zero_assignment_performed",
    "assigned_cooling_sensible_output_w",
    "assigned_cooling_sensible_output_w_ieee_bits",
    "resulting_supply_humidity_ratio",
    "resulting_supply_humidity_ratio_ieee_bits",
    "resulting_supply_enthalpy_j_per_kg",
    "resulting_supply_enthalpy_j_per_kg_ieee_bits",
    "resulting_supply_temperature_c",
    "resulting_supply_temperature_c_ieee_bits",
];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let lifecycle = &runtime[CP428_KEY];
    let predecessor = &runtime[CP427_KEY];
    assert_eq!(
        lifecycle["source"],
        json!("EnergyPlus 26.1 PurchasedAirManager.cc:2343")
    );
    assert_eq!(
        lifecycle["first_excluded_source"],
        json!("EnergyPlus 26.1 PurchasedAirManager.cc:2344")
    );
    let routes = array(lifecycle, "predecessor_route_counts");
    let assignments = array(
        lifecycle,
        "zero_supply_mass_flow_sensible_output_positive_zero_assignment_route_counts",
    );
    assert_eq!(routes.len(), 36);
    assert_eq!(assignments.len(), 36);
    for index in 0..36 {
        if !PUBLIC.contains(&index) {
            assert_eq!(count_value(&routes[index]), 0, "CP428 route {index}");
            assert_eq!(
                count_value(&assignments[index]),
                0,
                "CP428 assignment {index}"
            );
        }
        assert_eq!(
            count_value(&assignments[index]),
            if index == 2 {
                count_value(&routes[index])
            } else {
                0
            },
            "CP428 sole assignment route {index}"
        );
    }
    let transitions = routes.iter().map(count_value).sum::<u64>();
    let assignment_count = assignments.iter().map(count_value).sum::<u64>();
    assert_eq!(count(lifecycle, "transition_count"), transitions);
    assert_eq!(
        count(lifecycle, "inactive_transition_count"),
        transitions - assignment_count
    );
    for (field, expected) in [
        (
            "zero_supply_mass_flow_sensible_output_positive_zero_assignment_count",
            assignment_count,
        ),
        ("source_site_execution_count", assignment_count),
        (
            "cp428_cooling_sensible_output_state_owner_count",
            assignment_count,
        ),
        (
            "cooling_sensible_output_assignment_write_count",
            assignment_count,
        ),
    ] {
        assert_eq!(count(lifecycle, field), expected, "CP428 {field}");
    }
    let predecessor_temperature_owners =
        count(predecessor, "cp426_supply_temperature_state_owner_count")
            + count(predecessor, "cp427_supply_temperature_state_owner_count");
    for (owner, preserved, expected) in [
        (
            "cp427_supply_humidity_ratio_state_owner_count",
            "unchanged_supply_humidity_ratio_preservation_count",
            count(predecessor, "cp426_supply_humidity_ratio_state_owner_count"),
        ),
        (
            "cp427_supply_enthalpy_state_owner_count",
            "unchanged_supply_enthalpy_preservation_count",
            count(predecessor, "cp426_supply_enthalpy_state_owner_count"),
        ),
        (
            "cp427_supply_temperature_state_owner_count",
            "unchanged_supply_temperature_preservation_count",
            predecessor_temperature_owners,
        ),
    ] {
        assert_eq!(count(lifecycle, owner), expected, "CP428 {owner}");
        assert_eq!(count(lifecycle, preserved), expected, "CP428 {preserved}");
    }

    let latest = lifecycle["latest"]
        .as_object()
        .expect("CP428 latest object");
    let predecessor_latest = predecessor["latest"]
        .as_object()
        .expect("CP427 latest object");
    assert_eq!(latest["source_order"], json!(ORDER));
    for (cp428_field, cp427_field) in [
        (
            "predecessor_cp427_resulting_supply_humidity_ratio_ieee_bits",
            "resulting_supply_humidity_ratio_ieee_bits",
        ),
        (
            "predecessor_cp427_resulting_supply_enthalpy_j_per_kg_ieee_bits",
            "resulting_supply_enthalpy_j_per_kg_ieee_bits",
        ),
        (
            "predecessor_cp427_resulting_supply_temperature_c_ieee_bits",
            "resulting_supply_temperature_c_ieee_bits",
        ),
    ] {
        assert_eq!(latest[cp428_field], predecessor_latest[cp427_field]);
    }
    for field in [
        "resulting_supply_humidity_ratio_ieee_bits",
        "resulting_supply_enthalpy_j_per_kg_ieee_bits",
        "resulting_supply_temperature_c_ieee_bits",
    ] {
        assert_eq!(latest[field], predecessor_latest[field], "CP428 {field}");
    }
    let assignment =
        latest["cooling_zero_supply_mass_flow_sensible_output_positive_zero_assignment_executed"]
            .as_bool()
            .expect("CP428 assignment marker");
    assert_eq!(
        assignment,
        predecessor_latest
            ["cooling_zero_supply_mass_flow_supply_temperature_mixed_air_assignment_executed"]
            .as_bool()
            .expect("CP427 assignment marker")
    );
    assert_eq!(
        latest["cooling_sensible_output_positive_zero_assignment_performed"],
        json!(assignment)
    );
    if assignment {
        assert_eq!(latest["assigned_cooling_sensible_output_w"], json!(0.0));
        assert_eq!(
            latest["assigned_cooling_sensible_output_w_ieee_bits"],
            json!("0x0000000000000000")
        );
    } else {
        assert!(latest["assigned_cooling_sensible_output_w"].is_null());
        assert!(latest["assigned_cooling_sensible_output_w_ieee_bits"].is_null());
    }
    assert_actual_json_key_set(latest, predecessor_latest);
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
    assert!(!results.to_string().contains(CP428_KEY));
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP428_KEY));
    assert!(
        runtime[CP428_KEY].is_null(),
        "non-direct runtime must not publish CP428 evidence"
    );
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
    let declared_tail = TAIL.into_iter().collect::<BTreeSet<_>>();
    assert_eq!(declared_tail.len(), 19);
    expected.extend(declared_tail);
    let actual = latest.keys().map(String::as_str).collect::<BTreeSet<_>>();
    assert_eq!(actual, expected);
    assert_eq!(actual.len(), 421);
}

fn assert_schema_and_binding_cardinalities() {
    let core = include_str!(
        "../../../ep_runtime/src/ideal_loads/calc/cooling_zero_supply_mass_flow_sensible_output_positive_zero_assignment.rs"
    );
    let snapshot = core
        .split_once("pub struct PurchasedAirCalcCoolingZeroSupplyMassFlowSensibleOutputPositiveZeroAssignmentSnapshot")
        .and_then(|(_, tail)| tail.split_once("/// Final selected-unit CP428"))
        .map(|(snapshot, _)| snapshot)
        .expect("CP428 snapshot declaration");
    let fields = snapshot
        .lines()
        .filter(|line| line.trim_start().starts_with("pub "))
        .collect::<Vec<_>>();
    assert_eq!(fields.len(), 308);
    let names = fields
        .iter()
        .filter_map(|line| line.trim().strip_prefix("pub "))
        .filter_map(|line| line.split_once(':').map(|(name, _)| name))
        .collect::<BTreeSet<_>>();
    assert_eq!(names.len(), 308);
    assert_eq!(snapshot.matches("Option<f64>").count(), 113);
    assert_eq!(snapshot.matches("Option<bool>").count(), 2);
    assert_eq!(snapshot.matches("Option<").count() - 113 - 2, 1);
    let binding = include_str!("../../../ep_runtime/src/ideal_loads/binding/scheduled_output.rs");
    assert_eq!(binding.matches("    pub calculation_").count(), 119);
}

fn array<'a>(lifecycle: &'a Value, field: &str) -> &'a Vec<Value> {
    lifecycle[field].as_array().expect("CP428 route array")
}

fn count(lifecycle: &Value, field: &str) -> u64 {
    count_value(&lifecycle[field])
}

fn count_value(value: &Value) -> u64 {
    value.as_u64().expect("CP428 unsigned count")
}
