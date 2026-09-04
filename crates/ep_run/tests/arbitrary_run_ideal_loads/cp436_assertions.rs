//! CP436 heating maximum-flow-body outdoor-air volume-flow assignment assertions.

use std::collections::BTreeSet;

use serde_json::{Map, Value, json};

#[path = "cp437_assertions.rs"]
mod cp437_assertions;

const CP435_KEY: &str = "purchased_air_calc_heating_outdoor_air_maximum_flow_guard_lifecycle";
const CP436_KEY: &str =
    "purchased_air_calc_heating_outdoor_air_maximum_flow_body_volume_flow_assignment_lifecycle";
const ORDER: [&str; 4] = [
    "read-cp435-retained-outdoor-air-mass-flow-for-outdoor-air-volume-flow-division",
    "read-environment-standard-air-density-for-outdoor-air-volume-flow-division",
    "calculate-outdoor-air-mass-flow-divided-by-standard-air-density",
    "assign-local-outdoor-air-volume-flow-rate",
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
const TAIL: [&str; 30] = [
    "predecessor_cp435_resulting_supply_humidity_ratio",
    "predecessor_cp435_resulting_supply_humidity_ratio_ieee_bits",
    "predecessor_cp435_resulting_supply_enthalpy_j_per_kg",
    "predecessor_cp435_resulting_supply_enthalpy_j_per_kg_ieee_bits",
    "predecessor_cp435_resulting_supply_temperature_c",
    "predecessor_cp435_resulting_supply_temperature_c_ieee_bits",
    "heating_outdoor_air_maximum_flow_body_volume_flow_assignment_executed",
    "cp435_retained_supply_humidity_ratio_state_owned",
    "cp435_retained_supply_enthalpy_state_owned",
    "cp435_retained_supply_temperature_state_owned",
    "cp435_retained_outdoor_air_mass_flow_rate_owned_read",
    "outdoor_air_mass_flow_rate_for_outdoor_air_volume_flow_division_read",
    "outdoor_air_mass_flow_rate_for_outdoor_air_volume_flow_division_kg_per_s",
    "outdoor_air_mass_flow_rate_for_outdoor_air_volume_flow_division_kg_per_s_ieee_bits",
    "begin_environment_standard_air_density_owned_read",
    "standard_air_density_for_outdoor_air_volume_flow_division_read",
    "standard_air_density_for_outdoor_air_volume_flow_division_kg_per_m3",
    "standard_air_density_for_outdoor_air_volume_flow_division_kg_per_m3_ieee_bits",
    "outdoor_air_mass_flow_rate_standard_air_density_division_evaluated",
    "calculated_outdoor_air_volume_flow_rate_m3_per_s",
    "calculated_outdoor_air_volume_flow_rate_m3_per_s_ieee_bits",
    "local_outdoor_air_volume_flow_rate_assignment_performed",
    "assigned_outdoor_air_volume_flow_rate_m3_per_s",
    "assigned_outdoor_air_volume_flow_rate_m3_per_s_ieee_bits",
    "resulting_supply_humidity_ratio",
    "resulting_supply_humidity_ratio_ieee_bits",
    "resulting_supply_enthalpy_j_per_kg",
    "resulting_supply_enthalpy_j_per_kg_ieee_bits",
    "resulting_supply_temperature_c",
    "resulting_supply_temperature_c_ieee_bits",
];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let lifecycle = &runtime[CP436_KEY];
    let predecessor = &runtime[CP435_KEY];
    assert_eq!(
        lifecycle["source"],
        json!("EnergyPlus 26.1 PurchasedAirManager.cc:2363")
    );
    assert_eq!(
        lifecycle["first_excluded_source"],
        json!("EnergyPlus 26.1 PurchasedAirManager.cc:2364")
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
        lifecycle["predecessor_guard_false_fallthrough_route_counts"],
        predecessor["heating_outdoor_air_maximum_flow_guard_false_fallthrough_route_counts"]
    );
    assert_eq!(
        lifecycle["predecessor_guard_body_entry_route_counts"],
        predecessor["maximum_heating_flow_body_entry_route_counts"]
    );
    assert_eq!(
        lifecycle["heating_outdoor_air_volume_flow_assignment_route_counts"],
        predecessor["maximum_heating_flow_body_entry_route_counts"]
    );
    assert_eq!(
        lifecycle
            .as_object()
            .expect("CP436 lifecycle object")
            .keys()
            .filter(|key| key.ends_with("_route_counts"))
            .count(),
        4
    );
    for field in [
        "predecessor_route_counts",
        "predecessor_guard_false_fallthrough_route_counts",
        "predecessor_guard_body_entry_route_counts",
        "heating_outdoor_air_volume_flow_assignment_route_counts",
    ] {
        let values = array(lifecycle, field);
        assert_eq!(values.len(), 36, "CP436 {field} width");
        for (index, value) in values.iter().enumerate() {
            if !PUBLIC.contains(&index) {
                assert_eq!(count_value(value), 0, "CP436 {field} route {index}");
            }
        }
    }
    let transitions = sum(array(lifecycle, "predecessor_route_counts"));
    let assignments = sum(array(
        lifecycle,
        "heating_outdoor_air_volume_flow_assignment_route_counts",
    ));
    assert_eq!(assignments, 0, "exact public release admits no CP436 body");
    assert_eq!(count(lifecycle, "transition_count"), transitions);
    assert_eq!(count(lifecycle, "inactive_transition_count"), transitions);
    for field in [
        "outdoor_air_volume_flow_assignment_count",
        "cp435_outdoor_air_mass_flow_rate_owned_read_count",
        "outdoor_air_mass_flow_rate_for_volume_flow_division_read_count",
        "begin_environment_standard_air_density_owner_count",
        "standard_air_density_for_volume_flow_division_read_count",
        "outdoor_air_mass_flow_rate_standard_air_density_division_count",
        "local_outdoor_air_volume_flow_rate_assignment_write_count",
    ] {
        assert_eq!(count(lifecycle, field), assignments, "CP436 {field}");
    }
    assert_eq!(
        count(lifecycle, "source_site_execution_count"),
        assignments * 4
    );
    for (owner, preserved, predecessor_owner) in [
        (
            "cp435_supply_humidity_ratio_state_owner_count",
            "unchanged_supply_humidity_ratio_preservation_count",
            "cp434_supply_humidity_ratio_state_owner_count",
        ),
        (
            "cp435_supply_enthalpy_state_owner_count",
            "unchanged_supply_enthalpy_preservation_count",
            "cp434_supply_enthalpy_state_owner_count",
        ),
        (
            "cp435_supply_temperature_state_owner_count",
            "unchanged_supply_temperature_preservation_count",
            "cp434_supply_temperature_state_owner_count",
        ),
    ] {
        let expected = count(predecessor, predecessor_owner);
        assert_eq!(count(lifecycle, owner), expected, "CP436 {owner}");
        assert_eq!(count(lifecycle, preserved), expected, "CP436 {preserved}");
    }

    let latest = lifecycle["latest"]
        .as_object()
        .expect("CP436 latest object");
    let predecessor_latest = predecessor["latest"]
        .as_object()
        .expect("CP435 latest object");
    assert_actual_json_key_set(latest, predecessor_latest);
    assert_eq!(latest.len(), 542);
    assert_eq!(
        latest
            .keys()
            .filter(|key| key.ends_with("_ieee_bits"))
            .count(),
        140
    );
    for (key, expected) in predecessor_latest {
        if !["source", "first_excluded_source", "source_order"].contains(&key.as_str())
            && !REPLACED_PREDECESSOR_TAIL.contains(&key.as_str())
        {
            assert_eq!(latest.get(key).expect("CP436 predecessor key"), expected);
        }
    }
    for (cp436_field, cp435_field) in [
        (
            "predecessor_cp435_resulting_supply_humidity_ratio_ieee_bits",
            "resulting_supply_humidity_ratio_ieee_bits",
        ),
        (
            "predecessor_cp435_resulting_supply_enthalpy_j_per_kg_ieee_bits",
            "resulting_supply_enthalpy_j_per_kg_ieee_bits",
        ),
        (
            "predecessor_cp435_resulting_supply_temperature_c_ieee_bits",
            "resulting_supply_temperature_c_ieee_bits",
        ),
    ] {
        assert_eq!(latest[cp436_field], predecessor_latest[cp435_field]);
        assert_eq!(latest[cp435_field], predecessor_latest[cp435_field]);
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
    assert!(!results.to_string().contains(CP436_KEY));
    cp437_assertions::assert_direct(runtime, results);
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP436_KEY));
    assert!(
        runtime[CP436_KEY].is_null(),
        "non-direct runtime must not publish CP436 evidence"
    );
    cp437_assertions::assert_non_direct(runtime);
}

