//! CP378 saturation-limit assignment assertions.

#[path = "cp379_assertions.rs"]
mod cp379_assertions;

use serde_json::{Map, Value, json};

const CP377_KEY: &str =
    "purchased_air_calc_cooling_supply_humidity_ratio_saturation_assignment_lifecycle";
const CP345_NUMERICAL_IMPLEMENTATION_KEY: &str = "purchased_air_calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_lifecycle";
const CP378_KEY: &str =
    "purchased_air_calc_cooling_supply_humidity_ratio_saturation_limit_assignment_lifecycle";
const ORDER: [&str; 4] = [
    "read-local-original-supply-humidity-ratio-for-saturation-limit-minimum",
    "read-local-saturation-supply-humidity-ratio-for-saturation-limit-minimum",
    "apply-source-shaped-two-argument-minimum-for-saturation-limit",
    "assign-purchased-air-supply-humidity-ratio-for-saturation-limit",
];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let cp377 = &runtime[CP377_KEY];
    let cp378 = &runtime[CP378_KEY];
    assert_eq!(
        cp378["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2260"
    );
    assert_eq!(
        cp378["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2261"
    );
    assert_eq!(cp378["latest"]["source_order"], json!(ORDER));

    for field in [
        "system",
        "transition_count",
        "unit_off_skip_count",
        "non_cooling_skip_count",
        "positive_guard_false_fallthrough_skip_count",
        "heating_availability_guard_false_fallthrough_count",
        "humidification_control_guard_false_fallthrough_count",
        "dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count",
        "dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count",
        "dehumidification_control_guard_false_fallthrough_count",
    ] {
        assert_eq!(
            cp378[field], cp377[field],
            "CP378 must retain CP377 {field}"
        );
    }
    let active_count = [
        "heating_availability_guard_false_fallthrough_count",
        "humidification_control_guard_false_fallthrough_count",
        "dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count",
        "dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count",
        "dehumidification_control_guard_false_fallthrough_count",
    ]
    .into_iter()
    .map(|field| cp378[field].as_u64().expect("CP378 active-route count"))
    .sum::<u64>();
    assert_eq!(cp378["source_site_execution_count"], active_count * 4);
    for field in [
        "local_original_supply_humidity_ratio_for_saturation_limit_minimum_read_count",
        "local_saturation_supply_humidity_ratio_for_saturation_limit_minimum_read_count",
        "source_shaped_two_argument_minimum_evaluation_count",
        "purchased_air_supply_humidity_ratio_saturation_limit_assignment_count",
        "cp376_original_supply_humidity_ratio_owner_count",
        "cp377_saturation_supply_humidity_ratio_owner_count",
    ] {
        assert_eq!(cp378[field], active_count, "CP378 {field}");
    }

    for (current, predecessor) in [
        ("unit_off_skipped", "unit_off_skipped"),
        ("non_cooling_skipped", "non_cooling_skipped"),
        (
            "positive_guard_false_fallthrough_skipped",
            "positive_guard_false_fallthrough_skipped",
        ),
        (
            "heating_availability_guard_false_fallthrough",
            "heating_availability_guard_false_fallthrough",
        ),
        (
            "humidification_control_guard_false_fallthrough",
            "humidification_control_guard_false_fallthrough",
        ),
        (
            "dehumidification_control_humidistat_maximum_assignment_executed",
            "dehumidification_control_humidistat_maximum_assignment_executed",
        ),
        (
            "dehumidification_control_none_maximum_assignment_executed",
            "dehumidification_control_none_maximum_assignment_executed",
        ),
        (
            "dehumidification_control_guard_false_fallthrough",
            "dehumidification_control_guard_false_fallthrough",
        ),
        (
            "predecessor_dehumidification_control_type",
            "predecessor_dehumidification_control_type",
        ),
        (
            "predecessor_resulting_supply_humidity_ratio_original_ieee_bits",
            "predecessor_resulting_supply_humidity_ratio_original_ieee_bits",
        ),
        (
            "predecessor_resulting_saturation_supply_humidity_ratio_ieee_bits",
            "resulting_saturation_supply_humidity_ratio_ieee_bits",
        ),
    ] {
        assert_eq!(
            cp378["latest"][current], cp377["latest"][predecessor],
            "CP378 must retain exact CP377 lineage"
        );
    }

    let inactive = cp378["latest"]["unit_off_skipped"] == true
        || cp378["latest"]["non_cooling_skipped"] == true
        || cp378["latest"]["positive_guard_false_fallthrough_skipped"] == true;
    if inactive {
        assert_complete_null(&cp378["latest"]);
    } else {
        assert_active_values(&cp378["latest"]);
        assert_numerical_nonfeed_and_exact_reconciliation(runtime, results, &cp378["latest"]);
    }
    cp379_assertions::assert_direct(runtime, results);
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP378_KEY));
    assert!(
        runtime[CP378_KEY].is_null(),
        "non-direct runtime must not publish CP378 evidence"
    );
    cp379_assertions::assert_non_direct(runtime);
}

