//! CP377 checked-counter atomicity tests.

use super::super::{
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentTemperatureOwner as Owner,
    advance_cooling_supply_humidity_ratio_saturation_assignment_state as advance,
};
use super::predecessor_for_route;
use super::routes::input_for_route;

#[test]
fn cp377_active_counter_overflow_is_transactional() {
    let predecessor = predecessor_for_route(4, 0.008);
    for poison in 0..10 {
        let mut input = input_for_route(4).expect("active input");
        let mut state = State::new(predecessor.system);
        match poison {
            0 => state.transition_count = usize::MAX,
            1 => state.humidification_control_guard_false_fallthrough_count = usize::MAX,
            2 => state.source_site_execution_count = usize::MAX,
            3 => {
                state.purchased_air_supply_temperature_for_saturation_humidity_ratio_read_count =
                    usize::MAX;
            }
            4 => {
                state.environment_outdoor_barometric_pressure_for_saturation_humidity_ratio_read_count =
                    usize::MAX;
            }
            5 => {
                state.psy_w_fn_tdb_rh_pb_at_unity_relative_humidity_evaluation_count = usize::MAX;
            }
            6 => state.local_saturation_supply_humidity_ratio_assignment_count = usize::MAX,
            7 => {
                input.temperature_owner = Owner::Cp334MixedAirLimit;
                state.cp334_supply_temperature_mixed_air_limit_owner_count = usize::MAX;
            }
            8 => {
                input.temperature_owner = Owner::Cp344CapacityMixedAirLimit;
                state.cp344_capacity_limit_supply_temperature_mixed_air_limit_owner_count =
                    usize::MAX;
            }
            9 => state.environment_outdoor_barometric_pressure_owner_count = usize::MAX,
            _ => unreachable!(),
        }
        let before = state.clone();
        assert!(advance(&mut state, predecessor, Some(input)).is_none());
        assert_eq!(state, before);
    }
}

#[test]
fn cp377_skipped_route_counter_overflow_is_transactional() {
    let predecessor = predecessor_for_route(0, 0.0);
    let mut state = State::new(predecessor.system);
    state.unit_off_skip_count = usize::MAX;
    let before = state.clone();
    assert!(advance(&mut state, predecessor, None).is_none());
    assert_eq!(state, before);
}

#[test]
fn cp377_every_route_counter_overflow_is_transactional() {
    for route in 0..8 {
        let predecessor = predecessor_for_route(route, 0.008);
        let input = input_for_route(route);
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
        assert!(advance(&mut state, predecessor, input).is_none());
        assert_eq!(state, before);
    }
}
