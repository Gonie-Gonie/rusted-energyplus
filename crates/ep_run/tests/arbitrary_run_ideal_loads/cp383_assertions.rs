//! CP383 post-saturation dehumidifying total-output capacity-guard assertions.

use serde_json::{Map, Value, json};

#[path = "cp384_assertions.rs"]
mod cp384_assertions;

const CP321_KEY: &str = "purchased_air_calc_cooling_capacity_zero_flow_reset_lifecycle";
const CP340_KEY: &str =
    "purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_guard_lifecycle";
const CP382_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_lifecycle";
const CP383_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard_lifecycle";
const ORDER: [&str; 4] = [
    "read-retained-cooling-total-output-for-post-saturation-dehumidification-maximum-capacity-comparison",
    "read-retained-maximum-total-cooling-capacity-for-post-saturation-dehumidification-total-output-comparison",
    "compare-post-saturation-dehumidification-cooling-total-output-strictly-greater-than-maximum-total-cooling-capacity",
    "enter-post-saturation-dehumidification-total-output-capacity-adjustment-body-if-comparison-satisfied",
];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let cp382 = &runtime[CP382_KEY];
    let cp383 = &runtime[CP383_KEY];
    assert_eq!(
        cp383["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2268"
    );
    assert_eq!(
        cp383["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2269"
    );
    assert_eq!(cp383["latest"]["source_order"], json!(ORDER));

    let base_routes = [
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
        .chain(base_routes)
    {
        assert_eq!(
            cp383[field], cp382[field],
            "CP383 must retain CP382 {field}"
        );
    }
    for field in &base_routes[5..] {
        assert_eq!(cp383[*field], 0, "public direct CP383 {field}");
    }
    assert_route_partitions(cp382, cp383);

    let evaluations = count(cp382, "dehumidification_total_output_assignment_count");
    for field in evaluation_counter_fields() {
        assert_eq!(cp383[field], evaluations, "CP383 {field}");
    }
    let bodies = count(
        cp383,
        "dehumidification_total_output_capacity_adjustment_body_entry_count",
    );
    let guard_false = count(
        cp383,
        "dehumidification_total_output_capacity_guard_false_fallthrough_count",
    );
    assert_eq!(bodies + guard_false, evaluations);
    assert_eq!(
        cp383["cooling_total_output_strictly_greater_than_maximum_total_cooling_capacity_count"],
        bodies
    );
    assert_eq!(
        cp383["source_site_execution_count"],
        3 * evaluations + bodies
    );

    let latest = &cp383["latest"];
    assert_latest_predecessor_lineage(cp382, latest);
    if latest["dehumidification_total_output_capacity_guard_evaluated"] == true {
        assert_active_guard(runtime, latest);
    } else {
        assert_skipped_guard(latest);
    }
    assert_numerical_nonfeed_and_unchanged_enthalpy(cp383);
    cp384_assertions::assert_direct(runtime, results);
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP383_KEY));
    assert!(
        runtime[CP383_KEY].is_null(),
        "non-direct runtime must not publish CP383 evidence"
    );
    cp384_assertions::assert_non_direct(runtime);
}

fn assert_route_partitions(cp382: &Value, cp383: &Value) {
    let mut refined = [
        "unit_off_skip_count",
        "non_cooling_skip_count",
        "positive_guard_false_fallthrough_skip_count",
    ]
    .into_iter()
    .map(|field| count(cp383, field))
    .sum::<u64>();
    let mut skip_routes = refined;
    for prefix in lineage_prefixes() {
        for suffix in [
            "capacity_guard_false_count",
            "dehumidification_guard_false_count",
            "dehumidification_total_output_assignment_count",
        ] {
            let field = format!("{prefix}_{suffix}");
            assert_eq!(cp383[field.as_str()], cp382[field.as_str()]);
        }
        let capacity_false = count(cp383, &format!("{prefix}_capacity_guard_false_count"));
        let dehumidification_false = count(
            cp383,
            &format!("{prefix}_dehumidification_guard_false_count"),
        );
        let assignment = count(
            cp383,
            &format!("{prefix}_dehumidification_total_output_assignment_count"),
        );
        assert_eq!(
            assignment,
            count(
                cp382,
                &format!("{prefix}_dehumidification_body_entry_count")
            ),
            "CP383 {prefix} assignment/body-entry equivalence"
        );
        let body = count(
            cp383,
            &format!("{prefix}_dehumidification_total_output_capacity_adjustment_body_entry_count"),
        );
        let guard_false = count(
            cp383,
            &format!(
                "{prefix}_dehumidification_total_output_capacity_guard_false_fallthrough_count"
            ),
        );
        assert_eq!(body + guard_false, assignment, "CP383 {prefix} split");
        skip_routes += capacity_false + dehumidification_false;
        refined += capacity_false + dehumidification_false + guard_false + body;
    }
    assert_eq!(refined, count(cp383, "transition_count"));
    let total_guard_outcomes = count(
        cp383,
        "dehumidification_total_output_capacity_adjustment_body_entry_count",
    ) + count(
        cp383,
        "dehumidification_total_output_capacity_guard_false_fallthrough_count",
    );
    assert_eq!(skip_routes + total_guard_outcomes, refined);
}

