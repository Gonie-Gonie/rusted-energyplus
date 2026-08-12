//! CP419 dehumidification-guard else-branch CpAir assignment assertions.

#[path = "cp420_assertions.rs"]
mod cp420_assertions;

use ep_runtime::psychrometrics::energyplus_psy_cp_air_fn_w;
use serde_json::{Map, Value, json};

const CP329_KEY: &str = "purchased_air_calc_cooling_mixed_air_call_lifecycle";
const CP418_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_lifecycle";
const CP419_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_lifecycle";
const ORDER: [&str; 3] = [
    "read-purchased-air-mixed-air-humidity-ratio-for-post-saturation-capacity-limit-dehumidification-guard-else-branch-cp-air",
    "evaluate-psy-cp-air-fn-w-for-post-saturation-capacity-limit-dehumidification-guard-else-branch-cp-air",
    "assign-local-cp-air-for-post-saturation-capacity-limit-dehumidification-guard-else-branch",
];
const PUBLIC_LOGICAL_INDICES: [usize; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 20, 21, 26, 27];
const ASSIGNMENT_LOGICAL_INDICES: [usize; 5] = [4, 7, 10, 13, 16];
const ROUTE_FIELDS: [&str; 9] = [
    "predecessor_route_counts",
    "predecessor_guard_false_fallthrough_route_counts",
    "predecessor_guard_body_entry_route_counts",
    "predecessor_supply_temperature_saturation_assignment_route_counts",
    "predecessor_supply_temperature_mixed_air_limit_route_counts",
    "predecessor_supply_humidity_ratio_assignment_route_counts",
    "predecessor_supply_enthalpy_assignment_route_counts",
    "predecessor_dehumidification_guard_else_branch_entry_route_counts",
    "dehumidification_guard_else_branch_cp_air_assignment_route_counts",
];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let owner = &runtime[CP329_KEY];
    let predecessor = &runtime[CP418_KEY];
    let lifecycle = &runtime[CP419_KEY];
    assert_eq!(
        lifecycle["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2330",
    );
    assert_eq!(
        lifecycle["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2331",
    );
    assert_eq!(lifecycle["latest"]["source_order"], json!(ORDER));
    assert_eq!(lifecycle["system"], predecessor["system"]);
    assert_eq!(lifecycle["system"], owner["system"]);
    assert_eq!(
        lifecycle["transition_count"],
        predecessor["transition_count"]
    );
    assert_eq!(lifecycle["transition_count"], owner["transition_count"]);

    for field in ROUTE_FIELDS {
        assert_eq!(array(lifecycle, field).len(), 36, "CP419 {field} width");
        for (index, value) in array(lifecycle, field).iter().enumerate() {
            if !PUBLIC_LOGICAL_INDICES.contains(&index) {
                assert_eq!(count_value(value), 0, "public CP419 {field} route {index}");
            }
        }
    }
    for field in &ROUTE_FIELDS[..8] {
        let predecessor_field =
            if *field == "predecessor_dehumidification_guard_else_branch_entry_route_counts" {
                "dehumidification_guard_else_branch_entry_route_counts"
            } else {
                field
            };
        assert_eq!(
            lifecycle[*field], predecessor[predecessor_field],
            "CP419 {field}"
        );
    }

    let routes = array(lifecycle, "predecessor_route_counts");
    let assignments = array(
        lifecycle,
        "dehumidification_guard_else_branch_cp_air_assignment_route_counts",
    );
    for index in 0..36 {
        let expected = if ASSIGNMENT_LOGICAL_INDICES.contains(&index) {
            count_value(&routes[index])
        } else {
            0
        };
        assert_eq!(
            count_value(&assignments[index]),
            expected,
            "CP419 assignment route {index}"
        );
    }
    let transitions = sum(routes);
    let assignment_count = sum(assignments);
    assert_eq!(count(lifecycle, "transition_count"), transitions);
    assert_eq!(
        count(lifecycle, "inactive_transition_count") + assignment_count,
        transitions
    );
    assert_eq!(
        count(
            lifecycle,
            "dehumidification_guard_else_branch_cp_air_assignment_count"
        ),
        assignment_count
    );
    assert_eq!(
        count(lifecycle, "source_site_execution_count"),
        assignment_count * 3
    );
    for field in [
        "cp419_psychrometric_cp_air_state_owner_count",
        "cp329_retained_mixed_air_humidity_ratio_owned_read_count",
        "mixed_air_humidity_ratio_for_cp_air_read_count",
        "psychrometric_cp_air_evaluation_count",
        "cp_air_assignment_write_count",
    ] {
        assert_eq!(count(lifecycle, field), assignment_count, "CP419 {field}");
    }
    let total_outcomes = 54usize;
    let assignment_outcomes = ASSIGNMENT_LOGICAL_INDICES.len();
    let public_outcomes = 17usize;
    assert_eq!(
        (
            total_outcomes - assignment_outcomes,
            assignment_outcomes,
            assignment_outcomes * ORDER.len(),
            public_outcomes,
            total_outcomes - public_outcomes,
        ),
        (49, 5, 15, 17, 37),
    );

    let latest = &lifecycle["latest"];
    let predecessor_latest = &predecessor["latest"];
    let owner_latest = &owner["latest"];
    assert_eq!(latest["parent_call_ordinal"], transitions);
    assert_eq!(
        latest["predecessor_cp418_resulting_supply_humidity_ratio_ieee_bits"],
        predecessor_latest["resulting_supply_humidity_ratio_ieee_bits"]
    );
    assert_eq!(
        latest["predecessor_cp418_resulting_supply_enthalpy_j_per_kg_ieee_bits"],
        predecessor_latest["resulting_supply_enthalpy_j_per_kg_ieee_bits"]
    );
    assert_eq!(
        latest["predecessor_cp418_resulting_supply_temperature_c_ieee_bits"],
        predecessor_latest["resulting_supply_temperature_c_ieee_bits"]
    );
    assert_eq!(
        latest["predecessor_post_saturation_capacity_limit_dehumidification_guard_else_branch_entered"],
        predecessor_latest["post_saturation_capacity_limit_dehumidification_guard_else_branch_entered"]
    );
    for field in [
        "resulting_supply_humidity_ratio_ieee_bits",
        "resulting_supply_enthalpy_j_per_kg_ieee_bits",
        "resulting_supply_temperature_c_ieee_bits",
    ] {
        assert_eq!(
            latest[field], predecessor_latest[field],
            "CP419 {field} preservation"
        );
    }
    let active = latest["post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_executed"]
        .as_bool()
        .expect("CP419 executed marker");
    assert_eq!(
        latest["cp329_retained_mixed_air_humidity_ratio_owned_read"],
        active
    );
    assert_eq!(latest["mixed_air_humidity_ratio_for_cp_air_read"], active);
    assert_eq!(latest["psychrometric_cp_air_evaluated"], active);
    assert_eq!(latest["cp_air_assigned"], active);
    if active {
        let humidity = latest["mixed_air_humidity_ratio_for_cp_air"]
            .as_f64()
            .expect("CP419 humidity operand");
        let expected = energyplus_psy_cp_air_fn_w(humidity);
        assert_eq!(
            latest["mixed_air_humidity_ratio_for_cp_air_ieee_bits"],
            owner_latest["mixed_air_humidity_ratio_ieee_bits"]
        );
        assert_eq!(
            bits(latest, "psychrometric_cp_air_result_j_per_kg_k"),
            expected.to_bits()
        );
        assert_eq!(bits(latest, "cp_air_j_per_kg_k"), expected.to_bits());
    } else {
        for field in [
            "mixed_air_humidity_ratio_for_cp_air",
            "psychrometric_cp_air_result_j_per_kg_k",
            "cp_air_j_per_kg_k",
        ] {
            assert!(latest[field].is_null(), "inactive CP419 {field}");
        }
    }
    let object = latest.as_object().expect("CP419 latest object");
    assert_eq!(object.len(), 234, "CP419 JSON width");
    assert_eq!(
        object
            .keys()
            .filter(|key| key.ends_with("_ieee_bits"))
            .count(),
        60
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
        assert!(!serialized.contains(forbidden), "CP419 forbids {forbidden}");
    }
    assert!(!results.to_string().contains(CP419_KEY));
    cp420_assertions::assert_direct(runtime, results);
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP419_KEY));
    assert!(
        runtime[CP419_KEY].is_null(),
        "non-direct runtime must not publish CP419 evidence"
    );
    cp420_assertions::assert_non_direct(runtime);
}

fn bits(value: &Value, field: &str) -> u64 {
    let bits = value[format!("{field}_ieee_bits")]
        .as_str()
        .expect("CP419 IEEE sidecar");
    u64::from_str_radix(bits.trim_start_matches("0x"), 16).expect("CP419 IEEE bits")
}

fn count(value: &Value, field: &str) -> u64 {
    count_value(&value[field])
}
fn count_value(value: &Value) -> u64 {
    value.as_u64().expect("CP419 unsigned count")
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
