//! CP420 not-dehumidifying sensible-output assignment assertions.

use serde_json::{Map, Value, json};

const CP329_KEY: &str = "purchased_air_calc_cooling_mixed_air_call_lifecycle";
const CP330_KEY: &str = "purchased_air_calc_cooling_supply_mass_flow_positive_guard_lifecycle";
const CP419_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_lifecycle";
const CP420_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_lifecycle";
const ORDER: [&str; 8] = [
    "read-retained-supply-mass-flow-rate-for-post-saturation-capacity-limit-dehumidification-guard-else-branch-sensible-output-first-product",
    "read-local-cp-air-for-post-saturation-capacity-limit-dehumidification-guard-else-branch-sensible-output-first-product",
    "calculate-supply-mass-flow-rate-times-cp-air-for-post-saturation-capacity-limit-dehumidification-guard-else-branch-sensible-output",
    "read-purchased-air-mixed-air-temperature-for-post-saturation-capacity-limit-dehumidification-guard-else-branch-sensible-output-difference",
    "read-purchased-air-supply-temperature-for-post-saturation-capacity-limit-dehumidification-guard-else-branch-sensible-output-difference",
    "calculate-mixed-air-temperature-minus-supply-temperature-for-post-saturation-capacity-limit-dehumidification-guard-else-branch-sensible-output",
    "calculate-mass-flow-cp-air-product-times-temperature-difference-for-post-saturation-capacity-limit-dehumidification-guard-else-branch-sensible-output",
    "assign-local-cooling-sensible-output-for-post-saturation-capacity-limit-dehumidification-guard-else-branch",
];
const PUBLIC: [usize; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 20, 21, 26, 27];
const ACTIVE: [usize; 5] = [4, 7, 10, 13, 16];
const ROUTE_FIELDS: [&str; 10] = [
    "predecessor_route_counts",
    "predecessor_guard_false_fallthrough_route_counts",
    "predecessor_guard_body_entry_route_counts",
    "predecessor_supply_temperature_saturation_assignment_route_counts",
    "predecessor_supply_temperature_mixed_air_limit_route_counts",
    "predecessor_supply_humidity_ratio_assignment_route_counts",
    "predecessor_supply_enthalpy_assignment_route_counts",
    "predecessor_dehumidification_guard_else_branch_entry_route_counts",
    "predecessor_dehumidification_guard_else_branch_cp_air_assignment_route_counts",
    "dehumidification_guard_else_branch_sensible_output_assignment_route_counts",
];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let lifecycle = &runtime[CP420_KEY];
    let predecessor = &runtime[CP419_KEY];
    let mixed_air = &runtime[CP329_KEY];
    let supply_flow = &runtime[CP330_KEY];
    assert_eq!(
        lifecycle["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2331"
    );
    assert_eq!(lifecycle["latest"]["source_order"], json!(ORDER));
    assert_eq!(lifecycle["system"], predecessor["system"]);
    assert_eq!(lifecycle["system"], mixed_air["system"]);
    assert_eq!(lifecycle["system"], supply_flow["system"]);
    assert_eq!(
        lifecycle["transition_count"],
        predecessor["transition_count"]
    );

    for field in ROUTE_FIELDS {
        assert_eq!(array(lifecycle, field).len(), 36, "CP420 {field} width");
        for (index, value) in array(lifecycle, field).iter().enumerate() {
            if !PUBLIC.contains(&index) {
                assert_eq!(count_value(value), 0, "public CP420 {field} route {index}");
            }
        }
    }
    for field in &ROUTE_FIELDS[..9] {
        let predecessor_field = match *field {
            "predecessor_dehumidification_guard_else_branch_cp_air_assignment_route_counts" => {
                "dehumidification_guard_else_branch_cp_air_assignment_route_counts"
            }
            other => other,
        };
        assert_eq!(
            lifecycle[*field], predecessor[predecessor_field],
            "CP420 {field}"
        );
    }

    let routes = array(lifecycle, "predecessor_route_counts");
    let assignments = array(
        lifecycle,
        "dehumidification_guard_else_branch_sensible_output_assignment_route_counts",
    );
    for index in 0..36 {
        let expected = if ACTIVE.contains(&index) {
            count_value(&routes[index])
        } else {
            0
        };
        assert_eq!(
            count_value(&assignments[index]),
            expected,
            "CP420 route {index}"
        );
    }
    assert_eq!(
        lifecycle["predecessor_dehumidification_guard_else_branch_cp_air_assignment_route_counts"],
        lifecycle["dehumidification_guard_else_branch_sensible_output_assignment_route_counts"]
    );
    let transitions = sum(routes);
    let assignment_count = sum(assignments);
    assert_eq!(count(lifecycle, "transition_count"), transitions);
    assert_eq!(
        count(lifecycle, "inactive_transition_count") + assignment_count,
        transitions
    );
    assert_eq!(
        count(
            lifecycle,
            "dehumidification_guard_else_branch_sensible_output_assignment_count"
        ),
        assignment_count
    );
    assert_eq!(
        count(lifecycle, "source_site_execution_count"),
        assignment_count * 8
    );
    for field in [
        "supply_mass_flow_rate_owned_read_count",
        "supply_mass_flow_rate_bit_corroboration_count",
        "supply_mass_flow_rate_read_count",
        "cp_air_owned_read_count",
        "cp_air_read_count",
        "supply_mass_flow_rate_times_cp_air_calculation_count",
        "mixed_air_temperature_owned_read_count",
        "mixed_air_temperature_read_count",
        "supply_temperature_owned_read_count",
        "supply_temperature_read_count",
        "mixed_air_minus_supply_temperature_calculation_count",
        "cooling_sensible_output_calculation_count",
        "cooling_sensible_output_assignment_write_count",
    ] {
        assert_eq!(count(lifecycle, field), assignment_count, "CP420 {field}");
    }
    assert_eq!((54 - 5, 5, 5 * ORDER.len(), 17, 37), (49, 5, 40, 17, 37));

    let latest = &lifecycle["latest"];
    let predecessor_latest = &predecessor["latest"];
    for field in [
        "resulting_supply_humidity_ratio_ieee_bits",
        "resulting_supply_enthalpy_j_per_kg_ieee_bits",
        "resulting_supply_temperature_c_ieee_bits",
    ] {
        assert_eq!(latest[field], predecessor_latest[field], "CP420 {field}");
    }
    let active = latest["post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_executed"]
        .as_bool()
        .expect("CP420 executed marker");
    for flag in [
        "cp330_retained_supply_mass_flow_rate_owned_read",
        "cp329_supply_mass_flow_rate_bit_corroborated",
        "supply_mass_flow_rate_read",
        "cp419_retained_cp_air_owned_read",
        "cp_air_read",
        "supply_mass_flow_rate_times_cp_air_calculated",
        "cp329_retained_mixed_air_temperature_for_sensible_output_owned_read",
        "mixed_air_temperature_read",
        "cp419_retained_supply_temperature_owned_read",
        "supply_temperature_read",
        "mixed_air_minus_supply_temperature_calculated",
        "cooling_sensible_output_calculated",
        "cooling_sensible_output_assigned",
    ] {
        assert_eq!(latest[flag], active, "CP420 {flag}");
    }
    if active {
        let flow = f64::from_bits(bits(latest, "supply_mass_flow_rate_kg_per_s"));
        let cp_air = f64::from_bits(bits(latest, "cp419_cp_air_for_sensible_output_j_per_kg_k"));
        let mixed = f64::from_bits(bits(latest, "mixed_air_temperature_for_sensible_output_c"));
        let supply = f64::from_bits(bits(latest, "supply_temperature_for_sensible_output_c"));
        let first = flow * cp_air;
        let difference = mixed - supply;
        let result = first * difference;
        assert_eq!(
            bits(latest, "supply_mass_flow_rate_kg_per_s"),
            bits(&supply_flow["latest"], "supply_mass_flow_rate_kg_per_s")
        );
        assert_eq!(
            bits(latest, "supply_mass_flow_rate_kg_per_s"),
            bits(&mixed_air["latest"], "supply_mass_flow_rate_kg_per_s")
        );
        assert_eq!(
            bits(latest, "cp419_cp_air_for_sensible_output_j_per_kg_k"),
            bits(predecessor_latest, "cp_air_j_per_kg_k")
        );
        assert_eq!(
            bits(latest, "mixed_air_temperature_for_sensible_output_c"),
            bits(&mixed_air["latest"], "mixed_air_temperature_c")
        );
        assert_eq!(
            bits(latest, "supply_mass_flow_rate_times_cp_air_w_per_k"),
            first.to_bits()
        );
        assert_eq!(
            bits(latest, "mixed_air_minus_supply_temperature_k"),
            difference.to_bits()
        );
        assert_eq!(
            bits(latest, "calculated_cooling_sensible_output_w"),
            result.to_bits()
        );
        assert_eq!(bits(latest, "cooling_sensible_output_w"), result.to_bits());
    } else {
        for field in [
            "supply_mass_flow_rate_kg_per_s",
            "cp419_cp_air_for_sensible_output_j_per_kg_k",
            "supply_mass_flow_rate_times_cp_air_w_per_k",
            "mixed_air_temperature_for_sensible_output_c",
            "supply_temperature_for_sensible_output_c",
            "mixed_air_minus_supply_temperature_k",
            "calculated_cooling_sensible_output_w",
            "cooling_sensible_output_w",
        ] {
            assert!(latest[field].is_null(), "inactive CP420 {field}");
        }
    }
    let object = latest.as_object().expect("CP420 latest object");
    assert_eq!(object.len(), 273);
    assert_eq!(
        object
            .keys()
            .filter(|key| key.ends_with("_ieee_bits"))
            .count(),
        71
    );
    for forbidden in [
        "numerical_dto",
        "direct_zone_purchased_air_coupling_input",
        "reconciled",
    ] {
        assert!(
            !lifecycle
                .to_string()
                .to_ascii_lowercase()
                .contains(forbidden)
        );
    }
    assert!(!results.to_string().contains(CP420_KEY));
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP420_KEY));
    assert!(
        runtime[CP420_KEY].is_null(),
        "non-direct runtime must not publish CP420 evidence"
    );
}

fn bits(value: &Value, field: &str) -> u64 {
    let bits = value[format!("{field}_ieee_bits")]
        .as_str()
        .expect("CP420 IEEE sidecar");
    u64::from_str_radix(bits.trim_start_matches("0x"), 16).expect("CP420 IEEE bits")
}
fn count(value: &Value, field: &str) -> u64 {
    count_value(&value[field])
}
fn count_value(value: &Value) -> u64 {
    value.as_u64().expect("CP420 unsigned count")
}
fn array<'a>(value: &'a Value, field: &str) -> &'a [Value] {
    value[field]
        .as_array()
        .map(Vec::as_slice)
        .unwrap_or_default()
}
fn sum(values: &[Value]) -> u64 {
    values.iter().map(count_value).sum()
}
