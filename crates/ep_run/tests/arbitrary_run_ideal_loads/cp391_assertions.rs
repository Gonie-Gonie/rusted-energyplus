//! CP391 supply-enthalpy overdrying-limit assertions.

use serde_json::{Map, Value, json};

const CP390_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_lifecycle";
const CP391_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_lifecycle";
const ORDER: [&str; 5] = [
    "read-local-supply-enthalpy-for-constant-sensible-heat-ratio-overdrying-limit-maximum",
    "read-purchased-air-supply-temperature-for-constant-sensible-heat-ratio-overdrying-limit-enthalpy",
    "evaluate-psy-h-fn-tdb-w-at-minimum-humidity-ratio-for-constant-sensible-heat-ratio-overdrying-limit",
    "apply-source-shaped-two-argument-maximum-for-constant-sensible-heat-ratio-overdrying-limit",
    "assign-local-supply-enthalpy-for-constant-sensible-heat-ratio-overdrying-limit",
];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let predecessor = &runtime[CP390_KEY];
    let lifecycle = &runtime[CP391_KEY];
    assert_eq!(
        lifecycle["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2283"
    );
    assert_eq!(
        lifecycle["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2284"
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
        .expect("CP391 predecessor route counts");
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
        "dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_count",
    );
    assert_eq!(assignments, 0, "public direct CP391 assignment count");
    assert_eq!(
        assignments,
        count(
            predecessor,
            "dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_count",
        )
    );
    assert_eq!(count(lifecycle, "inactive_transition_count"), transitions);
    assert_eq!(
        count(lifecycle, "source_site_execution_count"),
        ORDER.len() as u64 * assignments
    );
    for field in [
        "supply_enthalpy_owned_read_count",
        "supply_enthalpy_for_overdrying_limit_maximum_read_count",
        "supply_temperature_owned_read_count",
        "supply_temperature_for_minimum_humidity_ratio_enthalpy_read_count",
        "psychrometric_minimum_supply_enthalpy_evaluation_count",
        "source_shaped_two_argument_maximum_evaluation_count",
        "supply_enthalpy_assignment_write_count",
    ] {
        assert_eq!(
            count(lifecycle, field),
            assignments,
            "public direct CP391 {field}"
        );
    }
    let owner_count = [5usize, 8, 11, 14, 17]
        .into_iter()
        .map(|index| routes[index].as_u64().unwrap_or_default())
        .sum::<u64>()
        + routes[18..]
            .iter()
            .map(|value| value.as_u64().unwrap_or_default())
            .sum::<u64>();
    assert_eq!(
        count(lifecycle, "cp390_supply_enthalpy_state_owner_count"),
        owner_count
    );
    assert_eq!(
        count(lifecycle, "unchanged_supply_enthalpy_preservation_count"),
        owner_count
    );

    let latest = &lifecycle["latest"];
    let prior = &predecessor["latest"];
    assert_eq!(latest["parent_call_ordinal"], transitions);
    assert_eq!(latest["system"], prior["system"]);
    assert_eq!(latest["controlled_zone"], prior["controlled_zone"]);
    assert_eq!(
        latest["predecessor_cp390_resulting_supply_enthalpy_j_per_kg_ieee_bits"],
        prior["resulting_supply_enthalpy_j_per_kg_ieee_bits"]
    );
    assert_eq!(
        latest["preexisting_supply_enthalpy_j_per_kg_ieee_bits"],
        prior["resulting_supply_enthalpy_j_per_kg_ieee_bits"]
    );
    assert_eq!(
        latest["resulting_supply_enthalpy_j_per_kg_ieee_bits"],
        prior["resulting_supply_enthalpy_j_per_kg_ieee_bits"]
    );
    assert_eq!(
        latest["predecessor_cp390_resulting_supply_temperature_c_ieee_bits"],
        prior["resulting_supply_temperature_c_ieee_bits"]
    );
    assert_eq!(
        latest["resulting_supply_temperature_c_ieee_bits"],
        prior["resulting_supply_temperature_c_ieee_bits"]
    );
    assert_eq!(
        latest["cp390_retained_supply_enthalpy_state_owned"],
        !prior["resulting_supply_enthalpy_j_per_kg_ieee_bits"].is_null()
    );
    for field in [
        "dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_executed",
        "cp390_retained_supply_enthalpy_owned_read",
        "supply_enthalpy_for_overdrying_limit_maximum_read",
        "cp390_retained_supply_temperature_owned_read",
        "supply_temperature_for_minimum_humidity_ratio_enthalpy_read",
        "psychrometric_minimum_supply_enthalpy_evaluated",
        "source_shaped_two_argument_maximum_evaluated",
        "supply_enthalpy_assignment_performed",
    ] {
        assert_eq!(latest[field], false, "public direct CP391 {field}");
    }
    for field in [
        "supply_enthalpy_before_overdrying_limit_j_per_kg",
        "supply_temperature_c",
        "psychrometric_minimum_supply_enthalpy_j_per_kg",
        "maximum_supply_enthalpy_j_per_kg",
        "assigned_supply_enthalpy_j_per_kg",
    ] {
        assert!(latest[field].is_null(), "public direct CP391 {field}");
        assert!(
            latest[format!("{field}_ieee_bits")].is_null(),
            "public direct CP391 {field} bits"
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
            "CP391 must not feed {forbidden}"
        );
    }
    assert!(
        !results.to_string().contains(CP391_KEY),
        "CP391 lifecycle must remain outside numerical result state"
    );
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP391_KEY));
    assert!(
        runtime[CP391_KEY].is_null(),
        "non-direct runtime must not publish CP391 evidence"
    );
}

fn count(value: &Value, field: &str) -> u64 {
    let count = value[field].as_u64();
    assert!(count.is_some(), "CP391 {field} count");
    count.unwrap_or_default()
}
