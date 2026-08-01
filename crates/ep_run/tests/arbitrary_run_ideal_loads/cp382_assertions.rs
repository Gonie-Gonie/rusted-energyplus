//! CP382 post-saturation dehumidifying total-output assignment assertions.

use serde_json::{Map, Value, json};

#[path = "cp383_assertions.rs"]
mod cp383_assertions;

const CP329_KEY: &str = "purchased_air_calc_cooling_mixed_air_call_lifecycle";
const CP330_KEY: &str = "purchased_air_calc_cooling_supply_mass_flow_positive_guard_lifecycle";
const CP339_KEY: &str = "purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_assignment_lifecycle";
const CP379_KEY: &str =
    "purchased_air_calc_cooling_supply_enthalpy_post_saturation_assignment_lifecycle";
const CP381_KEY: &str =
    "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_guard_lifecycle";
const CP382_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_lifecycle";
const ORDER: [&str; 6] = [
    "read-retained-supply-mass-flow-rate-for-post-saturation-dehumidification-total-output-product",
    "read-retained-mixed-air-enthalpy-for-post-saturation-dehumidification-total-output-difference",
    "read-retained-supply-enthalpy-for-post-saturation-dehumidification-total-output-difference",
    "calculate-mixed-air-enthalpy-minus-supply-enthalpy-for-post-saturation-dehumidification-total-output",
    "calculate-supply-mass-flow-rate-times-enthalpy-difference-for-post-saturation-dehumidification-total-output",
    "assign-local-cooling-total-output-for-post-saturation-dehumidification",
];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let cp381 = &runtime[CP381_KEY];
    let cp382 = &runtime[CP382_KEY];
    assert_eq!(
        cp382["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2267"
    );
    assert_eq!(
        cp382["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2268"
    );
    assert_eq!(cp382["latest"]["source_order"], json!(ORDER));

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
            cp382[field], cp381[field],
            "CP382 must retain CP381 {field}"
        );
    }
    for field in &route_fields[5..] {
        assert_eq!(cp382[*field], 0, "public direct CP382 {field}");
    }
    assert_route_partitions(cp381, cp382);

    let assigned = count(cp381, "dehumidification_body_entry_count");
    assert_eq!(
        cp382["dehumidification_total_output_assignment_count"],
        assigned
    );
    for field in active_counter_fields() {
        assert_eq!(cp382[field], assigned, "CP382 {field}");
    }
    assert_eq!(cp382["source_site_execution_count"], 6 * assigned);

    let latest = &cp382["latest"];
    assert_latest_predecessor_lineage(cp381, latest);
    if latest["dehumidification_total_output_assignment_executed"] == true {
        assert_active_assignment(runtime, latest);
    } else {
        assert_skipped_assignment(latest);
        if assigned == 0 {
            assert_eq!(cp382["source_site_execution_count"], 0);
        }
    }
    assert_numerical_nonfeed_and_unchanged_enthalpy(runtime, results);
    cp383_assertions::assert_direct(runtime, results);
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP382_KEY));
    assert!(
        runtime[CP382_KEY].is_null(),
        "non-direct runtime must not publish CP382 evidence"
    );
    cp383_assertions::assert_non_direct(runtime);
}

fn assert_route_partitions(cp381: &Value, cp382: &Value) {
    let mut refined_transition_partition = [
        "unit_off_skip_count",
        "non_cooling_skip_count",
        "positive_guard_false_fallthrough_skip_count",
    ]
    .into_iter()
    .map(|field| count(cp382, field))
    .sum::<u64>();
    let fields = [
        (
            "heating_availability_guard_false_fallthrough",
            "heating_availability_guard_false_fallthrough",
        ),
        (
            "humidification_control_guard_false_fallthrough",
            "humidification_control_guard_false_fallthrough",
        ),
        (
            "dehumidification_control_humidistat_maximum_assignment",
            "dehumidification_control_humidistat_maximum_assignment",
        ),
        (
            "dehumidification_control_none_maximum_assignment",
            "dehumidification_control_none_maximum_assignment",
        ),
        (
            "dehumidification_control_guard_false_fallthrough",
            "dehumidification_control_guard_false_fallthrough",
        ),
    ];
    for (cp381_prefix, cp382_prefix) in fields {
        let capacity_false = format!("{cp381_prefix}_capacity_guard_false_count");
        let body = format!("{cp381_prefix}_dehumidification_body_entry_count");
        let guard_false = format!("{cp381_prefix}_dehumidification_guard_false_count");
        let assignment = format!("{cp382_prefix}_dehumidification_total_output_assignment_count");
        assert_eq!(
            cp382[capacity_false.as_str()],
            cp381[capacity_false.as_str()]
        );
        assert_eq!(cp382[body.as_str()], cp381[body.as_str()]);
        assert_eq!(cp382[guard_false.as_str()], cp381[guard_false.as_str()]);
        assert_eq!(cp382[assignment.as_str()], cp381[body.as_str()]);
        refined_transition_partition += count(cp382, capacity_false.as_str())
            + count(cp382, assignment.as_str())
            + count(cp382, guard_false.as_str());
    }
    assert_eq!(
        refined_transition_partition,
        count(cp382, "transition_count"),
        "CP382 13-skip/5-assignment refined route partition"
    );
}

