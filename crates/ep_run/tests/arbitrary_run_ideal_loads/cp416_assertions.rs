//! CP416 psychrometric supply-humidity-ratio assignment assertions.

#[path = "cp417_assertions.rs"]
mod cp417_assertions;

use ep_runtime::psychrometrics::energyplus_psy_w_fn_tdb_h;
use serde_json::{Map, Value, json};

const CP415_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_lifecycle";
const CP416_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment_lifecycle";
const ORDER: [&str; 4] = [
    "read-purchased-air-supply-temperature-for-post-saturation-capacity-limit-dehumidification-humidity-ratio-inversion",
    "read-local-supply-enthalpy-for-post-saturation-capacity-limit-dehumidification-humidity-ratio-inversion",
    "evaluate-psy-w-fn-tdb-h-for-post-saturation-capacity-limit-dehumidification",
    "assign-purchased-air-supply-humidity-ratio-for-post-saturation-capacity-limit-dehumidification",
];
const SPLIT_PREDECESSOR_INDICES: [usize; 6] = [20, 21, 24, 25, 27, 29];
const PUBLIC_LOGICAL_INDICES: [usize; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 20, 21, 26, 27];
const ROUTE_FIELDS: [&str; 6] = [
    "predecessor_route_counts",
    "predecessor_guard_false_fallthrough_route_counts",
    "predecessor_guard_body_entry_route_counts",
    "predecessor_supply_temperature_saturation_assignment_route_counts",
    "predecessor_supply_temperature_mixed_air_limit_route_counts",
    "supply_humidity_ratio_assignment_route_counts",
];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let predecessor = &runtime[CP415_KEY];
    let lifecycle = &runtime[CP416_KEY];
    assert_eq!(
        lifecycle["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2320",
    );
    assert_eq!(
        lifecycle["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2321",
    );
    assert_eq!(lifecycle["latest"]["source_order"], json!(ORDER));
    assert_eq!(lifecycle["system"], predecessor["system"]);
    assert_eq!(
        lifecycle["transition_count"],
        predecessor["transition_count"]
    );

    for field in ROUTE_FIELDS {
        assert_eq!(array(lifecycle, field).len(), 36, "CP416 {field} width");
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
            "supply_temperature_mixed_air_limit_route_counts",
        ),
        (
            "supply_humidity_ratio_assignment_route_counts",
            "supply_temperature_mixed_air_limit_route_counts",
        ),
    ] {
        assert_eq!(lifecycle[next], predecessor[previous], "CP416 {next}");
    }
    assert_route_contract(lifecycle);
    assert_public_route_firewall(lifecycle);
    assert_conceptual_contract();

    let routes = array(lifecycle, "predecessor_route_counts");
    let transitions = sum(routes);
    let assignments = sum(array(
        lifecycle,
        "supply_humidity_ratio_assignment_route_counts",
    ));
    let humidity_ratio_owners = sum(&routes[18..]);
    let enthalpy_owners =
        sum_predecessor_indices(routes, |index| matches!(index, 5 | 8 | 11 | 14 | 17..=29));
    let temperature_owners = sum_predecessor_indices(routes, |index| index >= 3);
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
            "predecessor_supply_temperature_saturation_assignment_count"
        ),
    );
    assert_eq!(
        count(
            lifecycle,
            "predecessor_supply_temperature_saturation_mixed_air_limit_count"
        ),
        count(
            predecessor,
            "supply_temperature_saturation_mixed_air_limit_count"
        ),
    );
    assert_eq!(
        count(lifecycle, "supply_humidity_ratio_assignment_count"),
        assignments,
    );
    assert_eq!(
        count(lifecycle, "source_site_execution_count"),
        assignments * ORDER.len() as u64,
    );
    assert_eq!(
        count(lifecycle, "cp415_supply_humidity_ratio_state_owner_count"),
        humidity_ratio_owners,
    );
    assert_eq!(
        count(
            lifecycle,
            "unchanged_supply_humidity_ratio_preservation_count"
        ) + assignments,
        humidity_ratio_owners,
    );
    assert_eq!(
        count(lifecycle, "cp415_supply_enthalpy_state_owner_count"),
        enthalpy_owners,
    );
    assert_eq!(
        count(lifecycle, "unchanged_supply_enthalpy_preservation_count"),
        enthalpy_owners,
    );
    assert_eq!(
        count(lifecycle, "cp415_supply_temperature_state_owner_count"),
        temperature_owners,
    );
    assert_eq!(
        count(lifecycle, "unchanged_supply_temperature_preservation_count"),
        temperature_owners,
    );
    for field in [
        "cp416_psychrometric_supply_humidity_ratio_state_owner_count",
        "cp415_retained_supply_temperature_owned_read_count",
        "supply_temperature_for_humidity_ratio_inversion_read_count",
        "cp415_retained_supply_enthalpy_owned_read_count",
        "supply_enthalpy_for_humidity_ratio_inversion_read_count",
        "psychrometric_supply_humidity_ratio_evaluation_count",
        "supply_humidity_ratio_assignment_write_count",
    ] {
        assert_eq!(count(lifecycle, field), assignments, "CP416 {field}");
    }

    assert_latest_lineage(&lifecycle["latest"], &predecessor["latest"], transitions);
    let serialized = lifecycle.to_string().to_ascii_lowercase();
    for forbidden in ["supply_node", "zone_load", "report", "numerical_dto"] {
        assert!(!serialized.contains(forbidden), "CP416 forbids {forbidden}");
    }
    assert!(
        !results.to_string().contains(CP416_KEY),
        "CP416 lifecycle must remain outside numerical result state",
    );
    cp417_assertions::assert_direct(runtime, results);
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP416_KEY));
    assert!(
        runtime[CP416_KEY].is_null(),
        "non-direct runtime must not publish CP416 evidence",
    );
    cp417_assertions::assert_non_direct(runtime);
}

