//! CP409 post-saturation shared-case break assertions.

use serde_json::{Map, Value, json};

#[path = "cp410_assertions.rs"]
mod cp410_assertions;

const CP408_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit_lifecycle";
const CP409_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_break_lifecycle";
const ORDER: [&str; 1] = [
    "exit-purchased-air-post-saturation-capacity-limit-dehumidification-control-none-or-constant-supply-humidity-ratio-shared-case-via-break",
];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let predecessor = &runtime[CP408_KEY];
    let lifecycle = &runtime[CP409_KEY];
    assert_eq!(
        lifecycle["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2306"
    );
    assert_eq!(
        lifecycle["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2308"
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
        assert_eq!(lifecycle[field], predecessor[field], "CP409 {field}");
        assert_eq!(array(lifecycle, field).len(), 30, "CP409 {field} width");
    }
    assert_eq!(
        lifecycle["predecessor_guard_false_fallthrough_route_counts"],
        predecessor["supply_temperature_mixed_air_limit_route_counts"],
        "CP409 guard-false routes must equal CP408 owned mixed-air-limit routes"
    );
    assert_route_partition(lifecycle);

    let transitions = count(lifecycle, "transition_count");
    let inactive = count(lifecycle, "inactive_transition_count");
    let guard_false = count(lifecycle, "predecessor_guard_false_fallthrough_count");
    let maximum = count(lifecycle, "predecessor_maximum_capacity_assignment_count");
    let breaks = count(
        lifecycle,
        "dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_break_count",
    );
    assert_eq!(breaks, guard_false + maximum);
    assert_eq!(inactive + breaks, transitions);
    assert_eq!(
        count(predecessor, "inactive_transition_count"),
        inactive + maximum
    );
    assert_eq!(
        guard_false,
        count(
            predecessor,
            "dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit_count",
        )
    );
    assert_eq!(
        maximum,
        count(predecessor, "predecessor_maximum_capacity_assignment_count")
    );
    assert_eq!(count(lifecycle, "source_site_execution_count"), breaks);

    for (index, value) in array(lifecycle, "predecessor_route_counts")
        .iter()
        .enumerate()
    {
        if !matches!(index, 0..=8 | 20 | 24) {
            assert_eq!(
                value.as_u64().unwrap_or_default(),
                0,
                "public CP409 predecessor route {index}"
            );
        }
    }
    assert_latest_lineage(&lifecycle["latest"], &predecessor["latest"], transitions);

    let serialized = lifecycle.to_string().to_ascii_lowercase();
    for forbidden in ["supply_node", "zone_load", "report", "numerical_dto"] {
        assert!(!serialized.contains(forbidden), "CP409 forbids {forbidden}");
    }
    assert!(
        !results.to_string().contains(CP409_KEY),
        "CP409 lifecycle must remain outside numerical result state"
    );
    cp410_assertions::assert_direct(runtime, results);
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
            "CP409 active route evidence partition {index}"
        );
    }
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP409_KEY));
    assert!(
        runtime[CP409_KEY].is_null(),
        "non-direct runtime must not publish CP409 evidence"
    );
    cp410_assertions::assert_non_direct(runtime);
}

fn assert_latest_lineage(latest: &Value, predecessor: &Value, transitions: u64) {
    assert_eq!(latest["parent_call_ordinal"], transitions);
    assert_eq!(latest["system"], predecessor["system"]);
    assert_eq!(latest["controlled_zone"], predecessor["controlled_zone"]);
    for field in inherited_fields() {
        assert_eq!(latest[field], predecessor[field], "CP409 {field} lineage");
    }
    for field in [
        "predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered",
        "predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough",
        "predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed",
    ] {
        assert_eq!(latest[field], predecessor[field], "CP409 {field} lineage");
    }
    for (next, previous) in [
        (
            "predecessor_cp408_resulting_supply_humidity_ratio",
            "resulting_supply_humidity_ratio",
        ),
        (
            "predecessor_cp408_resulting_supply_enthalpy_j_per_kg",
            "resulting_supply_enthalpy_j_per_kg",
        ),
        (
            "predecessor_cp408_resulting_supply_temperature_c",
            "resulting_supply_temperature_c",
        ),
    ] {
        assert_same_bits(latest, predecessor, next, previous);
    }

    let guard_false = latest
        ["predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough"]
        .as_bool()
        .unwrap_or(false);
    let maximum = latest
        ["predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed"]
        .as_bool()
        .unwrap_or(false);
    let active = guard_false || maximum;
    assert!(!(guard_false && maximum));
    assert_eq!(
        latest["predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered"],
        active
    );
    assert_eq!(
        latest["dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_exited_via_break"],
        active
    );
    for field in [
        "resulting_supply_humidity_ratio",
        "resulting_supply_enthalpy_j_per_kg",
        "resulting_supply_temperature_c",
    ] {
        assert_same_bits(latest, predecessor, field, field);
    }
    assert_eq!(latest.as_object().map(|object| object.len()), Some(51));
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
        "CP409 {next_field} lineage"
    );
}

fn count(value: &Value, field: &str) -> u64 {
    value[field].as_u64().unwrap_or_default()
}

fn array<'a>(value: &'a Value, field: &str) -> &'a [Value] {
    value[field]
        .as_array()
        .map(Vec::as_slice)
        .unwrap_or_default()
}
