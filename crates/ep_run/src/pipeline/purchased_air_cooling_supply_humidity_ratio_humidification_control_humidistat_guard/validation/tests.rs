use super::*;

#[derive(Clone, Copy, Debug)]
enum Route {
    UnitOff,
    NonCooling,
    PositiveGuardFalse,
    NoneCase,
}

#[test]
fn missing_direct_lifecycle_fails_closed() {
    assert!(
        validate_direct_lifecycle(
            None,
            DirectLifecyclePredecessors {
                heating_availability_guard_cp369: None,
            },
            None,
            None,
        )
        .is_err()
    );
}

#[test]
fn three_site_guard_partition_and_source_product_are_checked() {
    let mut overflow = State::new(IdealLoadsAirSystemId(0));
    overflow.unit_off_skip_count = usize::MAX;
    overflow.non_cooling_skip_count = 1;
    assert!(validate_current_counters(&overflow).is_err());

    let (mut lifecycle, predecessor) = direct_lifecycles();
    lifecycle.state.source_site_execution_count = 3;
    assert!(validate_current_counters(&lifecycle.state).is_err());

    let (mut lifecycle, _) = direct_lifecycles();
    lifecycle
        .state
        .humidification_control_type_humidistat_comparison_count = 0;
    assert!(validate_current_counters(&lifecycle.state).is_err());

    let (mut lifecycle, _) = direct_lifecycles();
    lifecycle.state.humidification_control_body_entry_count = 1;
    lifecycle
        .state
        .humidification_control_guard_false_fallthrough_count = 0;
    lifecycle.state.source_site_execution_count = 3;
    assert!(
        validate_release_state(
            &lifecycle,
            &predecessor,
            IdealLoadsAirSystemId(0),
            ZoneId(0),
            1,
        )
        .is_err()
    );
}

#[test]
fn expected_snapshot_preserves_skips_and_direct_none_false_route() {
    for route in [Route::UnitOff, Route::NonCooling, Route::PositiveGuardFalse] {
        let snapshot = expected_snapshot(predecessor_snapshot(route));
        assert!(!snapshot.humidification_control_type_read, "{route:?}");
        assert_eq!(snapshot.humidification_control_type, None, "{route:?}");
        assert_eq!(
            snapshot.humidification_control_type_humidistat, None,
            "{route:?}"
        );
        assert!(!snapshot.humidification_control_body_entered, "{route:?}");
        assert!(
            !snapshot.humidification_control_guard_false_fallthrough,
            "{route:?}"
        );
    }

    let snapshot = expected_snapshot(predecessor_snapshot(Route::NoneCase));
    assert!(snapshot.predecessor_cooling_supply_humidity_ratio_humidification_body_entered);
    assert!(snapshot.humidification_control_type_read);
    assert_eq!(
        snapshot.humidification_control_type,
        Some(HumidificationControlType::None)
    );
    assert_eq!(snapshot.humidification_control_type_humidistat, Some(false));
    assert!(!snapshot.humidification_control_body_entered);
    assert!(snapshot.humidification_control_guard_false_fallthrough);
}

#[test]
fn direct_release_requires_exact_cp369_and_none_false_route() {
    let (lifecycle, predecessor) = direct_lifecycles();
    assert!(
        validate_release_state(
            &lifecycle,
            &predecessor,
            IdealLoadsAirSystemId(0),
            ZoneId(0),
            1,
        )
        .is_ok()
    );

    let mut forged_guard = lifecycle.clone();
    let latest = forged_guard.state.latest.as_mut().expect("latest");
    latest.humidification_control_type = Some(HumidificationControlType::Humidistat);
    latest.humidification_control_type_humidistat = Some(true);
    latest.humidification_control_body_entered = true;
    latest.humidification_control_guard_false_fallthrough = false;
    assert!(
        validate_release_state(
            &forged_guard,
            &predecessor,
            IdealLoadsAirSystemId(0),
            ZoneId(0),
            1,
        )
        .is_err()
    );

    let mut forged_predecessor = predecessor.clone();
    let latest = forged_predecessor.state.latest.as_mut().expect("latest");
    latest.heating_on = Some(false);
    latest.cooling_supply_humidity_ratio_humidification_body_entered = false;
    latest.heating_on_guard_false_fallthrough = true;
    assert!(
        validate_release_state(
            &lifecycle,
            &forged_predecessor,
            IdealLoadsAirSystemId(0),
            ZoneId(0),
            1,
        )
        .is_err()
    );
}

