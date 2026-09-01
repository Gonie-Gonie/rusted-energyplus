//! CP426 structural zero-flow supply-humidity-ratio assignment assertions.

use std::collections::BTreeSet;

use serde_json::{Map, Value, json};

#[path = "cp427_assertions.rs"]
mod cp427_assertions;

const CP425_KEY: &str = "purchased_air_calc_cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_lifecycle";
const CP426_KEY: &str = "purchased_air_calc_cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment_lifecycle";
const ORDER: [&str; 2] = [
    "read-retained-mixed-air-humidity-ratio-for-zero-supply-mass-flow-supply-humidity-ratio-assignment",
    "assign-purchased-air-supply-humidity-ratio-from-mixed-air-humidity-ratio-for-zero-supply-mass-flow",
];
const PUBLIC: [usize; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 20, 21, 26, 27];
const TAIL: [&str; 23] = [
    "predecessor_cp425_resulting_supply_humidity_ratio",
    "predecessor_cp425_resulting_supply_humidity_ratio_ieee_bits",
    "predecessor_cp425_resulting_supply_enthalpy_j_per_kg",
    "predecessor_cp425_resulting_supply_enthalpy_j_per_kg_ieee_bits",
    "predecessor_cp425_resulting_supply_temperature_c",
    "predecessor_cp425_resulting_supply_temperature_c_ieee_bits",
    "cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment_executed",
    "cp425_retained_supply_humidity_ratio_state_owned",
    "cp425_retained_supply_enthalpy_state_owned",
    "cp425_retained_supply_temperature_state_owned",
    "cp329_retained_mixed_air_humidity_ratio_for_zero_supply_mass_flow_supply_humidity_ratio_assignment_owned_read",
    "mixed_air_humidity_ratio_for_zero_supply_mass_flow_supply_humidity_ratio_assignment_read",
    "mixed_air_humidity_ratio_for_zero_supply_mass_flow_supply_humidity_ratio_assignment",
    "mixed_air_humidity_ratio_for_zero_supply_mass_flow_supply_humidity_ratio_assignment_ieee_bits",
    "zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment_performed",
    "assigned_supply_humidity_ratio_from_mixed_air",
    "assigned_supply_humidity_ratio_from_mixed_air_ieee_bits",
    "resulting_supply_humidity_ratio",
    "resulting_supply_humidity_ratio_ieee_bits",
    "resulting_supply_enthalpy_j_per_kg",
    "resulting_supply_enthalpy_j_per_kg_ieee_bits",
    "resulting_supply_temperature_c",
    "resulting_supply_temperature_c_ieee_bits",
];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let lifecycle = &runtime[CP426_KEY];
    let predecessor = &runtime[CP425_KEY];
    assert_eq!(
        lifecycle["source"],
        json!("EnergyPlus 26.1 PurchasedAirManager.cc:2341")
    );
    assert_eq!(
        lifecycle["first_excluded_source"],
        json!("EnergyPlus 26.1 PurchasedAirManager.cc:2342")
    );
    assert_eq!(
        lifecycle["transition_count"],
        predecessor["transition_count"]
    );

    let routes = array(lifecycle, "predecessor_route_counts");
    let assignments = array(
        lifecycle,
        "zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment_route_counts",
    );
    assert_eq!(routes.len(), 36);
    assert_eq!(assignments.len(), 36);
    for index in 0..36 {
        if !PUBLIC.contains(&index) {
            assert_eq!(count_value(&routes[index]), 0, "CP426 route {index}");
            assert_eq!(
                count_value(&assignments[index]),
                0,
                "CP426 assignment {index}"
            );
        }
        assert_eq!(
            count_value(&assignments[index]),
            if index == 2 {
                count_value(&routes[index])
            } else {
                0
            },
            "CP426 sole assignment route {index}"
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
            "zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment_count",
            assignment_count,
        ),
        ("source_site_execution_count", 2 * assignment_count),
        (
            "cp426_supply_humidity_ratio_state_owner_count",
            assignment_count,
        ),
        (
            "cp329_retained_mixed_air_humidity_ratio_for_zero_supply_mass_flow_supply_humidity_ratio_assignment_owned_read_count",
            assignment_count,
        ),
        (
            "mixed_air_humidity_ratio_for_zero_supply_mass_flow_supply_humidity_ratio_assignment_read_count",
            assignment_count,
        ),
        (
            "supply_humidity_ratio_assignment_write_count",
            assignment_count,
        ),
    ] {
        assert_eq!(count(lifecycle, field), expected, "CP426 {field}");
    }
    let predecessor_h_owners = count(predecessor, "cp424_supply_enthalpy_state_owner_count")
        + count(predecessor, "cp425_supply_enthalpy_state_owner_count");
    for (owner, preserved, expected) in [
        (
            "cp425_supply_humidity_ratio_state_owner_count",
            "unchanged_supply_humidity_ratio_preservation_count",
            count(predecessor, "cp424_supply_humidity_ratio_state_owner_count"),
        ),
        (
            "cp425_supply_enthalpy_state_owner_count",
            "unchanged_supply_enthalpy_preservation_count",
            predecessor_h_owners,
        ),
        (
            "cp425_supply_temperature_state_owner_count",
            "unchanged_supply_temperature_preservation_count",
            count(predecessor, "cp424_supply_temperature_state_owner_count"),
        ),
    ] {
        assert_eq!(count(lifecycle, owner), expected, "CP426 {owner}");
        assert_eq!(count(lifecycle, preserved), expected, "CP426 {preserved}");
    }

    let latest = lifecycle["latest"]
        .as_object()
        .expect("CP426 latest object");
    let predecessor_latest = predecessor["latest"]
        .as_object()
        .expect("CP425 latest object");
    assert_eq!(latest["source_order"], json!(ORDER));
    for (cp426_field, cp425_field) in [
        (
            "predecessor_cp425_resulting_supply_humidity_ratio_ieee_bits",
            "resulting_supply_humidity_ratio_ieee_bits",
        ),
        (
            "predecessor_cp425_resulting_supply_enthalpy_j_per_kg_ieee_bits",
            "resulting_supply_enthalpy_j_per_kg_ieee_bits",
        ),
        (
            "predecessor_cp425_resulting_supply_temperature_c_ieee_bits",
            "resulting_supply_temperature_c_ieee_bits",
        ),
    ] {
        assert_eq!(latest[cp426_field], predecessor_latest[cp425_field]);
    }
    let executed =
        latest["cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment_executed"]
            .as_bool()
            .expect("CP426 assignment marker");
    assert_eq!(
        executed,
        predecessor_latest
            ["cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_executed"]
            .as_bool()
            .expect("CP425 assignment marker")
    );
    if executed {
        for key in [
            "mixed_air_humidity_ratio_for_zero_supply_mass_flow_supply_humidity_ratio_assignment_ieee_bits",
            "assigned_supply_humidity_ratio_from_mixed_air_ieee_bits",
            "resulting_supply_humidity_ratio_ieee_bits",
        ] {
            assert!(!latest[key].is_null(), "active CP426 {key}");
        }
        assert_eq!(
            latest["assigned_supply_humidity_ratio_from_mixed_air_ieee_bits"],
            latest["mixed_air_humidity_ratio_for_zero_supply_mass_flow_supply_humidity_ratio_assignment_ieee_bits"]
        );
        assert_eq!(
            latest["resulting_supply_humidity_ratio_ieee_bits"],
            latest["assigned_supply_humidity_ratio_from_mixed_air_ieee_bits"]
        );
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
    assert!(!results.to_string().contains(CP426_KEY));
    cp427_assertions::assert_direct(runtime, results);
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP426_KEY));
    assert!(
        runtime[CP426_KEY].is_null(),
        "non-direct runtime must not publish CP426 evidence"
    );
    cp427_assertions::assert_non_direct(runtime);
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
    assert_eq!(declared_tail.len(), 23);
    expected.extend(declared_tail);
    let actual = latest.keys().map(String::as_str).collect::<BTreeSet<_>>();
    assert_eq!(actual, expected);
    assert_eq!(actual.len(), 391);
}

