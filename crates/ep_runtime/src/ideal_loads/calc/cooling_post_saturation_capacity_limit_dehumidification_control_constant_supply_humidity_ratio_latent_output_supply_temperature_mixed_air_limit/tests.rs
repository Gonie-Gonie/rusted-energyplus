//! CP408 boundary, route, IEEE, accounting, corruption, and overflow tests.

use super::release::test_counts_are_exact;
use super::transition::{
    RetainedRoute, logical_route_index, predecessor_index_is_active, predecessor_index_is_public,
    source_minimum, test_increment_counts, test_next_transition_fits,
};
use super::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE as EXCLUDED,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE as SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER as ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureMixedAirLimitRuntimeState as State,
};

#[test]
fn cp408_boundary_and_canonical_four_source_sites_are_exact() {
    assert_eq!(SOURCE, "EnergyPlus 26.1 PurchasedAirManager.cc:2304");
    assert_eq!(EXCLUDED, "EnergyPlus 26.1 PurchasedAirManager.cc:2306");
    assert_eq!(
        ORDER,
        &[
            "read-purchased-air-supply-temperature-for-minimum",
            "read-purchased-air-mixed-air-temperature-for-minimum",
            "apply-source-shaped-two-argument-minimum",
            "assign-purchased-air-supply-temperature",
        ],
    );
}

fn all_routes() -> Vec<RetainedRoute> {
    let mut routes = Vec::new();
    for predecessor_index in 0..30 {
        if predecessor_index_is_active(predecessor_index) {
            routes.push(RetainedRoute {
                predecessor_index,
                predecessor_guard_evaluated: true,
                predecessor_maximum_capacity_assignment_executed: false,
                active: true,
            });
            routes.push(RetainedRoute {
                predecessor_index,
                predecessor_guard_evaluated: true,
                predecessor_maximum_capacity_assignment_executed: true,
                active: false,
            });
        } else {
            routes.push(RetainedRoute {
                predecessor_index,
                predecessor_guard_evaluated: false,
                predecessor_maximum_capacity_assignment_executed: false,
                active: false,
            });
        }
    }
    routes
}

#[test]
fn thirty_six_public_and_private_routes_have_exact_flattened_order() {
    let routes = all_routes();
    assert_eq!(routes.len(), 36);
    assert_eq!(
        routes
            .iter()
            .copied()
            .map(logical_route_index)
            .collect::<Vec<_>>(),
        (0..36).collect::<Vec<_>>(),
    );
    assert_eq!(
        routes
            .iter()
            .copied()
            .filter(|route| route.active)
            .map(logical_route_index)
            .collect::<Vec<_>>(),
        [20, 22, 26, 28, 31, 34],
    );
    assert_eq!(
        routes
            .iter()
            .filter(|route| predecessor_index_is_public(route.predecessor_index))
            .count(),
        13,
    );
    assert_eq!(
        routes
            .iter()
            .copied()
            .filter(|route| {
                route.active && predecessor_index_is_public(route.predecessor_index)
            })
            .map(logical_route_index)
            .collect::<Vec<_>>(),
        [20, 26],
    );
}

#[test]
fn source_minimum_preserves_raw_cpp_ieee_semantics() {
    let left_nan = f64::from_bits(0x7ff8_0000_0000_0042);
    let right_nan = f64::from_bits(0x7ff8_0000_0000_0084);
    for (left, right) in [
        (15.0, 21.0),
        (21.0, 15.0),
        (0.0, -0.0),
        (-0.0, 0.0),
        (f64::NEG_INFINITY, 20.0),
        (f64::INFINITY, 20.0),
        (left_nan, 20.0),
        (20.0, right_nan),
    ] {
        let expected = if left < right { left } else { right };
        assert_eq!(source_minimum(left, right).to_bits(), expected.to_bits());
    }
}

fn exhaustive_state() -> State {
    let mut state = State::new(ep_model::IdealLoadsAirSystemId(408));
    for route in all_routes() {
        assert!(test_next_transition_fits(&state, route));
        test_increment_counts(&mut state, route);
    }
    state
}

