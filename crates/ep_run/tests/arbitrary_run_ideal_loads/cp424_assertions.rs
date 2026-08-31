//! CP424 structural positive-flow-guard else-entry assertions.

use std::collections::BTreeSet;

use serde_json::{Map, Value, json};

#[path = "cp425_assertions.rs"]
mod cp425_assertions;

const CP423_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment_lifecycle";
const CP424_KEY: &str =
    "purchased_air_calc_cooling_supply_mass_flow_positive_guard_else_branch_entry_lifecycle";
const MARKER: &str = "cooling_supply_mass_flow_positive_guard_else_branch_entered";
const ORDER: [&str; 1] =
    ["enter-cooling-supply-mass-flow-positive-guard-else-branch-after-guard-false-fallthrough"];
const PUBLIC: [usize; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 20, 21, 26, 27];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let lifecycle = &runtime[CP424_KEY];
    let predecessor = &runtime[CP423_KEY];
    assert_eq!(
        lifecycle["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2339"
    );
    assert_eq!(
        lifecycle["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2340"
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
        "positive_supply_mass_flow_guard_else_branch_entry_route_counts",
    ] {
        let values = array(lifecycle, field);
        assert_eq!(values.len(), 36, "CP424 {field} width");
        for (index, value) in values.iter().enumerate() {
            if !PUBLIC.contains(&index) {
                assert_eq!(count_value(value), 0, "CP424 {field} route {index}");
            }
        }
    }
    let predecessor_routes = array(lifecycle, "predecessor_route_counts");
    let entry_routes = array(
        lifecycle,
        "positive_supply_mass_flow_guard_else_branch_entry_route_counts",
    );
    for index in 0..36 {
        assert_eq!(
            count_value(&entry_routes[index]),
            if index == 2 {
                count_value(&predecessor_routes[index])
            } else {
                0
            },
            "CP424 sole entry route {index}",
        );
    }
    let transitions = sum(predecessor_routes);
    let entries = sum(entry_routes);
    assert_eq!(count(lifecycle, "transition_count"), transitions);
    assert_eq!(
        count(lifecycle, "inactive_transition_count"),
        transitions - entries
    );
    assert_eq!(
        count(
            lifecycle,
            "positive_supply_mass_flow_guard_else_branch_entry_count"
        ),
        entries
    );
    assert_eq!(count(lifecycle, "source_site_execution_count"), entries);
    for (owner, unchanged, expected) in [
        (
            "cp423_supply_humidity_ratio_state_owner_count",
            "unchanged_supply_humidity_ratio_preservation_count",
            36,
        ),
        (
            "cp423_supply_enthalpy_state_owner_count",
            "unchanged_supply_enthalpy_preservation_count",
            41,
        ),
        (
            "cp423_supply_temperature_state_owner_count",
            "unchanged_supply_temperature_preservation_count",
            56,
        ),
    ] {
        assert_eq!(count(lifecycle, unchanged), count(lifecycle, owner));
        assert!(count(lifecycle, owner) <= expected);
    }

    let latest = lifecycle["latest"]
        .as_object()
        .expect("CP424 latest object");
    let predecessor_latest = predecessor["latest"]
        .as_object()
        .expect("CP423 latest object");
    assert_eq!(latest.len(), 357);
    assert_eq!(predecessor_latest.len(), 356);
    let predecessor_key_set = predecessor_latest
        .keys()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    let mut successor_key_set = latest.keys().map(String::as_str).collect::<BTreeSet<_>>();
    assert!(successor_key_set.remove(MARKER));
    assert_eq!(successor_key_set, predecessor_key_set);
    let predecessor_keys = canonical_cp423_keys();
    let mut keys = predecessor_keys.clone();
    keys.push(MARKER);
    assert_eq!(keys.len(), 357);
    assert_eq!(keys.iter().copied().collect::<BTreeSet<_>>().len(), 357);
    assert_eq!(&keys[..356], predecessor_keys.as_slice());
    assert_eq!(keys[356], MARKER);
    for (index, key) in keys.iter().enumerate() {
        assert!(latest.contains_key(*key), "CP424 JSON key {index}: {key}");
        if let Some(base) = key.strip_suffix("_ieee_bits") {
            assert_eq!(
                keys.get(index.wrapping_sub(1)),
                Some(&base),
                "CP424 adjacent sidecar {key}"
            );
        }
    }
    assert_eq!(
        latest
            .keys()
            .filter(|key| key.ends_with("_ieee_bits"))
            .count(),
        94
    );
    assert_eq!(
        latest[MARKER],
        latest["positive_guard_false_fallthrough_skipped"],
    );
    for field in [
        "resulting_supply_humidity_ratio_ieee_bits",
        "resulting_supply_enthalpy_j_per_kg_ieee_bits",
        "resulting_supply_temperature_c_ieee_bits",
    ] {
        assert_eq!(latest[field], predecessor_latest[field], "CP424 {field}");
    }
    let mut mutated = lifecycle.clone();
    mutated["latest"][MARKER] = json!(!latest[MARKER].as_bool().expect("CP424 marker"));
    let mut mutated_prefix = mutated["latest"]
        .as_object()
        .expect("mutated CP424 latest")
        .clone();
    let mut original_prefix = (*latest).clone();
    assert!(mutated_prefix.remove(MARKER).is_some());
    assert!(original_prefix.remove(MARKER).is_some());
    assert_eq!(
        mutated_prefix, original_prefix,
        "CP424 marker mutation must not change its CP423 prefix"
    );
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
    assert!(!results.to_string().contains(CP424_KEY));
    cp425_assertions::assert_direct(runtime, results);
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP424_KEY));
    assert!(
        runtime[CP424_KEY].is_null(),
        "non-direct runtime must not publish CP424 evidence"
    );
    cp425_assertions::assert_non_direct(runtime);
}

