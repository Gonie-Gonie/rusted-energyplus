//! CP375 humidification supply-humidity-ratio maximum-assignment assertions.

use serde_json::{Map, Value, json};

const CP374_KEY: &str = "purchased_air_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit_lifecycle";
const CP375_KEY: &str = "purchased_air_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_lifecycle";
const ORDER: [&str; 4] = [
    "read-purchased-air-supply-humidity-ratio-for-humidification-supply-maximum",
    "read-local-supply-humidity-ratio-for-humidification-for-supply-maximum",
    "apply-source-shaped-two-argument-maximum-for-humidification-supply-maximum",
    "assign-purchased-air-supply-humidity-ratio-for-humidification-supply-maximum",
];
const DIRECT_FIXTURE_SUPPLY_HUMIDITY_RATIO_BITS: u64 = 0x3f5d_aac3_b48c_9d41;

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let cp374 = &runtime[CP374_KEY];
    let cp375 = &runtime[CP375_KEY];
    assert_eq!(
        cp375["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2251"
    );
    assert_eq!(
        cp375["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2258"
    );
    assert_eq!(cp375["latest"]["source_order"], json!(ORDER));

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
            cp375[field], cp374[field],
            "CP375 direct counters must retain exact CP374 lineage"
        );
    }
    for (current, predecessor) in [
        (
            "dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count",
            "dehumidification_control_humidistat_supply_humidity_ratio_for_humidification_maximum_limit_count",
        ),
        (
            "dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count",
            "dehumidification_control_none_supply_humidity_ratio_for_humidification_maximum_limit_count",
        ),
        (
            "purchased_air_supply_humidity_ratio_assignment_count",
            "supply_humidity_ratio_for_humidification_assignment_count",
        ),
    ] {
        assert_eq!(
            cp375[current], cp374[predecessor],
            "CP375 active-route counts must derive only from CP374"
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
            cp375["latest"][field], cp374["latest"][field],
            "CP375 must retain route-independent CP374 lineage"
        );
    }
    for (current, predecessor) in [
        (
            "predecessor_dehumidification_control_humidistat_supply_humidity_ratio_for_humidification_maximum_limit_executed",
            "dehumidification_control_humidistat_supply_humidity_ratio_for_humidification_maximum_limit_executed",
        ),
        (
            "predecessor_dehumidification_control_none_supply_humidity_ratio_for_humidification_maximum_limit_executed",
            "dehumidification_control_none_supply_humidity_ratio_for_humidification_maximum_limit_executed",
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
            cp375["latest"][current], cp374["latest"][predecessor],
            "CP375 must carry exact CP374 maximum-limit evidence"
        );
    }

    for field in [
        "dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count",
        "dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count",
        "purchased_air_supply_humidity_ratio_for_humidification_supply_maximum_read_count",
        "supply_humidity_ratio_for_humidification_for_supply_maximum_read_count",
        "source_shaped_two_argument_maximum_evaluation_count",
        "purchased_air_supply_humidity_ratio_assignment_count",
        "source_site_execution_count",
    ] {
        assert_eq!(cp375[field], 0, "public direct CP375 {field}");
    }
    for field in [
        "dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_executed",
        "dehumidification_control_none_supply_humidity_ratio_maximum_assignment_executed",
        "purchased_air_supply_humidity_ratio_for_humidification_supply_maximum_read",
        "supply_humidity_ratio_for_humidification_for_supply_maximum_read",
        "source_shaped_two_argument_maximum_evaluated",
        "purchased_air_supply_humidity_ratio_assignment_performed",
    ] {
        assert_eq!(cp375["latest"][field], false, "{field}");
    }
    for field in [
        "predecessor_resulting_supply_humidity_ratio_for_humidification",
        "purchased_air_supply_humidity_ratio_before_humidification_supply_maximum",
        "supply_humidity_ratio_for_humidification_for_supply_maximum",
        "maximum_supply_humidity_ratio",
        "assigned_supply_humidity_ratio",
        "resulting_supply_humidity_ratio",
    ] {
        assert!(cp375["latest"][field].is_null(), "{field}");
        assert!(
            cp375["latest"][format!("{field}_ieee_bits")].is_null(),
            "{field} bits"
        );
    }

    assert_numerical_nonfeed(runtime, results);
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP375_KEY));
    assert!(
        runtime[CP375_KEY].is_null(),
        "non-direct runtime must not publish CP375 evidence"
    );
}

fn assert_numerical_nonfeed(runtime: &Value, results: &Value) {
    let cp345_bits = runtime["purchased_air_calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_lifecycle"]
        ["latest"]["assigned_supply_humidity_ratio_ieee_bits"]
        .as_str()
        .expect("CP345 numerical humidity-owner bits");
    let cp345_bits = cp345_bits
        .strip_prefix("0x")
        .and_then(|bits| u64::from_str_radix(bits, 16).ok())
        .expect("CP345 bits must be canonical 0x-prefixed hexadecimal");
    assert_eq!(
        cp345_bits, DIRECT_FIXTURE_SUPPLY_HUMIDITY_RATIO_BITS,
        "direct fixture numerical humidity baseline changed"
    );
    let supply_humidity = results["series"]
        .as_array()
        .expect("result series")
        .iter()
        .find(|series| {
            series["key"] == "ZONE ONE INLET"
                && series["variable_name"] == "System Node Humidity Ratio"
        })
        .expect("supply-node humidity result series");
    for endpoint in ["first", "last"] {
        assert_eq!(
            supply_humidity[endpoint]
                .as_f64()
                .expect("supply-node humidity endpoint")
                .to_bits(),
            cp345_bits,
            "CP375 evidence must not replace the {endpoint} numerical supply humidity"
        );
    }
}
