//! CP371 nested dehumidification-control Humidistat-or-None guard assertions.

use serde_json::{Map, Value, json};

#[path = "cp372_assertions.rs"]
mod cp372_assertions;

const CP370_KEY: &str = "purchased_air_calc_cooling_supply_humidity_ratio_humidification_control_humidistat_guard_lifecycle";
const CP371_KEY: &str = "purchased_air_calc_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_lifecycle";
const ORDER: [&str; 5] = [
    "read-dehumidification-control-type-for-humidistat-comparison",
    "compare-dehumidification-control-type-equal-to-humidistat",
    "read-dehumidification-control-type-for-none-comparison-after-first-false",
    "compare-dehumidification-control-type-equal-to-none",
    "enter-admitted-humidification-body-if-control-condition-satisfied",
];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let cp370 = &runtime[CP370_KEY];
    let cp371 = &runtime[CP371_KEY];
    assert_eq!(
        cp371["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2247"
    );
    assert_eq!(
        cp371["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2248"
    );
    assert_eq!(cp371["latest"]["source_order"], json!(ORDER));

    for field in [
        "system",
        "transition_count",
        "unit_off_skip_count",
        "non_cooling_skip_count",
        "positive_guard_false_fallthrough_skip_count",
        "dehumidification_control_none_case_completed_skip_count",
        "dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count",
        "dehumidification_control_humidistat_case_completed_skip_count",
        "dehumidification_control_constant_supply_humidity_ratio_case_completed_skip_count",
        "heating_on_read_count",
        "heating_on_body_entry_count",
        "heating_on_guard_false_fallthrough_count",
        "humidification_control_type_read_count",
        "humidification_control_type_humidistat_comparison_count",
        "humidification_control_body_entry_count",
        "humidification_control_guard_false_fallthrough_count",
    ] {
        assert_eq!(
            cp371[field], cp370[field],
            "CP371 carried count/identity must equal its immediate CP370 predecessor"
        );
    }
    for field in [
        "unit_body_entered",
        "predecessor_cooling_body_entered",
        "predecessor_no_outdoor_air_fallback_entered",
        "predecessor_positive_supply_mass_flow_body_entered",
        "unit_off_skipped",
        "non_cooling_skipped",
        "positive_guard_false_fallthrough_skipped",
        "predecessor_dehumidification_control_type",
        "predecessor_dehumidification_control_none_case_completed_skip",
        "predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip",
        "predecessor_dehumidification_control_humidistat_case_completed_skip",
        "predecessor_dehumidification_control_constant_supply_humidity_ratio_case_completed_skip",
        "predecessor_dehumidification_control_default_supply_humidity_ratio_case_exited_via_break",
        "dehumidification_control_none_case_completed_skip",
        "dehumidification_control_constant_sensible_heat_ratio_case_completed_skip",
        "dehumidification_control_humidistat_case_completed_skip",
        "dehumidification_control_constant_supply_humidity_ratio_case_completed_skip",
        "predecessor_heating_on_read",
        "predecessor_heating_on",
        "predecessor_cooling_supply_humidity_ratio_humidification_body_entered",
        "predecessor_heating_on_guard_false_fallthrough",
    ] {
        assert_eq!(
            cp371["latest"][field], cp370["latest"][field],
            "CP371 must retain exact CP370 lineage"
        );
    }
    for (current, predecessor) in [
        (
            "predecessor_humidification_control_type_read",
            "humidification_control_type_read",
        ),
        (
            "predecessor_humidification_control_type",
            "humidification_control_type",
        ),
        (
            "predecessor_humidification_control_type_humidistat",
            "humidification_control_type_humidistat",
        ),
        (
            "predecessor_humidification_control_body_entered",
            "humidification_control_body_entered",
        ),
        (
            "predecessor_humidification_control_guard_false_fallthrough",
            "humidification_control_guard_false_fallthrough",
        ),
    ] {
        assert_eq!(
            cp371["latest"][current], cp370["latest"][predecessor],
            "CP371 must carry the CP370 outer guard result"
        );
    }

    assert_eq!(
        cp371["latest"]["predecessor_humidification_control_type"],
        "None"
    );
    assert_eq!(
        cp371["latest"]["predecessor_humidification_control_guard_false_fallthrough"],
        true
    );
    assert_eq!(
        cp371["latest"]["dehumidification_control_type_first_read"],
        false
    );
    assert!(cp371["latest"]["first_dehumidification_control_type"].is_null());
    assert!(cp371["latest"]["dehumidification_control_type_humidistat"].is_null());
    assert_eq!(
        cp371["latest"]["dehumidification_control_type_second_read"],
        false
    );
    assert!(cp371["latest"]["second_dehumidification_control_type"].is_null());
    assert!(cp371["latest"]["dehumidification_control_type_none"].is_null());
    assert_eq!(
        cp371["latest"]["dehumidification_control_body_entered"],
        false
    );
    assert_eq!(
        cp371["latest"]["dehumidification_control_guard_false_fallthrough"],
        false
    );
    for field in [
        "dehumidification_control_type_first_read_count",
        "dehumidification_control_type_humidistat_comparison_count",
        "dehumidification_control_type_humidistat_match_count",
        "dehumidification_control_type_second_read_count",
        "dehumidification_control_type_none_comparison_count",
        "dehumidification_control_type_none_match_count",
        "dehumidification_control_body_entry_count",
        "dehumidification_control_guard_false_fallthrough_count",
        "source_site_execution_count",
    ] {
        assert_eq!(
            cp371[field], 0,
            "public direct CP371 current sites must remain zero"
        );
    }

    let latest = cp371["latest"].as_object().expect("CP371 latest object");
    for key in latest.keys() {
        assert!(!key.ends_with("_ieee_bits"), "{key}");
    }
    for forbidden in [
        "mixed_air_humidity_ratio",
        "assigned_supply_humidity_ratio",
        "resulting_supply_humidity_ratio",
        "supply_humidity_ratio",
    ] {
        assert!(!latest.contains_key(forbidden), "{forbidden}");
    }
    cp372_assertions::assert_direct(runtime, results);
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP371_KEY));
    assert!(
        runtime[CP371_KEY].is_null(),
        "non-direct runtime must not publish CP371 evidence"
    );
    cp372_assertions::assert_non_direct(runtime);
}
