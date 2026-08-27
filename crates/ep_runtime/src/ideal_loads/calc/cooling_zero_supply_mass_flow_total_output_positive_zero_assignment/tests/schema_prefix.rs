//! CP429 flat-schema, lossless-prefix, visibility, and parity locks.

use super::*;

#[test]
fn cp429_flat_schema_is_exact_317_117_2_1_with_cp428_first_305_and_unique_tail() {
    let cp428 = public_fields(include_str!(
        "../../cooling_zero_supply_mass_flow_sensible_output_positive_zero_assignment.rs"
    ));
    let cp429 = public_fields(include_str!(
        "../../cooling_zero_supply_mass_flow_total_output_positive_zero_assignment.rs"
    ));
    assert_eq!(cp428.len(), 308);
    assert_eq!(cp429.len(), 317);
    assert_eq!(&cp429[..305], &cp428[..305]);
    assert_eq!(
        &cp429[305..308],
        &[
            "predecessor_cp428_resulting_supply_humidity_ratio",
            "predecessor_cp428_resulting_supply_enthalpy_j_per_kg",
            "predecessor_cp428_resulting_supply_temperature_c",
        ],
    );
    assert_eq!(
        &cp429[308..314],
        &[
            "cooling_zero_supply_mass_flow_total_output_positive_zero_assignment_executed",
            "cp428_retained_supply_humidity_ratio_state_owned",
            "cp428_retained_supply_enthalpy_state_owned",
            "cp428_retained_supply_temperature_state_owned",
            "cooling_total_output_positive_zero_assignment_performed",
            "assigned_cooling_total_output_w",
        ],
    );
    assert_eq!(
        &cp429[314..],
        &[
            "resulting_supply_humidity_ratio",
            "resulting_supply_enthalpy_j_per_kg",
            "resulting_supply_temperature_c",
        ],
    );
    let mut unique = cp429.clone();
    unique.sort_unstable();
    unique.dedup();
    assert_eq!(unique.len(), 317);

    let block = snapshot_block(include_str!(
        "../../cooling_zero_supply_mass_flow_total_output_positive_zero_assignment.rs"
    ));
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
fn cp429_exact_prefix_and_bitwise_matching_cover_all_59_outcomes() {
    for predecessor in cp428_all_snapshots_for_successor_tests() {
        let snapshot = advance(&mut State::new(predecessor.system), predecessor).expect("CP429");
        let reconstructed =
            super::super::cooling_zero_supply_mass_flow_total_output_positive_zero_assignment_predecessor_cp428_snapshot(
                snapshot,
            );
        assert!(
            crate::ideal_loads::cooling_zero_supply_mass_flow_sensible_output_positive_zero_assignment_snapshots_match_bit_exact(
                reconstructed,
                predecessor,
            )
        );
        assert!(
            super::super::cooling_zero_supply_mass_flow_total_output_positive_zero_assignment_snapshots_match_bit_exact(
                snapshot,
                snapshot,
            )
        );
    }
}

#[test]
fn cold_and_validated_paths_match_bit_exact_for_active_and_inactive_routes() {
    let all = cp428_all_snapshots_for_successor_tests();
    let representatives = [
        all.iter()
            .copied()
            .find(|snapshot| route_for(*snapshot).assignment_executed)
            .expect("active"),
        all.iter()
            .copied()
            .find(|snapshot| !route_for(*snapshot).assignment_executed)
            .expect("inactive"),
    ];
    for predecessor in representatives {
        let route = route_for(predecessor);
        let cold = advance(&mut State::new(predecessor.system), predecessor).expect("cold CP429");
        let validated = advance_validated(
            &mut State::new(predecessor.system),
            predecessor,
            route,
        )
        .expect("validated CP429");
        assert!(
            super::super::cooling_zero_supply_mass_flow_total_output_positive_zero_assignment_snapshots_match_bit_exact(
                cold,
                validated,
            )
        );
    }
}

#[test]
fn exact_public_private_partition_is_19_40_and_assignment_is_public_only() {
    let mut public = 0usize;
    let mut private = 0usize;
    let mut public_assignments = 0usize;
    let mut private_assignments = 0usize;
    for predecessor in cp428_all_snapshots_for_successor_tests() {
        let route = route_for(predecessor);
        if is_public_logical_index(route.logical_index) {
            public += 1;
            public_assignments += usize::from(route.assignment_executed);
        } else {
            private += 1;
            private_assignments += usize::from(route.assignment_executed);
        }
    }
    assert_eq!(
        (public, private, public_assignments, private_assignments),
        (19, 40, 1, 0)
    );
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
