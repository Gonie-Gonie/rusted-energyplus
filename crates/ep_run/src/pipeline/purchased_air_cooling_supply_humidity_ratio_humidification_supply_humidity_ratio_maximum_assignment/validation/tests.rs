use super::*;
use ep_model::{DehumidificationControlType, HumidificationControlType};

#[test]
fn missing_direct_lifecycle_fails_closed() {
    assert!(
        validate_direct_lifecycle(
            None,
            DirectLifecyclePredecessors {
                maximum_limit_cp374: None,
            },
            None,
            None,
        )
        .is_err()
    );
}

#[test]
fn public_direct_lifecycle_is_a_complete_null_zero_site_maximum_assignment_skip() {
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
    for count in [
        state.dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count,
        state.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count,
        state.purchased_air_supply_humidity_ratio_for_humidification_supply_maximum_read_count,
        state.supply_humidity_ratio_for_humidification_for_supply_maximum_read_count,
        state.source_shaped_two_argument_maximum_evaluation_count,
        state.purchased_air_supply_humidity_ratio_assignment_count,
        state.source_site_execution_count,
    ] {
        assert_eq!(count, 0);
    }
    assert!(state.latest.is_some(), "CP375 latest snapshot must exist");
    let Some(latest) = state.latest else {
        return;
    };
    for flag in [
        latest
            .dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_executed,
        latest.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_executed,
        latest.purchased_air_supply_humidity_ratio_for_humidification_supply_maximum_read,
        latest.supply_humidity_ratio_for_humidification_for_supply_maximum_read,
        latest.source_shaped_two_argument_maximum_evaluated,
        latest.purchased_air_supply_humidity_ratio_assignment_performed,
    ] {
        assert!(!flag);
    }
    for value in [
        latest.predecessor_resulting_supply_humidity_ratio_for_humidification,
        latest.purchased_air_supply_humidity_ratio_before_humidification_supply_maximum,
        latest.supply_humidity_ratio_for_humidification_for_supply_maximum,
        latest.maximum_supply_humidity_ratio,
        latest.assigned_supply_humidity_ratio,
        latest.resulting_supply_humidity_ratio,
    ] {
        assert!(value.is_none());
    }
}

#[test]
fn four_site_counters_are_exact_and_fail_closed_on_each_mismatch() {
    let corruptions: [fn(&mut State); 5] = [
        |state| {
            state
                .purchased_air_supply_humidity_ratio_for_humidification_supply_maximum_read_count =
                1;
        },
        |state| state.supply_humidity_ratio_for_humidification_for_supply_maximum_read_count = 1,
        |state| state.source_shaped_two_argument_maximum_evaluation_count = 1,
        |state| state.purchased_air_supply_humidity_ratio_assignment_count = 1,
        |state| state.source_site_execution_count = 1,
    ];
    for corrupt in corruptions {
        let (mut lifecycle, predecessor) = direct_lifecycles();
        corrupt(&mut lifecycle.state);
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
}

#[test]
fn partition_snapshot_and_overflow_drift_fail_closed() {
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
    assert!(
        lifecycle.state.latest.is_some(),
        "CP375 latest snapshot must exist"
    );
    let Some(latest) = lifecycle.state.latest.as_mut() else {
        return;
    };
    latest.source_shaped_two_argument_maximum_evaluated = true;
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
fn exact_cp374_predecessor_link_including_ieee_bits_is_required() {
    let (mut lifecycle, mut predecessor) = direct_lifecycles();
    assert!(
        predecessor.state.latest.is_some(),
        "CP374 latest snapshot must exist"
    );
    let Some(predecessor_latest) = predecessor.state.latest.as_mut() else {
        return;
    };
    predecessor_latest.resulting_supply_humidity_ratio_for_humidification =
        Some(f64::from_bits(0x7ff8_0000_0000_0373));
    assert!(
        lifecycle.state.latest.is_some(),
        "CP375 latest snapshot must exist"
    );
    let Some(latest) = lifecycle.state.latest.as_mut() else {
        return;
    };
    latest.predecessor_resulting_supply_humidity_ratio_for_humidification =
        Some(f64::from_bits(0x7ff8_0000_0000_0374));
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
    let predecessor_latest = direct_cp374_snapshot();
    let mut predecessor_state = PredecessorState::new(system);
    predecessor_state.transition_count = 1;
    predecessor_state.humidification_control_guard_false_fallthrough_count = 1;
    predecessor_state.latest = Some(predecessor_latest);
    let predecessor = PredecessorLifecycle {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_FOR_HUMIDIFICATION_MAXIMUM_LIMIT_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_FOR_HUMIDIFICATION_MAXIMUM_LIMIT_FIRST_EXCLUDED_SOURCE,
        state: predecessor_state,
    };

    let mut state = State::new(system);
    state.transition_count = 1;
    state.humidification_control_guard_false_fallthrough_count = 1;
    state.latest = Some(expected_snapshot(predecessor_latest));
    (
        Lifecycle {
            source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_MAXIMUM_ASSIGNMENT_SOURCE,
            first_excluded_source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_MAXIMUM_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
            state,
        },
        predecessor,
    )
}

fn direct_cp374_snapshot() -> PredecessorSnapshot {
    PredecessorSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_FOR_HUMIDIFICATION_MAXIMUM_LIMIT_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_FOR_HUMIDIFICATION_MAXIMUM_LIMIT_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_FOR_HUMIDIFICATION_MAXIMUM_LIMIT_SOURCE_ORDER,
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
        predecessor_dehumidification_control_type_first_read: false,
        predecessor_first_dehumidification_control_type: None,
        predecessor_dehumidification_control_type_humidistat: None,
        predecessor_dehumidification_control_type_second_read: false,
        predecessor_second_dehumidification_control_type: None,
        predecessor_dehumidification_control_type_none: None,
        predecessor_dehumidification_control_body_entered: false,
        predecessor_dehumidification_control_guard_false_fallthrough: false,
        predecessor_dehumidification_control_humidistat_supply_humidity_ratio_for_humidification_assignment_executed: false,
        predecessor_dehumidification_control_none_supply_humidity_ratio_for_humidification_assignment_executed: false,
        predecessor_resulting_supply_humidity_ratio_for_humidification: None,
        dehumidification_control_humidistat_supply_humidity_ratio_for_humidification_maximum_limit_executed: false,
        dehumidification_control_none_supply_humidity_ratio_for_humidification_maximum_limit_executed: false,
        supply_humidity_ratio_for_humidification_for_maximum_limit_minimum_read: false,
        supply_humidity_ratio_for_humidification_before_maximum_limit: None,
        maximum_heating_supply_air_humidity_ratio_for_minimum_read: false,
        maximum_heating_supply_air_humidity_ratio: None,
        source_shaped_two_argument_minimum_evaluated: false,
        minimum_supply_humidity_ratio_for_humidification: None,
        supply_humidity_ratio_for_humidification_assignment_performed: false,
        assigned_supply_humidity_ratio_for_humidification: None,
        resulting_supply_humidity_ratio_for_humidification: None,
    }
}
