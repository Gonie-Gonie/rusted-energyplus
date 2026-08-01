//! CP380 checked-counter and commit-atomicity tests.

use ep_model::IdealLoadsLimit;

use super::super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardRuntimeState as State,
    advance_cooling_post_saturation_capacity_limit_guard_state as advance,
};
use super::{active_input, predecessor_for_route};

#[test]
fn cp380_every_inherited_route_counter_overflow_is_transactional() {
    for route in 0..8 {
        let predecessor = predecessor_for_route(route, 1);
        let mut state = State::new(predecessor.system);
        match route {
            0 => state.unit_off_skip_count = usize::MAX,
            1 => state.non_cooling_skip_count = usize::MAX,
            2 => state.positive_guard_false_fallthrough_skip_count = usize::MAX,
            3 => state.heating_availability_guard_false_fallthrough_count = usize::MAX,
            4 => state.humidification_control_guard_false_fallthrough_count = usize::MAX,
            5 => {
                state
                    .dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count = usize::MAX;
            }
            6 => {
                state
                    .dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count =
                    usize::MAX;
            }
            7 => state.dehumidification_control_guard_false_fallthrough_count = usize::MAX,
            _ => unreachable!(),
        }
        let before = state.clone();
        let input = (route >= 3).then(|| active_input(IdealLoadsLimit::NoLimit).unwrap());
        assert!(advance(&mut state, predecessor, input).is_none());
        assert_eq!(state, before);
    }
}

#[test]
fn cp380_every_active_route_outcome_counter_overflow_is_transactional() {
    for route in 3..8 {
        for body in [true, false] {
            let predecessor = predecessor_for_route(route, 1);
            let mut state = State::new(predecessor.system);
            match (route, body) {
                (3, true) => {
                    state.heating_availability_guard_false_fallthrough_body_entry_count =
                        usize::MAX;
                }
                (3, false) => {
                    state.heating_availability_guard_false_fallthrough_capacity_guard_false_count =
                        usize::MAX;
                }
                (4, true) => {
                    state.humidification_control_guard_false_fallthrough_body_entry_count =
                        usize::MAX;
                }
                (4, false) => {
                    state.humidification_control_guard_false_fallthrough_capacity_guard_false_count =
                        usize::MAX;
                }
                (5, true) => {
                    state.dehumidification_control_humidistat_maximum_assignment_body_entry_count =
                        usize::MAX;
                }
                (5, false) => {
                    state.dehumidification_control_humidistat_maximum_assignment_capacity_guard_false_count =
                        usize::MAX;
                }
                (6, true) => {
                    state.dehumidification_control_none_maximum_assignment_body_entry_count =
                        usize::MAX;
                }
                (6, false) => {
                    state.dehumidification_control_none_maximum_assignment_capacity_guard_false_count =
                        usize::MAX;
                }
                (7, true) => {
                    state.dehumidification_control_guard_false_fallthrough_body_entry_count =
                        usize::MAX;
                }
                (7, false) => {
                    state.dehumidification_control_guard_false_fallthrough_capacity_guard_false_count =
                        usize::MAX;
                }
                _ => unreachable!(),
            }
            let before = state.clone();
            let limit = if body {
                IdealLoadsLimit::LimitCapacity
            } else {
                IdealLoadsLimit::NoLimit
            };
            assert!(advance(&mut state, predecessor, active_input(limit)).is_none());
            assert_eq!(state, before);
        }
    }
}

#[test]
fn cp380_every_selector_and_source_counter_overflow_is_transactional() {
    for poison in 0..14 {
        let limit = match poison {
            7 | 11 => IdealLoadsLimit::LimitCapacity,
            10 => IdealLoadsLimit::LimitFlowRateAndCapacity,
            _ => IdealLoadsLimit::NoLimit,
        };
        let predecessor = predecessor_for_route(4, 1);
        let mut state = State::new(predecessor.system);
        match poison {
            0 => state.transition_count = usize::MAX,
            1 => state.source_site_execution_count = usize::MAX,
            2 => state.capacity_limit_guard_evaluation_count = usize::MAX,
            3 => state.configured_cooling_limit_owned_read_count = usize::MAX,
            4 => state.cp337_same_call_selector_lineage_corroboration_count = usize::MAX,
            5 => state.first_cooling_limit_read_count = usize::MAX,
            6 => state.cooling_limit_capacity_comparison_count = usize::MAX,
            7 => state.cooling_limit_capacity_match_count = usize::MAX,
            8 => state.second_cooling_limit_read_count = usize::MAX,
            9 => state.cooling_limit_flow_rate_and_capacity_comparison_count = usize::MAX,
            10 => state.cooling_limit_flow_rate_and_capacity_match_count = usize::MAX,
            11 => state.capacity_limit_body_entry_count = usize::MAX,
            12 => state.cooling_limit_rejected_count = usize::MAX,
            13 => state.active_guard_false_fallthrough_count = usize::MAX,
            _ => unreachable!(),
        }
        let before = state.clone();
        assert!(advance(&mut state, predecessor, active_input(limit)).is_none());
        assert_eq!(state, before);
    }
}
