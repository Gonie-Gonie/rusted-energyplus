//! CP381 post-saturation capacity-limit dehumidification-guard assertions.

use serde_json::{Map, Value, json};

const CP329_KEY: &str = "purchased_air_calc_cooling_mixed_air_call_lifecycle";
const CP378_KEY: &str =
    "purchased_air_calc_cooling_supply_humidity_ratio_saturation_limit_assignment_lifecycle";
const CP379_KEY: &str =
    "purchased_air_calc_cooling_supply_enthalpy_post_saturation_assignment_lifecycle";
const CP380_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_guard_lifecycle";
const CP381_KEY: &str =
    "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_guard_lifecycle";
const ORDER: [&str; 4] = [
    "read-retained-purchased-air-supply-humidity-ratio-for-post-saturation-dehumidification-comparison",
    "read-retained-purchased-air-mixed-air-humidity-ratio-for-post-saturation-dehumidification-comparison",
    "compare-purchased-air-supply-humidity-ratio-strictly-less-than-mixed-air-humidity-ratio-for-post-saturation-dehumidification-guard",
    "enter-post-saturation-capacity-limit-dehumidification-body-if-comparison-satisfied",
];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let cp329 = &runtime[CP329_KEY];
    let cp378 = &runtime[CP378_KEY];
    let cp379 = &runtime[CP379_KEY];
    let cp380 = &runtime[CP380_KEY];
    let cp381 = &runtime[CP381_KEY];
    assert_eq!(
        cp381["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2266"
    );
    assert_eq!(
        cp381["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2267"
    );
    assert_eq!(cp381["latest"]["source_order"], json!(ORDER));

    let route_fields = [
        "unit_off_skip_count",
        "non_cooling_skip_count",
        "positive_guard_false_fallthrough_skip_count",
        "heating_availability_guard_false_fallthrough_count",
        "humidification_control_guard_false_fallthrough_count",
        "dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count",
        "dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count",
        "dehumidification_control_guard_false_fallthrough_count",
    ];
    for field in ["system", "transition_count"]
        .into_iter()
        .chain(route_fields)
    {
        assert_eq!(
            cp381[field], cp380[field],
            "CP381 must retain CP380 {field}"
        );
    }
    for field in &route_fields[5..] {
        assert_eq!(cp381[*field], 0, "public direct CP381 {field}");
    }

    let capacity_partitions = [
        (
            "heating_availability_guard_false_fallthrough_body_entry_count",
            "heating_availability_guard_false_fallthrough_capacity_guard_false_count",
        ),
        (
            "humidification_control_guard_false_fallthrough_body_entry_count",
            "humidification_control_guard_false_fallthrough_capacity_guard_false_count",
        ),
        (
            "dehumidification_control_humidistat_maximum_assignment_body_entry_count",
            "dehumidification_control_humidistat_maximum_assignment_capacity_guard_false_count",
        ),
        (
            "dehumidification_control_none_maximum_assignment_body_entry_count",
            "dehumidification_control_none_maximum_assignment_capacity_guard_false_count",
        ),
        (
            "dehumidification_control_guard_false_fallthrough_body_entry_count",
            "dehumidification_control_guard_false_fallthrough_capacity_guard_false_count",
        ),
    ];
    let dehumidification_partitions = [
        (
            "heating_availability_guard_false_fallthrough_dehumidification_body_entry_count",
            "heating_availability_guard_false_fallthrough_dehumidification_guard_false_count",
        ),
        (
            "humidification_control_guard_false_fallthrough_dehumidification_body_entry_count",
            "humidification_control_guard_false_fallthrough_dehumidification_guard_false_count",
        ),
        (
            "dehumidification_control_humidistat_maximum_assignment_dehumidification_body_entry_count",
            "dehumidification_control_humidistat_maximum_assignment_dehumidification_guard_false_count",
        ),
        (
            "dehumidification_control_none_maximum_assignment_dehumidification_body_entry_count",
            "dehumidification_control_none_maximum_assignment_dehumidification_guard_false_count",
        ),
        (
            "dehumidification_control_guard_false_fallthrough_dehumidification_body_entry_count",
            "dehumidification_control_guard_false_fallthrough_dehumidification_guard_false_count",
        ),
    ];
    for ((capacity_body, capacity_false), (body, guard_false)) in capacity_partitions
        .into_iter()
        .zip(dehumidification_partitions)
    {
        assert_eq!(cp381[capacity_body], cp380[capacity_body]);
        assert_eq!(cp381[capacity_false], cp380[capacity_false]);
        assert_eq!(
            count(cp381, body) + count(cp381, guard_false),
            count(cp381, capacity_body),
            "CP381 {capacity_body} refinement"
        );
    }

    let active = count(cp380, "capacity_limit_body_entry_count");
    let body = count(
        cp381,
        "supply_humidity_ratio_strictly_less_than_mixed_air_humidity_ratio_count",
    );
    let guard_false = active - body;
    for field in [
        "dehumidification_guard_evaluation_count",
        "cp378_supply_humidity_ratio_saturation_limit_owned_read_count",
        "cp379_same_call_supply_humidity_ratio_bit_corroboration_count",
        "purchased_air_supply_humidity_ratio_read_count",
        "cp329_mixed_air_humidity_ratio_owned_read_count",
        "purchased_air_mixed_air_humidity_ratio_read_count",
        "supply_humidity_ratio_mixed_air_humidity_ratio_comparison_count",
    ] {
        assert_eq!(cp381[field], active, "CP381 {field}");
    }
    assert_eq!(cp381["dehumidification_body_entry_count"], body);
    assert_eq!(
        cp381["dehumidification_guard_false_fallthrough_count"],
        guard_false
    );
    assert_eq!(cp381["source_site_execution_count"], 3 * active + body);

    let latest = &cp381["latest"];
    for field in [
        "unit_off_skipped",
        "non_cooling_skipped",
        "positive_guard_false_fallthrough_skipped",
        "heating_availability_guard_false_fallthrough",
        "humidification_control_guard_false_fallthrough",
        "dehumidification_control_humidistat_maximum_assignment_executed",
        "dehumidification_control_none_maximum_assignment_executed",
        "dehumidification_control_guard_false_fallthrough",
    ] {
        assert_eq!(latest[field], cp380["latest"][field], "CP381 CP380 {field}");
    }
    assert_eq!(
        latest["predecessor_capacity_limit_guard_evaluated"],
        cp380["latest"]["capacity_limit_guard_evaluated"]
    );
    assert_eq!(
        latest["predecessor_capacity_limit_body_entered"],
        cp380["latest"]["capacity_limit_body_entered"]
    );
    assert_eq!(
        latest["predecessor_active_capacity_limit_guard_false_fallthrough"],
        cp380["latest"]["active_guard_false_fallthrough"]
    );

    if latest["predecessor_capacity_limit_body_entered"] == true {
        assert_active_comparison(cp329, cp378, cp379, latest);
        assert!(matches!(
            cp380["latest"]["first_cooling_limit"].as_str(),
            Some("LimitCapacity" | "LimitFlowRateAndCapacity")
        ));
    } else {
        assert_null_comparison(latest);
        if cp380["latest"]["active_guard_false_fallthrough"] == true {
            assert!(matches!(
                cp380["latest"]["first_cooling_limit"].as_str(),
                Some("NoLimit" | "LimitFlowRate")
            ));
        }
        if active == 0 {
            assert_eq!(cp381["source_site_execution_count"], 0);
            assert_eq!(cp381["dehumidification_guard_evaluation_count"], 0);
        }
    }
    assert_numerical_nonfeed_and_unchanged_enthalpy(runtime, results);
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP381_KEY));
    assert!(
        runtime[CP381_KEY].is_null(),
        "non-direct runtime must not publish CP381 evidence"
    );
}

