//! CP353 run-summary assertions.

use serde_json::{Map, Value};

const KEY: &str = "purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_lifecycle";
const CP352_KEY: &str = "purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_lifecycle";
const ORDER: [&str; 5] = [
    "read-local-supply-enthalpy-for-constant-sensible-heat-ratio-overdrying-limit-maximum",
    "read-purchased-air-supply-temperature-for-constant-sensible-heat-ratio-overdrying-limit-enthalpy",
    "evaluate-psy-h-fn-tdb-w-at-minimum-humidity-ratio-for-constant-sensible-heat-ratio-overdrying-limit",
    "apply-source-shaped-two-argument-maximum-for-constant-sensible-heat-ratio-overdrying-limit",
    "assign-local-supply-enthalpy-for-constant-sensible-heat-ratio-overdrying-limit",
];

pub(super) fn assert_direct(runtime: &Value) {
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
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(KEY));
    assert!(
        runtime[KEY].is_null(),
        "non-direct runtime must not publish CP353 evidence"
    );
}
