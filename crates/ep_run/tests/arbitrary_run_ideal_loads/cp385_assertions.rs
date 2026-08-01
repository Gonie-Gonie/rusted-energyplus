//! CP385 post-saturation capacity-limited dehumidifying supply-enthalpy assertions.

use serde_json::{Map, Value, json};

#[path = "cp386_assertions.rs"]
mod cp386_assertions;

const CP382_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_lifecycle";
const CP384_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment_lifecycle";
const CP385_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_lifecycle";
const ORDER: [&str; 6] = [
    "read-retained-mixed-air-enthalpy-for-post-saturation-capacity-limited-dehumidification-supply-enthalpy-difference",
    "read-retained-cooling-total-output-for-post-saturation-capacity-limited-dehumidification-specific-cooling-output-division",
    "read-retained-supply-mass-flow-rate-for-post-saturation-capacity-limited-dehumidification-specific-cooling-output-division",
    "calculate-cooling-total-output-divided-by-supply-mass-flow-rate-for-post-saturation-capacity-limited-dehumidification-supply-enthalpy",
    "calculate-mixed-air-enthalpy-minus-specific-cooling-output-for-post-saturation-capacity-limited-dehumidification-supply-enthalpy",
    "assign-local-supply-enthalpy-after-post-saturation-capacity-limited-dehumidification-total-output-adjustment",
];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let cp382 = &runtime[CP382_KEY];
    let cp384 = &runtime[CP384_KEY];
    let cp385 = &runtime[CP385_KEY];
    assert_eq!(
        cp385["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2270"
    );
    assert_eq!(
        cp385["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2272"
    );
    assert_eq!(cp385["latest"]["source_order"], json!(ORDER));

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
            cp385[field], cp384[field],
            "CP385 must retain CP384 {field}"
        );
    }
    for field in &base_routes[5..] {
        assert_eq!(cp385[*field], 0, "public direct CP385 {field}");
    }
    assert_route_partitions(cp384, cp385);

    let assignments = count(
        cp384,
        "dehumidification_total_output_maximum_capacity_assignment_count",
    );
    let guard_false = count(
        cp384,
        "dehumidification_total_output_capacity_guard_false_fallthrough_count",
    );
    let evaluations = assignments + guard_false;
    for field in [
        "dehumidification_total_output_capacity_guard_evaluation_count",
        "dehumidification_total_output_capacity_guard_false_fallthrough_count",
        "dehumidification_total_output_maximum_capacity_assignment_count",
    ] {
        assert_eq!(cp385[field], cp384[field], "CP385 {field}");
    }
    assert_eq!(
        cp385["post_saturation_capacity_limited_dehumidification_supply_enthalpy_assignment_count"],
        assignments
    );
    assert_eq!(cp385["source_site_execution_count"], 6 * assignments);
    assert_eq!(
        cp385["cp379_retained_supply_enthalpy_owned_read_count"],
        evaluations
    );
    for field in active_counter_fields() {
        assert_eq!(cp385[field], assignments, "CP385 {field}");
    }

    let latest = &cp385["latest"];
    assert_latest_predecessor_lineage(&cp384["latest"], latest);
    if latest["supply_enthalpy_assignment_executed"] == true {
        assert_body_assignment(&cp382["latest"], &cp384["latest"], latest);
    } else if latest["dehumidification_total_output_capacity_guard_false_fallthrough"] == true {
        assert_guard_false(&cp382["latest"], latest);
    } else {
        assert_outer_skip(latest);
    }
    assert_numerical_nonfeed_and_local_enthalpy_only(cp385, results);
    cp386_assertions::assert_direct(runtime, results);
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP385_KEY));
    assert!(
        runtime[CP385_KEY].is_null(),
        "non-direct runtime must not publish CP385 evidence"
    );
    cp386_assertions::assert_non_direct(runtime);
}

