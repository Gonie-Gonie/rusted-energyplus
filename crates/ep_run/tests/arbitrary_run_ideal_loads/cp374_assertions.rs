//! CP374 humidification supply-humidity-ratio maximum-limit assertions.

use serde_json::{Map, Value, json};

#[path = "cp375_assertions.rs"]
mod cp375_assertions;

const CP373_KEY: &str = "purchased_air_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_lifecycle";
const CP374_KEY: &str = "purchased_air_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit_lifecycle";
const ORDER: [&str; 4] = [
    "read-local-supply-humidity-ratio-for-humidification-for-maximum-limit-minimum",
    "read-purchased-air-maximum-heating-supply-air-humidity-ratio-for-humidification-maximum-limit-minimum",
    "apply-source-shaped-two-argument-minimum-for-humidification-maximum-limit",
    "assign-local-supply-humidity-ratio-for-humidification-for-maximum-limit",
];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let cp373 = &runtime[CP373_KEY];
    let cp374 = &runtime[CP374_KEY];
    assert_eq!(
        cp374["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2250"
    );
    assert_eq!(
        cp374["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2251"
    );
    assert_eq!(cp374["latest"]["source_order"], json!(ORDER));

    for field in [
        "system",
        "transition_count",
        "unit_off_skip_count",
        "non_cooling_skip_count",
        "positive_guard_false_fallthrough_skip_count",
        "heating_availability_guard_false_fallthrough_count",
        "humidification_control_guard_false_fallthrough_count",
        "dehumidification_control_guard_false_fallthrough_count",
    ] {
        assert_eq!(
            cp374[field], cp373[field],
            "CP374 direct counters must retain exact CP373 lineage"
        );
    }
    for (current, predecessor) in [
        (
            "dehumidification_control_humidistat_supply_humidity_ratio_for_humidification_maximum_limit_count",
            "dehumidification_control_humidistat_supply_humidity_ratio_for_humidification_assignment_count",
        ),
        (
            "dehumidification_control_none_supply_humidity_ratio_for_humidification_maximum_limit_count",
            "dehumidification_control_none_supply_humidity_ratio_for_humidification_assignment_count",
        ),
        (
            "supply_humidity_ratio_for_humidification_assignment_count",
            "supply_humidity_ratio_for_humidification_assignment_count",
        ),
    ] {
        assert_eq!(
            cp374[current], cp373[predecessor],
            "CP374 active-route counts must derive only from CP373"
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
        "predecessor_humidification_control_type_read",
        "predecessor_humidification_control_type",
        "predecessor_humidification_control_type_humidistat",
        "predecessor_humidification_control_body_entered",
        "predecessor_humidification_control_guard_false_fallthrough",
        "predecessor_dehumidification_control_type_first_read",
        "predecessor_first_dehumidification_control_type",
        "predecessor_dehumidification_control_type_humidistat",
        "predecessor_dehumidification_control_type_second_read",
        "predecessor_second_dehumidification_control_type",
        "predecessor_dehumidification_control_type_none",
        "predecessor_dehumidification_control_body_entered",
        "predecessor_dehumidification_control_guard_false_fallthrough",
    ] {
        assert_eq!(
            cp374["latest"][field], cp373["latest"][field],
            "CP374 must retain route-independent CP373 lineage"
        );
    }
    for (current, predecessor) in [
        (
            "predecessor_dehumidification_control_humidistat_supply_humidity_ratio_for_humidification_assignment_executed",
            "dehumidification_control_humidistat_supply_humidity_ratio_for_humidification_assignment_executed",
        ),
        (
            "predecessor_dehumidification_control_none_supply_humidity_ratio_for_humidification_assignment_executed",
            "dehumidification_control_none_supply_humidity_ratio_for_humidification_assignment_executed",
        ),
        (
            "predecessor_resulting_supply_humidity_ratio_for_humidification",
            "resulting_supply_humidity_ratio_for_humidification",
        ),
        (
            "predecessor_resulting_supply_humidity_ratio_for_humidification_ieee_bits",
            "resulting_supply_humidity_ratio_for_humidification_ieee_bits",
        ),
    ] {
        assert_eq!(
            cp374["latest"][current], cp373["latest"][predecessor],
            "CP374 must carry exact CP373 assignment evidence"
        );
    }

    for field in [
        "dehumidification_control_humidistat_supply_humidity_ratio_for_humidification_maximum_limit_count",
        "dehumidification_control_none_supply_humidity_ratio_for_humidification_maximum_limit_count",
        "supply_humidity_ratio_for_humidification_for_maximum_limit_minimum_read_count",
        "maximum_heating_supply_air_humidity_ratio_for_minimum_read_count",
        "source_shaped_two_argument_minimum_evaluation_count",
        "supply_humidity_ratio_for_humidification_assignment_count",
        "source_site_execution_count",
    ] {
        assert_eq!(cp374[field], 0, "public direct CP374 {field}");
    }
    for field in [
        "dehumidification_control_humidistat_supply_humidity_ratio_for_humidification_maximum_limit_executed",
        "dehumidification_control_none_supply_humidity_ratio_for_humidification_maximum_limit_executed",
        "supply_humidity_ratio_for_humidification_for_maximum_limit_minimum_read",
        "maximum_heating_supply_air_humidity_ratio_for_minimum_read",
        "source_shaped_two_argument_minimum_evaluated",
        "supply_humidity_ratio_for_humidification_assignment_performed",
    ] {
        assert_eq!(cp374["latest"][field], false, "{field}");
    }
    for field in [
        "predecessor_resulting_supply_humidity_ratio_for_humidification",
        "supply_humidity_ratio_for_humidification_before_maximum_limit",
        "maximum_heating_supply_air_humidity_ratio",
        "minimum_supply_humidity_ratio_for_humidification",
        "assigned_supply_humidity_ratio_for_humidification",
        "resulting_supply_humidity_ratio_for_humidification",
    ] {
        assert!(cp374["latest"][field].is_null(), "{field}");
        assert!(
            cp374["latest"][format!("{field}_ieee_bits")].is_null(),
            "{field} bits"
        );
    }

    cp375_assertions::assert_direct(runtime, results);
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP374_KEY));
    assert!(
        runtime[CP374_KEY].is_null(),
        "non-direct runtime must not publish CP374 evidence"
    );
    cp375_assertions::assert_non_direct(runtime);
}
