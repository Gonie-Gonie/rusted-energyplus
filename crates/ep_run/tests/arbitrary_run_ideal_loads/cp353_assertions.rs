//! CP353-to-CP356 run-summary assertions.

use serde_json::{Map, Value};

const KEY: &str = "purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_lifecycle";
const CP354_KEY: &str =
    "purchased_air_calc_cooling_constant_shr_supply_humidity_ratio_overdrying_limit_lifecycle";
const CP355_KEY: &str =
    "purchased_air_calc_cooling_constant_shr_supply_humidity_ratio_minimum_limit_lifecycle";
const CP356_KEY: &str =
    "purchased_air_calc_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit_lifecycle";
const CP352_KEY: &str = "purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_lifecycle";
const ORDER: [&str; 5] = [
    "read-local-supply-enthalpy-for-constant-sensible-heat-ratio-overdrying-limit-maximum",
    "read-purchased-air-supply-temperature-for-constant-sensible-heat-ratio-overdrying-limit-enthalpy",
    "evaluate-psy-h-fn-tdb-w-at-minimum-humidity-ratio-for-constant-sensible-heat-ratio-overdrying-limit",
    "apply-source-shaped-two-argument-maximum-for-constant-sensible-heat-ratio-overdrying-limit",
    "assign-local-supply-enthalpy-for-constant-sensible-heat-ratio-overdrying-limit",
];
const CP354_ORDER: [&str; 6] = [
    "read-purchased-air-supply-humidity-ratio-for-constant-sensible-heat-ratio-overdrying-limit-minimum",
    "read-purchased-air-supply-temperature-for-constant-sensible-heat-ratio-humidity-ratio-inversion",
    "read-local-supply-enthalpy-for-constant-sensible-heat-ratio-humidity-ratio-inversion",
    "evaluate-psy-w-fn-tdb-h-for-constant-sensible-heat-ratio-overdrying-limit",
    "apply-source-shaped-two-argument-minimum-for-constant-sensible-heat-ratio-overdrying-limit",
    "assign-purchased-air-supply-humidity-ratio-for-constant-sensible-heat-ratio-overdrying-limit",
];
const CP355_ORDER: [&str; 4] = [
    "read-purchased-air-supply-humidity-ratio-for-constant-sensible-heat-ratio-minimum-limit-maximum",
    "read-purchased-air-minimum-cooling-supply-air-humidity-ratio-for-constant-sensible-heat-ratio-minimum-limit-maximum",
    "apply-source-shaped-two-argument-maximum-for-constant-sensible-heat-ratio-minimum-limit",
    "assign-purchased-air-supply-humidity-ratio-for-constant-sensible-heat-ratio-minimum-limit",
];
const CP356_ORDER: [&str; 4] = [
    "read-purchased-air-supply-humidity-ratio-for-constant-sensible-heat-ratio-mixed-air-limit-minimum",
    "read-purchased-air-mixed-air-humidity-ratio-for-constant-sensible-heat-ratio-mixed-air-limit-minimum",
    "apply-source-shaped-two-argument-minimum-for-constant-sensible-heat-ratio-mixed-air-limit",
    "assign-purchased-air-supply-humidity-ratio-for-constant-sensible-heat-ratio-mixed-air-limit",
];

