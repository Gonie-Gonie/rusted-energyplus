//! CP380 post-saturation capacity-limit guard assertions.

#[path = "cp381_assertions.rs"]
mod cp381_assertions;

use serde_json::{Map, Value, json};

const CP337_KEY: &str = "purchased_air_calc_cooling_positive_supply_capacity_limit_guard_lifecycle";
const CP379_KEY: &str =
    "purchased_air_calc_cooling_supply_enthalpy_post_saturation_assignment_lifecycle";
const CP380_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_guard_lifecycle";
const ORDER: [&str; 5] = [
    "read-cooling-limit-for-post-saturation-capacity-comparison",
    "compare-cooling-limit-equal-to-capacity-for-post-saturation-capacity-guard",
    "read-cooling-limit-for-post-saturation-flow-rate-and-capacity-comparison-after-first-false",
    "compare-cooling-limit-equal-to-flow-rate-and-capacity-for-post-saturation-capacity-guard",
    "enter-post-saturation-capacity-limit-body-if-compound-condition-satisfied",
];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let cp337 = &runtime[CP337_KEY];
    let cp379 = &runtime[CP379_KEY];
    let cp380 = &runtime[CP380_KEY];
    assert_eq!(
        cp380["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2264"
    );
    assert_eq!(
        cp380["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2266"
    );
    assert_eq!(cp380["latest"]["source_order"], json!(ORDER));

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
            cp380[field], cp379[field],
            "CP380 must retain CP379 {field}"
        );
    }
    for field in &route_fields[5..] {
        assert_eq!(cp380[*field], 0, "public direct CP380 {field}");
    }
    let active_routes = &route_fields[3..];
    let active = active_routes
        .iter()
        .map(|field| cp380[*field].as_u64().expect("CP380 active-route count"))
        .sum::<u64>();
    assert_eq!(
        active,
        cp379["local_supply_enthalpy_after_saturation_limit_assignment_count"]
            .as_u64()
            .expect("CP379 local enthalpy assignment count")
    );

    let latest = &cp380["latest"];
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
        assert_eq!(latest[field], cp379["latest"][field], "CP380 CP379 {field}");
    }
    assert_eq!(
        latest["predecessor_local_supply_enthalpy_after_saturation_limit_assignment_performed"],
        cp379["latest"]["local_supply_enthalpy_after_saturation_limit_assignment_performed"]
    );

    let inactive = latest["unit_off_skipped"] == true
        || latest["non_cooling_skipped"] == true
        || latest["positive_guard_false_fallthrough_skipped"] == true;
    if inactive {
        assert_complete_null(latest);
        assert_eq!(active, 0);
        assert_eq!(cp380["source_site_execution_count"], 0);
    } else {
        assert_active_selector_shape(cp337, cp380, latest, active);
    }
    assert_route_partitions(cp380, active_routes);
    assert_numerical_nonfeed_and_unchanged_enthalpy(runtime, results);
    cp381_assertions::assert_direct(runtime, results);
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP380_KEY));
    assert!(
        runtime[CP380_KEY].is_null(),
        "non-direct runtime must not publish CP380 evidence"
    );
    cp381_assertions::assert_non_direct(runtime);
}

fn assert_complete_null(latest: &Value) {
    for field in [
        "predecessor_local_supply_enthalpy_after_saturation_limit_assignment_performed",
        "capacity_limit_guard_evaluated",
        "configured_cooling_limit_owned_read",
        "cp337_same_call_selector_lineage_corroborated",
        "first_cooling_limit_read",
        "cooling_limit_capacity_comparison_evaluated",
        "second_cooling_limit_read",
        "cooling_limit_flow_rate_and_capacity_comparison_evaluated",
        "cooling_limit_rejected",
        "capacity_limit_body_entered",
        "active_guard_false_fallthrough",
    ] {
        assert_eq!(latest[field], false, "inactive CP380 {field}");
    }
    for field in [
        "first_cooling_limit",
        "cooling_limit_capacity",
        "second_cooling_limit",
        "cooling_limit_flow_rate_and_capacity",
        "cooling_limit_condition_satisfied",
    ] {
        assert!(latest[field].is_null(), "inactive CP380 {field}");
    }
}

