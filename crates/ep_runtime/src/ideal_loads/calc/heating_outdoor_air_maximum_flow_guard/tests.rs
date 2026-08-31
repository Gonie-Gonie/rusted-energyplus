//! CP435 boundary, exhaustive guard, forgery, overflow, and bounded-path tests.

mod schema_prefix;

use ep_model::IdealLoadsLimit;

use super::PurchasedAirCalcHeatingOutdoorAirMaximumFlowGuardRuntimeState as State;
use super::transition::{
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowGuardRetainedRoute as Route,
    advance_heating_outdoor_air_maximum_flow_guard_state as advance,
    advance_heating_outdoor_air_maximum_flow_guard_state_with_validated_route as advance_validated,
    heating_outdoor_air_maximum_flow_guard_route_from_committed_predecessor as successor_route,
};
use crate::ideal_loads::calc::{
    PurchasedAirCalcHeatingOperatingModeDeadbandAssignmentRetainedRoute as PredecessorRoute,
    cp434_all_snapshots_for_successor_tests,
    heating_operating_mode_deadband_assignment_snapshot_route as predecessor_route,
};
use crate::ideal_loads::PurchasedAirCalcHeatingOperatingModeDeadbandAssignmentSnapshot as Predecessor;

#[test]
fn cp435_boundary_maps_compound_guard_2361_2362_and_excludes_2363() {
    assert_eq!(
        super::PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_GUARD_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2361-2362",
    );
    assert_eq!(
        super::PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_GUARD_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2363",
    );
    assert_eq!(
        super::PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_GUARD_SOURCE_ORDER,
        &[
            "compare-heating-limit-to-flow-rate",
            "compare-heating-limit-to-flow-rate-and-capacity-after-short-circuit",
            "read-outdoor-air-mass-flow-after-limit-short-circuit",
            "read-maximum-heating-air-mass-flow-after-limit-short-circuit",
            "compare-strict-outdoor-air-above-maximum-heating-flow",
            "enter-maximum-heating-flow-body-if-satisfied",
        ],
    );
}

#[test]
fn exhaustive_61_predecessors_refine_to_64_successors_with_exact_guard_partition_and_accounting()
{
    let cases = route_cases();
    assert_eq!(cases.len(), 64);
    assert_eq!(
        cases
            .iter()
            .filter(|(predecessor, _, _, _)| predecessor.heating_or_no_load_case_entered)
            .count(),
        6,
    );
    let mut state = State::new(cases[0].0.system);
    let mut public = 0usize;
    let mut private = 0usize;
    let mut active_indices = Vec::new();
    for (predecessor, limit, outdoor, maximum) in cases {
        let predecessor_route = predecessor_route_for(predecessor);
        let route = route_for(predecessor, limit, outdoor, maximum);
        let snapshot = advance_validated(
            &mut state,
            predecessor,
            predecessor_route,
            limit,
            outdoor,
            maximum,
            route,
        )
        .expect("CP435 route");
        assert!(super::heating_outdoor_air_maximum_flow_guard_snapshot_is_exact(snapshot));
        if route.guard_evaluated {
            active_indices.push(route.logical_index);
        }
        if is_public_logical_index(route.logical_index)
            && !route.predecessor_single_cool_blocked
            && !route.body_entered
        {
            public += 1;
        } else {
            private += 1;
        }
    }
    assert_eq!((public, private), (20, 44));
    assert_eq!(active_indices, vec![1; 6]);
    assert_eq!(state.transition_count, 64);
    assert_eq!(state.inactive_transition_count, 58);
    assert_eq!(state.heating_outdoor_air_maximum_flow_guard_evaluation_count, 6);
    assert_eq!(state.heating_limit_flow_rate_comparison_count, 6);
    assert_eq!(state.heating_limit_flow_rate_match_count, 0);
    assert_eq!(state.heating_limit_flow_rate_and_capacity_comparison_count, 6);
    assert_eq!(state.heating_limit_flow_rate_and_capacity_match_count, 6);
    assert_eq!(state.heating_flow_limit_selector_rejection_count, 0);
    assert_eq!(state.cp311_same_call_outdoor_air_mass_flow_rate_bit_corroboration_count, 6);
    assert_eq!(state.outdoor_air_mass_flow_rate_read_after_heating_limit_short_circuit_count, 6);
    assert_eq!(state.maximum_heating_air_mass_flow_rate_read_after_heating_limit_short_circuit_count, 6);
    assert_eq!(state.outdoor_air_mass_flow_rate_maximum_heating_air_mass_flow_rate_comparison_count, 6);
    assert_eq!(state.outdoor_air_mass_flow_rate_strictly_greater_than_maximum_heating_air_mass_flow_rate_count, 3);
    assert_eq!(state.maximum_heating_flow_body_entry_count, 3);
    assert_eq!(state.heating_outdoor_air_maximum_flow_guard_false_fallthrough_count, 3);
    assert_eq!(state.maximum_heating_flow_body_entry_route_counts[1], 3);
    assert_eq!(state.heating_outdoor_air_maximum_flow_guard_false_fallthrough_route_counts[1], 3);
    assert_eq!(state.source_site_execution_count, 33);
    assert_eq!(state.cp434_supply_humidity_ratio_state_owner_count, 37);
    assert_eq!(state.cp434_supply_enthalpy_state_owner_count, 42);
    assert_eq!(state.cp434_supply_temperature_state_owner_count, 57);
    assert!(super::release::state_counts_are_consistent_for_test(&state));
}