const DIRECT_FIXTURE_SUPPLY_HUMIDITY_RATIO_BITS: u64 = 0x3f5d_aac3_b48c_9d41;

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let cp352 = &runtime[CP352_KEY];
    let cp353 = &runtime[KEY];
    assert_eq!(
        cp353["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2221"
    );
    assert_eq!(
        cp353["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2222"
    );
    assert_eq!(
        cp353["latest"]["source_order"]
            .as_array()
            .expect("CP353 source order"),
        &ORDER
    );
    for (cp353_field, cp352_field) in [
        (
            "predecessor_dehumidification_control_type",
            "predecessor_dehumidification_control_type",
        ),
        (
            "predecessor_dehumidification_control_none_case_completed_skip",
            "dehumidification_control_none_case_completed_skip",
        ),
        (
            "predecessor_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_executed",
            "dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_executed",
        ),
        (
            "predecessor_dehumidification_control_humidistat_case_selected_skip",
            "dehumidification_control_humidistat_case_selected_skip",
        ),
        (
            "predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip",
            "dehumidification_control_constant_supply_humidity_ratio_case_selected_skip",
        ),
    ] {
        assert_eq!(
            cp353["latest"][cp353_field], cp352["latest"][cp352_field],
            "CP353 must retain immediate CP352 lineage"
        );
    }
    assert_eq!(
        cp353["latest"]["dehumidification_control_none_case_completed_skip"],
        true
    );
    assert_eq!(
        cp353["latest"]["dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_executed"],
        false
    );
    for field in [
        "supply_enthalpy_for_overdrying_limit_maximum_read",
        "supply_temperature_for_minimum_humidity_ratio_enthalpy_read",
        "psychrometric_minimum_supply_enthalpy_evaluated",
        "source_shaped_two_argument_maximum_evaluated",
        "supply_enthalpy_assignment_performed",
    ] {
        assert_eq!(cp353["latest"][field], false, "{field}");
    }
    for field in [
        "supply_enthalpy_before_overdrying_limit_j_per_kg",
        "supply_temperature_c",
        "psychrometric_minimum_supply_enthalpy_j_per_kg",
        "maximum_supply_enthalpy_j_per_kg",
        "assigned_supply_enthalpy_j_per_kg",
        "resulting_supply_enthalpy_j_per_kg",
    ] {
        assert!(cp353["latest"][field].is_null(), "{field}");
        assert!(
            cp353["latest"][format!("{field}_ieee_bits")].is_null(),
            "{field} bits"
        );
    }
    assert_eq!(
        cp353["dehumidification_control_none_case_completed_skip_count"],
        cp352["dehumidification_control_none_case_completed_skip_count"]
    );
    assert_eq!(
        cp353["dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_count"],
        0
    );
    for field in [
        "source_site_execution_count",
        "supply_enthalpy_for_overdrying_limit_maximum_read_count",
        "supply_temperature_for_minimum_humidity_ratio_enthalpy_read_count",
        "psychrometric_minimum_supply_enthalpy_evaluation_count",
        "source_shaped_two_argument_maximum_evaluation_count",
        "supply_enthalpy_assignment_write_count",
    ] {
        assert_eq!(cp353[field], 0, "{field}");
    }
    assert!(
        runtime["purchased_air_calc_cooling_positive_supply_enthalpy_assignment_lifecycle"]
            ["latest"]["supply_enthalpy_j_per_kg_ieee_bits"]
            .as_str()
            .is_some(),
        "CP353 evidence must not replace numerical supply enthalpy"
    );
    assert_cp354(runtime, cp353);
    assert_cp356_numerical_nonfeed(runtime, results);
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(KEY));
    assert!(
        runtime[KEY].is_null(),
        "non-direct runtime must not publish CP353 evidence"
    );
    assert!(runtime.contains_key(CP354_KEY));
    assert!(
        runtime[CP354_KEY].is_null(),
        "non-direct runtime must not publish CP354 evidence"
    );
    assert!(runtime.contains_key(CP355_KEY));
    assert!(
        runtime[CP355_KEY].is_null(),
        "non-direct runtime must not publish CP355 evidence"
    );
    assert!(runtime.contains_key(CP356_KEY));
    assert!(
        runtime[CP356_KEY].is_null(),
        "non-direct runtime must not publish CP356 evidence"
    );
}