fn assert_active_selector_shape(cp337: &Value, cp380: &Value, latest: &Value, active: u64) {
    for field in [
        "predecessor_local_supply_enthalpy_after_saturation_limit_assignment_performed",
        "capacity_limit_guard_evaluated",
        "configured_cooling_limit_owned_read",
        "cp337_same_call_selector_lineage_corroborated",
        "first_cooling_limit_read",
        "cooling_limit_capacity_comparison_evaluated",
    ] {
        assert_eq!(latest[field], true, "active CP380 {field}");
    }
    let selector = latest["first_cooling_limit"]
        .as_str()
        .expect("CP380 typed selector name");
    assert!(matches!(
        selector,
        "LimitCapacity" | "LimitFlowRateAndCapacity" | "NoLimit" | "LimitFlowRate"
    ));
    assert_eq!(cp337["latest"]["first_cooling_limit"], selector);
    let capacity = selector == "LimitCapacity";
    let combined = selector == "LimitFlowRateAndCapacity";
    let selected = capacity || combined;
    let second = !capacity;
    let capacity_matches = active * u64::from(capacity);
    let combined_matches = active * u64::from(combined);
    let body = capacity_matches + combined_matches;
    let second_count = active - capacity_matches;
    assert_eq!(latest["cooling_limit_capacity"], capacity);
    assert_eq!(latest["second_cooling_limit_read"], second);
    assert_eq!(
        latest["second_cooling_limit"],
        if second { json!(selector) } else { Value::Null }
    );
    assert_eq!(latest["cooling_limit_condition_satisfied"], selected);
    assert_eq!(latest["capacity_limit_body_entered"], selected);
    assert_eq!(latest["active_guard_false_fallthrough"], !selected);
    for field in [
        "capacity_limit_guard_evaluation_count",
        "configured_cooling_limit_owned_read_count",
        "cp337_same_call_selector_lineage_corroboration_count",
        "first_cooling_limit_read_count",
        "cooling_limit_capacity_comparison_count",
    ] {
        assert_eq!(cp380[field], active, "CP380 {field}");
    }
    assert_eq!(
        cp380["cooling_limit_capacity_match_count"],
        capacity_matches
    );
    assert_eq!(cp380["second_cooling_limit_read_count"], second_count);
    assert_eq!(
        cp380["cooling_limit_flow_rate_and_capacity_comparison_count"],
        second_count
    );
    assert_eq!(
        cp380["cooling_limit_flow_rate_and_capacity_match_count"],
        combined_matches
    );
    assert_eq!(cp380["capacity_limit_body_entry_count"], body);
    assert_eq!(cp380["cooling_limit_rejected_count"], active - body);
    assert_eq!(cp380["active_guard_false_fallthrough_count"], active - body);
    assert_eq!(
        cp380["source_site_execution_count"],
        2 * active + 2 * second_count + body
    );
}

fn assert_route_partitions(cp380: &Value, active_route_fields: &[&str]) {
    let partitions = [
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
    for (route, (body, rejected)) in active_route_fields.iter().zip(partitions) {
        assert_eq!(
            cp380[body].as_u64().expect("CP380 route body")
                + cp380[rejected].as_u64().expect("CP380 route false"),
            cp380[*route].as_u64().expect("CP380 route count"),
        );
    }
}

pub(super) fn assert_numerical_nonfeed_and_unchanged_enthalpy(runtime: &Value, _results: &Value) {
    let cp379 = &runtime[CP379_KEY];
    assert!(
        cp379["latest"]["resulting_supply_enthalpy_j_per_kg_ieee_bits"]
            .as_str()
            .is_some()
            || cp379["latest"]["resulting_supply_enthalpy_j_per_kg_ieee_bits"].is_null()
    );
    let serialized = runtime[CP380_KEY].to_string();
    for forbidden in [
        "_ieee_bits",
        "enthalpy_j_per_kg",
        "capacity_w",
        "supply_node",
        "report",
    ] {
        assert!(
            !serialized.contains(forbidden),
            "CP380 control evidence must not feed or serialize numerical field {forbidden}"
        );
    }
}