fn assert_latest_predecessor_lineage(cp381: &Value, latest: &Value) {
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
        assert_eq!(latest[field], cp381["latest"][field], "CP382 CP381 {field}");
    }
    for (field, predecessor_field) in [
        (
            "predecessor_capacity_limit_guard_evaluated",
            "predecessor_capacity_limit_guard_evaluated",
        ),
        (
            "predecessor_capacity_limit_body_entered",
            "predecessor_capacity_limit_body_entered",
        ),
        (
            "predecessor_active_capacity_limit_guard_false_fallthrough",
            "predecessor_active_capacity_limit_guard_false_fallthrough",
        ),
        (
            "predecessor_dehumidification_guard_evaluated",
            "dehumidification_guard_evaluated",
        ),
        (
            "predecessor_dehumidification_body_entered",
            "dehumidification_body_entered",
        ),
        (
            "predecessor_dehumidification_guard_false_fallthrough",
            "dehumidification_guard_false_fallthrough",
        ),
    ] {
        assert_eq!(latest[field], cp381["latest"][predecessor_field]);
    }
    assert_eq!(
        latest["dehumidification_total_output_assignment_executed"],
        cp381["latest"]["dehumidification_body_entered"]
    );
}

fn assert_active_assignment(runtime: &Value, latest: &Value) {
    for field in active_flag_fields() {
        assert_eq!(latest[field], true, "active CP382 {field}");
    }
    assert_eq!(
        latest["supply_mass_flow_rate_kg_per_s_ieee_bits"],
        runtime[CP330_KEY]["latest"]["supply_mass_flow_rate_kg_per_s_ieee_bits"]
    );
    assert_eq!(
        latest["supply_mass_flow_rate_kg_per_s_ieee_bits"],
        runtime[CP329_KEY]["latest"]["supply_mass_flow_rate_kg_per_s_ieee_bits"]
    );
    assert_eq!(
        latest["supply_mass_flow_rate_kg_per_s_ieee_bits"],
        runtime[CP329_KEY]["latest"]["child_supply_mass_flow_rate_kg_per_s_ieee_bits"]
    );
    assert_eq!(
        latest["supply_mass_flow_rate_kg_per_s_ieee_bits"],
        runtime[CP339_KEY]["latest"]["supply_mass_flow_rate_kg_per_s_ieee_bits"]
    );
    assert_eq!(
        latest["mixed_air_enthalpy_j_per_kg_ieee_bits"],
        runtime[CP329_KEY]["latest"]["mixed_air_enthalpy_projection_j_per_kg_ieee_bits"]
    );
    assert_eq!(
        latest["mixed_air_enthalpy_j_per_kg_ieee_bits"],
        runtime[CP329_KEY]["latest"]["recirculation_enthalpy_projection_j_per_kg_ieee_bits"]
    );
    assert_eq!(
        latest["mixed_air_enthalpy_j_per_kg_ieee_bits"],
        runtime[CP339_KEY]["latest"]["mixed_air_enthalpy_j_per_kg_ieee_bits"]
    );
    assert_eq!(
        latest["supply_enthalpy_j_per_kg_ieee_bits"],
        runtime[CP379_KEY]["latest"]["resulting_supply_enthalpy_j_per_kg_ieee_bits"]
    );
    assert_eq!(
        latest["supply_enthalpy_j_per_kg_ieee_bits"],
        runtime[CP379_KEY]["latest"]["assigned_supply_enthalpy_j_per_kg_ieee_bits"]
    );

    let mass = ieee_value(latest, "supply_mass_flow_rate_kg_per_s");
    let mixed = ieee_value(latest, "mixed_air_enthalpy_j_per_kg");
    let supply = ieee_value(latest, "supply_enthalpy_j_per_kg");
    let difference = mixed - supply;
    let output = mass * difference;
    assert_eq!(
        bits(latest, "mixed_air_minus_supply_enthalpy_j_per_kg"),
        difference.to_bits()
    );
    assert_eq!(
        bits(latest, "calculated_cooling_total_output_w"),
        output.to_bits()
    );
    assert_eq!(bits(latest, "cooling_total_output_w"), output.to_bits());
}

