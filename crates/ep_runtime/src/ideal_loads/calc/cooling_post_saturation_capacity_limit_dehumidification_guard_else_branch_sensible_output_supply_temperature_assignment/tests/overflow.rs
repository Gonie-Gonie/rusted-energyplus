//! CP423 checked-counter overflow and transactional-state tests.

use super::*;

#[test]
fn every_assignment_scalar_and_route_counter_overflow_is_transactional() {
    let predecessor = assignment_predecessor();
    let route = successor_route_for(predecessor);
    let input = active_input(predecessor);
    macro_rules! scalar {
        ($field:ident) => {{
            let mut state = State::new(predecessor.system);
            state.$field = usize::MAX;
            let before = state.clone();
            assert!(advance_validated(&mut state, predecessor, route, input).is_none(), stringify!($field));
            assert_eq!(state, before, stringify!($field));
        }};
    }
    scalar!(transition_count);
    scalar!(cooling_sensible_output_supply_temperature_assignment_count);
    scalar!(source_site_execution_count);
    scalar!(cp422_supply_temperature_state_owner_count);
    scalar!(cp423_sensible_output_supply_temperature_state_owner_count);
    scalar!(cp329_retained_mixed_air_temperature_for_sensible_output_supply_temperature_owned_read_count);
    scalar!(mixed_air_temperature_for_sensible_output_supply_temperature_read_count);
    scalar!(cp422_retained_cooling_sensible_output_owned_read_count);
    scalar!(cooling_sensible_output_for_supply_temperature_read_count);
    scalar!(cp330_retained_supply_mass_flow_rate_for_sensible_output_supply_temperature_owned_read_count);
    scalar!(cp329_supply_mass_flow_rate_for_sensible_output_supply_temperature_bit_corroboration_count);
    scalar!(supply_mass_flow_rate_for_sensible_output_supply_temperature_read_count);
    scalar!(cp419_retained_cp_air_for_sensible_output_supply_temperature_owned_read_count);
    scalar!(cp_air_for_sensible_output_supply_temperature_read_count);
    scalar!(supply_mass_flow_rate_times_cp_air_for_sensible_output_supply_temperature_calculation_count);
    scalar!(cooling_sensible_output_over_air_capacity_rate_calculation_count);
    scalar!(sensible_output_supply_temperature_calculation_count);
    scalar!(sensible_output_supply_temperature_assignment_write_count);

    let mut state = State::new(predecessor.system);
    state.predecessor_route_counts[route.logical_index] = usize::MAX;
    assert_transactional_failure(state, predecessor, route, input);
    let mut state = State::new(predecessor.system);
    state.cooling_sensible_output_supply_temperature_assignment_route_counts[route.logical_index] = usize::MAX;
    assert_transactional_failure(state, predecessor, route, input);
}

#[test]
fn inactive_false_and_wht_owner_overflows_are_transactional() {
    let snapshots = cp422_all_snapshots_for_successor_tests();
    let inactive = snapshots.iter().copied().find(|snapshot| !successor_route_for(*snapshot).active).expect("inactive");
    let inactive_route = successor_route_for(inactive);
    let mut state = State::new(inactive.system);
    state.inactive_transition_count = usize::MAX;
    assert_transactional_failure(state, inactive, inactive_route, None);

    let guard_false = snapshots.iter().copied().find(|snapshot| {
        let route = successor_route_for(*snapshot);
        route.active && !route.assignment_executed
    }).expect("guard false");
    let false_route = successor_route_for(guard_false);
    let mut state = State::new(guard_false.system);
    state.predecessor_guard_false_fallthrough_count = usize::MAX;
    assert_transactional_failure(state, guard_false, false_route, None);
    let mut state = State::new(guard_false.system);
    state.predecessor_guard_false_fallthrough_route_counts[false_route.logical_index] = usize::MAX;
    assert_transactional_failure(state, guard_false, false_route, None);

    let w = snapshots.iter().copied().find(|snapshot| snapshot.resulting_supply_humidity_ratio.is_some()).expect("W owner");
    let w_route = successor_route_for(w);
    let w_input = active_input(w);
    for owner in [true, false] {
        let mut state = State::new(w.system);
        if owner {
            state.cp422_supply_humidity_ratio_state_owner_count = usize::MAX;
        } else {
            state.unchanged_supply_humidity_ratio_preservation_count = usize::MAX;
        }
        assert_transactional_failure(state, w, w_route, w_input);
    }
    let h = snapshots.iter().copied().find(|snapshot| snapshot.resulting_supply_enthalpy_j_per_kg.is_some()).expect("H owner");
    let h_route = successor_route_for(h);
    let h_input = active_input(h);
    for owner in [true, false] {
        let mut state = State::new(h.system);
        if owner {
            state.cp422_supply_enthalpy_state_owner_count = usize::MAX;
        } else {
            state.unchanged_supply_enthalpy_preservation_count = usize::MAX;
        }
        assert_transactional_failure(state, h, h_route, h_input);
    }
    let t = snapshots.iter().copied().find(|snapshot| {
        snapshot.resulting_supply_temperature_c.is_some() && !successor_route_for(*snapshot).assignment_executed
    }).expect("preserved T owner");
    let t_route = successor_route_for(t);
    for owner in [true, false] {
        let mut state = State::new(t.system);
        if owner {
            state.cp422_supply_temperature_state_owner_count = usize::MAX;
        } else {
            state.unchanged_supply_temperature_preservation_count = usize::MAX;
        }
        assert_transactional_failure(state, t, t_route, None);
    }
}

fn assert_transactional_failure(
    mut state: State,
    predecessor: Predecessor,
    route: Route,
    input: Option<ActiveInput>,
) {
    let before = state.clone();
    assert!(advance_validated(&mut state, predecessor, route, input).is_none());
    assert_eq!(state, before);
}
