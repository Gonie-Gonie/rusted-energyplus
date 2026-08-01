//! CP389 constant-SHR supply-temperature assignment assertions.

use serde_json::{Map, Value, json};

#[path = "cp390_assertions.rs"]
mod cp390_assertions;

const CP388_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_lifecycle";
const CP389_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_lifecycle";
const CP379_KEY: &str =
    "purchased_air_calc_cooling_supply_enthalpy_post_saturation_assignment_lifecycle";
const CP385_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_lifecycle";
const ORDER: [&str; 8] = [
    "read-purchased-air-mixed-air-temperature-for-post-saturation-capacity-limit-constant-sensible-heat-ratio-supply-temperature-difference-minuend",
    "read-local-cooling-sensible-output-for-post-saturation-capacity-limit-constant-sensible-heat-ratio-supply-temperature-quotient-numerator",
    "read-local-cp-air-for-post-saturation-capacity-limit-constant-sensible-heat-ratio-supply-temperature-denominator-first-factor",
    "read-retained-supply-mass-flow-rate-for-post-saturation-capacity-limit-constant-sensible-heat-ratio-supply-temperature-denominator-second-factor",
    "calculate-cp-air-times-supply-mass-flow-rate-for-post-saturation-capacity-limit-constant-sensible-heat-ratio-supply-temperature-denominator",
    "calculate-cooling-sensible-output-divided-by-air-capacity-rate-for-post-saturation-capacity-limit-constant-sensible-heat-ratio-supply-temperature-drop",
    "calculate-mixed-air-temperature-minus-sensible-temperature-drop-for-post-saturation-capacity-limit-constant-sensible-heat-ratio-supply-temperature",
    "assign-purchased-air-supply-temperature-for-post-saturation-capacity-limit-constant-sensible-heat-ratio-case",
];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let predecessor = &runtime[CP388_KEY];
    let lifecycle = &runtime[CP389_KEY];
    let temperature_owner = &runtime[CP379_KEY];
    let enthalpy_owner = &runtime[CP385_KEY];
    assert_eq!(
        lifecycle["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2279"
    );
    assert_eq!(
        lifecycle["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2281"
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
    assert_eq!(
        lifecycle["predecessor_route_counts"]
            .as_array()
            .map(Vec::len),
        Some(30)
    );

    let transitions = count(lifecycle, "transition_count");
    let assignments = count(
        lifecycle,
        "dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_count",
    );
    assert_eq!(assignments, 0, "public direct CP389 assignment count");
    assert_eq!(count(lifecycle, "inactive_transition_count"), transitions);
    assert_eq!(
        count(lifecycle, "source_site_execution_count"),
        8 * assignments
    );
    for field in [
        "mixed_air_temperature_owned_read_count",
        "cooling_sensible_output_owned_read_count",
        "cp_air_owned_read_count",
        "supply_mass_flow_rate_owned_read_count",
        "supply_mass_flow_rate_bit_corroboration_count",
        "air_capacity_rate_calculation_count",
        "sensible_temperature_drop_calculation_count",
        "supply_temperature_calculation_count",
        "supply_temperature_assignment_write_count",
    ] {
        assert_eq!(
            count(lifecycle, field),
            assignments,
            "public direct CP389 {field}"
        );
    }
    let owner_count = count(
        temperature_owner,
        "cp334_supply_temperature_mixed_air_limit_owner_count",
    ) + count(
        temperature_owner,
        "cp344_capacity_limit_supply_temperature_mixed_air_limit_owner_count",
    );
    assert_eq!(
        count(lifecycle, "cp379_supply_temperature_state_owner_count"),
        owner_count
    );
    assert_eq!(
        count(lifecycle, "unchanged_supply_temperature_preservation_count"),
        owner_count
    );

    let latest = &lifecycle["latest"];
    let prior = &predecessor["latest"];
    assert_eq!(latest["parent_call_ordinal"], transitions);
    assert_eq!(latest["system"], prior["system"]);
    assert_eq!(latest["controlled_zone"], prior["controlled_zone"]);
    assert_eq!(
        latest["predecessor_cooling_sensible_output_w_ieee_bits"],
        prior["cooling_sensible_output_w_ieee_bits"]
    );
    assert_eq!(
        latest["resulting_supply_enthalpy_j_per_kg_ieee_bits"],
        prior["resulting_supply_enthalpy_j_per_kg_ieee_bits"]
    );
    assert_eq!(
        latest["resulting_supply_enthalpy_j_per_kg_ieee_bits"],
        enthalpy_owner["latest"]["resulting_supply_enthalpy_j_per_kg_ieee_bits"]
    );
    assert_eq!(
        latest["preexisting_supply_temperature_c_ieee_bits"],
        temperature_owner["latest"]["supply_temperature_c_ieee_bits"]
    );
    assert_eq!(
        latest["resulting_supply_temperature_c_ieee_bits"],
        temperature_owner["latest"]["supply_temperature_c_ieee_bits"]
    );
    for field in [
        "dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_executed",
        "cp329_retained_mixed_air_temperature_owned_read",
        "mixed_air_temperature_read",
        "cp388_retained_cooling_sensible_output_owned_read",
        "cooling_sensible_output_read",
        "cp387_retained_cp_air_owned_read",
        "cp_air_read",
        "cp330_retained_supply_mass_flow_rate_owned_read",
        "cp329_supply_mass_flow_rate_bit_corroborated",
        "supply_mass_flow_rate_read",
        "cp_air_times_supply_mass_flow_rate_calculated",
        "cooling_sensible_output_over_air_capacity_rate_calculated",
        "supply_temperature_calculated",
        "supply_temperature_assigned",
    ] {
        assert_eq!(latest[field], false, "public direct CP389 {field}");
    }
    for field in [
        "mixed_air_temperature_c",
        "cooling_sensible_output_w",
        "cp_air_j_per_kg_k",
        "supply_mass_flow_rate_kg_per_s",
        "cp_air_times_supply_mass_flow_rate_w_per_k",
        "cooling_sensible_output_over_air_capacity_rate_k",
        "calculated_supply_temperature_c",
        "assigned_supply_temperature_c",
    ] {
        assert!(latest[field].is_null(), "public direct CP389 {field}");
        assert!(
            latest[format!("{field}_ieee_bits")].is_null(),
            "public direct CP389 {field} bits"
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
            "CP389 must not feed {forbidden}"
        );
    }
    assert!(
        !results.to_string().contains(CP389_KEY),
        "CP389 lifecycle must remain outside numerical result state"
    );
    cp390_assertions::assert_direct(runtime, results);
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP389_KEY));
    assert!(
        runtime[CP389_KEY].is_null(),
        "non-direct runtime must not publish CP389 evidence"
    );
    cp390_assertions::assert_non_direct(runtime);
}

fn count(value: &Value, field: &str) -> u64 {
    let count = value[field].as_u64();
    assert!(count.is_some(), "CP389 {field} count");
    count.unwrap_or_default()
}
