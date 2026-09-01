//! CP436 boundary, exhaustive assignment, density, forgery, and topology tests.

mod committed_seal;
mod schema_prefix;

use ep_model::IdealLoadsLimit;

use super::PurchasedAirCalcHeatingOutdoorAirMaximumFlowBodyVolumeFlowAssignmentRuntimeState as State;
use super::transition::{
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowBodyVolumeFlowAssignmentRetainedRoute as Route,
    advance_heating_outdoor_air_maximum_flow_body_volume_flow_assignment_state as advance,
    advance_heating_outdoor_air_maximum_flow_body_volume_flow_assignment_state_with_validated_route as advance_validated,
    heating_outdoor_air_maximum_flow_body_volume_flow_assignment_route_from_committed_predecessor as successor_route,
};
use crate::ideal_loads::calc::{
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowGuardRetainedRoute as PredecessorRoute,
    cp435_all_snapshots_for_successor_tests,
    heating_outdoor_air_maximum_flow_guard_snapshot_route as predecessor_route,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingMixedAirCallSnapshot as Cp329Snapshot,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowGuardSnapshot as Predecessor,
    PurchasedAirUnitRuntimeState,
};

#[test]
fn cp436_boundary_maps_volume_flow_assignment_2363_and_excludes_2364() {
    assert_eq!(
        super::PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_BODY_VOLUME_FLOW_ASSIGNMENT_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2363",
    );
    assert_eq!(
        super::PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_BODY_VOLUME_FLOW_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2364",
    );
    assert_eq!(
        super::PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_BODY_VOLUME_FLOW_ASSIGNMENT_SOURCE_ORDER,
        &[
            "read-cp435-retained-outdoor-air-mass-flow-for-outdoor-air-volume-flow-division",
            "read-environment-standard-air-density-for-outdoor-air-volume-flow-division",
            "calculate-outdoor-air-mass-flow-divided-by-standard-air-density",
            "assign-local-outdoor-air-volume-flow-rate",
        ],
    );
}

#[test]
fn exhaustive_64_routes_preserve_20_44_visibility_and_assign_only_three_private_bodies() {
    let predecessors = cp435_all_snapshots_for_successor_tests();
    assert_eq!(predecessors.len(), 64);
    let mut state = State::new(predecessors[0].system);
    let mut expected = [[0usize; 36]; 4];
    let mut public = 0usize;
    let mut public_assignments = 0usize;
    let mut private_assignments = 0usize;
    for predecessor in predecessors {
        let predecessor_route = predecessor_route_for(predecessor);
        let route = route_for(predecessor);
        let density = 2.0;
        let snapshot = advance_validated(
            &mut state,
            predecessor,
            predecessor_route,
            density,
            route,
        )
        .expect("CP436");
        expected[0][route.logical_index] += 1;
        expected[1][route.logical_index] +=
            usize::from(route.predecessor_guard_false_fallthrough);
        expected[2][route.logical_index] += usize::from(route.predecessor_guard_body_entered);
        expected[3][route.logical_index] += usize::from(route.assignment_executed);

        let is_public = is_public_logical_index(route.logical_index)
            && !predecessor_route.predecessor_single_cool_blocked
            && !route.predecessor_guard_body_entered;
        public += usize::from(is_public);
        public_assignments += usize::from(is_public && route.assignment_executed);
        private_assignments += usize::from(!is_public && route.assignment_executed);

        assert_eq!(route.assignment_executed, route.predecessor_guard_body_entered);
        assert_eq!(
            snapshot.heating_outdoor_air_maximum_flow_body_volume_flow_assignment_executed,
            route.assignment_executed,
        );
        if route.assignment_executed {
            let numerator = predecessor
                .outdoor_air_mass_flow_rate_for_heating_maximum_flow_guard_kg_per_s
                .expect("sealed numerator");
            let quotient = numerator / density;
            assert_eq!(
                snapshot
                    .standard_air_density_for_outdoor_air_volume_flow_division_kg_per_m3
                    .expect("density")
                    .to_bits(),
                density.to_bits(),
            );
            assert_eq!(
                snapshot
                    .assigned_outdoor_air_volume_flow_rate_m3_per_s
                    .expect("assigned volume flow")
                    .to_bits(),
                quotient.to_bits(),
            );
        } else {
            assert!(!snapshot.cp435_retained_outdoor_air_mass_flow_rate_owned_read);
            assert!(
                snapshot
                    .standard_air_density_for_outdoor_air_volume_flow_division_kg_per_m3
                    .is_none()
            );
            assert!(snapshot.assigned_outdoor_air_volume_flow_rate_m3_per_s.is_none());
        }
        assert_bits(
            snapshot.resulting_supply_humidity_ratio,
            predecessor.resulting_supply_humidity_ratio,
        );
        assert_bits(
            snapshot.resulting_supply_enthalpy_j_per_kg,
            predecessor.resulting_supply_enthalpy_j_per_kg,
        );
        assert_bits(
            snapshot.resulting_supply_temperature_c,
            predecessor.resulting_supply_temperature_c,
        );
        assert!(
            super::heating_outdoor_air_maximum_flow_body_volume_flow_assignment_snapshot_is_exact(
                snapshot,
            )
        );
    }
    assert_eq!((public, 64 - public), (20, 44));
    assert_eq!((public_assignments, private_assignments), (0, 3));
    assert_eq!(state.transition_count, 64);
    assert_eq!(state.inactive_transition_count, 61);
    assert_eq!(state.outdoor_air_volume_flow_assignment_count, 3);
    assert_eq!(state.source_site_execution_count, 12);
    assert_eq!(state.predecessor_route_counts, expected[0]);
    assert_eq!(state.predecessor_guard_false_fallthrough_route_counts, expected[1]);
    assert_eq!(state.predecessor_guard_body_entry_route_counts, expected[2]);
    assert_eq!(
        state.heating_outdoor_air_volume_flow_assignment_route_counts,
        expected[3],
    );
    assert_eq!(expected[0][1], 6);
    assert_eq!(expected[1][1], 3);
    assert_eq!(expected[2][1], 3);
    assert_eq!(expected[3][1], 3);
    assert_eq!(state.cp435_supply_humidity_ratio_state_owner_count, 37);
    assert_eq!(state.unchanged_supply_humidity_ratio_preservation_count, 37);
    assert_eq!(state.cp435_supply_enthalpy_state_owner_count, 42);
    assert_eq!(state.unchanged_supply_enthalpy_preservation_count, 42);
    assert_eq!(state.cp435_supply_temperature_state_owner_count, 57);
    assert_eq!(state.unchanged_supply_temperature_preservation_count, 57);
    assert_eq!(state.cp435_outdoor_air_mass_flow_rate_owned_read_count, 3);
    assert_eq!(state.begin_environment_standard_air_density_owner_count, 3);
    assert_eq!(state.outdoor_air_mass_flow_rate_standard_air_density_division_count, 3);
    assert_eq!(state.local_outdoor_air_volume_flow_rate_assignment_write_count, 3);
    assert!(super::release::state_counts_are_consistent_for_test(&state));
}

