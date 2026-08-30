//! Focused CP431 retained-route seal and coordinated-forgery tests.

use super::*;
use crate::ideal_loads::calc::{
    PurchasedAirCalcCoolingZeroSupplyMassFlowTotalOutputPositiveZeroAssignmentRuntimeState as Cp429State,
    PurchasedAirCalcHeatingModeGuardRetainedRoute as Route,
    PurchasedAirCalcHeatingOrNoLoadCaseEntryRuntimeState as Cp430State,
    advance_cooling_zero_supply_mass_flow_total_output_positive_zero_assignment_state_with_validated_route as advance_cp429,
    advance_heating_or_no_load_case_entry_state_with_validated_route as advance_cp430_validated,
    cooling_zero_supply_mass_flow_total_output_positive_zero_assignment_route_from_committed_predecessor as cp429_route_from_predecessor,
    cooling_zero_supply_mass_flow_total_output_positive_zero_assignment_snapshot_route as cp429_snapshot_route,
    cp422_all_snapshots_for_successor_tests,
    cp428_fixture_unit_for_successor_tests,
    heating_mode_guard_committed_latest_route as committed,
    heating_or_no_load_case_entry_route_from_committed_predecessor as cp430_route_from_predecessor,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingMixedAirCallSnapshot as Cp329Snapshot,
    PurchasedAirCalcHeatingModeGuardSnapshot as Cp431Snapshot,
    PurchasedAirUnitRuntimeState,
};

#[test]
fn cp431_committed_seal_is_retained_constant_time_and_owner_lazy() {
    let source = include_str!("../release/committed.rs");
    for forbidden in [
        "cooling_entry_gate_committed_latest_heating_mode_guard_numeric_operands",
        "cooling_entry_gate_committed_latest_heating_mode_guard_temperature_control_type",
        "heating_mode_guard_route_from_committed_predecessor",
        "retained_route_matches_snapshot_bounded",
        "snapshot_route(",
        "completed_",
        "private_characterization",
        " < ",
    ] {
        assert!(!source.contains(forbidden), "{forbidden}");
    }
    assert_eq!(
        source
            .matches("heating_or_no_load_case_entry_committed_latest_route(")
            .count(),
        1,
    );
    let validation = include_str!("../release/snapshot_validation.rs");
    let start = validation
        .find("pub(super) fn committed_prefix_and_local_route_shape_match")
        .expect("committed local matcher");
    let end = validation[start..]
        .find("pub(super) fn prefix_and_local_shape_match")
        .map(|offset| start + offset)
        .expect("committed local matcher end");
    let committed_local = &validation[start..end];
    for forbidden in [
        "cooling_entry_gate_committed_latest_heating_mode_guard_numeric_operands",
        "cooling_entry_gate_committed_latest_heating_mode_guard_temperature_control_type",
        "SingleCool",
        " < ",
        "heating_mode_guard_route_from_committed_predecessor",
        "local_shape_is_exact(",
        "snapshot_route(",
    ] {
        assert!(!committed_local.contains(forbidden), "{forbidden}");
    }

    let (unit, snapshot, route, owner) = fixture();
    assert_eq!(committed(&unit, snapshot, owner), Some(route));
    assert!(owner.is_none(), "heating entry must not acquire the cooling owner");
}

