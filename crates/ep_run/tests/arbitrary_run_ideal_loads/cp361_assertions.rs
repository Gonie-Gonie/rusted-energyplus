//! CP361 Humidistat local minimum-limit and numerical-nonfeed assertions.

use serde_json::{Map, Value, json};

#[path = "cp362_assertions.rs"]
mod cp362_assertions;

const CP360_KEY: &str = "purchased_air_calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_lifecycle";
const CP361_KEY: &str = "purchased_air_calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_lifecycle";
const ORDER: [&str; 4] = [
    "read-local-supply-humidity-ratio-for-dehumidification-for-humidistat-minimum-limit-maximum",
    "read-purchased-air-minimum-cooling-supply-air-humidity-ratio-for-humidistat-minimum-limit-maximum",
    "apply-source-shaped-two-argument-maximum-for-humidistat-minimum-limit",
    "assign-local-supply-humidity-ratio-for-dehumidification-for-humidistat-minimum-limit",
];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let cp360 = &runtime[CP360_KEY];
    let cp361 = &runtime[CP361_KEY];
    assert_eq!(
        cp361["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2231"
    );
    assert_eq!(
        cp361["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2232"
    );
    assert_eq!(cp361["latest"]["source_order"], json!(ORDER));
    for (cp361_field, cp360_field) in [
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
            "predecessor_dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_assignment_executed",
            "dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_assignment_executed",
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
            cp361["latest"][cp361_field], cp360["latest"][cp360_field],
            "CP361 must retain immediate bit-exact CP360 lineage"
        );
    }
    assert_eq!(
        cp361["latest"]["dehumidification_control_none_case_completed_skip"],
        true
    );
    assert_eq!(
        cp361["latest"]["dehumidification_control_constant_sensible_heat_ratio_case_completed_skip"],
        false
    );
    assert_eq!(
        cp361["latest"]["dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_executed"],
        false
    );
    assert_eq!(
        cp361["latest"]["dehumidification_control_constant_supply_humidity_ratio_case_selected_skip"],
        false
    );
    assert_eq!(
        cp361["dehumidification_control_none_case_completed_skip_count"],
        cp360["dehumidification_control_none_case_completed_skip_count"]
    );
    assert_eq!(
        cp361["dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count"],
        cp360["dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count"]
    );
    assert_eq!(
        cp361["dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_count"],
        cp360["dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_assignment_count"]
    );
    assert_eq!(
        cp361["dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_count"],
        0
    );
    for field in [
        "source_site_execution_count",
        "supply_humidity_ratio_for_dehumidification_for_minimum_limit_maximum_read_count",
        "minimum_cooling_supply_air_humidity_ratio_for_maximum_read_count",
        "source_shaped_two_argument_maximum_evaluation_count",
        "supply_humidity_ratio_for_dehumidification_assignment_count",
    ] {
        assert_eq!(cp361[field], 0, "{field}");
    }
    for field in [
        "supply_humidity_ratio_for_dehumidification_for_minimum_limit_maximum_read",
        "minimum_cooling_supply_air_humidity_ratio_for_maximum_read",
        "source_shaped_two_argument_maximum_evaluated",
        "supply_humidity_ratio_for_dehumidification_assignment_performed",
    ] {
        assert_eq!(cp361["latest"][field], false, "{field}");
    }
    for field in [
        "predecessor_resulting_supply_humidity_ratio_for_dehumidification",
        "predecessor_resulting_supply_humidity_ratio_for_dehumidification_ieee_bits",
        "supply_humidity_ratio_for_dehumidification_before_minimum_limit",
        "supply_humidity_ratio_for_dehumidification_before_minimum_limit_ieee_bits",
        "minimum_cooling_supply_air_humidity_ratio",
        "minimum_cooling_supply_air_humidity_ratio_ieee_bits",
        "maximum_supply_humidity_ratio_for_dehumidification",
        "maximum_supply_humidity_ratio_for_dehumidification_ieee_bits",
        "assigned_supply_humidity_ratio_for_dehumidification",
        "assigned_supply_humidity_ratio_for_dehumidification_ieee_bits",
        "resulting_supply_humidity_ratio_for_dehumidification",
        "resulting_supply_humidity_ratio_for_dehumidification_ieee_bits",
    ] {
        assert!(cp361["latest"][field].is_null(), "{field}");
    }
    cp362_assertions::assert_direct(runtime, results);
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP361_KEY));
    assert!(
        runtime[CP361_KEY].is_null(),
        "non-direct runtime must not publish CP361 evidence"
    );
    cp362_assertions::assert_non_direct(runtime);
}
