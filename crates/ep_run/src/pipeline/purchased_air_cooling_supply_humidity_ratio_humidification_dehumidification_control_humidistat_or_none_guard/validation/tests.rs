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
                humidification_control_humidistat_guard_cp370: None,
            },
            None,
            None,
        )
        .is_err()
    );
}

#[test]
fn public_direct_lifecycle_has_zero_current_sites() {
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
    let state = &lifecycle.state;
    assert_eq!(state.dehumidification_control_type_first_read_count, 0);
    assert_eq!(state.dehumidification_control_type_second_read_count, 0);
    assert_eq!(state.dehumidification_control_body_entry_count, 0);
    assert_eq!(state.source_site_execution_count, 0);
}

#[test]
fn current_site_drift_and_counter_overflow_fail_closed() {
    let (mut lifecycle, predecessor) = direct_lifecycles();
    lifecycle
        .state
        .dehumidification_control_type_first_read_count = 1;
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

    let (mut overflow, predecessor) = direct_lifecycles();
    overflow.state.unit_off_skip_count = usize::MAX;
    overflow.state.non_cooling_skip_count = 1;
    assert!(
        validate_release_state(
            &overflow,
            &predecessor,
            IdealLoadsAirSystemId(0),
            ZoneId(0),
            1,
        )
        .is_err()
    );

    let (mut source_drift, predecessor) = direct_lifecycles();
    source_drift.state.source_site_execution_count = 1;
    assert!(
        validate_release_state(
            &source_drift,
            &predecessor,
            IdealLoadsAirSystemId(0),
            ZoneId(0),
            1,
        )
        .is_err()
    );
}

#[test]
fn exact_cp370_predecessor_and_snapshot_link_are_required() {
    let (lifecycle, mut predecessor) = direct_lifecycles();
    let latest = predecessor.state.latest.as_mut().expect("CP370 latest");
    latest.humidification_control_type = Some(HumidificationControlType::Humidistat);
    latest.humidification_control_type_humidistat = Some(true);
    latest.humidification_control_body_entered = true;
    latest.humidification_control_guard_false_fallthrough = false;
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

    let (mut lifecycle, predecessor) = direct_lifecycles();
    let latest = lifecycle.state.latest.as_mut().expect("CP371 latest");
    latest.dehumidification_control_type_first_read = true;
    latest.first_dehumidification_control_type = Some(DehumidificationControlType::None);
    latest.dehumidification_control_type_humidistat = Some(false);
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
fn expected_snapshot_preserves_all_direct_routes_and_skips_current_guard() {
    for route in [
        Route::UnitOff,
        Route::NonCooling,
        Route::PositiveGuardFalse,
        Route::NoneCase,
    ] {
        let predecessor = predecessor_snapshot(route);
        assert!(
            predecessor_latest_is_exact_direct_shape(predecessor),
            "{route:?}"
        );
        let snapshot = expected_snapshot(predecessor);
        assert!(
            !snapshot.dehumidification_control_type_first_read,
            "{route:?}"
        );
        assert_eq!(
            snapshot.first_dehumidification_control_type, None,
            "{route:?}"
        );
        assert!(
            !snapshot.dehumidification_control_type_second_read,
            "{route:?}"
        );
        assert_eq!(
            snapshot.second_dehumidification_control_type, None,
            "{route:?}"
        );
        assert_eq!(
            snapshot.dehumidification_control_type_none, None,
            "{route:?}"
        );
        assert!(!snapshot.dehumidification_control_body_entered, "{route:?}");
        assert!(
            !snapshot.dehumidification_control_guard_false_fallthrough,
            "{route:?}"
        );
    }
}

fn direct_lifecycles() -> (Lifecycle, PredecessorLifecycle) {
    let system = IdealLoadsAirSystemId(0);
    let predecessor_latest = predecessor_snapshot(Route::NoneCase);
    let mut predecessor_state = PredecessorState::new(system);
    predecessor_state.transition_count = 1;
    predecessor_state.dehumidification_control_none_case_completed_skip_count = 1;
    predecessor_state.heating_on_read_count = 1;
    predecessor_state.heating_on_body_entry_count = 1;
    predecessor_state.humidification_control_type_read_count = 1;
    predecessor_state.humidification_control_type_humidistat_comparison_count = 1;
    predecessor_state.humidification_control_guard_false_fallthrough_count = 1;
    predecessor_state.source_site_execution_count = 2;
    predecessor_state.latest = Some(predecessor_latest);
    let predecessor = PredecessorLifecycle {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_CONTROL_HUMIDISTAT_GUARD_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_CONTROL_HUMIDISTAT_GUARD_FIRST_EXCLUDED_SOURCE,
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
    state.latest = Some(expected_snapshot(predecessor_latest));
    (
        Lifecycle {
            source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_SOURCE,
            first_excluded_source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_FIRST_EXCLUDED_SOURCE,
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
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_CONTROL_HUMIDISTAT_GUARD_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_CONTROL_HUMIDISTAT_GUARD_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_CONTROL_HUMIDISTAT_GUARD_SOURCE_ORDER,
        system: IdealLoadsAirSystemId(0),
        parent_call_ordinal: 1,
        controlled_zone: ZoneId(0),
        unit_body_entered: !unit_off,
        predecessor_cooling_body_entered: !unit_off && !non_cooling,
        predecessor_no_outdoor_air_fallback_entered: !unit_off && !non_cooling,
        predecessor_positive_supply_mass_flow_body_entered:
            !unit_off && !non_cooling && !positive_guard_false,
        unit_off_skipped: unit_off,
        non_cooling_skipped: non_cooling,
        positive_guard_false_fallthrough_skipped: positive_guard_false,
        predecessor_dehumidification_control_type:
            active.then_some(DehumidificationControlType::None),
        predecessor_dehumidification_control_none_case_completed_skip: active,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip:
            false,
        predecessor_dehumidification_control_humidistat_case_completed_skip: false,
        predecessor_dehumidification_control_constant_supply_humidity_ratio_case_completed_skip:
            false,
        predecessor_dehumidification_control_default_supply_humidity_ratio_case_exited_via_break:
            false,
        dehumidification_control_none_case_completed_skip: active,
        dehumidification_control_constant_sensible_heat_ratio_case_completed_skip: false,
        dehumidification_control_humidistat_case_completed_skip: false,
        dehumidification_control_constant_supply_humidity_ratio_case_completed_skip: false,
        predecessor_heating_on_read: active,
        predecessor_heating_on: active.then_some(true),
        predecessor_cooling_supply_humidity_ratio_humidification_body_entered: active,
        predecessor_heating_on_guard_false_fallthrough: false,
        humidification_control_type_read: active,
        humidification_control_type: active.then_some(HumidificationControlType::None),
        humidification_control_type_humidistat: active.then_some(false),
        humidification_control_body_entered: false,
        humidification_control_guard_false_fallthrough: active,
    }
}
