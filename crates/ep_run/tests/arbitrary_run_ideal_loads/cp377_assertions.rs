//! CP377 saturation-assignment assertions.

#[path = "cp378_assertions.rs"]
mod cp378_assertions;

use serde_json::{Map, Value, json};

const CP334_KEY: &str =
    "purchased_air_calc_cooling_positive_supply_temperature_mixed_air_limit_lifecycle";
const CP344_KEY: &str = "purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_lifecycle";
const CP376_KEY: &str =
    "purchased_air_calc_cooling_supply_humidity_ratio_pre_saturation_original_assignment_lifecycle";
const CP377_KEY: &str =
    "purchased_air_calc_cooling_supply_humidity_ratio_saturation_assignment_lifecycle";
const ORDER: [&str; 4] = [
    "read-purchased-air-supply-temperature-for-saturation-humidity-ratio",
    "read-environment-outdoor-barometric-pressure-for-saturation-humidity-ratio",
    "evaluate-psy-w-fn-tdb-rh-pb-at-unity-relative-humidity",
    "assign-local-saturation-supply-humidity-ratio",
];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let cp334 = &runtime[CP334_KEY];
    let cp344 = &runtime[CP344_KEY];
    let cp376 = &runtime[CP376_KEY];
    let cp377 = &runtime[CP377_KEY];
    assert_eq!(
        cp377["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2259"
    );
    assert_eq!(
        cp377["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2260"
    );
    assert_eq!(cp377["latest"]["source_order"], json!(ORDER));

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
            cp377[field], cp376[field],
            "CP377 must retain CP376 {field}"
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
    .map(|field| cp377[field].as_u64().expect("CP377 active-route count"))
    .sum::<u64>();
    assert_eq!(cp377["source_site_execution_count"], active_count * 4);
    for field in [
        "purchased_air_supply_temperature_for_saturation_humidity_ratio_read_count",
        "environment_outdoor_barometric_pressure_for_saturation_humidity_ratio_read_count",
        "psy_w_fn_tdb_rh_pb_at_unity_relative_humidity_evaluation_count",
        "local_saturation_supply_humidity_ratio_assignment_count",
        "environment_outdoor_barometric_pressure_owner_count",
    ] {
        assert_eq!(cp377[field], active_count, "CP377 {field}");
    }
    assert_eq!(
        cp377["cp334_supply_temperature_mixed_air_limit_owner_count"]
            .as_u64()
            .expect("CP334 owner count")
            + cp377["cp344_capacity_limit_supply_temperature_mixed_air_limit_owner_count"]
                .as_u64()
                .expect("CP344 owner count"),
        active_count,
    );
    assert_eq!(
        cp377["cp344_capacity_limit_supply_temperature_mixed_air_limit_owner_count"],
        cp344["capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count"]
    );

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
            "predecessor_local_supply_humidity_ratio_original_assignment_performed",
            "local_supply_humidity_ratio_original_assignment_performed",
        ),
        (
            "predecessor_resulting_supply_humidity_ratio_original_ieee_bits",
            "resulting_supply_humidity_ratio_original_ieee_bits",
        ),
    ] {
        assert_eq!(
            cp377["latest"][current], cp376["latest"][predecessor],
            "CP377 must retain exact CP376 lineage"
        );
    }

    let inactive = cp377["latest"]["unit_off_skipped"] == true
        || cp377["latest"]["non_cooling_skipped"] == true
        || cp377["latest"]["positive_guard_false_fallthrough_skipped"] == true;
    if inactive {
        assert_complete_null(&cp377["latest"]);
    } else {
        assert_active_values(cp334, cp344, &cp377["latest"]);
    }
    cp378_assertions::assert_direct(runtime, results);
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP377_KEY));
    assert!(
        runtime[CP377_KEY].is_null(),
        "non-direct runtime must not publish CP377 evidence"
    );
    cp378_assertions::assert_non_direct(runtime);
}

fn assert_complete_null(latest: &Value) {
    for field in [
        "cp334_supply_temperature_mixed_air_limit_owned_read",
        "cp344_capacity_limit_supply_temperature_mixed_air_limit_owned_read",
        "environment_outdoor_barometric_pressure_owned_read",
        "purchased_air_supply_temperature_for_saturation_humidity_ratio_read",
        "environment_outdoor_barometric_pressure_for_saturation_humidity_ratio_read",
        "psy_w_fn_tdb_rh_pb_at_unity_relative_humidity_evaluated",
        "local_saturation_supply_humidity_ratio_assignment_performed",
    ] {
        assert_eq!(latest[field], false, "inactive CP377 {field}");
    }
    for field in [
        "supply_temperature_for_saturation_humidity_ratio_c",
        "outdoor_barometric_pressure_pa",
        "saturation_supply_humidity_ratio",
        "assigned_saturation_supply_humidity_ratio",
        "resulting_saturation_supply_humidity_ratio",
    ] {
        assert!(latest[field].is_null(), "inactive CP377 {field}");
        assert!(
            latest[format!("{field}_ieee_bits")].is_null(),
            "inactive CP377 {field} bits"
        );
    }
}

fn assert_active_values(cp334: &Value, cp344: &Value, latest: &Value) {
    let cp344_owned = cp344["latest"]["capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed"]
        == true;
    assert_eq!(
        latest["cp334_supply_temperature_mixed_air_limit_owned_read"],
        !cp344_owned
    );
    assert_eq!(
        latest["cp344_capacity_limit_supply_temperature_mixed_air_limit_owned_read"],
        cp344_owned
    );
    let owner_bits = if cp344_owned {
        &cp344["latest"]["resulting_supply_temperature_c_ieee_bits"]
    } else {
        &cp334["latest"]["assigned_supply_temperature_c_ieee_bits"]
    };
    assert_eq!(
        latest["supply_temperature_for_saturation_humidity_ratio_c_ieee_bits"],
        *owner_bits
    );
    assert!(
        latest["supply_temperature_for_saturation_humidity_ratio_c"]
            .as_f64()
            .is_some(),
        "finite CP377 temperature projection"
    );
    assert!(
        latest["outdoor_barometric_pressure_pa"].as_f64().is_some(),
        "finite CP377 pressure projection"
    );
    let temperature = f64::from_bits(parse_bits(
        &latest["supply_temperature_for_saturation_humidity_ratio_c_ieee_bits"],
    ));
    let pressure = f64::from_bits(parse_bits(
        &latest["outdoor_barometric_pressure_pa_ieee_bits"],
    ));
    assert!(pressure > 0.0);
    assert!(temperature.is_finite());
    let saturation_bits = parse_bits(&latest["saturation_supply_humidity_ratio_ieee_bits"]);
    assert!(f64::from_bits(saturation_bits).is_finite());
    for field in [
        "saturation_supply_humidity_ratio",
        "assigned_saturation_supply_humidity_ratio",
        "resulting_saturation_supply_humidity_ratio",
    ] {
        assert!(
            latest[field].as_f64().is_some(),
            "finite CP377 {field} projection"
        );
        assert_eq!(
            parse_bits(&latest[format!("{field}_ieee_bits")]),
            saturation_bits,
            "CP377 {field} sidecar"
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
