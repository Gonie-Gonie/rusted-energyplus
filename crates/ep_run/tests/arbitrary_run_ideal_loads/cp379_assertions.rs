//! CP379 post-saturation supply-enthalpy assignment assertions.

use ep_runtime::psychrometrics::energyplus_psy_h_fn_tdb_w;
use serde_json::{Map, Value, json};

const CP336_NUMERICAL_IMPLEMENTATION_KEY: &str =
    "purchased_air_calc_cooling_positive_supply_enthalpy_assignment_lifecycle";
const CP377_KEY: &str =
    "purchased_air_calc_cooling_supply_humidity_ratio_saturation_assignment_lifecycle";
const CP378_KEY: &str =
    "purchased_air_calc_cooling_supply_humidity_ratio_saturation_limit_assignment_lifecycle";
const CP379_KEY: &str =
    "purchased_air_calc_cooling_supply_enthalpy_post_saturation_assignment_lifecycle";
const ORDER: [&str; 4] = [
    "read-purchased-air-supply-temperature-for-post-saturation-enthalpy",
    "read-purchased-air-supply-humidity-ratio-for-post-saturation-enthalpy",
    "evaluate-psy-h-fn-tdb-w-for-post-saturation-enthalpy",
    "assign-local-supply-enthalpy-after-saturation-limit",
];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let cp377 = &runtime[CP377_KEY];
    let cp378 = &runtime[CP378_KEY];
    let cp379 = &runtime[CP379_KEY];
    assert_eq!(
        cp379["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2261"
    );
    assert_eq!(
        cp379["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2264"
    );
    assert_eq!(cp379["latest"]["source_order"], json!(ORDER));

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
            cp379[field], cp378[field],
            "CP379 must retain CP378 {field}"
        );
        assert_eq!(
            cp379[field], cp377[field],
            "CP379 must retain CP377 {field}"
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
    .map(|field| cp379[field].as_u64().expect("CP379 active-route count"))
    .sum::<u64>();
    assert_eq!(cp379["source_site_execution_count"], active_count * 4);
    for field in [
        "dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count",
        "dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count",
        "dehumidification_control_guard_false_fallthrough_count",
    ] {
        assert_eq!(cp379[field], 0, "public direct CP379 {field}");
    }
    for field in [
        "purchased_air_supply_temperature_for_post_saturation_enthalpy_read_count",
        "purchased_air_supply_humidity_ratio_for_post_saturation_enthalpy_read_count",
        "psy_h_fn_tdb_w_for_post_saturation_enthalpy_evaluation_count",
        "local_supply_enthalpy_after_saturation_limit_assignment_count",
        "cp378_supply_humidity_ratio_saturation_limit_owner_count",
    ] {
        assert_eq!(cp379[field], active_count, "CP379 {field}");
    }
    assert_eq!(
        cp379["cp334_supply_temperature_mixed_air_limit_owner_count"]
            .as_u64()
            .expect("CP379 CP334 owner count")
            + cp379["cp344_capacity_limit_supply_temperature_mixed_air_limit_owner_count"]
                .as_u64()
                .expect("CP379 CP344 owner count"),
        active_count
    );
    for field in [
        "cp334_supply_temperature_mixed_air_limit_owner_count",
        "cp344_capacity_limit_supply_temperature_mixed_air_limit_owner_count",
    ] {
        assert_eq!(
            cp379[field], cp377[field],
            "CP379 must retain CP377 {field}"
        );
    }

    for field in [
        "unit_off_skipped",
        "non_cooling_skipped",
        "positive_guard_false_fallthrough_skipped",
        "heating_availability_guard_false_fallthrough",
        "humidification_control_guard_false_fallthrough",
        "dehumidification_control_humidistat_maximum_assignment_executed",
        "dehumidification_control_none_maximum_assignment_executed",
        "dehumidification_control_guard_false_fallthrough",
        "predecessor_dehumidification_control_type",
    ] {
        assert_eq!(
            cp379["latest"][field], cp378["latest"][field],
            "CP379 must retain exact CP378 {field} lineage"
        );
        assert_eq!(
            cp379["latest"][field], cp377["latest"][field],
            "CP379 must retain exact CP377 {field} lineage"
        );
    }
    assert_eq!(
        cp379["latest"]["predecessor_supply_humidity_ratio_saturation_limit_assignment_performed"],
        cp378["latest"]["purchased_air_supply_humidity_ratio_saturation_limit_assignment_performed"]
    );
    assert_eq!(
        cp379["latest"]["predecessor_resulting_supply_humidity_ratio_ieee_bits"],
        cp378["latest"]["resulting_supply_humidity_ratio_ieee_bits"]
    );

    let inactive = cp379["latest"]["unit_off_skipped"] == true
        || cp379["latest"]["non_cooling_skipped"] == true
        || cp379["latest"]["positive_guard_false_fallthrough_skipped"] == true;
    if inactive {
        assert_complete_null(&cp379["latest"]);
    } else {
        assert_active_values(cp377, cp378, &cp379["latest"]);
    }
    assert_numerical_nonfeed_and_unchanged_enthalpy(runtime, results);
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP379_KEY));
    assert!(
        runtime[CP379_KEY].is_null(),
        "non-direct runtime must not publish CP379 evidence"
    );
}

