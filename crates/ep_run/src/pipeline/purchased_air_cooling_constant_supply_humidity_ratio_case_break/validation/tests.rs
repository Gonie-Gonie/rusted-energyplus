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
                assignment_cp365: None,
            },
            None,
            None,
        )
        .is_err()
    );
}

#[test]
fn checked_partitions_and_source_counts_fail_closed() {
    let mut state = PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseBreakRuntimeState::new(
        IdealLoadsAirSystemId(0),
    );
    state.unit_off_skip_count = usize::MAX;
    state.non_cooling_skip_count = 1;
    assert!(validate_route_partition(&state).is_err());

    let mut active = PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseBreakRuntimeState::new(
        IdealLoadsAirSystemId(0),
    );
    active.dehumidification_control_constant_supply_humidity_ratio_case_break_count = 1;
    assert!(validate_source_counters(&active).is_err());
}

#[test]
fn expected_snapshot_maps_only_constant_supply_assignment_to_break() {
    let none_case = expected_snapshot(predecessor_snapshot(Route::NoneCase));
    assert!(none_case.dehumidification_control_none_case_completed_skip);
    assert!(
        !none_case.dehumidification_control_constant_supply_humidity_ratio_case_exited_via_break
    );

    let constant_shr = expected_snapshot(predecessor_snapshot(Route::ConstantShr));
    assert!(constant_shr.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip);
    assert!(
        !constant_shr.dehumidification_control_constant_supply_humidity_ratio_case_exited_via_break
    );

    let humidistat = expected_snapshot(predecessor_snapshot(Route::Humidistat));
    assert!(humidistat.dehumidification_control_humidistat_case_completed_skip);
    assert!(
        !humidistat.dehumidification_control_constant_supply_humidity_ratio_case_exited_via_break
    );

    let constant_supply =
        expected_snapshot(predecessor_snapshot(Route::ConstantSupplyHumidityRatio));
    assert!(
        constant_supply
            .dehumidification_control_constant_supply_humidity_ratio_case_exited_via_break
    );
    assert!(!constant_supply.dehumidification_control_none_case_completed_skip);
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
    let latest = corrupted_latest.state.latest.as_mut().expect("latest");
    latest.dehumidification_control_constant_supply_humidity_ratio_case_exited_via_break = true;
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

    let mut numeric_predecessor = predecessor.clone();
    let latest = numeric_predecessor.state.latest.as_mut().expect("latest");
    latest.minimum_cooling_supply_air_humidity_ratio = Some(f64::NAN);
    assert!(
        validate_release_state(
            &lifecycle,
            &numeric_predecessor,
            IdealLoadsAirSystemId(0),
            ZoneId(0),
            1,
        )
        .is_err()
    );

    let mut forged_predecessor = predecessor;
    let latest = forged_predecessor.state.latest.as_mut().expect("latest");
    latest.predecessor_dehumidification_control_none_case_completed_skip = false;
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

fn direct_lifecycles() -> (
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseBreakLifecycleSummary,
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentLifecycleSummary,
) {
    let system = IdealLoadsAirSystemId(0);
    let predecessor_latest = predecessor_snapshot(Route::NoneCase);
    let mut predecessor_state =
        PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentRuntimeState::new(system);
    predecessor_state.transition_count = 1;
    predecessor_state.dehumidification_control_none_case_completed_skip_count = 1;
    predecessor_state.latest = Some(predecessor_latest);
    let predecessor =
        PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentLifecycleSummary {
            source:
                PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
            state: predecessor_state,
        };

    let mut state =
        PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseBreakRuntimeState::new(system);
    state.transition_count = 1;
    state.dehumidification_control_none_case_completed_skip_count = 1;
    state.latest = Some(expected_snapshot(predecessor_latest));
    (
        PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseBreakLifecycleSummary {
            source: PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_FIRST_EXCLUDED_SOURCE,
            state,
        },
        predecessor,
    )
}

fn predecessor_snapshot(
    route: Route,
) -> PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentSnapshot {
    let none_case = matches!(route, Route::NoneCase);
    let constant_shr = matches!(route, Route::ConstantShr);
    let humidistat = matches!(route, Route::Humidistat);
    let constant_supply = matches!(route, Route::ConstantSupplyHumidityRatio);
    let value = constant_supply.then_some(-0.0);
    let control = match route {
        Route::NoneCase => DehumidificationControlType::None,
        Route::ConstantShr => DehumidificationControlType::ConstantSensibleHeatRatio,
        Route::Humidistat => DehumidificationControlType::Humidistat,
        Route::ConstantSupplyHumidityRatio => {
            DehumidificationControlType::ConstantSupplyHumidityRatio
        }
    };
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE_ORDER,
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
        predecessor_dehumidification_control_humidistat_case_completed_skip: humidistat,
        predecessor_dehumidification_control_constant_supply_humidity_ratio_case_entered:
            constant_supply,
        dehumidification_control_none_case_completed_skip: none_case,
        dehumidification_control_constant_sensible_heat_ratio_case_completed_skip: constant_shr,
        dehumidification_control_humidistat_case_completed_skip: humidistat,
        dehumidification_control_constant_supply_humidity_ratio_assignment_executed:
            constant_supply,
        minimum_cooling_supply_air_humidity_ratio_read: constant_supply,
        minimum_cooling_supply_air_humidity_ratio: value,
        supply_humidity_ratio_assigned: constant_supply,
        assigned_supply_humidity_ratio: value,
        resulting_supply_humidity_ratio: value,
    }
}