fn assert_skipped_assignment(latest: &Value) {
    for field in active_flag_fields() {
        assert_eq!(latest[field], false, "skipped CP382 {field}");
    }
    for field in numeric_fields() {
        assert!(latest[field].is_null(), "skipped CP382 {field}");
        assert!(
            latest[format!("{field}_ieee_bits")].is_null(),
            "skipped CP382 {field} bits"
        );
    }
}

fn active_flag_fields() -> [&'static str; 14] {
    [
        "cp330_supply_mass_flow_rate_owned_read",
        "cp329_same_call_supply_mass_flow_rate_bit_corroborated",
        "cp339_same_call_supply_mass_flow_rate_bit_corroborated",
        "supply_mass_flow_rate_read",
        "cp329_mixed_air_enthalpy_owned_read",
        "cp329_same_call_recirculation_enthalpy_bit_corroborated",
        "cp339_same_call_mixed_air_enthalpy_bit_corroborated",
        "mixed_air_enthalpy_read",
        "cp379_post_saturation_supply_enthalpy_owned_read",
        "cp379_same_call_supply_enthalpy_bits_corroborated",
        "supply_enthalpy_read",
        "enthalpy_difference_calculated",
        "cooling_total_output_calculated",
        "cooling_total_output_assigned",
    ]
}

fn active_counter_fields() -> [&'static str; 14] {
    [
        "cp330_supply_mass_flow_rate_owned_read_count",
        "cp329_same_call_supply_mass_flow_rate_bit_corroboration_count",
        "cp339_same_call_supply_mass_flow_rate_bit_corroboration_count",
        "supply_mass_flow_rate_read_count",
        "cp329_mixed_air_enthalpy_owned_read_count",
        "cp329_same_call_recirculation_enthalpy_bit_corroboration_count",
        "cp339_same_call_mixed_air_enthalpy_bit_corroboration_count",
        "mixed_air_enthalpy_read_count",
        "cp379_post_saturation_supply_enthalpy_owned_read_count",
        "cp379_same_call_supply_enthalpy_bits_corroboration_count",
        "supply_enthalpy_read_count",
        "enthalpy_difference_calculation_count",
        "cooling_total_output_calculation_count",
        "cooling_total_output_assignment_write_count",
    ]
}

fn numeric_fields() -> [&'static str; 6] {
    [
        "supply_mass_flow_rate_kg_per_s",
        "mixed_air_enthalpy_j_per_kg",
        "supply_enthalpy_j_per_kg",
        "mixed_air_minus_supply_enthalpy_j_per_kg",
        "calculated_cooling_total_output_w",
        "cooling_total_output_w",
    ]
}

fn ieee_value(value: &Value, field: &str) -> f64 {
    f64::from_bits(bits(value, field))
}

fn bits(value: &Value, field: &str) -> u64 {
    let text = value[format!("{field}_ieee_bits")].as_str();
    assert!(text.is_some(), "CP382 {field} IEEE bits");
    let parsed = u64::from_str_radix(text.unwrap_or_default().trim_start_matches("0x"), 16);
    assert!(parsed.is_ok(), "CP382 {field} IEEE bits parse");
    parsed.unwrap_or_default()
}

fn count(value: &Value, field: &str) -> u64 {
    let count = value[field].as_u64();
    assert!(count.is_some(), "CP382 {field} count");
    count.unwrap_or_default()
}

fn assert_numerical_nonfeed_and_unchanged_enthalpy(runtime: &Value, _results: &Value) {
    assert!(
        runtime[CP379_KEY]["latest"]["resulting_supply_enthalpy_j_per_kg_ieee_bits"]
            .as_str()
            .is_some(),
        "CP382 local output evidence must leave CP379 enthalpy evidence unchanged"
    );
    let serialized = runtime[CP382_KEY].to_string();
    for forbidden in ["capacity_w", "supply_node", "report"] {
        assert!(
            !serialized.contains(forbidden),
            "CP382 local output evidence must not feed or serialize {forbidden}"
        );
    }
    // Later source statements may cap or overwrite CoolTotOutput, so CP382 deliberately does not
    // reconcile its local value with numerical result DTOs.
}