#[test]
fn active_routes_reject_every_nonpositive_or_nonfinite_density_transactionally() {
    let predecessor = active_predecessor();
    let predecessor_route = predecessor_route_for(predecessor);
    let route = route_for(predecessor);
    assert!(route.assignment_executed);
    for density in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, 0.0, -0.0, -1.0] {
        let mut state = State::new(predecessor.system);
        let before = state.clone();
        assert!(
            advance_validated(&mut state, predecessor, predecessor_route, density, route).is_none(),
            "density {density:?}",
        );
        assert_eq!(state, before);
    }
}

#[test]
fn inactive_routes_do_not_read_or_validate_density() {
    let predecessor = cp435_all_snapshots_for_successor_tests()
        .into_iter()
        .find(|snapshot| !route_for(*snapshot).assignment_executed)
        .expect("inactive predecessor");
    let snapshot = advance(&mut State::new(predecessor.system), predecessor, f64::NAN)
        .expect("inactive CP436");
    assert!(!snapshot.begin_environment_standard_air_density_owned_read);
    assert!(
        snapshot
            .standard_air_density_for_outdoor_air_volume_flow_division_kg_per_m3
            .is_none()
    );
    assert!(snapshot.calculated_outdoor_air_volume_flow_rate_m3_per_s.is_none());
}

#[test]
fn every_cp436_route_component_forgery_and_overflow_is_transactional() {
    let predecessor = active_predecessor();
    let predecessor_route = predecessor_route_for(predecessor);
    let exact = route_for(predecessor);
    for component in 0..4 {
        let mut forged = exact;
        match component {
            0 => forged.logical_index = (forged.logical_index + 1) % 36,
            1 => forged.predecessor_guard_false_fallthrough ^= true,
            2 => forged.predecessor_guard_body_entered ^= true,
            _ => forged.assignment_executed ^= true,
        }
        let mut state = State::new(predecessor.system);
        let before = state.clone();
        assert!(
            advance_validated(&mut state, predecessor, predecessor_route, 2.0, forged).is_none()
        );
        assert_eq!(state, before);
    }
    let mut state = State::new(predecessor.system);
    state.transition_count = usize::MAX;
    let before = state.clone();
    assert!(advance(&mut state, predecessor, 2.0).is_none());
    assert_eq!(state, before);
}

#[test]
fn cp436_new_state_has_four_zeroed_width_36_arrays() {
    let state = State::new(cp435_all_snapshots_for_successor_tests()[0].system);
    let arrays = [
        state.predecessor_route_counts,
        state.predecessor_guard_false_fallthrough_route_counts,
        state.predecessor_guard_body_entry_route_counts,
        state.heating_outdoor_air_volume_flow_assignment_route_counts,
    ];
    assert_eq!(arrays.len(), 4);
    assert!(arrays.into_iter().flatten().all(|count| count == 0));
    assert!(state.latest.is_none());
}

