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
                case_break_cp363: None,
            },
            None,
            None,
        )
        .is_err()
    );
}

#[test]
fn checked_partitions_and_source_counts_fail_closed() {
    let mut state = PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntryRuntimeState::new(
        IdealLoadsAirSystemId(0),
    );
    state.unit_off_skip_count = usize::MAX;
    state.non_cooling_skip_count = 1;
    assert!(validate_route_partition(&state).is_err());

    let mut active = PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntryRuntimeState::new(
        IdealLoadsAirSystemId(0),
    );
    active.dehumidification_control_constant_supply_humidity_ratio_case_entry_count = 1;
    assert!(validate_source_counters(&active).is_err());
}

#[test]
fn expected_snapshot_maps_only_constant_supply_to_entry() {
    let none_case = expected_snapshot(predecessor_snapshot(Route::NoneCase));
    assert!(none_case.dehumidification_control_none_case_completed_skip);
    assert!(!none_case.dehumidification_control_constant_supply_humidity_ratio_case_entered);

    let constant_shr = expected_snapshot(predecessor_snapshot(Route::ConstantShr));
    assert!(constant_shr.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip);
    assert!(!constant_shr.dehumidification_control_constant_supply_humidity_ratio_case_entered);

    let humidistat = expected_snapshot(predecessor_snapshot(Route::Humidistat));
    assert!(humidistat.dehumidification_control_humidistat_case_completed_skip);
    assert!(!humidistat.dehumidification_control_constant_supply_humidity_ratio_case_entered);

    let constant_supply =
        expected_snapshot(predecessor_snapshot(Route::ConstantSupplyHumidityRatio));
    assert!(constant_supply.dehumidification_control_constant_supply_humidity_ratio_case_entered);
    assert!(!constant_supply.dehumidification_control_humidistat_case_completed_skip);
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
    latest.dehumidification_control_constant_supply_humidity_ratio_case_entered = true;
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

    let mut corrupted_selector = predecessor.clone();
    let Some(latest) = corrupted_selector.state.latest.as_mut() else {
        return;
    };
    latest.predecessor_dehumidification_control_type =
        Some(DehumidificationControlType::ConstantSupplyHumidityRatio);
    let forged_predecessor_latest = *latest;
    let mut corrupted_selector_lifecycle = lifecycle.clone();
    corrupted_selector_lifecycle.state.latest = Some(expected_snapshot(forged_predecessor_latest));
    assert!(
        validate_release_state(
            &corrupted_selector_lifecycle,
            &corrupted_selector,
            IdealLoadsAirSystemId(0),
            ZoneId(0),
            1,
        )
        .is_err()
    );

    let mut corrupted_prefix = predecessor.clone();
    let Some(latest) = corrupted_prefix.state.latest.as_mut() else {
        return;
    };
    latest.predecessor_positive_supply_mass_flow_body_entered = false;
    let forged_predecessor_latest = *latest;
    let mut corrupted_prefix_lifecycle = lifecycle.clone();
    corrupted_prefix_lifecycle.state.latest = Some(expected_snapshot(forged_predecessor_latest));
    assert!(
        validate_release_state(
            &corrupted_prefix_lifecycle,
            &corrupted_prefix,
            IdealLoadsAirSystemId(0),
            ZoneId(0),
            1,
        )
        .is_err()
    );

    let mut corrupted_lineage = predecessor.clone();
    let Some(latest) = corrupted_lineage.state.latest.as_mut() else {
        return;
    };
    latest.predecessor_dehumidification_control_none_case_completed_skip = false;
    let forged_predecessor_latest = *latest;
    let mut corrupted_lineage_lifecycle = lifecycle.clone();
    corrupted_lineage_lifecycle.state.latest = Some(expected_snapshot(forged_predecessor_latest));
    assert!(
        validate_release_state(
            &corrupted_lineage_lifecycle,
            &corrupted_lineage,
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
    latest.dehumidification_control_humidistat_case_exited_via_break = true;
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
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntryLifecycleSummary,
    PurchasedAirCalcCoolingHumidistatCaseBreakLifecycleSummary,
) {
    let system = IdealLoadsAirSystemId(0);
    let predecessor_latest = predecessor_snapshot(Route::NoneCase);
    let mut predecessor_state = PurchasedAirCalcCoolingHumidistatCaseBreakRuntimeState::new(system);
    predecessor_state.transition_count = 1;
    predecessor_state.dehumidification_control_none_case_completed_skip_count = 1;
    predecessor_state.latest = Some(predecessor_latest);
    let predecessor = PurchasedAirCalcCoolingHumidistatCaseBreakLifecycleSummary {
        source: PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_CASE_BREAK_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_CASE_BREAK_FIRST_EXCLUDED_SOURCE,
        state: predecessor_state,
    };

    let mut state =
        PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntryRuntimeState::new(system);
    state.transition_count = 1;
    state.dehumidification_control_none_case_completed_skip_count = 1;
    state.latest = Some(expected_snapshot(predecessor_latest));
    (
        PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntryLifecycleSummary {
            source:
                PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_ENTRY_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_ENTRY_FIRST_EXCLUDED_SOURCE,
            state,
        },
        predecessor,
    )
}

fn predecessor_snapshot(route: Route) -> PurchasedAirCalcCoolingHumidistatCaseBreakSnapshot {
    let none_case = matches!(route, Route::NoneCase);
    let constant_shr = matches!(route, Route::ConstantShr);
    let humidistat = matches!(route, Route::Humidistat);
    let constant_supply = matches!(route, Route::ConstantSupplyHumidityRatio);
    let control = match route {
        Route::NoneCase => DehumidificationControlType::None,
        Route::ConstantShr => DehumidificationControlType::ConstantSensibleHeatRatio,
        Route::Humidistat => DehumidificationControlType::Humidistat,
        Route::ConstantSupplyHumidityRatio => {
            DehumidificationControlType::ConstantSupplyHumidityRatio
        }
    };
    PurchasedAirCalcCoolingHumidistatCaseBreakSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_CASE_BREAK_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_CASE_BREAK_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_CASE_BREAK_SOURCE_ORDER,
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
        predecessor_dehumidification_control_humidistat_supply_humidity_ratio_mixed_air_limit_executed:
            humidistat,
        predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip:
            constant_supply,
        dehumidification_control_none_case_completed_skip: none_case,
        dehumidification_control_constant_sensible_heat_ratio_case_completed_skip: constant_shr,
        dehumidification_control_humidistat_case_exited_via_break: humidistat,
        dehumidification_control_constant_supply_humidity_ratio_case_selected_skip:
            constant_supply,
    }
}
