use super::*;

#[derive(Clone, Copy, Debug)]
enum Route {
    UnitOff,
    NonCooling,
    PositiveGuardFalse,
    NoneCase,
    ConstantShr,
    Humidistat,
    ConstantSupplyHumidityRatio,
}

#[test]
fn missing_direct_lifecycle_fails_closed() {
    assert!(
        validate_direct_lifecycle(
            None,
            DirectLifecyclePredecessors {
                case_break_cp368: None,
            },
            None,
            None,
        )
        .is_err()
    );
}

#[test]
fn two_site_guard_partition_and_source_product_are_checked() {
    let mut overflow = State::new(IdealLoadsAirSystemId(0));
    overflow.unit_off_skip_count = usize::MAX;
    overflow.non_cooling_skip_count = 1;
    assert!(validate_current_counters(&overflow).is_err());

    let (mut lifecycle, _) = direct_lifecycles();
    lifecycle.state.source_site_execution_count = 1;
    assert!(validate_current_counters(&lifecycle.state).is_err());

    let (mut lifecycle, _) = direct_lifecycles();
    lifecycle.state.heating_on_body_entry_count = 0;
    assert!(validate_current_counters(&lifecycle.state).is_err());

    let (mut lifecycle, _) = direct_lifecycles();
    lifecycle.state.heating_on_guard_false_fallthrough_count = 1;
    assert!(validate_current_counters(&lifecycle.state).is_err());
}

#[test]
fn expected_snapshot_preserves_inactive_nulls_and_active_true_body() {
    for route in [Route::UnitOff, Route::NonCooling, Route::PositiveGuardFalse] {
        let snapshot = expected_snapshot(predecessor_snapshot(route));
        assert!(!snapshot.heating_on_read, "{route:?}");
        assert_eq!(snapshot.heating_on, None, "{route:?}");
        assert!(
            !snapshot.cooling_supply_humidity_ratio_humidification_body_entered,
            "{route:?}"
        );
        assert!(!snapshot.heating_on_guard_false_fallthrough, "{route:?}");
    }

    for route in [
        Route::NoneCase,
        Route::ConstantShr,
        Route::Humidistat,
        Route::ConstantSupplyHumidityRatio,
    ] {
        let snapshot = expected_snapshot(predecessor_snapshot(route));
        assert!(snapshot.heating_on_read, "{route:?}");
        assert_eq!(snapshot.heating_on, Some(true), "{route:?}");
        assert!(
            snapshot.cooling_supply_humidity_ratio_humidification_body_entered,
            "{route:?}"
        );
        assert!(!snapshot.heating_on_guard_false_fallthrough, "{route:?}");
    }
}

#[test]
fn direct_release_and_immediate_cp368_predecessor_are_strict() {
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
    latest.heating_on = Some(false);
    latest.cooling_supply_humidity_ratio_humidification_body_entered = false;
    latest.heating_on_guard_false_fallthrough = true;
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
    forged_predecessor
        .state
        .latest
        .as_mut()
        .expect("latest")
        .dehumidification_control_default_supply_humidity_ratio_case_exited_via_break = true;
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

    let mut forged_false_route = lifecycle;
    forged_false_route.state.heating_on_body_entry_count = 0;
    forged_false_route
        .state
        .heating_on_guard_false_fallthrough_count = 1;
    forged_false_route.state.source_site_execution_count = 1;
    assert!(
        validate_release_state(
            &forged_false_route,
            &predecessor,
            IdealLoadsAirSystemId(0),
            ZoneId(0),
            1,
        )
        .is_err()
    );
}

#[test]
fn latest_direct_route_requires_matching_current_and_predecessor_cumulative_evidence() {
    let (lifecycle, predecessor) = direct_lifecycles();
    for route in [Route::UnitOff, Route::NonCooling, Route::PositiveGuardFalse] {
        let predecessor_latest = predecessor_snapshot(route);
        let mut forged_predecessor = predecessor.clone();
        forged_predecessor.state.latest = Some(predecessor_latest);
        let mut forged_lifecycle = lifecycle.clone();
        forged_lifecycle.state.latest = Some(expected_snapshot(predecessor_latest));

        assert!(
            validate_release_state(
                &forged_lifecycle,
                &forged_predecessor,
                IdealLoadsAirSystemId(0),
                ZoneId(0),
                1,
            )
            .is_err(),
            "{route:?} latest must not pass while aggregates retain only C0 evidence"
        );
    }
}

fn direct_lifecycles() -> (Lifecycle, PredecessorLifecycle) {
    let system = IdealLoadsAirSystemId(0);
    let predecessor_latest = predecessor_snapshot(Route::NoneCase);
    let mut predecessor_state = PredecessorState::new(system);
    predecessor_state.transition_count = 1;
    predecessor_state.dehumidification_control_none_case_completed_skip_count = 1;
    predecessor_state.latest = Some(predecessor_latest);
    let predecessor = PredecessorLifecycle {
        source: PURCHASED_AIR_CALC_COOLING_DEFAULT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_DEFAULT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_FIRST_EXCLUDED_SOURCE,
        state: predecessor_state,
    };

    let mut state = State::new(system);
    state.transition_count = 1;
    state.dehumidification_control_none_case_completed_skip_count = 1;
    state.heating_on_read_count = 1;
    state.heating_on_body_entry_count = 1;
    state.source_site_execution_count = 2;
    state.latest = Some(expected_snapshot(predecessor_latest));
    (
        Lifecycle {
            source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_FIRST_EXCLUDED_SOURCE,
            state,
        },
        predecessor,
    )
}

fn predecessor_snapshot(route: Route) -> PredecessorSnapshot {
    let unit_off = matches!(route, Route::UnitOff);
    let non_cooling = matches!(route, Route::NonCooling);
    let positive_guard_false = matches!(route, Route::PositiveGuardFalse);
    let none_case = matches!(route, Route::NoneCase);
    let constant_shr = matches!(route, Route::ConstantShr);
    let humidistat = matches!(route, Route::Humidistat);
    let constant_supply = matches!(route, Route::ConstantSupplyHumidityRatio);
    let control = match route {
        Route::UnitOff | Route::NonCooling | Route::PositiveGuardFalse => None,
        Route::NoneCase => Some(DehumidificationControlType::None),
        Route::ConstantShr => Some(DehumidificationControlType::ConstantSensibleHeatRatio),
        Route::Humidistat => Some(DehumidificationControlType::Humidistat),
        Route::ConstantSupplyHumidityRatio => {
            Some(DehumidificationControlType::ConstantSupplyHumidityRatio)
        }
    };
    PredecessorSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_DEFAULT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_DEFAULT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_DEFAULT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_SOURCE_ORDER,
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
        predecessor_dehumidification_control_type: control,
        predecessor_dehumidification_control_none_case_completed_skip: none_case,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip:
            constant_shr,
        predecessor_dehumidification_control_humidistat_case_completed_skip: humidistat,
        predecessor_dehumidification_control_constant_supply_humidity_ratio_case_completed_skip:
            constant_supply,
        predecessor_dehumidification_control_default_supply_humidity_ratio_mixed_air_assignment_executed:
            false,
        dehumidification_control_none_case_completed_skip: none_case,
        dehumidification_control_constant_sensible_heat_ratio_case_completed_skip: constant_shr,
        dehumidification_control_humidistat_case_completed_skip: humidistat,
        dehumidification_control_constant_supply_humidity_ratio_case_completed_skip:
            constant_supply,
        dehumidification_control_default_supply_humidity_ratio_case_exited_via_break: false,
    }
}