#[test]
fn public_release_seals_begin_environment_density_before_rejecting_private_body() {
    let release = include_str!("release.rs");
    let density = release
        .find("begin_environment_standard_air_density_is_bit_exact(unit, system)")
        .expect("density seal");
    let reject = release
        .find("return Err(Error::ExactReleaseReductionViolated")
        .expect("private body rejection");
    let clone = release
        .find("let mut next_state = unit")
        .expect("transactional clone");
    assert!(density < reject && reject < clone);
    assert!(release.contains("density.is_finite()"));
    assert!(release.contains("density > 0.0"));
    assert!(release.contains("unit.maximum_heating_air_mass_flow_rate_kg_per_s.to_bits()"));
    assert!(release.contains("expected_maximum_mass_flow.to_bits()"));
    let transition = include_str!("transition.rs");
    assert!(transition.contains("!standard_air_density_kg_per_m3.is_finite()"));
    assert!(transition.contains("standard_air_density_kg_per_m3 <= 0.0"));
}

#[test]
fn cp436_subtree_is_twelve_files_and_every_file_is_bounded() {
    let files = [
        include_str!("../heating_outdoor_air_maximum_flow_body_volume_flow_assignment.rs"),
        include_str!("release.rs"),
        include_str!("state.rs"),
        include_str!("tests.rs"),
        include_str!("transition.rs"),
        include_str!("release/error.rs"),
        include_str!("release/prefix.rs"),
        include_str!("release/runtime_validation.rs"),
        include_str!("release/snapshot_validation.rs"),
        include_str!("tests/schema_prefix.rs"),
        include_str!("transition/accounting.rs"),
        include_str!("transition/snapshot.rs"),
    ];
    assert_eq!(files.len(), 12);
    assert!(files.into_iter().all(|source| source.lines().count() <= 500));
}

#[allow(dead_code)]
pub(in crate::ideal_loads::calc) fn cp436_all_snapshots_for_successor_tests(
) -> Vec<super::PurchasedAirCalcHeatingOutdoorAirMaximumFlowBodyVolumeFlowAssignmentSnapshot> {
    let predecessors = cp435_all_snapshots_for_successor_tests();
    let mut state = State::new(predecessors[0].system);
    predecessors
        .into_iter()
        .map(|predecessor| advance(&mut state, predecessor, 2.0).expect("CP436 snapshot"))
        .collect()
}

#[allow(dead_code)]
pub(in crate::ideal_loads::calc) fn cp436_fixture_unit_for_successor_tests() -> (
    PurchasedAirUnitRuntimeState,
    super::PurchasedAirCalcHeatingOutdoorAirMaximumFlowBodyVolumeFlowAssignmentSnapshot,
    Route,
    Option<Cp329Snapshot>,
) {
    let (mut unit, predecessor, predecessor_route, owner) =
        crate::ideal_loads::calc::cp434_fixture_unit_for_successor_tests();
    let cp435_route = crate::ideal_loads::calc::heating_outdoor_air_maximum_flow_guard_route_from_committed_predecessor(
        predecessor,
        predecessor_route,
        IdealLoadsLimit::LimitFlowRateAndCapacity,
        1.0,
        0.0,
    )
    .expect("CP435 route");
    let mut cp435_state =
        crate::ideal_loads::PurchasedAirCalcHeatingOutdoorAirMaximumFlowGuardRuntimeState::new(
            predecessor.system,
        );
    let cp435 = crate::ideal_loads::calc::advance_heating_outdoor_air_maximum_flow_guard_state_with_validated_route(
        &mut cp435_state,
        predecessor,
        predecessor_route,
        IdealLoadsLimit::LimitFlowRateAndCapacity,
        1.0,
        0.0,
        cp435_route,
    )
    .expect("CP435");
    unit.calc_heating_outdoor_air_maximum_flow_guard = cp435_state;
    let route = route_for(cp435);
    let mut state = State::new(cp435.system);
    let snapshot = advance_validated(
        &mut state,
        cp435,
        cp435_route,
        2.0,
        route,
    )
    .expect("CP436");
    unit.calc_heating_outdoor_air_maximum_flow_body_volume_flow_assignment = state;
    (unit, snapshot, route, owner)
}

fn active_predecessor() -> Predecessor {
    cp435_all_snapshots_for_successor_tests()
        .into_iter()
        .find(|snapshot| route_for(*snapshot).assignment_executed)
        .expect("CP435 body predecessor")
}

fn route_for(predecessor: Predecessor) -> Route {
    successor_route(predecessor, predecessor_route_for(predecessor)).expect("CP436 route")
}

fn predecessor_route_for(predecessor: Predecessor) -> PredecessorRoute {
    predecessor_route(predecessor).expect("CP435 route")
}

fn is_public_logical_index(index: usize) -> bool {
    matches!(index, 0..=8 | 20 | 21 | 26 | 27)
}

fn assert_bits(left: Option<f64>, right: Option<f64>) {
    assert_eq!(left.map(f64::to_bits), right.map(f64::to_bits));
}