#[test]
fn strict_comparison_and_selector_short_circuits_have_exact_local_shape() {
    let active = active_predecessors();
    assert_eq!(active.len(), 3);
    for predecessor in active {
        let rejected = characterize(predecessor, IdealLoadsLimit::NoLimit, 7.0, 1.0);
        assert_eq!(rejected.heating_flow_limit_active, Some(false));
        assert!(rejected.heating_flow_limit_selector_rejected);
        assert!(!rejected.outdoor_air_mass_flow_rate_read_after_heating_limit_short_circuit);
        assert!(rejected.outdoor_air_mass_flow_rate_for_heating_maximum_flow_guard_kg_per_s.is_none());

        let equal = characterize(
            predecessor,
            IdealLoadsLimit::LimitFlowRateAndCapacity,
            1.0,
            1.0,
        );
        assert_eq!(equal.outdoor_air_mass_flow_rate_strictly_greater_than_maximum_heating_air_mass_flow_rate, Some(false));
        assert!(!equal.maximum_heating_flow_body_entered);
        assert!(equal.heating_outdoor_air_maximum_flow_guard_false_fallthrough);

        let above = characterize(
            predecessor,
            IdealLoadsLimit::LimitFlowRateAndCapacity,
            f64::from_bits(1.0f64.to_bits() + 1),
            1.0,
        );
        assert_eq!(above.outdoor_air_mass_flow_rate_strictly_greater_than_maximum_heating_air_mass_flow_rate, Some(true));
        assert!(above.maximum_heating_flow_body_entered);
        assert!(!above.heating_outdoor_air_maximum_flow_guard_false_fallthrough);
    }
}

#[test]
fn route_forgery_and_overflow_fail_without_mutation() {
    let predecessor = active_predecessors()[0];
    let predecessor_route = predecessor_route_for(predecessor);
    let canonical = route_for(
        predecessor,
        IdealLoadsLimit::LimitFlowRateAndCapacity,
        0.0,
        0.0,
    );
    for component in 0..23 {
        let mut forged = canonical;
        flip_route_component(&mut forged, component);
        let mut state = State::new(predecessor.system);
        let before = state.clone();
        assert!(
            advance_validated(
                &mut state,
                predecessor,
                predecessor_route,
                IdealLoadsLimit::LimitFlowRateAndCapacity,
                0.0,
                0.0,
                forged,
            )
            .is_none()
        );
        assert_eq!(state, before);
    }
    let mut state = State::new(predecessor.system);
    state.transition_count = usize::MAX;
    let before = state.clone();
    assert!(
        advance(
            &mut state,
            predecessor,
            IdealLoadsLimit::LimitFlowRateAndCapacity,
            0.0,
            0.0,
        )
        .is_none()
    );
    assert_eq!(state, before);
}

#[test]
fn cp435_public_release_seals_cp311_cache_and_rejects_body_before_mutation() {
    let release = include_str!("release.rs");
    let read = release.find("minimum_oa_is_sealed_same_call_no_oa").unwrap();
    let reject = release.find("if route.body_entered").unwrap();
    let clone = release
        .find("let mut next_state = unit.calc_heating_outdoor_air_maximum_flow_guard.clone()")
        .unwrap();
    assert!(read < reject && reject < clone);
    assert!(release.contains("outdoor.to_bits() != 0.0f64.to_bits()"));
    assert!(release.contains("maximum_heating_cache_is_bit_exact"));
    assert!(release.contains("ExactReleaseReductionViolated"));
}

