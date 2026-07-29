//! CP358 Humidistat case-entry and cumulative numerical-nonfeed assertions.

use serde_json::{Map, Value, json};

#[path = "cp359_assertions.rs"]
mod cp359_assertions;

const CP357_KEY: &str = "purchased_air_calc_cooling_constant_shr_case_break_lifecycle";
const CP358_KEY: &str = "purchased_air_calc_cooling_humidistat_case_entry_lifecycle";
const ORDER: [&str; 1] = ["enter-purchased-air-dehumidification-control-humidistat-case"];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let cp357 = &runtime[CP357_KEY];
    let cp358 = &runtime[CP358_KEY];
    assert_eq!(
        cp358["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2228"
    );
    assert_eq!(
        cp358["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2229"
    );
    assert_eq!(cp358["latest"]["source_order"], json!(ORDER));
    for (cp358_field, cp357_field) in [
        (
            "predecessor_dehumidification_control_type",
            "predecessor_dehumidification_control_type",
        ),
        (
            "predecessor_dehumidification_control_none_case_completed_skip",
            "dehumidification_control_none_case_completed_skip",
        ),
        (
            "predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break",
            "dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break",
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
            cp358["latest"][cp358_field], cp357["latest"][cp357_field],
            "CP358 must retain immediate CP357 lineage"
        );
    }
    assert_eq!(
        cp358["latest"]["dehumidification_control_none_case_completed_skip"],
        true
    );
    assert_eq!(
        cp358["latest"]["dehumidification_control_constant_sensible_heat_ratio_case_completed_skip"],
        false
    );
    assert_eq!(
        cp358["latest"]["dehumidification_control_humidistat_case_entered"],
        false
    );
    assert_eq!(
        cp358["latest"]["dehumidification_control_constant_supply_humidity_ratio_case_selected_skip"],
        false
    );
    assert_eq!(
        cp358["dehumidification_control_none_case_completed_skip_count"],
        cp357["dehumidification_control_none_case_completed_skip_count"]
    );
    assert_eq!(
        cp358["dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count"],
        cp357["dehumidification_control_constant_sensible_heat_ratio_case_break_count"]
    );
    assert_eq!(
        cp358["dehumidification_control_humidistat_case_entry_count"],
        cp357["dehumidification_control_humidistat_case_selected_skip_count"]
    );
    assert_eq!(
        cp358["dehumidification_control_humidistat_case_entry_count"],
        0
    );
    assert_eq!(cp358["source_site_execution_count"], 0);
    cp359_assertions::assert_direct(runtime, results);
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP358_KEY));
    assert!(
        runtime[CP358_KEY].is_null(),
        "non-direct runtime must not publish CP358 evidence"
    );
    cp359_assertions::assert_non_direct(runtime);
}
