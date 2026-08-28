//! Focused CP430 committed-route, forgery-loop, and lazy-owner tests.

use super::*;
use crate::ideal_loads::calc::{
    PurchasedAirCalcCoolingZeroSupplyMassFlowTotalOutputPositiveZeroAssignmentRuntimeState as Cp429State,
    advance_cooling_zero_supply_mass_flow_total_output_positive_zero_assignment_state_with_validated_route as advance_cp429,
    cooling_zero_supply_mass_flow_total_output_positive_zero_assignment_committed_latest_route as committed_cp429,
    cooling_zero_supply_mass_flow_total_output_positive_zero_assignment_route_from_committed_predecessor as cp429_route_from_predecessor,
    cooling_zero_supply_mass_flow_total_output_positive_zero_assignment_snapshot_route as cp429_snapshot_route,
    cp422_all_snapshots_for_successor_tests, cp428_fixture_unit_for_successor_tests,
    heating_or_no_load_case_entry_committed_latest_route as committed,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingMixedAirCallSnapshot as Cp329Snapshot,
    PurchasedAirCalcHeatingOrNoLoadCaseEntrySnapshot as Cp430Snapshot,
    PurchasedAirUnitRuntimeState,
};

#[test]
fn cp430_committed_seal_is_lazy_on_the_entry_route() {
    let (unit, snapshot, route, _) = fixture(true);
    assert_eq!(committed(&unit, snapshot, None), Some(route));
    assert!(committed(&unit, snapshot, Some(active_cp329_owner())).is_none());
}

#[test]
fn cp430_committed_seal_rejects_each_route_and_accounting_forgery() {
    let (unit, snapshot, _, _) = fixture(true);
    enum Forgery {
        LogicalIndex,
        Active,
        PredecessorAssignment,
        PredecessorEntered,
        Assignment,
        Entered,
        WitnessMarker,
        LatestMissing,
        TransitionCount,
        InitCount,
        TransitionOrdinal,
        PredecessorRouteCount,
        EntryRouteCount,
        EntryCount,
        SiteCount,
        HumidityOwner,
        EnthalpyOwner,
        TemperatureOwner,
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
        Forgery::Entered,
        Forgery::WitnessMarker,
        Forgery::LatestMissing,
        Forgery::TransitionCount,
        Forgery::InitCount,
        Forgery::TransitionOrdinal,
        Forgery::PredecessorRouteCount,
        Forgery::EntryRouteCount,
        Forgery::EntryCount,
        Forgery::SiteCount,
        Forgery::HumidityOwner,
        Forgery::EnthalpyOwner,
        Forgery::TemperatureOwner,
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
            Forgery::LogicalIndex => current(&mut forged).latest_route.as_mut().expect("route").logical_index = 2,
            Forgery::Active => current(&mut forged).latest_route.as_mut().expect("route").active ^= true,
            Forgery::PredecessorAssignment => current(&mut forged).latest_route.as_mut().expect("route").predecessor_assignment_executed ^= true,
            Forgery::PredecessorEntered => current(&mut forged).latest_route.as_mut().expect("route").predecessor_entered ^= true,
            Forgery::Assignment => current(&mut forged).latest_route.as_mut().expect("route").assignment_executed ^= true,
            Forgery::Entered => current(&mut forged).latest_route.as_mut().expect("route").entered ^= true,
            Forgery::WitnessMarker => witness.heating_or_no_load_case_entered ^= true,
            Forgery::LatestMissing => current(&mut forged).latest = None,
            Forgery::TransitionCount => current(&mut forged).transition_count += 1,
            Forgery::InitCount => forged.init_call_count += 1,
            Forgery::TransitionOrdinal => current(&mut forged).latest_transition_ordinal = Some(0),
            Forgery::PredecessorRouteCount => current(&mut forged).predecessor_route_counts[1] += 1,
            Forgery::EntryRouteCount => current(&mut forged).heating_or_no_load_case_entry_route_counts[1] += 1,
            Forgery::EntryCount => current(&mut forged).heating_or_no_load_case_entry_count += 1,
            Forgery::SiteCount => current(&mut forged).source_site_execution_count += 1,
            Forgery::HumidityOwner => current(&mut forged).cp429_supply_humidity_ratio_state_owner_count += 1,
            Forgery::EnthalpyOwner => current(&mut forged).cp429_supply_enthalpy_state_owner_count += 1,
            Forgery::TemperatureOwner => current(&mut forged).cp429_supply_temperature_state_owner_count += 1,
            Forgery::PredecessorCount => forged.calc_cooling_zero_supply_mass_flow_total_output_positive_zero_assignment.predecessor_route_counts[1] += 1,
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
            Forgery::StateSystem => current(&mut forged).system = ep_model::IdealLoadsAirSystemId(snapshot.system.0.wrapping_add(1)),
        }
        assert!(committed(&forged, witness, None).is_none(), "forgery {index}");
    }
}

fn fixture(expect_entry: bool) -> (
    PurchasedAirUnitRuntimeState,
    Cp430Snapshot,
    Route,
    Option<Cp329Snapshot>,
) {
    for predecessor in cp422_all_snapshots_for_successor_tests() {
        let (mut unit, cp428, _) = cp428_fixture_unit_for_successor_tests(predecessor);
        let cp428_route = crate::ideal_loads::calc::cooling_zero_supply_mass_flow_sensible_output_positive_zero_assignment_snapshot_route(cp428).expect("CP428 route");
        let cp429_route = cp429_route_from_predecessor(cp428, cp428_route).expect("CP429 route");
        let mut cp429_state = Cp429State::new(cp428.system);
        let cp429 = advance_cp429(&mut cp429_state, cp428, cp429_route).expect("CP429");
        unit.calc_cooling_zero_supply_mass_flow_total_output_positive_zero_assignment = cp429_state;
        let owner = cp429_route
            .assignment_executed
            .then_some(unit.calc_cooling_mixed_air_call.latest)
            .flatten();
        if committed_cp429(&unit, cp429, owner).is_none() {
            continue;
        }
        let route = successor_route(cp429, cp429_snapshot_route(cp429).expect("route"))
            .expect("CP430 route");
        let mut state = State::new(cp429.system);
        let snapshot = advance_validated(&mut state, cp429, route).expect("CP430");
        unit.calc_heating_or_no_load_case_entry = state;
        if route.entered == expect_entry {
            return (unit, snapshot, route, owner);
        }
    }
    unreachable!("CP430 fixture with entry={expect_entry}")
}

fn active_cp329_owner() -> Cp329Snapshot {
    for predecessor in cp422_all_snapshots_for_successor_tests() {
        let (unit, cp428, _) = cp428_fixture_unit_for_successor_tests(predecessor);
        let predecessor_route = crate::ideal_loads::calc::cooling_zero_supply_mass_flow_sensible_output_positive_zero_assignment_snapshot_route(cp428).expect("CP428 route");
        let route = cp429_route_from_predecessor(cp428, predecessor_route).expect("CP429 route");
        if route.assignment_executed {
            return unit.calc_cooling_mixed_air_call.latest.expect("active CP329 owner");
        }
    }
    unreachable!("active CP329 owner")
}

fn current(unit: &mut PurchasedAirUnitRuntimeState) -> &mut State {
    &mut unit.calc_heating_or_no_load_case_entry
}
