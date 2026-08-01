//! CP383 checked-overflow transactional rejection.

use super::{active_input, predecessor_for_route};
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_total_output_guard::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputGuardRuntimeState as State,
    advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard_state as advance,
};

#[test]
fn cp383_rejects_every_active_counter_overflow_without_mutation() {
    let predecessor = predecessor_for_route(3, 1, 1);
    for counter in 0..14 {
        let mut state = State::new(predecessor.system);
        match counter {
            0 => state.transition_count = usize::MAX,
            1 => state.heating_availability_guard_false_fallthrough_count = usize::MAX,
            2 => state.heating_availability_guard_false_fallthrough_dehumidification_total_output_assignment_count = usize::MAX,
            3 => state.heating_availability_guard_false_fallthrough_dehumidification_total_output_capacity_adjustment_body_entry_count = usize::MAX,
            4 => state.source_site_execution_count = usize::MAX,
            5 => state.dehumidification_total_output_capacity_guard_evaluation_count = usize::MAX,
            6 => state.cp382_cooling_total_output_owned_read_count = usize::MAX,
            7 => state.cooling_total_output_read_count = usize::MAX,
            8 => state.cp321_maximum_total_cooling_capacity_owned_read_count = usize::MAX,
            9 => state.cp340_same_call_maximum_total_cooling_capacity_bit_corroboration_count = usize::MAX,
            10 => state.maximum_total_cooling_capacity_read_count = usize::MAX,
            11 => state.cooling_total_output_maximum_total_cooling_capacity_comparison_count = usize::MAX,
            12 => state.cooling_total_output_strictly_greater_than_maximum_total_cooling_capacity_count = usize::MAX,
            13 => state.dehumidification_total_output_capacity_adjustment_body_entry_count = usize::MAX,
            _ => unreachable!(),
        }
        let before = state.clone();
        assert!(
            advance(&mut state, predecessor, active_input(predecessor, 90.0)).is_none(),
            "counter {counter} must fail closed",
        );
        assert_eq!(state, before);
    }
}

#[test]
fn cp383_rejects_guard_false_counter_overflow_without_mutation() {
    let predecessor = predecessor_for_route(3, 1, 1);
    let mut state = State::new(predecessor.system);
    state.dehumidification_total_output_capacity_guard_false_fallthrough_count = usize::MAX;
    let before = state.clone();
    assert!(advance(&mut state, predecessor, active_input(predecessor, 100.0)).is_none());
    assert_eq!(state, before);
}

#[test]
fn cp383_rejects_skip_route_overflow_without_mutation() {
    let predecessor = predecessor_for_route(0, 0, 1);
    let mut state = State::new(predecessor.system);
    state.unit_off_skip_count = usize::MAX;
    let before = state.clone();
    assert!(advance(&mut state, predecessor, None).is_none());
    assert_eq!(state, before);
}
