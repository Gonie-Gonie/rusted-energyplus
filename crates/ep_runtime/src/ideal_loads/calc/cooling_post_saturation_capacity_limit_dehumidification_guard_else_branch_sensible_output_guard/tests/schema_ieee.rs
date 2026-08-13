use std::collections::BTreeSet;

use super::*;
use crate::ideal_loads::calc::advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_state_with_validated_route as advance_validated;
use crate::ideal_loads::{
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_snapshots_match_bit_exact as cp420_bits,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_predecessor_cp420_snapshot as reconstruct,
};

#[test]
fn snapshot_schema_is_exact_220_76_2_1_with_cp420_first_199_and_unique_tail() {
    let cp420_source = include_str!(
        "../../cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment.rs"
    );
    let cp421_source = include_str!(
        "../../cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard.rs"
    );
    let cp420 = snapshot_fields(cp420_source, "SensibleOutputAssignmentSnapshot");
    let cp421 = snapshot_fields(cp421_source, "SensibleOutputGuardSnapshot");
    assert_eq!(cp420.len(), 202);
    assert_eq!(cp421.len(), 220);
    assert_eq!(&cp421[..199], &cp420[..199]);
    assert_eq!(cp421.iter().collect::<BTreeSet<_>>().len(), 220);
    let block = snapshot_block(cp421_source, "SensibleOutputGuardSnapshot");
    assert_eq!(block.matches("Option<f64>").count(), 76);
    assert_eq!(block.matches("Option<bool>").count(), 2);
    assert_eq!(block.matches("Option<DehumidificationControlType>").count(), 1);
    assert_eq!(&cp421[199..], CP421_TAIL_FIELDS);
}

#[test]
fn predecessor_reconstruction_and_cold_validated_paths_are_bit_exact() {
    let predecessors = cp420_predecessors();
    for predecessor in [
        predecessors
            .iter()
            .copied()
            .find(|snapshot| snapshot.cooling_sensible_output_w.is_some())
            .expect("active"),
        predecessors
            .iter()
            .copied()
            .find(|snapshot| snapshot.cooling_sensible_output_w.is_none())
            .expect("inactive"),
    ] {
        let route = successor_route_for(predecessor);
        let input = active_input(predecessor, false);
        let mut cold_state = State::new(predecessor.system);
        let mut validated_state = State::new(predecessor.system);
        let cold = advance(&mut cold_state, predecessor, input).expect("cold");
        let validated =
            advance_validated(&mut validated_state, predecessor, route, input).expect("bounded");
        assert!(super::super::cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_snapshots_match_bit_exact(
            cold,
            validated,
        ));
        assert_eq!(cold_state, validated_state);
        assert!(cp420_bits(reconstruct(cold), predecessor));
    }
}

#[test]
fn validated_route_and_owner_marker_forgeries_are_transactional() {
    let predecessor = cp420_predecessors()
        .into_iter()
        .find(|snapshot| snapshot.cooling_sensible_output_w.is_some())
        .expect("active");
    let route = successor_route_for(predecessor);
    let input = active_input(predecessor, false).expect("input");

    let mut routes = [route, route, route];
    routes[0].logical_index = (route.logical_index + 1) % 36;
    routes[1].active = false;
    routes[2].body_entered = true;
    for forged in routes {
        let mut state = State::new(predecessor.system);
        let before = state.clone();
        assert!(advance_validated(&mut state, predecessor, forged, Some(input)).is_none());
        assert_eq!(state, before);
    }

    for mutate in [
        |input: &mut ActiveInput| input.cp420_cooling_sensible_output_owned_read = false,
        |input: &mut ActiveInput| {
            input.cp321_maximum_total_cooling_capacity_owned_read = false
        },
        |input: &mut ActiveInput| {
            input.cp340_same_call_maximum_total_cooling_capacity_bit_corroborated = false
        },
        |input: &mut ActiveInput| {
            input.cooling_sensible_output_w =
                f64::from_bits(input.cooling_sensible_output_w.to_bits() ^ 1)
        },
    ] {
        let mut forged = input;
        mutate(&mut forged);
        let mut state = State::new(predecessor.system);
        let before = state.clone();
        assert!(advance_validated(&mut state, predecessor, route, Some(forged)).is_none());
        assert_eq!(state, before);
    }
}

