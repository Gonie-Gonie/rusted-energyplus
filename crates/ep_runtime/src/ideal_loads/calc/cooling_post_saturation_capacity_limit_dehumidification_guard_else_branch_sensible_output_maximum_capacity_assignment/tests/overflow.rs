//! CP422 transactional counter-overflow rejection.

use super::*;

#[test]
fn common_transition_route_and_owner_overflows_are_transactional() {
    let predecessor = cp421_all_snapshots_for_successor_tests()
        .into_iter()
        .find(|snapshot| {
            snapshot.resulting_supply_humidity_ratio.is_some()
                && snapshot.resulting_supply_enthalpy_j_per_kg.is_some()
                && snapshot.resulting_supply_temperature_c.is_some()
        })
        .expect("all owners");
    let route = successor_route_for(predecessor);
    let input = active_input(predecessor);
    let mut states = [
        State::new(predecessor.system),
        State::new(predecessor.system),
        State::new(predecessor.system),
        State::new(predecessor.system),
        State::new(predecessor.system),
        State::new(predecessor.system),
        State::new(predecessor.system),
        State::new(predecessor.system),
    ];
    states[0].transition_count = usize::MAX;
    states[1].predecessor_route_counts[route.logical_index] = usize::MAX;
    states[2].cp421_supply_humidity_ratio_state_owner_count = usize::MAX;
    states[3].unchanged_supply_temperature_preservation_count = usize::MAX;
    states[4].unchanged_supply_humidity_ratio_preservation_count = usize::MAX;
    states[5].cp421_supply_enthalpy_state_owner_count = usize::MAX;
    states[6].unchanged_supply_enthalpy_preservation_count = usize::MAX;
    states[7].cp421_supply_temperature_state_owner_count = usize::MAX;
    for mut state in states {
        let before = state.clone();
        assert!(advance_validated(&mut state, predecessor, route, input).is_none());
        assert_eq!(state, before);
    }
}

#[test]
fn inactive_counter_overflow_is_transactional() {
    let predecessor = cp421_all_snapshots_for_successor_tests()
        .into_iter()
        .find(|snapshot| {
            !snapshot.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_guard_evaluated
        })
        .expect("inactive");
    let route = successor_route_for(predecessor);
    let mut state = State::new(predecessor.system);
    state.inactive_transition_count = usize::MAX;
    let before = state.clone();
    assert!(advance_validated(&mut state, predecessor, route, None).is_none());
    assert_eq!(state, before);
}

#[test]
fn guard_false_counter_overflows_are_transactional() {
    let predecessor = cp421_all_snapshots_for_successor_tests()
        .into_iter()
        .find(|snapshot| {
            snapshot.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_guard_false_fallthrough
        })
        .expect("false fallthrough");
    let route = successor_route_for(predecessor);
    let input = active_input(predecessor);
    let mut states = [
        State::new(predecessor.system),
        State::new(predecessor.system),
    ];
    states[0].predecessor_guard_false_fallthrough_count = usize::MAX;
    states[1].predecessor_guard_false_fallthrough_route_counts[route.logical_index] = usize::MAX;
    for mut state in states {
        let before = state.clone();
        assert!(advance_validated(&mut state, predecessor, route, input).is_none());
        assert_eq!(state, before);
    }
}

#[test]
fn assignment_counter_and_site_overflows_are_transactional() {
    let predecessor = cp421_all_snapshots_for_successor_tests()
        .into_iter()
        .find(|snapshot| {
            snapshot.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_adjustment_body_entered
        })
        .expect("assignment");
    let route = successor_route_for(predecessor);
    let input = active_input(predecessor);
    let mut states = [
        State::new(predecessor.system),
        State::new(predecessor.system),
        State::new(predecessor.system),
        State::new(predecessor.system),
        State::new(predecessor.system),
        State::new(predecessor.system),
    ];
    states[0].cooling_sensible_output_maximum_capacity_assignment_count = usize::MAX;
    states[1].cooling_sensible_output_maximum_capacity_assignment_route_counts
        [route.logical_index] = usize::MAX;
    states[2].source_site_execution_count = usize::MAX;
    states[3].cp421_retained_maximum_total_cooling_capacity_owned_read_count = usize::MAX;
    states[4].maximum_total_cooling_capacity_for_sensible_output_assignment_read_count =
        usize::MAX;
    states[5].cooling_sensible_output_maximum_capacity_assignment_write_count = usize::MAX;
    for mut state in states {
        let before = state.clone();
        assert!(advance_validated(&mut state, predecessor, route, input).is_none());
        assert_eq!(state, before);
    }
}
