//! CP399 post-saturation shared-body `CpAir` assignment assertions.

use serde_json::{Map, Value, json};

const CP329_KEY: &str = "purchased_air_calc_cooling_mixed_air_call_lifecycle";
const CP398_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry_lifecycle";
const CP399_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment_lifecycle";
const ORDER: [&str; 3] = [
    "read-purchased-air-mixed-air-humidity-ratio-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-cp-air",
    "evaluate-psy-cp-air-fn-w-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-cp-air",
    "assign-local-cp-air-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-case",
];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let predecessor = &runtime[CP398_KEY];
    let owner = &runtime[CP329_KEY];
    let lifecycle = &runtime[CP399_KEY];
    assert_eq!(
        lifecycle["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2294"
    );
    assert_eq!(
        lifecycle["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2295"
    );
    assert_eq!(lifecycle["latest"]["source_order"], json!(ORDER));
    assert_eq!(lifecycle["system"], predecessor["system"]);
    assert_eq!(
        lifecycle["transition_count"],
        predecessor["transition_count"]
    );
    assert_eq!(
        lifecycle["predecessor_route_counts"],
        predecessor["predecessor_route_counts"]
    );

    let routes = lifecycle["predecessor_route_counts"]
        .as_array()
        .expect("CP399 predecessor route counts");
    assert_eq!(routes.len(), 30);
    let route = |index: usize| routes[index].as_u64().unwrap_or_default();
    let transitions = count(lifecycle, "transition_count");
    assert_eq!(
        routes
            .iter()
            .map(|value| value.as_u64().unwrap_or_default())
            .sum::<u64>(),
        transitions
    );
    let assignments = count(
        lifecycle,
        "dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment_count",
    );
    assert_eq!(route(20) + route(24), assignments);
    assert_eq!(
        assignments,
        count(
            predecessor,
            "dehumidification_control_constant_supply_humidity_ratio_case_entry_count"
        )
    );
    for (index, value) in routes.iter().enumerate() {
        if !(0..=8).contains(&index) && index != 20 && index != 24 {
            assert_eq!(
                value.as_u64().unwrap_or_default(),
                0,
                "public direct CP399 route {index}"
            );
        }
    }
    assert_eq!(
        count(lifecycle, "inactive_transition_count") + assignments,
        transitions
    );
    for field in [
        "mixed_air_humidity_ratio_read_count",
        "psychrometric_cp_air_evaluation_count",
        "cp_air_assignment_write_count",
    ] {
        assert_eq!(count(lifecycle, field), assignments, "CP399 {field}");
    }
    assert_eq!(
        count(lifecycle, "source_site_execution_count"),
        ORDER.len() as u64 * assignments
    );

    let latest = &lifecycle["latest"];
    let prior = &predecessor["latest"];
    let owner_latest = &owner["latest"];
    assert_eq!(latest["parent_call_ordinal"], transitions);
    assert_eq!(latest["system"], prior["system"]);
    assert_eq!(latest["controlled_zone"], prior["controlled_zone"]);
    assert_eq!(latest["system"], owner_latest["system"]);
    assert_eq!(latest["controlled_zone"], owner_latest["controlled_zone"]);
    assert_eq!(
        latest["predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered"],
        prior["dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered"]
    );
    let active =
        prior["dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered"]
            .as_bool() == Some(true);
    for field in [
        "dehumidification_control_none_or_constant_supply_humidity_ratio_cp_air_assignment_executed",
        "mixed_air_humidity_ratio_read",
        "psychrometric_cp_air_evaluated",
        "cp_air_assigned",
    ] {
        assert_eq!(latest[field], active, "CP399 {field}");
    }
    for suffix in ["humidity_ratio", "enthalpy_j_per_kg", "temperature_c"] {
        assert_eq!(
            latest[format!("predecessor_cp397_resulting_supply_{suffix}_ieee_bits")],
            prior[format!("predecessor_cp397_resulting_supply_{suffix}_ieee_bits")]
        );
        assert_eq!(
            latest[format!("predecessor_cp398_resulting_supply_{suffix}_ieee_bits")],
            prior[format!("resulting_supply_{suffix}_ieee_bits")]
        );
        assert_eq!(
            latest[format!("resulting_supply_{suffix}_ieee_bits")],
            prior[format!("resulting_supply_{suffix}_ieee_bits")]
        );
    }
    for field in [
        "mixed_air_humidity_ratio",
        "psychrometric_cp_air_result_j_per_kg_k",
        "cp_air_j_per_kg_k",
    ] {
        assert_eq!(latest[field].is_number(), active, "CP399 {field}");
        assert_eq!(
            latest[format!("{field}_ieee_bits")].is_string(),
            active,
            "CP399 {field} bits"
        );
    }
    if active {
        assert_eq!(
            latest["mixed_air_humidity_ratio_ieee_bits"],
            owner_latest["mixed_air_humidity_ratio_ieee_bits"],
            "CP399 must read the CP329-owned mixed-air humidity ratio"
        );
        assert_eq!(
            latest["psychrometric_cp_air_result_j_per_kg_k_ieee_bits"],
            latest["cp_air_j_per_kg_k_ieee_bits"]
        );
    }

    let latest_object = latest.as_object().expect("CP399 latest object");
    let numeric_fields = [
        "predecessor_cp397_resulting_supply_humidity_ratio",
        "predecessor_cp397_resulting_supply_enthalpy_j_per_kg",
        "predecessor_cp397_resulting_supply_temperature_c",
        "predecessor_cp398_resulting_supply_humidity_ratio",
        "predecessor_cp398_resulting_supply_enthalpy_j_per_kg",
        "predecessor_cp398_resulting_supply_temperature_c",
        "mixed_air_humidity_ratio",
        "psychrometric_cp_air_result_j_per_kg_k",
        "cp_air_j_per_kg_k",
        "resulting_supply_humidity_ratio",
        "resulting_supply_enthalpy_j_per_kg",
        "resulting_supply_temperature_c",
    ];
    assert_eq!(
        latest_object
            .keys()
            .filter(|key| key.ends_with("_ieee_bits"))
            .count(),
        numeric_fields.len()
    );
    for field in numeric_fields {
        assert!(latest_object.contains_key(field), "CP399 {field}");
        assert!(
            latest_object.contains_key(&format!("{field}_ieee_bits")),
            "CP399 {field} IEEE sidecar"
        );
    }
    for forbidden in [
        "dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered",
        "predecessor_cp396_resulting_supply_humidity_ratio",
        "zone_humidity_ratio",
        "assigned_supply_humidity_ratio",
    ] {
        assert!(
            !latest_object.contains_key(forbidden),
            "CP399 boundary must not expose {forbidden}"
        );
    }

    let serialized = lifecycle.to_string().to_ascii_lowercase();
    for forbidden in [
        "supply_node",
        "load",
        "report",
        "reconciled",
        "numerical_dto",
    ] {
        assert!(
            !serialized.contains(forbidden),
            "CP399 must not feed {forbidden}"
        );
    }
    assert!(
        !results.to_string().contains(CP399_KEY),
        "CP399 lifecycle must remain outside numerical result state"
    );
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP399_KEY));
    assert!(
        runtime[CP399_KEY].is_null(),
        "non-direct runtime must not publish CP399 evidence"
    );
}

fn count(value: &Value, field: &str) -> u64 {
    let count = value[field].as_u64();
    assert!(count.is_some(), "CP399 {field} count");
    count.unwrap_or_default()
}
