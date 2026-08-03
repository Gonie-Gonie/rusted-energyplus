//! CP410 post-saturation typed-default case-break assertions.

use serde_json::{Map, Value, json};

#[path = "cp411_assertions.rs"]
mod cp411_assertions;

const CP409_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_break_lifecycle";
const CP410_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_default_case_break_lifecycle";
const ORDER: [&str; 1] = [
    "exit-purchased-air-post-saturation-capacity-limit-dehumidification-control-default-case-via-break",
];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let predecessor = &runtime[CP409_KEY];
    let lifecycle = &runtime[CP410_KEY];
    assert_eq!(
        lifecycle["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2308"
    );
    assert_eq!(
        lifecycle["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2313"
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
        assert_eq!(lifecycle[field], predecessor[field], "CP410 {field}");
        assert_eq!(array(lifecycle, field).len(), 30, "CP410 {field} width");
    }
    assert_route_partition(lifecycle);

    let transitions = count(lifecycle, "transition_count");
    let guard_false = count(lifecycle, "predecessor_guard_false_fallthrough_count");
    let maximum = count(lifecycle, "predecessor_maximum_capacity_assignment_count");
    let predecessor_breaks = guard_false + maximum;
    assert_eq!(count(lifecycle, "inactive_transition_count"), transitions);
    assert_eq!(
        guard_false,
        count(predecessor, "predecessor_guard_false_fallthrough_count")
    );
    assert_eq!(
        maximum,
        count(predecessor, "predecessor_maximum_capacity_assignment_count")
    );
    assert_eq!(
        predecessor_breaks,
        count(
            predecessor,
            "dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_break_count",
        )
    );
    assert_eq!(
        count(predecessor, "inactive_transition_count") + predecessor_breaks,
        transitions
    );
    assert_eq!(
        count(
            lifecycle,
            "dehumidification_control_default_case_break_count"
        ),
        0
    );
    assert_eq!(count(lifecycle, "source_site_execution_count"), 0);

    for (index, value) in array(lifecycle, "predecessor_route_counts")
        .iter()
        .enumerate()
    {
        if !matches!(index, 0..=8 | 20 | 24) {
            assert_eq!(
                value.as_u64().unwrap_or_default(),
                0,
                "public CP410 predecessor route {index}"
            );
        }
    }
    assert_latest_lineage(&lifecycle["latest"], &predecessor["latest"], transitions);

    let serialized = lifecycle.to_string().to_ascii_lowercase();
    for forbidden in ["supply_node", "zone_load", "report", "numerical_dto"] {
        assert!(!serialized.contains(forbidden), "CP410 forbids {forbidden}");
    }
    assert!(
        !results.to_string().contains(CP410_KEY),
        "CP410 lifecycle must remain outside numerical result state"
    );
    cp411_assertions::assert_direct(runtime, results);
}

fn assert_route_partition(lifecycle: &Value) {
    let routes = array(lifecycle, "predecessor_route_counts");
    let guard_false = array(
        lifecycle,
        "predecessor_guard_false_fallthrough_route_counts",
    );
    let maximum = array(
        lifecycle,
        "predecessor_maximum_capacity_assignment_route_counts",
    );
    for index in 0..routes.len() {
        let branch_count = guard_false[index]
            .as_u64()
            .unwrap_or_default()
            .checked_add(maximum[index].as_u64().unwrap_or_default());
        let expected = if matches!(index, 20 | 21 | 24 | 25 | 27 | 29) {
            routes[index].as_u64()
        } else {
            Some(0)
        };
        assert_eq!(
            branch_count, expected,
            "CP410 predecessor break route partition {index}"
        );
    }
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP410_KEY));
    assert!(
        runtime[CP410_KEY].is_null(),
        "non-direct runtime must not publish CP410 evidence"
    );
    cp411_assertions::assert_non_direct(runtime);
}

fn assert_latest_lineage(latest: &Value, predecessor: &Value, transitions: u64) {
    assert_eq!(latest["parent_call_ordinal"], transitions);
    assert_eq!(latest["system"], predecessor["system"]);
    assert_eq!(latest["controlled_zone"], predecessor["controlled_zone"]);
    for field in inherited_fields() {
        assert_eq!(latest[field], predecessor[field], "CP410 {field} lineage");
    }
    for field in [
        "predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered",
        "predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough",
        "predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed",
    ] {
        assert_eq!(latest[field], predecessor[field], "CP410 {field} lineage");
    }
    assert_eq!(
        latest["predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_exited_via_break"],
        predecessor["dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_exited_via_break"]
    );
    for (next, previous) in [
        (
            "predecessor_cp409_resulting_supply_humidity_ratio",
            "resulting_supply_humidity_ratio",
        ),
        (
            "predecessor_cp409_resulting_supply_enthalpy_j_per_kg",
            "resulting_supply_enthalpy_j_per_kg",
        ),
        (
            "predecessor_cp409_resulting_supply_temperature_c",
            "resulting_supply_temperature_c",
        ),
    ] {
        assert_same_bits(latest, predecessor, next, previous);
    }
    assert_eq!(
        latest["dehumidification_control_default_case_exited_via_break"],
        false
    );
    for field in [
        "resulting_supply_humidity_ratio",
        "resulting_supply_enthalpy_j_per_kg",
        "resulting_supply_temperature_c",
    ] {
        assert_same_bits(latest, predecessor, field, field);
    }
    for removed in [
        "predecessor_cp408_resulting_supply_humidity_ratio",
        "predecessor_cp408_resulting_supply_enthalpy_j_per_kg",
        "predecessor_cp408_resulting_supply_temperature_c",
    ] {
        assert!(latest.get(removed).is_none(), "CP410 omits {removed}");
    }
    assert_eq!(latest.as_object().map(|object| object.len()), Some(52));
    assert_eq!(
        latest.as_object().map(|object| {
            object
                .keys()
                .filter(|key| key.ends_with("_ieee_bits"))
                .count()
        }),
        Some(6)
    );
}

fn inherited_fields() -> [&'static str; 29] {
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
    ]
}

fn assert_same_bits(next: &Value, previous: &Value, next_field: &str, previous_field: &str) {
    assert_eq!(
        next[format!("{next_field}_ieee_bits")],
        previous[format!("{previous_field}_ieee_bits")],
        "CP410 {next_field} lineage"
    );
}

fn count(value: &Value, field: &str) -> u64 {
    value[field].as_u64().expect("CP410 count")
}

fn array<'a>(value: &'a Value, field: &str) -> &'a [Value] {
    value[field]
        .as_array()
        .map(Vec::as_slice)
        .unwrap_or_default()
}