#[test]
fn exhaustive_accounting_is_36_30_6_24_with_exact_owners() {
    let state = exhaustive_state();
    assert!(test_counts_are_exact(&state));
    assert_eq!(state.transition_count, 36);
    assert_eq!(state.inactive_transition_count, 30);
    assert_eq!(state.predecessor_guard_false_fallthrough_count, 6);
    assert_eq!(state.predecessor_maximum_capacity_assignment_count, 6);
    assert_eq!(state.predecessor_else_branch_entry_count, 6);
    assert_eq!(state.predecessor_supply_temperature_assignment_count, 6);
    assert_eq!(
        state.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit_count,
        6,
    );
    assert_eq!(state.source_site_execution_count, 24);
    for count in [
        state.cp407_retained_supply_temperature_owned_read_count,
        state.supply_temperature_for_minimum_read_count,
        state.cp329_retained_mixed_air_temperature_owned_read_count,
        state.mixed_air_temperature_for_minimum_read_count,
        state.source_shaped_two_argument_minimum_evaluation_count,
        state.supply_temperature_assignment_write_count,
    ] {
        assert_eq!(count, 6);
    }
    assert_eq!(state.cp407_supply_temperature_state_owner_count, 33);
    assert_eq!(state.unchanged_supply_humidity_ratio_preservation_count, 18);
    assert_eq!(state.unchanged_supply_enthalpy_preservation_count, 23);
    assert_eq!(state.unchanged_supply_temperature_preservation_count, 27);
}

#[test]
fn counter_corruption_is_fail_closed() {
    let exact = exhaustive_state();
    let mutations: &[fn(&mut State)] = &[
        |s| s.transition_count += 1,
        |s| s.inactive_transition_count += 1,
        |s| s.predecessor_guard_false_fallthrough_count += 1,
        |s| s.predecessor_maximum_capacity_assignment_count += 1,
        |s| s.predecessor_else_branch_entry_count += 1,
        |s| s.predecessor_supply_temperature_assignment_count += 1,
        |s| {
            s.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit_count += 1
        },
        |s| s.predecessor_route_counts[20] += 1,
        |s| s.predecessor_guard_false_fallthrough_route_counts[20] += 1,
        |s| s.predecessor_maximum_capacity_assignment_route_counts[20] += 1,
        |s| s.predecessor_else_branch_entry_route_counts[20] += 1,
        |s| s.predecessor_supply_temperature_assignment_route_counts[20] += 1,
        |s| s.supply_temperature_mixed_air_limit_route_counts[20] += 1,
        |s| s.source_site_execution_count += 1,
        |s| s.cp407_retained_supply_temperature_owned_read_count += 1,
        |s| s.cp329_retained_mixed_air_temperature_owned_read_count += 1,
        |s| s.cp407_supply_temperature_state_owner_count += 1,
        |s| s.unchanged_supply_humidity_ratio_preservation_count += 1,
        |s| s.unchanged_supply_enthalpy_preservation_count += 1,
        |s| s.unchanged_supply_temperature_preservation_count += 1,
    ];
    for mutate in mutations {
        let mut corrupted = exact.clone();
        mutate(&mut corrupted);
        assert!(!test_counts_are_exact(&corrupted));
    }
}

