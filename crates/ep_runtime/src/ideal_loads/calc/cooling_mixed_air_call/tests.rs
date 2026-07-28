use ep_model::{IdealLoadsAirSystemId, NodeId, ZoneId};

use super::*;
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_BODY_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_BODY_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_BODY_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot, moist_air_enthalpy_j_per_kg,
};

fn predecessor(route: Route) -> PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot {
    let (unit_body_entered, unit_off, non_cooling, cooling, body, fallthrough, supply) = match route
    {
        Route::UnitOff => (false, true, false, false, false, false, None),
        Route::NonCooling => (true, false, true, false, false, false, None),
        Route::CoolingAssigned => (true, false, false, true, true, false, Some(0.0)),
        Route::CoolingFallthrough => (true, false, false, true, false, true, Some(0.25)),
    };
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_BODY_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_BODY_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_BODY_SOURCE_ORDER,
        system: IdealLoadsAirSystemId(3),
        parent_call_ordinal: 1,
        controlled_zone: ZoneId(4),
        unit_body_entered,
        predecessor_cooling_body_entered: cooling,
        predecessor_ems_supply_mass_flow_override_body_entered: false,
        predecessor_ems_supply_mass_flow_override_body_skipped: true,
        predecessor_ems_disabled_fallthrough: cooling,
        predecessor_supply_mass_flow_limit_body_entered: false,
        predecessor_supply_mass_flow_limit_body_skipped: true,
        predecessor_supply_mass_flow_limit_active_guard_false_fallthrough: cooling,
        predecessor_zero_flow_reset_body_entered: body,
        predecessor_active_guard_false_fallthrough: fallthrough,
        unit_off_skipped: unit_off,
        non_cooling_skipped: non_cooling,
        cooling_body_entered: cooling,
        zero_flow_reset_body_entered: body,
        body_skipped: !body,
        active_guard_false_fallthrough: fallthrough,
        predecessor_supply_mass_flow_rate_kg_per_s: supply,
        supply_mass_flow_rate_positive_zero_assignment_performed: body,
        assigned_supply_mass_flow_rate_kg_per_s: body.then_some(0.0),
        resulting_supply_mass_flow_rate_kg_per_s: supply,
    }
}

#[derive(Clone, Copy)]
enum Route {
    UnitOff,
    NonCooling,
    CoolingAssigned,
    CoolingFallthrough,
}

fn active_input(supply: f64) -> PurchasedAirCalcCoolingMixedAirCallActiveInput {
    let recirculation_temperature_c = 23.5;
    let recirculation_humidity_ratio = 0.008;
    PurchasedAirCalcCoolingMixedAirCallActiveInput {
        recirculation_node: NodeId(9),
        recirculation_temperature_c,
        recirculation_humidity_ratio,
        recirculation_enthalpy_projection_j_per_kg: moist_air_enthalpy_j_per_kg(
            recirculation_temperature_c,
            recirculation_humidity_ratio,
        ),
        outdoor_air_mass_flow_rate_kg_per_s: 0.0,
        supply_mass_flow_rate_kg_per_s: supply,
    }
}

#[test]
fn active_cooling_calls_child_for_positive_and_zero_supply() {
    for (route, supply) in [
        (Route::CoolingAssigned, 0.0),
        (Route::CoolingFallthrough, 0.25),
    ] {
        let predecessor = predecessor(route);
        let mut state = PurchasedAirCalcCoolingMixedAirCallRuntimeState::new(predecessor.system);
        let snapshot = advance_cooling_mixed_air_call_state(
            &mut state,
            predecessor,
            Some(active_input(supply)),
        );

        assert!(cooling_mixed_air_call_snapshot_is_exact_direct_release(
            snapshot
        ));
        assert_eq!(
            snapshot.operating_mode,
            Some(IdealLoadsSensibleMode::Cooling)
        );
        assert_eq!(
            snapshot
                .outdoor_air_mass_flow_rate_kg_per_s
                .map(f64::to_bits),
            Some(0)
        );
        assert_eq!(
            snapshot.supply_mass_flow_rate_kg_per_s.map(f64::to_bits),
            Some(supply.to_bits())
        );
        assert_eq!(
            snapshot
                .resulting_recirculation_mass_flow_rate_kg_per_s
                .map(f64::to_bits),
            Some(supply.to_bits())
        );
        assert_eq!(
            snapshot.mixed_air_temperature_c.map(f64::to_bits),
            snapshot.recirculation_temperature_c.map(f64::to_bits)
        );
        assert_eq!(
            snapshot.mixed_air_humidity_ratio.map(f64::to_bits),
            snapshot.recirculation_humidity_ratio.map(f64::to_bits)
        );
        assert_eq!(
            snapshot
                .mixed_air_enthalpy_projection_j_per_kg
                .map(f64::to_bits),
            snapshot
                .recirculation_enthalpy_projection_j_per_kg
                .map(f64::to_bits)
        );
        assert_eq!(state.transition_count, 1);
        assert_eq!(state.cooling_call_count, 1);
        assert_eq!(state.caller_source_site_execution_count, 9);
        assert_eq!(state.child_source_site_execution_count, 22);
    }
}

