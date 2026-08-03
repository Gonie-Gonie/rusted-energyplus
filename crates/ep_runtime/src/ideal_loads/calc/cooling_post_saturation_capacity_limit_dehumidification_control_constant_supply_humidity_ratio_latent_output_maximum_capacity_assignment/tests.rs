//! CP405 boundary, route, IEEE, accounting, corruption, and overflow tests.

use super::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE as EXCLUDED,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE as SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE_ORDER as ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputMaximumCapacityAssignmentRuntimeState as State,
};
use super::release::test_counts_are_exact;
use super::transition::{
    RetainedRoute, logical_route_index, predecessor_index_is_active,
    predecessor_index_is_public, source_assignment, test_increment_counts,
    test_next_transition_fits,
};

#[test]
fn cp405_boundary_and_two_source_sites_are_exact() {
    assert_eq!(SOURCE, "EnergyPlus 26.1 PurchasedAirManager.cc:2300");
    assert_eq!(EXCLUDED, "EnergyPlus 26.1 PurchasedAirManager.cc:2302");
    assert_eq!(
        ORDER,
        &[
            "read-retained-maximum-total-cooling-capacity-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-latent-output-assignment",
            "assign-local-cooling-latent-output-from-maximum-total-cooling-capacity",
        ],
    );
}

fn all_routes() -> Vec<RetainedRoute> {
    let mut routes = Vec::new();
    for predecessor_index in 0..30 {
        if predecessor_index_is_active(predecessor_index) {
            routes.push(RetainedRoute {
                predecessor_index,
                guard_evaluated: true,
                assignment_executed: false,
            });
            routes.push(RetainedRoute {
                predecessor_index,
                guard_evaluated: true,
                assignment_executed: true,
            });
        } else {
            routes.push(RetainedRoute {
                predecessor_index,
                guard_evaluated: false,
                assignment_executed: false,
            });
        }
    }
    routes
}

#[test]
fn thirty_six_routes_and_public_release_partition_are_exact() {
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
            .filter(|route| route.assignment_executed)
            .map(|route| route.predecessor_index)
            .collect::<Vec<_>>(),
        [20, 21, 24, 25, 27, 29],
    );
    assert_eq!(
        routes
            .iter()
            .copied()
            .filter(|route| route.assignment_executed)
            .map(logical_route_index)
            .collect::<Vec<_>>(),
        [21, 23, 27, 29, 32, 35],
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
                route.assignment_executed
                    && predecessor_index_is_public(route.predecessor_index)
            })
            .map(logical_route_index)
            .collect::<Vec<_>>(),
        [21, 27],
    );
}

#[test]
fn source_assignment_copies_every_binary64_payload_bit_exactly() {
    for bits in [
        0,
        1u64 << 63,
        1,
        f64::INFINITY.to_bits(),
        f64::NEG_INFINITY.to_bits(),
        0x7ff8_0000_0000_0405,
    ] {
        assert_eq!(source_assignment(f64::from_bits(bits)).to_bits(), bits);
    }
}

fn exhaustive_state() -> State {
    let mut state = State::new(ep_model::IdealLoadsAirSystemId(405));
    for route in all_routes() {
        assert!(test_next_transition_fits(&state, route));
        test_increment_counts(&mut state, route);
    }
    state
}

#[test]
fn exhaustive_accounting_is_36_30_6_12_with_exact_owners() {
    let state = exhaustive_state();
    assert!(test_counts_are_exact(&state));
    assert_eq!(state.transition_count, 36);
    assert_eq!(state.inactive_transition_count, 24);
    assert_eq!(state.predecessor_guard_false_fallthrough_count, 6);
    assert_eq!(
        state.cooling_latent_output_maximum_capacity_assignment_count,
        6,
    );
    assert_eq!(state.source_site_execution_count, 12);
    assert_eq!(state.cp404_supply_humidity_ratio_state_owner_count, 12);
    assert_eq!(state.cp404_supply_enthalpy_state_owner_count, 23);
    assert_eq!(state.cp404_supply_temperature_state_owner_count, 33);
    assert_eq!(
        state
            .predecessor_route_counts
            .iter()
            .filter(|count| **count == 2)
            .count(),
        6,
    );
}

