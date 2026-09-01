//! CP435 structural heating outdoor-air maximum-flow guard assertions.

use std::collections::BTreeSet;

use serde_json::{Map, Value, json};

#[path = "cp436_assertions.rs"]
mod cp436_assertions;

const CP434_KEY: &str = "purchased_air_calc_heating_operating_mode_deadband_assignment_lifecycle";
const CP435_KEY: &str = "purchased_air_calc_heating_outdoor_air_maximum_flow_guard_lifecycle";
const ORDER: [&str; 6] = [
    "compare-heating-limit-to-flow-rate",
    "compare-heating-limit-to-flow-rate-and-capacity-after-short-circuit",
    "read-outdoor-air-mass-flow-after-limit-short-circuit",
    "read-maximum-heating-air-mass-flow-after-limit-short-circuit",
    "compare-strict-outdoor-air-above-maximum-heating-flow",
    "enter-maximum-heating-flow-body-if-satisfied",
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
const TAIL: [&str; 35] = [
    "predecessor_cp434_resulting_supply_humidity_ratio",
    "predecessor_cp434_resulting_supply_humidity_ratio_ieee_bits",
    "predecessor_cp434_resulting_supply_enthalpy_j_per_kg",
    "predecessor_cp434_resulting_supply_enthalpy_j_per_kg_ieee_bits",
    "predecessor_cp434_resulting_supply_temperature_c",
    "predecessor_cp434_resulting_supply_temperature_c_ieee_bits",
    "heating_outdoor_air_maximum_flow_guard_evaluated",
    "heating_limit_flow_rate_comparison_evaluated",
    "heating_limit_flow_rate_value",
    "heating_limit_flow_rate_comparison_satisfied",
    "heating_limit_flow_rate_and_capacity_comparison_evaluated",
    "heating_limit_flow_rate_and_capacity_value",
    "heating_limit_flow_rate_and_capacity_comparison_satisfied",
    "heating_flow_limit_active",
    "heating_flow_limit_selector_rejected",
    "cp311_same_call_outdoor_air_mass_flow_rate_bit_corroborated",
    "outdoor_air_mass_flow_rate_read_after_heating_limit_short_circuit",
    "outdoor_air_mass_flow_rate_for_heating_maximum_flow_guard_kg_per_s",
    "outdoor_air_mass_flow_rate_for_heating_maximum_flow_guard_kg_per_s_ieee_bits",
    "maximum_heating_air_mass_flow_rate_read_after_heating_limit_short_circuit",
    "maximum_heating_air_mass_flow_rate_for_guard_kg_per_s",
    "maximum_heating_air_mass_flow_rate_for_guard_kg_per_s_ieee_bits",
    "outdoor_air_mass_flow_rate_maximum_heating_air_mass_flow_rate_comparison_evaluated",
    "outdoor_air_mass_flow_rate_strictly_greater_than_maximum_heating_air_mass_flow_rate",
    "maximum_heating_flow_body_entered",
    "heating_outdoor_air_maximum_flow_guard_false_fallthrough",
    "cp434_retained_supply_humidity_ratio_state_owned",
    "cp434_retained_supply_enthalpy_state_owned",
    "cp434_retained_supply_temperature_state_owned",
    "resulting_supply_humidity_ratio",
    "resulting_supply_humidity_ratio_ieee_bits",
    "resulting_supply_enthalpy_j_per_kg",
    "resulting_supply_enthalpy_j_per_kg_ieee_bits",
    "resulting_supply_temperature_c",
    "resulting_supply_temperature_c_ieee_bits",
];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let lifecycle = &runtime[CP435_KEY];
    let predecessor = &runtime[CP434_KEY];
    assert_eq!(
        lifecycle["source"],
        json!("EnergyPlus 26.1 PurchasedAirManager.cc:2361-2362")
    );
    assert_eq!(
        lifecycle["first_excluded_source"],
        json!("EnergyPlus 26.1 PurchasedAirManager.cc:2363")
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
        lifecycle
            .as_object()
            .expect("CP435 lifecycle object")
            .keys()
            .filter(|key| key.ends_with("_route_counts"))
            .count(),
        3
    );
    for field in [
        "predecessor_route_counts",
        "heating_outdoor_air_maximum_flow_guard_false_fallthrough_route_counts",
        "maximum_heating_flow_body_entry_route_counts",
    ] {
        let values = array(lifecycle, field);
        assert_eq!(values.len(), 36, "CP435 {field} width");
        for (index, value) in values.iter().enumerate() {
            if !PUBLIC.contains(&index) {
                assert_eq!(count_value(value), 0, "CP435 {field} route {index}");
            }
        }
    }
    assert!(
        array(lifecycle, "maximum_heating_flow_body_entry_route_counts")
            .iter()
            .all(|value| count_value(value) == 0)
    );
    let transitions = sum(array(lifecycle, "predecessor_route_counts"));
    let false_fallthroughs = sum(array(
        lifecycle,
        "heating_outdoor_air_maximum_flow_guard_false_fallthrough_route_counts",
    ));
    let body_entries = sum(array(
        lifecycle,
        "maximum_heating_flow_body_entry_route_counts",
    ));
    let evaluations = false_fallthroughs + body_entries;
    assert_eq!(count(lifecycle, "transition_count"), transitions);
    assert_eq!(
        count(lifecycle, "inactive_transition_count"),
        transitions - evaluations
    );
    for field in [
        "heating_outdoor_air_maximum_flow_guard_evaluation_count",
        "heating_limit_flow_rate_comparison_count",
        "heating_limit_flow_rate_and_capacity_comparison_count",
        "heating_limit_flow_rate_and_capacity_match_count",
        "cp311_same_call_outdoor_air_mass_flow_rate_bit_corroboration_count",
        "outdoor_air_mass_flow_rate_read_after_heating_limit_short_circuit_count",
        "maximum_heating_air_mass_flow_rate_read_after_heating_limit_short_circuit_count",
        "outdoor_air_mass_flow_rate_maximum_heating_air_mass_flow_rate_comparison_count",
    ] {
        assert_eq!(count(lifecycle, field), evaluations, "CP435 {field}");
    }
    assert_eq!(count(lifecycle, "heating_limit_flow_rate_match_count"), 0);
    assert_eq!(
        count(lifecycle, "heating_flow_limit_selector_rejection_count"),
        0
    );
    assert_eq!(
        count(
            lifecycle,
            "heating_outdoor_air_maximum_flow_guard_false_fallthrough_count"
        ),
        false_fallthroughs
    );
    for field in [
        "maximum_heating_flow_body_entry_count",
        "outdoor_air_mass_flow_rate_strictly_greater_than_maximum_heating_air_mass_flow_rate_count",
    ] {
        assert_eq!(count(lifecycle, field), body_entries, "CP435 {field}");
    }
    assert_eq!(
        count(lifecycle, "source_site_execution_count"),
        evaluations * 5 + body_entries
    );
    for (owner, preserved, predecessor_owner) in [
        (
            "cp434_supply_humidity_ratio_state_owner_count",
            "unchanged_supply_humidity_ratio_preservation_count",
            "cp433_supply_humidity_ratio_state_owner_count",
        ),
        (
            "cp434_supply_enthalpy_state_owner_count",
            "unchanged_supply_enthalpy_preservation_count",
            "cp433_supply_enthalpy_state_owner_count",
        ),
        (
            "cp434_supply_temperature_state_owner_count",
            "unchanged_supply_temperature_preservation_count",
            "cp433_supply_temperature_state_owner_count",
        ),
    ] {
        let expected = count(predecessor, predecessor_owner);
        assert_eq!(count(lifecycle, owner), expected, "CP435 {owner}");
        assert_eq!(count(lifecycle, preserved), expected, "CP435 {preserved}");
    }

    let latest = lifecycle["latest"]
        .as_object()
        .expect("CP435 latest object");
    let predecessor_latest = predecessor["latest"]
        .as_object()
        .expect("CP434 latest object");
    assert_actual_json_key_set(latest, predecessor_latest);
    assert_eq!(latest.len(), 518);
    assert_eq!(
        latest
            .keys()
            .filter(|key| key.ends_with("_ieee_bits"))
            .count(),
        133
    );
    for (key, expected) in predecessor_latest {
        if !["source", "first_excluded_source", "source_order"].contains(&key.as_str())
            && !REPLACED_PREDECESSOR_TAIL.contains(&key.as_str())
        {
            assert_eq!(
                latest.get(key).expect("CP435 retained predecessor key"),
                expected
            );
        }
    }
    for (cp435_field, cp434_field) in [
        (
            "predecessor_cp434_resulting_supply_humidity_ratio_ieee_bits",
            "resulting_supply_humidity_ratio_ieee_bits",
        ),
        (
            "predecessor_cp434_resulting_supply_enthalpy_j_per_kg_ieee_bits",
            "resulting_supply_enthalpy_j_per_kg_ieee_bits",
        ),
        (
            "predecessor_cp434_resulting_supply_temperature_c_ieee_bits",
            "resulting_supply_temperature_c_ieee_bits",
        ),
    ] {
        assert_eq!(latest[cp435_field], predecessor_latest[cp434_field]);
        assert_eq!(latest[cp434_field], predecessor_latest[cp434_field]);
    }
    for (owner, predecessor_bits) in [
        (
            "cp434_retained_supply_humidity_ratio_state_owned",
            "resulting_supply_humidity_ratio_ieee_bits",
        ),
        (
            "cp434_retained_supply_enthalpy_state_owned",
            "resulting_supply_enthalpy_j_per_kg_ieee_bits",
        ),
        (
            "cp434_retained_supply_temperature_state_owned",
            "resulting_supply_temperature_c_ieee_bits",
        ),
    ] {
        assert_eq!(
            latest[owner],
            json!(!predecessor_latest[predecessor_bits].is_null())
        );
    }
    assert_public_latest_guard_shape(latest, predecessor_latest);
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
    assert!(!results.to_string().contains(CP435_KEY));
    cp436_assertions::assert_direct(runtime, results);
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP435_KEY));
    assert!(
        runtime[CP435_KEY].is_null(),
        "non-direct runtime must not publish CP435 evidence"
    );
    cp436_assertions::assert_non_direct(runtime);
}

