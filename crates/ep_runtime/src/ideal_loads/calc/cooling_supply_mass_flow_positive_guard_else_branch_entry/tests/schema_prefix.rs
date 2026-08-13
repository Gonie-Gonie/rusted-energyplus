//! CP424 flat-schema, lossless-prefix, visibility, and cold/validated parity locks.

use super::*;

#[test]
fn cp424_flat_schema_is_cp423_first_262_plus_one_unique_marker() {
    let cp423 = public_fields(include_str!(
        "../../cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment.rs"
    ));
    let cp424 = public_fields(include_str!("../../cooling_supply_mass_flow_positive_guard_else_branch_entry.rs"));
    assert_eq!(cp423.len(), 262);
    assert_eq!(cp424.len(), 263);
    assert_eq!(&cp424[..262], cp423);
    assert_eq!(cp424[262], "cooling_supply_mass_flow_positive_guard_else_branch_entered");
    let mut unique = cp424.clone();
    unique.sort_unstable();
    unique.dedup();
    assert_eq!(unique.len(), 263);

    let snapshot = snapshot_block(include_str!("../../cooling_supply_mass_flow_positive_guard_else_branch_entry.rs"));
    assert_eq!(snapshot.matches("Option<f64>").count(), 94);
    assert_eq!(snapshot.matches("Option<bool>").count(), 2);
    assert_eq!(snapshot.matches("Option<DehumidificationControlType>").count(), 1);
}

#[test]
fn cp424_exact_prefix_reconstruction_and_bitwise_matching_cover_all_59_outcomes() {
    for predecessor in cp423_all_snapshots_for_successor_tests() {
        let snapshot = advance(&mut State::new(predecessor.system), predecessor).expect("CP424");
        let reconstructed = super::super::cooling_supply_mass_flow_positive_guard_else_branch_entry_predecessor_cp423_snapshot(snapshot);
        assert!(crate::ideal_loads::cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment_snapshots_match_bit_exact(
            reconstructed,
            predecessor,
        ));
        assert!(super::super::cooling_supply_mass_flow_positive_guard_else_branch_entry_snapshots_match_bit_exact(snapshot, snapshot));
    }
}

#[test]
fn cold_and_validated_paths_match_bit_exact_for_four_representative_route_classes() {
    let predecessors = cp423_all_snapshots_for_successor_tests();
    let selected = [
        predecessors.iter().copied().find(|snapshot| snapshot.positive_guard_false_fallthrough_skipped),
        predecessors.iter().copied().find(|snapshot| cp423_route(*snapshot).is_some_and(|route| !route.active) && !snapshot.positive_guard_false_fallthrough_skipped),
        predecessors.iter().copied().find(|snapshot| cp423_route(*snapshot).is_some_and(|route| route.active && !route.assignment_executed)),
        predecessors.iter().copied().find(|snapshot| cp423_route(*snapshot).is_some_and(|route| route.assignment_executed)),
    ];
    for predecessor in selected.into_iter().map(|item| item.expect("representative route")) {
        let route = route_for(predecessor);
        let cold = advance(&mut State::new(predecessor.system), predecessor).expect("cold CP424");
        let validated = advance_validated(&mut State::new(predecessor.system), predecessor, route).expect("validated CP424");
        assert!(super::super::cooling_supply_mass_flow_positive_guard_else_branch_entry_snapshots_match_bit_exact(cold, validated));
    }
}

#[test]
fn exact_public_private_partition_is_19_40_and_the_entry_is_public_only() {
    let mut public = 0usize;
    let mut private = 0usize;
    let mut public_entries = 0usize;
    let mut private_entries = 0usize;
    for predecessor in cp423_all_snapshots_for_successor_tests() {
        let route = route_for(predecessor);
        if is_public_logical_index(route.logical_index) {
            public += 1;
            public_entries += usize::from(route.entered);
        } else {
            private += 1;
            private_entries += usize::from(route.entered);
        }
    }
    assert_eq!((public, private, public_entries, private_entries), (19, 40, 1, 0));
}

fn is_public_logical_index(index: usize) -> bool {
    matches!(index, 0..=8 | 20 | 21 | 26 | 27)
}

fn public_fields(source: &'static str) -> Vec<&'static str> {
    let block = snapshot_block(source);
    block
        .lines()
        .filter_map(|line| line.trim().strip_prefix("pub "))
        .filter_map(|line| line.split_once(':').map(|(field, _)| field))
        .collect()
}

fn snapshot_block(source: &'static str) -> &'static str {
    let start = source.find("pub struct PurchasedAirCalc").expect("snapshot start");
    let source = &source[start..];
    let end = source.find("\n}\n\n/// Final").expect("snapshot end");
    &source[..end]
}
