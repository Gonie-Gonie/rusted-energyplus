//! CP430 flat-schema, lossless-prefix, visibility, and parity locks.

use super::*;

#[test]
fn cp430_flat_schema_is_exact_318_117_2_1_with_cp429_prefix_and_marker() {
    let cp429 = public_fields(include_str!(
        "../../cooling_zero_supply_mass_flow_total_output_positive_zero_assignment.rs"
    ));
    let cp430 = public_fields(include_str!("../../heating_or_no_load_case_entry.rs"));
    assert_eq!(cp429.len(), 317);
    assert_eq!(cp430.len(), 318);
    assert_eq!(&cp430[..317], cp429.as_slice());
    assert_eq!(cp430[317], "heating_or_no_load_case_entered");
    let mut unique = cp430.clone();
    unique.sort_unstable();
    unique.dedup();
    assert_eq!(unique.len(), 318);

    let block = snapshot_block(include_str!("../../heating_or_no_load_case_entry.rs"));
    assert_eq!(block.matches("Option<f64>").count(), 117);
    assert_eq!(block.matches("Option<bool>").count(), 2);
    assert_eq!(
        block
            .matches("Option<DehumidificationControlType>")
            .count(),
        1
    );
}

#[test]
fn cp430_exact_prefix_and_bitwise_matching_cover_all_59_outcomes() {
    for predecessor in cp429_all_snapshots_for_successor_tests() {
        let snapshot = advance(&mut State::new(predecessor.system), predecessor).expect("CP430");
        let reconstructed =
            super::super::heating_or_no_load_case_entry_predecessor_cp429_snapshot(snapshot);
        assert!(
            crate::ideal_loads::cooling_zero_supply_mass_flow_total_output_positive_zero_assignment_snapshots_match_bit_exact(
                reconstructed,
                predecessor,
            )
        );
        assert!(super::super::heating_or_no_load_case_entry_snapshots_match_bit_exact(
            snapshot, snapshot,
        ));
    }
}

#[test]
fn cold_and_validated_paths_match_bit_exact_for_entry_and_inactive_routes() {
    let all = cp429_all_snapshots_for_successor_tests();
    let representatives = [
        all.iter()
            .copied()
            .find(|snapshot| route_for(*snapshot).entered)
            .expect("entry"),
        all.iter()
            .copied()
            .find(|snapshot| !route_for(*snapshot).entered)
            .expect("inactive"),
    ];
    for predecessor in representatives {
        let route = route_for(predecessor);
        let cold = advance(&mut State::new(predecessor.system), predecessor).expect("cold CP430");
        let validated = advance_validated(
            &mut State::new(predecessor.system),
            predecessor,
            route,
        )
        .expect("validated CP430");
        assert!(super::super::heating_or_no_load_case_entry_snapshots_match_bit_exact(
            cold, validated,
        ));
    }
}

#[test]
fn exact_public_private_partition_is_19_40_and_entry_is_public_only() {
    let mut public = 0usize;
    let mut private = 0usize;
    let mut public_entries = 0usize;
    let mut private_entries = 0usize;
    for predecessor in cp429_all_snapshots_for_successor_tests() {
        let route = route_for(predecessor);
        if is_public_logical_index(route.logical_index) {
            public += 1;
            public_entries += usize::from(route.entered);
        } else {
            private += 1;
            private_entries += usize::from(route.entered);
        }
    }
    assert_eq!(
        (public, private, public_entries, private_entries),
        (19, 40, 1, 0)
    );
}

fn is_public_logical_index(index: usize) -> bool {
    matches!(index, 0..=8 | 20 | 21 | 26 | 27)
}

fn public_fields(source: &'static str) -> Vec<&'static str> {
    snapshot_block(source)
        .lines()
        .filter_map(|line| line.trim().strip_prefix("pub "))
        .filter_map(|line| line.split_once(':').map(|(field, _)| field))
        .collect()
}

fn snapshot_block(source: &'static str) -> &'static str {
    let start = source
        .find("pub struct PurchasedAirCalc")
        .expect("snapshot start");
    let source = &source[start..];
    let end = source.find("\n}\n\n/// Final").expect("snapshot end");
    &source[..end]
}
