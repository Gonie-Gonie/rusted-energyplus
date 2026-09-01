//! Focused CP424 and CP329 committed-capability acceptance and forgery tests.

use super::*;
use crate::ideal_loads::calc::{
    cooling_mixed_air_call_committed_latest_mixed_air_enthalpy as committed_cp329_enthalpy,
    cooling_supply_mass_flow_positive_guard_else_branch_entry_committed_latest_route as committed,
};
use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputMaximumCapacityAssignmentSnapshot as Cp422Predecessor;

#[test]
fn cp424_committed_route_seal_accepts_entry_inactive_guard_false_and_assignment_routes() {
    for predecessor in representative_predecessors() {
        let (unit, snapshot, route) = cp424_fixture_unit_for_successor_tests(predecessor);
        assert_eq!(committed(&unit, snapshot), Some(route));
    }
}

#[test]
fn cp424_committed_route_seal_rejects_route_count_ordinal_latest_witness_and_identity_forgery() {
    for predecessor in representative_predecessors() {
        let (unit, snapshot, route) = cp424_fixture_unit_for_successor_tests(predecessor);
        let forgeries: [Cp424Forgery; 15] = [
            |forged, _, route| {
                forged
                    .calc_cooling_supply_mass_flow_positive_guard_else_branch_entry
                    .latest_route
                    .as_mut()
                    .expect("route")
                    .logical_index = (route.logical_index + 1) % 36;
            },
            |forged, _, _| {
                forged
                    .calc_cooling_supply_mass_flow_positive_guard_else_branch_entry
                    .latest_route
                    .as_mut()
                    .expect("route")
                    .active ^= true;
            },
            |forged, _, _| {
                forged
                    .calc_cooling_supply_mass_flow_positive_guard_else_branch_entry
                    .latest_route
                    .as_mut()
                    .expect("route")
                    .assignment_executed ^= true;
            },
            |forged, _, _| {
                forged
                    .calc_cooling_supply_mass_flow_positive_guard_else_branch_entry
                    .latest_route
                    .as_mut()
                    .expect("route")
                    .entered ^= true;
            },
            |forged, _, _| {
                forged
                    .calc_cooling_supply_mass_flow_positive_guard_else_branch_entry
                    .transition_count += 1;
            },
            |forged, _, _| {
                forged
                    .calc_cooling_supply_mass_flow_positive_guard_else_branch_entry
                    .latest_transition_ordinal = Some(0);
            },
            |forged, _, route| {
                forged
                    .calc_cooling_supply_mass_flow_positive_guard_else_branch_entry
                    .predecessor_route_counts[route.logical_index] += 1;
            },
            |forged, _, route| {
                forged
                    .calc_cooling_supply_mass_flow_positive_guard_else_branch_entry
                    .positive_supply_mass_flow_guard_else_branch_entry_route_counts
                    [route.logical_index] += 1;
            },
            |forged, _, _| {
                forged
                    .calc_cooling_supply_mass_flow_positive_guard_else_branch_entry
                    .source_site_execution_count += 1;
            },
            |_, witness, _| {
                witness.system =
                    ep_model::IdealLoadsAirSystemId(witness.system.0.wrapping_add(1));
            },
            |_, witness, _| {
                witness.controlled_zone =
                    ep_model::ZoneId(witness.controlled_zone.0.wrapping_add(1));
            },
            |_, witness, _| {
                witness.parent_call_ordinal = witness.parent_call_ordinal.wrapping_add(1);
            },
            |forged, witness, _| {
                witness.system =
                    ep_model::IdealLoadsAirSystemId(witness.system.0.wrapping_add(1));
                forged
                    .calc_cooling_supply_mass_flow_positive_guard_else_branch_entry
                    .latest
                    .as_mut()
                    .expect("latest")
                    .system = witness.system;
            },
            |forged, witness, _| {
                witness.controlled_zone =
                    ep_model::ZoneId(witness.controlled_zone.0.wrapping_add(1));
                forged
                    .calc_cooling_supply_mass_flow_positive_guard_else_branch_entry
                    .latest
                    .as_mut()
                    .expect("latest")
                    .controlled_zone = witness.controlled_zone;
            },
            |forged, witness, _| {
                witness.parent_call_ordinal = witness.parent_call_ordinal.wrapping_add(1);
                forged
                    .calc_cooling_supply_mass_flow_positive_guard_else_branch_entry
                    .latest
                    .as_mut()
                    .expect("latest")
                    .parent_call_ordinal = witness.parent_call_ordinal;
            },
        ];
        for (case_index, forgery) in forgeries.into_iter().enumerate() {
            assert_cp424_committed_rejects_forgery(
                &unit, snapshot, route, case_index, forgery,
            );
        }
    }
}

type Cp424Snapshot = super::super::PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardElseBranchEntrySnapshot;
type Cp424Forgery = fn(
    &mut crate::ideal_loads::PurchasedAirUnitRuntimeState,
    &mut Cp424Snapshot,
    Route,
);

#[inline(never)]
fn assert_cp424_committed_rejects_forgery(
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    witness: Cp424Snapshot,
    route: Route,
    case_index: usize,
    forgery: Cp424Forgery,
) {
    let mut forged = unit.clone();
    let mut forged_witness = witness;
    forgery(&mut forged, &mut forged_witness, route);
    assert!(
        committed(&forged, forged_witness).is_none(),
        "route {route:?} case {case_index}",
    );
}

