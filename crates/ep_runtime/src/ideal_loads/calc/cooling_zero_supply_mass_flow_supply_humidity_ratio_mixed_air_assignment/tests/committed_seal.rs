//! Focused CP426 committed-route and independent CP329-witness tests.

use super::*;
use crate::ideal_loads::calc::{
    PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyEnthalpyMixedAirAssignmentRuntimeState as Cp425State,
    advance_cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_state_with_validated_route as advance_cp425,
    cooling_mixed_air_call_committed_latest_mixed_air_enthalpy,
    cooling_mixed_air_call_committed_latest_mixed_air_humidity_ratio,
    cooling_supply_mass_flow_positive_guard_else_branch_entry_snapshot_route as cp424_route,
    cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_route_from_committed_predecessor as cp425_successor_route,
    cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment_committed_latest_route as committed,
    cp422_all_snapshots_for_successor_tests,
    cp424_fixture_unit_for_successor_tests,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingMixedAirCallSnapshot as Cp329Snapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputMaximumCapacityAssignmentSnapshot as Cp422Snapshot,
    PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyHumidityRatioMixedAirAssignmentSnapshot as Cp426Snapshot,
    PurchasedAirUnitRuntimeState,
};

#[test]
fn cp426_committed_seal_requires_the_cp329_witness_only_on_the_active_route() {
    let (unit, snapshot, route, owner) = fixture(true);
    assert_eq!(committed(&unit, snapshot, owner), Some(route));
    assert!(committed(&unit, snapshot, None).is_none());

    let (unit, snapshot, route, owner) = fixture(false);
    assert_eq!(committed(&unit, snapshot, None), Some(route));
    if owner.is_some() {
        assert!(committed(&unit, snapshot, owner).is_none());
    }
}

#[test]
fn cp426_committed_seal_rejects_route_witness_count_and_identity_forgeries() {
    let (unit, snapshot, route, owner) = fixture(true);
    enum Forgery {
        LogicalIndex,
        Active,
        PredecessorAssignment,
        Assignment,
        WitnessValue,
        LatestMissing,
        TransitionCount,
        InitCount,
        TransitionOrdinal,
        RouteCount,
        AssignmentCount,
        SiteCount,
        PredecessorCount,
        CoordinatedOrdinal,
        CoordinatedSystem,
        CoordinatedZone,
        StateSystem,
    }
    let forgeries = [
        Forgery::LogicalIndex,
        Forgery::Active,
        Forgery::PredecessorAssignment,
        Forgery::Assignment,
        Forgery::WitnessValue,
        Forgery::LatestMissing,
        Forgery::TransitionCount,
        Forgery::InitCount,
        Forgery::TransitionOrdinal,
        Forgery::RouteCount,
        Forgery::AssignmentCount,
        Forgery::SiteCount,
        Forgery::PredecessorCount,
        Forgery::CoordinatedOrdinal,
        Forgery::CoordinatedSystem,
        Forgery::CoordinatedZone,
        Forgery::StateSystem,
    ];

    for (index, forgery) in forgeries.into_iter().enumerate() {
        let mut forged = unit.clone();
        let mut witness = snapshot;
        match forgery {
            Forgery::LogicalIndex => {
                forged
                    .calc_cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment
                    .latest_route
                    .as_mut()
                    .expect("route")
                    .logical_index = 3;
            }
            Forgery::Active => {
                forged
                    .calc_cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment
                    .latest_route
                    .as_mut()
                    .expect("route")
                    .active = false;
            }
            Forgery::PredecessorAssignment => {
                forged
                    .calc_cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment
                    .latest_route
                    .as_mut()
                    .expect("route")
                    .predecessor_assignment_executed ^= true;
            }
            Forgery::Assignment => {
                forged
                    .calc_cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment
                    .latest_route
                    .as_mut()
                    .expect("route")
                    .assignment_executed = false;
            }
            Forgery::WitnessValue => {
                witness.assigned_supply_humidity_ratio_from_mixed_air = witness
                    .assigned_supply_humidity_ratio_from_mixed_air
                    .map(flip);
            }
            Forgery::LatestMissing => {
                forged
                    .calc_cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment
                    .latest = None;
            }
            Forgery::TransitionCount => {
                forged
                    .calc_cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment
                    .transition_count += 1;
            }
            Forgery::InitCount => forged.init_call_count += 1,
            Forgery::TransitionOrdinal => {
                forged
                    .calc_cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment
                    .latest_transition_ordinal = Some(0);
            }
            Forgery::RouteCount => {
                forged
                    .calc_cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment
                    .predecessor_route_counts[route.logical_index] += 1;
            }
            Forgery::AssignmentCount => {
                forged
                    .calc_cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment
                    .zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment_count += 1;
            }
            Forgery::SiteCount => {
                forged
                    .calc_cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment
                    .source_site_execution_count += 1;
            }
            Forgery::PredecessorCount => {
                forged
                    .calc_cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment
                    .predecessor_route_counts[route.logical_index] += 1;
            }
            Forgery::CoordinatedOrdinal => {
                let forged_ordinal = snapshot.parent_call_ordinal.wrapping_add(1);
                forged
                    .calc_cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment
                    .latest
                    .as_mut()
                    .expect("latest")
                    .parent_call_ordinal = forged_ordinal;
                witness.parent_call_ordinal = forged_ordinal;
            }
            Forgery::CoordinatedSystem => {
                let forged_system =
                    ep_model::IdealLoadsAirSystemId(snapshot.system.0.wrapping_add(1));
                forged
                    .calc_cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment
                    .latest
                    .as_mut()
                    .expect("latest")
                    .system = forged_system;
                witness.system = forged_system;
            }
            Forgery::CoordinatedZone => {
                let forged_zone = ep_model::ZoneId(snapshot.controlled_zone.0.wrapping_add(1));
                forged.controlled_zone = Some(forged_zone);
                forged
                    .calc_cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment
                    .latest
                    .as_mut()
                    .expect("latest")
                    .controlled_zone = forged_zone;
                witness.controlled_zone = forged_zone;
            }
            Forgery::StateSystem => {
                forged
                    .calc_cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment
                    .system = ep_model::IdealLoadsAirSystemId(snapshot.system.0.wrapping_add(1));
            }
        }
        assert!(
            committed(&forged, witness, owner).is_none(),
            "forgery {index}",
        );
    }
}