fn assert_route_partitions(cp384: &Value, cp385: &Value) {
    let mut refined = [
        "unit_off_skip_count",
        "non_cooling_skip_count",
        "positive_guard_false_fallthrough_skip_count",
    ]
    .into_iter()
    .map(|field| count(cp385, field))
    .sum::<u64>();
    for prefix in lineage_prefixes() {
        for suffix in [
            "capacity_guard_false_count",
            "dehumidification_guard_false_count",
            "dehumidification_total_output_assignment_count",
            "dehumidification_total_output_capacity_guard_false_fallthrough_count",
            "dehumidification_total_output_maximum_capacity_assignment_count",
        ] {
            let field = format!("{prefix}_{suffix}");
            assert_eq!(cp385[field.as_str()], cp384[field.as_str()]);
        }
        let capacity_false = count(cp385, &format!("{prefix}_capacity_guard_false_count"));
        let dehumidification_false = count(
            cp385,
            &format!("{prefix}_dehumidification_guard_false_count"),
        );
        let guard_false = count(
            cp385,
            &format!(
                "{prefix}_dehumidification_total_output_capacity_guard_false_fallthrough_count"
            ),
        );
        let assignment = count(
            cp385,
            &format!("{prefix}_dehumidification_total_output_maximum_capacity_assignment_count"),
        );
        refined += capacity_false + dehumidification_false + guard_false + assignment;
    }
    assert_eq!(refined, count(cp385, "transition_count"));
}

fn assert_latest_predecessor_lineage(cp384: &Value, latest: &Value) {
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
        "predecessor_dehumidification_total_output_assignment_executed",
        "predecessor_dehumidification_total_output_capacity_guard_evaluated",
        "predecessor_dehumidification_total_output_capacity_adjustment_body_entered",
        "predecessor_dehumidification_total_output_capacity_guard_false_fallthrough",
        "dehumidification_total_output_capacity_guard_false_fallthrough",
        "dehumidification_total_output_maximum_capacity_assignment_executed",
    ] {
        assert_eq!(latest[field], cp384[field], "CP385 CP384 {field}");
    }
    assert_eq!(
        latest["supply_enthalpy_assignment_executed"],
        cp384["dehumidification_total_output_maximum_capacity_assignment_executed"]
    );
}

fn assert_body_assignment(cp382: &Value, cp384: &Value, latest: &Value) {
    for field in [
        "cp379_retained_supply_enthalpy_owned_read",
        "cp329_retained_mixed_air_enthalpy_owned_read",
        "mixed_air_enthalpy_read",
        "cp384_retained_cooling_total_output_owned_read",
        "cooling_total_output_read",
        "cp330_retained_supply_mass_flow_rate_owned_read",
        "supply_mass_flow_rate_read",
        "specific_cooling_output_calculated",
        "supply_enthalpy_difference_calculated",
        "supply_enthalpy_assigned",
    ] {
        assert_eq!(latest[field], true, "active CP385 {field}");
    }
    assert_eq!(
        latest["preexisting_supply_enthalpy_j_per_kg_ieee_bits"],
        cp382["supply_enthalpy_j_per_kg_ieee_bits"]
    );
    assert_eq!(
        latest["mixed_air_enthalpy_j_per_kg_ieee_bits"],
        cp382["mixed_air_enthalpy_j_per_kg_ieee_bits"]
    );
    assert_eq!(
        latest["cooling_total_output_w_ieee_bits"],
        cp384["resulting_cooling_total_output_w_ieee_bits"]
    );
    assert_eq!(
        latest["supply_mass_flow_rate_kg_per_s_ieee_bits"],
        cp382["supply_mass_flow_rate_kg_per_s_ieee_bits"]
    );

    let output = ieee_value(latest, "cooling_total_output_w");
    let flow = ieee_value(latest, "supply_mass_flow_rate_kg_per_s");
    let mixed = ieee_value(latest, "mixed_air_enthalpy_j_per_kg");
    let specific = output / flow;
    let calculated = mixed - specific;
    assert_eq!(
        bits(latest, "specific_cooling_output_j_per_kg"),
        specific.to_bits()
    );
    for field in [
        "calculated_supply_enthalpy_j_per_kg",
        "assigned_supply_enthalpy_j_per_kg",
        "resulting_supply_enthalpy_j_per_kg",
    ] {
        assert_eq!(
            bits(latest, field),
            calculated.to_bits(),
            "active CP385 {field}"
        );
    }
}

