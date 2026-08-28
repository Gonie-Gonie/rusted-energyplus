//! Focused CP429 committed-route, forgery-loop, and lazy-owner tests.

use super::*;
use crate::ideal_loads::calc::{
    cooling_zero_supply_mass_flow_total_output_positive_zero_assignment_committed_latest_route as committed,
    cp422_all_snapshots_for_successor_tests, cp428_fixture_unit_for_successor_tests,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingMixedAirCallSnapshot as Cp329Snapshot,
    PurchasedAirCalcCoolingZeroSupplyMassFlowTotalOutputPositiveZeroAssignmentSnapshot as Cp429Snapshot,
    PurchasedAirUnitRuntimeState,
};

#[test]
fn cp429_committed_seal_requires_cp329_only_on_the_active_route() {
    let (unit, snapshot, route, owner) = fixture(true);
    assert_eq!(committed(&unit, snapshot, owner), Some(route));
    assert!(committed(&unit, snapshot, None).is_none());
    let active_owner = owner.expect("active CP329 owner");

    let (unit, snapshot, route, _) = fixture(false);
    assert_eq!(committed(&unit, snapshot, None), Some(route));
    assert!(committed(&unit, snapshot, Some(active_owner)).is_none());
}

#[test]
fn cp429_committed_seal_rejects_each_route_count_identity_and_value_forgery() {
    let (unit, snapshot, route, owner) = fixture(true);
    enum Forgery {
        LogicalIndex,
        Active,
        PredecessorAssignment,
        PredecessorEntered,
        Assignment,
        WitnessValue,
        LatestMissing,
        TransitionCount,
        InitCount,
        TransitionOrdinal,
        RouteCount,
        AssignmentRouteCount,
        AssignmentCount,
        SiteCount,
        HumidityOwnerCount,
        EnthalpyOwnerCount,
        TemperatureOwnerCount,
        TotalOutputOwnerCount,
        WriteCount,
        PredecessorCount,
        CoordinatedOrdinal,
        CoordinatedSystem,
        CoordinatedZone,
        StateSystem,
        CoordinatedNegativeZero,
    }
    let forgeries = [
        Forgery::LogicalIndex,
        Forgery::Active,
        Forgery::PredecessorAssignment,
        Forgery::PredecessorEntered,
        Forgery::Assignment,
        Forgery::WitnessValue,
        Forgery::LatestMissing,
        Forgery::TransitionCount,
        Forgery::InitCount,
        Forgery::TransitionOrdinal,
        Forgery::RouteCount,
        Forgery::AssignmentRouteCount,
        Forgery::AssignmentCount,
        Forgery::SiteCount,
        Forgery::HumidityOwnerCount,
        Forgery::EnthalpyOwnerCount,
        Forgery::TemperatureOwnerCount,
        Forgery::TotalOutputOwnerCount,
        Forgery::WriteCount,
        Forgery::PredecessorCount,
        Forgery::CoordinatedOrdinal,
        Forgery::CoordinatedSystem,
        Forgery::CoordinatedZone,
        Forgery::StateSystem,
        Forgery::CoordinatedNegativeZero,
    ];
    for (index, forgery) in forgeries.into_iter().enumerate() {
        let mut forged = unit.clone();
        let mut witness = snapshot;
        match forgery {
            Forgery::LogicalIndex => current(&mut forged).latest_route.as_mut().expect("route").logical_index = 3,
            Forgery::Active => current(&mut forged).latest_route.as_mut().expect("route").active = false,
            Forgery::PredecessorAssignment => {
                current(&mut forged).latest_route.as_mut().expect("route").predecessor_assignment_executed ^= true;
            }
            Forgery::PredecessorEntered => {
                current(&mut forged).latest_route.as_mut().expect("route").predecessor_entered = false;
            }
            Forgery::Assignment => {
                current(&mut forged).latest_route.as_mut().expect("route").assignment_executed = false;
            }
            Forgery::WitnessValue => {
                witness.assigned_cooling_total_output_w =
                    witness.assigned_cooling_total_output_w.map(flip);
            }
            Forgery::LatestMissing => current(&mut forged).latest = None,
            Forgery::TransitionCount => current(&mut forged).transition_count += 1,
            Forgery::InitCount => forged.init_call_count += 1,
            Forgery::TransitionOrdinal => current(&mut forged).latest_transition_ordinal = Some(0),
            Forgery::RouteCount => {
                current(&mut forged).predecessor_route_counts[route.logical_index] += 1;
            }
            Forgery::AssignmentRouteCount => {
                current(&mut forged)
                    .zero_supply_mass_flow_total_output_positive_zero_assignment_route_counts
                    [route.logical_index] += 1;
            }
            Forgery::AssignmentCount => {
                current(&mut forged)
                    .zero_supply_mass_flow_total_output_positive_zero_assignment_count += 1;
            }
            Forgery::SiteCount => current(&mut forged).source_site_execution_count += 1,
            Forgery::HumidityOwnerCount => {
                current(&mut forged).cp428_supply_humidity_ratio_state_owner_count += 1;
            }
            Forgery::EnthalpyOwnerCount => {
                current(&mut forged).cp428_supply_enthalpy_state_owner_count += 1;
            }
            Forgery::TemperatureOwnerCount => {
                current(&mut forged).cp428_supply_temperature_state_owner_count += 1;
            }
            Forgery::TotalOutputOwnerCount => {
                current(&mut forged).cp429_cooling_total_output_state_owner_count += 1;
            }
            Forgery::WriteCount => {
                current(&mut forged).cooling_total_output_assignment_write_count += 1;
            }
            Forgery::PredecessorCount => {
                forged
                    .calc_cooling_zero_supply_mass_flow_sensible_output_positive_zero_assignment
                    .predecessor_route_counts[route.logical_index] += 1;
            }
            Forgery::CoordinatedOrdinal => {
                let value = snapshot.parent_call_ordinal.wrapping_add(1);
                current(&mut forged).latest.as_mut().expect("latest").parent_call_ordinal = value;
                witness.parent_call_ordinal = value;
            }
            Forgery::CoordinatedSystem => {
                let value = ep_model::IdealLoadsAirSystemId(snapshot.system.0.wrapping_add(1));
                current(&mut forged).latest.as_mut().expect("latest").system = value;
                witness.system = value;
            }
            Forgery::CoordinatedZone => {
                let value = ep_model::ZoneId(snapshot.controlled_zone.0.wrapping_add(1));
                forged.controlled_zone = Some(value);
                current(&mut forged).latest.as_mut().expect("latest").controlled_zone = value;
                witness.controlled_zone = value;
            }
            Forgery::StateSystem => {
                current(&mut forged).system =
                    ep_model::IdealLoadsAirSystemId(snapshot.system.0.wrapping_add(1));
            }
            Forgery::CoordinatedNegativeZero => {
                current(&mut forged)
                    .latest
                    .as_mut()
                    .expect("latest")
                    .assigned_cooling_total_output_w = Some(-0.0);
                witness.assigned_cooling_total_output_w = Some(-0.0);
            }
        }
        assert!(
            committed(&forged, witness, owner).is_none(),
            "forgery {index}"
        );
    }
}

