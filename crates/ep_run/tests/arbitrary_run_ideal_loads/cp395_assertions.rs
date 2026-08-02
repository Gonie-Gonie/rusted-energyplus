//! CP395 post-saturation Humidistat supply-humidity-ratio assignment assertions.

use serde_json::{Map, Value, json};

#[path = "cp396_assertions.rs"]
mod cp396_assertions;

const CP394_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry_lifecycle";
const CP395_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment_lifecycle";
const ORDER: [&str; 4] = [
    "read-purchased-air-supply-temperature-for-humidistat-humidity-ratio-inversion",
    "read-local-supply-enthalpy-for-humidistat-humidity-ratio-inversion",
    "evaluate-psy-w-fn-tdb-h-for-humidistat-capacity-limit",
    "assign-purchased-air-supply-humidity-ratio-for-humidistat-capacity-limit",
];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let predecessor = &runtime[CP394_KEY];
    let lifecycle = &runtime[CP395_KEY];
    assert_eq!(
        lifecycle["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2288"
    );
    assert_eq!(
        lifecycle["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2289"
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
        .expect("CP395 predecessor route counts");
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
        "dehumidification_control_humidistat_supply_humidity_ratio_assignment_count",
    );
    assert_eq!(assignments, 0, "public direct CP395 assignment count");
    assert_eq!(route(19) + route(23) + route(26), assignments);
    for (index, value) in routes.iter().enumerate() {
        if !(0..=8).contains(&index) && index != 20 && index != 24 {
            assert_eq!(
                value.as_u64().unwrap_or_default(),
                0,
                "public direct CP395 route {index}"
            );
        }
    }
    assert_eq!(count(lifecycle, "inactive_transition_count"), transitions);
    assert_eq!(
        count(lifecycle, "source_site_execution_count"),
        ORDER.len() as u64 * assignments
    );
    let humidity_ratio_owners = route(18) + route(22) + route(28);
    let temperature_owners = routes[3..]
        .iter()
        .map(|value| value.as_u64().unwrap_or_default())
        .sum::<u64>();
    let enthalpy_owners = route(5)
        + route(8)
        + route(11)
        + route(14)
        + route(17)
        + routes[18..]
            .iter()
            .map(|value| value.as_u64().unwrap_or_default())
            .sum::<u64>();
    for (field, expected) in [
        (
            "cp394_supply_humidity_ratio_state_owner_count",
            humidity_ratio_owners,
        ),
        (
            "unchanged_supply_humidity_ratio_preservation_count",
            humidity_ratio_owners,
        ),
        (
            "cp394_supply_temperature_state_owner_count",
            temperature_owners,
        ),
        (
            "unchanged_supply_temperature_preservation_count",
            temperature_owners,
        ),
        ("cp394_supply_enthalpy_state_owner_count", enthalpy_owners),
        (
            "unchanged_supply_enthalpy_preservation_count",
            enthalpy_owners,
        ),
    ] {
        assert_eq!(count(lifecycle, field), expected, "CP395 {field}");
    }
    for field in [
        "supply_temperature_owned_read_count",
        "supply_temperature_for_humidity_ratio_inversion_read_count",
        "supply_enthalpy_owned_read_count",
        "supply_enthalpy_for_humidity_ratio_inversion_read_count",
        "psychrometric_supply_humidity_ratio_evaluation_count",
        "supply_humidity_ratio_assignment_write_count",
    ] {
        assert_eq!(count(lifecycle, field), assignments, "CP395 {field}");
    }

    let latest = &lifecycle["latest"];
    let prior = &predecessor["latest"];
    assert_eq!(latest["parent_call_ordinal"], transitions);
    assert_eq!(latest["system"], prior["system"]);
    assert_eq!(latest["controlled_zone"], prior["controlled_zone"]);
    assert_eq!(
        latest["predecessor_dehumidification_control_humidistat_case_entered"],
        prior["dehumidification_control_humidistat_case_entered"]
    );
    for suffix in ["humidity_ratio", "enthalpy_j_per_kg", "temperature_c"] {
        assert_eq!(
            latest[format!("predecessor_cp393_resulting_supply_{suffix}_ieee_bits")],
            prior[format!("predecessor_cp393_resulting_supply_{suffix}_ieee_bits")]
        );
        assert_eq!(
            latest[format!("predecessor_cp394_resulting_supply_{suffix}_ieee_bits")],
            prior[format!("resulting_supply_{suffix}_ieee_bits")]
        );
        assert_eq!(
            latest[format!("resulting_supply_{suffix}_ieee_bits")],
            prior[format!("resulting_supply_{suffix}_ieee_bits")]
        );
    }
    assert_eq!(
        latest["dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed"],
        false
    );
    for field in [
        "cp394_retained_supply_temperature_owned_read",
        "supply_temperature_for_humidity_ratio_inversion_read",
        "cp394_retained_supply_enthalpy_owned_read",
        "supply_enthalpy_for_humidity_ratio_inversion_read",
        "psychrometric_supply_humidity_ratio_evaluated",
        "supply_humidity_ratio_assignment_performed",
    ] {
        assert_eq!(latest[field], false, "CP395 {field}");
    }
    for field in [
        "supply_temperature_c",
        "supply_enthalpy_j_per_kg",
        "psychrometric_supply_humidity_ratio",
        "assigned_supply_humidity_ratio",
    ] {
        assert!(latest[field].is_null(), "CP395 inactive {field}");
    }

    let latest_object = latest.as_object().expect("CP395 latest object");
    let numeric_fields = [
        "predecessor_cp393_resulting_supply_humidity_ratio",
        "predecessor_cp393_resulting_supply_enthalpy_j_per_kg",
        "predecessor_cp393_resulting_supply_temperature_c",
        "predecessor_cp394_resulting_supply_humidity_ratio",
        "predecessor_cp394_resulting_supply_enthalpy_j_per_kg",
        "predecessor_cp394_resulting_supply_temperature_c",
        "supply_temperature_c",
        "supply_enthalpy_j_per_kg",
        "psychrometric_supply_humidity_ratio",
        "assigned_supply_humidity_ratio",
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
        assert!(latest_object.contains_key(field), "CP395 {field}");
        assert!(
            latest_object.contains_key(&format!("{field}_ieee_bits")),
            "CP395 {field} IEEE sidecar"
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
            "CP395 must not feed {forbidden}"
        );
    }
    assert!(
        !results.to_string().contains(CP395_KEY),
        "CP395 lifecycle must remain outside numerical result state"
    );
    cp396_assertions::assert_direct(runtime, results);
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP395_KEY));
    assert!(
        runtime[CP395_KEY].is_null(),
        "non-direct runtime must not publish CP395 evidence"
    );
    cp396_assertions::assert_non_direct(runtime);
}

fn count(value: &Value, field: &str) -> u64 {
    let count = value[field].as_u64();
    assert!(count.is_some(), "CP395 {field} count");
    count.unwrap_or_default()
}