#[test]
fn unit_off_and_non_cooling_skip_all_call_and_child_sites() {
    for route in [Route::UnitOff, Route::NonCooling] {
        let predecessor = predecessor(route);
        let mut state = PurchasedAirCalcCoolingMixedAirCallRuntimeState::new(predecessor.system);
        let snapshot = advance_cooling_mixed_air_call_state(&mut state, predecessor, None);

        assert!(cooling_mixed_air_call_snapshot_is_exact_direct_release(
            snapshot
        ));
        assert!(!snapshot.cooling_call_executed);
        assert!(!snapshot.calc_purch_air_mixed_air_called);
        assert!(snapshot.supply_mass_flow_rate_kg_per_s.is_none());
        assert!(
            snapshot
                .recirculation_enthalpy_projection_j_per_kg
                .is_none()
        );
        assert!(snapshot.mixed_air_temperature_c.is_none());
        assert_eq!(state.cooling_call_count, 0);
        assert_eq!(state.caller_source_site_execution_count, 0);
        assert_eq!(state.child_source_site_execution_count, 0);
    }
}

#[test]
fn release_predicate_rejects_negative_zero_recovery_write() {
    let predecessor = predecessor(Route::CoolingFallthrough);
    let mut state = PurchasedAirCalcCoolingMixedAirCallRuntimeState::new(predecessor.system);
    let mut snapshot =
        advance_cooling_mixed_air_call_state(&mut state, predecessor, Some(active_input(0.25)));
    snapshot.heat_recovery_sensible_output_w = Some(-0.0);

    assert!(!cooling_mixed_air_call_snapshot_is_exact_direct_release(
        snapshot
    ));
}

#[test]
fn release_predicate_rejects_nonfinite_or_incoherent_recirculation_projection() {
    let predecessor = predecessor(Route::CoolingFallthrough);
    let mut state = PurchasedAirCalcCoolingMixedAirCallRuntimeState::new(predecessor.system);
    let snapshot =
        advance_cooling_mixed_air_call_state(&mut state, predecessor, Some(active_input(0.25)));

    let mut nonfinite_temperature = snapshot;
    nonfinite_temperature.recirculation_temperature_c = Some(f64::NAN);
    nonfinite_temperature.mixed_air_temperature_c = Some(f64::NAN);
    assert!(!cooling_mixed_air_call_snapshot_is_exact_direct_release(
        nonfinite_temperature
    ));

    let mut nonfinite_humidity = snapshot;
    nonfinite_humidity.recirculation_humidity_ratio = Some(f64::INFINITY);
    nonfinite_humidity.mixed_air_humidity_ratio = Some(f64::INFINITY);
    assert!(!cooling_mixed_air_call_snapshot_is_exact_direct_release(
        nonfinite_humidity
    ));

    let mut nonfinite_enthalpy = snapshot;
    nonfinite_enthalpy.recirculation_enthalpy_projection_j_per_kg = Some(f64::INFINITY);
    nonfinite_enthalpy.mixed_air_enthalpy_projection_j_per_kg = Some(f64::INFINITY);
    assert!(!cooling_mixed_air_call_snapshot_is_exact_direct_release(
        nonfinite_enthalpy
    ));

    let mut incoherent_enthalpy = snapshot;
    incoherent_enthalpy.recirculation_enthalpy_projection_j_per_kg = incoherent_enthalpy
        .recirculation_enthalpy_projection_j_per_kg
        .map(|value| value + 1.0);
    incoherent_enthalpy.mixed_air_enthalpy_projection_j_per_kg =
        incoherent_enthalpy.recirculation_enthalpy_projection_j_per_kg;
    assert!(!cooling_mixed_air_call_snapshot_is_exact_direct_release(
        incoherent_enthalpy
    ));
}