fn assert_public_skip_shape(latest: &Map<String, Value>) {
    for field in [
        "heating_outdoor_air_maximum_flow_body_volume_flow_assignment_executed",
        "cp435_retained_outdoor_air_mass_flow_rate_owned_read",
        "outdoor_air_mass_flow_rate_for_outdoor_air_volume_flow_division_read",
        "begin_environment_standard_air_density_owned_read",
        "standard_air_density_for_outdoor_air_volume_flow_division_read",
        "outdoor_air_mass_flow_rate_standard_air_density_division_evaluated",
        "local_outdoor_air_volume_flow_rate_assignment_performed",
    ] {
        assert_eq!(latest[field], json!(false), "CP436 skipped {field}");
    }
    for field in [
        "outdoor_air_mass_flow_rate_for_outdoor_air_volume_flow_division_kg_per_s",
        "outdoor_air_mass_flow_rate_for_outdoor_air_volume_flow_division_kg_per_s_ieee_bits",
        "standard_air_density_for_outdoor_air_volume_flow_division_kg_per_m3",
        "standard_air_density_for_outdoor_air_volume_flow_division_kg_per_m3_ieee_bits",
        "calculated_outdoor_air_volume_flow_rate_m3_per_s",
        "calculated_outdoor_air_volume_flow_rate_m3_per_s_ieee_bits",
        "assigned_outdoor_air_volume_flow_rate_m3_per_s",
        "assigned_outdoor_air_volume_flow_rate_m3_per_s_ieee_bits",
    ] {
        assert!(latest[field].is_null(), "CP436 skipped {field}");
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
    assert_eq!(tail.len(), 30);
    expected.extend(tail);
    let actual = latest.keys().map(String::as_str).collect::<BTreeSet<_>>();
    assert_eq!(actual, expected);
    assert_eq!(actual.len(), 542);
}

fn assert_schema_and_binding_cardinalities() {
    let core = include_str!(
        "../../../ep_runtime/src/ideal_loads/calc/heating_outdoor_air_maximum_flow_body_volume_flow_assignment.rs"
    );
    let snapshot = core
        .split_once(
            "pub struct PurchasedAirCalcHeatingOutdoorAirMaximumFlowBodyVolumeFlowAssignmentSnapshot",
        )
        .and_then(|(_, tail)| tail.split_once("/// Final selected-unit CP436"))
        .map(|(snapshot, _)| snapshot)
        .expect("CP436 snapshot declaration");
    let fields = snapshot
        .lines()
        .filter(|line| line.trim_start().starts_with("pub "))
        .collect::<Vec<_>>();
    assert_eq!(fields.len(), 402);
    assert_eq!(
        fields
            .iter()
            .filter_map(|line| line.trim().strip_prefix("pub "))
            .filter_map(|line| line.split_once(':').map(|(name, _)| name))
            .collect::<BTreeSet<_>>()
            .len(),
        402
    );
    assert_eq!(snapshot.matches("Option<f64>").count(), 140);
    assert_eq!(snapshot.matches("Option<bool>").count(), 8);
    assert_eq!(snapshot.matches("Option<").count() - 140 - 8, 6);
    let binding = include_str!("../../../ep_runtime/src/ideal_loads/binding/scheduled_output.rs");
    let fields = binding
        .lines()
        .filter(|line| line.starts_with("    pub calculation_"))
        .collect::<Vec<_>>();
    assert_eq!(fields.len(), 131);
    assert!(fields[125].contains("calculation_heating_outdoor_air_maximum_flow_guard"));
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
    lifecycle[field].as_array().expect("CP436 route array")
}

fn sum(values: &[Value]) -> u64 {
    values.iter().map(count_value).sum()
}

fn count(lifecycle: &Value, field: &str) -> u64 {
    count_value(&lifecycle[field])
}

fn count_value(value: &Value) -> u64 {
    value.as_u64().expect("CP436 unsigned count")
}
