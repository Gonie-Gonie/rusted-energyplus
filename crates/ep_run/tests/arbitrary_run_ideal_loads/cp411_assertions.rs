//! CP411 pre-saturation original humidity-ratio assignment assertions.

use serde_json::{Map, Value, json};

const CP410_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_default_case_break_lifecycle";
const CP411_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment_lifecycle";
const ORDER: [&str; 2] = [
    "read-purchased-air-supply-humidity-ratio-before-saturation-limit",
    "assign-local-original-supply-humidity-ratio-before-saturation-limit",
];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let predecessor = &runtime[CP410_KEY];
    let lifecycle = &runtime[CP411_KEY];
    assert_eq!(
        lifecycle["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2313"
    );
    assert_eq!(
        lifecycle["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2314"
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
        "predecessor_maximum_capacity_assignment_route_counts",
    ] {
        assert_eq!(lifecycle[field], predecessor[field], "CP411 {field}");
        assert_eq!(array(lifecycle, field).len(), 30, "CP411 {field} width");
    }
    assert_eq!(
        array(
            lifecycle,
            "supply_humidity_ratio_pre_saturation_original_assignment_route_counts"
        )
        .len(),
        30
    );
    assert_route_partitions(lifecycle);
    assert_public_route_firewall(lifecycle);

    let routes = array(lifecycle, "predecessor_route_counts");
    let transitions = count(lifecycle, "transition_count");
    let assignments = count(
        lifecycle,
        "supply_humidity_ratio_pre_saturation_original_assignment_count",
    );
    let expected_assignments = sum(&routes[18..]);
    let enthalpy_owners = sum_indices(routes, &[5, 8, 11, 14, 17]) + sum(&routes[18..]);
    let temperature_owners = sum(&routes[3..]);
    assert_eq!(assignments, expected_assignments);
    assert_eq!(
        count(lifecycle, "inactive_transition_count") + assignments,
        transitions
    );
    assert_eq!(
        count(lifecycle, "predecessor_guard_false_fallthrough_count"),
        count(predecessor, "predecessor_guard_false_fallthrough_count")
    );
    assert_eq!(
        count(lifecycle, "predecessor_maximum_capacity_assignment_count"),
        count(predecessor, "predecessor_maximum_capacity_assignment_count")
    );
    assert_eq!(
        count(lifecycle, "source_site_execution_count"),
        2 * assignments
    );
    for field in [
        "cp410_supply_humidity_ratio_state_owner_count",
        "unchanged_supply_humidity_ratio_preservation_count",
    ] {
        assert_eq!(
            count(lifecycle, field),
            expected_assignments,
            "CP411 {field}"
        );
    }
    for field in [
        "cp410_supply_enthalpy_state_owner_count",
        "unchanged_supply_enthalpy_preservation_count",
    ] {
        assert_eq!(count(lifecycle, field), enthalpy_owners, "CP411 {field}");
    }
    for field in [
        "cp410_supply_temperature_state_owner_count",
        "unchanged_supply_temperature_preservation_count",
    ] {
        assert_eq!(count(lifecycle, field), temperature_owners, "CP411 {field}");
    }
    for field in [
        "cp410_retained_supply_humidity_ratio_owned_read_count",
        "purchased_air_supply_humidity_ratio_before_saturation_limit_read_count",
        "local_supply_humidity_ratio_original_assignment_write_count",
    ] {
        assert_eq!(count(lifecycle, field), assignments, "CP411 {field}");
    }

    assert_latest_lineage(&lifecycle["latest"], &predecessor["latest"], transitions);

    let serialized = lifecycle.to_string().to_ascii_lowercase();
    for forbidden in ["supply_node", "zone_load", "report", "numerical_dto"] {
        assert!(!serialized.contains(forbidden), "CP411 forbids {forbidden}");
    }
    assert!(
        !results.to_string().contains(CP411_KEY),
        "CP411 lifecycle must remain outside numerical result state"
    );
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP411_KEY));
    assert!(
        runtime[CP411_KEY].is_null(),
        "non-direct runtime must not publish CP411 evidence"
    );
}

