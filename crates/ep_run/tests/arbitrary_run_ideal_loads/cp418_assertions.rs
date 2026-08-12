//! CP418 dehumidification-guard else-branch-entry assertions.

use serde_json::{Map, Value, json};

const CP417_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment_lifecycle";
const CP418_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_lifecycle";
const ORDER: [&str; 1] = [
    "enter-post-saturation-capacity-limit-dehumidification-guard-else-branch-after-guard-false-fallthrough",
];
const PUBLIC_LOGICAL_INDICES: [usize; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 20, 21, 26, 27];
const ELSE_ENTRY_LOGICAL_INDICES: [usize; 5] = [4, 7, 10, 13, 16];
const ROUTE_FIELDS: [&str; 8] = [
    "predecessor_route_counts",
    "predecessor_guard_false_fallthrough_route_counts",
    "predecessor_guard_body_entry_route_counts",
    "predecessor_supply_temperature_saturation_assignment_route_counts",
    "predecessor_supply_temperature_mixed_air_limit_route_counts",
    "predecessor_supply_humidity_ratio_assignment_route_counts",
    "predecessor_supply_enthalpy_assignment_route_counts",
    "dehumidification_guard_else_branch_entry_route_counts",
];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let predecessor = &runtime[CP417_KEY];
    let lifecycle = &runtime[CP418_KEY];
    assert_eq!(
        lifecycle["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2327",
    );
    assert_eq!(
        lifecycle["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2330",
    );
    assert_eq!(lifecycle["latest"]["source_order"], json!(ORDER));
    assert_eq!(lifecycle["system"], predecessor["system"]);
    assert_eq!(
        lifecycle["transition_count"],
        predecessor["transition_count"]
    );

    for field in ROUTE_FIELDS {
        assert_eq!(array(lifecycle, field).len(), 36, "CP418 {field} width");
        for (index, value) in array(lifecycle, field).iter().enumerate() {
            if !PUBLIC_LOGICAL_INDICES.contains(&index) {
                assert_eq!(count_value(value), 0, "public CP418 {field} route {index}");
            }
        }
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
            "predecessor_supply_temperature_saturation_assignment_route_counts",
        ),
        (
            "predecessor_supply_temperature_mixed_air_limit_route_counts",
            "predecessor_supply_temperature_mixed_air_limit_route_counts",
        ),
        (
            "predecessor_supply_humidity_ratio_assignment_route_counts",
            "predecessor_supply_humidity_ratio_assignment_route_counts",
        ),
        (
            "predecessor_supply_enthalpy_assignment_route_counts",
            "supply_enthalpy_assignment_route_counts",
        ),
    ] {
        assert_eq!(lifecycle[next], predecessor[previous], "CP418 {next}");
    }

    let routes = array(lifecycle, "predecessor_route_counts");
    let entries = array(
        lifecycle,
        "dehumidification_guard_else_branch_entry_route_counts",
    );
    for index in 0..36 {
        let expected = if ELSE_ENTRY_LOGICAL_INDICES.contains(&index) {
            count_value(&routes[index])
        } else {
            0
        };
        assert_eq!(
            count_value(&entries[index]),
            expected,
            "CP418 entry route {index}"
        );
    }
    let transitions = sum(routes);
    let entry_count = sum(entries);
    assert_eq!(count(lifecycle, "transition_count"), transitions);
    assert_eq!(
        count(lifecycle, "inactive_transition_count") + entry_count,
        transitions
    );
    assert_eq!(
        count(lifecycle, "dehumidification_guard_else_branch_entry_count"),
        entry_count
    );
    assert_eq!(count(lifecycle, "source_site_execution_count"), entry_count);

    let total_outcomes = 54usize;
    let else_entries = ELSE_ENTRY_LOGICAL_INDICES.len();
    let public_outcomes = 17usize;
    assert_eq!(
        (
            total_outcomes - else_entries,
            else_entries,
            public_outcomes,
            total_outcomes - public_outcomes,
        ),
        (49, 5, 17, 37),
    );
    let latest = &lifecycle["latest"];
    let predecessor_latest = &predecessor["latest"];
    assert_eq!(latest["parent_call_ordinal"], transitions);
    for (field, value) in predecessor_latest.as_object().expect("CP417 latest object") {
        if matches!(
            field.as_str(),
            "source" | "first_excluded_source" | "source_order"
        ) {
            continue;
        }
        assert_eq!(latest.get(field), Some(value), "CP418 {field} lineage");
    }
    assert_eq!(
        latest["post_saturation_capacity_limit_dehumidification_guard_else_branch_entered"],
        predecessor_latest["predecessor_dehumidification_guard_false_fallthrough"],
    );
    let object = latest.as_object().expect("CP418 latest object");
    assert_eq!(object.len(), 217, "CP418 JSON width");
    assert_eq!(
        object
            .keys()
            .filter(|key| key.ends_with("_ieee_bits"))
            .count(),
        54,
        "CP418 IEEE sidecar count",
    );

    let serialized = lifecycle.to_string().to_ascii_lowercase();
    for forbidden in [
        "supply_node",
        "zone_load",
        "report",
        "reconciled",
        "numerical_dto",
        "direct_zone_purchased_air_coupling_input",
    ] {
        assert!(!serialized.contains(forbidden), "CP418 forbids {forbidden}");
    }
    assert!(
        !results.to_string().contains(CP418_KEY),
        "CP418 lifecycle must remain outside numerical result state",
    );
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP418_KEY));
    assert!(
        runtime[CP418_KEY].is_null(),
        "non-direct runtime must not publish CP418 evidence",
    );
}

fn count(value: &Value, field: &str) -> u64 {
    count_value(&value[field])
}

fn count_value(value: &Value) -> u64 {
    value.as_u64().expect("CP418 unsigned count")
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
