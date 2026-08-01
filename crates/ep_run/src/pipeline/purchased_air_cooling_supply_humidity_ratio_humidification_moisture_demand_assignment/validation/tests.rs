use super::*;
use ep_model::{DehumidificationControlType, HumidificationControlType};

#[test]
fn missing_direct_lifecycle_fails_closed() {
    assert!(
        validate_direct_lifecycle(
            None,
            DirectLifecyclePredecessors {
                dehumidification_control_guard_cp371: None,
            },
            None,
            None,
        )
        .is_err()
    );
}

#[test]
fn public_direct_lifecycle_is_a_null_zero_site_assignment_skip() {
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
    assert_eq!(state.humidification_moisture_demand_assignment_count, 0);
    assert_eq!(
        state.zone_humidifying_setpoint_moisture_demand_read_count,
        0
    );
    assert_eq!(
        state.zone_humidifying_setpoint_moisture_demand_assignment_count,
        0
    );
    assert_eq!(state.source_site_execution_count, 0);
    assert!(state.latest.is_some(), "CP372 latest");
    let Some(latest) = state.latest else {
        return;
    };
    assert!(!latest.humidification_moisture_demand_assignment_executed);
    assert!(
        latest
            .zone_humidifying_setpoint_moisture_demand_kg_per_s
            .is_none()
    );
    assert!(
        latest
            .assigned_zone_humidifying_setpoint_moisture_demand_kg_per_s
            .is_none()
    );
    assert!(
        latest
            .resulting_zone_humidifying_setpoint_moisture_demand_kg_per_s
            .is_none()
    );
}

#[test]
fn counter_snapshot_and_overflow_drift_fail_closed() {
    let (mut lifecycle, predecessor) = direct_lifecycles();
    lifecycle.state.source_site_execution_count = 1;
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
    lifecycle.state.unit_off_skip_count = usize::MAX;
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
    assert!(lifecycle.state.latest.is_some(), "CP372 latest");
    let Some(latest) = lifecycle.state.latest.as_mut() else {
        return;
    };
    latest.zone_humidifying_setpoint_moisture_demand_read = true;
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
fn exact_cp371_predecessor_link_is_required() {
    let (lifecycle, mut predecessor) = direct_lifecycles();
    assert!(predecessor.state.latest.is_some(), "CP371 latest");
    let Some(latest) = predecessor.state.latest.as_mut() else {
        return;
    };
    latest.predecessor_humidification_control_guard_false_fallthrough = false;
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

fn direct_lifecycles() -> (Lifecycle, PredecessorLifecycle) {
    let system = IdealLoadsAirSystemId(0);
    let predecessor_latest = direct_cp371_snapshot();
    let mut predecessor_state = PredecessorState::new(system);
    predecessor_state.transition_count = 1;
    predecessor_state.dehumidification_control_none_case_completed_skip_count = 1;
    predecessor_state.heating_on_read_count = 1;
    predecessor_state.heating_on_body_entry_count = 1;
    predecessor_state.humidification_control_type_read_count = 1;
    predecessor_state.humidification_control_type_humidistat_comparison_count = 1;
    predecessor_state.humidification_control_guard_false_fallthrough_count = 1;
    predecessor_state.latest = Some(predecessor_latest);
    let predecessor = PredecessorLifecycle {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_FIRST_EXCLUDED_SOURCE,
        state: predecessor_state,
    };

    let mut state = State::new(system);
    state.transition_count = 1;
    state.humidification_control_guard_false_fallthrough_count = 1;
    state.latest = Some(expected_snapshot(predecessor_latest));
    (
        Lifecycle {
            source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_SOURCE,
            first_excluded_source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
            state,
        },
        predecessor,
    )
}

fn direct_cp371_snapshot() -> PredecessorSnapshot {
    PredecessorSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_SOURCE_ORDER,
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
        predecessor_dehumidification_control_type: Some(DehumidificationControlType::None),
        predecessor_dehumidification_control_none_case_completed_skip: true,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip: false,
        predecessor_dehumidification_control_humidistat_case_completed_skip: false,
        predecessor_dehumidification_control_constant_supply_humidity_ratio_case_completed_skip: false,
        predecessor_dehumidification_control_default_supply_humidity_ratio_case_exited_via_break: false,
        dehumidification_control_none_case_completed_skip: true,
        dehumidification_control_constant_sensible_heat_ratio_case_completed_skip: false,
        dehumidification_control_humidistat_case_completed_skip: false,
        dehumidification_control_constant_supply_humidity_ratio_case_completed_skip: false,
        predecessor_heating_on_read: true,
        predecessor_heating_on: Some(true),
        predecessor_cooling_supply_humidity_ratio_humidification_body_entered: true,
        predecessor_heating_on_guard_false_fallthrough: false,
        predecessor_humidification_control_type_read: true,
        predecessor_humidification_control_type: Some(HumidificationControlType::None),
        predecessor_humidification_control_type_humidistat: Some(false),
        predecessor_humidification_control_body_entered: false,
        predecessor_humidification_control_guard_false_fallthrough: true,
        dehumidification_control_type_first_read: false,
        first_dehumidification_control_type: None,
        dehumidification_control_type_humidistat: None,
        dehumidification_control_type_second_read: false,
        second_dehumidification_control_type: None,
        dehumidification_control_type_none: None,
        dehumidification_control_body_entered: false,
        dehumidification_control_guard_false_fallthrough: false,
    }
}
