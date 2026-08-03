//! CP412 checked-counter transactional tests.

use super::{all_routes, predecessor_for_route};
use crate::ideal_loads::calc::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationAssignmentActiveInput as ActiveInput,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationAssignmentRuntimeState as State,
    advance_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_state as advance,
};
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment::transition::test_next_transition_fits;

#[test]
fn representative_counter_overflow_is_transactional() {
    let active = all_routes()
        .into_iter()
        .find(|route| route.predecessor_index == 20 && route.predecessor_guard_false_fallthrough)
        .expect("active split route");
    type Mutation = fn(&mut State);
    let mutations: &[Mutation] = &[
        |state| state.transition_count = usize::MAX,
        |state| state.predecessor_route_counts[20] = usize::MAX,
        |state| state.predecessor_guard_false_fallthrough_count = usize::MAX,
        |state| state.predecessor_guard_false_fallthrough_route_counts[20] = usize::MAX,
        |state| {
            state.predecessor_supply_humidity_ratio_pre_saturation_original_assignment_count =
                usize::MAX
        },
        |state| {
            state.predecessor_supply_humidity_ratio_pre_saturation_original_assignment_route_counts
                [20] = usize::MAX
        },
        |state| state.supply_humidity_ratio_saturation_assignment_count = usize::MAX,
        |state| state.supply_humidity_ratio_saturation_assignment_route_counts[20] = usize::MAX,
        |state| state.source_site_execution_count = usize::MAX,
        |state| state.cp411_supply_humidity_ratio_state_owner_count = usize::MAX,
        |state| state.unchanged_supply_humidity_ratio_preservation_count = usize::MAX,
        |state| state.cp411_supply_enthalpy_state_owner_count = usize::MAX,
        |state| state.unchanged_supply_enthalpy_preservation_count = usize::MAX,
        |state| state.cp411_supply_temperature_state_owner_count = usize::MAX,
        |state| state.unchanged_supply_temperature_preservation_count = usize::MAX,
        |state| state.cp411_retained_supply_temperature_owned_read_count = usize::MAX,
        |state| {
            state.purchased_air_supply_temperature_for_saturation_humidity_ratio_read_count =
                usize::MAX
        },
        |state| state.environment_outdoor_barometric_pressure_owner_count = usize::MAX,
        |state| {
            state.environment_outdoor_barometric_pressure_for_saturation_humidity_ratio_read_count =
                usize::MAX
        },
        |state| {
            state.psy_w_fn_tdb_rh_pb_at_unity_relative_humidity_evaluation_count = usize::MAX
        },
        |state| state.local_saturation_supply_humidity_ratio_assignment_write_count = usize::MAX,
    ];
    for mutate in mutations {
        let predecessor = predecessor_for_route(active, 1);
        let mut state = State::new(predecessor.system);
        mutate(&mut state);
        let before = state.clone();
        assert!(!test_next_transition_fits(&state, active));
        assert!(advance(
            &mut state,
            predecessor,
            Some(ActiveInput {
                outdoor_barometric_pressure_pa: 101_325.0,
            }),
        )
        .is_none());
        assert_eq!(state, before);
    }

    let maximum = all_routes()
        .into_iter()
        .find(|route| {
            route.predecessor_index == 20
                && route.predecessor_maximum_capacity_assignment_executed
        })
        .expect("active maximum route");
    for mutate in [
        (|state: &mut State| state.predecessor_maximum_capacity_assignment_count = usize::MAX)
            as Mutation,
        |state: &mut State| {
            state.predecessor_maximum_capacity_assignment_route_counts[20] = usize::MAX;
        },
    ] {
        let predecessor = predecessor_for_route(maximum, 1);
        let mut state = State::new(predecessor.system);
        mutate(&mut state);
        let before = state.clone();
        assert!(!test_next_transition_fits(&state, maximum));
        assert!(advance(
            &mut state,
            predecessor,
            Some(ActiveInput {
                outdoor_barometric_pressure_pa: 101_325.0,
            }),
        )
        .is_none());
        assert_eq!(state, before);
    }

    let inactive = all_routes()[0];
    let predecessor = predecessor_for_route(inactive, 1);
    let mut state = State::new(predecessor.system);
    state.inactive_transition_count = usize::MAX;
    let before = state.clone();
    assert!(!test_next_transition_fits(&state, inactive));
    assert!(advance(&mut state, predecessor, None).is_none());
    assert_eq!(state, before);
}