fn direct_lifecycles() -> (Lifecycle, PredecessorLifecycle) {
    let system = IdealLoadsAirSystemId(0);
    let predecessor_latest = predecessor_snapshot(Route::NoneCase);
    let mut predecessor_state = PredecessorState::new(system);
    predecessor_state.transition_count = 1;
    predecessor_state.dehumidification_control_none_case_completed_skip_count = 1;
    predecessor_state.heating_on_read_count = 1;
    predecessor_state.heating_on_body_entry_count = 1;
    predecessor_state.source_site_execution_count = 2;
    predecessor_state.latest = Some(predecessor_latest);
    let predecessor = PredecessorLifecycle {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_FIRST_EXCLUDED_SOURCE,
        state: predecessor_state,
    };

    let mut state = State::new(system);
    state.transition_count = 1;
    state.dehumidification_control_none_case_completed_skip_count = 1;
    state.heating_on_read_count = 1;
    state.heating_on_body_entry_count = 1;
    state.humidification_control_type_read_count = 1;
    state.humidification_control_type_humidistat_comparison_count = 1;
    state.humidification_control_guard_false_fallthrough_count = 1;
    state.source_site_execution_count = 2;
    state.latest = Some(expected_snapshot(predecessor_latest));
    (
        Lifecycle {
            source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_CONTROL_HUMIDISTAT_GUARD_SOURCE,
            first_excluded_source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_CONTROL_HUMIDISTAT_GUARD_FIRST_EXCLUDED_SOURCE,
            state,
        },
        predecessor,
    )
}

fn predecessor_snapshot(route: Route) -> PredecessorSnapshot {
    let unit_off = matches!(route, Route::UnitOff);
    let non_cooling = matches!(route, Route::NonCooling);
    let positive_guard_false = matches!(route, Route::PositiveGuardFalse);
    let active = matches!(route, Route::NoneCase);
    PredecessorSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_SOURCE_ORDER,
        system: IdealLoadsAirSystemId(0),
        parent_call_ordinal: 1,
        controlled_zone: ZoneId(0),
        unit_body_entered: !unit_off,
        predecessor_cooling_body_entered: !unit_off && !non_cooling,
        predecessor_no_outdoor_air_fallback_entered: !unit_off && !non_cooling,
        predecessor_positive_supply_mass_flow_body_entered: !unit_off && !non_cooling && !positive_guard_false,
        unit_off_skipped: unit_off,
        non_cooling_skipped: non_cooling,
        positive_guard_false_fallthrough_skipped: positive_guard_false,
        predecessor_dehumidification_control_type: active.then_some(DehumidificationControlType::None),
        predecessor_dehumidification_control_none_case_completed_skip: active,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip: false,
        predecessor_dehumidification_control_humidistat_case_completed_skip: false,
        predecessor_dehumidification_control_constant_supply_humidity_ratio_case_completed_skip: false,
        predecessor_dehumidification_control_default_supply_humidity_ratio_case_exited_via_break: false,
        dehumidification_control_none_case_completed_skip: active,
        dehumidification_control_constant_sensible_heat_ratio_case_completed_skip: false,
        dehumidification_control_humidistat_case_completed_skip: false,
        dehumidification_control_constant_supply_humidity_ratio_case_completed_skip: false,
        heating_on_read: active,
        heating_on: active.then_some(true),
        cooling_supply_humidity_ratio_humidification_body_entered: active,
        heating_on_guard_false_fallthrough: false,
    }
}