#[test]
fn route_derived_owner_and_preservation_pairs_reject_equal_counter_forgeries() {
    let predecessors = cp420_predecessors();
    type CarrierForgery = (fn(Predecessor) -> bool, fn(&mut State));
    let carriers: [CarrierForgery; 3] = [
        (
            |predecessor| predecessor.resulting_supply_humidity_ratio.is_some(),
            |state| {
                state.cp420_supply_humidity_ratio_state_owner_count += 1;
                state.unchanged_supply_humidity_ratio_preservation_count += 1;
            },
        ),
        (
            |predecessor| predecessor.resulting_supply_enthalpy_j_per_kg.is_some(),
            |state| {
                state.cp420_supply_enthalpy_state_owner_count += 1;
                state.unchanged_supply_enthalpy_preservation_count += 1;
            },
        ),
        (
            |predecessor| predecessor.resulting_supply_temperature_c.is_some(),
            |state| {
                state.cp420_supply_temperature_state_owner_count += 1;
                state.unchanged_supply_temperature_preservation_count += 1;
            },
        ),
    ];
    for (present, forge) in carriers {
        let predecessor = predecessors
            .iter()
            .copied()
            .find(|predecessor| present(*predecessor))
            .expect("carrier");
        let route = successor_route_for(predecessor);
        let input = active_input(predecessor, false);
        let mut state = State::new(predecessor.system);
        advance_validated(&mut state, predecessor, route, input).expect("transition");
        assert!(super::super::release::state_counts_are_consistent_for_test(&state));
        forge(&mut state);
        assert!(!super::super::release::state_counts_are_consistent_for_test(&state));
    }
}

#[test]
fn raw_ieee_greater_equal_truth_table_and_payload_bits_are_preserved() {
    let baseline = cp420_predecessors()
        .into_iter()
        .find(|snapshot| snapshot.cooling_sensible_output_w.is_some())
        .expect("active");
    let output = baseline.cooling_sensible_output_w.expect("output");
    for (capacity, expected) in [
        (output, true),
        (f64::from_bits(0x7ff8_0000_0000_0421), false),
        (f64::INFINITY, output == f64::INFINITY),
        (f64::NEG_INFINITY, true),
        (-1.0, output >= -1.0),
        (0.0, output >= 0.0),
    ] {
        let snapshot = run_guard(baseline, capacity);
        assert_eq!(
            snapshot.cooling_sensible_output_greater_than_or_equal_to_maximum_total_cooling_capacity,
            Some(expected),
        );
        assert_eq!(
            snapshot.maximum_total_cooling_capacity_w.expect("capacity").to_bits(),
            capacity.to_bits(),
        );
        assert!(super::super::cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_snapshot_is_exact(snapshot));
    }

    let nan_payload = f64::from_bits(0x7ff8_0000_0000_1421);
    let nan_left = cp420_with_flow_and_delta(nan_payload, 1.0);
    let snapshot = run_guard(nan_left, 1.0);
    assert_eq!(
        snapshot.cooling_sensible_output_greater_than_or_equal_to_maximum_total_cooling_capacity,
        Some(false),
    );
    assert_eq!(
        snapshot
            .cp420_cooling_sensible_output_for_capacity_guard_w
            .expect("NaN")
            .to_bits(),
        nan_left.cooling_sensible_output_w.expect("owner").to_bits(),
    );

    for (flow, delta, expected) in [
        (f64::INFINITY, 1.0, true),
        (f64::INFINITY, -1.0, false),
    ] {
        let predecessor = cp420_with_flow_and_delta(flow, delta);
        let result = run_guard(predecessor, 1.0);
        assert_eq!(
            result.cooling_sensible_output_greater_than_or_equal_to_maximum_total_cooling_capacity,
            Some(expected),
        );
    }

    for flow in [0.0, -0.0] {
        let predecessor = cp420_with_flow_and_delta(flow, 1.0);
        let left = predecessor.cooling_sensible_output_w.expect("signed zero");
        let result = run_guard(predecessor, -left);
        assert_eq!(
            result.cooling_sensible_output_greater_than_or_equal_to_maximum_total_cooling_capacity,
            Some(true),
        );
    }
}

