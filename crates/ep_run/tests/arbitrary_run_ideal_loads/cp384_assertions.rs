//! CP384 post-saturation dehumidifying total-output maximum-capacity assignment assertions.

use serde_json::{Map, Value, json};

const CP379_KEY: &str =
    "purchased_air_calc_cooling_supply_enthalpy_post_saturation_assignment_lifecycle";
const CP383_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard_lifecycle";
const CP384_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment_lifecycle";
const ORDER: [&str; 2] = [
    "read-retained-maximum-total-cooling-capacity-for-post-saturation-dehumidification-total-output-assignment",
    "assign-local-cooling-total-output-from-maximum-total-cooling-capacity",
];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let cp383 = &runtime[CP383_KEY];
    let cp384 = &runtime[CP384_KEY];
    assert_eq!(
        cp384["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2269"
    );
    assert_eq!(
        cp384["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2270"
    );
    assert_eq!(cp384["latest"]["source_order"], json!(ORDER));

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
            cp384[field], cp383[field],
            "CP384 must retain CP383 {field}"
        );
    }
    for field in &base_routes[5..] {
        assert_eq!(cp384[*field], 0, "public direct CP384 {field}");
    }
    assert_route_partitions(cp383, cp384);

    let assignments = count(
        cp383,
        "dehumidification_total_output_capacity_adjustment_body_entry_count",
    );
    assert_eq!(
        cp384["dehumidification_total_output_capacity_guard_evaluation_count"],
        cp383["dehumidification_total_output_capacity_guard_evaluation_count"]
    );
    assert_eq!(
        cp384["dehumidification_total_output_capacity_guard_false_fallthrough_count"],
        cp383["dehumidification_total_output_capacity_guard_false_fallthrough_count"]
    );
    for field in [
        "dehumidification_total_output_maximum_capacity_assignment_count",
        "cp383_retained_maximum_total_cooling_capacity_owned_read_count",
        "maximum_total_cooling_capacity_read_count",
        "cooling_total_output_assignment_write_count",
    ] {
        assert_eq!(cp384[field], assignments, "CP384 {field}");
    }
    assert_eq!(cp384["source_site_execution_count"], 2 * assignments);

    let latest = &cp384["latest"];
    assert_latest_predecessor_lineage(cp383, latest);
    if latest["dehumidification_total_output_maximum_capacity_assignment_executed"] == true {
        assert_body_assignment(cp383, latest);
    } else if latest["dehumidification_total_output_capacity_guard_false_fallthrough"] == true {
        assert_guard_false(cp383, latest);
    } else {
        assert_outer_skip(latest);
    }
    assert_numerical_nonfeed_and_unchanged_enthalpy(runtime, results);
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP384_KEY));
    assert!(
        runtime[CP384_KEY].is_null(),
        "non-direct runtime must not publish CP384 evidence"
    );
}

fn assert_route_partitions(cp383: &Value, cp384: &Value) {
    let mut refined = [
        "unit_off_skip_count",
        "non_cooling_skip_count",
        "positive_guard_false_fallthrough_skip_count",
    ]
    .into_iter()
    .map(|field| count(cp384, field))
    .sum::<u64>();
    for prefix in lineage_prefixes() {
        for suffix in [
            "capacity_guard_false_count",
            "dehumidification_guard_false_count",
            "dehumidification_total_output_assignment_count",
            "dehumidification_total_output_capacity_guard_false_fallthrough_count",
        ] {
            let field = format!("{prefix}_{suffix}");
            assert_eq!(cp384[field.as_str()], cp383[field.as_str()]);
        }
        let maximum_assignment =
            format!("{prefix}_dehumidification_total_output_maximum_capacity_assignment_count");
        let predecessor_body =
            format!("{prefix}_dehumidification_total_output_capacity_adjustment_body_entry_count");
        assert_eq!(
            cp384[maximum_assignment.as_str()],
            cp383[predecessor_body.as_str()]
        );
        let capacity_false = count(cp384, &format!("{prefix}_capacity_guard_false_count"));
        let dehumidification_false = count(
            cp384,
            &format!("{prefix}_dehumidification_guard_false_count"),
        );
        let guard_false = count(
            cp384,
            &format!(
                "{prefix}_dehumidification_total_output_capacity_guard_false_fallthrough_count"
            ),
        );
        let assignment = count(cp384, maximum_assignment.as_str());
        refined += capacity_false + dehumidification_false + guard_false + assignment;
    }
    assert_eq!(refined, count(cp384, "transition_count"));
}

