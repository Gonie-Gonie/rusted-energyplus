//! CP379 checked-counter and commit atomicity tests.

use super::super::{
    PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentRuntimeState as State,
    advance_cooling_supply_enthalpy_post_saturation_assignment_state as advance,
};
use super::prefix_for_route;

#[test]
fn cp379_every_active_counter_overflow_is_transactional() {
    let prefix = prefix_for_route(4, 0.008);
    for poison in 0..9 {
        let mut state = State::new(prefix.cp378.system);
        match poison {
            0 => state.transition_count = usize::MAX,
            1 => state.humidification_control_guard_false_fallthrough_count = usize::MAX,
            2 => state.source_site_execution_count = usize::MAX,
            3 => {
                state.purchased_air_supply_temperature_for_post_saturation_enthalpy_read_count =
                    usize::MAX;
            }
            4 => {
                state.purchased_air_supply_humidity_ratio_for_post_saturation_enthalpy_read_count =
                    usize::MAX;
            }
            5 => state.psy_h_fn_tdb_w_for_post_saturation_enthalpy_evaluation_count = usize::MAX,
            6 => {
                state.local_supply_enthalpy_after_saturation_limit_assignment_count = usize::MAX;
            }
            7 => {
                state.cp344_capacity_limit_supply_temperature_mixed_air_limit_owner_count =
                    usize::MAX
            }
            8 => state.cp378_supply_humidity_ratio_saturation_limit_owner_count = usize::MAX,
            _ => unreachable!(),
        }
        let before = state.clone();
        assert!(advance(&mut state, prefix.cp378, prefix.input).is_none());
        assert_eq!(state, before);
    }
}

#[test]
fn cp379_every_route_counter_overflow_is_transactional() {
    for route in 0..8 {
        let prefix = prefix_for_route(route, 0.008);
        let mut state = State::new(prefix.cp378.system);
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
        assert!(advance(&mut state, prefix.cp378, prefix.input).is_none());
        assert_eq!(state, before);
    }
}
