//! CP433 flat-schema, lossless-prefix, visibility, and cold/validated parity locks.

use super::*;

#[test]
fn cp433_schema_is_exact_352_125_4_3_with_cp432_first_351_and_marker() {
    let cp432 = public_fields(include_str!(
        "../../heating_operating_mode_heat_assignment.rs"
    ));
    let cp433 = public_fields(include_str!(
        "../../heating_mode_guard_else_branch_entry.rs"
    ));
    assert_eq!(cp432.len(), 351);
    assert_eq!(cp433.len(), 352);
    assert_eq!(&cp433[..351], cp432.as_slice());
    assert_eq!(cp433[351], "heating_mode_guard_else_branch_entered");
    let mut unique = cp433.clone();
    unique.sort_unstable();
    unique.dedup();
    assert_eq!(unique.len(), 352);

    let block = snapshot_block(include_str!(
        "../../heating_mode_guard_else_branch_entry.rs"
    ));
    assert_eq!(block.matches("Option<f64>").count(), 125);
    assert_eq!(block.matches("Option<bool>").count(), 4);
    assert_eq!(block.matches("Option<").count() - 125 - 4, 3);
}

#[test]
fn predecessor_reconstruction_and_cold_validated_paths_are_bit_exact_for_all_61_routes() {
    for predecessor in cp432_all_snapshots_for_successor_tests() {
        let predecessor_route = predecessor_route_for(predecessor);
        let route = route_for(predecessor);
        let cold = advance(&mut State::new(predecessor.system), predecessor).expect("cold CP433");
        let validated = advance_validated(
            &mut State::new(predecessor.system),
            predecessor,
            predecessor_route,
            route,
        )
        .expect("validated CP433");
        let reconstructed = super::super::heating_mode_guard_else_branch_entry_predecessor_cp432_snapshot(cold);
        assert!(
            crate::ideal_loads::heating_operating_mode_heat_assignment_snapshots_match_bit_exact(
                reconstructed,
                predecessor,
            )
        );
        assert!(
            super::super::heating_mode_guard_else_branch_entry_snapshots_match_bit_exact(
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
    let end = source.find("\n}\n\n/// Final").expect("snapshot end");
    &source[..end]
}
