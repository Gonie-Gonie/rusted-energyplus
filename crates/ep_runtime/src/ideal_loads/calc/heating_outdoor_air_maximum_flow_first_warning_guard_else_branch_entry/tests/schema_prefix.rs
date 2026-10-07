//! CP442 flat-schema, lossless-prefix, visibility, and cold/validated parity locks.

use super::*;

#[test]
fn cp442_schema_is_exact_430_with_cp441_first_429_and_one_marker() {
    let cp441 = public_fields(include_str!(
        "../../heating_outdoor_air_maximum_flow_continue_warning_timestamp_call.rs"
    ));
    let cp442 = public_fields(include_str!(
        "../../heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry.rs"
    ));
    assert_eq!(cp441.len(), 429);
    assert_eq!(cp442.len(), 430);
    assert_eq!(&cp442[..429], cp441.as_slice());
    assert_eq!(
        cp442[429],
        "heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entered"
    );
    let mut unique = cp442.clone();
    unique.sort_unstable();
    unique.dedup();
    assert_eq!(unique.len(), 430);

    let block = snapshot_block(include_str!(
        "../../heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry.rs"
    ));
    assert_eq!(block.matches("Option<f64>").count(), 146);
    assert_eq!(block.matches("Option<bool>").count(), 9);
    assert_eq!(block.matches("Option<usize>").count(), 2);
    assert_eq!(block.matches("Option<").count() - 146 - 9 - 2, 6);
}

#[test]
fn predecessor_reconstruction_and_cold_validated_paths_are_bit_exact_for_all_67_routes() {
    for predecessor in cp441_all_snapshots_for_successor_tests() {
        let predecessor_route = predecessor_route_for(predecessor);
        let route = route_for(predecessor);
        let cold = advance(&mut State::new(predecessor.system), predecessor).expect("cold CP442");
        let validated = advance_validated(
            &mut State::new(predecessor.system),
            predecessor,
            predecessor_route,
            route,
        )
        .expect("validated CP442");
        let reconstructed = super::super::heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry_predecessor_cp441_snapshot(cold);
        assert!(
            crate::ideal_loads::heating_outdoor_air_maximum_flow_continue_warning_timestamp_call_snapshots_match_bit_exact(
                reconstructed,
                predecessor,
            )
        );
        assert!(
            super::super::heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry_snapshots_match_bit_exact(
                cold,
                validated,
            )
        );
    }
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
    let end = source.find("\n/// Final").expect("snapshot end");
    &source[..end]
}