fn assert_cp354(runtime: &Value, cp353: &Value) {
    let cp354 = &runtime[CP354_KEY];
    assert_eq!(
        cp354["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2222"
    );
    assert_eq!(
        cp354["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2224"
    );
    assert_eq!(
        cp354["latest"]["source_order"]
            .as_array()
            .expect("CP354 source order"),
        &CP354_ORDER
    );
    for (cp354_field, cp353_field) in [
        (
            "predecessor_dehumidification_control_type",
            "predecessor_dehumidification_control_type",
        ),
        (
            "predecessor_dehumidification_control_none_case_completed_skip",
            "dehumidification_control_none_case_completed_skip",
        ),
        (
            "predecessor_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_executed",
            "dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_executed",
        ),
        (
            "predecessor_dehumidification_control_humidistat_case_selected_skip",
            "dehumidification_control_humidistat_case_selected_skip",
        ),
        (
            "predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip",
            "dehumidification_control_constant_supply_humidity_ratio_case_selected_skip",
        ),
    ] {
        assert_eq!(
            cp354["latest"][cp354_field], cp353["latest"][cp353_field],
            "CP354 must retain immediate CP353 lineage"
        );
    }
    assert_eq!(
        cp354["latest"]["dehumidification_control_none_case_completed_skip"],
        true
    );
    assert_eq!(
        cp354["latest"]["dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_overdrying_limit_executed"],
        false
    );
    for field in [
        "supply_humidity_ratio_for_overdrying_limit_minimum_read",
        "supply_temperature_for_humidity_ratio_inversion_read",
        "supply_enthalpy_for_humidity_ratio_inversion_read",
        "psychrometric_supply_humidity_ratio_evaluated",
        "source_shaped_two_argument_minimum_evaluated",
        "supply_humidity_ratio_assignment_performed",
    ] {
        assert_eq!(cp354["latest"][field], false, "{field}");
    }
    for field in [
        "supply_humidity_ratio_before_overdrying_limit",
        "supply_temperature_c",
        "supply_enthalpy_j_per_kg",
        "psychrometric_supply_humidity_ratio",
        "minimum_supply_humidity_ratio",
        "assigned_supply_humidity_ratio",
        "resulting_supply_humidity_ratio",
    ] {
        assert!(cp354["latest"][field].is_null(), "{field}");
        assert!(
            cp354["latest"][format!("{field}_ieee_bits")].is_null(),
            "{field} bits"
        );
    }
    assert_eq!(
        cp354["dehumidification_control_none_case_completed_skip_count"],
        cp353["dehumidification_control_none_case_completed_skip_count"]
    );
    assert_eq!(
        cp354["dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_overdrying_limit_count"],
        0
    );
    for field in [
        "source_site_execution_count",
        "supply_humidity_ratio_for_overdrying_limit_minimum_read_count",
        "supply_temperature_for_humidity_ratio_inversion_read_count",
        "supply_enthalpy_for_humidity_ratio_inversion_read_count",
        "psychrometric_supply_humidity_ratio_evaluation_count",
        "source_shaped_two_argument_minimum_evaluation_count",
        "supply_humidity_ratio_assignment_write_count",
    ] {
        assert_eq!(cp354[field], 0, "{field}");
    }
    assert!(
        runtime["purchased_air_calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_lifecycle"]
            ["latest"]["assigned_supply_humidity_ratio_ieee_bits"]
            .as_str()
            .is_some(),
        "CP354 evidence must not replace numerical supply humidity"
    );
    assert_cp355(runtime, cp354);
}

fn assert_cp355(runtime: &Value, cp354: &Value) {
    let cp355 = &runtime[CP355_KEY];
    assert_eq!(
        cp355["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2224"
    );
    assert_eq!(
        cp355["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2226"
    );
    assert_eq!(
        cp355["latest"]["source_order"]
            .as_array()
            .expect("CP355 source order"),
        &CP355_ORDER
    );
    for (cp355_field, cp354_field) in [
        (
            "predecessor_dehumidification_control_type",
            "predecessor_dehumidification_control_type",
        ),
        (
            "predecessor_dehumidification_control_none_case_completed_skip",
            "dehumidification_control_none_case_completed_skip",
        ),
        (
            "predecessor_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_overdrying_limit_executed",
            "dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_overdrying_limit_executed",
        ),
        (
            "predecessor_dehumidification_control_humidistat_case_selected_skip",
            "dehumidification_control_humidistat_case_selected_skip",
        ),
        (
            "predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip",
            "dehumidification_control_constant_supply_humidity_ratio_case_selected_skip",
        ),
    ] {
        assert_eq!(
            cp355["latest"][cp355_field], cp354["latest"][cp354_field],
            "CP355 must retain immediate CP354 lineage"
        );
    }
    assert_eq!(
        cp355["latest"]["dehumidification_control_none_case_completed_skip"],
        true
    );
    assert_eq!(
        cp355["latest"]["dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_minimum_limit_executed"],
        false
    );
    for field in [
        "supply_humidity_ratio_for_minimum_limit_maximum_read",
        "minimum_cooling_supply_air_humidity_ratio_for_maximum_read",
        "source_shaped_two_argument_maximum_evaluated",
        "supply_humidity_ratio_assignment_performed",
    ] {
        assert_eq!(cp355["latest"][field], false, "{field}");
    }
    for field in [
        "supply_humidity_ratio_before_minimum_limit",
        "minimum_cooling_supply_air_humidity_ratio",
        "maximum_supply_humidity_ratio",
        "assigned_supply_humidity_ratio",
        "resulting_supply_humidity_ratio",
    ] {
        assert!(cp355["latest"][field].is_null(), "{field}");
        assert!(
            cp355["latest"][format!("{field}_ieee_bits")].is_null(),
            "{field} bits"
        );
    }
    assert_eq!(
        cp355["dehumidification_control_none_case_completed_skip_count"],
        cp354["dehumidification_control_none_case_completed_skip_count"]
    );
    assert_eq!(
        cp355["dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_minimum_limit_count"],
        0
    );
    for field in [
        "source_site_execution_count",
        "supply_humidity_ratio_for_minimum_limit_maximum_read_count",
        "minimum_cooling_supply_air_humidity_ratio_for_maximum_read_count",
        "source_shaped_two_argument_maximum_evaluation_count",
        "supply_humidity_ratio_assignment_write_count",
    ] {
        assert_eq!(cp355[field], 0, "{field}");
    }
    assert!(
        runtime["purchased_air_calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_lifecycle"]
            ["latest"]["assigned_supply_humidity_ratio_ieee_bits"]
            .as_str()
            .is_some(),
        "CP355 evidence must not replace numerical supply humidity"
    );
    assert_cp356(runtime, cp355);
}

fn assert_cp356(runtime: &Value, cp355: &Value) {
    let cp356 = &runtime[CP356_KEY];
    assert_eq!(
        cp356["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2226"
    );
    assert_eq!(
        cp356["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2227"
    );
    assert_eq!(
        cp356["latest"]["source_order"]
            .as_array()
            .expect("CP356 source order"),
        &CP356_ORDER
    );
    for (cp356_field, cp355_field) in [
        (
            "predecessor_dehumidification_control_type",
            "predecessor_dehumidification_control_type",
        ),
        (
            "predecessor_dehumidification_control_none_case_completed_skip",
            "dehumidification_control_none_case_completed_skip",
        ),
        (
            "predecessor_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_minimum_limit_executed",
            "dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_minimum_limit_executed",
        ),
        (
            "predecessor_dehumidification_control_humidistat_case_selected_skip",
            "dehumidification_control_humidistat_case_selected_skip",
        ),
        (
            "predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip",
            "dehumidification_control_constant_supply_humidity_ratio_case_selected_skip",
        ),
    ] {
        assert_eq!(
            cp356["latest"][cp356_field], cp355["latest"][cp355_field],
            "CP356 must retain immediate CP355 lineage"
        );
    }
    assert_eq!(
        cp356["latest"]["dehumidification_control_none_case_completed_skip"],
        true
    );
    assert_eq!(
        cp356["latest"]["dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_mixed_air_limit_executed"],
        false
    );
    for field in [
        "supply_humidity_ratio_for_mixed_air_limit_minimum_read",
        "mixed_air_humidity_ratio_for_minimum_read",
        "source_shaped_two_argument_minimum_evaluated",
        "supply_humidity_ratio_assignment_performed",
    ] {
        assert_eq!(cp356["latest"][field], false, "{field}");
    }
    for field in [
        "supply_humidity_ratio_before_mixed_air_limit",
        "mixed_air_humidity_ratio",
        "minimum_supply_humidity_ratio",
        "assigned_supply_humidity_ratio",
        "resulting_supply_humidity_ratio",
    ] {
        assert!(cp356["latest"][field].is_null(), "{field}");
        assert!(
            cp356["latest"][format!("{field}_ieee_bits")].is_null(),
            "{field} bits"
        );
    }
    assert_eq!(
        cp356["dehumidification_control_none_case_completed_skip_count"],
        cp355["dehumidification_control_none_case_completed_skip_count"]
    );
    assert_eq!(
        cp356["dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_mixed_air_limit_count"],
        0
    );
    for field in [
        "source_site_execution_count",
        "supply_humidity_ratio_for_mixed_air_limit_minimum_read_count",
        "mixed_air_humidity_ratio_for_minimum_read_count",
        "source_shaped_two_argument_minimum_evaluation_count",
        "supply_humidity_ratio_assignment_write_count",
    ] {
        assert_eq!(cp356[field], 0, "{field}");
    }
}

fn assert_cp356_numerical_nonfeed(runtime: &Value, results: &Value) {
    let cp345_bits = runtime["purchased_air_calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_lifecycle"]
        ["latest"]["assigned_supply_humidity_ratio_ieee_bits"]
        .as_str()
        .expect("CP345 numerical humidity-owner bits");
    let cp345_bits = cp345_bits
        .strip_prefix("0x")
        .and_then(|bits| u64::from_str_radix(bits, 16).ok())
        .expect("CP345 bits must be canonical 0x-prefixed hexadecimal");
    assert_eq!(
        cp345_bits, DIRECT_FIXTURE_SUPPLY_HUMIDITY_RATIO_BITS,
        "direct fixture numerical humidity baseline changed"
    );
    let supply_humidity = results["series"]
        .as_array()
        .expect("result series")
        .iter()
        .find(|series| {
            series["key"] == "ZONE ONE INLET"
                && series["variable_name"] == "System Node Humidity Ratio"
        })
        .expect("supply-node humidity result series");
    for endpoint in ["first", "last"] {
        assert_eq!(
            supply_humidity[endpoint]
                .as_f64()
                .expect("supply-node humidity endpoint")
                .to_bits(),
            cp345_bits,
            "CP356 evidence must not replace the {endpoint} numerical supply humidity"
        );
    }
}
