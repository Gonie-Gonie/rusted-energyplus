use ep_model::{
    DehumidificationControlType, HumidificationControlType, IdealLoadsAirSystemId, ZoneId,
};

use super::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentActiveInput as ActiveInput,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentRuntimeState as State,
    advance_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_state as advance,
};
use crate::ideal_loads::calc::{
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardRuntimeState as Cp370State,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardRuntimeState as Cp371State,
    advance_cooling_supply_humidity_ratio_humidification_control_humidistat_guard_state as advance_cp370,
    advance_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_state as advance_cp371,
};
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_HUMIDIFICATION_FLOW_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardSnapshot as Predecessor,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardSnapshot as Cp369Snapshot,
};

mod release;

#[test]
fn source_order_is_exact_cp320_assignment_slice() {
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_SOURCE_ORDER,
        &PURCHASED_AIR_CALC_COOLING_HUMIDIFICATION_FLOW_SOURCE_ORDER[11..13],
    );
}

#[test]
fn outer_false_predecessor_skips_both_sites_without_an_operand() {
    let predecessor = cp371(
        DehumidificationControlType::None,
        HumidificationControlType::None,
    );
    let mut state = State::new(predecessor.system);
    let snapshot = advance(&mut state, predecessor, None).expect("CP372 skip");

    assert!(!snapshot.humidification_moisture_demand_assignment_executed);
    assert!(!snapshot.zone_humidifying_setpoint_moisture_demand_read);
    assert!(!snapshot.zone_humidifying_setpoint_moisture_demand_assigned);
    assert_eq!(
        snapshot.resulting_zone_humidifying_setpoint_moisture_demand_kg_per_s,
        None
    );
    assert_eq!(state.humidification_control_guard_false_fallthrough_count, 1);
    assert_eq!(state.source_site_execution_count, 0);
}

#[test]
fn both_admitted_control_routes_copy_every_ieee_payload_bit_exact() {
    for selector in [
        DehumidificationControlType::None,
        DehumidificationControlType::Humidistat,
    ] {
        for value in [
            -0.0,
            f64::from_bits(0x7ff8_0000_0000_0372),
            f64::INFINITY,
            f64::NEG_INFINITY,
        ] {
            let predecessor = cp371(selector, HumidificationControlType::Humidistat);
            let mut state = State::new(predecessor.system);
            let snapshot = advance(
                &mut state,
                predecessor,
                Some(ActiveInput {
                    zone_humidifying_setpoint_moisture_demand_kg_per_s: value,
                }),
            )
            .expect("admitted CP372 assignment");

            assert!(snapshot.humidification_moisture_demand_assignment_executed);
            assert!(snapshot.zone_humidifying_setpoint_moisture_demand_read);
            assert!(snapshot.zone_humidifying_setpoint_moisture_demand_assigned);
            assert_eq!(
                snapshot
                    .resulting_zone_humidifying_setpoint_moisture_demand_kg_per_s
                    .expect("result")
                    .to_bits(),
                value.to_bits()
            );
            assert_eq!(state.humidification_moisture_demand_assignment_count, 1);
            assert_eq!(state.source_site_execution_count, 2);
        }
    }
}

#[test]
fn rejected_nested_controls_skip_the_assignment() {
    for selector in [
        DehumidificationControlType::ConstantSensibleHeatRatio,
        DehumidificationControlType::ConstantSupplyHumidityRatio,
    ] {
        let predecessor = cp371(selector, HumidificationControlType::Humidistat);
        let mut state = State::new(predecessor.system);
        let snapshot = advance(&mut state, predecessor, None).expect("guard-false CP372");
        assert!(snapshot.predecessor_dehumidification_control_guard_false_fallthrough);
        assert!(!snapshot.humidification_moisture_demand_assignment_executed);
        assert_eq!(state.dehumidification_control_guard_false_fallthrough_count, 1);
        assert_eq!(state.source_site_execution_count, 0);
    }
}

#[test]
fn heating_availability_guard_false_skips_the_assignment() {
    let cp369 = inactive_cp369(DehumidificationControlType::None);
    let mut cp370_state = Cp370State::new(cp369.system);
    let cp370 = advance_cp370(
        &mut cp370_state,
        cp369,
        HumidificationControlType::Humidistat,
    )
    .expect("heating-false CP370 fixture");
    let mut cp371_state = Cp371State::new(cp370.system);
    let predecessor = advance_cp371(
        &mut cp371_state,
        cp370,
        DehumidificationControlType::None,
    )
    .expect("heating-false CP371 fixture");
    let mut state = State::new(predecessor.system);
    let snapshot = advance(&mut state, predecessor, None).expect("heating-false CP372");

    assert!(snapshot.predecessor_heating_on_guard_false_fallthrough);
    assert!(!snapshot.humidification_moisture_demand_assignment_executed);
    assert!(!snapshot.zone_humidifying_setpoint_moisture_demand_read);
    assert!(!snapshot.zone_humidifying_setpoint_moisture_demand_assigned);
    assert_eq!(
        state.heating_availability_guard_false_fallthrough_count,
        1
    );
    assert_eq!(state.source_site_execution_count, 0);
}

