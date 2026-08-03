//! CP406 boundary, route, accounting, corruption, and overflow tests.

use super::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_CAPACITY_GUARD_ELSE_BRANCH_ENTRY_FIRST_EXCLUDED_SOURCE as EXCLUDED,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_CAPACITY_GUARD_ELSE_BRANCH_ENTRY_SOURCE as SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_CAPACITY_GUARD_ELSE_BRANCH_ENTRY_SOURCE_ORDER as ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputCapacityGuardElseBranchEntryRuntimeState as State,
};
use super::release::test_counts_are_exact;
use super::transition::{
    RetainedRoute, logical_route_index, predecessor_index_is_active,
    predecessor_index_is_public, test_increment_counts, test_next_transition_fits,
};

#[test]
fn cp406_boundary_and_single_else_entry_site_are_exact() {
    assert_eq!(SOURCE, "EnergyPlus 26.1 PurchasedAirManager.cc:2301");
    assert_eq!(EXCLUDED, "EnergyPlus 26.1 PurchasedAirManager.cc:2302");
    assert_eq!(
        ORDER,
        &["enter-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-latent-output-capacity-guard-else-branch-after-guard-false-fallthrough"],
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
fn thirty_six_routes_and_else_entry_partition_are_exact() {
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
            .filter(|route| route.guard_evaluated && !route.assignment_executed)
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
                route.guard_evaluated
                    && !route.assignment_executed
                    && predecessor_index_is_public(route.predecessor_index)
            })
            .map(logical_route_index)
            .collect::<Vec<_>>(),
        [20, 26],
    );
}

fn exhaustive_state() -> State {
    let mut state = State::new(ep_model::IdealLoadsAirSystemId(406));
    for route in all_routes() {
        assert!(test_next_transition_fits(&state, route));
        test_increment_counts(&mut state, route);
    }
    state
}

#[test]
fn exhaustive_accounting_is_36_30_6_6() {
    let state = exhaustive_state();
    assert!(test_counts_are_exact(&state));
    assert_eq!(state.transition_count, 36);
    assert_eq!(state.inactive_transition_count, 30);
    assert_eq!(state.predecessor_guard_false_fallthrough_count, 6);
    assert_eq!(state.predecessor_maximum_capacity_assignment_count, 6);
    assert_eq!(
        state.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry_count,
        6,
    );
    assert_eq!(state.source_site_execution_count, 6);
    assert_eq!(
        state.else_branch_entry_route_counts,
        state.predecessor_guard_false_fallthrough_route_counts,
    );
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
        |s| s.predecessor_maximum_capacity_assignment_count += 1,
        |s| {
            s.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry_count += 1
        },
        |s| s.predecessor_route_counts[20] += 1,
        |s| s.predecessor_guard_false_fallthrough_route_counts[20] += 1,
        |s| s.predecessor_maximum_capacity_assignment_route_counts[20] += 1,
        |s| s.else_branch_entry_route_counts[20] += 1,
        |s| s.source_site_execution_count += 1,
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
    const ELSE_ENTRY: RetainedRoute = RetainedRoute {
        predecessor_index: 20,
        guard_evaluated: true,
        assignment_executed: false,
    };
    const ASSIGNMENT: RetainedRoute = RetainedRoute {
        predecessor_index: 20,
        guard_evaluated: true,
        assignment_executed: true,
    };
    let mutations: &[OverflowMutation] = &[
        (INACTIVE, |s| s.transition_count = usize::MAX),
        (INACTIVE, |s| s.predecessor_route_counts[0] = usize::MAX),
        (INACTIVE, |s| s.inactive_transition_count = usize::MAX),
        (ELSE_ENTRY, |s| s.predecessor_guard_false_fallthrough_count = usize::MAX),
        (ELSE_ENTRY, |s| {
            s.predecessor_guard_false_fallthrough_route_counts[20] = usize::MAX
        }),
        (ELSE_ENTRY, |s| {
            s.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry_count = usize::MAX
        }),
        (ELSE_ENTRY, |s| s.else_branch_entry_route_counts[20] = usize::MAX),
        (ELSE_ENTRY, |s| s.source_site_execution_count = usize::MAX),
        (ASSIGNMENT, |s| s.inactive_transition_count = usize::MAX),
        (ASSIGNMENT, |s| {
            s.predecessor_maximum_capacity_assignment_count = usize::MAX
        }),
        (ASSIGNMENT, |s| {
            s.predecessor_maximum_capacity_assignment_route_counts[20] = usize::MAX
        }),
    ];
    for (route, mutate) in mutations {
        let mut state = State::new(ep_model::IdealLoadsAirSystemId(406));
        mutate(&mut state);
        let before = state.clone();
        assert!(!test_next_transition_fits(&state, *route));
        assert_eq!(state, before);
    }
}

#[test]
fn new_state_is_exactly_zeroed_and_consistent() {
    let state = State::new(ep_model::IdealLoadsAirSystemId(406));
    assert!(test_counts_are_exact(&state));
    assert_eq!(state.predecessor_route_counts, [0; 30]);
    assert_eq!(state.predecessor_guard_false_fallthrough_route_counts, [0; 30]);
    assert_eq!(
        state.predecessor_maximum_capacity_assignment_route_counts,
        [0; 30],
    );
    assert_eq!(state.else_branch_entry_route_counts, [0; 30]);
    assert!(state.latest.is_none());
}
