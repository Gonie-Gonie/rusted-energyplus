//! CP417 psychrometric supply-enthalpy assignment assertions.

use ep_runtime::psychrometrics::energyplus_psy_h_fn_tdb_w;
use serde_json::{Map, Value, json};

const CP416_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment_lifecycle";
const CP417_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment_lifecycle";
const ORDER: [&str; 4] = [
    "read-purchased-air-supply-temperature-for-post-saturation-capacity-limit-dehumidification-enthalpy",
    "read-purchased-air-supply-humidity-ratio-for-post-saturation-capacity-limit-dehumidification-enthalpy",
    "evaluate-psy-h-fn-tdb-w-for-post-saturation-capacity-limit-dehumidification",
    "assign-local-supply-enthalpy-after-post-saturation-capacity-limit-dehumidification-humidity-ratio-assignment",
];
const SPLIT_PREDECESSOR_INDICES: [usize; 6] = [20, 21, 24, 25, 27, 29];
const PUBLIC_LOGICAL_INDICES: [usize; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 20, 21, 26, 27];
const ROUTE_FIELDS: [&str; 7] = [
    "predecessor_route_counts",
    "predecessor_guard_false_fallthrough_route_counts",
    "predecessor_guard_body_entry_route_counts",
    "predecessor_supply_temperature_saturation_assignment_route_counts",
    "predecessor_supply_temperature_mixed_air_limit_route_counts",
    "predecessor_supply_humidity_ratio_assignment_route_counts",
    "supply_enthalpy_assignment_route_counts",
];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let predecessor = &runtime[CP416_KEY];
    let lifecycle = &runtime[CP417_KEY];
    assert_eq!(
        lifecycle["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2321",
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
        assert_eq!(array(lifecycle, field).len(), 36, "CP417 {field} width");
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
            "supply_humidity_ratio_assignment_route_counts",
        ),
        (
            "supply_enthalpy_assignment_route_counts",
            "supply_humidity_ratio_assignment_route_counts",
        ),
    ] {
        assert_eq!(lifecycle[next], predecessor[previous], "CP417 {next}");
    }
    assert_route_contract(lifecycle);
    assert_public_route_firewall(lifecycle);
    assert_conceptual_contract();

    let routes = array(lifecycle, "predecessor_route_counts");
    let transitions = sum(routes);
    let assignments = sum(array(lifecycle, "supply_enthalpy_assignment_route_counts"));
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
            "predecessor_supply_humidity_ratio_assignment_count"
        ),
        assignments,
    );
    assert_eq!(
        count(predecessor, "supply_humidity_ratio_assignment_count"),
        assignments,
    );
    assert_eq!(
        count(lifecycle, "supply_enthalpy_assignment_count"),
        assignments,
    );
    assert_eq!(
        count(lifecycle, "source_site_execution_count"),
        assignments * ORDER.len() as u64,
    );
    assert_eq!(
        count(lifecycle, "cp416_supply_humidity_ratio_state_owner_count"),
        humidity_ratio_owners,
    );
    assert_eq!(
        count(
            lifecycle,
            "unchanged_supply_humidity_ratio_preservation_count"
        ),
        humidity_ratio_owners,
    );
    assert_eq!(
        count(lifecycle, "cp416_supply_enthalpy_state_owner_count"),
        enthalpy_owners,
    );
    assert_eq!(
        count(lifecycle, "unchanged_supply_enthalpy_preservation_count") + assignments,
        enthalpy_owners,
    );
    assert_eq!(
        count(lifecycle, "cp416_supply_temperature_state_owner_count"),
        temperature_owners,
    );
    assert_eq!(
        count(lifecycle, "unchanged_supply_temperature_preservation_count"),
        temperature_owners,
    );
    for field in [
        "cp417_psychrometric_supply_enthalpy_state_owner_count",
        "cp416_retained_supply_temperature_owned_read_count",
        "supply_temperature_for_enthalpy_read_count",
        "cp416_retained_supply_humidity_ratio_owned_read_count",
        "supply_humidity_ratio_for_enthalpy_read_count",
        "psychrometric_supply_enthalpy_evaluation_count",
        "supply_enthalpy_assignment_write_count",
    ] {
        assert_eq!(count(lifecycle, field), assignments, "CP417 {field}");
    }

    assert_latest_lineage(&lifecycle["latest"], &predecessor["latest"], transitions);
    let serialized = lifecycle.to_string().to_ascii_lowercase();
    for forbidden in [
        "supply_node",
        "zone_load",
        "report",
        "reconciled",
        "numerical_dto",
        "direct_zone_purchased_air_coupling_input",
    ] {
        assert!(!serialized.contains(forbidden), "CP417 forbids {forbidden}");
    }
    assert!(
        !results.to_string().contains(CP417_KEY),
        "CP417 lifecycle must remain outside numerical result state",
    );
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP417_KEY));
    assert!(
        runtime[CP417_KEY].is_null(),
        "non-direct runtime must not publish CP417 evidence",
    );
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
    let predecessor_humidity_assignments = array(
        lifecycle,
        "predecessor_supply_humidity_ratio_assignment_route_counts",
    );
    let assignments = array(lifecycle, "supply_enthalpy_assignment_route_counts");
    for index in 0..36 {
        assert_eq!(
            count_value(&predecessor_assignments[index]),
            count_value(&body[index]),
            "CP417 predecessor assignment/body route {index}",
        );
        assert_eq!(
            count_value(&predecessor_limits[index]),
            count_value(&predecessor_assignments[index]),
            "CP417 predecessor limit/assignment route {index}",
        );
        assert_eq!(
            count_value(&predecessor_humidity_assignments[index]),
            count_value(&predecessor_limits[index]),
            "CP417 predecessor humidity/limit route {index}",
        );
        assert_eq!(
            count_value(&assignments[index]),
            count_value(&predecessor_humidity_assignments[index]),
            "CP417 assignment/predecessor humidity route {index}",
        );
    }
}

