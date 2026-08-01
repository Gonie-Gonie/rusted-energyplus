//! CP381 checked-counter and commit-atomicity tests.

use super::super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardRuntimeState as State,
    advance_cooling_post_saturation_capacity_limit_dehumidification_guard_state as advance,
};
use super::{active_input, predecessor_for_route};

#[test]
fn cp381_every_retained_route_counter_overflow_is_transactional() {
    let cases = (0..3)
        .map(|inherited| (inherited, 0))
        .chain((3..8).flat_map(|inherited| (0..3).map(move |outcome| (inherited, outcome))));
    for (inherited, outcome) in cases {
        let predecessor = predecessor_for_route(inherited, outcome != 0, 1);
        let mut state = State::new(predecessor.system);
        poison_retained_route(&mut state, inherited, outcome);
        let before = state.clone();
        let input = match outcome {
            0 => None,
            1 => active_input(0.007, 0.009),
            2 => active_input(0.009, 0.009),
            _ => unreachable!(),
        };
        assert!(advance(&mut state, predecessor, input).is_none());
        assert_eq!(state, before);
    }
}

#[test]
fn cp381_inherited_and_cp380_body_counter_overflow_is_transactional() {
    for inherited in 3..8 {
        for counter in 0..2 {
            let predecessor = predecessor_for_route(inherited, true, 1);
            let mut state = State::new(predecessor.system);
            if counter == 0 {
                poison_inherited_route(&mut state, inherited);
            } else {
                poison_cp380_body_route(&mut state, inherited);
            }
            let before = state.clone();
            assert!(advance(&mut state, predecessor, active_input(0.007, 0.009),).is_none());
            assert_eq!(state, before);
        }
    }
}

#[test]
fn cp381_every_guard_and_source_counter_overflow_is_transactional() {
    for poison in 0..12 {
        let body = poison != 11;
        let predecessor = predecessor_for_route(4, true, 1);
        let mut state = State::new(predecessor.system);
        match poison {
            0 => state.transition_count = usize::MAX,
            1 => state.source_site_execution_count = usize::MAX,
            2 => state.dehumidification_guard_evaluation_count = usize::MAX,
            3 => state.cp378_supply_humidity_ratio_saturation_limit_owned_read_count = usize::MAX,
            4 => {
                state.cp379_same_call_supply_humidity_ratio_bit_corroboration_count = usize::MAX;
            }
            5 => state.purchased_air_supply_humidity_ratio_read_count = usize::MAX,
            6 => state.cp329_mixed_air_humidity_ratio_owned_read_count = usize::MAX,
            7 => state.purchased_air_mixed_air_humidity_ratio_read_count = usize::MAX,
            8 => {
                state.supply_humidity_ratio_mixed_air_humidity_ratio_comparison_count = usize::MAX;
            }
            9 => {
                state.supply_humidity_ratio_strictly_less_than_mixed_air_humidity_ratio_count =
                    usize::MAX;
            }
            10 => state.dehumidification_body_entry_count = usize::MAX,
            11 => state.dehumidification_guard_false_fallthrough_count = usize::MAX,
            _ => unreachable!(),
        }
        let before = state.clone();
        let input = if body {
            active_input(0.007, 0.009)
        } else {
            active_input(0.009, 0.009)
        };
        assert!(advance(&mut state, predecessor, input).is_none());
        assert_eq!(state, before);
    }
}

fn poison_retained_route(state: &mut State, inherited: usize, outcome: usize) {
    match (inherited, outcome) {
        (0, 0) => state.unit_off_skip_count = usize::MAX,
        (1, 0) => state.non_cooling_skip_count = usize::MAX,
        (2, 0) => state.positive_guard_false_fallthrough_skip_count = usize::MAX,
        (3, 0) => {
            state.heating_availability_guard_false_fallthrough_capacity_guard_false_count =
                usize::MAX;
        }
        (3, 1) => {
            state.heating_availability_guard_false_fallthrough_dehumidification_body_entry_count =
                usize::MAX;
        }
        (3, 2) => {
            state.heating_availability_guard_false_fallthrough_dehumidification_guard_false_count =
                usize::MAX;
        }
        (4, 0) => {
            state.humidification_control_guard_false_fallthrough_capacity_guard_false_count =
                usize::MAX;
        }
        (4, 1) => {
            state
                .humidification_control_guard_false_fallthrough_dehumidification_body_entry_count =
                usize::MAX;
        }
        (4, 2) => {
            state
                .humidification_control_guard_false_fallthrough_dehumidification_guard_false_count =
                usize::MAX;
        }
        (5, 0) => {
            state.dehumidification_control_humidistat_maximum_assignment_capacity_guard_false_count =
                usize::MAX;
        }
        (5, 1) => {
            state
                .dehumidification_control_humidistat_maximum_assignment_dehumidification_body_entry_count =
                usize::MAX;
        }
        (5, 2) => {
            state
                .dehumidification_control_humidistat_maximum_assignment_dehumidification_guard_false_count =
                usize::MAX;
        }
        (6, 0) => {
            state.dehumidification_control_none_maximum_assignment_capacity_guard_false_count =
                usize::MAX;
        }
        (6, 1) => {
            state
                .dehumidification_control_none_maximum_assignment_dehumidification_body_entry_count =
                usize::MAX;
        }
        (6, 2) => {
            state
                .dehumidification_control_none_maximum_assignment_dehumidification_guard_false_count =
                usize::MAX;
        }
        (7, 0) => {
            state.dehumidification_control_guard_false_fallthrough_capacity_guard_false_count =
                usize::MAX;
        }
        (7, 1) => {
            state
                .dehumidification_control_guard_false_fallthrough_dehumidification_body_entry_count =
                usize::MAX;
        }
        (7, 2) => {
            state
                .dehumidification_control_guard_false_fallthrough_dehumidification_guard_false_count =
                usize::MAX;
        }
        _ => unreachable!(),
    }
}

fn poison_inherited_route(state: &mut State, inherited: usize) {
    match inherited {
        3 => state.heating_availability_guard_false_fallthrough_count = usize::MAX,
        4 => state.humidification_control_guard_false_fallthrough_count = usize::MAX,
        5 => {
            state
                .dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count =
                usize::MAX;
        }
        6 => {
            state.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count =
                usize::MAX;
        }
        7 => state.dehumidification_control_guard_false_fallthrough_count = usize::MAX,
        _ => unreachable!(),
    }
}

fn poison_cp380_body_route(state: &mut State, inherited: usize) {
    match inherited {
        3 => state.heating_availability_guard_false_fallthrough_body_entry_count = usize::MAX,
        4 => state.humidification_control_guard_false_fallthrough_body_entry_count = usize::MAX,
        5 => {
            state.dehumidification_control_humidistat_maximum_assignment_body_entry_count =
                usize::MAX;
        }
        6 => {
            state.dehumidification_control_none_maximum_assignment_body_entry_count = usize::MAX;
        }
        7 => state.dehumidification_control_guard_false_fallthrough_body_entry_count = usize::MAX,
        _ => unreachable!(),
    }
}