#[test]
fn cp429_committed_seal_rejects_the_active_private_owner_forgery() {
    let (unit, snapshot, _, owner) = fixture(true);
    let mut forged = owner.expect("active CP329 owner");
    forged.mixed_air_temperature_c = forged.mixed_air_temperature_c.map(flip);
    assert!(committed(&unit, snapshot, Some(forged)).is_none());
    assert!(committed(&unit, snapshot, owner).is_some());
}

fn fixture(expect_active: bool) -> (
    PurchasedAirUnitRuntimeState,
    Cp429Snapshot,
    Route,
    Option<Cp329Snapshot>,
) {
    for predecessor in cp422_all_snapshots_for_successor_tests() {
        let (mut unit, cp428, _) = cp428_fixture_unit_for_successor_tests(predecessor);
        let route = route_for(cp428);
        let mut state = State::new(cp428.system);
        let snapshot = advance_validated(&mut state, cp428, route).expect("CP429");
        unit.calc_cooling_zero_supply_mass_flow_total_output_positive_zero_assignment = state;
        if route.assignment_executed == expect_active {
            let owner = unit.calc_cooling_mixed_air_call.latest;
            if !expect_active || owner.is_some() {
                return (unit, snapshot, route, owner);
            }
        }
    }
    unreachable!("CP429 fixture with active={expect_active}")
}

fn current(
    unit: &mut PurchasedAirUnitRuntimeState,
) -> &mut crate::ideal_loads::PurchasedAirCalcCoolingZeroSupplyMassFlowTotalOutputPositiveZeroAssignmentRuntimeState {
    &mut unit.calc_cooling_zero_supply_mass_flow_total_output_positive_zero_assignment
}

fn flip(value: f64) -> f64 {
    f64::from_bits(value.to_bits() ^ 1)
}
