//! CP357 case-break and cumulative numerical-nonfeed assertions.

use serde_json::{Map, Value};

#[path = "cp358_assertions.rs"]
mod cp358_assertions;

const CP356_KEY: &str =
    "purchased_air_calc_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit_lifecycle";
const CP357_KEY: &str = "purchased_air_calc_cooling_constant_shr_case_break_lifecycle";
const ORDER: [&str; 1] =
    ["exit-purchased-air-dehumidification-control-constant-sensible-heat-ratio-case-via-break"];
#[allow(dead_code)]
const DIRECT_FIXTURE_SUPPLY_HUMIDITY_RATIO_BITS: u64 = 0x3f5d_aac3_b48c_9d41;

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let cp356 = &runtime[CP356_KEY];
    let cp357 = &runtime[CP357_KEY];
    assert_eq!(
        cp357["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2227"
    );
    assert_eq!(
        cp357["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2229"
    );
    assert_eq!(
        cp357["latest"]["source_order"]
            .as_array()
            .expect("CP357 source order"),
        &ORDER
    );
    for (cp357_field, cp356_field) in [
        (
            "predecessor_dehumidification_control_type",
            "predecessor_dehumidification_control_type",
        ),
        (
            "predecessor_dehumidification_control_none_case_completed_skip",
            "dehumidification_control_none_case_completed_skip",
        ),
        (
            "predecessor_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_mixed_air_limit_executed",
            "dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_mixed_air_limit_executed",
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
            cp357["latest"][cp357_field], cp356["latest"][cp356_field],
            "CP357 must retain immediate CP356 lineage"
        );
    }
    assert_eq!(
        cp357["latest"]["dehumidification_control_none_case_completed_skip"],
        true
    );
    assert_eq!(
        cp357["latest"]["dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break"],
        false
    );
    assert_eq!(
        cp357["latest"]["dehumidification_control_humidistat_case_selected_skip"], false,
        "direct CP357 cannot fall through to Humidistat"
    );
    assert_eq!(
        cp357["latest"]["dehumidification_control_constant_supply_humidity_ratio_case_selected_skip"],
        false
    );
    assert_eq!(
        cp357["dehumidification_control_none_case_completed_skip_count"],
        cp356["dehumidification_control_none_case_completed_skip_count"]
    );
    assert_eq!(
        cp357["dehumidification_control_constant_sensible_heat_ratio_case_break_count"],
        cp356["dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_mixed_air_limit_count"]
    );
    assert_eq!(
        cp357["dehumidification_control_constant_sensible_heat_ratio_case_break_count"],
        0
    );
    assert_eq!(cp357["source_site_execution_count"], 0);
    cp358_assertions::assert_direct(runtime, results);
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP357_KEY));
    assert!(
        runtime[CP357_KEY].is_null(),
        "non-direct runtime must not publish CP357 evidence"
    );
    cp358_assertions::assert_non_direct(runtime);
}

#[allow(dead_code)]
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
            "source-order evidence must not replace the {endpoint} numerical supply humidity"
        );
    }
}