fn assert_public_route_firewall(lifecycle: &Value) {
    for field in ROUTE_FIELDS {
        for (index, value) in array(lifecycle, field).iter().enumerate() {
            if !PUBLIC_LOGICAL_INDICES.contains(&index) {
                assert_eq!(count_value(value), 0, "public CP417 {field} route {index}");
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
    assert_eq!(
        (
            humidity_ratio_presence,
            enthalpy_presence,
            temperature_presence,
            humidity_ratio_presence,
            enthalpy_presence - assignment_outcomes,
            temperature_presence,
        ),
        (36, 41, 51, 36, 23, 51),
    );
}

fn assert_latest_lineage(latest: &Value, predecessor: &Value, transitions: u64) {
    assert_eq!(latest["parent_call_ordinal"], transitions);
    for (field, value) in predecessor.as_object().expect("CP416 latest object") {
        let inherited_field = match field.as_str() {
            "source" | "first_excluded_source" | "source_order" => continue,
            "resulting_supply_humidity_ratio" => {
                "predecessor_cp416_resulting_supply_humidity_ratio"
            }
            "resulting_supply_humidity_ratio_ieee_bits" => {
                "predecessor_cp416_resulting_supply_humidity_ratio_ieee_bits"
            }
            "resulting_supply_enthalpy_j_per_kg" => {
                "predecessor_cp416_resulting_supply_enthalpy_j_per_kg"
            }
            "resulting_supply_enthalpy_j_per_kg_ieee_bits" => {
                "predecessor_cp416_resulting_supply_enthalpy_j_per_kg_ieee_bits"
            }
            "resulting_supply_temperature_c" => "predecessor_cp416_resulting_supply_temperature_c",
            "resulting_supply_temperature_c_ieee_bits" => {
                "predecessor_cp416_resulting_supply_temperature_c_ieee_bits"
            }
            field => field,
        };
        assert_eq!(
            latest.get(inherited_field),
            Some(value),
            "CP417 {field} lineage"
        );
    }

    let active = boolean(
        predecessor,
        "post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment_executed",
    );
    assert_eq!(
        latest["post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment_executed"],
        active,
    );
    assert_eq!(
        latest["cp416_retained_supply_humidity_ratio_state_owned"],
        ieee_bits(predecessor, "resulting_supply_humidity_ratio").is_some(),
    );
    assert_eq!(
        latest["cp416_retained_supply_enthalpy_state_owned"],
        ieee_bits(predecessor, "resulting_supply_enthalpy_j_per_kg").is_some(),
    );
    assert_eq!(
        latest["cp416_retained_supply_temperature_state_owned"],
        ieee_bits(predecessor, "resulting_supply_temperature_c").is_some(),
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
        "resulting_supply_temperature_c",
        "resulting_supply_temperature_c",
    );

    for field in [
        "cp416_retained_supply_temperature_owned_read",
        "supply_temperature_for_enthalpy_read",
        "cp416_retained_supply_humidity_ratio_owned_read",
        "supply_humidity_ratio_for_enthalpy_read",
        "psychrometric_supply_enthalpy_evaluated",
        "supply_enthalpy_assignment_performed",
    ] {
        assert_eq!(latest[field], active, "CP417 {field}");
    }
    if active {
        assert_same_bits(
            latest,
            predecessor,
            "supply_temperature_for_enthalpy_c",
            "resulting_supply_temperature_c",
        );
        assert_same_bits(
            latest,
            predecessor,
            "supply_humidity_ratio_for_enthalpy",
            "resulting_supply_humidity_ratio",
        );
        let temperature = f64::from_bits(
            ieee_bits(latest, "supply_temperature_for_enthalpy_c")
                .expect("active CP417 temperature"),
        );
        let humidity_ratio = f64::from_bits(
            ieee_bits(latest, "supply_humidity_ratio_for_enthalpy")
                .expect("active CP417 humidity ratio"),
        );
        let expected = energyplus_psy_h_fn_tdb_w(temperature, humidity_ratio).to_bits();
        for field in [
            "psychrometric_supply_enthalpy_j_per_kg",
            "assigned_supply_enthalpy_j_per_kg",
            "resulting_supply_enthalpy_j_per_kg",
        ] {
            assert_eq!(ieee_bits(latest, field), Some(expected), "CP417 {field}");
        }
    } else {
        for field in [
            "supply_temperature_for_enthalpy_c",
            "supply_humidity_ratio_for_enthalpy",
            "psychrometric_supply_enthalpy_j_per_kg",
            "assigned_supply_enthalpy_j_per_kg",
        ] {
            assert!(latest[field].is_null(), "inactive CP417 {field}");
            assert!(latest[format!("{field}_ieee_bits")].is_null());
        }
        assert_same_bits(
            latest,
            predecessor,
            "resulting_supply_enthalpy_j_per_kg",
            "resulting_supply_enthalpy_j_per_kg",
        );
    }

    let object = latest.as_object().expect("CP417 latest object");
    assert_eq!(object.len(), 216, "CP417 JSON width");
    assert_eq!(
        object
            .keys()
            .filter(|key| key.ends_with("_ieee_bits"))
            .count(),
        54,
        "CP417 IEEE sidecar count",
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
        "CP417 {next_field} lineage",
    );
}

fn ieee_bits(value: &Value, field: &str) -> Option<u64> {
    let sidecar = &value[format!("{field}_ieee_bits")];
    if sidecar.is_null() {
        return None;
    }
    let text = sidecar.as_str().expect("CP417 IEEE sidecar string");
    let hex = text.strip_prefix("0x").expect("CP417 IEEE sidecar prefix");
    Some(u64::from_str_radix(hex, 16).expect("CP417 IEEE sidecar bits"))
}

fn boolean(value: &Value, field: &str) -> bool {
    value[field].as_bool().expect("CP417 boolean")
}

fn count(value: &Value, field: &str) -> u64 {
    count_value(&value[field])
}

fn count_value(value: &Value) -> u64 {
    value.as_u64().expect("CP417 unsigned count")
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