fn assert_route_contract(lifecycle: &Value) {
    let body = array(lifecycle, "predecessor_guard_body_entry_route_counts");
    let predecessor_assignments = array(
        lifecycle,
        "predecessor_supply_temperature_saturation_assignment_route_counts",
    );
    let predecessor_limits = array(
        lifecycle,
        "predecessor_supply_temperature_mixed_air_limit_route_counts",
    );
    let assignments = array(lifecycle, "supply_humidity_ratio_assignment_route_counts");
    for index in 0..36 {
        assert_eq!(
            count_value(&predecessor_assignments[index]),
            count_value(&body[index]),
            "CP416 predecessor assignment/body route {index}",
        );
        assert_eq!(
            count_value(&predecessor_limits[index]),
            count_value(&predecessor_assignments[index]),
            "CP416 predecessor limit/assignment route {index}",
        );
        assert_eq!(
            count_value(&assignments[index]),
            count_value(&predecessor_limits[index]),
            "CP416 assignment/predecessor limit route {index}",
        );
    }
}

fn assert_public_route_firewall(lifecycle: &Value) {
    for field in ROUTE_FIELDS {
        for (index, value) in array(lifecycle, field).iter().enumerate() {
            if !PUBLIC_LOGICAL_INDICES.contains(&index) {
                assert_eq!(count_value(value), 0, "public CP416 {field} route {index}");
            }
        }
    }
}

fn assert_conceptual_contract() {
    let active_public_flattened = [23usize, 25, 35, 37];
    assert_eq!(
        (
            36 + 18,
            PUBLIC_LOGICAL_INDICES.len() + active_public_flattened.len(),
            37,
            18,
            18 * ORDER.len(),
        ),
        (54, 17, 37, 18, 72),
    );
    let humidity_ratio_presence = 36;
    let enthalpy_presence = 41;
    let temperature_presence = 51;
    let assignment_outcomes = 18;
    let unchanged_humidity_ratio = humidity_ratio_presence - assignment_outcomes;
    assert_eq!(
        (
            humidity_ratio_presence,
            enthalpy_presence,
            temperature_presence,
            unchanged_humidity_ratio,
        ),
        (36, 41, 51, 18),
    );
}

