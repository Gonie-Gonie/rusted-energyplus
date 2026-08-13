//! CP422 sensible-output maximum-capacity assignment assertions.

use serde_json::{Map, Value, json};

#[path = "cp423_assertions.rs"]
mod cp423_assertions;

const CP421_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_lifecycle";
const CP422_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_lifecycle";
const ORDER: [&str; 2] = [
    "read-retained-maximum-total-cooling-capacity-for-post-saturation-capacity-limit-dehumidification-guard-else-branch-sensible-output-assignment",
    "assign-local-cooling-sensible-output-from-maximum-total-cooling-capacity",
];
const PUBLIC: [usize; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 20, 21, 26, 27];
const ACTIVE: [usize; 5] = [4, 7, 10, 13, 16];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let lifecycle = &runtime[CP422_KEY];
    let predecessor = &runtime[CP421_KEY];
    assert_eq!(
        lifecycle["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2333"
    );
    assert_eq!(lifecycle["latest"]["source_order"], json!(ORDER));
    assert_eq!(lifecycle["system"], predecessor["system"]);
    assert_eq!(
        lifecycle["transition_count"],
        predecessor["transition_count"]
    );
    for field in [
        "predecessor_route_counts",
        "predecessor_guard_false_fallthrough_route_counts",
        "cooling_sensible_output_maximum_capacity_assignment_route_counts",
    ] {
        let values = array(lifecycle, field);
        assert_eq!(values.len(), 36, "CP422 {field} width");
        for (index, value) in values.iter().enumerate() {
            if !PUBLIC.contains(&index) {
                assert_eq!(count_value(value), 0, "CP422 {field} route {index}");
            }
        }
    }
    assert_eq!(
        lifecycle["predecessor_route_counts"],
        predecessor["predecessor_route_counts"]
    );
    assert_eq!(
        lifecycle["predecessor_guard_false_fallthrough_route_counts"],
        predecessor["guard_false_fallthrough_route_counts"]
    );
    assert_eq!(
        lifecycle["cooling_sensible_output_maximum_capacity_assignment_route_counts"],
        predecessor["adjustment_body_entry_route_counts"]
    );
    let routes = array(lifecycle, "predecessor_route_counts");
    let false_routes = array(
        lifecycle,
        "predecessor_guard_false_fallthrough_route_counts",
    );
    let assignments = array(
        lifecycle,
        "cooling_sensible_output_maximum_capacity_assignment_route_counts",
    );
    for index in 0..36 {
        let expected = if ACTIVE.contains(&index) {
            count_value(&routes[index])
        } else {
            0
        };
        assert_eq!(
            count_value(&false_routes[index]) + count_value(&assignments[index]),
            expected
        );
    }
    let transitions = sum(routes);
    let false_count = sum(false_routes);
    let assignment_count = sum(assignments);
    assert_eq!(count(lifecycle, "transition_count"), transitions);
    assert_eq!(
        count(lifecycle, "inactive_transition_count") + false_count + assignment_count,
        transitions
    );
    assert_eq!(
        count(lifecycle, "predecessor_guard_false_fallthrough_count"),
        false_count
    );
    assert_eq!(
        count(
            lifecycle,
            "cooling_sensible_output_maximum_capacity_assignment_count"
        ),
        assignment_count
    );
    assert_eq!(
        count(lifecycle, "source_site_execution_count"),
        assignment_count * 2
    );
    for field in [
        "cp421_retained_maximum_total_cooling_capacity_owned_read_count",
        "maximum_total_cooling_capacity_for_sensible_output_assignment_read_count",
        "cooling_sensible_output_maximum_capacity_assignment_write_count",
    ] {
        assert_eq!(count(lifecycle, field), assignment_count, "CP422 {field}");
    }
    for (owner, preserved) in [
        (
            "cp421_supply_humidity_ratio_state_owner_count",
            "unchanged_supply_humidity_ratio_preservation_count",
        ),
        (
            "cp421_supply_enthalpy_state_owner_count",
            "unchanged_supply_enthalpy_preservation_count",
        ),
        (
            "cp421_supply_temperature_state_owner_count",
            "unchanged_supply_temperature_preservation_count",
        ),
    ] {
        assert_eq!(lifecycle[owner], lifecycle[preserved]);
    }
    let latest = &lifecycle["latest"];
    let predecessor_latest = &predecessor["latest"];
    for field in [
        "resulting_supply_humidity_ratio_ieee_bits",
        "resulting_supply_enthalpy_j_per_kg_ieee_bits",
        "resulting_supply_temperature_c_ieee_bits",
    ] {
        assert_eq!(latest[field], predecessor_latest[field], "CP422 {field}");
    }
    let active = predecessor_latest["post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_guard_evaluated"]
        .as_bool().expect("CP421 evaluated marker");
    let assignment = predecessor_latest["post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_adjustment_body_entered"]
        .as_bool().expect("CP421 assignment marker");
    assert_eq!(
        latest["post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_executed"],
        assignment
    );
    for flag in [
        "cp421_retained_maximum_total_cooling_capacity_owned_read",
        "maximum_total_cooling_capacity_for_sensible_output_assignment_read",
        "cooling_sensible_output_maximum_capacity_assignment_performed",
    ] {
        assert_eq!(latest[flag], assignment, "CP422 {flag}");
    }
    if !active {
        assert!(
            latest["preexisting_cooling_sensible_output_for_maximum_capacity_assignment_w"]
                .is_null()
        );
        assert!(
            latest["resulting_cooling_sensible_output_after_maximum_capacity_assignment_w"]
                .is_null()
        );
    } else if assignment {
        let maximum =
            &latest["maximum_total_cooling_capacity_for_sensible_output_assignment_w_ieee_bits"];
        assert_eq!(
            &latest["assigned_cooling_sensible_output_from_maximum_capacity_w_ieee_bits"],
            maximum
        );
        assert_eq!(
            &latest["resulting_cooling_sensible_output_after_maximum_capacity_assignment_w_ieee_bits"],
            maximum
        );
    } else {
        assert_eq!(
            latest["resulting_cooling_sensible_output_after_maximum_capacity_assignment_w_ieee_bits"],
            latest["preexisting_cooling_sensible_output_for_maximum_capacity_assignment_w_ieee_bits"]
        );
    }
    let object = latest.as_object().expect("CP422 latest object");
    assert_eq!(object.len(), 317);
    assert_eq!(
        object
            .keys()
            .filter(|key| key.ends_with("_ieee_bits"))
            .count(),
        83
    );
    let (conceptual, inactive, false_paths, assignment_paths) = (59usize, 49usize, 5usize, 5usize);
    assert_eq!(conceptual, inactive + false_paths + assignment_paths);
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
    assert!(!results.to_string().contains(CP422_KEY));
    cp423_assertions::assert_direct(runtime, results);
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP422_KEY));
    assert!(
        runtime[CP422_KEY].is_null(),
        "non-direct runtime must not publish CP422 evidence"
    );
    cp423_assertions::assert_non_direct(runtime);
}

fn count(value: &Value, field: &str) -> u64 {
    count_value(&value[field])
}
fn count_value(value: &Value) -> u64 {
    value.as_u64().expect("CP422 unsigned count")
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
