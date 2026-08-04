//! CP415 saturation-temperature mixed-air-limit assertions.

use serde_json::{Map, Value, json};

const CP329_KEY: &str = "purchased_air_calc_cooling_mixed_air_call_lifecycle";
const CP414_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_lifecycle";
const CP415_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_lifecycle";
const ORDER: [&str; 4] = [
    "read-purchased-air-supply-temperature-for-minimum",
    "read-purchased-air-mixed-air-temperature-for-minimum",
    "apply-source-shaped-two-argument-minimum",
    "assign-purchased-air-supply-temperature",
];
const PUBLIC_LOGICAL_INDICES: [usize; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 20, 21, 26, 27];
const ROUTE_FIELDS: [&str; 5] = [
    "predecessor_route_counts",
    "predecessor_guard_false_fallthrough_route_counts",
    "predecessor_guard_body_entry_route_counts",
    "predecessor_supply_temperature_saturation_assignment_route_counts",
    "supply_temperature_mixed_air_limit_route_counts",
];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let mixed_air_owner = &runtime[CP329_KEY];
    let predecessor = &runtime[CP414_KEY];
    let lifecycle = &runtime[CP415_KEY];
    assert_eq!(
        lifecycle["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2319",
    );
    assert_eq!(
        lifecycle["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2320",
    );
    assert_eq!(lifecycle["latest"]["source_order"], json!(ORDER));
    assert_eq!(lifecycle["system"], predecessor["system"]);
    assert_eq!(lifecycle["system"], mixed_air_owner["system"]);
    assert_eq!(
        lifecycle["transition_count"],
        predecessor["transition_count"]
    );
    assert_eq!(
        lifecycle["transition_count"],
        mixed_air_owner["transition_count"]
    );

    for field in ROUTE_FIELDS {
        assert_eq!(array(lifecycle, field).len(), 36, "CP415 {field} width");
    }
    for (next, previous) in [
        ("predecessor_route_counts", "predecessor_route_counts"),
        (
            "predecessor_guard_false_fallthrough_route_counts",
            "predecessor_guard_false_fallthrough_route_counts",
        ),
        (
            "predecessor_guard_body_entry_route_counts",
            "predecessor_guard_body_entry_route_counts",
        ),
        (
            "predecessor_supply_temperature_saturation_assignment_route_counts",
            "supply_temperature_saturation_assignment_route_counts",
        ),
        (
            "supply_temperature_mixed_air_limit_route_counts",
            "supply_temperature_saturation_assignment_route_counts",
        ),
    ] {
        assert_eq!(lifecycle[next], predecessor[previous], "CP415 {next}");
    }
    assert_route_contract(lifecycle);
    assert_public_route_firewall(lifecycle);
    assert_conceptual_contract();

    let transitions = sum(array(lifecycle, "predecessor_route_counts"));
    let assignments = sum(array(
        lifecycle,
        "supply_temperature_mixed_air_limit_route_counts",
    ));
    assert_eq!(count(lifecycle, "transition_count"), transitions);
    assert_eq!(
        count(lifecycle, "inactive_transition_count") + assignments,
        transitions,
    );
    assert_eq!(
        count(
            lifecycle,
            "predecessor_supply_temperature_saturation_assignment_count"
        ),
        count(
            predecessor,
            "saturation_supply_temperature_assignment_count"
        ),
    );
    assert_eq!(
        count(
            lifecycle,
            "supply_temperature_saturation_mixed_air_limit_count"
        ),
        assignments,
    );
    assert_eq!(
        count(lifecycle, "source_site_execution_count"),
        assignments * ORDER.len() as u64,
    );
    for (next, previous) in [
        (
            "cp414_supply_humidity_ratio_state_owner_count",
            "cp413_supply_humidity_ratio_state_owner_count",
        ),
        (
            "unchanged_supply_humidity_ratio_preservation_count",
            "unchanged_supply_humidity_ratio_preservation_count",
        ),
        (
            "cp414_supply_enthalpy_state_owner_count",
            "cp413_supply_enthalpy_state_owner_count",
        ),
        (
            "unchanged_supply_enthalpy_preservation_count",
            "unchanged_supply_enthalpy_preservation_count",
        ),
        (
            "cp414_supply_temperature_state_owner_count",
            "cp413_supply_temperature_state_owner_count",
        ),
    ] {
        assert_eq!(
            count(lifecycle, next),
            count(predecessor, previous),
            "CP415 {next}"
        );
    }
    assert_eq!(
        count(lifecycle, "unchanged_supply_temperature_preservation_count") + assignments,
        count(lifecycle, "cp414_supply_temperature_state_owner_count"),
    );
    for field in [
        "cp415_mixed_air_limited_supply_temperature_state_owner_count",
        "cp414_retained_supply_temperature_owned_read_count",
        "supply_temperature_for_minimum_read_count",
        "cp329_retained_mixed_air_temperature_owned_read_count",
        "mixed_air_temperature_for_minimum_read_count",
        "source_shaped_two_argument_minimum_evaluation_count",
        "supply_temperature_assignment_write_count",
    ] {
        assert_eq!(count(lifecycle, field), assignments, "CP415 {field}");
    }

    assert_latest_lineage(
        &lifecycle["latest"],
        &predecessor["latest"],
        &mixed_air_owner["latest"],
        transitions,
    );
    let serialized = lifecycle.to_string().to_ascii_lowercase();
    for forbidden in ["supply_node", "zone_load", "report", "numerical_dto"] {
        assert!(!serialized.contains(forbidden), "CP415 forbids {forbidden}");
    }
    assert!(
        !results.to_string().contains(CP415_KEY),
        "CP415 lifecycle must remain outside numerical result state",
    );
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP415_KEY));
    assert!(
        runtime[CP415_KEY].is_null(),
        "non-direct runtime must not publish CP415 evidence",
    );
}

