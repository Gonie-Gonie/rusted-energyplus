//! CP413 saturation humidity-ratio guard assertions.

use serde_json::{Map, Value, json};

const CP412_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_lifecycle";
const CP413_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_lifecycle";
const ORDER: [&str; 4] = [
    "read-local-saturation-supply-humidity-ratio-for-saturation-guard",
    "read-local-original-supply-humidity-ratio-for-saturation-guard",
    "compare-local-saturation-supply-humidity-ratio-strictly-less-than-local-original-supply-humidity-ratio",
    "enter-saturation-supply-humidity-ratio-guard-body-if-comparison-satisfied",
];
const SPLIT_PREDECESSOR_INDICES: [usize; 6] = [20, 21, 24, 25, 27, 29];
const PUBLIC_LOGICAL_INDICES: [usize; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 20, 21, 26, 27];
const ROUTE_FIELDS: [&str; 3] = [
    "predecessor_route_counts",
    "guard_false_fallthrough_route_counts",
    "guard_body_entry_route_counts",
];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let predecessor = &runtime[CP412_KEY];
    let lifecycle = &runtime[CP413_KEY];
    assert_eq!(
        lifecycle["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2315",
    );
    assert_eq!(
        lifecycle["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2316",
    );
    assert_eq!(lifecycle["latest"]["source_order"], json!(ORDER));
    assert_eq!(lifecycle["system"], predecessor["system"]);
    assert_eq!(
        lifecycle["transition_count"],
        predecessor["transition_count"],
    );

    for field in ROUTE_FIELDS {
        assert_eq!(array(lifecycle, field).len(), 36, "CP413 {field} width");
    }
    let flattened_routes = flattened_predecessor_routes(predecessor);
    assert_eq!(
        flattened_routes.as_slice(),
        array(lifecycle, "predecessor_route_counts"),
    );
    assert_route_contract(lifecycle);
    assert_public_route_firewall(lifecycle);
    assert_conceptual_contract();

    let routes = array(lifecycle, "predecessor_route_counts");
    let guard_false = array(lifecycle, "guard_false_fallthrough_route_counts");
    let body = array(lifecycle, "guard_body_entry_route_counts");
    let predecessor_routes = array(predecessor, "predecessor_route_counts");
    let transitions = sum(routes);
    let evaluations = sum(&routes[18..]);
    let false_fallthroughs = sum(guard_false);
    let body_entries = sum(body);
    let enthalpy_owners = predecessor_routes
        .iter()
        .enumerate()
        .filter(|(index, _)| matches!(*index, 5 | 8 | 11 | 14 | 17..=29))
        .map(|(_, value)| count_value(value))
        .sum::<u64>();
    let temperature_owners = sum(&predecessor_routes[3..]);
    assert_eq!(count(lifecycle, "transition_count"), transitions);
    assert_eq!(
        count(lifecycle, "inactive_transition_count") + evaluations,
        transitions
    );
    assert_eq!(false_fallthroughs + body_entries, evaluations);
    assert_eq!(
        count(
            lifecycle,
            "saturation_supply_humidity_ratio_guard_evaluation_count"
        ),
        evaluations,
    );
    assert_eq!(
        count(lifecycle, "source_site_execution_count"),
        evaluations * 3 + body_entries,
    );
    for field in [
        "cp412_supply_humidity_ratio_state_owner_count",
        "unchanged_supply_humidity_ratio_preservation_count",
    ] {
        assert_eq!(count(lifecycle, field), evaluations, "CP413 {field}");
    }
    for field in [
        "cp412_supply_enthalpy_state_owner_count",
        "unchanged_supply_enthalpy_preservation_count",
    ] {
        assert_eq!(count(lifecycle, field), enthalpy_owners, "CP413 {field}");
    }
    for field in [
        "cp412_supply_temperature_state_owner_count",
        "unchanged_supply_temperature_preservation_count",
    ] {
        assert_eq!(count(lifecycle, field), temperature_owners, "CP413 {field}");
    }
    for field in [
        "cp412_saturation_supply_humidity_ratio_owned_read_count",
        "saturation_supply_humidity_ratio_for_guard_read_count",
        "cp411_original_supply_humidity_ratio_owned_read_count",
        "cp412_same_call_original_supply_humidity_ratio_bit_corroboration_count",
        "original_supply_humidity_ratio_for_guard_read_count",
        "saturation_original_supply_humidity_ratio_comparison_count",
    ] {
        assert_eq!(count(lifecycle, field), evaluations, "CP413 {field}");
    }
    for field in [
        "saturation_supply_humidity_ratio_strictly_less_than_original_supply_humidity_ratio_count",
        "saturation_supply_humidity_ratio_guard_body_entry_count",
    ] {
        assert_eq!(count(lifecycle, field), body_entries, "CP413 {field}");
    }
    assert_eq!(
        count(
            lifecycle,
            "saturation_supply_humidity_ratio_guard_false_fallthrough_count"
        ),
        false_fallthroughs,
    );

    assert_latest_lineage(&lifecycle["latest"], &predecessor["latest"], transitions);

    let serialized = lifecycle.to_string().to_ascii_lowercase();
    for forbidden in ["supply_node", "zone_load", "report", "numerical_dto"] {
        assert!(!serialized.contains(forbidden), "CP413 forbids {forbidden}");
    }
    assert!(
        !results.to_string().contains(CP413_KEY),
        "CP413 lifecycle must remain outside numerical result state",
    );
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP413_KEY));
    assert!(
        runtime[CP413_KEY].is_null(),
        "non-direct runtime must not publish CP413 evidence",
    );
}

