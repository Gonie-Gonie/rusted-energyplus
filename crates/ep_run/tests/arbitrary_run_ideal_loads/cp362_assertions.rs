//! CP362 Humidistat mixed-air-limit and numerical-nonfeed assertions.

use serde_json::{Map, Value, json};

#[path = "cp363_assertions.rs"]
mod cp363_assertions;

const CP361_KEY: &str = "purchased_air_calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_lifecycle";
const CP362_KEY: &str =
    "purchased_air_calc_cooling_humidistat_supply_humidity_ratio_mixed_air_limit_lifecycle";
const ORDER: [&str; 4] = [
    "read-purchased-air-mixed-air-humidity-ratio-for-humidistat-mixed-air-limit-minimum",
    "read-local-supply-humidity-ratio-for-dehumidification-for-humidistat-mixed-air-limit-minimum",
    "apply-source-shaped-two-argument-minimum-for-humidistat-mixed-air-limit",
    "assign-purchased-air-supply-humidity-ratio-for-humidistat-mixed-air-limit",
];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let cp361 = &runtime[CP361_KEY];
    let cp362 = &runtime[CP362_KEY];
    assert_eq!(
        cp362["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2232"
    );
    assert_eq!(
        cp362["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2233"
    );
    assert_eq!(cp362["latest"]["source_order"], json!(ORDER));
    for (cp362_field, cp361_field) in [
        (
            "predecessor_dehumidification_control_type",
            "predecessor_dehumidification_control_type",
        ),
        (
            "predecessor_dehumidification_control_none_case_completed_skip",
            "dehumidification_control_none_case_completed_skip",
        ),
        (
            "predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip",
            "dehumidification_control_constant_sensible_heat_ratio_case_completed_skip",
        ),
        (
            "predecessor_dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_executed",
            "dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_executed",
        ),
        (
            "predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip",
            "dehumidification_control_constant_supply_humidity_ratio_case_selected_skip",
        ),
        (
            "predecessor_resulting_supply_humidity_ratio_for_dehumidification",
            "resulting_supply_humidity_ratio_for_dehumidification",
        ),
        (
            "predecessor_resulting_supply_humidity_ratio_for_dehumidification_ieee_bits",
            "resulting_supply_humidity_ratio_for_dehumidification_ieee_bits",
        ),
    ] {
        assert_eq!(
            cp362["latest"][cp362_field], cp361["latest"][cp361_field],
            "CP362 must retain immediate bit-exact CP361 lineage"
        );
    }
    assert_eq!(
        cp362["latest"]["dehumidification_control_none_case_completed_skip"],
        true
    );
    assert_eq!(
        cp362["latest"]["dehumidification_control_constant_sensible_heat_ratio_case_completed_skip"],
        false
    );
    assert_eq!(
        cp362["latest"]["dehumidification_control_humidistat_supply_humidity_ratio_mixed_air_limit_executed"],
        false
    );
    assert_eq!(
        cp362["latest"]["dehumidification_control_constant_supply_humidity_ratio_case_selected_skip"],
        false
    );
    assert_eq!(
        cp362["dehumidification_control_none_case_completed_skip_count"],
        cp361["dehumidification_control_none_case_completed_skip_count"]
    );
    assert_eq!(
        cp362["dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count"],
        cp361["dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count"]
    );
    assert_eq!(
        cp362["dehumidification_control_humidistat_supply_humidity_ratio_mixed_air_limit_count"],
        cp361["dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_count"]
    );
    assert_eq!(
        cp362["dehumidification_control_humidistat_supply_humidity_ratio_mixed_air_limit_count"],
        0
    );
    for field in [
        "source_site_execution_count",
        "mixed_air_humidity_ratio_for_minimum_read_count",
        "supply_humidity_ratio_for_dehumidification_for_mixed_air_limit_minimum_read_count",
        "source_shaped_two_argument_minimum_evaluation_count",
        "supply_humidity_ratio_assignment_count",
    ] {
        assert_eq!(cp362[field], 0, "{field}");
    }
    for field in [
        "mixed_air_humidity_ratio_for_minimum_read",
        "supply_humidity_ratio_for_dehumidification_for_mixed_air_limit_minimum_read",
        "source_shaped_two_argument_minimum_evaluated",
        "supply_humidity_ratio_assignment_performed",
    ] {
        assert_eq!(cp362["latest"][field], false, "{field}");
    }
    for field in [
        "predecessor_resulting_supply_humidity_ratio_for_dehumidification",
        "predecessor_resulting_supply_humidity_ratio_for_dehumidification_ieee_bits",
        "mixed_air_humidity_ratio",
        "mixed_air_humidity_ratio_ieee_bits",
        "supply_humidity_ratio_for_dehumidification_before_mixed_air_limit",
        "supply_humidity_ratio_for_dehumidification_before_mixed_air_limit_ieee_bits",
        "minimum_supply_humidity_ratio",
        "minimum_supply_humidity_ratio_ieee_bits",
        "assigned_supply_humidity_ratio",
        "assigned_supply_humidity_ratio_ieee_bits",
        "resulting_supply_humidity_ratio",
        "resulting_supply_humidity_ratio_ieee_bits",
    ] {
        assert!(cp362["latest"][field].is_null(), "{field}");
    }
    cp363_assertions::assert_direct(runtime, results);
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP362_KEY));
    assert!(
        runtime[CP362_KEY].is_null(),
        "non-direct runtime must not publish CP362 evidence"
    );
    cp363_assertions::assert_non_direct(runtime);
}
