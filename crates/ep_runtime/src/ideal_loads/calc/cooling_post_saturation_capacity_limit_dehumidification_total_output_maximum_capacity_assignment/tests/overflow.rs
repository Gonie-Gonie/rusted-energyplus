//! CP384 checked-overflow transactional rejection.

use super::predecessor_for_route;
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputMaximumCapacityAssignmentRuntimeState as State,
    advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment_state as advance,
};

#[test]
fn cp384_rejects_every_assignment_counter_overflow_without_mutation() {
    let predecessor = predecessor_for_route(3, 1, true, 1);
    for counter in 0..10 {
        let mut state = State::new(predecessor.system);
        match counter {
            0 => state.transition_count = usize::MAX,
            1 => state.heating_availability_guard_false_fallthrough_count = usize::MAX,
            2 => state.heating_availability_guard_false_fallthrough_dehumidification_total_output_assignment_count = usize::MAX,
            3 => state.heating_availability_guard_false_fallthrough_dehumidification_total_output_maximum_capacity_assignment_count = usize::MAX,
            4 => state.dehumidification_total_output_capacity_guard_evaluation_count = usize::MAX,
            5 => state.dehumidification_total_output_maximum_capacity_assignment_count = usize::MAX,
            6 => state.source_site_execution_count = usize::MAX,
            7 => state.cp383_retained_maximum_total_cooling_capacity_owned_read_count = usize::MAX,
            8 => state.maximum_total_cooling_capacity_read_count = usize::MAX,
            9 => state.cooling_total_output_assignment_write_count = usize::MAX,
            _ => unreachable!(),
        }
        let before = state.clone();
        assert!(
            advance(&mut state, predecessor).is_none(),
            "counter {counter} must fail closed",
        );
        assert_eq!(state, before);
    }
}

#[test]
fn cp384_rejects_guard_false_counter_overflow_without_mutation() {
    let predecessor = predecessor_for_route(3, 1, false, 1);
    for counter in 0..2 {
        let mut state = State::new(predecessor.system);
        if counter == 0 {
            state.heating_availability_guard_false_fallthrough_dehumidification_total_output_capacity_guard_false_fallthrough_count = usize::MAX;
        } else {
            state.dehumidification_total_output_capacity_guard_false_fallthrough_count = usize::MAX;
        }
        let before = state.clone();
        assert!(advance(&mut state, predecessor).is_none());
        assert_eq!(state, before);
    }
}

#[test]
fn cp384_rejects_skip_route_overflow_without_mutation() {
    let predecessor = predecessor_for_route(0, 0, false, 1);
    let mut state = State::new(predecessor.system);
    state.unit_off_skip_count = usize::MAX;
    let before = state.clone();
    assert!(advance(&mut state, predecessor).is_none());
    assert_eq!(state, before);
}
