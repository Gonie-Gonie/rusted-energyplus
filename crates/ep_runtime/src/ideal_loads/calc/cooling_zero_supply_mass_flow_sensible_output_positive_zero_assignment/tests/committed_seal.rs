//! Focused CP428 committed-route and inherited private-witness tests.

use super::*;
use crate::ideal_loads::calc::{
    advance_cooling_zero_supply_mass_flow_supply_temperature_mixed_air_assignment_state_with_validated_route as cp427_advance_validated,
    cooling_mixed_air_call_committed_latest_mixed_air_temperature,
    cooling_zero_supply_mass_flow_sensible_output_positive_zero_assignment_committed_latest_route as committed,
    cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment_snapshot_route as cp426_route,
    cooling_zero_supply_mass_flow_supply_temperature_mixed_air_assignment_route_from_committed_predecessor as cp427_successor_route,
    cooling_zero_supply_mass_flow_supply_temperature_mixed_air_assignment_snapshot_route as cp427_route,
    cp422_all_snapshots_for_successor_tests, cp426_fixture_unit_for_successor_tests,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingMixedAirCallSnapshot as Cp329Snapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputMaximumCapacityAssignmentSnapshot as Cp422Snapshot,
    PurchasedAirCalcCoolingZeroSupplyMassFlowSensibleOutputPositiveZeroAssignmentSnapshot as Cp428Snapshot,
    PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyTemperatureMixedAirAssignmentRuntimeState as Cp427State,
    PurchasedAirUnitRuntimeState,
};

#[test]
fn cp428_committed_seal_requires_the_cp329_witness_only_on_the_active_route() {
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
fn cp428_committed_seal_rejects_route_witness_count_and_identity_forgeries() {
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
        OwnerCount,
        WriteCount,
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
        Forgery::OwnerCount,
        Forgery::WriteCount,
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
                witness.assigned_cooling_sensible_output_w =
                    witness.assigned_cooling_sensible_output_w.map(flip);
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
                    .zero_supply_mass_flow_sensible_output_positive_zero_assignment_route_counts
                    [route.logical_index] += 1;
            }
            Forgery::AssignmentCount => {
                current(&mut forged)
                    .zero_supply_mass_flow_sensible_output_positive_zero_assignment_count += 1;
            }
            Forgery::SiteCount => current(&mut forged).source_site_execution_count += 1,
            Forgery::OwnerCount => {
                current(&mut forged).cp428_cooling_sensible_output_state_owner_count += 1;
            }
            Forgery::WriteCount => {
                current(&mut forged).cooling_sensible_output_assignment_write_count += 1;
            }
            Forgery::PredecessorCount => {
                forged
                    .calc_cooling_zero_supply_mass_flow_supply_temperature_mixed_air_assignment
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
        }
        assert!(committed(&forged, witness, owner).is_none(), "forgery {index}");
    }
}

