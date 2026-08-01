//! CP383 23-route refinement and exact accounting tests.

use super::{active_input, predecessor_for_route};
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_total_output_guard::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputGuardRuntimeState as State,
    advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard_state as advance,
    cooling_post_saturation_capacity_limit_dehumidification_total_output_guard_snapshot_is_exact_direct_release,
};

#[test]
fn cp383_refines_eighteen_predecessors_into_exactly_twenty_three_routes() {
    let system = predecessor_for_route(0, 0, 1).system;
    let mut state = State::new(system);
    let mut snapshots = Vec::new();
    let mut ordinal = 1;

    for inherited in 0..3 {
        let predecessor = predecessor_for_route(inherited, 0, ordinal);
        snapshots.push(advance(&mut state, predecessor, None).expect("complete skip"));
        ordinal += 1;
    }
    for inherited in 3..8 {
        let capacity_skip = predecessor_for_route(inherited, 0, ordinal);
        snapshots.push(advance(&mut state, capacity_skip, None).expect("capacity skip"));
        ordinal += 1;

        let dehumidification_skip = predecessor_for_route(inherited, 2, ordinal);
        snapshots.push(
            advance(&mut state, dehumidification_skip, None).expect("dehumidification skip"),
        );
        ordinal += 1;

        let guard_false = predecessor_for_route(inherited, 1, ordinal);
        snapshots.push(
            advance(&mut state, guard_false, active_input(guard_false, 100.0))
                .expect("strict-greater false"),
        );
        ordinal += 1;

        let body = predecessor_for_route(inherited, 1, ordinal);
        snapshots.push(
            advance(&mut state, body, active_input(body, 99.0)).expect("strict-greater body"),
        );
        ordinal += 1;
    }

    assert_eq!(snapshots.len(), 23);
    assert_eq!(state.transition_count, 23);
    assert_eq!(state.dehumidification_total_output_capacity_guard_evaluation_count, 10);
    assert_eq!(state.dehumidification_total_output_capacity_guard_false_fallthrough_count, 5);
    assert_eq!(state.dehumidification_total_output_capacity_adjustment_body_entry_count, 5);
    assert_eq!(state.source_site_execution_count, 35);
    assert_eq!(state.cp382_cooling_total_output_owned_read_count, 10);
    assert_eq!(state.cp321_maximum_total_cooling_capacity_owned_read_count, 10);
    assert_eq!(state.cp340_same_call_maximum_total_cooling_capacity_bit_corroboration_count, 10);
    assert_eq!(state.cooling_total_output_maximum_total_cooling_capacity_comparison_count, 10);
    assert_eq!(state.cooling_total_output_strictly_greater_than_maximum_total_cooling_capacity_count, 5);
    assert_eq!(
        snapshots
            .into_iter()
            .filter(|snapshot| {
                cooling_post_saturation_capacity_limit_dehumidification_total_output_guard_snapshot_is_exact_direct_release(*snapshot)
            })
            .count(),
        11,
        "eleven public exact and twelve private route shapes",
    );
}

#[test]
fn cp383_skip_routes_execute_no_line_2268_sites() {
    for (route, outcome) in [(0, 0), (3, 0), (3, 2)] {
        let predecessor = predecessor_for_route(route, outcome, 1);
        let mut state = State::new(predecessor.system);
        let snapshot = advance(&mut state, predecessor, None).expect("skip");
        assert!(!snapshot.dehumidification_total_output_capacity_guard_evaluated);
        assert!(!snapshot.cooling_total_output_read);
        assert!(snapshot.cooling_total_output_w.is_none());
        assert!(!snapshot.maximum_total_cooling_capacity_read);
        assert!(snapshot.maximum_total_cooling_capacity_w.is_none());
        assert!(snapshot
            .cooling_total_output_strictly_greater_than_maximum_total_cooling_capacity
            .is_none());
        assert_eq!(state.source_site_execution_count, 0);
    }
}