#[test]
fn cp435_topology_is_11_files_one_nested_test_and_every_file_is_bounded() {
    let files = [
        include_str!("../heating_outdoor_air_maximum_flow_guard.rs"),
        include_str!("release.rs"),
        include_str!("release/error.rs"),
        include_str!("release/prefix.rs"),
        include_str!("release/runtime_validation.rs"),
        include_str!("release/snapshot_validation.rs"),
        include_str!("state.rs"),
        include_str!("tests.rs"),
        include_str!("tests/schema_prefix.rs"),
        include_str!("transition.rs"),
        include_str!("transition/accounting.rs"),
        include_str!("transition/snapshot.rs"),
    ];
    assert_eq!(files.len() - 1, 11);
    assert!(files.into_iter().all(|source| source.lines().count() <= 500));
}

#[allow(dead_code)]
pub(in crate::ideal_loads::calc) fn cp435_all_snapshots_for_successor_tests(
) -> Vec<super::PurchasedAirCalcHeatingOutdoorAirMaximumFlowGuardSnapshot> {
    let cases = route_cases();
    let mut state = State::new(cases[0].0.system);
    cases
        .into_iter()
        .map(|(predecessor, limit, outdoor, maximum)| {
            advance(&mut state, predecessor, limit, outdoor, maximum).expect("CP435 snapshot")
        })
        .collect()
}

fn route_cases() -> Vec<(Predecessor, IdealLoadsLimit, f64, f64)> {
    let mut cases = Vec::new();
    for predecessor in cp434_all_snapshots_for_successor_tests() {
        if predecessor.heating_or_no_load_case_entered {
            cases.push((predecessor, IdealLoadsLimit::LimitFlowRateAndCapacity, 0.0, 0.0));
            cases.push((predecessor, IdealLoadsLimit::LimitFlowRateAndCapacity, 1.0, 0.0));
        } else {
            cases.push((predecessor, IdealLoadsLimit::LimitFlowRateAndCapacity, 0.0, 0.0));
        }
    }
    cases
}

fn active_predecessors() -> Vec<Predecessor> {
    cp434_all_snapshots_for_successor_tests()
        .into_iter()
        .filter(|snapshot| snapshot.heating_or_no_load_case_entered)
        .collect()
}

fn characterize(
    predecessor: Predecessor,
    limit: IdealLoadsLimit,
    outdoor: f64,
    maximum: f64,
) -> super::PurchasedAirCalcHeatingOutdoorAirMaximumFlowGuardSnapshot {
    super::private_heating_outdoor_air_maximum_flow_guard_characterization(
        predecessor,
        limit,
        outdoor,
        maximum,
    )
    .expect("CP435 characterization")
}

fn route_for(
    predecessor: Predecessor,
    limit: IdealLoadsLimit,
    outdoor: f64,
    maximum: f64,
) -> Route {
    successor_route(
        predecessor,
        predecessor_route_for(predecessor),
        limit,
        outdoor,
        maximum,
    )
    .expect("CP435 route")
}

fn predecessor_route_for(predecessor: Predecessor) -> PredecessorRoute {
    predecessor_route(predecessor).expect("CP434 route")
}

fn flip_route_component(route: &mut Route, component: usize) {
    match component {
        0 => route.logical_index = (route.logical_index + 1) % 36,
        1 => route.predecessor_active ^= true,
        2 => route.predecessor_assignment_executed ^= true,
        3 => route.predecessor_entered ^= true,
        4 => route.predecessor_total_output_assignment_executed ^= true,
        5 => route.predecessor_heating_or_no_load_case_entered ^= true,
        6 => route.predecessor_heating_mode_guard_evaluated ^= true,
        7 => route.predecessor_sensible_comparison_satisfied ^= true,
        8 => route.predecessor_single_cool_blocked ^= true,
        9 => route.predecessor_heating_operating_mode_body_entered ^= true,
        10 => route.predecessor_heating_mode_guard_false_fallthrough ^= true,
        11 => route.predecessor_heating_operating_mode_heat_assignment_executed ^= true,
        12 => route.predecessor_heating_mode_guard_else_branch_entered ^= true,
        13 => route.predecessor_heating_operating_mode_deadband_assignment_executed ^= true,
        14 => route.guard_evaluated ^= true,
        15 => route.heating_limit_flow_rate_comparison_satisfied ^= true,
        16 => route.heating_limit_flow_rate_and_capacity_comparison_evaluated ^= true,
        17 => route.heating_limit_flow_rate_and_capacity_comparison_satisfied ^= true,
        18 => route.heating_flow_limit_active ^= true,
        19 => route.heating_flow_limit_selector_rejected ^= true,
        20 => route.strict_mass_flow_comparison_evaluated ^= true,
        21 => route.body_entered ^= true,
        _ => route.false_fallthrough ^= true,
    }
}

fn is_public_logical_index(index: usize) -> bool {
    matches!(index, 0..=8 | 20 | 21 | 26 | 27)
}
