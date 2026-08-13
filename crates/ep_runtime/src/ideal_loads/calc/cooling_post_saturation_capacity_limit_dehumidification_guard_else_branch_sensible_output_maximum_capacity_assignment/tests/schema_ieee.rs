//! CP422 schema, reconstruction, bit-parity, and IEEE assignment tests.

use std::collections::BTreeSet;

use super::*;
use crate::ideal_loads::{
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_snapshots_match_bit_exact as cp421_bits,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_predecessor_cp421_snapshot as reconstruct,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_snapshots_match_bit_exact as cp422_bits,
};

#[test]
fn snapshot_schema_is_exact_234_83_2_1_with_cp421_first_217_and_unique_tail() {
    let cp421_source = include_str!(
        "../../cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard.rs"
    );
    let cp422_source = include_str!(
        "../../cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment.rs"
    );
    let cp421 = snapshot_fields(cp421_source, "SensibleOutputGuardSnapshot");
    let cp422 = snapshot_fields(
        cp422_source,
        "SensibleOutputMaximumCapacityAssignmentSnapshot",
    );
    assert_eq!(cp421.len(), 220);
    assert_eq!(cp422.len(), 234);
    assert_eq!(&cp422[..217], &cp421[..217]);
    assert_eq!(cp422.iter().collect::<BTreeSet<_>>().len(), 234);
    let block = snapshot_block(
        cp422_source,
        "SensibleOutputMaximumCapacityAssignmentSnapshot",
    );
    assert_eq!(block.matches("Option<f64>").count(), 83);
    assert_eq!(block.matches("Option<bool>").count(), 2);
    assert_eq!(block.matches("Option<DehumidificationControlType>").count(), 1);
    assert_eq!(&cp422[217..], CP422_TAIL_FIELDS);
}

#[test]
fn predecessor_reconstruction_and_cold_validated_paths_are_bit_exact() {
    let predecessors = cp421_all_snapshots_for_successor_tests();
    for predecessor in [
        predecessors
            .iter()
            .copied()
            .find(|snapshot| {
                snapshot.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_adjustment_body_entered
            })
            .expect("body"),
        predecessors
            .iter()
            .copied()
            .find(|snapshot| {
                !snapshot.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_guard_evaluated
            })
            .expect("inactive"),
    ] {
        let route = successor_route_for(predecessor);
        let input = active_input(predecessor);
        let mut cold_state = State::new(predecessor.system);
        let mut validated_state = State::new(predecessor.system);
        let cold = advance(&mut cold_state, predecessor, input).expect("cold");
        let validated =
            advance_validated(&mut validated_state, predecessor, route, input).expect("bounded");
        assert!(cp422_bits(cold, validated));
        assert_eq!(cold_state, validated_state);
        assert!(cp421_bits(reconstruct(cold), predecessor));
    }
}

#[test]
fn source_assignment_copies_all_non_nan_ieee_classes_bit_exact() {
    let template = cp421_all_snapshots_for_successor_tests()
        .into_iter()
        .find(|snapshot| {
            snapshot.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_adjustment_body_entered
        })
        .expect("body");
    let route = successor_route_for(template);
    for bits in [
        (-0.0f64).to_bits(),
        1u64,
        f64::INFINITY.to_bits(),
        f64::NEG_INFINITY.to_bits(),
    ] {
        let mut predecessor = template;
        let value = f64::from_bits(bits);
        predecessor.cp420_cooling_sensible_output_for_capacity_guard_w = Some(value);
        predecessor.maximum_total_cooling_capacity_w = Some(value);
        predecessor.cooling_sensible_output_greater_than_or_equal_to_maximum_total_cooling_capacity = Some(true);
        let mut state = State::new(predecessor.system);
        let snapshot = advance_validated(
            &mut state,
            predecessor,
            route,
            Some(ActiveInput {
                preexisting_cooling_sensible_output_w: value,
                maximum_total_cooling_capacity_w: value,
                cp421_retained_maximum_total_cooling_capacity_owned_read: true,
            }),
        )
        .expect("IEEE assignment");
        assert_eq!(
            snapshot
                .assigned_cooling_sensible_output_from_maximum_capacity_w
                .expect("assigned")
                .to_bits(),
            bits,
        );
        assert_eq!(
            snapshot
                .resulting_cooling_sensible_output_after_maximum_capacity_assignment_w
                .expect("result")
                .to_bits(),
            bits,
        );
    }
}