fn assert_latest_predecessor_lineage(cp382: &Value, latest: &Value) {
    for field in [
        "unit_off_skipped",
        "non_cooling_skipped",
        "positive_guard_false_fallthrough_skipped",
        "heating_availability_guard_false_fallthrough",
        "humidification_control_guard_false_fallthrough",
        "dehumidification_control_humidistat_maximum_assignment_executed",
        "dehumidification_control_none_maximum_assignment_executed",
        "dehumidification_control_guard_false_fallthrough",
        "predecessor_capacity_limit_guard_evaluated",
        "predecessor_capacity_limit_body_entered",
        "predecessor_active_capacity_limit_guard_false_fallthrough",
        "predecessor_dehumidification_guard_evaluated",
        "predecessor_dehumidification_body_entered",
        "predecessor_dehumidification_guard_false_fallthrough",
    ] {
        assert_eq!(latest[field], cp382["latest"][field], "CP383 CP382 {field}");
    }
    assert_eq!(
        latest["predecessor_dehumidification_total_output_assignment_executed"],
        cp382["latest"]["dehumidification_total_output_assignment_executed"]
    );
    assert_eq!(
        latest["dehumidification_total_output_capacity_guard_evaluated"],
        cp382["latest"]["dehumidification_total_output_assignment_executed"]
    );
}

fn assert_active_guard(runtime: &Value, latest: &Value) {
    for field in active_flag_fields() {
        assert_eq!(latest[field], true, "active CP383 {field}");
    }
    assert_eq!(
        latest["cooling_total_output_w_ieee_bits"],
        runtime[CP382_KEY]["latest"]["cooling_total_output_w_ieee_bits"]
    );
    assert_eq!(
        latest["maximum_total_cooling_capacity_w_ieee_bits"],
        runtime[CP321_KEY]["latest"]["maximum_total_cooling_capacity_w_ieee_bits"]
    );
    assert_eq!(
        latest["maximum_total_cooling_capacity_w_ieee_bits"],
        runtime[CP340_KEY]["latest"]["maximum_total_cooling_capacity_w_ieee_bits"]
    );
    let comparison = ieee_value(latest, "cooling_total_output_w")
        > ieee_value(latest, "maximum_total_cooling_capacity_w");
    assert_eq!(
        latest["cooling_total_output_strictly_greater_than_maximum_total_cooling_capacity"],
        comparison
    );
    assert_eq!(
        latest["dehumidification_total_output_capacity_adjustment_body_entered"],
        comparison
    );
    assert_eq!(
        latest["dehumidification_total_output_capacity_guard_false_fallthrough"],
        !comparison
    );
}

fn assert_skipped_guard(latest: &Value) {
    for field in active_flag_fields() {
        assert_eq!(latest[field], false, "skipped CP383 {field}");
    }
    for field in ["cooling_total_output_w", "maximum_total_cooling_capacity_w"] {
        assert!(latest[field].is_null(), "skipped CP383 {field}");
        assert!(
            latest[format!("{field}_ieee_bits")].is_null(),
            "skipped CP383 {field} bits"
        );
    }
    assert!(
        latest["cooling_total_output_strictly_greater_than_maximum_total_cooling_capacity"]
            .is_null()
    );
    assert_eq!(
        latest["dehumidification_total_output_capacity_adjustment_body_entered"],
        false
    );
    assert_eq!(
        latest["dehumidification_total_output_capacity_guard_false_fallthrough"],
        false
    );
}

fn lineage_prefixes() -> [&'static str; 5] {
    [
        "heating_availability_guard_false_fallthrough",
        "humidification_control_guard_false_fallthrough",
        "dehumidification_control_humidistat_maximum_assignment",
        "dehumidification_control_none_maximum_assignment",
        "dehumidification_control_guard_false_fallthrough",
    ]
}

fn active_flag_fields() -> [&'static str; 7] {
    [
        "dehumidification_total_output_capacity_guard_evaluated",
        "cp382_cooling_total_output_owned_read",
        "cooling_total_output_read",
        "cp321_maximum_total_cooling_capacity_owned_read",
        "cp340_same_call_maximum_total_cooling_capacity_bit_corroborated",
        "maximum_total_cooling_capacity_read",
        "cooling_total_output_maximum_total_cooling_capacity_comparison_evaluated",
    ]
}

fn evaluation_counter_fields() -> [&'static str; 7] {
    [
        "dehumidification_total_output_capacity_guard_evaluation_count",
        "cp382_cooling_total_output_owned_read_count",
        "cooling_total_output_read_count",
        "cp321_maximum_total_cooling_capacity_owned_read_count",
        "cp340_same_call_maximum_total_cooling_capacity_bit_corroboration_count",
        "maximum_total_cooling_capacity_read_count",
        "cooling_total_output_maximum_total_cooling_capacity_comparison_count",
    ]
}

fn ieee_value(value: &Value, field: &str) -> f64 {
    f64::from_bits(bits(value, field))
}

fn bits(value: &Value, field: &str) -> u64 {
    let text = value[format!("{field}_ieee_bits")].as_str();
    assert!(text.is_some(), "CP383 {field} IEEE bits");
    let parsed = u64::from_str_radix(text.unwrap_or_default().trim_start_matches("0x"), 16);
    assert!(parsed.is_ok(), "CP383 {field} IEEE bits parse");
    parsed.unwrap_or_default()
}

fn count(value: &Value, field: &str) -> u64 {
    let count = value[field].as_u64();
    assert!(count.is_some(), "CP383 {field} count");
    count.unwrap_or_default()
}

fn assert_numerical_nonfeed_and_unchanged_enthalpy(cp383: &Value) {
    let serialized = cp383.to_string();
    for forbidden in ["adjusted_", "assigned_", "resulting_", "reconciled_"] {
        assert!(
            !serialized.contains(forbidden),
            "CP383 guard evidence must not feed or serialize {forbidden} values"
        );
    }
    // CP383 records only the guard decision. The excluded body owns every later numerical write.
}