fn assert_route_partitions(lifecycle: &Value) {
    let routes = array(lifecycle, "predecessor_route_counts");
    let guard_false = array(
        lifecycle,
        "predecessor_guard_false_fallthrough_route_counts",
    );
    let maximum = array(
        lifecycle,
        "predecessor_maximum_capacity_assignment_route_counts",
    );
    let assignment = array(
        lifecycle,
        "supply_humidity_ratio_pre_saturation_original_assignment_route_counts",
    );
    for index in 0..routes.len() {
        let route_count = routes[index].as_u64().unwrap_or_default();
        let predecessor_branch_count = guard_false[index]
            .as_u64()
            .unwrap_or_default()
            .checked_add(maximum[index].as_u64().unwrap_or_default());
        let expected_branch = if matches!(index, 20 | 21 | 24 | 25 | 27 | 29) {
            Some(route_count)
        } else {
            Some(0)
        };
        assert_eq!(
            predecessor_branch_count, expected_branch,
            "CP411 predecessor split route {index}"
        );
        assert_eq!(
            assignment[index].as_u64().unwrap_or_default(),
            if index >= 18 { route_count } else { 0 },
            "CP411 assignment route {index}"
        );
    }
}

fn assert_public_route_firewall(lifecycle: &Value) {
    for field in [
        "predecessor_route_counts",
        "predecessor_guard_false_fallthrough_route_counts",
        "predecessor_maximum_capacity_assignment_route_counts",
        "supply_humidity_ratio_pre_saturation_original_assignment_route_counts",
    ] {
        for (index, value) in array(lifecycle, field).iter().enumerate() {
            if !matches!(index, 0..=8 | 20 | 24) {
                assert_eq!(
                    value.as_u64().unwrap_or_default(),
                    0,
                    "public CP411 {field} route {index}"
                );
            }
        }
    }
}

