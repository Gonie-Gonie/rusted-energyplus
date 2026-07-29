//! CP364 constant-supply-humidity-ratio case-entry and terminal nonfeed assertions.

use serde_json::{Map, Value, json};

const CP363_KEY: &str = "purchased_air_calc_cooling_humidistat_case_break_lifecycle";
const CP364_KEY: &str =
    "purchased_air_calc_cooling_constant_supply_humidity_ratio_case_entry_lifecycle";
const ORDER: [&str; 1] =
    ["enter-purchased-air-dehumidification-control-constant-supply-humidity-ratio-case"];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let cp363 = &runtime[CP363_KEY];
    let cp364 = &runtime[CP364_KEY];
    assert_eq!(
        cp364["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2234"
    );
    assert_eq!(
        cp364["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2235"
    );
    assert_eq!(cp364["latest"]["source_order"], json!(ORDER));
    for (cp364_field, cp363_field) in [
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
            "predecessor_dehumidification_control_humidistat_case_exited_via_break",
            "dehumidification_control_humidistat_case_exited_via_break",
        ),
        (
            "predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip",
            "dehumidification_control_constant_supply_humidity_ratio_case_selected_skip",
        ),
    ] {
        assert_eq!(
            cp364["latest"][cp364_field], cp363["latest"][cp363_field],
            "CP364 must retain immediate CP363 lineage"
        );
    }
    assert_eq!(
        cp364["latest"]["dehumidification_control_none_case_completed_skip"],
        true
    );
    assert_eq!(
        cp364["latest"]["dehumidification_control_constant_sensible_heat_ratio_case_completed_skip"],
        false
    );
    assert_eq!(
        cp364["latest"]["dehumidification_control_humidistat_case_completed_skip"],
        false
    );
    assert_eq!(
        cp364["latest"]["dehumidification_control_constant_supply_humidity_ratio_case_entered"],
        false
    );
    for (cp364_count, cp363_count) in [
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
            "dehumidification_control_humidistat_case_break_count",
        ),
        (
            "dehumidification_control_constant_supply_humidity_ratio_case_entry_count",
            "dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count",
        ),
    ] {
        assert_eq!(
            cp364[cp364_count], cp363[cp363_count],
            "CP364 route count must equal CP363 immediate predecessor count"
        );
    }
    assert_eq!(
        cp364["dehumidification_control_constant_supply_humidity_ratio_case_entry_count"],
        0
    );
    assert_eq!(cp364["source_site_execution_count"], 0);
    let Some(latest) = cp364["latest"].as_object() else {
        return;
    };
    for forbidden in [
        "minimum_cooling_supply_air_humidity_ratio",
        "assigned_supply_humidity_ratio",
        "resulting_supply_humidity_ratio",
        "minimum_cooling_supply_air_humidity_ratio_ieee_bits",
        "assigned_supply_humidity_ratio_ieee_bits",
        "resulting_supply_humidity_ratio_ieee_bits",
    ] {
        assert!(!latest.contains_key(forbidden), "{forbidden}");
    }
    super::super::super::super::super::super::super::assert_numerical_nonfeed(runtime, results);
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP364_KEY));
    assert!(
        runtime[CP364_KEY].is_null(),
        "non-direct runtime must not publish CP364 evidence"
    );
}