fn assert_active_comparison(cp329: &Value, cp378: &Value, cp379: &Value, latest: &Value) {
    for field in [
        "dehumidification_guard_evaluated",
        "cp378_supply_humidity_ratio_saturation_limit_owned_read",
        "cp379_same_call_supply_humidity_ratio_bit_corroborated",
        "purchased_air_supply_humidity_ratio_read",
        "cp329_mixed_air_humidity_ratio_owned_read",
        "purchased_air_mixed_air_humidity_ratio_read",
        "supply_humidity_ratio_mixed_air_humidity_ratio_comparison_evaluated",
    ] {
        assert_eq!(latest[field], true, "active CP381 {field}");
    }
    assert_eq!(
        latest["supply_humidity_ratio_ieee_bits"],
        cp378["latest"]["resulting_supply_humidity_ratio_ieee_bits"]
    );
    assert_eq!(
        latest["supply_humidity_ratio_ieee_bits"],
        cp379["latest"]["supply_humidity_ratio_ieee_bits"]
    );
    assert_eq!(
        latest["mixed_air_humidity_ratio_ieee_bits"],
        cp329["latest"]["mixed_air_humidity_ratio_ieee_bits"]
    );
    let supply = ieee_value(latest, "supply_humidity_ratio");
    let mixed = ieee_value(latest, "mixed_air_humidity_ratio");
    let less = supply < mixed;
    assert_eq!(
        latest["supply_humidity_ratio_strictly_less_than_mixed_air_humidity_ratio"],
        less
    );
    assert_eq!(latest["dehumidification_body_entered"], less);
    assert_eq!(latest["dehumidification_guard_false_fallthrough"], !less);
}