#[test]
fn every_incremented_counter_is_checked_transactionally_for_overflow() {
    type OverflowMutation = (RetainedRoute, fn(&mut State));
    const INACTIVE: RetainedRoute = RetainedRoute {
        predecessor_index: 0,
        predecessor_guard_evaluated: false,
        predecessor_maximum_capacity_assignment_executed: false,
        active: false,
    };
    const ACTIVE: RetainedRoute = RetainedRoute {
        predecessor_index: 20,
        predecessor_guard_evaluated: true,
        predecessor_maximum_capacity_assignment_executed: false,
        active: true,
    };
    const MAXIMUM_SIBLING: RetainedRoute = RetainedRoute {
        predecessor_index: 20,
        predecessor_guard_evaluated: true,
        predecessor_maximum_capacity_assignment_executed: true,
        active: false,
    };
    let mutations: &[OverflowMutation] = &[
        (INACTIVE, |s| s.transition_count = usize::MAX),
        (INACTIVE, |s| s.predecessor_route_counts[0] = usize::MAX),
        (INACTIVE, |s| s.inactive_transition_count = usize::MAX),
        (ACTIVE, |s| {
            s.predecessor_guard_false_fallthrough_count = usize::MAX
        }),
        (ACTIVE, |s| {
            s.predecessor_guard_false_fallthrough_route_counts[20] = usize::MAX
        }),
        (ACTIVE, |s| {
            s.predecessor_else_branch_entry_count = usize::MAX
        }),
        (ACTIVE, |s| {
            s.predecessor_else_branch_entry_route_counts[20] = usize::MAX
        }),
        (ACTIVE, |s| {
            s.predecessor_supply_temperature_assignment_count = usize::MAX
        }),
        (ACTIVE, |s| {
            s.predecessor_supply_temperature_assignment_route_counts[20] = usize::MAX
        }),
        (ACTIVE, |s| {
            s.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit_count = usize::MAX
        }),
        (ACTIVE, |s| {
            s.supply_temperature_mixed_air_limit_route_counts[20] = usize::MAX
        }),
        (ACTIVE, |s| s.source_site_execution_count = usize::MAX),
        (ACTIVE, |s| {
            s.cp407_retained_supply_temperature_owned_read_count = usize::MAX
        }),
        (ACTIVE, |s| {
            s.supply_temperature_for_minimum_read_count = usize::MAX
        }),
        (ACTIVE, |s| {
            s.cp329_retained_mixed_air_temperature_owned_read_count = usize::MAX
        }),
        (ACTIVE, |s| {
            s.mixed_air_temperature_for_minimum_read_count = usize::MAX
        }),
        (ACTIVE, |s| {
            s.source_shaped_two_argument_minimum_evaluation_count = usize::MAX
        }),
        (ACTIVE, |s| {
            s.supply_temperature_assignment_write_count = usize::MAX
        }),
        (ACTIVE, |s| {
            s.cp407_supply_temperature_state_owner_count = usize::MAX
        }),
        (ACTIVE, |s| {
            s.unchanged_supply_humidity_ratio_preservation_count = usize::MAX
        }),
        (ACTIVE, |s| {
            s.unchanged_supply_enthalpy_preservation_count = usize::MAX
        }),
        (MAXIMUM_SIBLING, |s| {
            s.inactive_transition_count = usize::MAX
        }),
        (MAXIMUM_SIBLING, |s| {
            s.predecessor_maximum_capacity_assignment_count = usize::MAX
        }),
        (MAXIMUM_SIBLING, |s| {
            s.predecessor_maximum_capacity_assignment_route_counts[20] = usize::MAX
        }),
        (MAXIMUM_SIBLING, |s| {
            s.unchanged_supply_temperature_preservation_count = usize::MAX
        }),
    ];
    for (route, mutate) in mutations {
        let mut state = State::new(ep_model::IdealLoadsAirSystemId(408));
        mutate(&mut state);
        let before = state.clone();
        assert!(!test_next_transition_fits(&state, *route));
        assert_eq!(state, before);
    }
}

#[test]
fn new_state_is_exactly_zeroed_and_consistent() {
    let state = State::new(ep_model::IdealLoadsAirSystemId(408));
    assert!(test_counts_are_exact(&state));
    assert_eq!(state.predecessor_route_counts, [0; 30]);
    assert_eq!(
        state.predecessor_supply_temperature_assignment_route_counts,
        [0; 30]
    );
    assert_eq!(
        state.supply_temperature_mixed_air_limit_route_counts,
        [0; 30]
    );
    assert!(state.latest.is_none());
}
