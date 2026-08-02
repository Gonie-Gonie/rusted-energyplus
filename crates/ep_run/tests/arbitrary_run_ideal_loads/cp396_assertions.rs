//! CP396 post-saturation Humidistat case-break assertions.

use serde_json::{Map, Value, json};

const CP395_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment_lifecycle";
const CP396_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_break_lifecycle";
const ORDER: [&str; 1] = [
    "exit-purchased-air-post-saturation-capacity-limit-dehumidification-control-humidistat-case-via-break",
];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let predecessor = &runtime[CP395_KEY];
    let lifecycle = &runtime[CP396_KEY];
    assert_eq!(
        lifecycle["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2289"
    );
    assert_eq!(
        lifecycle["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2294"
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
        .expect("CP396 predecessor route counts");
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
    let breaks = count(
        lifecycle,
        "dehumidification_control_humidistat_case_break_count",
    );
    assert_eq!(breaks, 0, "public direct CP396 break count");
    assert_eq!(
        breaks,
        count(
            predecessor,
            "dehumidification_control_humidistat_supply_humidity_ratio_assignment_count",
        )
    );
    assert_eq!(route(19) + route(23) + route(26), breaks);
    for (index, value) in routes.iter().enumerate() {
        if !(0..=8).contains(&index) && index != 20 && index != 24 {
            assert_eq!(
                value.as_u64().unwrap_or_default(),
                0,
                "public direct CP396 route {index}"
            );
        }
    }
    assert_eq!(count(lifecycle, "inactive_transition_count"), transitions);
    assert_eq!(
        count(lifecycle, "source_site_execution_count"),
        ORDER.len() as u64 * breaks
    );

    let latest = &lifecycle["latest"];
    let prior = &predecessor["latest"];
    assert_eq!(latest["parent_call_ordinal"], transitions);
    assert_eq!(latest["system"], prior["system"]);
    assert_eq!(latest["controlled_zone"], prior["controlled_zone"]);
    assert_eq!(
        latest["predecessor_dehumidification_control_humidistat_case_entered"],
        prior["predecessor_dehumidification_control_humidistat_case_entered"]
    );
    assert_eq!(
        latest["predecessor_dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed"],
        prior["dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed"]
    );
    for suffix in ["humidity_ratio", "enthalpy_j_per_kg", "temperature_c"] {
        assert_eq!(
            latest[format!("predecessor_cp395_resulting_supply_{suffix}_ieee_bits")],
            prior[format!("resulting_supply_{suffix}_ieee_bits")]
        );
        assert_eq!(
            latest[format!("resulting_supply_{suffix}_ieee_bits")],
            prior[format!("resulting_supply_{suffix}_ieee_bits")]
        );
    }
    assert_eq!(
        latest["dehumidification_control_humidistat_case_exited_via_break"],
        false
    );

    let latest_object = latest.as_object().expect("CP396 latest object");
    let numeric_fields = [
        "predecessor_cp395_resulting_supply_humidity_ratio",
        "predecessor_cp395_resulting_supply_enthalpy_j_per_kg",
        "predecessor_cp395_resulting_supply_temperature_c",
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
        assert!(latest_object.contains_key(field), "CP396 {field}");
        assert!(
            latest_object.contains_key(&format!("{field}_ieee_bits")),
            "CP396 {field} IEEE sidecar"
        );
    }
    for forbidden in [
        "predecessor_cp393_resulting_supply_humidity_ratio",
        "predecessor_cp394_resulting_supply_humidity_ratio",
        "cp394_retained_supply_humidity_ratio_state_owned",
        "supply_temperature_for_humidity_ratio_inversion_read",
        "supply_enthalpy_for_humidity_ratio_inversion_read",
        "psychrometric_supply_humidity_ratio",
        "assigned_supply_humidity_ratio",
        "supply_humidity_ratio_assignment_performed",
    ] {
        assert!(
            !latest_object.contains_key(forbidden),
            "CP396 boundary must not retain CP395 field {forbidden}"
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
            "CP396 must not feed {forbidden}"
        );
    }
    assert!(
        !results.to_string().contains(CP396_KEY),
        "CP396 lifecycle must remain outside numerical result state"
    );
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP396_KEY));
    assert!(
        runtime[CP396_KEY].is_null(),
        "non-direct runtime must not publish CP396 evidence"
    );
}

fn count(value: &Value, field: &str) -> u64 {
    let count = value[field].as_u64();
    assert!(count.is_some(), "CP396 {field} count");
    count.unwrap_or_default()
}