fn assert_complete_null(latest: &Value) {
    for field in [
        "cp377_supply_temperature_owned_read",
        "cp334_supply_temperature_mixed_air_limit_owned_read",
        "cp344_capacity_limit_supply_temperature_mixed_air_limit_owned_read",
        "cp378_supply_humidity_ratio_saturation_limit_owned_read",
        "purchased_air_supply_temperature_for_post_saturation_enthalpy_read",
        "purchased_air_supply_humidity_ratio_for_post_saturation_enthalpy_read",
        "psy_h_fn_tdb_w_for_post_saturation_enthalpy_evaluated",
        "local_supply_enthalpy_after_saturation_limit_assignment_performed",
    ] {
        assert_eq!(latest[field], false, "inactive CP379 {field}");
    }
    for field in [
        "supply_temperature_c",
        "supply_humidity_ratio",
        "psychrometric_supply_enthalpy_j_per_kg",
        "assigned_supply_enthalpy_j_per_kg",
        "resulting_supply_enthalpy_j_per_kg",
    ] {
        assert!(latest[field].is_null(), "inactive CP379 {field}");
        assert!(
            latest[format!("{field}_ieee_bits")].is_null(),
            "inactive CP379 {field} bits"
        );
    }
}

fn assert_active_values(cp377: &Value, cp378: &Value, latest: &Value) {
    for field in [
        "cp377_supply_temperature_owned_read",
        "cp378_supply_humidity_ratio_saturation_limit_owned_read",
        "purchased_air_supply_temperature_for_post_saturation_enthalpy_read",
        "purchased_air_supply_humidity_ratio_for_post_saturation_enthalpy_read",
        "psy_h_fn_tdb_w_for_post_saturation_enthalpy_evaluated",
        "local_supply_enthalpy_after_saturation_limit_assignment_performed",
    ] {
        assert_eq!(latest[field], true, "active CP379 {field}");
    }
    assert_eq!(
        latest["cp334_supply_temperature_mixed_air_limit_owned_read"]
            .as_bool()
            .expect("CP379 CP334 owner flag") as u8
            + latest["cp344_capacity_limit_supply_temperature_mixed_air_limit_owned_read"]
                .as_bool()
                .expect("CP379 CP344 owner flag") as u8,
        1
    );
    assert_eq!(
        latest["supply_temperature_c_ieee_bits"],
        cp377["latest"]["supply_temperature_for_saturation_humidity_ratio_c_ieee_bits"]
    );
    assert_eq!(
        latest["supply_humidity_ratio_ieee_bits"],
        cp378["latest"]["resulting_supply_humidity_ratio_ieee_bits"]
    );
    let temperature = latest["supply_temperature_c"]
        .as_f64()
        .expect("finite CP379 temperature");
    let humidity = latest["supply_humidity_ratio"]
        .as_f64()
        .expect("finite CP379 humidity ratio");
    let expected = energyplus_psy_h_fn_tdb_w(temperature, humidity);
    let expected_bits = format!("0x{:016x}", expected.to_bits());
    for field in [
        "psychrometric_supply_enthalpy_j_per_kg",
        "assigned_supply_enthalpy_j_per_kg",
        "resulting_supply_enthalpy_j_per_kg",
    ] {
        assert_eq!(
            latest[format!("{field}_ieee_bits")],
            expected_bits,
            "CP379 {field} canonical bits"
        );
    }
}

fn assert_numerical_nonfeed_and_unchanged_enthalpy(runtime: &Value, _results: &Value) {
    let numerical = &runtime[CP336_NUMERICAL_IMPLEMENTATION_KEY];
    assert_eq!(
        numerical["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2191"
    );
    assert!(
        numerical["latest"]["supply_enthalpy_j_per_kg_ieee_bits"]
            .as_str()
            .is_some(),
        "CP379 evidence must leave the unchanged CP336 numerical enthalpy implementation intact"
    );
    // Deliberately do not reconcile CP379's canonical local value with numerical output or DTOs:
    // the source statement is evidence-only and later source statements may overwrite enthalpy.
}
