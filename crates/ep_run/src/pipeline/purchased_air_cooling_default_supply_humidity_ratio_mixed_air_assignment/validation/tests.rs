use ep_model::{DehumidificationControlType, IdealLoadsAirSystemId, ZoneId};

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
                case_break_cp366: None,
            },
            None,
            None,
        )
        .is_err()
    );
}

#[test]
fn checked_partition_and_all_zero_source_counters_fail_closed() {
    let mut state =
        PurchasedAirCalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentRuntimeState::new(
            IdealLoadsAirSystemId(0),
        );
    state.unit_off_skip_count = usize::MAX;
    state.non_cooling_skip_count = 1;
    assert!(validate_route_partition(&state).is_err());

    let mut read = state.clone();
    read.unit_off_skip_count = 0;
    read.non_cooling_skip_count = 0;
    read.mixed_air_humidity_ratio_read_count = 1;
    assert!(validate_source_counters(&read).is_err());

    let mut assigned =
        PurchasedAirCalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentRuntimeState::new(
            IdealLoadsAirSystemId(0),
        );
    assigned.supply_humidity_ratio_assignment_count = 1;
    assert!(validate_source_counters(&assigned).is_err());

    let mut sourced =
        PurchasedAirCalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentRuntimeState::new(
            IdealLoadsAirSystemId(0),
        );
    sourced.source_site_execution_count = 1;
    assert!(validate_source_counters(&sourced).is_err());
}

#[test]
fn expected_snapshot_preserves_all_typed_routes_and_never_executes_default_assignment() {
    let none_case = expected_snapshot(predecessor_snapshot(Route::NoneCase));
    assert!(none_case.dehumidification_control_none_case_completed_skip);
    assert!(
        !none_case
            .dehumidification_control_default_supply_humidity_ratio_mixed_air_assignment_executed
    );

    let constant_shr = expected_snapshot(predecessor_snapshot(Route::ConstantShr));
    assert!(constant_shr.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip);
    assert!(
        !constant_shr
            .dehumidification_control_default_supply_humidity_ratio_mixed_air_assignment_executed
    );

    let humidistat = expected_snapshot(predecessor_snapshot(Route::Humidistat));
    assert!(humidistat.dehumidification_control_humidistat_case_completed_skip);
    assert!(
        !humidistat
            .dehumidification_control_default_supply_humidity_ratio_mixed_air_assignment_executed
    );

    let constant_supply =
        expected_snapshot(predecessor_snapshot(Route::ConstantSupplyHumidityRatio));
    assert!(
        constant_supply.dehumidification_control_constant_supply_humidity_ratio_case_completed_skip
    );
    assert!(
        !constant_supply
            .dehumidification_control_default_supply_humidity_ratio_mixed_air_assignment_executed
    );
}

#[test]
fn direct_release_and_immediate_cp366_predecessor_are_strict() {
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

    let mut forged_execution = lifecycle.clone();
    forged_execution
        .state
        .latest
        .as_mut()
        .expect("latest")
        .dehumidification_control_default_supply_humidity_ratio_mixed_air_assignment_executed =
        true;
    assert!(
        validate_release_state(
            &forged_execution,
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
        .predecessor_dehumidification_control_none_case_completed_skip = false;
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

    let mut forged_read = lifecycle;
    forged_read.state.mixed_air_humidity_ratio_read_count = 1;
    assert!(
        validate_release_state(
            &forged_read,
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
            "{route:?} latest must not pass while both aggregate states retain only C0 evidence"
        );
    }
}

fn direct_lifecycles() -> (
    PurchasedAirCalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseBreakLifecycleSummary,
) {
    let system = IdealLoadsAirSystemId(0);
    let predecessor_latest = predecessor_snapshot(Route::NoneCase);
    let mut predecessor_state =
        PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseBreakRuntimeState::new(system);
    predecessor_state.transition_count = 1;
    predecessor_state.dehumidification_control_none_case_completed_skip_count = 1;
    predecessor_state.latest = Some(predecessor_latest);
    let predecessor = PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseBreakLifecycleSummary {
        source: PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_FIRST_EXCLUDED_SOURCE,
        state: predecessor_state,
    };

    let mut state =
        PurchasedAirCalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentRuntimeState::new(
            system,
        );
    state.transition_count = 1;
    state.dehumidification_control_none_case_completed_skip_count = 1;
    state.latest = Some(expected_snapshot(predecessor_latest));
    (
        PurchasedAirCalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentLifecycleSummary {
            source:
                PURCHASED_AIR_CALC_COOLING_DEFAULT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_DEFAULT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
            state,
        },
        predecessor,
    )
}

fn predecessor_snapshot(
    route: Route,
) -> PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseBreakSnapshot {
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
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseBreakSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_SOURCE_ORDER,
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
        predecessor_dehumidification_control_constant_supply_humidity_ratio_assignment_executed:
            constant_supply,
        dehumidification_control_none_case_completed_skip: none_case,
        dehumidification_control_constant_sensible_heat_ratio_case_completed_skip: constant_shr,
        dehumidification_control_humidistat_case_completed_skip: humidistat,
        dehumidification_control_constant_supply_humidity_ratio_case_exited_via_break:
            constant_supply,
    }
}
