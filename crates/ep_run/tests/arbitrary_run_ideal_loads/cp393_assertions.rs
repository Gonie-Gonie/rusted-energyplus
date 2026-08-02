//! CP393 constant-SHR case-break assertions.

use serde_json::{Map, Value, json};

const CP392_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment_lifecycle";
const CP393_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_break_lifecycle";
const ORDER: [&str; 1] = [
    "exit-purchased-air-post-saturation-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-case-via-break",
];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let predecessor = &runtime[CP392_KEY];
    let lifecycle = &runtime[CP393_KEY];
    assert_eq!(
        lifecycle["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2285"
    );
    assert_eq!(
        lifecycle["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2288"
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
        .expect("CP393 predecessor route counts");
    assert_eq!(routes.len(), 30);
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
        "dehumidification_control_constant_sensible_heat_ratio_case_break_count",
    );
    assert_eq!(breaks, 0, "public direct CP393 break count");
    assert_eq!(
        breaks,
        count(
            predecessor,
            "dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment_count",
        )
    );
    assert_eq!(
        routes[18].as_u64().unwrap_or_default()
            + routes[22].as_u64().unwrap_or_default()
            + routes[28].as_u64().unwrap_or_default(),
        breaks
    );
    for (index, route) in routes.iter().enumerate() {
        if !(0..=8).contains(&index) && index != 20 && index != 24 {
            assert_eq!(
                route.as_u64().unwrap_or_default(),
                0,
                "public direct CP393 route {index}"
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
    for field in [
        "predecessor_dehumidification_control_type_read",
        "predecessor_dehumidification_control_type",
        "predecessor_dehumidification_control_switch_dispatched",
        "predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered",
    ] {
        assert_eq!(latest[field], prior[field], "CP393 predecessor {field}");
    }
    assert_eq!(
        latest["predecessor_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment_executed"],
        prior["dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment_executed"]
    );
    for (cp393_predecessor, cp392_result, cp393_result) in [
        (
            "predecessor_cp392_resulting_supply_humidity_ratio_ieee_bits",
            "resulting_supply_humidity_ratio_ieee_bits",
            "resulting_supply_humidity_ratio_ieee_bits",
        ),
        (
            "predecessor_cp392_resulting_supply_enthalpy_j_per_kg_ieee_bits",
            "resulting_supply_enthalpy_j_per_kg_ieee_bits",
            "resulting_supply_enthalpy_j_per_kg_ieee_bits",
        ),
        (
            "predecessor_cp392_resulting_supply_temperature_c_ieee_bits",
            "resulting_supply_temperature_c_ieee_bits",
            "resulting_supply_temperature_c_ieee_bits",
        ),
    ] {
        assert_eq!(latest[cp393_predecessor], prior[cp392_result]);
        assert_eq!(latest[cp393_result], prior[cp392_result]);
    }
    assert_eq!(
        latest["dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break"],
        false
    );

    let latest_object = latest.as_object().expect("CP393 latest object");
    let numeric_fields = [
        "predecessor_cp392_resulting_supply_humidity_ratio",
        "predecessor_cp392_resulting_supply_enthalpy_j_per_kg",
        "predecessor_cp392_resulting_supply_temperature_c",
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
        assert!(latest_object.contains_key(field), "CP393 {field}");
        assert!(
            latest_object.contains_key(&format!("{field}_ieee_bits")),
            "CP393 {field} IEEE sidecar"
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
            "CP393 must not feed {forbidden}"
        );
    }
    assert!(
        !results.to_string().contains(CP393_KEY),
        "CP393 lifecycle must remain outside numerical result state"
    );
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP393_KEY));
    assert!(
        runtime[CP393_KEY].is_null(),
        "non-direct runtime must not publish CP393 evidence"
    );
}

fn count(value: &Value, field: &str) -> u64 {
    let count = value[field].as_u64();
    assert!(count.is_some(), "CP393 {field} count");
    count.unwrap_or_default()
}
