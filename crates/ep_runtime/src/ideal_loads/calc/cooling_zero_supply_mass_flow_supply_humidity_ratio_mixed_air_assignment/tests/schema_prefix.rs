//! CP426 flat-schema, lossless-prefix, visibility, and parity locks.

use super::*;

#[test]
fn cp426_flat_schema_is_exact_287_104_2_1_with_cp425_first_272_and_unique_tail() {
    let cp425 = public_fields(include_str!(
        "../../cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment.rs"
    ));
    let cp426 = public_fields(include_str!(
        "../../cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment.rs"
    ));
    assert_eq!(cp425.len(), 275);
    assert_eq!(cp426.len(), 287);
    assert_eq!(&cp426[..272], &cp425[..272]);
    assert_eq!(
        &cp426[272..275],
        &[
            "predecessor_cp425_resulting_supply_humidity_ratio",
            "predecessor_cp425_resulting_supply_enthalpy_j_per_kg",
            "predecessor_cp425_resulting_supply_temperature_c",
        ],
    );
    assert_eq!(
        &cp426[275..284],
        &[
            "cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment_executed",
            "cp425_retained_supply_humidity_ratio_state_owned",
            "cp425_retained_supply_enthalpy_state_owned",
            "cp425_retained_supply_temperature_state_owned",
            "cp329_retained_mixed_air_humidity_ratio_for_zero_supply_mass_flow_supply_humidity_ratio_assignment_owned_read",
            "mixed_air_humidity_ratio_for_zero_supply_mass_flow_supply_humidity_ratio_assignment_read",
            "mixed_air_humidity_ratio_for_zero_supply_mass_flow_supply_humidity_ratio_assignment",
            "zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment_performed",
            "assigned_supply_humidity_ratio_from_mixed_air",
        ],
    );
    assert_eq!(
        &cp426[284..],
        &[
            "resulting_supply_humidity_ratio",
            "resulting_supply_enthalpy_j_per_kg",
            "resulting_supply_temperature_c",
        ],
    );
    let mut unique = cp426.clone();
    unique.sort_unstable();
    unique.dedup();
    assert_eq!(unique.len(), 287);

    let block = snapshot_block(include_str!(
        "../../cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment.rs"
    ));
    assert_eq!(block.matches("Option<f64>").count(), 104);
    assert_eq!(block.matches("Option<bool>").count(), 2);
    assert_eq!(
        block
            .matches("Option<DehumidificationControlType>")
            .count(),
        1
    );
}

#[test]
fn cp426_exact_prefix_and_bitwise_matching_cover_all_59_outcomes() {
    for predecessor in cp425_all_snapshots_for_successor_tests() {
        let route = route_for(predecessor);
        let rhs = route.assignment_executed.then_some(f64::from_bits(1));
        let snapshot =
            advance(&mut State::new(predecessor.system), predecessor, rhs).expect("CP426");
        let reconstructed =
            super::super::cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment_predecessor_cp425_snapshot(
                snapshot,
            );
        assert!(
            crate::ideal_loads::cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_snapshots_match_bit_exact(
                reconstructed,
                predecessor,
            )
        );
        assert!(
            super::super::cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment_snapshots_match_bit_exact(
                snapshot,
                snapshot,
            )
        );
    }
}

#[test]
fn cold_and_validated_paths_match_bit_exact_for_active_and_inactive_routes() {
    let all = cp425_all_snapshots_for_successor_tests();
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
        let rhs = route.assignment_executed.then_some(f64::from_bits(1));
        let cold =
            advance(&mut State::new(predecessor.system), predecessor, rhs).expect("cold CP426");
        let validated = advance_validated(
            &mut State::new(predecessor.system),
            predecessor,
            route,
            rhs,
        )
        .expect("validated CP426");
        assert!(
            super::super::cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment_snapshots_match_bit_exact(
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
    for predecessor in cp425_all_snapshots_for_successor_tests() {
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
