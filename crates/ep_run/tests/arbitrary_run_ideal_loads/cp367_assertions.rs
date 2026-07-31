//! CP367 typed-default assignment skip assertions.

use serde_json::{Map, Value, json};

#[path = "cp368_assertions.rs"]
mod cp368_assertions;

const CP366_KEY: &str =
    "purchased_air_calc_cooling_constant_supply_humidity_ratio_case_break_lifecycle";
const CP367_KEY: &str =
    "purchased_air_calc_cooling_default_supply_humidity_ratio_mixed_air_assignment_lifecycle";
const ORDER: [&str; 2] = [
    "read-purchased-air-mixed-air-humidity-ratio-for-dehumidification-control-default-assignment",
    "assign-purchased-air-supply-humidity-ratio-for-dehumidification-control-default-case",
];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let cp366 = &runtime[CP366_KEY];
    let cp367 = &runtime[CP367_KEY];
    assert_eq!(
        cp367["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2238"
    );
    assert_eq!(
        cp367["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2239"
    );
    assert_eq!(cp367["latest"]["source_order"], json!(ORDER));
    for (cp367_field, cp366_field) in [
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
            "predecessor_dehumidification_control_constant_supply_humidity_ratio_case_exited_via_break",
            "dehumidification_control_constant_supply_humidity_ratio_case_exited_via_break",
        ),
    ] {
        assert_eq!(
            cp367["latest"][cp367_field], cp366["latest"][cp366_field],
            "CP367 must retain immediate CP366 lineage"
        );
    }
    assert_eq!(
        cp367["latest"]["dehumidification_control_none_case_completed_skip"],
        true
    );
    assert_eq!(
        cp367["latest"]["dehumidification_control_constant_sensible_heat_ratio_case_completed_skip"],
        false
    );
    assert_eq!(
        cp367["latest"]["dehumidification_control_humidistat_case_completed_skip"],
        false
    );
    assert_eq!(
        cp367["latest"]["dehumidification_control_constant_supply_humidity_ratio_case_completed_skip"],
        false
    );
    assert_eq!(
        cp367["latest"]["dehumidification_control_default_supply_humidity_ratio_mixed_air_assignment_executed"],
        false
    );
    for (cp367_count, cp366_count) in [
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
            "dehumidification_control_constant_supply_humidity_ratio_case_completed_skip_count",
            "dehumidification_control_constant_supply_humidity_ratio_case_break_count",
        ),
    ] {
        assert_eq!(
            cp367[cp367_count], cp366[cp366_count],
            "CP367 route count must equal CP366 immediate predecessor count"
        );
    }
    assert_eq!(cp367["mixed_air_humidity_ratio_read_count"], 0);
    assert_eq!(cp367["supply_humidity_ratio_assignment_count"], 0);
    assert_eq!(cp367["source_site_execution_count"], 0);
    let latest = cp367["latest"].as_object().expect("CP367 latest object");
    for forbidden in [
        "mixed_air_humidity_ratio",
        "assigned_supply_humidity_ratio",
        "resulting_supply_humidity_ratio",
        "mixed_air_humidity_ratio_ieee_bits",
        "assigned_supply_humidity_ratio_ieee_bits",
        "resulting_supply_humidity_ratio_ieee_bits",
        "default_case_entered",
        "default_case_fallthrough",
    ] {
        assert!(!latest.contains_key(forbidden), "{forbidden}");
    }
    cp368_assertions::assert_direct(runtime, results);
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP367_KEY));
    assert!(
        runtime[CP367_KEY].is_null(),
        "non-direct runtime must not publish CP367 evidence"
    );
    cp368_assertions::assert_non_direct(runtime);
}
