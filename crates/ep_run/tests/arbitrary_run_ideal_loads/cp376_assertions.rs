//! CP376 pre-saturation original-assignment assertions.

use serde_json::{Map, Value, json};

const CP345_KEY: &str = "purchased_air_calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_lifecycle";
const CP347_KEY: &str = "purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_lifecycle";
const CP375_KEY: &str = "purchased_air_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_lifecycle";
const CP376_KEY: &str =
    "purchased_air_calc_cooling_supply_humidity_ratio_pre_saturation_original_assignment_lifecycle";
const ORDER: [&str; 2] = [
    "read-purchased-air-supply-humidity-ratio-before-saturation-limit",
    "assign-local-original-supply-humidity-ratio-before-saturation-limit",
];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let cp347 = &runtime[CP347_KEY];
    let cp375 = &runtime[CP375_KEY];
    let cp376 = &runtime[CP376_KEY];
    assert_eq!(
        cp376["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2258"
    );
    assert_eq!(
        cp376["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2259"
    );
    assert_eq!(cp376["latest"]["source_order"], json!(ORDER));

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
            cp376[field], cp375[field],
            "CP376 direct counters must retain exact CP375 lineage"
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
    .map(|field| cp376[field].as_u64().expect("CP376 active-route count"))
    .sum::<u64>();
    assert_eq!(
        cp376["cp347_none_case_owner_count"], active_count,
        "public direct CP376 must read the CP347 owner exactly once per active route"
    );
    assert_eq!(
        cp376["cp347_none_case_owner_count"],
        cp347["dehumidification_control_none_case_completion_count"]
    );
    for field in [
        "cp375_maximum_assignment_owner_count",
        "cp356_constant_shr_owner_count",
        "cp362_humidistat_owner_count",
        "cp365_constant_supply_humidity_ratio_owner_count",
    ] {
        assert_eq!(cp376[field], 0, "public direct CP376 {field}");
    }
    assert_eq!(cp376["source_site_execution_count"], active_count * 2);
    assert_eq!(
        cp376["purchased_air_supply_humidity_ratio_before_saturation_limit_read_count"],
        active_count
    );
    assert_eq!(
        cp376["local_original_supply_humidity_ratio_before_saturation_limit_assignment_count"],
        active_count
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
            "predecessor_heating_on_guard_false_fallthrough",
        ),
        (
            "humidification_control_guard_false_fallthrough",
            "predecessor_humidification_control_guard_false_fallthrough",
        ),
        (
            "dehumidification_control_humidistat_maximum_assignment_executed",
            "dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_executed",
        ),
        (
            "dehumidification_control_none_maximum_assignment_executed",
            "dehumidification_control_none_supply_humidity_ratio_maximum_assignment_executed",
        ),
        (
            "dehumidification_control_guard_false_fallthrough",
            "predecessor_dehumidification_control_guard_false_fallthrough",
        ),
        (
            "predecessor_dehumidification_control_type",
            "predecessor_dehumidification_control_type",
        ),
        (
            "predecessor_purchased_air_supply_humidity_ratio_assignment_performed",
            "purchased_air_supply_humidity_ratio_assignment_performed",
        ),
        (
            "predecessor_resulting_supply_humidity_ratio",
            "resulting_supply_humidity_ratio",
        ),
        (
            "predecessor_resulting_supply_humidity_ratio_ieee_bits",
            "resulting_supply_humidity_ratio_ieee_bits",
        ),
    ] {
        assert_eq!(
            cp376["latest"][current], cp375["latest"][predecessor],
            "CP376 must retain exact CP375 route and value lineage"
        );
    }

    let inactive = cp376["latest"]["unit_off_skipped"] == true
        || cp376["latest"]["non_cooling_skipped"] == true
        || cp376["latest"]["positive_guard_false_fallthrough_skipped"] == true;
    if inactive {
        for field in [
            "cp375_maximum_assignment_owned_read",
            "cp347_none_case_owned_read",
            "cp356_constant_shr_owned_read",
            "cp362_humidistat_owned_read",
            "cp365_constant_supply_humidity_ratio_owned_read",
            "purchased_air_supply_humidity_ratio_read",
            "local_supply_humidity_ratio_original_assignment_performed",
        ] {
            assert_eq!(cp376["latest"][field], false, "inactive CP376 {field}");
        }
        for field in [
            "purchased_air_supply_humidity_ratio_before_saturation_check",
            "assigned_supply_humidity_ratio_original",
            "resulting_supply_humidity_ratio_original",
        ] {
            assert!(cp376["latest"][field].is_null(), "inactive CP376 {field}");
            assert!(
                cp376["latest"][format!("{field}_ieee_bits")].is_null(),
                "inactive CP376 {field} bits"
            );
        }
    } else {
        assert_eq!(
            cp376["latest"]["cp375_maximum_assignment_owned_read"],
            false
        );
        assert_eq!(cp376["latest"]["cp347_none_case_owned_read"], true);
        assert_eq!(cp376["latest"]["cp356_constant_shr_owned_read"], false);
        assert_eq!(cp376["latest"]["cp362_humidistat_owned_read"], false);
        assert_eq!(
            cp376["latest"]["cp365_constant_supply_humidity_ratio_owned_read"],
            false
        );
        assert_eq!(
            cp376["latest"]["purchased_air_supply_humidity_ratio_read"],
            true
        );
        assert_eq!(
            cp376["latest"]["local_supply_humidity_ratio_original_assignment_performed"],
            true
        );
        let owner_bits = &cp347["latest"]["resulting_supply_humidity_ratio_ieee_bits"];
        for field in [
            "purchased_air_supply_humidity_ratio_before_saturation_check_ieee_bits",
            "assigned_supply_humidity_ratio_original_ieee_bits",
            "resulting_supply_humidity_ratio_original_ieee_bits",
        ] {
            assert_eq!(
                cp376["latest"][field], *owner_bits,
                "CP376 must preserve the CP347 owner bits"
            );
        }
    }

    assert_numerical_nonfeed(runtime, results);
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP376_KEY));
    assert!(
        runtime[CP376_KEY].is_null(),
        "non-direct runtime must not publish CP376 evidence"
    );
}

fn assert_numerical_nonfeed(runtime: &Value, results: &Value) {
    let cp345_bits = runtime[CP345_KEY]["latest"]["assigned_supply_humidity_ratio_ieee_bits"]
        .as_str()
        .expect("CP345 numerical humidity-owner bits");
    let cp345_bits = cp345_bits
        .strip_prefix("0x")
        .and_then(|bits| u64::from_str_radix(bits, 16).ok())
        .expect("CP345 bits must be canonical 0x-prefixed hexadecimal");
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
            "CP376 evidence must not feed the {endpoint} numerical supply humidity"
        );
    }
}