fn assert_route_contract(lifecycle: &Value) {
    let routes = array(lifecycle, "predecessor_route_counts");
    let guard_false = array(lifecycle, "guard_false_fallthrough_route_counts");
    let body = array(lifecycle, "guard_body_entry_route_counts");
    for index in 0..36 {
        let outcomes = count_value(&guard_false[index]).checked_add(count_value(&body[index]));
        assert_eq!(
            outcomes,
            Some(if index >= 18 {
                count_value(&routes[index])
            } else {
                0
            }),
            "CP413 route outcome partition {index}",
        );
    }
}

fn assert_public_route_firewall(lifecycle: &Value) {
    for field in ROUTE_FIELDS {
        for (index, value) in array(lifecycle, field).iter().enumerate() {
            if !PUBLIC_LOGICAL_INDICES.contains(&index) {
                assert_eq!(count_value(value), 0, "public CP413 {field} route {index}",);
            }
        }
    }
}

fn assert_conceptual_contract() {
    let conceptual_outcomes = 36 + 18;
    let public_outcomes = PUBLIC_LOGICAL_INDICES.len() + 4;
    let private_outcomes = conceptual_outcomes - public_outcomes;
    let source_sites = 18 * 3 + 18 * 4;
    let route_width = |index: usize| 1 + usize::from(SPLIT_PREDECESSOR_INDICES.contains(&index));
    let active_outcomes = (18..30).map(route_width).sum::<usize>() * 2;
    let inactive_enthalpy = [5, 8, 11, 14, 17]
        .into_iter()
        .map(route_width)
        .sum::<usize>();
    let inactive_temperature = (3..18).map(route_width).sum::<usize>();
    assert_eq!(
        (
            conceptual_outcomes,
            public_outcomes,
            private_outcomes,
            source_sites
        ),
        (54, 17, 37, 126),
    );
    assert_eq!(
        (
            active_outcomes,
            inactive_enthalpy + active_outcomes,
            inactive_temperature + active_outcomes,
        ),
        (36, 41, 51),
        "CP413 W/H/T conceptual presence",
    );
}