fn assert_public_latest_guard_shape(
    latest: &Map<String, Value>,
    predecessor_latest: &Map<String, Value>,
) {
    let evaluated = predecessor_latest["heating_or_no_load_case_entered"]
        .as_bool()
        .expect("CP434 heating-case marker");
    assert_eq!(
        latest["heating_outdoor_air_maximum_flow_guard_evaluated"],
        json!(evaluated)
    );
    if !evaluated {
        for field in [
            "heating_limit_flow_rate_comparison_evaluated",
            "heating_limit_flow_rate_and_capacity_comparison_evaluated",
            "heating_flow_limit_selector_rejected",
            "cp311_same_call_outdoor_air_mass_flow_rate_bit_corroborated",
            "outdoor_air_mass_flow_rate_read_after_heating_limit_short_circuit",
            "maximum_heating_air_mass_flow_rate_read_after_heating_limit_short_circuit",
            "outdoor_air_mass_flow_rate_maximum_heating_air_mass_flow_rate_comparison_evaluated",
            "maximum_heating_flow_body_entered",
            "heating_outdoor_air_maximum_flow_guard_false_fallthrough",
        ] {
            assert_eq!(latest[field], json!(false), "CP435 skipped {field}");
        }
        for field in [
            "heating_limit_flow_rate_value",
            "heating_limit_flow_rate_comparison_satisfied",
            "heating_limit_flow_rate_and_capacity_value",
            "heating_limit_flow_rate_and_capacity_comparison_satisfied",
            "heating_flow_limit_active",
            "outdoor_air_mass_flow_rate_for_heating_maximum_flow_guard_kg_per_s_ieee_bits",
            "maximum_heating_air_mass_flow_rate_for_guard_kg_per_s_ieee_bits",
            "outdoor_air_mass_flow_rate_strictly_greater_than_maximum_heating_air_mass_flow_rate",
        ] {
            assert!(latest[field].is_null(), "CP435 skipped {field}");
        }
        return;
    }
    assert_eq!(
        latest["heating_limit_flow_rate_comparison_evaluated"],
        json!(true)
    );
    assert_eq!(
        latest["heating_limit_flow_rate_value"],
        json!("LimitFlowRateAndCapacity")
    );
    assert_eq!(
        latest["heating_limit_flow_rate_comparison_satisfied"],
        json!(false)
    );
    assert_eq!(
        latest["heating_limit_flow_rate_and_capacity_comparison_evaluated"],
        json!(true)
    );
    assert_eq!(
        latest["heating_limit_flow_rate_and_capacity_value"],
        json!("LimitFlowRateAndCapacity")
    );
    assert_eq!(
        latest["heating_limit_flow_rate_and_capacity_comparison_satisfied"],
        json!(true)
    );
    assert_eq!(latest["heating_flow_limit_active"], json!(true));
    assert_eq!(latest["heating_flow_limit_selector_rejected"], json!(false));
    assert_eq!(
        latest["cp311_same_call_outdoor_air_mass_flow_rate_bit_corroborated"],
        json!(true)
    );
    assert_eq!(
        latest["outdoor_air_mass_flow_rate_read_after_heating_limit_short_circuit"],
        json!(true)
    );
    assert_eq!(
        latest["outdoor_air_mass_flow_rate_for_heating_maximum_flow_guard_kg_per_s_ieee_bits"],
        json!("0x0000000000000000")
    );
    assert_eq!(
        latest["maximum_heating_air_mass_flow_rate_read_after_heating_limit_short_circuit"],
        json!(true)
    );
    assert!(!latest["maximum_heating_air_mass_flow_rate_for_guard_kg_per_s_ieee_bits"].is_null());
    assert_eq!(
        latest["outdoor_air_mass_flow_rate_maximum_heating_air_mass_flow_rate_comparison_evaluated"],
        json!(true)
    );
    assert_eq!(
        latest["outdoor_air_mass_flow_rate_strictly_greater_than_maximum_heating_air_mass_flow_rate"],
        json!(false)
    );
    assert_eq!(latest["maximum_heating_flow_body_entered"], json!(false));
    assert_eq!(
        latest["heating_outdoor_air_maximum_flow_guard_false_fallthrough"],
        json!(true)
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
    for key in REPLACED_PREDECESSOR_TAIL {
        assert!(expected.remove(key));
    }
    let tail = TAIL.into_iter().collect::<BTreeSet<_>>();
    assert_eq!(tail.len(), 35);
    expected.extend(tail);
    let actual = latest.keys().map(String::as_str).collect::<BTreeSet<_>>();
    assert_eq!(actual, expected);
    assert_eq!(actual.len(), 518);
}

fn assert_schema_and_binding_cardinalities() {
    let core = include_str!(
        "../../../ep_runtime/src/ideal_loads/calc/heating_outdoor_air_maximum_flow_guard.rs"
    );
    let snapshot = core
        .split_once("pub struct PurchasedAirCalcHeatingOutdoorAirMaximumFlowGuardSnapshot")
        .and_then(|(_, tail)| tail.split_once("/// Final selected-unit CP435"))
        .map(|(snapshot, _)| snapshot)
        .expect("CP435 snapshot declaration");
    let fields = snapshot
        .lines()
        .filter(|line| line.trim_start().starts_with("pub "))
        .collect::<Vec<_>>();
    assert_eq!(fields.len(), 385);
    assert_eq!(
        fields
            .iter()
            .filter_map(|line| line.trim().strip_prefix("pub "))
            .filter_map(|line| line.split_once(':').map(|(name, _)| name))
            .collect::<BTreeSet<_>>()
            .len(),
        385
    );
    assert_eq!(snapshot.matches("Option<f64>").count(), 133);
    assert_eq!(snapshot.matches("Option<bool>").count(), 8);
    assert_eq!(snapshot.matches("Option<").count() - 133 - 8, 6);
    let binding = include_str!("../../../ep_runtime/src/ideal_loads/binding/scheduled_output.rs");
    let fields = binding
        .lines()
        .filter(|line| line.starts_with("    pub calculation_"))
        .collect::<Vec<_>>();
    assert_eq!(fields.len(), 129);
    assert!(fields[124].contains("calculation_heating_operating_mode_deadband_assignment"));
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
    lifecycle[field].as_array().expect("CP435 route array")
}

fn sum(values: &[Value]) -> u64 {
    values.iter().map(count_value).sum()
}

fn count(lifecycle: &Value, field: &str) -> u64 {
    count_value(&lifecycle[field])
}

fn count_value(value: &Value) -> u64 {
    value.as_u64().expect("CP435 unsigned count")
}