fn assert_null_comparison(latest: &Value) {
    for field in [
        "dehumidification_guard_evaluated",
        "cp378_supply_humidity_ratio_saturation_limit_owned_read",
        "cp379_same_call_supply_humidity_ratio_bit_corroborated",
        "purchased_air_supply_humidity_ratio_read",
        "cp329_mixed_air_humidity_ratio_owned_read",
        "purchased_air_mixed_air_humidity_ratio_read",
        "supply_humidity_ratio_mixed_air_humidity_ratio_comparison_evaluated",
        "dehumidification_body_entered",
        "dehumidification_guard_false_fallthrough",
    ] {
        assert_eq!(latest[field], false, "inactive CP381 {field}");
    }
    for field in [
        "supply_humidity_ratio",
        "supply_humidity_ratio_ieee_bits",
        "mixed_air_humidity_ratio",
        "mixed_air_humidity_ratio_ieee_bits",
        "supply_humidity_ratio_strictly_less_than_mixed_air_humidity_ratio",
    ] {
        assert!(latest[field].is_null(), "inactive CP381 {field}");
    }
}

fn ieee_value(value: &Value, field: &str) -> f64 {
    let bits = value[format!("{field}_ieee_bits")].as_str();
    assert!(bits.is_some(), "CP381 {field} IEEE bits");
    let parsed = u64::from_str_radix(bits.unwrap_or_default().trim_start_matches("0x"), 16);
    assert!(parsed.is_ok(), "CP381 {field} IEEE bits parse");
    f64::from_bits(parsed.unwrap_or_default())
}

fn count(value: &Value, field: &str) -> u64 {
    let count = value[field].as_u64();
    assert!(count.is_some(), "CP381 {field} count");
    count.unwrap_or_default()
}

fn assert_numerical_nonfeed_and_unchanged_enthalpy(runtime: &Value, _results: &Value) {
    assert!(
        runtime[CP379_KEY]["latest"]["resulting_supply_enthalpy_j_per_kg_ieee_bits"]
            .as_str()
            .is_some(),
        "CP381 guard evidence must leave CP379 enthalpy evidence unchanged"
    );
    let serialized = runtime[CP381_KEY].to_string();
    for forbidden in ["enthalpy_j_per_kg", "capacity_w", "supply_node", "report"] {
        assert!(
            !serialized.contains(forbidden),
            "CP381 comparison evidence must not feed or serialize {forbidden}"
        );
    }
}