#[test]
fn operand_shape_mismatch_is_transactional() {
    let active = cp371(
        DehumidificationControlType::None,
        HumidificationControlType::Humidistat,
    );
    let skipped = cp371(
        DehumidificationControlType::None,
        HumidificationControlType::None,
    );
    let mut state = State::new(active.system);
    let before = state.clone();
    assert!(advance(&mut state, active, None).is_none());
    assert_eq!(state, before);
    assert!(
        advance(
            &mut state,
            skipped,
            Some(ActiveInput {
                zone_humidifying_setpoint_moisture_demand_kg_per_s: 1.0,
            }),
        )
        .is_none()
    );
    assert_eq!(state, before);
}

#[test]
fn active_counter_overflow_is_transactional() {
    let predecessor = cp371(
        DehumidificationControlType::None,
        HumidificationControlType::Humidistat,
    );
    let mutators: [fn(&mut State); 6] = [
        |state| state.transition_count = usize::MAX,
        |state| state.dehumidification_control_none_moisture_demand_assignment_count = usize::MAX,
        |state| state.humidification_moisture_demand_assignment_count = usize::MAX,
        |state| state.source_site_execution_count = usize::MAX - 1,
        |state| state.zone_humidifying_setpoint_moisture_demand_read_count = usize::MAX,
        |state| state.zone_humidifying_setpoint_moisture_demand_assignment_count = usize::MAX,
    ];
    for mutate in mutators {
        let mut state = State::new(predecessor.system);
        mutate(&mut state);
        let before = state.clone();
        assert!(
            advance(
                &mut state,
                predecessor,
                Some(ActiveInput {
                    zone_humidifying_setpoint_moisture_demand_kg_per_s: 0.001,
                }),
            )
            .is_none()
        );
        assert_eq!(state, before);
    }
}

fn cp371(
    selector: DehumidificationControlType,
    humidification_control: HumidificationControlType,
) -> Predecessor {
    let cp369 = active_cp369(selector);
    let mut cp370_state = Cp370State::new(cp369.system);
    let cp370 = advance_cp370(&mut cp370_state, cp369, humidification_control)
        .expect("valid CP370 fixture");
    let mut cp371_state = Cp371State::new(cp370.system);
    advance_cp371(&mut cp371_state, cp370, selector).expect("valid CP371 fixture")
}

fn active_cp369(selector: DehumidificationControlType) -> Cp369Snapshot {
    Cp369Snapshot {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_SOURCE_ORDER,
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
        predecessor_dehumidification_control_type: Some(selector),
        predecessor_dehumidification_control_none_case_completed_skip:
            selector == DehumidificationControlType::None,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip:
            selector == DehumidificationControlType::ConstantSensibleHeatRatio,
        predecessor_dehumidification_control_humidistat_case_completed_skip:
            selector == DehumidificationControlType::Humidistat,
        predecessor_dehumidification_control_constant_supply_humidity_ratio_case_completed_skip:
            selector == DehumidificationControlType::ConstantSupplyHumidityRatio,
        predecessor_dehumidification_control_default_supply_humidity_ratio_case_exited_via_break:
            false,
        dehumidification_control_none_case_completed_skip:
            selector == DehumidificationControlType::None,
        dehumidification_control_constant_sensible_heat_ratio_case_completed_skip:
            selector == DehumidificationControlType::ConstantSensibleHeatRatio,
        dehumidification_control_humidistat_case_completed_skip:
            selector == DehumidificationControlType::Humidistat,
        dehumidification_control_constant_supply_humidity_ratio_case_completed_skip:
            selector == DehumidificationControlType::ConstantSupplyHumidityRatio,
        heating_on_read: true,
        heating_on: Some(true),
        cooling_supply_humidity_ratio_humidification_body_entered: true,
        heating_on_guard_false_fallthrough: false,
    }
}

fn inactive_cp369(selector: DehumidificationControlType) -> Cp369Snapshot {
    let mut snapshot = active_cp369(selector);
    snapshot.heating_on = Some(false);
    snapshot.cooling_supply_humidity_ratio_humidification_body_entered = false;
    snapshot.heating_on_guard_false_fallthrough = true;
    snapshot
}
