use super::*;
use ep_model::{DehumidificationControlType, HumidificationControlType};

#[test]
fn missing_direct_lifecycle_fails_closed() {
    assert!(
        validate_direct_lifecycle(
            None,
            DirectLifecyclePredecessors {
                moisture_demand_assignment_cp372: None,
            },
            None,
            None,
        )
        .is_err()
    );
}

#[test]
fn public_direct_lifecycle_is_a_complete_null_zero_site_assignment_skip() {
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
        state.dehumidification_control_humidistat_supply_humidity_ratio_for_humidification_assignment_count,
        state.dehumidification_control_none_supply_humidity_ratio_for_humidification_assignment_count,
        state.zone_humidifying_setpoint_moisture_demand_read_count,
        state.supply_mass_flow_rate_read_count,
        state.moisture_demand_derived_supply_humidity_ratio_calculation_count,
        state.zone_node_humidity_ratio_read_count,
        state.supply_humidity_ratio_for_humidification_calculation_count,
        state.supply_humidity_ratio_for_humidification_assignment_count,
        state.source_site_execution_count,
    ] {
        assert_eq!(count, 0);
    }
    assert!(state.latest.is_some(), "CP373 latest snapshot must exist");
    let Some(latest) = state.latest else { return };
    for flag in [
        latest.dehumidification_control_humidistat_supply_humidity_ratio_for_humidification_assignment_executed,
        latest.dehumidification_control_none_supply_humidity_ratio_for_humidification_assignment_executed,
        latest.zone_humidifying_setpoint_moisture_demand_read,
        latest.supply_mass_flow_rate_read,
        latest.moisture_demand_derived_supply_humidity_ratio_calculated,
        latest.zone_node_humidity_ratio_read,
        latest.supply_humidity_ratio_for_humidification_calculated,
        latest.supply_humidity_ratio_for_humidification_assigned,
    ] {
        assert!(!flag);
    }
    for value in [
        latest.predecessor_zone_humidifying_setpoint_moisture_demand_kg_per_s,
        latest.predecessor_assigned_zone_humidifying_setpoint_moisture_demand_kg_per_s,
        latest.predecessor_resulting_zone_humidifying_setpoint_moisture_demand_kg_per_s,
        latest.zone_humidifying_setpoint_moisture_demand_kg_per_s,
        latest.supply_mass_flow_rate_kg_per_s,
        latest.moisture_demand_derived_supply_humidity_ratio,
        latest.zone_node_humidity_ratio,
        latest.calculated_supply_humidity_ratio_for_humidification,
        latest.assigned_supply_humidity_ratio_for_humidification,
        latest.resulting_supply_humidity_ratio_for_humidification,
    ] {
        assert!(value.is_none());
    }
}

#[test]
fn six_site_counters_are_exact_and_fail_closed_on_each_mismatch() {
    let corruptions: [fn(&mut State); 7] = [
        |state| state.zone_humidifying_setpoint_moisture_demand_read_count = 1,
        |state| state.supply_mass_flow_rate_read_count = 1,
        |state| state.moisture_demand_derived_supply_humidity_ratio_calculation_count = 1,
        |state| state.zone_node_humidity_ratio_read_count = 1,
        |state| state.supply_humidity_ratio_for_humidification_calculation_count = 1,
        |state| state.supply_humidity_ratio_for_humidification_assignment_count = 1,
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
        "CP373 latest snapshot must exist"
    );
    let Some(latest) = lifecycle.state.latest.as_mut() else {
        return;
    };
    latest.zone_node_humidity_ratio_read = true;
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
fn exact_cp372_predecessor_link_including_ieee_bits_is_required() {
    let (mut lifecycle, mut predecessor) = direct_lifecycles();
    assert!(
        predecessor.state.latest.is_some(),
        "CP372 latest snapshot must exist"
    );
    let Some(predecessor_latest) = predecessor.state.latest.as_mut() else {
        return;
    };
    predecessor_latest.zone_humidifying_setpoint_moisture_demand_kg_per_s =
        Some(f64::from_bits(0x7ff8_0000_0000_0042));
    assert!(
        lifecycle.state.latest.is_some(),
        "CP373 latest snapshot must exist"
    );
    let Some(latest) = lifecycle.state.latest.as_mut() else {
        return;
    };
    latest.predecessor_zone_humidifying_setpoint_moisture_demand_kg_per_s =
        Some(f64::from_bits(0x7ff8_0000_0000_0043));
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
    let predecessor_latest = direct_cp372_snapshot();
    let mut predecessor_state = PredecessorState::new(system);
    predecessor_state.transition_count = 1;
    predecessor_state.humidification_control_guard_false_fallthrough_count = 1;
    predecessor_state.latest = Some(predecessor_latest);
    let predecessor = PredecessorLifecycle {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        state: predecessor_state,
    };

    let mut state = State::new(system);
    state.transition_count = 1;
    state.humidification_control_guard_false_fallthrough_count = 1;
    state.latest = Some(expected_snapshot(predecessor_latest));
    (
        Lifecycle {
            source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_FOR_HUMIDIFICATION_ASSIGNMENT_SOURCE,
            first_excluded_source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_FOR_HUMIDIFICATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
            state,
        },
        predecessor,
    )
}

fn direct_cp372_snapshot() -> PredecessorSnapshot {
    PredecessorSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_SOURCE_ORDER,
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
        humidification_moisture_demand_assignment_executed: false,
        zone_humidifying_setpoint_moisture_demand_read: false,
        zone_humidifying_setpoint_moisture_demand_kg_per_s: None,
        zone_humidifying_setpoint_moisture_demand_assigned: false,
        assigned_zone_humidifying_setpoint_moisture_demand_kg_per_s: None,
        resulting_zone_humidifying_setpoint_moisture_demand_kg_per_s: None,
    }
}
