//! CP366 constant-supply-humidity-ratio case-break assertions.

use serde_json::{Map, Value, json};

#[path = "cp367_assertions.rs"]
mod cp367_assertions;

const CP365_KEY: &str =
    "purchased_air_calc_cooling_constant_supply_humidity_ratio_assignment_lifecycle";
const CP366_KEY: &str =
    "purchased_air_calc_cooling_constant_supply_humidity_ratio_case_break_lifecycle";
const ORDER: [&str; 1] =
    ["exit-purchased-air-dehumidification-control-constant-supply-humidity-ratio-case-via-break"];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let cp365 = &runtime[CP365_KEY];
    let cp366 = &runtime[CP366_KEY];
    assert_eq!(
        cp366["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2236"
    );
    assert_eq!(
        cp366["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2238"
    );
    assert_eq!(cp366["latest"]["source_order"], json!(ORDER));
    for (cp366_field, cp365_field) in [
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
            "predecessor_dehumidification_control_constant_supply_humidity_ratio_assignment_executed",
            "dehumidification_control_constant_supply_humidity_ratio_assignment_executed",
        ),
    ] {
        assert_eq!(
            cp366["latest"][cp366_field], cp365["latest"][cp365_field],
            "CP366 must retain immediate CP365 lineage"
        );
    }
    assert_eq!(
        cp366["latest"]["dehumidification_control_none_case_completed_skip"],
        true
    );
    assert_eq!(
        cp366["latest"]["dehumidification_control_constant_sensible_heat_ratio_case_completed_skip"],
        false
    );
    assert_eq!(
        cp366["latest"]["dehumidification_control_humidistat_case_completed_skip"],
        false
    );
    assert_eq!(
        cp366["latest"]["dehumidification_control_constant_supply_humidity_ratio_case_exited_via_break"],
        false
    );
    for (cp366_count, cp365_count) in [
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
            "dehumidification_control_constant_supply_humidity_ratio_case_break_count",
            "dehumidification_control_constant_supply_humidity_ratio_assignment_count",
        ),
    ] {
        assert_eq!(
            cp366[cp366_count], cp365[cp365_count],
            "CP366 route count must equal CP365 immediate predecessor count"
        );
    }
    assert_eq!(
        cp366["dehumidification_control_constant_supply_humidity_ratio_case_break_count"],
        0
    );
    assert_eq!(cp366["source_site_execution_count"], 0);
    let latest = cp366["latest"].as_object().expect("CP366 latest object");
    for forbidden in [
        "minimum_cooling_supply_air_humidity_ratio",
        "assigned_supply_humidity_ratio",
        "resulting_supply_humidity_ratio",
        "minimum_cooling_supply_air_humidity_ratio_ieee_bits",
        "assigned_supply_humidity_ratio_ieee_bits",
        "resulting_supply_humidity_ratio_ieee_bits",
        "default_case_entered",
        "default_case_fallthrough",
    ] {
        assert!(!latest.contains_key(forbidden), "{forbidden}");
    }
    cp367_assertions::assert_direct(runtime, results);
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP366_KEY));
    assert!(
        runtime[CP366_KEY].is_null(),
        "non-direct runtime must not publish CP366 evidence"
    );
    cp367_assertions::assert_non_direct(runtime);
}