fn assert_latest_lineage(latest: &Value, predecessor: &Value, transitions: u64) {
    assert_eq!(latest["parent_call_ordinal"], transitions);
    for (field, value) in predecessor.as_object().expect("CP415 latest object") {
        let inherited_field = match field.as_str() {
            "source" | "first_excluded_source" | "source_order" => continue,
            "resulting_supply_humidity_ratio" => {
                "predecessor_cp415_resulting_supply_humidity_ratio"
            }
            "resulting_supply_humidity_ratio_ieee_bits" => {
                "predecessor_cp415_resulting_supply_humidity_ratio_ieee_bits"
            }
            "resulting_supply_enthalpy_j_per_kg" => {
                "predecessor_cp415_resulting_supply_enthalpy_j_per_kg"
            }
            "resulting_supply_enthalpy_j_per_kg_ieee_bits" => {
                "predecessor_cp415_resulting_supply_enthalpy_j_per_kg_ieee_bits"
            }
            "resulting_supply_temperature_c" => "predecessor_cp415_resulting_supply_temperature_c",
            "resulting_supply_temperature_c_ieee_bits" => {
                "predecessor_cp415_resulting_supply_temperature_c_ieee_bits"
            }
            field => field,
        };
        assert_eq!(
            latest.get(inherited_field),
            Some(value),
            "CP416 {field} lineage"
        );
    }

    let active = boolean(
        predecessor,
        "post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_executed",
    );
    assert_eq!(
        latest["post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment_executed"],
        active,
    );
    assert_eq!(
        latest["cp415_retained_supply_humidity_ratio_state_owned"],
        ieee_bits(predecessor, "resulting_supply_humidity_ratio").is_some(),
    );
    assert_eq!(
        latest["cp415_retained_supply_enthalpy_state_owned"],
        ieee_bits(predecessor, "resulting_supply_enthalpy_j_per_kg").is_some(),
    );
    assert_eq!(
        latest["cp415_retained_supply_temperature_state_owned"],
        ieee_bits(predecessor, "resulting_supply_temperature_c").is_some(),
    );
    assert_same_bits(
        latest,
        predecessor,
        "resulting_supply_enthalpy_j_per_kg",
        "resulting_supply_enthalpy_j_per_kg",
    );
    assert_same_bits(
        latest,
        predecessor,
        "resulting_supply_temperature_c",
        "resulting_supply_temperature_c",
    );

    let local_flags = [
        "cp415_retained_supply_temperature_owned_read",
        "supply_temperature_for_humidity_ratio_inversion_read",
        "cp415_retained_supply_enthalpy_owned_read",
        "supply_enthalpy_for_humidity_ratio_inversion_read",
        "psychrometric_supply_humidity_ratio_evaluated",
        "supply_humidity_ratio_assignment_performed",
    ];
    for field in local_flags {
        assert_eq!(latest[field], active, "CP416 {field}");
    }
    if active {
        assert_same_bits(
            latest,
            predecessor,
            "supply_temperature_c",
            "resulting_supply_temperature_c",
        );
        assert_same_bits(
            latest,
            predecessor,
            "supply_enthalpy_j_per_kg",
            "resulting_supply_enthalpy_j_per_kg",
        );
        let temperature = f64::from_bits(
            ieee_bits(latest, "supply_temperature_c").expect("active CP416 temperature"),
        );
        let enthalpy = f64::from_bits(
            ieee_bits(latest, "supply_enthalpy_j_per_kg").expect("active CP416 enthalpy"),
        );
        let expected = energyplus_psy_w_fn_tdb_h(temperature, enthalpy).to_bits();
        for field in [
            "psychrometric_supply_humidity_ratio",
            "assigned_supply_humidity_ratio",
            "resulting_supply_humidity_ratio",
        ] {
            assert_eq!(ieee_bits(latest, field), Some(expected), "CP416 {field}");
        }
    } else {
        for field in [
            "supply_temperature_c",
            "supply_enthalpy_j_per_kg",
            "psychrometric_supply_humidity_ratio",
            "assigned_supply_humidity_ratio",
        ] {
            assert!(latest[field].is_null(), "inactive CP416 {field}");
            assert!(latest[format!("{field}_ieee_bits")].is_null());
        }
        assert_same_bits(
            latest,
            predecessor,
            "resulting_supply_humidity_ratio",
            "resulting_supply_humidity_ratio",
        );
    }

    let object = latest.as_object().expect("CP416 latest object");
    assert_eq!(object.len(), 192, "CP416 JSON width");
    assert_eq!(
        object
            .keys()
            .filter(|key| key.ends_with("_ieee_bits"))
            .count(),
        47,
        "CP416 IEEE sidecar count",
    );
}

fn sum_predecessor_indices(values: &[Value], include: impl Fn(usize) -> bool) -> u64 {
    let mut logical = 0usize;
    let mut total = 0u64;
    for predecessor_index in 0..30 {
        let width = 1 + usize::from(SPLIT_PREDECESSOR_INDICES.contains(&predecessor_index));
        if include(predecessor_index) {
            total += sum(&values[logical..logical + width]);
        }
        logical += width;
    }
    assert_eq!(logical, 36);
    total
}

fn assert_same_bits(next: &Value, previous: &Value, next_field: &str, previous_field: &str) {
    assert_eq!(
        ieee_bits(next, next_field),
        ieee_bits(previous, previous_field),
        "CP416 {next_field} lineage",
    );
}

fn ieee_bits(value: &Value, field: &str) -> Option<u64> {
    let sidecar = &value[format!("{field}_ieee_bits")];
    if sidecar.is_null() {
        return None;
    }
    let text = sidecar.as_str().expect("CP416 IEEE sidecar string");
    let hex = text.strip_prefix("0x").expect("CP416 IEEE sidecar prefix");
    Some(u64::from_str_radix(hex, 16).expect("CP416 IEEE sidecar bits"))
}

fn boolean(value: &Value, field: &str) -> bool {
    value[field].as_bool().expect("CP416 boolean")
}

fn count(value: &Value, field: &str) -> u64 {
    count_value(&value[field])
}

fn count_value(value: &Value) -> u64 {
    value.as_u64().expect("CP416 unsigned count")
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