fn run_guard(predecessor: Predecessor, capacity: f64) -> super::super::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardSnapshot {
    let output = predecessor.cooling_sensible_output_w.expect("active output");
    let input = ActiveInput {
        cooling_sensible_output_w: output,
        maximum_total_cooling_capacity_w: capacity,
        cp420_cooling_sensible_output_owned_read: true,
        cp321_maximum_total_cooling_capacity_owned_read: true,
        cp340_same_call_maximum_total_cooling_capacity_bit_corroborated: true,
    };
    advance(&mut State::new(predecessor.system), predecessor, Some(input)).expect("raw CP421")
}

fn cp420_with_flow_and_delta(flow: f64, delta: f64) -> Predecessor {
    let cp419 = cp419_all_snapshots_for_successor_tests()
        .into_iter()
        .find(|snapshot| {
            snapshot.post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_executed
        })
        .expect("active CP419");
    let supply = cp419.resulting_supply_temperature_c.expect("supply");
    advance_cp420(
        &mut Cp420State::new(cp419.system),
        cp419,
        Some(Cp420Input {
            supply_mass_flow_rate_kg_per_s: flow,
            mixed_air_temperature_c: supply + delta,
        }),
    )
    .expect("CP420 edge")
}

fn snapshot_fields<'a>(source: &'a str, suffix: &str) -> Vec<&'a str> {
    snapshot_block(source, suffix)
        .lines()
        .filter_map(|line| line.trim().strip_prefix("pub "))
        .filter_map(|line| line.split_once(':').map(|(name, _)| name))
        .collect()
}

fn snapshot_block<'a>(source: &'a str, suffix: &str) -> &'a str {
    source
        .split_once(suffix)
        .expect("snapshot declaration")
        .1
        .split_once("/// Final selected-unit")
        .expect("snapshot terminator")
        .0
}

const CP421_TAIL_FIELDS: &[&str] = &[
    "predecessor_cp420_resulting_supply_humidity_ratio",
    "predecessor_cp420_resulting_supply_enthalpy_j_per_kg",
    "predecessor_cp420_resulting_supply_temperature_c",
    "post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_guard_evaluated",
    "cp420_retained_cooling_sensible_output_owned_read",
    "cooling_sensible_output_read",
    "cp420_cooling_sensible_output_for_capacity_guard_w",
    "cp321_maximum_total_cooling_capacity_owned_read",
    "cp340_same_call_maximum_total_cooling_capacity_bit_corroborated",
    "maximum_total_cooling_capacity_read",
    "maximum_total_cooling_capacity_w",
    "cooling_sensible_output_maximum_total_cooling_capacity_comparison_evaluated",
    "cooling_sensible_output_greater_than_or_equal_to_maximum_total_cooling_capacity",
    "post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_adjustment_body_entered",
    "post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_guard_false_fallthrough",
    "cp420_retained_supply_humidity_ratio_state_owned",
    "cp420_retained_supply_enthalpy_state_owned",
    "cp420_retained_supply_temperature_state_owned",
    "resulting_supply_humidity_ratio",
    "resulting_supply_enthalpy_j_per_kg",
    "resulting_supply_temperature_c",
];