#[test]
fn cp431_committed_seal_rejects_latest_witness_route_and_accounting_forgeries() {
    let (unit, snapshot, _, owner) = fixture();
    enum Forgery {
        Witness,
        Latest,
        Route,
        CoordinatedLatestWitnessRouteAccounting,
        Accounting,
        TransitionOrdinal,
        CoordinatedOrdinal,
        CoordinatedSystem,
        CoordinatedZone,
        PredecessorAccounting,
    }
    let forgeries = [
        Forgery::Witness,
        Forgery::Latest,
        Forgery::Route,
        Forgery::CoordinatedLatestWitnessRouteAccounting,
        Forgery::Accounting,
        Forgery::TransitionOrdinal,
        Forgery::CoordinatedOrdinal,
        Forgery::CoordinatedSystem,
        Forgery::CoordinatedZone,
        Forgery::PredecessorAccounting,
    ];
    for (index, forgery) in forgeries.into_iter().enumerate() {
        let mut forged = unit.clone();
        let mut witness = snapshot;
        match forgery {
            Forgery::Witness => witness.heating_operating_mode_body_entered ^= true,
            Forgery::Latest => current(&mut forged)
                .latest
                .as_mut()
                .expect("latest")
                .single_cool_blocked ^= true,
            Forgery::Route => current(&mut forged)
                .latest_route
                .as_mut()
                .expect("route")
                .body_entered ^= true,
            Forgery::CoordinatedLatestWitnessRouteAccounting => {
                let state = current(&mut forged);
                let route = state.latest_route.as_mut().expect("route");
                route.single_cool_blocked = true;
                route.body_entered = false;
                route.false_fallthrough = true;
                let latest = state.latest.as_mut().expect("latest");
                latest.single_cool_blocked = true;
                latest.heating_operating_mode_body_entered = false;
                latest.heating_mode_guard_false_fallthrough = true;
                witness.single_cool_blocked = true;
                witness.heating_operating_mode_body_entered = false;
                witness.heating_mode_guard_false_fallthrough = true;
                state.heating_operating_mode_body_entry_count = 0;
                state.heating_operating_mode_body_entry_route_counts[1] = 0;
                state.heating_mode_guard_false_fallthrough_count = 1;
                state.heating_mode_guard_false_fallthrough_route_counts[1] = 1;
                state.temperature_control_type_permits_heating_count = 0;
                state.single_cool_block_count = 1;
                state.source_site_execution_count = 5;
            }
            Forgery::Accounting => current(&mut forged).source_site_execution_count += 1,
            Forgery::TransitionOrdinal => {
                current(&mut forged).latest_transition_ordinal = Some(0)
            }
            Forgery::CoordinatedOrdinal => {
                let ordinal = snapshot.parent_call_ordinal.wrapping_add(1);
                current(&mut forged)
                    .latest
                    .as_mut()
                    .expect("latest")
                    .parent_call_ordinal = ordinal;
                witness.parent_call_ordinal = ordinal;
            }
            Forgery::CoordinatedSystem => {
                let system = ep_model::IdealLoadsAirSystemId(snapshot.system.0.wrapping_add(1));
                current(&mut forged)
                    .latest
                    .as_mut()
                    .expect("latest")
                    .system = system;
                witness.system = system;
            }
            Forgery::CoordinatedZone => {
                let zone = ep_model::ZoneId(snapshot.controlled_zone.0.wrapping_add(1));
                forged.controlled_zone = Some(zone);
                current(&mut forged)
                    .latest
                    .as_mut()
                    .expect("latest")
                    .controlled_zone = zone;
                witness.controlled_zone = zone;
            }
            Forgery::PredecessorAccounting => forged
                .calc_heating_or_no_load_case_entry
                .predecessor_route_counts[1] += 1,
        }
        assert!(committed(&forged, witness, owner).is_none(), "forgery {index}");
    }
}

fn fixture() -> (
    PurchasedAirUnitRuntimeState,
    Cp431Snapshot,
    Route,
    Option<Cp329Snapshot>,
) {
    for predecessor in cp422_all_snapshots_for_successor_tests() {
        let (mut unit, cp428, _) = cp428_fixture_unit_for_successor_tests(predecessor);
        let cp428_route = crate::ideal_loads::calc::cooling_zero_supply_mass_flow_sensible_output_positive_zero_assignment_snapshot_route(cp428)
            .expect("CP428 route");
        let cp429_route = cp429_route_from_predecessor(cp428, cp428_route).expect("CP429 route");
        let mut cp429_state = Cp429State::new(cp428.system);
        let cp429 = advance_cp429(&mut cp429_state, cp428, cp429_route).expect("CP429");
        unit.calc_cooling_zero_supply_mass_flow_total_output_positive_zero_assignment = cp429_state;
        let owner = cp429_route
            .assignment_executed
            .then_some(unit.calc_cooling_mixed_air_call.latest)
            .flatten();
        let cp430_route = cp430_route_from_predecessor(
            cp429,
            cp429_snapshot_route(cp429).expect("CP429 snapshot route"),
        )
        .expect("CP430 route");
        let mut cp430_state = Cp430State::new(cp429.system);
        let cp430 = advance_cp430_validated(&mut cp430_state, cp429, cp430_route).expect("CP430");
        unit.calc_heating_or_no_load_case_entry = cp430_state;
        if !cp430_route.entered {
            continue;
        }
        let input = active_input(0.0, 1.0, Some(Control::DualHeatCool));
        let route = successor_route(cp430, cp430_route, input).expect("CP431 route");
        let mut state = State::new(cp430.system);
        let snapshot = advance_validated(&mut state, cp430, cp430_route, input, route)
            .expect("CP431");
        unit.calc_heating_mode_guard = state;
        return (unit, snapshot, route, owner);
    }
    unreachable!("active CP431 fixture")
}

pub(in crate::ideal_loads::calc) fn cp431_committed_fixture_for_successor_tests() -> (
    PurchasedAirUnitRuntimeState,
    Cp431Snapshot,
    Route,
    Option<Cp329Snapshot>,
) {
    fixture()
}

fn current(unit: &mut PurchasedAirUnitRuntimeState) -> &mut State {
    &mut unit.calc_heating_mode_guard
}