fn assert_route_contract(lifecycle: &Value) {
    let body = array(lifecycle, "predecessor_guard_body_entry_route_counts");
    let predecessor_assignments = array(
        lifecycle,
        "predecessor_supply_temperature_saturation_assignment_route_counts",
    );
    let limits = array(lifecycle, "supply_temperature_mixed_air_limit_route_counts");
    for index in 0..36 {
        assert_eq!(
            count_value(&predecessor_assignments[index]),
            count_value(&body[index]),
            "CP415 predecessor assignment/body route {index}",
        );
        assert_eq!(
            count_value(&limits[index]),
            count_value(&predecessor_assignments[index]),
            "CP415 limit/predecessor assignment route {index}",
        );
    }
}

fn assert_public_route_firewall(lifecycle: &Value) {
    for field in ROUTE_FIELDS {
        for (index, value) in array(lifecycle, field).iter().enumerate() {
            if !PUBLIC_LOGICAL_INDICES.contains(&index) {
                assert_eq!(count_value(value), 0, "public CP415 {field} route {index}");
            }
        }
    }
}

fn assert_conceptual_contract() {
    let active_public_flattened = [23usize, 25, 35, 37];
    let humidity_ratio_presence = 36;
    let enthalpy_presence = 41;
    let temperature_presence = 51;
    let assignment_outcomes = 18;
    let unchanged_temperature = temperature_presence - assignment_outcomes;
    assert_eq!(
        (
            36 + 18,
            PUBLIC_LOGICAL_INDICES.len() + active_public_flattened.len(),
            37,
            18,
            18 * ORDER.len()
        ),
        (54, 17, 37, 18, 72),
    );
    assert_eq!(
        (
            humidity_ratio_presence,
            enthalpy_presence,
            temperature_presence,
            unchanged_temperature,
        ),
        (36, 41, 51, 33),
        "CP415 W/H/T conceptual ownership and preservation",
    );
}