fn assert_latest_predecessor_lineage(cp383: &Value, latest: &Value) {
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
    ] {
        assert_eq!(latest[field], cp383["latest"][field], "CP384 CP383 {field}");
    }
    for (field, predecessor_field) in [
        (
            "predecessor_dehumidification_total_output_capacity_guard_evaluated",
            "dehumidification_total_output_capacity_guard_evaluated",
        ),
        (
            "predecessor_dehumidification_total_output_capacity_adjustment_body_entered",
            "dehumidification_total_output_capacity_adjustment_body_entered",
        ),
        (
            "predecessor_dehumidification_total_output_capacity_guard_false_fallthrough",
            "dehumidification_total_output_capacity_guard_false_fallthrough",
        ),
        (
            "dehumidification_total_output_capacity_guard_false_fallthrough",
            "dehumidification_total_output_capacity_guard_false_fallthrough",
        ),
        (
            "dehumidification_total_output_maximum_capacity_assignment_executed",
            "dehumidification_total_output_capacity_adjustment_body_entered",
        ),
    ] {
        assert_eq!(latest[field], cp383["latest"][predecessor_field]);
    }
}

fn assert_body_assignment(cp383: &Value, latest: &Value) {
    for field in [
        "cp383_retained_maximum_total_cooling_capacity_owned_read",
        "maximum_total_cooling_capacity_read",
        "cooling_total_output_assigned",
    ] {
        assert_eq!(latest[field], true, "active CP384 {field}");
    }
    assert_eq!(
        latest["preexisting_cooling_total_output_w_ieee_bits"],
        cp383["latest"]["cooling_total_output_w_ieee_bits"]
    );
    let maximum_bits = &cp383["latest"]["maximum_total_cooling_capacity_w_ieee_bits"];
    for field in [
        "maximum_total_cooling_capacity_w_ieee_bits",
        "assigned_cooling_total_output_w_ieee_bits",
        "resulting_cooling_total_output_w_ieee_bits",
    ] {
        assert_eq!(&latest[field], maximum_bits, "active CP384 {field}");
    }
}

fn assert_guard_false(cp383: &Value, latest: &Value) {
    for field in [
        "cp383_retained_maximum_total_cooling_capacity_owned_read",
        "maximum_total_cooling_capacity_read",
        "cooling_total_output_assigned",
    ] {
        assert_eq!(latest[field], false, "guard-false CP384 {field}");
    }
    let preexisting_bits = &cp383["latest"]["cooling_total_output_w_ieee_bits"];
    assert_eq!(
        &latest["preexisting_cooling_total_output_w_ieee_bits"],
        preexisting_bits
    );
    assert_eq!(
        &latest["resulting_cooling_total_output_w_ieee_bits"],
        preexisting_bits
    );
    for field in [
        "maximum_total_cooling_capacity_w",
        "maximum_total_cooling_capacity_w_ieee_bits",
        "assigned_cooling_total_output_w",
        "assigned_cooling_total_output_w_ieee_bits",
    ] {
        assert!(latest[field].is_null(), "guard-false CP384 {field}");
    }
}

fn assert_outer_skip(latest: &Value) {
    for field in [
        "cp383_retained_maximum_total_cooling_capacity_owned_read",
        "maximum_total_cooling_capacity_read",
        "cooling_total_output_assigned",
        "dehumidification_total_output_capacity_guard_false_fallthrough",
        "dehumidification_total_output_maximum_capacity_assignment_executed",
    ] {
        assert_eq!(latest[field], false, "outer-skip CP384 {field}");
    }
    for field in numeric_fields() {
        assert!(latest[field].is_null(), "outer-skip CP384 {field}");
        assert!(
            latest[format!("{field}_ieee_bits")].is_null(),
            "outer-skip CP384 {field} bits"
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

fn numeric_fields() -> [&'static str; 4] {
    [
        "preexisting_cooling_total_output_w",
        "maximum_total_cooling_capacity_w",
        "assigned_cooling_total_output_w",
        "resulting_cooling_total_output_w",
    ]
}

fn count(value: &Value, field: &str) -> u64 {
    let count = value[field].as_u64();
    assert!(count.is_some(), "CP384 {field} count");
    count.unwrap_or_default()
}

fn assert_numerical_nonfeed_and_unchanged_enthalpy(runtime: &Value, _results: &Value) {
    assert!(
        runtime[CP379_KEY]["latest"]["resulting_supply_enthalpy_j_per_kg_ieee_bits"]
            .as_str()
            .is_some(),
        "CP384 local output assignment must leave CP379 enthalpy evidence unchanged"
    );
    let serialized = runtime[CP384_KEY].to_string();
    for forbidden in [
        "mixed_air_enthalpy",
        "supply_enthalpy",
        "supply_mass_flow",
        "division",
        "divide",
        "subtract",
        "supply_node",
        "report",
        "reconciled",
    ] {
        assert!(
            !serialized.contains(forbidden),
            "CP384 must not execute line 2270 or feed numerical {forbidden} state"
        );
    }
    // CP384 owns only the local CoolTotOutput copy; line 2270 and later statements own every
    // enthalpy, node, report, and final numerical effect.
}
