//! CP384 23-route refinement and exact accounting tests.

use super::predecessor_for_route;
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputMaximumCapacityAssignmentRuntimeState as State,
    advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment_state as advance,
    cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment_snapshot_is_exact_direct_release,
};

#[test]
fn cp384_maps_twenty_three_predecessor_routes_one_to_one() {
    let system = predecessor_for_route(0, 0, false, 1).system;
    let mut state = State::new(system);
    let mut snapshots = Vec::new();
    let mut ordinal = 1;

    for inherited in 0..3 {
        let predecessor = predecessor_for_route(inherited, 0, false, ordinal);
        snapshots.push(advance(&mut state, predecessor).expect("complete skip"));
        ordinal += 1;
    }
    for inherited in 3..8 {
        let capacity_skip = predecessor_for_route(inherited, 0, false, ordinal);
        snapshots.push(advance(&mut state, capacity_skip).expect("capacity skip"));
        ordinal += 1;

        let dehumidification_skip = predecessor_for_route(inherited, 2, false, ordinal);
        snapshots.push(
            advance(&mut state, dehumidification_skip).expect("dehumidification skip"),
        );
        ordinal += 1;

        let guard_false = predecessor_for_route(inherited, 1, false, ordinal);
        snapshots.push(advance(&mut state, guard_false).expect("guard false"));
        ordinal += 1;

        let body = predecessor_for_route(inherited, 1, true, ordinal);
        snapshots.push(advance(&mut state, body).expect("maximum assignment"));
        ordinal += 1;
    }

    assert_eq!(snapshots.len(), 23);
    assert_eq!(state.transition_count, 23);
    assert_eq!(
        state.dehumidification_total_output_capacity_guard_evaluation_count,
        10
    );
    assert_eq!(
        state.dehumidification_total_output_capacity_guard_false_fallthrough_count,
        5
    );
    assert_eq!(
        state.dehumidification_total_output_maximum_capacity_assignment_count,
        5
    );
    assert_eq!(state.source_site_execution_count, 10);
    assert_eq!(
        state.cp383_retained_maximum_total_cooling_capacity_owned_read_count,
        5
    );
    assert_eq!(state.maximum_total_cooling_capacity_read_count, 5);
    assert_eq!(state.cooling_total_output_assignment_write_count, 5);
    assert_eq!(
        snapshots
            .iter()
            .filter(|snapshot| {
                cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment_snapshot_is_exact_direct_release(
                    **snapshot,
                )
            })
            .count(),
        11,
        "eleven public exact and twelve private route shapes",
    );

    for snapshot in snapshots {
        if snapshot.dehumidification_total_output_capacity_guard_false_fallthrough {
            assert_eq!(
                snapshot.preexisting_cooling_total_output_w.map(f64::to_bits),
                snapshot.resulting_cooling_total_output_w.map(f64::to_bits),
            );
        }
        if snapshot.dehumidification_total_output_maximum_capacity_assignment_executed {
            let expected = snapshot.maximum_total_cooling_capacity_w.map(f64::to_bits);
            assert_eq!(snapshot.assigned_cooling_total_output_w.map(f64::to_bits), expected);
            assert_eq!(snapshot.resulting_cooling_total_output_w.map(f64::to_bits), expected);
        }
    }
}

#[test]
fn cp384_outer_skips_execute_no_line_2269_sites_or_line_2270_feed() {
    for (route, outcome) in [(0, 0), (3, 0), (3, 2)] {
        let predecessor = predecessor_for_route(route, outcome, false, 1);
        let mut state = State::new(predecessor.system);
        let snapshot = advance(&mut state, predecessor).expect("skip");
        assert!(snapshot.preexisting_cooling_total_output_w.is_none());
        assert!(!snapshot.maximum_total_cooling_capacity_read);
        assert!(snapshot.maximum_total_cooling_capacity_w.is_none());
        assert!(!snapshot.cooling_total_output_assigned);
        assert!(snapshot.assigned_cooling_total_output_w.is_none());
        assert!(snapshot.resulting_cooling_total_output_w.is_none());
        assert_eq!(state.source_site_execution_count, 0);
    }
}