fn assert_latest_lineage(
    latest: &Value,
    predecessor: &Value,
    mixed_air_owner: &Value,
    transitions: u64,
) {
    assert_eq!(latest["parent_call_ordinal"], transitions);
    for (field, value) in predecessor.as_object().expect("CP414 latest object") {
        if !matches!(
            field.as_str(),
            "source"
                | "first_excluded_source"
                | "source_order"
                | "resulting_supply_humidity_ratio"
                | "resulting_supply_humidity_ratio_ieee_bits"
                | "resulting_supply_enthalpy_j_per_kg"
                | "resulting_supply_enthalpy_j_per_kg_ieee_bits"
                | "resulting_supply_temperature_c"
                | "resulting_supply_temperature_c_ieee_bits"
        ) {
            assert_eq!(latest.get(field), Some(value), "CP415 {field} lineage");
        }
    }
    for (next, previous) in [
        (
            "predecessor_cp414_resulting_supply_humidity_ratio",
            "resulting_supply_humidity_ratio",
        ),
        (
            "predecessor_cp414_resulting_supply_enthalpy_j_per_kg",
            "resulting_supply_enthalpy_j_per_kg",
        ),
        (
            "predecessor_cp414_resulting_supply_temperature_c",
            "resulting_supply_temperature_c",
        ),
    ] {
        assert_same_bits(latest, predecessor, next, previous);
    }

    let active = boolean(
        predecessor,
        "post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_executed",
    );
    assert_eq!(
        latest["post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_executed"],
        active,
    );
    assert_eq!(
        latest["cp414_retained_supply_temperature_state_owned"],
        ieee_bits(predecessor, "resulting_supply_temperature_c").is_some(),
    );
    assert_same_bits(
        latest,
        predecessor,
        "preexisting_supply_temperature_c",
        "resulting_supply_temperature_c",
    );
    assert_same_bits(
        latest,
        predecessor,
        "resulting_supply_humidity_ratio",
        "resulting_supply_humidity_ratio",
    );
    assert_same_bits(
        latest,
        predecessor,
        "resulting_supply_enthalpy_j_per_kg",
        "resulting_supply_enthalpy_j_per_kg",
    );

    if active {
        for field in [
            "cp414_retained_supply_temperature_owned_read",
            "supply_temperature_for_minimum_read",
            "cp329_retained_mixed_air_temperature_owned_read",
            "mixed_air_temperature_for_minimum_read",
            "source_shaped_two_argument_minimum_evaluated",
            "supply_temperature_assignment_performed",
        ] {
            assert_eq!(latest[field], true, "active CP415 {field}");
        }
        assert_same_bits(
            latest,
            predecessor,
            "supply_temperature_before_mixed_air_limit_c",
            "resulting_supply_temperature_c",
        );
        assert_same_bits(
            latest,
            mixed_air_owner,
            "mixed_air_temperature_c",
            "mixed_air_temperature_c",
        );
        let left = f64::from_bits(
            ieee_bits(latest, "supply_temperature_before_mixed_air_limit_c")
                .expect("active CP415 left operand"),
        );
        let right = f64::from_bits(
            ieee_bits(latest, "mixed_air_temperature_c").expect("active CP415 right operand"),
        );
        let expected = if left < right { left } else { right };
        for field in [
            "minimum_supply_temperature_c",
            "assigned_supply_temperature_c",
            "resulting_supply_temperature_c",
        ] {
            assert_eq!(
                ieee_bits(latest, field),
                Some(expected.to_bits()),
                "CP415 {field}"
            );
        }
    } else {
        for field in [
            "cp414_retained_supply_temperature_owned_read",
            "supply_temperature_for_minimum_read",
            "cp329_retained_mixed_air_temperature_owned_read",
            "mixed_air_temperature_for_minimum_read",
            "source_shaped_two_argument_minimum_evaluated",
            "supply_temperature_assignment_performed",
        ] {
            assert_eq!(latest[field], false, "inactive CP415 {field}");
        }
        for field in [
            "supply_temperature_before_mixed_air_limit_c",
            "mixed_air_temperature_c",
            "minimum_supply_temperature_c",
            "assigned_supply_temperature_c",
        ] {
            assert!(latest[field].is_null(), "inactive CP415 {field}");
            assert!(latest[format!("{field}_ieee_bits")].is_null());
        }
        assert_same_bits(
            latest,
            predecessor,
            "resulting_supply_temperature_c",
            "resulting_supply_temperature_c",
        );
    }

    let object = latest.as_object().expect("CP415 latest object");
    assert_eq!(object.len(), 168, "CP415 JSON width");
    assert_eq!(
        object
            .keys()
            .filter(|key| key.ends_with("_ieee_bits"))
            .count(),
        40,
        "CP415 IEEE sidecar count",
    );
}

fn assert_same_bits(next: &Value, previous: &Value, next_field: &str, previous_field: &str) {
    assert_eq!(
        ieee_bits(next, next_field),
        ieee_bits(previous, previous_field),
        "CP415 {next_field} lineage",
    );
}

fn ieee_bits(value: &Value, field: &str) -> Option<u64> {
    let sidecar = &value[format!("{field}_ieee_bits")];
    if sidecar.is_null() {
        return None;
    }
    let text = sidecar.as_str().expect("CP415 IEEE sidecar string");
    let hex = text.strip_prefix("0x").expect("CP415 IEEE sidecar prefix");
    Some(u64::from_str_radix(hex, 16).expect("CP415 IEEE sidecar bits"))
}

fn boolean(value: &Value, field: &str) -> bool {
    value[field].as_bool().expect("CP415 boolean")
}

fn count(value: &Value, field: &str) -> u64 {
    count_value(&value[field])
}

fn count_value(value: &Value) -> u64 {
    value.as_u64().expect("CP415 unsigned count")
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
