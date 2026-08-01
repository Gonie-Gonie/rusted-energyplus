//! CP382 checked-counter and commit-atomicity tests.

use super::super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentRuntimeState as State,
    advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_state as advance,
};
use super::{active_input, predecessor_for_route};

#[test]
fn cp382_every_retained_route_counter_overflow_is_transactional() {
    let cases = (0..3)
        .map(|inherited| (inherited, 0))
        .chain((3..8).flat_map(|inherited| (0..3).map(move |outcome| (inherited, outcome))));
    for (inherited, outcome) in cases {
        let predecessor = predecessor_for_route(inherited, outcome, 1);
        let mut state = State::new(predecessor.system);
        poison_retained_route(&mut state, inherited, outcome);
        let before = state.clone();
        let input = (outcome == 1)
            .then(|| active_input(1.0, 48_000.0, 40_000.0).expect("active CP382 input"));
        assert!(advance(&mut state, predecessor, input).is_none());
        assert_eq!(state, before);
    }
}

#[test]
fn cp382_inherited_and_body_partition_overflow_is_transactional() {
    for inherited in 3..8 {
        for body_partition in [false, true] {
            let predecessor = predecessor_for_route(inherited, 1, 1);
            let mut state = State::new(predecessor.system);
            if body_partition {
                poison_body_partition(&mut state, inherited);
            } else {
                poison_inherited_route(&mut state, inherited);
            }
            let before = state.clone();
            assert!(
                advance(
                    &mut state,
                    predecessor,
                    active_input(1.0, 48_000.0, 40_000.0),
                )
                .is_none()
            );
            assert_eq!(state, before);
        }
    }
}

#[test]
fn cp382_every_global_and_active_counter_overflow_is_transactional() {
    for poison in 0..17 {
        let predecessor = predecessor_for_route(4, 1, 1);
        let mut state = State::new(predecessor.system);
        match poison {
            0 => state.transition_count = usize::MAX,
            1 => state.dehumidification_total_output_assignment_count = usize::MAX,
            2 => state.source_site_execution_count = usize::MAX,
            3 => state.cp330_supply_mass_flow_rate_owned_read_count = usize::MAX,
            4 => state.cp329_same_call_supply_mass_flow_rate_bit_corroboration_count = usize::MAX,
            5 => state.cp339_same_call_supply_mass_flow_rate_bit_corroboration_count = usize::MAX,
            6 => state.supply_mass_flow_rate_read_count = usize::MAX,
            7 => state.cp329_mixed_air_enthalpy_owned_read_count = usize::MAX,
            8 => state.cp329_same_call_recirculation_enthalpy_bit_corroboration_count = usize::MAX,
            9 => state.cp339_same_call_mixed_air_enthalpy_bit_corroboration_count = usize::MAX,
            10 => state.mixed_air_enthalpy_read_count = usize::MAX,
            11 => state.cp379_post_saturation_supply_enthalpy_owned_read_count = usize::MAX,
            12 => state.cp379_same_call_supply_enthalpy_bits_corroboration_count = usize::MAX,
            13 => state.supply_enthalpy_read_count = usize::MAX,
            14 => state.enthalpy_difference_calculation_count = usize::MAX,
            15 => state.cooling_total_output_calculation_count = usize::MAX,
            16 => state.cooling_total_output_assignment_write_count = usize::MAX,
            _ => unreachable!(),
        }
        let before = state.clone();
        assert!(
            advance(
                &mut state,
                predecessor,
                active_input(1.0, 48_000.0, 40_000.0),
            )
            .is_none()
        );
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
            state.heating_availability_guard_false_fallthrough_dehumidification_total_output_assignment_count = usize::MAX;
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
            state.humidification_control_guard_false_fallthrough_dehumidification_total_output_assignment_count = usize::MAX;
        }
        (4, 2) => {
            state.humidification_control_guard_false_fallthrough_dehumidification_guard_false_count = usize::MAX;
        }
        (5, 0) => {
            state.dehumidification_control_humidistat_maximum_assignment_capacity_guard_false_count = usize::MAX;
        }
        (5, 1) => {
            state.dehumidification_control_humidistat_maximum_assignment_dehumidification_total_output_assignment_count = usize::MAX;
        }
        (5, 2) => {
            state.dehumidification_control_humidistat_maximum_assignment_dehumidification_guard_false_count = usize::MAX;
        }
        (6, 0) => {
            state.dehumidification_control_none_maximum_assignment_capacity_guard_false_count =
                usize::MAX;
        }
        (6, 1) => {
            state.dehumidification_control_none_maximum_assignment_dehumidification_total_output_assignment_count = usize::MAX;
        }
        (6, 2) => {
            state.dehumidification_control_none_maximum_assignment_dehumidification_guard_false_count = usize::MAX;
        }
        (7, 0) => {
            state.dehumidification_control_guard_false_fallthrough_capacity_guard_false_count =
                usize::MAX;
        }
        (7, 1) => {
            state.dehumidification_control_guard_false_fallthrough_dehumidification_total_output_assignment_count = usize::MAX;
        }
        (7, 2) => {
            state.dehumidification_control_guard_false_fallthrough_dehumidification_guard_false_count = usize::MAX;
        }
        _ => unreachable!(),
    }
}

fn poison_inherited_route(state: &mut State, inherited: usize) {
    match inherited {
        3 => state.heating_availability_guard_false_fallthrough_count = usize::MAX,
        4 => state.humidification_control_guard_false_fallthrough_count = usize::MAX,
        5 => {
            state.dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count = usize::MAX;
        }
        6 => {
            state.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count =
                usize::MAX;
        }
        7 => state.dehumidification_control_guard_false_fallthrough_count = usize::MAX,
        _ => unreachable!(),
    }
}

fn poison_body_partition(state: &mut State, inherited: usize) {
    match inherited {
        3 => {
            state.heating_availability_guard_false_fallthrough_dehumidification_body_entry_count =
                usize::MAX;
        }
        4 => {
            state
                .humidification_control_guard_false_fallthrough_dehumidification_body_entry_count =
                usize::MAX;
        }
        5 => {
            state.dehumidification_control_humidistat_maximum_assignment_dehumidification_body_entry_count = usize::MAX;
        }
        6 => {
            state.dehumidification_control_none_maximum_assignment_dehumidification_body_entry_count = usize::MAX;
        }
        7 => {
            state.dehumidification_control_guard_false_fallthrough_dehumidification_body_entry_count = usize::MAX;
        }
        _ => unreachable!(),
    }
}
