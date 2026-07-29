//! CP365 constant-supply-humidity-ratio assignment and terminal nonfeed assertions.

use serde_json::{Map, Value, json};

const CP364_KEY: &str =
    "purchased_air_calc_cooling_constant_supply_humidity_ratio_case_entry_lifecycle";
const CP365_KEY: &str =
    "purchased_air_calc_cooling_constant_supply_humidity_ratio_assignment_lifecycle";
const ORDER: [&str; 2] = [
    "read-purchased-air-minimum-cooling-supply-air-humidity-ratio-for-constant-supply-humidity-ratio-assignment",
    "assign-purchased-air-supply-humidity-ratio-for-constant-supply-humidity-ratio-case",
];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let cp364 = &runtime[CP364_KEY];
    let cp365 = &runtime[CP365_KEY];
    assert_eq!(
        cp365["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2235"
    );
    assert_eq!(
        cp365["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2236"
    );
    assert_eq!(cp365["latest"]["source_order"], json!(ORDER));
    for (cp365_field, cp364_field) in [
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
            "predecessor_dehumidification_control_humidistat_case_completed_skip",
            "dehumidification_control_humidistat_case_completed_skip",
        ),
        (
            "predecessor_dehumidification_control_constant_supply_humidity_ratio_case_entered",
            "dehumidification_control_constant_supply_humidity_ratio_case_entered",
        ),
    ] {
        assert_eq!(
            cp365["latest"][cp365_field], cp364["latest"][cp364_field],
            "CP365 must retain immediate CP364 lineage"
        );
    }
    assert_eq!(
        cp365["latest"]["dehumidification_control_none_case_completed_skip"],
        true
    );
    assert_eq!(
        cp365["latest"]["dehumidification_control_constant_sensible_heat_ratio_case_completed_skip"],
        false
    );
    assert_eq!(
        cp365["latest"]["dehumidification_control_humidistat_case_completed_skip"],
        false
    );
    assert_eq!(
        cp365["latest"]["dehumidification_control_constant_supply_humidity_ratio_assignment_executed"],
        false
    );
    for (cp365_count, cp364_count) in [
        (
            "dehumidification_control_none_case_completed_skip_count",
            "dehumidification_control_none_case_completed_skip_count",
        ),
        (
            "dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count",
            "dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count",
        ),
        (
            "dehumidification_control_humidistat_case_completed_skip_count",
            "dehumidification_control_humidistat_case_completed_skip_count",
        ),
        (
            "dehumidification_control_constant_supply_humidity_ratio_assignment_count",
            "dehumidification_control_constant_supply_humidity_ratio_case_entry_count",
        ),
    ] {
        assert_eq!(
            cp365[cp365_count], cp364[cp364_count],
            "CP365 route count must equal CP364 immediate predecessor count"
        );
    }
    for field in [
        "dehumidification_control_constant_supply_humidity_ratio_assignment_count",
        "source_site_execution_count",
        "minimum_cooling_supply_air_humidity_ratio_read_count",
        "supply_humidity_ratio_assignment_count",
    ] {
        assert_eq!(cp365[field], 0, "{field}");
    }
    for field in [
        "minimum_cooling_supply_air_humidity_ratio_read",
        "supply_humidity_ratio_assigned",
    ] {
        assert_eq!(cp365["latest"][field], false, "{field}");
    }
    for field in [
        "minimum_cooling_supply_air_humidity_ratio",
        "minimum_cooling_supply_air_humidity_ratio_ieee_bits",
        "assigned_supply_humidity_ratio",
        "assigned_supply_humidity_ratio_ieee_bits",
        "resulting_supply_humidity_ratio",
        "resulting_supply_humidity_ratio_ieee_bits",
    ] {
        assert!(cp365["latest"][field].is_null(), "{field}");
    }
    super::super::super::super::super::super::super::super::assert_numerical_nonfeed(
        runtime, results,
    );
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP365_KEY));
    assert!(
        runtime[CP365_KEY].is_null(),
        "non-direct runtime must not publish CP365 evidence"
    );
}