fn assert_schema_and_binding_cardinalities() {
    let core = include_str!(
        "../../../ep_runtime/src/ideal_loads/calc/cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment.rs"
    );
    let snapshot = core
        .split_once("pub struct PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyHumidityRatioMixedAirAssignmentSnapshot")
        .and_then(|(_, tail)| tail.split_once("/// Final selected-unit CP426"))
        .map(|(snapshot, _)| snapshot)
        .expect("CP426 snapshot declaration");
    let fields = snapshot
        .lines()
        .filter(|line| line.trim_start().starts_with("pub "))
        .collect::<Vec<_>>();
    assert_eq!(fields.len(), 287);
    let names = fields
        .iter()
        .filter_map(|line| line.trim().strip_prefix("pub "))
        .filter_map(|line| line.split_once(':').map(|(name, _)| name))
        .collect::<BTreeSet<_>>();
    assert_eq!(names.len(), 287);
    assert_eq!(snapshot.matches("Option<f64>").count(), 104);
    assert_eq!(snapshot.matches("Option<bool>").count(), 2);
    assert_eq!(snapshot.matches("Option<").count() - 104 - 2, 1);
    let binding = include_str!("../../../ep_runtime/src/ideal_loads/binding/scheduled_output.rs");
    assert_eq!(binding.matches("    pub calculation_").count(), 128);
}

fn array<'a>(lifecycle: &'a Value, field: &str) -> &'a Vec<Value> {
    lifecycle[field].as_array().expect("CP426 route array")
}

fn count(lifecycle: &Value, field: &str) -> u64 {
    count_value(&lifecycle[field])
}

fn count_value(value: &Value) -> u64 {
    value.as_u64().expect("CP426 unsigned count")
}