fn assert_latest_lineage(latest: &Value, predecessor: &Value, transitions: u64) {
    assert_eq!(
        latest["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2315",
    );
    assert_eq!(
        latest["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2316",
    );
    assert_eq!(latest["source_order"], json!(ORDER));
    assert_eq!(latest["parent_call_ordinal"], transitions);
    for (field, value) in predecessor
        .as_object()
        .expect("CP412 latest must be an object")
    {
        if !matches!(
            field.as_str(),
            "source" | "first_excluded_source" | "source_order"
        ) {
            assert_eq!(latest.get(field), Some(value), "CP413 {field} lineage");
        }
    }
    for (next, previous) in [
        (
            "predecessor_cp412_resulting_supply_humidity_ratio",
            "resulting_supply_humidity_ratio",
        ),
        (
            "predecessor_cp412_resulting_supply_enthalpy_j_per_kg",
            "resulting_supply_enthalpy_j_per_kg",
        ),
        (
            "predecessor_cp412_resulting_supply_temperature_c",
            "resulting_supply_temperature_c",
        ),
    ] {
        assert_same_bits(latest, predecessor, next, previous);
    }

    let active = boolean(
        predecessor,
        "post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_executed",
    );
    for field in [
        "post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_evaluated",
        "cp412_saturation_supply_humidity_ratio_owned_read",
        "saturation_supply_humidity_ratio_for_guard_read",
        "cp411_original_supply_humidity_ratio_owned_read",
        "cp412_same_call_original_supply_humidity_ratio_bit_corroborated",
        "original_supply_humidity_ratio_for_guard_read",
        "saturation_original_supply_humidity_ratio_comparison_evaluated",
    ] {
        assert_eq!(latest[field], active, "CP413 {field}");
    }
    if active {
        assert_same_bits(
            latest,
            predecessor,
            "saturation_supply_humidity_ratio_for_guard",
            "resulting_saturation_supply_humidity_ratio",
        );
        assert_same_bits(
            latest,
            predecessor,
            "original_supply_humidity_ratio_for_guard",
            "resulting_supply_humidity_ratio_original",
        );
        assert_same_bits(
            predecessor,
            predecessor,
            "resulting_supply_humidity_ratio_original",
            "predecessor_cp411_resulting_supply_humidity_ratio",
        );
        let comparison = number(latest, "saturation_supply_humidity_ratio_for_guard")
            < number(latest, "original_supply_humidity_ratio_for_guard");
        assert_eq!(
            latest["saturation_supply_humidity_ratio_strictly_less_than_original_supply_humidity_ratio"],
            comparison,
        );
        assert_eq!(
            latest["saturation_supply_humidity_ratio_guard_body_entered"],
            comparison
        );
        assert_eq!(
            latest["saturation_supply_humidity_ratio_guard_false_fallthrough"],
            !comparison,
        );
    } else {
        for field in [
            "saturation_supply_humidity_ratio_for_guard",
            "original_supply_humidity_ratio_for_guard",
        ] {
            assert!(latest[field].is_null(), "CP413 inactive {field}");
            assert!(
                latest[format!("{field}_ieee_bits")].is_null(),
                "CP413 inactive {field} bits",
            );
        }
        assert!(
            latest["saturation_supply_humidity_ratio_strictly_less_than_original_supply_humidity_ratio"]
                .is_null(),
        );
        assert_eq!(
            latest["saturation_supply_humidity_ratio_guard_body_entered"],
            false
        );
        assert_eq!(
            latest["saturation_supply_humidity_ratio_guard_false_fallthrough"],
            false,
        );
    }
    for (owner, predecessor_field) in [
        (
            "cp412_retained_supply_humidity_ratio_state_owned",
            "resulting_supply_humidity_ratio",
        ),
        (
            "cp412_retained_supply_enthalpy_state_owned",
            "resulting_supply_enthalpy_j_per_kg",
        ),
        (
            "cp412_retained_supply_temperature_state_owned",
            "resulting_supply_temperature_c",
        ),
    ] {
        assert_eq!(
            latest[owner],
            ieee_bits(predecessor, predecessor_field).is_some(),
            "CP413 {owner}",
        );
    }
    assert_eq!(latest.as_object().map(|object| object.len()), Some(120));
    assert_eq!(
        latest.as_object().map(|object| {
            object
                .keys()
                .filter(|key| key.ends_with("_ieee_bits"))
                .count()
        }),
        Some(25),
    );
}

fn flattened_predecessor_routes(predecessor: &Value) -> Vec<Value> {
    let routes = array(predecessor, "predecessor_route_counts");
    let guard_false = array(
        predecessor,
        "predecessor_guard_false_fallthrough_route_counts",
    );
    let maximum = array(
        predecessor,
        "predecessor_maximum_capacity_assignment_route_counts",
    );
    let mut flattened = Vec::with_capacity(36);
    for index in 0..30 {
        if SPLIT_PREDECESSOR_INDICES.contains(&index) {
            assert_eq!(
                count_value(&guard_false[index]).checked_add(count_value(&maximum[index])),
                Some(count_value(&routes[index])),
                "CP413 predecessor split {index}",
            );
            flattened.push(guard_false[index].clone());
            flattened.push(maximum[index].clone());
        } else {
            flattened.push(routes[index].clone());
        }
    }
    assert_eq!(flattened.len(), 36);
    flattened
}

fn assert_same_bits(next: &Value, previous: &Value, next_field: &str, previous_field: &str) {
    assert_eq!(
        ieee_bits(next, next_field),
        ieee_bits(previous, previous_field),
        "CP413 {next_field} lineage",
    );
}

fn ieee_bits(value: &Value, field: &str) -> Option<u64> {
    let sidecar = &value[format!("{field}_ieee_bits")];
    if sidecar.is_null() {
        return None;
    }
    let text = sidecar
        .as_str()
        .expect("CP413 IEEE sidecar must be a string");
    let hex = text
        .strip_prefix("0x")
        .expect("CP413 IEEE sidecar must use a hexadecimal prefix");
    Some(u64::from_str_radix(hex, 16).expect("CP413 IEEE sidecar must contain binary64 bits"))
}

fn number(value: &Value, field: &str) -> f64 {
    value[field]
        .as_f64()
        .expect("CP413 value must be a finite JSON number")
}

fn boolean(value: &Value, field: &str) -> bool {
    value[field]
        .as_bool()
        .expect("CP413 value must be a boolean")
}

fn count(value: &Value, field: &str) -> u64 {
    count_value(&value[field])
}

fn count_value(value: &Value) -> u64 {
    value
        .as_u64()
        .expect("CP413 value must be an unsigned count")
}

fn array<'a>(value: &'a Value, field: &str) -> &'a [Value] {
    match value[field].as_array() {
        Some(values) => values.as_slice(),
        None => &[],
    }
}

fn sum(values: &[Value]) -> u64 {
    values.iter().map(count_value).sum()
}
