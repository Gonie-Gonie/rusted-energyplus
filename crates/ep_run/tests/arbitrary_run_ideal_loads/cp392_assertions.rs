//! CP392 supply-humidity-ratio assignment assertions.

use serde_json::{Map, Value, json};

#[path = "cp393_assertions.rs"]
mod cp393_assertions;

const CP391_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_lifecycle";
const CP392_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment_lifecycle";
const ORDER: [&str; 4] = [
    "read-purchased-air-supply-temperature-for-constant-sensible-heat-ratio-humidity-ratio-inversion",
    "read-local-supply-enthalpy-for-constant-sensible-heat-ratio-humidity-ratio-inversion",
    "evaluate-psy-w-fn-tdb-h-for-constant-sensible-heat-ratio-overdrying-limit",
    "assign-purchased-air-supply-humidity-ratio-for-constant-sensible-heat-ratio-overdrying-limit",
];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let predecessor = &runtime[CP391_KEY];
    let lifecycle = &runtime[CP392_KEY];
    assert_eq!(
        lifecycle["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2284"
    );
    assert_eq!(
        lifecycle["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2285"
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
        .expect("CP392 predecessor route counts");
    assert_eq!(routes.len(), 30);
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
        "dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment_count",
    );
    assert_eq!(assignments, 0, "public direct CP392 assignment count");
    assert_eq!(
        assignments,
        count(
            predecessor,
            "dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_count",
        )
    );
    assert_eq!(count(lifecycle, "inactive_transition_count"), transitions);
    assert_eq!(
        count(lifecycle, "source_site_execution_count"),
        ORDER.len() as u64 * assignments
    );
    for field in [
        "supply_temperature_owned_read_count",
        "supply_temperature_for_humidity_ratio_inversion_read_count",
        "supply_enthalpy_owned_read_count",
        "supply_enthalpy_for_humidity_ratio_inversion_read_count",
        "psychrometric_supply_humidity_ratio_evaluation_count",
        "supply_humidity_ratio_assignment_write_count",
    ] {
        assert_eq!(
            count(lifecycle, field),
            assignments,
            "public direct CP392 {field}"
        );
    }
    let temperature_owner_count = routes[3..]
        .iter()
        .map(|value| value.as_u64().unwrap_or_default())
        .sum::<u64>();
    assert_eq!(
        count(lifecycle, "cp391_supply_temperature_state_owner_count"),
        temperature_owner_count
    );
    assert_eq!(
        count(lifecycle, "unchanged_supply_temperature_preservation_count"),
        temperature_owner_count
    );
    let enthalpy_owner_count = [5usize, 8, 11, 14, 17]
        .into_iter()
        .map(|index| routes[index].as_u64().unwrap_or_default())
        .sum::<u64>()
        + routes[18..]
            .iter()
            .map(|value| value.as_u64().unwrap_or_default())
            .sum::<u64>();
    assert_eq!(
        count(lifecycle, "cp391_supply_enthalpy_state_owner_count"),
        enthalpy_owner_count
    );
    assert_eq!(
        count(lifecycle, "unchanged_supply_enthalpy_preservation_count"),
        enthalpy_owner_count
    );

    let latest = &lifecycle["latest"];
    let prior = &predecessor["latest"];
    assert_eq!(latest["parent_call_ordinal"], transitions);
    assert_eq!(latest["system"], prior["system"]);
    assert_eq!(latest["controlled_zone"], prior["controlled_zone"]);
    assert_eq!(
        latest["predecessor_cp391_resulting_supply_enthalpy_j_per_kg_ieee_bits"],
        prior["resulting_supply_enthalpy_j_per_kg_ieee_bits"]
    );
    assert_eq!(
        latest["resulting_supply_enthalpy_j_per_kg_ieee_bits"],
        prior["resulting_supply_enthalpy_j_per_kg_ieee_bits"]
    );
    assert_eq!(
        latest["predecessor_cp391_resulting_supply_temperature_c_ieee_bits"],
        prior["resulting_supply_temperature_c_ieee_bits"]
    );
    assert_eq!(
        latest["resulting_supply_temperature_c_ieee_bits"],
        prior["resulting_supply_temperature_c_ieee_bits"]
    );
    assert_eq!(
        latest["cp391_retained_supply_temperature_state_owned"],
        !prior["resulting_supply_temperature_c_ieee_bits"].is_null()
    );
    assert_eq!(
        latest["cp391_retained_supply_enthalpy_state_owned"],
        !prior["resulting_supply_enthalpy_j_per_kg_ieee_bits"].is_null()
    );
    for field in [
        "dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment_executed",
        "cp391_retained_supply_temperature_owned_read",
        "supply_temperature_for_humidity_ratio_inversion_read",
        "cp391_retained_supply_enthalpy_owned_read",
        "supply_enthalpy_for_humidity_ratio_inversion_read",
        "psychrometric_supply_humidity_ratio_evaluated",
        "supply_humidity_ratio_assignment_performed",
    ] {
        assert_eq!(latest[field], false, "public direct CP392 {field}");
    }
    for field in [
        "supply_temperature_c",
        "supply_enthalpy_j_per_kg",
        "psychrometric_supply_humidity_ratio",
        "assigned_supply_humidity_ratio",
        "resulting_supply_humidity_ratio",
    ] {
        assert!(latest[field].is_null(), "public direct CP392 {field}");
        assert!(
            latest[format!("{field}_ieee_bits")].is_null(),
            "public direct CP392 {field} bits"
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
            "CP392 must not feed {forbidden}"
        );
    }
    assert!(
        !results.to_string().contains(CP392_KEY),
        "CP392 lifecycle must remain outside numerical result state"
    );
    cp393_assertions::assert_direct(runtime, results);
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP392_KEY));
    assert!(
        runtime[CP392_KEY].is_null(),
        "non-direct runtime must not publish CP392 evidence"
    );
    cp393_assertions::assert_non_direct(runtime);
}

fn count(value: &Value, field: &str) -> u64 {
    let count = value[field].as_u64();
    assert!(count.is_some(), "CP392 {field} count");
    count.unwrap_or_default()
}