#[test]
fn cp426_committed_seal_binds_each_active_humidity_carrier_to_private_cp329() {
    let (unit, snapshot, _, owner) = fixture(true);
    for field in 0..3 {
        let mut forged = unit.clone();
        let latest = forged
            .calc_cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment
            .latest
            .as_mut()
            .expect("latest");
        match field {
            0 => {
                latest.mixed_air_humidity_ratio_for_zero_supply_mass_flow_supply_humidity_ratio_assignment = latest.mixed_air_humidity_ratio_for_zero_supply_mass_flow_supply_humidity_ratio_assignment.map(flip);
            }
            1 => {
                latest.assigned_supply_humidity_ratio_from_mixed_air =
                    latest.assigned_supply_humidity_ratio_from_mixed_air.map(flip);
            }
            _ => {
                latest.resulting_supply_humidity_ratio =
                    latest.resulting_supply_humidity_ratio.map(flip);
            }
        }
        let forged_witness = *latest;
        assert!(committed(&forged, forged_witness, owner).is_none());
    }

    let mut coordinated = unit.clone();
    let forged_humidity = {
        let cp329 = coordinated
            .calc_cooling_mixed_air_call
            .latest
            .as_mut()
            .expect("CP329 latest");
        cp329.recirculation_humidity_ratio = cp329.recirculation_humidity_ratio.map(flip);
        cp329.mixed_air_humidity_ratio = cp329.mixed_air_humidity_ratio.map(flip);
        cp329.mixed_air_humidity_ratio
    };
    let latest = coordinated
        .calc_cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment
        .latest
        .as_mut()
        .expect("CP426 latest");
    latest.mixed_air_humidity_ratio_for_zero_supply_mass_flow_supply_humidity_ratio_assignment = forged_humidity;
    latest.assigned_supply_humidity_ratio_from_mixed_air = forged_humidity;
    latest.resulting_supply_humidity_ratio = forged_humidity;
    let forged_cp426 = *latest;
    assert!(committed(&coordinated, forged_cp426, owner).is_none());

    assert!(committed(&unit, snapshot, owner).is_some());
}

pub(in crate::ideal_loads::calc) fn cp426_fixture_unit_for_successor_tests(
    mut predecessor: Cp422Snapshot,
) -> (PurchasedAirUnitRuntimeState, Cp426Snapshot, Route) {
    predecessor.parent_call_ordinal = 1;
    let (mut unit, cp424, _) = cp424_fixture_unit_for_successor_tests(predecessor);
    let cp425_route = cp425_successor_route(
        cp424,
        cp424_route(cp424).expect("CP424 route"),
    )
    .expect("CP425 route");
    let cp329_witness = unit.calc_cooling_mixed_air_call.latest;
    let enthalpy = cp425_route.assignment_executed.then(|| {
        cooling_mixed_air_call_committed_latest_mixed_air_enthalpy(
            &unit,
            cp329_witness.expect("active CP329 witness"),
        )
        .expect("committed CP329 enthalpy")
    });
    let mut cp425_state = Cp425State::new(cp424.system);
    let cp425 = advance_cp425(&mut cp425_state, cp424, cp425_route, enthalpy)
        .expect("CP425");
    unit.calc_cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment = cp425_state;

    let route = route_for(cp425);
    let humidity = route.assignment_executed.then(|| {
        cooling_mixed_air_call_committed_latest_mixed_air_humidity_ratio(
            &unit,
            cp329_witness.expect("active CP329 witness"),
        )
        .expect("committed CP329 humidity ratio")
    });
    let mut state = State::new(cp425.system);
    let snapshot = advance_validated(&mut state, cp425, route, humidity).expect("CP426");
    unit.calc_cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment = state;
    (unit, snapshot, route)
}

fn fixture(expect_active: bool) -> (
    PurchasedAirUnitRuntimeState,
    Cp426Snapshot,
    Route,
    Option<Cp329Snapshot>,
) {
    for predecessor in cp422_all_snapshots_for_successor_tests() {
        let (unit, snapshot, route) = cp426_fixture_unit_for_successor_tests(predecessor);
        if route.assignment_executed == expect_active {
            let owner = unit.calc_cooling_mixed_air_call.latest;
            if !expect_active || owner.is_some() {
                return (unit, snapshot, route, owner);
            }
        }
    }
    unreachable!("CP426 fixture with active={expect_active}")
}

fn flip(value: f64) -> f64 {
    f64::from_bits(value.to_bits() ^ 1)
}
