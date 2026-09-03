//! CP427 structural zero-flow supply-temperature assignment assertions.

use std::collections::BTreeSet;

use serde_json::{Map, Value, json};

#[path = "cp428_assertions.rs"]
mod cp428_assertions;

const CP426_KEY: &str = "purchased_air_calc_cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment_lifecycle";
const CP427_KEY: &str = "purchased_air_calc_cooling_zero_supply_mass_flow_supply_temperature_mixed_air_assignment_lifecycle";
const ORDER: [&str; 2] = [
    "read-retained-mixed-air-temperature-for-zero-supply-mass-flow-supply-temperature-assignment",
    "assign-purchased-air-supply-temperature-from-mixed-air-temperature-for-zero-supply-mass-flow",
];
const PUBLIC: [usize; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 20, 21, 26, 27];
const TAIL: [&str; 23] = [
    "predecessor_cp426_resulting_supply_humidity_ratio",
    "predecessor_cp426_resulting_supply_humidity_ratio_ieee_bits",
    "predecessor_cp426_resulting_supply_enthalpy_j_per_kg",
    "predecessor_cp426_resulting_supply_enthalpy_j_per_kg_ieee_bits",
    "predecessor_cp426_resulting_supply_temperature_c",
    "predecessor_cp426_resulting_supply_temperature_c_ieee_bits",
    "cooling_zero_supply_mass_flow_supply_temperature_mixed_air_assignment_executed",
    "cp426_retained_supply_humidity_ratio_state_owned",
    "cp426_retained_supply_enthalpy_state_owned",
    "cp426_retained_supply_temperature_state_owned",
    "cp329_retained_mixed_air_temperature_for_zero_supply_mass_flow_supply_temperature_assignment_owned_read",
    "mixed_air_temperature_for_zero_supply_mass_flow_supply_temperature_assignment_read",
    "mixed_air_temperature_for_zero_supply_mass_flow_supply_temperature_assignment_c",
    "mixed_air_temperature_for_zero_supply_mass_flow_supply_temperature_assignment_c_ieee_bits",
    "zero_supply_mass_flow_supply_temperature_mixed_air_assignment_performed",
    "assigned_supply_temperature_from_mixed_air_c",
    "assigned_supply_temperature_from_mixed_air_c_ieee_bits",
    "resulting_supply_humidity_ratio",
    "resulting_supply_humidity_ratio_ieee_bits",
    "resulting_supply_enthalpy_j_per_kg",
    "resulting_supply_enthalpy_j_per_kg_ieee_bits",
    "resulting_supply_temperature_c",
    "resulting_supply_temperature_c_ieee_bits",
];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let lifecycle = &runtime[CP427_KEY];
    let predecessor = &runtime[CP426_KEY];
    assert_eq!(
        lifecycle["source"],
        json!("EnergyPlus 26.1 PurchasedAirManager.cc:2342")
    );
    assert_eq!(
        lifecycle["first_excluded_source"],
        json!("EnergyPlus 26.1 PurchasedAirManager.cc:2343")
    );
    let routes = array(lifecycle, "predecessor_route_counts");
    let assignments = array(
        lifecycle,
        "zero_supply_mass_flow_supply_temperature_mixed_air_assignment_route_counts",
    );
    assert_eq!(routes.len(), 36);
    assert_eq!(assignments.len(), 36);
    for index in 0..36 {
        if !PUBLIC.contains(&index) {
            assert_eq!(count_value(&routes[index]), 0, "CP427 route {index}");
            assert_eq!(
                count_value(&assignments[index]),
                0,
                "CP427 assignment {index}"
            );
        }
        assert_eq!(
            count_value(&assignments[index]),
            if index == 2 {
                count_value(&routes[index])
            } else {
                0
            },
            "CP427 sole assignment route {index}"
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
            "zero_supply_mass_flow_supply_temperature_mixed_air_assignment_count",
            assignment_count,
        ),
        ("source_site_execution_count", assignment_count * 2),
        (
            "cp427_supply_temperature_state_owner_count",
            assignment_count,
        ),
        (
            "cp329_retained_mixed_air_temperature_for_zero_supply_mass_flow_supply_temperature_assignment_owned_read_count",
            assignment_count,
        ),
        (
            "mixed_air_temperature_for_zero_supply_mass_flow_supply_temperature_assignment_read_count",
            assignment_count,
        ),
        (
            "supply_temperature_assignment_write_count",
            assignment_count,
        ),
    ] {
        assert_eq!(count(lifecycle, field), expected, "CP427 {field}");
    }
    let predecessor_w_owners = count(predecessor, "cp425_supply_humidity_ratio_state_owner_count")
        + count(predecessor, "cp426_supply_humidity_ratio_state_owner_count");
    for (owner, preserved, expected) in [
        (
            "cp426_supply_humidity_ratio_state_owner_count",
            "unchanged_supply_humidity_ratio_preservation_count",
            predecessor_w_owners,
        ),
        (
            "cp426_supply_enthalpy_state_owner_count",
            "unchanged_supply_enthalpy_preservation_count",
            count(predecessor, "cp425_supply_enthalpy_state_owner_count"),
        ),
        (
            "cp426_supply_temperature_state_owner_count",
            "unchanged_supply_temperature_preservation_count",
            count(predecessor, "cp425_supply_temperature_state_owner_count"),
        ),
    ] {
        assert_eq!(count(lifecycle, owner), expected, "CP427 {owner}");
        assert_eq!(count(lifecycle, preserved), expected, "CP427 {preserved}");
    }

    let latest = lifecycle["latest"]
        .as_object()
        .expect("CP427 latest object");
    let predecessor_latest = predecessor["latest"]
        .as_object()
        .expect("CP426 latest object");
    assert_eq!(latest["source_order"], json!(ORDER));
    for (cp427_field, cp426_field) in [
        (
            "predecessor_cp426_resulting_supply_humidity_ratio_ieee_bits",
            "resulting_supply_humidity_ratio_ieee_bits",
        ),
        (
            "predecessor_cp426_resulting_supply_enthalpy_j_per_kg_ieee_bits",
            "resulting_supply_enthalpy_j_per_kg_ieee_bits",
        ),
        (
            "predecessor_cp426_resulting_supply_temperature_c_ieee_bits",
            "resulting_supply_temperature_c_ieee_bits",
        ),
    ] {
        assert_eq!(latest[cp427_field], predecessor_latest[cp426_field]);
    }
    assert_eq!(
        latest["resulting_supply_humidity_ratio_ieee_bits"],
        predecessor_latest["resulting_supply_humidity_ratio_ieee_bits"]
    );
    assert_eq!(
        latest["resulting_supply_enthalpy_j_per_kg_ieee_bits"],
        predecessor_latest["resulting_supply_enthalpy_j_per_kg_ieee_bits"]
    );
    let assignment =
        latest["cooling_zero_supply_mass_flow_supply_temperature_mixed_air_assignment_executed"]
            .as_bool()
            .expect("CP427 assignment marker");
    assert_eq!(
        assignment,
        predecessor_latest
            ["cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment_executed"]
            .as_bool()
            .expect("CP426 assignment marker")
    );
    let local = [
        "mixed_air_temperature_for_zero_supply_mass_flow_supply_temperature_assignment_c_ieee_bits",
        "assigned_supply_temperature_from_mixed_air_c_ieee_bits",
    ];
    if assignment {
        for key in local {
            assert!(!latest[key].is_null(), "active CP427 {key}");
        }
        assert_eq!(latest[local[0]], latest[local[1]]);
        assert_eq!(
            latest[local[1]],
            latest["resulting_supply_temperature_c_ieee_bits"]
        );
    } else {
        for key in local {
            assert!(latest[key].is_null(), "inactive CP427 {key}");
        }
        assert_eq!(
            latest["resulting_supply_temperature_c_ieee_bits"],
            predecessor_latest["resulting_supply_temperature_c_ieee_bits"]
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
    assert!(!results.to_string().contains(CP427_KEY));
    cp428_assertions::assert_direct(runtime, results);
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP427_KEY));
    assert!(
        runtime[CP427_KEY].is_null(),
        "non-direct runtime must not publish CP427 evidence"
    );
    cp428_assertions::assert_non_direct(runtime);
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
    assert_eq!(actual.len(), 408);
}

