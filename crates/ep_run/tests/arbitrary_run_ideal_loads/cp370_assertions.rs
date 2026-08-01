//! CP370 Cooling humidification-control Humidistat-guard assertions.

use serde_json::{Map, Value, json};

#[path = "cp371_assertions.rs"]
mod cp371_assertions;

const CP369_KEY: &str = "purchased_air_calc_cooling_supply_humidity_ratio_humidification_heating_availability_guard_lifecycle";
const CP370_KEY: &str = "purchased_air_calc_cooling_supply_humidity_ratio_humidification_control_humidistat_guard_lifecycle";
const ORDER: [&str; 3] = [
    "read-purchased-air-humidification-control-type-for-cooling-supply-humidity-ratio-humidification-guard",
    "compare-purchased-air-humidification-control-type-equal-to-humidistat",
    "enter-cooling-supply-humidity-ratio-humidification-control-body-if-humidistat",
];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let cp369 = &runtime[CP369_KEY];
    let cp370 = &runtime[CP370_KEY];
    assert_eq!(
        cp370["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2246"
    );
    assert_eq!(
        cp370["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2247"
    );
    assert_eq!(cp370["latest"]["source_order"], json!(ORDER));

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
    ] {
        assert_eq!(
            cp370[field], cp369[field],
            "CP370 count/identity must equal its immediate CP369 predecessor"
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
    ] {
        assert_eq!(
            cp370["latest"][field], cp369["latest"][field],
            "CP370 must retain immediate CP369 lineage"
        );
    }
    for (current, predecessor) in [
        ("predecessor_heating_on_read", "heating_on_read"),
        ("predecessor_heating_on", "heating_on"),
        (
            "predecessor_cooling_supply_humidity_ratio_humidification_body_entered",
            "cooling_supply_humidity_ratio_humidification_body_entered",
        ),
        (
            "predecessor_heating_on_guard_false_fallthrough",
            "heating_on_guard_false_fallthrough",
        ),
    ] {
        assert_eq!(
            cp370["latest"][current], cp369["latest"][predecessor],
            "CP370 must carry the CP369 guard result"
        );
    }

    assert_eq!(
        cp370["latest"]["dehumidification_control_none_case_completed_skip"],
        true
    );
    assert_eq!(cp370["latest"]["predecessor_heating_on"], true);
    assert_eq!(
        cp370["latest"]["predecessor_cooling_supply_humidity_ratio_humidification_body_entered"],
        true
    );
    assert_eq!(cp370["latest"]["humidification_control_type_read"], true);
    assert_eq!(cp370["latest"]["humidification_control_type"], "None");
    assert_eq!(
        cp370["latest"]["humidification_control_type_humidistat"],
        false
    );
    assert_eq!(
        cp370["latest"]["humidification_control_body_entered"],
        false
    );
    assert_eq!(
        cp370["latest"]["humidification_control_guard_false_fallthrough"],
        true
    );

    let reads = cp370["humidification_control_type_read_count"]
        .as_u64()
        .expect("CP370 control-type read count");
    let comparisons = cp370["humidification_control_type_humidistat_comparison_count"]
        .as_u64()
        .expect("CP370 Humidistat comparison count");
    let bodies = cp370["humidification_control_body_entry_count"]
        .as_u64()
        .expect("CP370 control body count");
    let fallthroughs = cp370["humidification_control_guard_false_fallthrough_count"]
        .as_u64()
        .expect("CP370 false-fallthrough count");
    assert_eq!(reads, cp370["heating_on_body_entry_count"]);
    assert_eq!(comparisons, reads);
    assert_eq!(bodies, 0);
    assert_eq!(fallthroughs, reads);
    assert_eq!(cp370["source_site_execution_count"], reads * 2 + bodies);

    let latest = cp370["latest"].as_object().expect("CP370 latest object");
    for forbidden in [
        "mixed_air_humidity_ratio",
        "assigned_supply_humidity_ratio",
        "resulting_supply_humidity_ratio",
        "supply_humidity_ratio",
        "mixed_air_humidity_ratio_ieee_bits",
        "assigned_supply_humidity_ratio_ieee_bits",
        "resulting_supply_humidity_ratio_ieee_bits",
    ] {
        assert!(!latest.contains_key(forbidden), "{forbidden}");
    }
    cp371_assertions::assert_direct(runtime, results);
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP370_KEY));
    assert!(
        runtime[CP370_KEY].is_null(),
        "non-direct runtime must not publish CP370 evidence"
    );
    cp371_assertions::assert_non_direct(runtime);
}