#[test]
fn cp428_committed_seal_binds_markers_positive_zero_prefix_and_private_witness() {
    let (unit, snapshot, _, owner) = fixture(true);
    for field in 0..4 {
        let mut forged = unit.clone();
        let latest = current(&mut forged).latest.as_mut().expect("latest");
        match field {
            0 => latest.cooling_zero_supply_mass_flow_sensible_output_positive_zero_assignment_executed = false,
            1 => latest.cooling_sensible_output_positive_zero_assignment_performed = false,
            2 => latest.assigned_cooling_sensible_output_w = Some(-0.0_f64),
            _ => {
                latest.assigned_supply_temperature_from_mixed_air_c =
                    latest.assigned_supply_temperature_from_mixed_air_c.map(flip);
            }
        }
        let forged_witness = *latest;
        assert!(committed(&forged, forged_witness, owner).is_none(), "field {field}");
    }

    let mut forged_owner = owner.expect("active owner");
    forged_owner.mixed_air_temperature_c = forged_owner.mixed_air_temperature_c.map(flip);
    assert!(committed(&unit, snapshot, Some(forged_owner)).is_none());

    let mut coordinated = unit.clone();
    let forged_temperature = {
        let cp329 = coordinated.calc_cooling_mixed_air_call.latest.as_mut().expect("CP329 latest");
        cp329.recirculation_temperature_c = cp329.recirculation_temperature_c.map(flip);
        cp329.mixed_air_temperature_c = cp329.mixed_air_temperature_c.map(flip);
        cp329.mixed_air_temperature_c
    };
    {
        let cp427 = coordinated
            .calc_cooling_zero_supply_mass_flow_supply_temperature_mixed_air_assignment
            .latest
            .as_mut()
            .expect("CP427 latest");
        cp427.mixed_air_temperature_for_zero_supply_mass_flow_supply_temperature_assignment_c = forged_temperature;
        cp427.assigned_supply_temperature_from_mixed_air_c = forged_temperature;
        cp427.resulting_supply_temperature_c = forged_temperature;
    }
    let forged_cp428 = {
        let latest = current(&mut coordinated).latest.as_mut().expect("CP428 latest");
        latest.mixed_air_temperature_for_zero_supply_mass_flow_supply_temperature_assignment_c = forged_temperature;
        latest.assigned_supply_temperature_from_mixed_air_c = forged_temperature;
        latest.predecessor_cp427_resulting_supply_temperature_c = forged_temperature;
        *latest
    };
    let coordinated_owner = coordinated.calc_cooling_mixed_air_call.latest;
    assert!(committed(&coordinated, forged_cp428, coordinated_owner).is_none());
    assert!(committed(&unit, snapshot, owner).is_some());
}

fn cp428_fixture_unit_for_successor_tests(
    mut predecessor: Cp422Snapshot,
) -> (PurchasedAirUnitRuntimeState, Cp428Snapshot, Route) {
    predecessor.parent_call_ordinal = 1;
    let (mut unit, cp426, _) = cp426_fixture_unit_for_successor_tests(predecessor);
    let cp427_retained = cp427_successor_route(cp426, cp426_route(cp426).expect("CP426 route"))
        .expect("CP427 route");
    let cp329_witness = unit.calc_cooling_mixed_air_call.latest;
    let temperature = cp427_retained.assignment_executed.then(|| {
        cooling_mixed_air_call_committed_latest_mixed_air_temperature(
            &unit,
            cp329_witness.expect("active CP329 witness"),
        )
        .expect("committed CP329 temperature")
    });
    let mut cp427_state = Cp427State::new(cp426.system);
    let cp427 = cp427_advance_validated(
        &mut cp427_state,
        cp426,
        cp427_retained,
        temperature,
    )
    .expect("CP427");
    unit.calc_cooling_zero_supply_mass_flow_supply_temperature_mixed_air_assignment = cp427_state;

    let route = route_for(cp427);
    assert_eq!(cp427_route(cp427), Some(cp427_retained));
    let mut state = State::new(cp427.system);
    let snapshot = advance_validated(&mut state, cp427, route).expect("CP428");
    unit.calc_cooling_zero_supply_mass_flow_sensible_output_positive_zero_assignment = state;
    (unit, snapshot, route)
}

fn fixture(expect_active: bool) -> (
    PurchasedAirUnitRuntimeState,
    Cp428Snapshot,
    Route,
    Option<Cp329Snapshot>,
) {
    for predecessor in cp422_all_snapshots_for_successor_tests() {
        let (unit, snapshot, route) = cp428_fixture_unit_for_successor_tests(predecessor);
        if route.assignment_executed == expect_active {
            let owner = unit.calc_cooling_mixed_air_call.latest;
            if !expect_active || owner.is_some() {
                return (unit, snapshot, route, owner);
            }
        }
    }
    unreachable!("CP428 fixture with active={expect_active}")
}

fn current(
    unit: &mut PurchasedAirUnitRuntimeState,
) -> &mut crate::ideal_loads::PurchasedAirCalcCoolingZeroSupplyMassFlowSensibleOutputPositiveZeroAssignmentRuntimeState {
    &mut unit.calc_cooling_zero_supply_mass_flow_sensible_output_positive_zero_assignment
}

fn flip(value: f64) -> f64 {
    f64::from_bits(value.to_bits() ^ 1)
}
