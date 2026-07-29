use ep_model::{DehumidificationControlType, IdealLoadsAirSystemId, ZoneId};

use super::*;

#[derive(Clone, Copy)]
enum Route {
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
                mixed_air_limit_cp362: None,
            },
            None,
            None,
        )
        .is_err()
    );
}

#[test]
fn checked_partitions_and_source_counts_fail_closed() {
    let mut state =
        PurchasedAirCalcCoolingHumidistatCaseBreakRuntimeState::new(IdealLoadsAirSystemId(0));
    state.unit_off_skip_count = usize::MAX;
    state.non_cooling_skip_count = 1;
    assert!(validate_route_partition(&state).is_err());

    let mut active =
        PurchasedAirCalcCoolingHumidistatCaseBreakRuntimeState::new(IdealLoadsAirSystemId(0));
    active.dehumidification_control_humidistat_case_break_count = 1;
    assert!(validate_source_counters(&active).is_err());
}

#[test]
fn expected_snapshot_maps_only_humidistat_to_break() {
    let none_case = expected_snapshot(predecessor_snapshot(Route::NoneCase));
    assert!(none_case.dehumidification_control_none_case_completed_skip);
    assert!(!none_case.dehumidification_control_humidistat_case_exited_via_break);

    let constant_shr = expected_snapshot(predecessor_snapshot(Route::ConstantShr));
    assert!(constant_shr.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip);
    assert!(!constant_shr.dehumidification_control_humidistat_case_exited_via_break);

    let humidistat = expected_snapshot(predecessor_snapshot(Route::Humidistat));
    assert!(humidistat.dehumidification_control_humidistat_case_exited_via_break);
    assert!(!humidistat.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip);

    let constant_supply =
        expected_snapshot(predecessor_snapshot(Route::ConstantSupplyHumidityRatio));
    assert!(
        constant_supply.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip
    );
    assert!(!constant_supply.dehumidification_control_humidistat_case_exited_via_break);
}

#[test]
fn direct_release_and_immediate_predecessor_are_strict() {
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

    let mut corrupted_latest = lifecycle.clone();
    let Some(latest) = corrupted_latest.state.latest.as_mut() else {
        return;
    };
    latest.dehumidification_control_humidistat_case_exited_via_break = true;
    assert!(
        validate_release_state(
            &corrupted_latest,
            &predecessor,
            IdealLoadsAirSystemId(0),
            ZoneId(0),
            1,
        )
        .is_err()
    );

    let mut corrupted_predecessor = predecessor;
    let Some(latest) = corrupted_predecessor.state.latest.as_mut() else {
        return;
    };
    latest.mixed_air_humidity_ratio = Some(0.0);
    assert!(
        validate_release_state(
            &lifecycle,
            &corrupted_predecessor,
            IdealLoadsAirSystemId(0),
            ZoneId(0),
            1,
        )
        .is_err()
    );
}

fn direct_lifecycles() -> (
    PurchasedAirCalcCoolingHumidistatCaseBreakLifecycleSummary,
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitLifecycleSummary,
) {
    let system = IdealLoadsAirSystemId(0);
    let predecessor_latest = predecessor_snapshot(Route::NoneCase);
    let mut predecessor_state =
        PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitRuntimeState::new(system);
    predecessor_state.transition_count = 1;
    predecessor_state.dehumidification_control_none_case_completed_skip_count = 1;
    predecessor_state.latest = Some(predecessor_latest);
    let predecessor =
        PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitLifecycleSummary {
            source:
                PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
            state: predecessor_state,
        };

    let mut state = PurchasedAirCalcCoolingHumidistatCaseBreakRuntimeState::new(system);
    state.transition_count = 1;
    state.dehumidification_control_none_case_completed_skip_count = 1;
    state.latest = Some(expected_snapshot(predecessor_latest));
    (
        PurchasedAirCalcCoolingHumidistatCaseBreakLifecycleSummary {
            source: PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_CASE_BREAK_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_CASE_BREAK_FIRST_EXCLUDED_SOURCE,
            state,
        },
        predecessor,
    )
}

fn predecessor_snapshot(
    route: Route,
) -> PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitSnapshot {
    let none_case = matches!(route, Route::NoneCase);
    let constant_shr = matches!(route, Route::ConstantShr);
    let humidistat = matches!(route, Route::Humidistat);
    let constant_supply = matches!(route, Route::ConstantSupplyHumidityRatio);
    let scalar = humidistat.then_some(0.007);
    let control = match route {
        Route::NoneCase => DehumidificationControlType::None,
        Route::ConstantShr => DehumidificationControlType::ConstantSensibleHeatRatio,
        Route::Humidistat => DehumidificationControlType::Humidistat,
        Route::ConstantSupplyHumidityRatio => {
            DehumidificationControlType::ConstantSupplyHumidityRatio
        }
    };
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitSnapshot {
        source:
            PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_SOURCE_ORDER,
        system: IdealLoadsAirSystemId(0),
        parent_call_ordinal: 1,
        controlled_zone: ZoneId(0),
        unit_body_entered: true,
        predecessor_cooling_body_entered: true,
        predecessor_no_outdoor_air_fallback_entered: true,
        predecessor_positive_supply_mass_flow_body_entered: true,
        unit_off_skipped: false,
        non_cooling_skipped: false,
        positive_guard_false_fallthrough_skipped: false,
        predecessor_dehumidification_control_type: Some(control),
        predecessor_dehumidification_control_none_case_completed_skip: none_case,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip:
            constant_shr,
        predecessor_dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_executed:
            humidistat,
        predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip:
            constant_supply,
        predecessor_resulting_supply_humidity_ratio_for_dehumidification: scalar,
        dehumidification_control_none_case_completed_skip: none_case,
        dehumidification_control_constant_sensible_heat_ratio_case_completed_skip: constant_shr,
        dehumidification_control_humidistat_supply_humidity_ratio_mixed_air_limit_executed:
            humidistat,
        dehumidification_control_constant_supply_humidity_ratio_case_selected_skip: constant_supply,
        mixed_air_humidity_ratio_for_minimum_read: humidistat,
        mixed_air_humidity_ratio: scalar,
        supply_humidity_ratio_for_dehumidification_for_mixed_air_limit_minimum_read: humidistat,
        supply_humidity_ratio_for_dehumidification_before_mixed_air_limit: scalar,
        source_shaped_two_argument_minimum_evaluated: humidistat,
        minimum_supply_humidity_ratio: scalar,
        supply_humidity_ratio_assignment_performed: humidistat,
        assigned_supply_humidity_ratio: scalar,
        resulting_supply_humidity_ratio: scalar,
    }
}