fn assert_schema_and_binding_cardinalities() {
    let core = include_str!(
        "../../../ep_runtime/src/ideal_loads/calc/cooling_supply_mass_flow_positive_guard_else_branch_entry.rs"
    );
    let snapshot = core
        .split_once(
            "pub struct PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardElseBranchEntrySnapshot",
        )
        .and_then(|(_, tail)| tail.split_once("/// Final selected-unit CP424"))
        .map(|(snapshot, _)| snapshot)
        .expect("CP424 snapshot declaration");
    assert_eq!(
        snapshot
            .lines()
            .filter(|line| line.trim_start().starts_with("pub "))
            .count(),
        263
    );
    assert_eq!(snapshot.matches("Option<f64>").count(), 94);
    assert_eq!(snapshot.matches("Option<bool>").count(), 2);
    assert_eq!(snapshot.matches("Option<").count() - 94 - 2, 1);
    let binding = include_str!("../../../ep_runtime/src/ideal_loads/binding/scheduled_output.rs");
    assert_eq!(binding.matches("    pub calculation_").count(), 125);
    let historical_audit = include_str!(
        "../../../../scripts/quality/ideal-loads-structure-audit/cp424-cooling-supply-mass-flow-positive-guard-else-branch-entry.ps1"
    );
    assert!(
        historical_audit
            .contains("non_direct_runtime_rejects_cp316_through_cp434_lifecycle_evidence")
    );
    assert!(
        !historical_audit
            .contains("non_direct_runtime_rejects_cp316_through_cp433_lifecycle_evidence")
    );
    let core_tests = include_str!(
        "../../../ep_runtime/src/ideal_loads/calc/cooling_supply_mass_flow_positive_guard_else_branch_entry/tests.rs"
    );
    for evidence in [
        "assert_eq!(predecessors.len(), 59)",
        "assert_eq!(state.inactive_transition_count, 58)",
        "assert_eq!(state.positive_supply_mass_flow_guard_else_branch_entry_count, 1)",
        "assert_eq!(state.source_site_execution_count, 1)",
        "assert_eq!(nonzero_indices(&entry_counts), [2])",
        "assert_eq!(state.cp423_supply_humidity_ratio_state_owner_count, 36)",
        "assert_eq!(state.cp423_supply_enthalpy_state_owner_count, 41)",
        "assert_eq!(state.cp423_supply_temperature_state_owner_count, 56)",
    ] {
        assert!(
            core_tests.contains(evidence),
            "CP424 exhaustive evidence: {evidence}"
        );
    }
    let schema = include_str!(
        "../../../ep_runtime/src/ideal_loads/calc/cooling_supply_mass_flow_positive_guard_else_branch_entry/tests/schema_prefix.rs"
    );
    assert!(schema.contains(
        "assert_eq!((public, private, public_entries, private_entries), (19, 40, 1, 0))"
    ));
}

fn canonical_cp423_keys() -> Vec<&'static str> {
    let cp420 = literal_keys(include_str!(
        "../../src/pipeline/purchased_air_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment/serialization/snapshot.rs"
    ));
    let cp421_tail = literal_keys(include_str!(
        "../../src/pipeline/purchased_air_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard/serialization/snapshot.rs"
    ));
    let cp422_tail = literal_keys(include_str!(
        "../../src/pipeline/purchased_air_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment/serialization/snapshot.rs"
    ));
    let cp423_tail = literal_keys(include_str!(
        "../../src/pipeline/purchased_air_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment/serialization/snapshot.rs"
    ));
    assert_eq!(
        (
            cp420.len(),
            cp421_tail.len(),
            cp422_tail.len(),
            cp423_tail.len()
        ),
        (273, 29, 27, 45)
    );
    let mut cp421 = cp420[..267].to_vec();
    cp421.extend(cp421_tail);
    let mut cp422 = cp421[..290].to_vec();
    cp422.extend(cp422_tail);
    let mut cp423 = cp422[..311].to_vec();
    cp423.extend(cp423_tail);
    cp423
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
    value.as_u64().expect("CP424 unsigned count")
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
