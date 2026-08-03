//! Exhaustive CP410-snapshot-to-CP411 advance and reconstruction regression.

use super::{
    advance, all_routes, predecessor_for_route, snapshot_route, test_counts_are_exact, State,
};
use crate::ideal_loads::calc::{
    cooling_post_saturation_capacity_limit_dehumidification_control_default_case_break_snapshot_route as predecessor_snapshot_route,
    cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment_snapshot_is_exact,
};

#[test]
fn all_thirty_six_valid_cp410_snapshots_advance_and_reconstruct_bit_exact() {
    let routes = all_routes();
    let mut state = State::new(predecessor_for_route(routes[0], 1).system);
    let mut public_active = 0;
    let mut private_active = 0;

    for (logical_index, expected_route) in routes.iter().copied().enumerate() {
        let predecessor = predecessor_for_route(expected_route, logical_index + 1);
        let cp410_route = predecessor_snapshot_route(predecessor).expect("exact CP410 predecessor");
        assert_eq!(
            cp410_route.predecessor_index,
            expected_route.predecessor_index
        );
        assert_eq!(
            cp410_route.predecessor_guard_false_fallthrough,
            expected_route.predecessor_guard_false_fallthrough
        );
        assert_eq!(
            cp410_route.predecessor_maximum_capacity_assignment_executed,
            expected_route.predecessor_maximum_capacity_assignment_executed
        );

        let snapshot = advance(&mut state, predecessor).expect("CP411 advance");
        let reconstructed = snapshot_route(snapshot).expect("CP411 reconstruction");
        assert_eq!(reconstructed, expected_route);
        assert!(cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment_snapshot_is_exact(snapshot));
        let active = logical_index >= 18;
        assert_eq!(snapshot.post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment_executed, active);
        assert_eq!(snapshot.purchased_air_supply_humidity_ratio_read, active);
        assert_eq!(
            snapshot.local_supply_humidity_ratio_original_assignment_performed,
            active
        );
        if active {
            let source = predecessor
                .resulting_supply_humidity_ratio
                .expect("active carrier");
            for value in [
                snapshot.purchased_air_supply_humidity_ratio_before_saturation_check,
                snapshot.assigned_supply_humidity_ratio_original,
                snapshot.resulting_supply_humidity_ratio_original,
            ] {
                assert_eq!(value.map(f64::to_bits), Some(source.to_bits()));
            }
            if matches!(expected_route.predecessor_index, 20 | 24) {
                public_active += 1;
            } else {
                private_active += 1;
            }
        } else {
            assert!(snapshot
                .purchased_air_supply_humidity_ratio_before_saturation_check
                .is_none());
            assert!(snapshot.assigned_supply_humidity_ratio_original.is_none());
            assert!(snapshot.resulting_supply_humidity_ratio_original.is_none());
        }
        assert_eq!(
            snapshot.resulting_supply_humidity_ratio.map(f64::to_bits),
            predecessor
                .resulting_supply_humidity_ratio
                .map(f64::to_bits)
        );
        assert_eq!(
            snapshot
                .resulting_supply_enthalpy_j_per_kg
                .map(f64::to_bits),
            predecessor
                .resulting_supply_enthalpy_j_per_kg
                .map(f64::to_bits)
        );
        assert_eq!(
            snapshot.resulting_supply_temperature_c.map(f64::to_bits),
            predecessor.resulting_supply_temperature_c.map(f64::to_bits)
        );
    }

    assert!(test_counts_are_exact(&state));
    assert_eq!(state.transition_count, 36);
    assert_eq!(state.inactive_transition_count, 18);
    assert_eq!(
        state.supply_humidity_ratio_pre_saturation_original_assignment_count,
        18
    );
    assert_eq!(state.source_site_execution_count, 36);
    assert_eq!(state.predecessor_guard_false_fallthrough_count, 6);
    assert_eq!(state.predecessor_maximum_capacity_assignment_count, 6);
    assert_eq!(state.cp410_supply_humidity_ratio_state_owner_count, 18);
    assert_eq!(state.unchanged_supply_humidity_ratio_preservation_count, 18);
    assert_eq!(state.cp410_supply_enthalpy_state_owner_count, 23);
    assert_eq!(state.unchanged_supply_enthalpy_preservation_count, 23);
    assert_eq!(state.cp410_supply_temperature_state_owner_count, 33);
    assert_eq!(state.unchanged_supply_temperature_preservation_count, 33);
    assert_eq!(
        state.cp410_retained_supply_humidity_ratio_owned_read_count,
        18
    );
    assert_eq!(
        state.purchased_air_supply_humidity_ratio_before_saturation_limit_read_count,
        18
    );
    assert_eq!(
        state.local_supply_humidity_ratio_original_assignment_write_count,
        18
    );
    assert_eq!(public_active, 4);
    assert_eq!(private_active, 14);
    for index in 0..30 {
        let expected = if matches!(index, 18..=29) {
            state.predecessor_route_counts[index]
        } else {
            0
        };
        assert_eq!(
            state.supply_humidity_ratio_pre_saturation_original_assignment_route_counts[index],
            expected
        );
    }
}
