//! CP423 schema, reconstruction, IEEE grouping, and bit-parity tests.

use std::collections::BTreeSet;

use super::*;
use crate::ideal_loads::{
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_snapshots_match_bit_exact as cp422_bits,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment_predecessor_cp422_snapshot as reconstruct,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment_snapshot_is_exact as cp423_exact,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment_snapshots_match_bit_exact as cp423_bits,
};

#[test]
fn snapshot_schema_is_exact_262_94_2_1_with_cp422_first_231_and_unique_local25() {
    let cp422_source = include_str!("../../cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment.rs");
    let cp423_source = include_str!("../../cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment.rs");
    let cp422 = snapshot_fields(cp422_source, "SensibleOutputMaximumCapacityAssignmentSnapshot");
    let cp423 = snapshot_fields(cp423_source, "SensibleOutputSupplyTemperatureAssignmentSnapshot");
    assert_eq!(cp422.len(), 234);
    assert_eq!(cp423.len(), 262);
    assert_eq!(&cp423[..231], &cp422[..231]);
    assert_eq!(cp423.iter().collect::<BTreeSet<_>>().len(), 262);
    let block = snapshot_block(cp423_source, "SensibleOutputSupplyTemperatureAssignmentSnapshot");
    assert_eq!(block.matches("Option<f64>").count(), 94);
    assert_eq!(block.matches("Option<bool>").count(), 2);
    assert_eq!(block.matches("Option<DehumidificationControlType>").count(), 1);
    assert_eq!(&cp423[231..], CP423_TAIL_FIELDS);
    assert_eq!(&cp423[234..259], &CP423_TAIL_FIELDS[3..28]);
}

#[test]
fn predecessor_reconstruction_and_cold_validated_paths_are_bit_exact() {
    let predecessors = cp422_all_snapshots_for_successor_tests();
    for predecessor in [
        predecessors.iter().copied().find(|snapshot| snapshot.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_executed).expect("assignment"),
        predecessors.iter().copied().find(|snapshot| !cp422_route(*snapshot).expect("route").active).expect("inactive"),
    ] {
        let route = successor_route_for(predecessor);
        let input = active_input(predecessor);
        let mut cold_state = State::new(predecessor.system);
        let mut validated_state = State::new(predecessor.system);
        let cold = advance(&mut cold_state, predecessor, input).expect("cold");
        let validated = advance_validated(&mut validated_state, predecessor, route, input).expect("bounded");
        assert!(cp423_bits(cold, validated));
        assert_eq!(cold_state, validated_state);
        assert!(cp422_bits(reconstruct(cold), predecessor));
        assert!(cp423_exact(cold));
    }
}

#[test]
fn exact_ieee_ast_and_grouping_are_preserved_for_all_special_classes() {
    let cases = [
        (-0.0, 0.0, 1.0, 1.0),
        (17.0, f64::INFINITY, 2.0, 3.0),
        (17.0, 1.0, 0.0, 3.0),
        (f64::from_bits(0x7ff8_0000_0000_0042), 1.0, 2.0, 3.0),
        (20.0, 10.0, 2.0, 5.0),
    ];
    for (mixed, output, flow, cp_air) in cases {
        let input = ActiveInput {
            mixed_air_temperature_c: mixed,
            cooling_sensible_output_w: output,
            supply_mass_flow_rate_kg_per_s: flow,
            cp_air_j_per_kg_k: cp_air,
        };
        let denominator = flow * cp_air;
        let drop = output / denominator;
        let calculated = mixed - drop;
        let actual = calculate_supply_temperature(input);
        assert_eq!(actual.0.to_bits(), denominator.to_bits());
        assert_eq!(actual.1.to_bits(), drop.to_bits());
        assert_eq!(actual.2.to_bits(), calculated.to_bits());
    }
    let input = ActiveInput {
        mixed_air_temperature_c: 0.0,
        cooling_sensible_output_w: f64::MAX,
        supply_mass_flow_rate_kg_per_s: f64::MAX,
        cp_air_j_per_kg_k: 2.0,
    };
    let (_, grouped_drop, _) = calculate_supply_temperature(input);
    let regrouped_drop = (f64::MAX / f64::MAX) / 2.0;
    assert_eq!(grouped_drop.to_bits(), 0.0_f64.to_bits());
    assert_eq!(regrouped_drop.to_bits(), 0.5_f64.to_bits());
}

