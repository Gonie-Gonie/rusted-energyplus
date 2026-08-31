//! CP434 flat-schema, lossless-prefix, enum, and cold/validated parity locks.

use super::*;

#[test]
fn cp434_schema_is_exact_361_128_4_4_with_cp433_first_348_and_locked_tail() {
    let cp433 = public_fields(include_str!(
        "../../heating_mode_guard_else_branch_entry.rs"
    ));
    let cp434 = public_fields(include_str!(
        "../../heating_operating_mode_deadband_assignment.rs"
    ));
    assert_eq!(cp433.len(), 352);
    assert_eq!(cp434.len(), 361);
    assert_eq!(&cp434[..348], &cp433[..348]);
    assert_eq!(
        &cp434[348..],
        &[
            "predecessor_cp433_resulting_supply_humidity_ratio",
            "predecessor_cp433_resulting_supply_enthalpy_j_per_kg",
            "predecessor_cp433_resulting_supply_temperature_c",
            "heating_mode_guard_else_branch_entered",
            "heating_operating_mode_deadband_assignment_executed",
            "cp433_retained_supply_humidity_ratio_state_owned",
            "cp433_retained_supply_enthalpy_state_owned",
            "cp433_retained_supply_temperature_state_owned",
            "heating_operating_mode_deadband_assignment_performed",
            "assigned_heating_operating_mode_deadband",
            "resulting_supply_humidity_ratio",
            "resulting_supply_enthalpy_j_per_kg",
            "resulting_supply_temperature_c",
        ],
    );
    assert_eq!(
        &cp433[348..],
        &[
            "resulting_supply_humidity_ratio",
            "resulting_supply_enthalpy_j_per_kg",
            "resulting_supply_temperature_c",
            "heating_mode_guard_else_branch_entered",
        ],
    );
    let mut unique = cp434.clone();
    unique.sort_unstable();
    unique.dedup();
    assert_eq!(unique.len(), 361);

    let block = snapshot_block(include_str!(
        "../../heating_operating_mode_deadband_assignment.rs"
    ));
    assert_eq!(block.matches("Option<f64>").count(), 128);
    assert_eq!(block.matches("Option<bool>").count(), 4);
    assert_eq!(block.matches("Option<").count() - 128 - 4, 4);
}

#[test]
fn predecessor_reconstruction_and_cold_validated_paths_are_bit_exact_for_all_61_routes() {
    for predecessor in cp433_all_snapshots_for_successor_tests() {
        let predecessor_route = predecessor_route_for(predecessor);
        let route = route_for(predecessor);
        let cold = advance(&mut State::new(predecessor.system), predecessor).expect("cold CP434");
        let validated = advance_validated(
            &mut State::new(predecessor.system),
            predecessor,
            predecessor_route,
            route,
        )
        .expect("validated CP434");
        let reconstructed =
            super::super::heating_operating_mode_deadband_assignment_predecessor_cp433_snapshot(
                cold,
            );
        assert!(
            crate::ideal_loads::heating_mode_guard_else_branch_entry_snapshots_match_bit_exact(
                reconstructed,
                predecessor,
            )
        );
        assert!(
            super::super::heating_operating_mode_deadband_assignment_snapshots_match_bit_exact(
                cold, validated,
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
