//! CP406 latent-output capacity-guard else-branch-entry assertions.

use serde_json::{Map, Value, json};

const CP405_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_lifecycle";
const CP406_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry_lifecycle";
const ORDER: [&str; 1] = [
    "enter-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-latent-output-capacity-guard-else-branch-after-guard-false-fallthrough",
];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let predecessor = &runtime[CP405_KEY];
    let lifecycle = &runtime[CP406_KEY];
    assert_eq!(
        lifecycle["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2301"
    );
    assert_eq!(
        lifecycle["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2302"
    );
    assert_eq!(lifecycle["latest"]["source_order"], json!(ORDER));
    assert_eq!(lifecycle["system"], predecessor["system"]);
    assert_eq!(
        lifecycle["transition_count"],
        predecessor["transition_count"]
    );
    for (next, previous) in [
        ("predecessor_route_counts", "predecessor_route_counts"),
        (
            "predecessor_guard_false_fallthrough_route_counts",
            "predecessor_guard_false_fallthrough_route_counts",
        ),
        (
            "predecessor_maximum_capacity_assignment_route_counts",
            "cooling_latent_output_maximum_capacity_assignment_route_counts",
        ),
        (
            "else_branch_entry_route_counts",
            "predecessor_guard_false_fallthrough_route_counts",
        ),
    ] {
        assert_eq!(lifecycle[next], predecessor[previous], "CP406 {next}");
        assert_eq!(array(lifecycle, next).len(), 30, "CP406 {next} width");
    }

    let routes = array(lifecycle, "predecessor_route_counts");
    let false_routes = array(
        lifecycle,
        "predecessor_guard_false_fallthrough_route_counts",
    );
    let assignment_routes = array(
        lifecycle,
        "predecessor_maximum_capacity_assignment_route_counts",
    );
    let else_routes = array(lifecycle, "else_branch_entry_route_counts");
    let transitions = count(lifecycle, "transition_count");
    let inherited_inactive = count(lifecycle, "inactive_transition_count");
    let guard_false = count(lifecycle, "predecessor_guard_false_fallthrough_count");
    let maximum_assignments = count(lifecycle, "predecessor_maximum_capacity_assignment_count");
    let entries = count(
        lifecycle,
        "dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry_count",
    );
    assert_eq!(sum(routes), transitions);
    assert_eq!(sum(false_routes), guard_false);
    assert_eq!(sum(assignment_routes), maximum_assignments);
    assert_eq!(sum(else_routes), entries);
    assert_eq!(guard_false, entries);
    assert_eq!(
        guard_false,
        count(predecessor, "predecessor_guard_false_fallthrough_count")
    );
    assert_eq!(
        maximum_assignments,
        count(
            predecessor,
            "cooling_latent_output_maximum_capacity_assignment_count"
        )
    );
    assert_eq!(
        inherited_inactive,
        count(predecessor, "inactive_transition_count") + maximum_assignments
    );
    assert_eq!(inherited_inactive + entries, transitions);
    assert_eq!(count(lifecycle, "source_site_execution_count"), entries);
    for (index, value) in routes.iter().enumerate() {
        if !matches!(index, 0..=8 | 20 | 24) {
            assert_eq!(
                value.as_u64().unwrap_or_default(),
                0,
                "public CP406 predecessor route {index}"
            );
        }
        let successor_count = false_routes[index].as_u64().unwrap_or_default()
            + assignment_routes[index].as_u64().unwrap_or_default();
        assert_eq!(
            successor_count,
            if matches!(index, 20 | 21 | 24 | 25 | 27 | 29) {
                value.as_u64().unwrap_or_default()
            } else {
                0
            },
            "CP406 successor partition {index}"
        );
    }

    assert_latest_lineage(&lifecycle["latest"], &predecessor["latest"], transitions);
    assert!(
        !results.to_string().contains(CP406_KEY),
        "CP406 lifecycle must remain outside numerical result state"
    );
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP406_KEY));
    assert!(
        runtime[CP406_KEY].is_null(),
        "non-direct runtime must not publish CP406 evidence"
    );
}

fn assert_latest_lineage(latest: &Value, predecessor: &Value, transitions: u64) {
    assert_eq!(latest["parent_call_ordinal"], transitions);
    assert_eq!(latest["system"], predecessor["system"]);
    assert_eq!(latest["controlled_zone"], predecessor["controlled_zone"]);
    assert_eq!(
        latest["predecessor_dehumidification_control_type"],
        predecessor["predecessor_dehumidification_control_type"]
    );
    for field in inherited_control_fields() {
        assert_eq!(latest[field], predecessor[field], "CP406 {field} lineage");
    }
    for field in [
        "predecessor_dehumidification_control_none_case_entered",
        "predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered",
        "predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough",
    ] {
        assert_eq!(latest[field], predecessor[field], "CP406 {field} lineage");
    }
    assert_eq!(
        latest["predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed"],
        predecessor["dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed"]
    );
    assert_eq!(
        latest["dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entered"],
        predecessor["predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough"]
    );
    for suffix in ["humidity_ratio", "enthalpy_j_per_kg", "temperature_c"] {
        assert_same_bits(
            latest,
            predecessor,
            &format!("predecessor_cp405_resulting_supply_{suffix}"),
            &format!("resulting_supply_{suffix}"),
        );
        assert_same_bits(
            latest,
            predecessor,
            &format!("resulting_supply_{suffix}"),
            &format!("resulting_supply_{suffix}"),
        );
    }

    let object = latest.as_object().expect("CP406 latest object");
    assert_eq!(object.len(), 52, "CP406 compact JSON width");
    assert_eq!(
        object
            .keys()
            .filter(|key| key.ends_with("_ieee_bits"))
            .count(),
        6,
        "CP406 IEEE sidecar count"
    );
    for forbidden in [
        "preexisting_cooling_latent_output_w",
        "maximum_total_cooling_capacity_w",
        "assigned_cooling_latent_output_w",
        "resulting_cooling_latent_output_w",
        "numerical_dto",
        "coupling",
        "supply_node",
    ] {
        assert!(!object.contains_key(forbidden), "CP406 dropped {forbidden}");
        assert!(
            !object.contains_key(&format!("{forbidden}_ieee_bits")),
            "CP406 dropped {forbidden} sidecar"
        );
    }
}

fn inherited_control_fields() -> [&'static str; 28] {
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
        "CP406 {next_field} lineage"
    );
}

fn count(value: &Value, field: &str) -> u64 {
    value[field].as_u64().expect("CP406 count")
}

fn array<'a>(value: &'a Value, field: &str) -> &'a Vec<Value> {
    value[field].as_array().expect("CP406 route array")
}

fn sum(values: &[Value]) -> u64 {
    values
        .iter()
        .map(|value| value.as_u64().unwrap_or_default())
        .sum()
}