fn assert_schema_and_binding_cardinalities() {
    let core = include_str!(
        "../../../ep_runtime/src/ideal_loads/calc/cooling_zero_supply_mass_flow_supply_temperature_mixed_air_assignment.rs"
    );
    let snapshot = core
        .split_once("pub struct PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyTemperatureMixedAirAssignmentSnapshot")
        .and_then(|(_, tail)| tail.split_once("/// Final selected-unit CP427"))
        .map(|(snapshot, _)| snapshot)
        .expect("CP427 snapshot declaration");
    let fields = snapshot
        .lines()
        .filter(|line| line.trim_start().starts_with("pub "))
        .collect::<Vec<_>>();
    assert_eq!(fields.len(), 299);
    let names = fields
        .iter()
        .filter_map(|line| line.trim().strip_prefix("pub "))
        .filter_map(|line| line.split_once(':').map(|(name, _)| name))
        .collect::<BTreeSet<_>>();
    assert_eq!(names.len(), 299);
    assert_eq!(snapshot.matches("Option<f64>").count(), 109);
    assert_eq!(snapshot.matches("Option<bool>").count(), 2);
    assert_eq!(snapshot.matches("Option<").count() - 109 - 2, 1);
    let binding = include_str!("../../../ep_runtime/src/ideal_loads/binding/scheduled_output.rs");
    assert_eq!(binding.matches("    pub calculation_").count(), 130);
}

fn array<'a>(lifecycle: &'a Value, field: &str) -> &'a Vec<Value> {
    lifecycle[field].as_array().expect("CP427 route array")
}

fn count(lifecycle: &Value, field: &str) -> u64 {
    count_value(&lifecycle[field])
}

fn count_value(value: &Value) -> u64 {
    value.as_u64().expect("CP427 unsigned count")
}