fn assert_latest_lineage(latest: &Value, predecessor: &Value, transitions: u64) {
    assert_eq!(
        latest["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2313"
    );
    assert_eq!(
        latest["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2314"
    );
    assert_eq!(latest["parent_call_ordinal"], transitions);
    assert_eq!(latest["system"], predecessor["system"]);
    assert_eq!(latest["controlled_zone"], predecessor["controlled_zone"]);
    for field in inherited_fields() {
        assert_eq!(latest[field], predecessor[field], "CP411 {field} lineage");
    }
    assert_eq!(
        latest["predecessor_dehumidification_control_default_case_exited_via_break"],
        predecessor["dehumidification_control_default_case_exited_via_break"],
    );
    for (next, previous) in [
        (
            "predecessor_cp409_resulting_supply_humidity_ratio",
            "predecessor_cp409_resulting_supply_humidity_ratio",
        ),
        (
            "predecessor_cp409_resulting_supply_enthalpy_j_per_kg",
            "predecessor_cp409_resulting_supply_enthalpy_j_per_kg",
        ),
        (
            "predecessor_cp409_resulting_supply_temperature_c",
            "predecessor_cp409_resulting_supply_temperature_c",
        ),
        (
            "predecessor_cp410_resulting_supply_humidity_ratio",
            "resulting_supply_humidity_ratio",
        ),
        (
            "predecessor_cp410_resulting_supply_enthalpy_j_per_kg",
            "resulting_supply_enthalpy_j_per_kg",
        ),
        (
            "predecessor_cp410_resulting_supply_temperature_c",
            "resulting_supply_temperature_c",
        ),
    ] {
        assert_same_bits(latest, predecessor, next, previous);
    }
    let active = predecessor["predecessor_dehumidification_control_switch_dispatched"]
        .as_bool()
        .unwrap_or(false);
    assert_eq!(
        latest["post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment_executed"],
        active
    );
    assert_eq!(
        latest["local_supply_humidity_ratio_original_assignment_performed"],
        active
    );
    assert_eq!(
        latest["cp410_retained_supply_humidity_ratio_owned_read"],
        active
    );
    assert_eq!(latest["purchased_air_supply_humidity_ratio_read"], active);
    for (owner, predecessor_field) in [
        (
            "cp410_retained_supply_humidity_ratio_state_owned",
            "resulting_supply_humidity_ratio",
        ),
        (
            "cp410_retained_supply_enthalpy_state_owned",
            "resulting_supply_enthalpy_j_per_kg",
        ),
        (
            "cp410_retained_supply_temperature_state_owned",
            "resulting_supply_temperature_c",
        ),
    ] {
        assert_eq!(
            latest[owner],
            !predecessor[format!("{predecessor_field}_ieee_bits")].is_null(),
            "CP411 {owner}",
        );
    }
    for field in [
        "purchased_air_supply_humidity_ratio_before_saturation_check",
        "assigned_supply_humidity_ratio_original",
        "resulting_supply_humidity_ratio_original",
    ] {
        if active {
            assert_same_bits(
                latest,
                predecessor,
                field,
                "resulting_supply_humidity_ratio",
            );
        } else {
            assert!(latest[field].is_null(), "CP411 inactive {field}");
            assert!(
                latest[format!("{field}_ieee_bits")].is_null(),
                "CP411 inactive {field} bits"
            );
        }
    }
    for field in [
        "resulting_supply_humidity_ratio",
        "resulting_supply_enthalpy_j_per_kg",
        "resulting_supply_temperature_c",
    ] {
        assert_same_bits(latest, predecessor, field, field);
    }
    assert_eq!(latest.as_object().map(|object| object.len()), Some(71));
    assert_eq!(
        latest.as_object().map(|object| {
            object
                .keys()
                .filter(|key| key.ends_with("_ieee_bits"))
                .count()
        }),
        Some(12)
    );
}

fn inherited_fields() -> [&'static str; 33] {
    [
        "unit_off_skipped",
        "non_cooling_skipped",
        "positive_guard_false_fallthrough_skipped",
        "heating_availability_guard_false_fallthrough",
        "humidification_control_guard_false_fallthrough",
        "dehumidification_control_humidistat_maximum_assignment_executed",
        "dehumidification_control_none_maximum_assignment_executed",
        "dehumidification_control_guard_false_fallthrough",
        "predecessor_capacity_limit_guard_evaluated",
        "predecessor_capacity_limit_body_entered",
        "predecessor_active_capacity_limit_guard_false_fallthrough",
        "predecessor_dehumidification_guard_evaluated",
        "predecessor_dehumidification_body_entered",
        "predecessor_dehumidification_guard_false_fallthrough",
        "predecessor_dehumidification_total_output_assignment_executed",
        "predecessor_dehumidification_total_output_capacity_guard_evaluated",
        "predecessor_dehumidification_total_output_capacity_adjustment_body_entered",
        "predecessor_dehumidification_total_output_capacity_guard_false_fallthrough",
        "dehumidification_total_output_capacity_guard_false_fallthrough",
        "dehumidification_total_output_maximum_capacity_assignment_executed",
        "predecessor_supply_enthalpy_assignment_executed",
        "predecessor_dehumidification_control_type_read",
        "predecessor_dehumidification_control_type",
        "predecessor_dehumidification_control_switch_dispatched",
        "predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered",
        "predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break",
        "predecessor_dehumidification_control_humidistat_case_entered",
        "predecessor_dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed",
        "predecessor_dehumidification_control_humidistat_case_exited_via_break",
        "predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered",
        "predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough",
        "predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed",
        "predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_exited_via_break",
    ]
}

fn assert_same_bits(next: &Value, previous: &Value, next_field: &str, previous_field: &str) {
    assert_eq!(
        next[format!("{next_field}_ieee_bits")],
        previous[format!("{previous_field}_ieee_bits")],
        "CP411 {next_field} lineage"
    );
}

fn sum(values: &[Value]) -> u64 {
    values
        .iter()
        .map(|value| value.as_u64().unwrap_or_default())
        .sum()
}

fn sum_indices(values: &[Value], indices: &[usize]) -> u64 {
    indices
        .iter()
        .map(|index| values[*index].as_u64().unwrap_or_default())
        .sum()
}

fn count(value: &Value, field: &str) -> u64 {
    value[field].as_u64().expect("CP411 count")
}

fn array<'a>(value: &'a Value, field: &str) -> &'a [Value] {
    value[field]
        .as_array()
        .map(Vec::as_slice)
        .unwrap_or_default()
}