#[test]
fn nan_guard_false_preserves_preexisting_payload_and_bit_comparator_detects_change() {
    let mut predecessor = cp421_all_snapshots_for_successor_tests()
        .into_iter()
        .find(|snapshot| {
            snapshot.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_guard_false_fallthrough
        })
        .expect("false route");
    let route = successor_route_for(predecessor);
    let nan_bits = 0x7ff8_0000_0000_0042;
    predecessor.cp420_cooling_sensible_output_for_capacity_guard_w =
        Some(f64::from_bits(nan_bits));
    predecessor.cooling_sensible_output_greater_than_or_equal_to_maximum_total_cooling_capacity =
        Some(false);
    let mut state = State::new(predecessor.system);
    let snapshot = advance_validated(
        &mut state,
        predecessor,
        route,
        Some(ActiveInput {
            preexisting_cooling_sensible_output_w: f64::from_bits(nan_bits),
            maximum_total_cooling_capacity_w: predecessor
                .maximum_total_cooling_capacity_w
                .expect("capacity"),
            cp421_retained_maximum_total_cooling_capacity_owned_read: true,
        }),
    )
    .expect("NaN false fallthrough");
    assert_eq!(
        snapshot
            .resulting_cooling_sensible_output_after_maximum_capacity_assignment_w
            .expect("preserved")
            .to_bits(),
        nan_bits,
    );
    let mut changed = snapshot;
    changed.resulting_cooling_sensible_output_after_maximum_capacity_assignment_w =
        Some(f64::from_bits(nan_bits ^ 1));
    assert!(!cp422_bits(snapshot, changed));
}

fn snapshot_block<'a>(source: &'a str, suffix: &str) -> &'a str {
    let marker = format!(
        "pub struct PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranch{suffix}"
    );
    let start = source.find(&marker).expect("snapshot start");
    let body = &source[start..];
    let end = body.find("\n}\n").expect("snapshot end") + 3;
    &body[..end]
}

fn snapshot_fields<'a>(source: &'a str, suffix: &str) -> Vec<&'a str> {
    snapshot_block(source, suffix)
        .lines()
        .filter_map(|line| {
            line.trim_start()
                .strip_prefix("pub ")
                .and_then(|field| field.split_once(':').map(|(name, _)| name))
        })
        .collect()
}

const CP422_TAIL_FIELDS: &[&str] = &[
    "predecessor_cp421_resulting_supply_humidity_ratio",
    "predecessor_cp421_resulting_supply_enthalpy_j_per_kg",
    "predecessor_cp421_resulting_supply_temperature_c",
    "post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_executed",
    "cp421_retained_supply_humidity_ratio_state_owned",
    "cp421_retained_supply_enthalpy_state_owned",
    "cp421_retained_supply_temperature_state_owned",
    "preexisting_cooling_sensible_output_for_maximum_capacity_assignment_w",
    "cp421_retained_maximum_total_cooling_capacity_owned_read",
    "maximum_total_cooling_capacity_for_sensible_output_assignment_read",
    "maximum_total_cooling_capacity_for_sensible_output_assignment_w",
    "cooling_sensible_output_maximum_capacity_assignment_performed",
    "assigned_cooling_sensible_output_from_maximum_capacity_w",
    "resulting_cooling_sensible_output_after_maximum_capacity_assignment_w",
    "resulting_supply_humidity_ratio",
    "resulting_supply_enthalpy_j_per_kg",
    "resulting_supply_temperature_c",
];
