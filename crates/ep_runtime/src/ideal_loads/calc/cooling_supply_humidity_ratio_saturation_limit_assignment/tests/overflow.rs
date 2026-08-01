//! CP378 checked-counter atomicity tests.

use super::super::{
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentRuntimeState as State,
    advance_cooling_supply_humidity_ratio_saturation_limit_assignment_state as advance,
};
use super::predecessor_for_route;

#[test]
fn cp378_every_active_counter_overflow_is_transactional() {
    let predecessor = predecessor_for_route(4, 0.008);
    for poison in 0..9 {
        let mut state = State::new(predecessor.system);
        match poison {
            0 => state.transition_count = usize::MAX,
            1 => state.humidification_control_guard_false_fallthrough_count = usize::MAX,
            2 => state.source_site_execution_count = usize::MAX,
            3 => {
                state
                    .local_original_supply_humidity_ratio_for_saturation_limit_minimum_read_count =
                    usize::MAX;
            }
            4 => {
                state.local_saturation_supply_humidity_ratio_for_saturation_limit_minimum_read_count =
                    usize::MAX;
            }
            5 => state.source_shaped_two_argument_minimum_evaluation_count = usize::MAX,
            6 => {
                state.purchased_air_supply_humidity_ratio_saturation_limit_assignment_count =
                    usize::MAX;
            }
            7 => state.cp376_original_supply_humidity_ratio_owner_count = usize::MAX,
            8 => state.cp377_saturation_supply_humidity_ratio_owner_count = usize::MAX,
            _ => unreachable!(),
        }
        let before = state.clone();
        assert!(advance(&mut state, predecessor).is_none());
        assert_eq!(state, before);
    }
}

#[test]
fn cp378_every_route_counter_overflow_is_transactional() {
    for route in 0..8 {
        let predecessor = predecessor_for_route(route, 0.008);
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
        assert!(advance(&mut state, predecessor).is_none());
        assert_eq!(state, before);
    }
}
