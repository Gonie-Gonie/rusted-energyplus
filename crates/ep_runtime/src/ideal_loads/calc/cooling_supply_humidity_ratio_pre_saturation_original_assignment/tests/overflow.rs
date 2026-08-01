//! CP376 checked-counter atomicity tests.

use super::super::{
    advance_cooling_supply_humidity_ratio_pre_saturation_original_assignment_state as advance,
    PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentRuntimeState as State,
};
use super::release::completed_cp375_case;
use super::routes::{input_for_route, predecessor_for_route};

#[test]
fn cp376_active_counter_overflow_is_transactional() {
    let (_, _, direct) = completed_cp375_case();
    let predecessor = predecessor_for_route(direct, 4, 0.008);
    let input = input_for_route(4, 0.008);
    for poison in 0..6 {
        let mut state = State::new(predecessor.system);
        match poison {
            0 => state.transition_count = usize::MAX,
            1 => state.humidification_control_guard_false_fallthrough_count = usize::MAX,
            2 => state.source_site_execution_count = usize::MAX,
            3 => {
                state.purchased_air_supply_humidity_ratio_before_saturation_limit_read_count =
                    usize::MAX;
            }
            4 => {
                state.local_original_supply_humidity_ratio_before_saturation_limit_assignment_count =
                    usize::MAX;
            }
            5 => state.cp347_none_case_owner_count = usize::MAX,
            _ => unreachable!(),
        }
        let before = state.clone();
        assert!(advance(&mut state, predecessor, input).is_none());
        assert_eq!(state, before);
    }
}

#[test]
fn cp376_skipped_route_counter_overflow_is_transactional() {
    let (_, _, direct) = completed_cp375_case();
    let predecessor = predecessor_for_route(direct, 0, 0.0);
    let mut state = State::new(predecessor.system);
    state.unit_off_skip_count = usize::MAX;
    let before = state.clone();
    assert!(advance(&mut state, predecessor, None).is_none());
    assert_eq!(state, before);
}