fn assert_guard_false(cp382: &Value, latest: &Value) {
    assert_eq!(latest["cp379_retained_supply_enthalpy_owned_read"], true);
    assert_eq!(latest["supply_enthalpy_assignment_executed"], false);
    let preexisting_bits = &cp382["supply_enthalpy_j_per_kg_ieee_bits"];
    assert_eq!(
        &latest["preexisting_supply_enthalpy_j_per_kg_ieee_bits"],
        preexisting_bits
    );
    assert_eq!(
        &latest["resulting_supply_enthalpy_j_per_kg_ieee_bits"],
        preexisting_bits
    );
    for field in active_flag_fields() {
        assert_eq!(latest[field], false, "guard-false CP385 {field}");
    }
    for field in &numeric_fields()[1..7] {
        assert!(latest[*field].is_null(), "guard-false CP385 {field}");
        assert!(
            latest[format!("{field}_ieee_bits")].is_null(),
            "guard-false CP385 {field} bits"
        );
    }
}

fn assert_outer_skip(latest: &Value) {
    assert_eq!(latest["supply_enthalpy_assignment_executed"], false);
    assert_eq!(latest["cp379_retained_supply_enthalpy_owned_read"], false);
    for field in active_flag_fields() {
        assert_eq!(latest[field], false, "outer-skip CP385 {field}");
    }
    for field in numeric_fields() {
        assert!(latest[field].is_null(), "outer-skip CP385 {field}");
        assert!(
            latest[format!("{field}_ieee_bits")].is_null(),
            "outer-skip CP385 {field} bits"
        );
    }
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

fn active_counter_fields() -> [&'static str; 9] {
    [
        "cp329_retained_mixed_air_enthalpy_owned_read_count",
        "mixed_air_enthalpy_read_count",
        "cp384_retained_cooling_total_output_owned_read_count",
        "cooling_total_output_read_count",
        "cp330_retained_supply_mass_flow_rate_owned_read_count",
        "supply_mass_flow_rate_read_count",
        "specific_cooling_output_calculation_count",
        "supply_enthalpy_difference_calculation_count",
        "supply_enthalpy_assignment_write_count",
    ]
}

fn active_flag_fields() -> [&'static str; 9] {
    [
        "cp329_retained_mixed_air_enthalpy_owned_read",
        "mixed_air_enthalpy_read",
        "cp384_retained_cooling_total_output_owned_read",
        "cooling_total_output_read",
        "cp330_retained_supply_mass_flow_rate_owned_read",
        "supply_mass_flow_rate_read",
        "specific_cooling_output_calculated",
        "supply_enthalpy_difference_calculated",
        "supply_enthalpy_assigned",
    ]
}

fn numeric_fields() -> [&'static str; 8] {
    [
        "preexisting_supply_enthalpy_j_per_kg",
        "mixed_air_enthalpy_j_per_kg",
        "cooling_total_output_w",
        "supply_mass_flow_rate_kg_per_s",
        "specific_cooling_output_j_per_kg",
        "calculated_supply_enthalpy_j_per_kg",
        "assigned_supply_enthalpy_j_per_kg",
        "resulting_supply_enthalpy_j_per_kg",
    ]
}

fn ieee_value(value: &Value, field: &str) -> f64 {
    f64::from_bits(bits(value, field))
}

fn bits(value: &Value, field: &str) -> u64 {
    let text = value[format!("{field}_ieee_bits")].as_str();
    assert!(text.is_some(), "CP385 {field} IEEE bits");
    let parsed = u64::from_str_radix(text.unwrap_or_default().trim_start_matches("0x"), 16);
    assert!(parsed.is_ok(), "CP385 {field} IEEE bits parse");
    parsed.unwrap_or_default()
}

fn count(value: &Value, field: &str) -> u64 {
    let count = value[field].as_u64();
    assert!(count.is_some(), "CP385 {field} count");
    count.unwrap_or_default()
}

fn assert_numerical_nonfeed_and_local_enthalpy_only(cp385: &Value, results: &Value) {
    let serialized = cp385.to_string();
    for forbidden in [
        "supply_node",
        "report",
        "reconciled",
        "numerical_dto",
        "direct_zone_purchased_air_coupling_input",
    ] {
        assert!(
            !serialized.to_ascii_lowercase().contains(forbidden),
            "CP385 evidence must not feed {forbidden} state"
        );
    }
    assert!(
        !results.to_string().contains(CP385_KEY),
        "CP385 lifecycle must remain outside numerical result state"
    );
}