fn assert_complete_null(latest: &Value) {
    for field in [
        "cp376_original_supply_humidity_ratio_owned_read",
        "cp377_saturation_supply_humidity_ratio_owned_read",
        "local_original_supply_humidity_ratio_for_saturation_limit_minimum_read",
        "local_saturation_supply_humidity_ratio_for_saturation_limit_minimum_read",
        "source_shaped_two_argument_minimum_evaluated",
        "purchased_air_supply_humidity_ratio_saturation_limit_assignment_performed",
    ] {
        assert_eq!(latest[field], false, "inactive CP378 {field}");
    }
    for field in [
        "original_supply_humidity_ratio_before_saturation_limit",
        "saturation_supply_humidity_ratio_for_limit",
        "minimum_supply_humidity_ratio_after_saturation_limit",
        "assigned_supply_humidity_ratio",
        "resulting_supply_humidity_ratio",
    ] {
        assert!(latest[field].is_null(), "inactive CP378 {field}");
        assert!(
            latest[format!("{field}_ieee_bits")].is_null(),
            "inactive CP378 {field} bits"
        );
    }
}

fn assert_active_values(latest: &Value) {
    assert_eq!(
        latest["cp376_original_supply_humidity_ratio_owned_read"],
        true
    );
    assert_eq!(
        latest["cp377_saturation_supply_humidity_ratio_owned_read"],
        true
    );
    let original_bits =
        parse_bits(&latest["original_supply_humidity_ratio_before_saturation_limit_ieee_bits"]);
    let saturation_bits =
        parse_bits(&latest["saturation_supply_humidity_ratio_for_limit_ieee_bits"]);
    let original = f64::from_bits(original_bits);
    let saturation = f64::from_bits(saturation_bits);
    let selected_bits = if original < saturation {
        original_bits
    } else {
        saturation_bits
    };
    for field in [
        "minimum_supply_humidity_ratio_after_saturation_limit",
        "assigned_supply_humidity_ratio",
        "resulting_supply_humidity_ratio",
    ] {
        assert!(latest[field].as_f64().is_some(), "finite CP378 {field}");
        assert_eq!(
            parse_bits(&latest[format!("{field}_ieee_bits")]),
            selected_bits,
            "CP378 {field} sidecar"
        );
    }
}

fn assert_numerical_nonfeed_and_exact_reconciliation(
    runtime: &Value,
    results: &Value,
    latest: &Value,
) {
    let terminal_source_bits = parse_bits(&latest["resulting_supply_humidity_ratio_ieee_bits"]);
    let numerical_implementation_bits = parse_bits(
        &runtime[CP345_NUMERICAL_IMPLEMENTATION_KEY]["latest"]["assigned_supply_humidity_ratio_ieee_bits"],
    );
    assert_eq!(
        numerical_implementation_bits, terminal_source_bits,
        "CP378 must reconcile with the unchanged CP345 numerical implementation"
    );
    let series = results["series"]
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
            series[endpoint]
                .as_f64()
                .expect("humidity endpoint")
                .to_bits(),
            terminal_source_bits,
            "CP378 must not feed or overwrite and must exactly reconcile the unchanged {endpoint} numerical humidity"
        );
    }
}

fn parse_bits(value: &Value) -> u64 {
    value
        .as_str()
        .and_then(|bits| bits.strip_prefix("0x"))
        .and_then(|bits| u64::from_str_radix(bits, 16).ok())
        .expect("canonical IEEE hexadecimal sidecar")
}
