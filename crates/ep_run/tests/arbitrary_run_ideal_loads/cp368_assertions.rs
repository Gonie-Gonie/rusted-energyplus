//! CP368 typed-default case break skip and terminal nonfeed assertions.

use serde_json::{Map, Value, json};

const CP367_KEY: &str =
    "purchased_air_calc_cooling_default_supply_humidity_ratio_mixed_air_assignment_lifecycle";
const CP368_KEY: &str =
    "purchased_air_calc_cooling_default_supply_humidity_ratio_case_break_lifecycle";
const ORDER: [&str; 1] = ["exit-purchased-air-dehumidification-control-default-case-via-break"];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let cp367 = &runtime[CP367_KEY];
    let cp368 = &runtime[CP368_KEY];
    assert_eq!(
        cp368["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2239"
    );
    assert_eq!(
        cp368["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2245"
    );
    assert_eq!(cp368["latest"]["source_order"], json!(ORDER));
    for (cp368_field, cp367_field) in [
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
            "predecessor_dehumidification_control_constant_supply_humidity_ratio_case_completed_skip",
            "dehumidification_control_constant_supply_humidity_ratio_case_completed_skip",
        ),
        (
            "predecessor_dehumidification_control_default_supply_humidity_ratio_mixed_air_assignment_executed",
            "dehumidification_control_default_supply_humidity_ratio_mixed_air_assignment_executed",
        ),
    ] {
        assert_eq!(
            cp368["latest"][cp368_field], cp367["latest"][cp367_field],
            "CP368 must retain immediate CP367 lineage"
        );
    }
    assert_eq!(
        cp368["latest"]["dehumidification_control_none_case_completed_skip"],
        true
    );
    assert_eq!(
        cp368["latest"]["dehumidification_control_constant_sensible_heat_ratio_case_completed_skip"],
        false
    );
    assert_eq!(
        cp368["latest"]["dehumidification_control_humidistat_case_completed_skip"],
        false
    );
    assert_eq!(
        cp368["latest"]["dehumidification_control_constant_supply_humidity_ratio_case_completed_skip"],
        false
    );
    assert_eq!(
        cp368["latest"]["dehumidification_control_default_supply_humidity_ratio_case_exited_via_break"],
        false
    );
    for (cp368_count, cp367_count) in [
        ("transition_count", "transition_count"),
        ("unit_off_skip_count", "unit_off_skip_count"),
        ("non_cooling_skip_count", "non_cooling_skip_count"),
        (
            "positive_guard_false_fallthrough_skip_count",
            "positive_guard_false_fallthrough_skip_count",
        ),
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
            "dehumidification_control_constant_supply_humidity_ratio_case_completed_skip_count",
        ),
    ] {
        assert_eq!(
            cp368[cp368_count], cp367[cp367_count],
            "CP368 route count must equal CP367 immediate predecessor count"
        );
    }
    assert_eq!(
        cp368["dehumidification_control_default_supply_humidity_ratio_case_break_count"],
        0
    );
    assert_eq!(cp368["source_site_execution_count"], 0);
    let latest = cp368["latest"].as_object().expect("CP368 latest object");
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
    super::super::super::super::super::super::super::super::super::super::super::assert_numerical_nonfeed(
        runtime, results,
    );
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP368_KEY));
    assert!(
        runtime[CP368_KEY].is_null(),
        "non-direct runtime must not publish CP368 evidence"
    );
}
