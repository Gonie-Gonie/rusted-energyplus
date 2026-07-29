//! CP363 Humidistat case-break and cumulative numerical-nonfeed assertions.

use serde_json::{Map, Value, json};

const CP362_KEY: &str =
    "purchased_air_calc_cooling_humidistat_supply_humidity_ratio_mixed_air_limit_lifecycle";
const CP363_KEY: &str = "purchased_air_calc_cooling_humidistat_case_break_lifecycle";
const ORDER: [&str; 1] = ["exit-purchased-air-dehumidification-control-humidistat-case-via-break"];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let cp362 = &runtime[CP362_KEY];
    let cp363 = &runtime[CP363_KEY];
    assert_eq!(
        cp363["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2233"
    );
    assert_eq!(
        cp363["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2235"
    );
    assert_eq!(cp363["latest"]["source_order"], json!(ORDER));
    for (cp363_field, cp362_field) in [
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
            "predecessor_dehumidification_control_humidistat_supply_humidity_ratio_mixed_air_limit_executed",
            "dehumidification_control_humidistat_supply_humidity_ratio_mixed_air_limit_executed",
        ),
        (
            "predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip",
            "dehumidification_control_constant_supply_humidity_ratio_case_selected_skip",
        ),
    ] {
        assert_eq!(
            cp363["latest"][cp363_field], cp362["latest"][cp362_field],
            "CP363 must retain immediate CP362 lineage"
        );
    }
    assert_eq!(
        cp363["latest"]["dehumidification_control_none_case_completed_skip"],
        true
    );
    assert_eq!(
        cp363["latest"]["dehumidification_control_constant_sensible_heat_ratio_case_completed_skip"],
        false
    );
    assert_eq!(
        cp363["latest"]["dehumidification_control_humidistat_case_exited_via_break"],
        false
    );
    assert_eq!(
        cp363["latest"]["dehumidification_control_constant_supply_humidity_ratio_case_selected_skip"],
        false
    );
    for (cp363_count, cp362_count) in [
        (
            "dehumidification_control_none_case_completed_skip_count",
            "dehumidification_control_none_case_completed_skip_count",
        ),
        (
            "dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count",
            "dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count",
        ),
        (
            "dehumidification_control_humidistat_case_break_count",
            "dehumidification_control_humidistat_supply_humidity_ratio_mixed_air_limit_count",
        ),
        (
            "dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count",
            "dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count",
        ),
    ] {
        assert_eq!(
            cp363[cp363_count], cp362[cp362_count],
            "CP363 route count must equal CP362 immediate predecessor count"
        );
    }
    assert_eq!(
        cp363["dehumidification_control_humidistat_case_break_count"],
        0
    );
    assert_eq!(cp363["source_site_execution_count"], 0);
    assert!(cp363["latest"].is_object());
    let Some(latest) = cp363["latest"].as_object() else {
        return;
    };
    for forbidden in [
        "predecessor_resulting_supply_humidity_ratio_for_dehumidification",
        "mixed_air_humidity_ratio",
        "supply_humidity_ratio_for_dehumidification_before_mixed_air_limit",
        "minimum_supply_humidity_ratio",
        "assigned_supply_humidity_ratio",
        "resulting_supply_humidity_ratio",
        "predecessor_resulting_supply_humidity_ratio_for_dehumidification_ieee_bits",
        "mixed_air_humidity_ratio_ieee_bits",
        "supply_humidity_ratio_for_dehumidification_before_mixed_air_limit_ieee_bits",
        "minimum_supply_humidity_ratio_ieee_bits",
        "assigned_supply_humidity_ratio_ieee_bits",
        "resulting_supply_humidity_ratio_ieee_bits",
    ] {
        assert!(!latest.contains_key(forbidden), "{forbidden}");
    }
    super::super::super::super::super::super::assert_numerical_nonfeed(runtime, results);
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP363_KEY));
    assert!(
        runtime[CP363_KEY].is_null(),
        "non-direct runtime must not publish CP363 evidence"
    );
}