#[test]
fn formula_corruption_and_nan_payload_change_fail_exact_validation() {
    let predecessor = assignment_predecessor();
    let route = successor_route_for(predecessor);
    let mut state = State::new(predecessor.system);
    let snapshot = advance_validated(&mut state, predecessor, route, active_input(predecessor)).expect("CP423");
    let mut corrupted = snapshot;
    let value = corrupted.cooling_sensible_output_over_air_capacity_rate_k.expect("drop");
    corrupted.cooling_sensible_output_over_air_capacity_rate_k = Some(f64::from_bits(value.to_bits() ^ 1));
    assert!(!cp423_exact(corrupted));
    assert!(!cp423_bits(snapshot, corrupted));

    let mut left = snapshot;
    let mut right = snapshot;
    left.assigned_sensible_output_supply_temperature_c = Some(f64::from_bits(0x7ff8_0000_0000_0042));
    right.assigned_sensible_output_supply_temperature_c = Some(f64::from_bits(0x7ff8_0000_0000_0043));
    assert!(!cp423_bits(left, right));
}

fn snapshot_block<'a>(source: &'a str, suffix: &str) -> &'a str {
    let marker = format!("pub struct PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranch{suffix}");
    let start = source.find(&marker).expect("snapshot start");
    let body = &source[start..];
    let end = body.find("\n}\n").expect("snapshot end") + 3;
    &body[..end]
}

fn snapshot_fields<'a>(source: &'a str, suffix: &str) -> Vec<&'a str> {
    snapshot_block(source, suffix).lines().filter_map(|line| {
        line.trim_start().strip_prefix("pub ").and_then(|field| field.split_once(':').map(|(name, _)| name))
    }).collect()
}

const CP423_TAIL_FIELDS: &[&str] = &[
    "predecessor_cp422_resulting_supply_humidity_ratio",
    "predecessor_cp422_resulting_supply_enthalpy_j_per_kg",
    "predecessor_cp422_resulting_supply_temperature_c",
    "post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment_executed",
    "cp422_retained_supply_humidity_ratio_state_owned",
    "cp422_retained_supply_enthalpy_state_owned",
    "cp422_retained_supply_temperature_state_owned",
    "cp329_retained_mixed_air_temperature_for_sensible_output_supply_temperature_owned_read",
    "mixed_air_temperature_for_sensible_output_supply_temperature_read",
    "mixed_air_temperature_for_sensible_output_supply_temperature_c",
    "cp422_retained_cooling_sensible_output_owned_read",
    "cooling_sensible_output_for_supply_temperature_read",
    "cooling_sensible_output_for_supply_temperature_w",
    "cp330_retained_supply_mass_flow_rate_for_sensible_output_supply_temperature_owned_read",
    "cp329_supply_mass_flow_rate_for_sensible_output_supply_temperature_bit_corroborated",
    "supply_mass_flow_rate_for_sensible_output_supply_temperature_read",
    "supply_mass_flow_rate_for_sensible_output_supply_temperature_kg_per_s",
    "cp419_retained_cp_air_for_sensible_output_supply_temperature_owned_read",
    "cp_air_for_sensible_output_supply_temperature_read",
    "cp_air_for_sensible_output_supply_temperature_j_per_kg_k",
    "supply_mass_flow_rate_times_cp_air_for_sensible_output_supply_temperature_calculated",
    "supply_mass_flow_rate_times_cp_air_for_sensible_output_supply_temperature_w_per_k",
    "cooling_sensible_output_over_air_capacity_rate_calculated",
    "cooling_sensible_output_over_air_capacity_rate_k",
    "sensible_output_supply_temperature_calculated",
    "calculated_sensible_output_supply_temperature_c",
    "sensible_output_supply_temperature_assignment_performed",
    "assigned_sensible_output_supply_temperature_c",
    "resulting_supply_humidity_ratio",
    "resulting_supply_enthalpy_j_per_kg",
    "resulting_supply_temperature_c",
];
