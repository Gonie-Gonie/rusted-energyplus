//! CP390 supply-temperature mixed-air-limit assertions.

#[path = "cp391_assertions.rs"]
mod cp391_assertions;

use serde_json::{Map, Value, json};

const CP389_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_lifecycle";
const CP390_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_lifecycle";
const ORDER: [&str; 4] = [
    "read-purchased-air-supply-temperature-for-minimum",
    "read-purchased-air-mixed-air-temperature-for-minimum",
    "apply-source-shaped-two-argument-minimum",
    "assign-purchased-air-supply-temperature",
];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let predecessor = &runtime[CP389_KEY];
    let lifecycle = &runtime[CP390_KEY];
    assert_eq!(
        lifecycle["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2281"
    );
    assert_eq!(
        lifecycle["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2283"
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
        .expect("CP390 predecessor route counts");
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
        "dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_count",
    );
    assert_eq!(assignments, 0, "public direct CP390 assignment count");
    assert_eq!(
        assignments,
        count(
            predecessor,
            "dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_count",
        )
    );
    assert_eq!(count(lifecycle, "inactive_transition_count"), transitions);
    assert_eq!(
        count(lifecycle, "source_site_execution_count"),
        ORDER.len() as u64 * assignments
    );
    for field in [
        "supply_temperature_owned_read_count",
        "supply_temperature_for_minimum_read_count",
        "mixed_air_temperature_owned_read_count",
        "mixed_air_temperature_bit_corroboration_count",
        "mixed_air_temperature_for_minimum_read_count",
        "source_shaped_two_argument_minimum_evaluation_count",
        "supply_temperature_assignment_write_count",
    ] {
        assert_eq!(
            count(lifecycle, field),
            assignments,
            "public direct CP390 {field}"
        );
    }
    let owner_count = routes[3..]
        .iter()
        .map(|value| value.as_u64().unwrap_or_default())
        .sum::<u64>();
    assert_eq!(
        count(lifecycle, "cp389_supply_temperature_state_owner_count"),
        owner_count
    );
    assert_eq!(
        count(lifecycle, "unchanged_supply_temperature_preservation_count"),
        owner_count
    );
    assert_eq!(
        owner_count,
        count(predecessor, "cp379_supply_temperature_state_owner_count")
    );

    let latest = &lifecycle["latest"];
    let prior = &predecessor["latest"];
    assert_eq!(latest["parent_call_ordinal"], transitions);
    assert_eq!(latest["system"], prior["system"]);
    assert_eq!(latest["controlled_zone"], prior["controlled_zone"]);
    assert_eq!(
        latest["predecessor_resulting_supply_temperature_c_ieee_bits"],
        prior["resulting_supply_temperature_c_ieee_bits"]
    );
    assert_eq!(
        latest["predecessor_resulting_supply_enthalpy_j_per_kg_ieee_bits"],
        prior["resulting_supply_enthalpy_j_per_kg_ieee_bits"]
    );
    assert_eq!(
        latest["resulting_supply_enthalpy_j_per_kg_ieee_bits"],
        prior["resulting_supply_enthalpy_j_per_kg_ieee_bits"]
    );
    assert_eq!(
        latest["preexisting_supply_temperature_c_ieee_bits"],
        prior["resulting_supply_temperature_c_ieee_bits"]
    );
    assert_eq!(
        latest["resulting_supply_temperature_c_ieee_bits"],
        prior["resulting_supply_temperature_c_ieee_bits"]
    );
    assert_eq!(
        latest["cp389_retained_supply_temperature_state_owned"],
        !prior["resulting_supply_temperature_c_ieee_bits"].is_null()
    );
    for field in [
        "dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_executed",
        "cp389_retained_supply_temperature_owned_read",
        "supply_temperature_for_minimum_read",
        "cp329_retained_mixed_air_temperature_owned_read",
        "cp389_mixed_air_temperature_bit_corroborated",
        "mixed_air_temperature_for_minimum_read",
        "source_shaped_two_argument_minimum_evaluated",
        "supply_temperature_assignment_performed",
    ] {
        assert_eq!(latest[field], false, "public direct CP390 {field}");
    }
    for field in [
        "supply_temperature_before_mixed_air_limit_c",
        "mixed_air_temperature_c",
        "minimum_supply_temperature_c",
        "assigned_supply_temperature_c",
    ] {
        assert!(latest[field].is_null(), "public direct CP390 {field}");
        assert!(
            latest[format!("{field}_ieee_bits")].is_null(),
            "public direct CP390 {field} bits"
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
            "CP390 must not feed {forbidden}"
        );
    }
    assert!(
        !results.to_string().contains(CP390_KEY),
        "CP390 lifecycle must remain outside numerical result state"
    );
    cp391_assertions::assert_direct(runtime, results);
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP390_KEY));
    assert!(
        runtime[CP390_KEY].is_null(),
        "non-direct runtime must not publish CP390 evidence"
    );
    cp391_assertions::assert_non_direct(runtime);
}

fn count(value: &Value, field: &str) -> u64 {
    let count = value[field].as_u64();
    assert!(count.is_some(), "CP390 {field} count");
    count.unwrap_or_default()
}