#[test]
fn counter_corruption_is_fail_closed() {
    let exact = exhaustive_state();
    let mutations: &[fn(&mut State)] = &[
        |s| s.transition_count += 1,
        |s| s.inactive_transition_count += 1,
        |s| s.predecessor_guard_false_fallthrough_count += 1,
        |s| s.cooling_latent_output_maximum_capacity_assignment_count += 1,
        |s| s.predecessor_route_counts[20] += 1,
        |s| s.predecessor_guard_false_fallthrough_route_counts[20] += 1,
        |s| s.cooling_latent_output_maximum_capacity_assignment_route_counts[20] += 1,
        |s| s.source_site_execution_count += 1,
        |s| s.cp404_supply_humidity_ratio_state_owner_count += 1,
        |s| s.unchanged_supply_humidity_ratio_preservation_count += 1,
        |s| s.cp404_supply_enthalpy_state_owner_count += 1,
        |s| s.cp404_supply_temperature_state_owner_count += 1,
        |s| s.cp404_retained_maximum_total_cooling_capacity_owned_read_count += 1,
        |s| s.maximum_total_cooling_capacity_read_count += 1,
        |s| s.cooling_latent_output_assignment_write_count += 1,
    ];
    for mutate in mutations {
        let mut corrupted = exact.clone();
        mutate(&mut corrupted);
        assert!(!test_counts_are_exact(&corrupted));
    }
}

#[test]
fn every_incremented_counter_is_checked_for_overflow() {
    type OverflowMutation = (RetainedRoute, fn(&mut State));

    const INACTIVE: RetainedRoute = RetainedRoute {
        predecessor_index: 0,
        guard_evaluated: false,
        assignment_executed: false,
    };
    const FALSE: RetainedRoute = RetainedRoute {
        predecessor_index: 20,
        guard_evaluated: true,
        assignment_executed: false,
    };
    const ASSIGN: RetainedRoute = RetainedRoute {
        predecessor_index: 20,
        guard_evaluated: true,
        assignment_executed: true,
    };
    let mutations: &[OverflowMutation] = &[
        (INACTIVE, |s| s.transition_count = usize::MAX),
        (INACTIVE, |s| s.predecessor_route_counts[0] = usize::MAX),
        (INACTIVE, |s| s.inactive_transition_count = usize::MAX),
        (FALSE, |s| s.predecessor_guard_false_fallthrough_count = usize::MAX),
        (FALSE, |s| {
            s.predecessor_guard_false_fallthrough_route_counts[20] = usize::MAX
        }),
        (ASSIGN, |s| {
            s.cooling_latent_output_maximum_capacity_assignment_count = usize::MAX
        }),
        (ASSIGN, |s| {
            s.cooling_latent_output_maximum_capacity_assignment_route_counts[20] = usize::MAX
        }),
        (ASSIGN, |s| s.source_site_execution_count = usize::MAX),
        (ASSIGN, |s| s.cp404_supply_humidity_ratio_state_owner_count = usize::MAX),
        (ASSIGN, |s| s.unchanged_supply_humidity_ratio_preservation_count = usize::MAX),
        (ASSIGN, |s| s.cp404_supply_enthalpy_state_owner_count = usize::MAX),
        (ASSIGN, |s| s.unchanged_supply_enthalpy_preservation_count = usize::MAX),
        (ASSIGN, |s| s.cp404_supply_temperature_state_owner_count = usize::MAX),
        (ASSIGN, |s| s.unchanged_supply_temperature_preservation_count = usize::MAX),
        (ASSIGN, |s| {
            s.cp404_retained_maximum_total_cooling_capacity_owned_read_count = usize::MAX
        }),
        (ASSIGN, |s| s.maximum_total_cooling_capacity_read_count = usize::MAX),
        (ASSIGN, |s| s.cooling_latent_output_assignment_write_count = usize::MAX),
    ];
    for (route, mutate) in mutations {
        let mut state = State::new(ep_model::IdealLoadsAirSystemId(405));
        mutate(&mut state);
        assert!(!test_next_transition_fits(&state, *route));
    }
}

#[test]
fn new_state_is_exactly_zeroed_and_consistent() {
    let state = State::new(ep_model::IdealLoadsAirSystemId(405));
    assert!(test_counts_are_exact(&state));
    assert_eq!(state.predecessor_route_counts, [0; 30]);
    assert_eq!(state.predecessor_guard_false_fallthrough_route_counts, [0; 30]);
    assert_eq!(
        state.cooling_latent_output_maximum_capacity_assignment_route_counts,
        [0; 30],
    );
    assert!(state.latest.is_none());
}
