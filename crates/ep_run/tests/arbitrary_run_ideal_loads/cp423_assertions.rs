//! CP423 sensible-output supply-temperature assignment assertions.

#[path = "cp424_assertions.rs"]
mod cp424_assertions;

use serde_json::{Map, Value, json};

const CP422_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_lifecycle";
const CP423_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment_lifecycle";
const ORDER: [&str; 8] = [
    "read-purchased-air-mixed-air-temperature-for-post-saturation-capacity-limit-dehumidification-guard-else-branch-sensible-output-supply-temperature-difference-minuend",
    "read-local-cooling-sensible-output-for-post-saturation-capacity-limit-dehumidification-guard-else-branch-sensible-output-supply-temperature-quotient-numerator",
    "read-retained-supply-mass-flow-rate-for-post-saturation-capacity-limit-dehumidification-guard-else-branch-sensible-output-supply-temperature-denominator-first-factor",
    "read-retained-cp-air-for-post-saturation-capacity-limit-dehumidification-guard-else-branch-sensible-output-supply-temperature-denominator-second-factor",
    "calculate-supply-mass-flow-rate-times-cp-air-for-post-saturation-capacity-limit-dehumidification-guard-else-branch-sensible-output-supply-temperature-denominator",
    "calculate-cooling-sensible-output-divided-by-air-capacity-rate-for-post-saturation-capacity-limit-dehumidification-guard-else-branch-sensible-output-supply-temperature-drop",
    "calculate-mixed-air-temperature-minus-sensible-temperature-drop-for-post-saturation-capacity-limit-dehumidification-guard-else-branch-sensible-output-supply-temperature",
    "assign-purchased-air-supply-temperature-for-post-saturation-capacity-limit-dehumidification-guard-else-branch-sensible-output-case",
];
const PUBLIC: [usize; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 20, 21, 26, 27];
const ACTIVE: [usize; 5] = [4, 7, 10, 13, 16];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let lifecycle = &runtime[CP423_KEY];
    let predecessor = &runtime[CP422_KEY];
    assert_eq!(
        lifecycle["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2334"
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
    for field in [
        "predecessor_route_counts",
        "predecessor_guard_false_fallthrough_route_counts",
        "cooling_sensible_output_supply_temperature_assignment_route_counts",
    ] {
        let values = array(lifecycle, field);
        assert_eq!(values.len(), 36, "CP423 {field} width");
        for (index, value) in values.iter().enumerate() {
            if !PUBLIC.contains(&index) {
                assert_eq!(count_value(value), 0, "CP423 {field} route {index}");
            }
        }
    }
    assert_eq!(
        lifecycle["predecessor_route_counts"],
        predecessor["predecessor_route_counts"]
    );
    assert_eq!(
        lifecycle["predecessor_guard_false_fallthrough_route_counts"],
        predecessor["predecessor_guard_false_fallthrough_route_counts"]
    );
    assert_eq!(
        lifecycle["cooling_sensible_output_supply_temperature_assignment_route_counts"],
        predecessor["cooling_sensible_output_maximum_capacity_assignment_route_counts"]
    );
    let routes = array(lifecycle, "predecessor_route_counts");
    let false_routes = array(
        lifecycle,
        "predecessor_guard_false_fallthrough_route_counts",
    );
    let assignments = array(
        lifecycle,
        "cooling_sensible_output_supply_temperature_assignment_route_counts",
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
            "cooling_sensible_output_supply_temperature_assignment_count"
        ),
        assignment_count
    );
    assert_eq!(
        count(lifecycle, "source_site_execution_count"),
        assignment_count * 8
    );
    for field in [
        "cp423_sensible_output_supply_temperature_state_owner_count",
        "cp329_retained_mixed_air_temperature_for_sensible_output_supply_temperature_owned_read_count",
        "mixed_air_temperature_for_sensible_output_supply_temperature_read_count",
        "cp422_retained_cooling_sensible_output_owned_read_count",
        "cooling_sensible_output_for_supply_temperature_read_count",
        "cp330_retained_supply_mass_flow_rate_for_sensible_output_supply_temperature_owned_read_count",
        "cp329_supply_mass_flow_rate_for_sensible_output_supply_temperature_bit_corroboration_count",
        "supply_mass_flow_rate_for_sensible_output_supply_temperature_read_count",
        "cp419_retained_cp_air_for_sensible_output_supply_temperature_owned_read_count",
        "cp_air_for_sensible_output_supply_temperature_read_count",
        "supply_mass_flow_rate_times_cp_air_for_sensible_output_supply_temperature_calculation_count",
        "cooling_sensible_output_over_air_capacity_rate_calculation_count",
        "sensible_output_supply_temperature_calculation_count",
        "sensible_output_supply_temperature_assignment_write_count",
    ] {
        assert_eq!(count(lifecycle, field), assignment_count, "CP423 {field}");
    }
    let latest = &lifecycle["latest"];
    let predecessor_latest = &predecessor["latest"];
    for field in [
        "resulting_supply_humidity_ratio_ieee_bits",
        "resulting_supply_enthalpy_j_per_kg_ieee_bits",
    ] {
        assert_eq!(latest[field], predecessor_latest[field], "CP423 {field}");
    }
    let assignment = predecessor_latest["post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_executed"]
        .as_bool().expect("CP422 assignment marker");
    assert_eq!(
        latest["post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment_executed"],
        assignment
    );
    for flag in [
        "cp329_retained_mixed_air_temperature_for_sensible_output_supply_temperature_owned_read",
        "mixed_air_temperature_for_sensible_output_supply_temperature_read",
        "cp422_retained_cooling_sensible_output_owned_read",
        "cooling_sensible_output_for_supply_temperature_read",
        "cp330_retained_supply_mass_flow_rate_for_sensible_output_supply_temperature_owned_read",
        "cp329_supply_mass_flow_rate_for_sensible_output_supply_temperature_bit_corroborated",
        "supply_mass_flow_rate_for_sensible_output_supply_temperature_read",
        "cp419_retained_cp_air_for_sensible_output_supply_temperature_owned_read",
        "cp_air_for_sensible_output_supply_temperature_read",
        "supply_mass_flow_rate_times_cp_air_for_sensible_output_supply_temperature_calculated",
        "cooling_sensible_output_over_air_capacity_rate_calculated",
        "sensible_output_supply_temperature_calculated",
        "sensible_output_supply_temperature_assignment_performed",
    ] {
        assert_eq!(latest[flag], assignment, "CP423 {flag}");
    }
    if assignment {
        let mixed = number(
            latest,
            "mixed_air_temperature_for_sensible_output_supply_temperature_c",
        );
        let cooling = number(latest, "cooling_sensible_output_for_supply_temperature_w");
        let flow = number(
            latest,
            "supply_mass_flow_rate_for_sensible_output_supply_temperature_kg_per_s",
        );
        let cp_air = number(
            latest,
            "cp_air_for_sensible_output_supply_temperature_j_per_kg_k",
        );
        let capacity_rate = flow * cp_air;
        let drop = cooling / capacity_rate;
        let calculated = mixed - drop;
        assert_bits(
            latest,
            "supply_mass_flow_rate_times_cp_air_for_sensible_output_supply_temperature_w_per_k",
            capacity_rate,
        );
        assert_bits(
            latest,
            "cooling_sensible_output_over_air_capacity_rate_k",
            drop,
        );
        assert_bits(
            latest,
            "calculated_sensible_output_supply_temperature_c",
            calculated,
        );
        assert_bits(
            latest,
            "assigned_sensible_output_supply_temperature_c",
            calculated,
        );
        assert_bits(latest, "resulting_supply_temperature_c", calculated);
    } else {
        for field in [
            "mixed_air_temperature_for_sensible_output_supply_temperature_c",
            "cooling_sensible_output_for_supply_temperature_w",
            "supply_mass_flow_rate_for_sensible_output_supply_temperature_kg_per_s",
            "cp_air_for_sensible_output_supply_temperature_j_per_kg_k",
            "supply_mass_flow_rate_times_cp_air_for_sensible_output_supply_temperature_w_per_k",
            "cooling_sensible_output_over_air_capacity_rate_k",
            "calculated_sensible_output_supply_temperature_c",
            "assigned_sensible_output_supply_temperature_c",
        ] {
            assert!(latest[field].is_null(), "CP423 {field}");
        }
        assert_eq!(
            latest["resulting_supply_temperature_c_ieee_bits"],
            predecessor_latest["resulting_supply_temperature_c_ieee_bits"]
        );
    }
    let object = latest.as_object().expect("CP423 latest object");
    assert_eq!(object.len(), 356);
    assert_eq!(
        object
            .keys()
            .filter(|key| key.ends_with("_ieee_bits"))
            .count(),
        94
    );
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
    assert!(!results.to_string().contains(CP423_KEY));
    cp424_assertions::assert_direct(runtime, results);
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP423_KEY));
    assert!(
        runtime[CP423_KEY].is_null(),
        "non-direct runtime must not publish CP423 evidence"
    );
    cp424_assertions::assert_non_direct(runtime);
}

fn count(value: &Value, field: &str) -> u64 {
    count_value(&value[field])
}
fn count_value(value: &Value) -> u64 {
    value.as_u64().expect("CP423 unsigned count")
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
fn number(value: &Value, field: &str) -> f64 {
    value[field].as_f64().expect("CP423 finite number")
}
fn assert_bits(value: &Value, field: &str, expected: f64) {
    assert_eq!(
        value[format!("{field}_ieee_bits")],
        format!("0x{:016x}", expected.to_bits()),
        "CP423 {field}"
    );
}
