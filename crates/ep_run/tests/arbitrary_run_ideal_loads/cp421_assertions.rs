//! CP421 post-saturation sensible-output capacity-guard assertions.

use serde_json::{Map, Value, json};

#[path = "cp422_assertions.rs"]
mod cp422_assertions;

const CP321_KEY: &str = "purchased_air_calc_cooling_capacity_zero_flow_reset_lifecycle";
const CP340_KEY: &str =
    "purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_guard_lifecycle";
const CP420_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_lifecycle";
const CP421_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_lifecycle";
const ORDER: [&str; 4] = [
    "read-retained-cooling-sensible-output-for-post-saturation-capacity-limit-dehumidification-guard-else-branch-maximum-capacity-comparison",
    "read-retained-maximum-total-cooling-capacity-for-post-saturation-capacity-limit-dehumidification-guard-else-branch-sensible-output-comparison",
    "compare-post-saturation-capacity-limit-dehumidification-guard-else-branch-cooling-sensible-output-greater-than-or-equal-to-maximum-total-cooling-capacity",
    "enter-post-saturation-capacity-limit-dehumidification-guard-else-branch-sensible-output-capacity-adjustment-body-if-comparison-satisfied",
];
const PUBLIC: [usize; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 20, 21, 26, 27];
const ACTIVE: [usize; 5] = [4, 7, 10, 13, 16];
const ROUTE_FIELDS: [&str; 3] = [
    "predecessor_route_counts",
    "guard_false_fallthrough_route_counts",
    "adjustment_body_entry_route_counts",
];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let lifecycle = &runtime[CP421_KEY];
    let predecessor = &runtime[CP420_KEY];
    let capacity_owner = &runtime[CP321_KEY];
    let capacity_corroborator = &runtime[CP340_KEY];
    assert_eq!(
        lifecycle["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2332"
    );
    assert_eq!(lifecycle["latest"]["source_order"], json!(ORDER));
    assert_eq!(lifecycle["system"], predecessor["system"]);
    assert_eq!(lifecycle["system"], capacity_owner["system"]);
    assert_eq!(lifecycle["system"], capacity_corroborator["system"]);
    assert_eq!(
        lifecycle["transition_count"],
        predecessor["transition_count"]
    );

    for field in ROUTE_FIELDS {
        assert_eq!(array(lifecycle, field).len(), 36, "CP421 {field} width");
        for (index, value) in array(lifecycle, field).iter().enumerate() {
            if !PUBLIC.contains(&index) {
                assert_eq!(count_value(value), 0, "public CP421 {field} route {index}");
            }
        }
    }
    assert_eq!(
        lifecycle["predecessor_route_counts"],
        predecessor["predecessor_route_counts"]
    );
    let routes = array(lifecycle, "predecessor_route_counts");
    let false_routes = array(lifecycle, "guard_false_fallthrough_route_counts");
    let body_routes = array(lifecycle, "adjustment_body_entry_route_counts");
    for index in 0..36 {
        let expected = if ACTIVE.contains(&index) {
            count_value(&routes[index])
        } else {
            0
        };
        assert_eq!(
            count_value(&false_routes[index]) + count_value(&body_routes[index]),
            expected,
            "CP421 route {index}"
        );
    }
    let transitions = sum(routes);
    let false_count = sum(false_routes);
    let body_count = sum(body_routes);
    let evaluations = false_count + body_count;
    assert_eq!(count(lifecycle, "transition_count"), transitions);
    assert_eq!(
        count(lifecycle, "inactive_transition_count") + evaluations,
        transitions
    );
    assert_eq!(
        count(
            lifecycle,
            "post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_guard_evaluation_count"
        ),
        evaluations
    );
    assert_eq!(
        count(
            lifecycle,
            "post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_guard_false_fallthrough_count"
        ),
        false_count
    );
    assert_eq!(
        count(
            lifecycle,
            "post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_adjustment_body_entry_count"
        ),
        body_count
    );
    assert_eq!(
        count(lifecycle, "source_site_execution_count"),
        evaluations * 3 + body_count
    );
    for field in [
        "cp420_cooling_sensible_output_owned_read_count",
        "cooling_sensible_output_read_count",
        "cp321_maximum_total_cooling_capacity_owned_read_count",
        "cp340_same_call_maximum_total_cooling_capacity_bit_corroboration_count",
        "maximum_total_cooling_capacity_read_count",
        "cooling_sensible_output_maximum_total_cooling_capacity_comparison_count",
    ] {
        assert_eq!(count(lifecycle, field), evaluations, "CP421 {field}");
    }
    assert_eq!(
        count(
            lifecycle,
            "cooling_sensible_output_greater_than_or_equal_to_maximum_total_cooling_capacity_count"
        ),
        body_count
    );
    let (conceptual, inactive, false_paths, body_paths, public, private) =
        (59usize, 49usize, 5usize, 5usize, 19usize, 40usize);
    assert_eq!(conceptual, inactive + false_paths + body_paths);
    assert_eq!(conceptual, public + private);

    let latest = &lifecycle["latest"];
    let predecessor_latest = &predecessor["latest"];
    for field in [
        "resulting_supply_humidity_ratio_ieee_bits",
        "resulting_supply_enthalpy_j_per_kg_ieee_bits",
        "resulting_supply_temperature_c_ieee_bits",
    ] {
        assert_eq!(latest[field], predecessor_latest[field], "CP421 {field}");
    }
    let active = latest["post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_guard_evaluated"]
        .as_bool()
        .expect("CP421 evaluated marker");
    for flag in [
        "cp420_retained_cooling_sensible_output_owned_read",
        "cooling_sensible_output_read",
        "cp321_maximum_total_cooling_capacity_owned_read",
        "cp340_same_call_maximum_total_cooling_capacity_bit_corroborated",
        "maximum_total_cooling_capacity_read",
        "cooling_sensible_output_maximum_total_cooling_capacity_comparison_evaluated",
    ] {
        assert_eq!(latest[flag], active, "CP421 {flag}");
    }
    if active {
        let cooling = bits(latest, "cp420_cooling_sensible_output_for_capacity_guard_w");
        let maximum = bits(latest, "maximum_total_cooling_capacity_w");
        assert_eq!(
            cooling,
            bits(predecessor_latest, "cooling_sensible_output_w")
        );
        assert_eq!(
            maximum,
            capacity_owner["latest"]["maximum_total_cooling_capacity_w"]
                .as_f64()
                .expect("CP421 active CP321 capacity")
                .to_bits()
        );
        assert_eq!(
            maximum,
            bits(
                &capacity_corroborator["latest"],
                "maximum_total_cooling_capacity_w"
            )
        );
        let expected = f64::from_bits(cooling) >= f64::from_bits(maximum);
        assert_eq!(
            latest["cooling_sensible_output_greater_than_or_equal_to_maximum_total_cooling_capacity"],
            expected
        );
        assert_eq!(
            latest["post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_adjustment_body_entered"],
            expected
        );
        assert_eq!(
            latest["post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_guard_false_fallthrough"],
            !expected
        );
    } else {
        assert!(latest["cp420_cooling_sensible_output_for_capacity_guard_w"].is_null());
        assert!(latest["maximum_total_cooling_capacity_w"].is_null());
        assert!(latest["cooling_sensible_output_greater_than_or_equal_to_maximum_total_cooling_capacity"].is_null());
    }
    let object = latest.as_object().expect("CP421 latest object");
    assert_eq!(object.len(), 296);
    assert_eq!(
        object
            .keys()
            .filter(|key| key.ends_with("_ieee_bits"))
            .count(),
        76
    );
    for forbidden in [
        "numerical_dto",
        "direct_zone_purchased_air_coupling_input",
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
    assert!(!results.to_string().contains(CP421_KEY));
    cp422_assertions::assert_direct(runtime, results);
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP421_KEY));
    assert!(
        runtime[CP421_KEY].is_null(),
        "non-direct runtime must not publish CP421 evidence"
    );
    cp422_assertions::assert_non_direct(runtime);
}

fn bits(value: &Value, field: &str) -> u64 {
    let bits = value[format!("{field}_ieee_bits")]
        .as_str()
        .expect("CP421 IEEE sidecar");
    u64::from_str_radix(bits.trim_start_matches("0x"), 16).expect("CP421 IEEE bits")
}
fn count(value: &Value, field: &str) -> u64 {
    count_value(&value[field])
}
fn count_value(value: &Value) -> u64 {
    value.as_u64().expect("CP421 unsigned count")
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