#[test]
fn cp329_committed_enthalpy_seal_accepts_exact_owner_and_rejects_value_witness_and_count_drift() {
    let predecessor = representative_predecessors()[0];
    let (unit, _, _) = cp424_fixture_unit_for_successor_tests(predecessor);
    let witness = unit.calc_cooling_mixed_air_call.latest.expect("CP329");
    let expected = witness
        .mixed_air_enthalpy_projection_j_per_kg
        .expect("mixed enthalpy");
    assert_eq!(
        committed_cp329_enthalpy(&unit, witness).map(f64::to_bits),
        Some(expected.to_bits()),
    );

    let mut value = unit.clone();
    value
        .calc_cooling_mixed_air_call
        .latest
        .as_mut()
        .expect("CP329")
        .mixed_air_enthalpy_projection_j_per_kg = Some(flip(expected));
    assert!(committed_cp329_enthalpy(&value, witness).is_none());

    let mut forged_witness = witness;
    forged_witness.mixed_air_enthalpy_projection_j_per_kg = Some(flip(expected));
    assert!(committed_cp329_enthalpy(&unit, forged_witness).is_none());

    let mut count = unit.clone();
    count.calc_cooling_mixed_air_call.transition_count += 1;
    assert!(committed_cp329_enthalpy(&count, witness).is_none());

    let mut ordinal = unit.clone();
    crate::ideal_loads::calc::cooling_mixed_air_call_forge_latest_ordinal_for_test(
        &mut ordinal,
        Some(0),
    );
    assert!(committed_cp329_enthalpy(&ordinal, witness).is_none());

    let mut route = unit.clone();
    crate::ideal_loads::calc::cooling_mixed_air_call_clear_latest_route_for_test(&mut route);
    assert!(committed_cp329_enthalpy(&route, witness).is_none());

    let mut identity = unit.clone();
    identity
        .calc_cooling_mixed_air_call
        .latest
        .as_mut()
        .expect("CP329")
        .system = ep_model::IdealLoadsAirSystemId(witness.system.0.wrapping_add(1));
    let coordinated = identity.calc_cooling_mixed_air_call.latest.expect("CP329");
    assert!(committed_cp329_enthalpy(&identity, coordinated).is_none());

    let mut zone = unit.clone();
    zone.calc_cooling_mixed_air_call
        .latest
        .as_mut()
        .expect("CP329")
        .controlled_zone = ep_model::ZoneId(witness.controlled_zone.0.wrapping_add(1));
    let coordinated = zone.calc_cooling_mixed_air_call.latest.expect("CP329");
    assert!(committed_cp329_enthalpy(&zone, coordinated).is_none());

    let mut coordinated_ordinal = unit.clone();
    let forged_ordinal = witness.parent_call_ordinal.wrapping_add(1);
    coordinated_ordinal
        .calc_cooling_mixed_air_call
        .latest
        .as_mut()
        .expect("CP329")
        .parent_call_ordinal = forged_ordinal;
    let mut coordinated_witness = witness;
    coordinated_witness.parent_call_ordinal = forged_ordinal;
    assert!(
        committed_cp329_enthalpy(&coordinated_ordinal, coordinated_witness).is_none()
    );
}

fn representative_predecessors() -> [Cp422Predecessor; 4] {
    let all = crate::ideal_loads::calc::cp422_all_snapshots_for_successor_tests();
    let mut representatives = [
        all.iter()
            .copied()
            .find(|snapshot| snapshot.positive_guard_false_fallthrough_skipped)
            .expect("entry"),
        all.iter()
            .copied()
            .find(|snapshot| {
                crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_snapshot_route(*snapshot)
                    .is_some_and(|route| !route.active)
                    && !snapshot.positive_guard_false_fallthrough_skipped
            })
            .expect("inactive"),
        all.iter()
            .copied()
            .find(|snapshot| {
                crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_snapshot_route(*snapshot)
                    .is_some_and(|route| route.active && !route.assignment_executed)
            })
            .expect("guard false"),
        all.iter()
            .copied()
            .find(|snapshot| {
                crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_snapshot_route(*snapshot)
                    .is_some_and(|route| route.assignment_executed)
            })
            .expect("assignment"),
    ];
    for snapshot in &mut representatives {
        snapshot.parent_call_ordinal = 1;
    }
    representatives
}

pub(in crate::ideal_loads::calc) fn cp424_fixture_unit_for_successor_tests(
    predecessor: Cp422Predecessor,
) -> (
    crate::ideal_loads::PurchasedAirUnitRuntimeState,
    super::super::PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardElseBranchEntrySnapshot,
    Route,
) {
    let (mut unit, cp423, _) =
        crate::ideal_loads::calc::cp423_fixture_unit_for_successor_tests(predecessor);
    let route = route_for(cp423);
    let mut state = State::new(cp423.system);
    let snapshot = advance_validated(&mut state, cp423, route).expect("CP424");
    unit.calc_cooling_supply_mass_flow_positive_guard_else_branch_entry = state;
    (unit, snapshot, route)
}

fn flip(value: f64) -> f64 {
    f64::from_bits(value.to_bits() ^ 1)
}
