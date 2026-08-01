use ep_model::{DehumidificationControlType, IdealLoadsAirSystemId, ZoneId};

use super::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardRuntimeState as State,
    advance_cooling_supply_humidity_ratio_humidification_heating_availability_guard_state as advance,
};
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_DEFAULT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_DEFAULT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_SOURCE,
    PURCHASED_AIR_CALC_COOLING_DEFAULT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_SOURCE_ORDER,
    PurchasedAirCalcCoolingDefaultSupplyHumidityRatioCaseBreakSnapshot as Predecessor,
};

mod release;

#[test]
fn active_true_and_false_routes_use_two_site_contract() {
    let mut state = State::new(IdealLoadsAirSystemId(0));
    let entered = advance(
        &mut state,
        active_predecessor(1, DehumidificationControlType::None),
        true,
    )
    .expect("true HeatOn route");
    assert_eq!(
        entered.source_order,
        PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_SOURCE_ORDER
    );
    assert!(entered.heating_on_read);
    assert_eq!(entered.heating_on, Some(true));
    assert!(entered.cooling_supply_humidity_ratio_humidification_body_entered);
    assert!(!entered.heating_on_guard_false_fallthrough);
    assert_eq!(state.heating_on_read_count, 1);
    assert_eq!(state.heating_on_body_entry_count, 1);
    assert_eq!(state.source_site_execution_count, 2);

    let fell_through = advance(
        &mut state,
        active_predecessor(2, DehumidificationControlType::Humidistat),
        false,
    )
    .expect("false HeatOn route");
    assert!(fell_through.heating_on_read);
    assert_eq!(fell_through.heating_on, Some(false));
    assert!(!fell_through.cooling_supply_humidity_ratio_humidification_body_entered);
    assert!(fell_through.heating_on_guard_false_fallthrough);
    assert_eq!(state.heating_on_read_count, 2);
    assert_eq!(state.heating_on_body_entry_count, 1);
    assert_eq!(state.heating_on_guard_false_fallthrough_count, 1);
    assert_eq!(state.source_site_execution_count, 3);
}

#[test]
fn all_four_completed_selector_routes_reach_the_guard() {
    for selector in [
        DehumidificationControlType::None,
        DehumidificationControlType::ConstantSensibleHeatRatio,
        DehumidificationControlType::Humidistat,
        DehumidificationControlType::ConstantSupplyHumidityRatio,
    ] {
        let mut state = State::new(IdealLoadsAirSystemId(0));
        let snapshot = advance(&mut state, active_predecessor(1, selector), true)
            .expect("typed completed route");
        assert!(snapshot.heating_on_read);
        assert_eq!(state.heating_on_read_count, 1);
        assert_eq!(state.heating_on_body_entry_count, 1);
        assert_eq!(state.source_site_execution_count, 2);
    }
}

#[test]
fn inactive_routes_skip_both_sites_and_input_boolean() {
    for mut predecessor in [
        inactive_predecessor(1, true, false, false),
        inactive_predecessor(1, false, true, false),
        inactive_predecessor(1, false, false, true),
    ] {
        for heating_on in [false, true] {
            predecessor.parent_call_ordinal = 1;
            let mut state = State::new(IdealLoadsAirSystemId(0));
            let snapshot = advance(&mut state, predecessor, heating_on).expect("inactive route");
            assert!(!snapshot.heating_on_read);
            assert_eq!(snapshot.heating_on, None);
            assert!(!snapshot.cooling_supply_humidity_ratio_humidification_body_entered);
            assert!(!snapshot.heating_on_guard_false_fallthrough);
            assert_eq!(state.source_site_execution_count, 0);
        }
    }
}

#[test]
fn executed_untyped_default_break_is_rejected() {
    let mut predecessor = active_predecessor(1, DehumidificationControlType::None);
    predecessor.dehumidification_control_default_supply_humidity_ratio_case_exited_via_break = true;
    let mut state = State::new(IdealLoadsAirSystemId(0));
    assert!(advance(&mut state, predecessor, true).is_none());
    assert_eq!(state.transition_count, 0);
}

#[test]
fn active_transition_counter_overflow_matrix_is_nonmutating() {
    let predecessor = active_predecessor(1, DehumidificationControlType::None);
    let mut cases = Vec::new();

    let mut transition = State::new(IdealLoadsAirSystemId(0));
    transition.transition_count = usize::MAX;
    cases.push(transition);

    let mut selector = State::new(IdealLoadsAirSystemId(0));
    selector.dehumidification_control_none_case_completed_skip_count = usize::MAX;
    cases.push(selector);

    let mut read = State::new(IdealLoadsAirSystemId(0));
    read.heating_on_read_count = usize::MAX;
    cases.push(read);

    let mut body = State::new(IdealLoadsAirSystemId(0));
    body.heating_on_body_entry_count = usize::MAX;
    cases.push(body);

    let mut witnessed_body = State::new(IdealLoadsAirSystemId(0));
    witnessed_body.witnessed_heating_on_body_entry_count = usize::MAX;
    cases.push(witnessed_body);

    let mut source = State::new(IdealLoadsAirSystemId(0));
    source.source_site_execution_count = usize::MAX - 1;
    cases.push(source);

    for mut state in cases {
        let before = state.clone();
        assert!(advance(&mut state, predecessor, true).is_none());
        assert_eq!(state, before);
    }
}

fn active_predecessor(
    ordinal: usize,
    selector: DehumidificationControlType,
) -> Predecessor {
    Predecessor {
        source: PURCHASED_AIR_CALC_COOLING_DEFAULT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_DEFAULT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_DEFAULT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_SOURCE_ORDER,
        system: IdealLoadsAirSystemId(0),
        parent_call_ordinal: ordinal,
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
        predecessor_dehumidification_control_default_supply_humidity_ratio_mixed_air_assignment_executed:
            false,
        dehumidification_control_none_case_completed_skip:
            selector == DehumidificationControlType::None,
        dehumidification_control_constant_sensible_heat_ratio_case_completed_skip:
            selector == DehumidificationControlType::ConstantSensibleHeatRatio,
        dehumidification_control_humidistat_case_completed_skip:
            selector == DehumidificationControlType::Humidistat,
        dehumidification_control_constant_supply_humidity_ratio_case_completed_skip:
            selector == DehumidificationControlType::ConstantSupplyHumidityRatio,
        dehumidification_control_default_supply_humidity_ratio_case_exited_via_break: false,
    }
}

fn inactive_predecessor(
    ordinal: usize,
    unit_off: bool,
    non_cooling: bool,
    positive_guard_false: bool,
) -> Predecessor {
    let mut predecessor = active_predecessor(ordinal, DehumidificationControlType::None);
    predecessor.unit_body_entered = !unit_off;
    predecessor.predecessor_cooling_body_entered = positive_guard_false;
    predecessor.predecessor_no_outdoor_air_fallback_entered = positive_guard_false;
    predecessor.predecessor_positive_supply_mass_flow_body_entered = false;
    predecessor.unit_off_skipped = unit_off;
    predecessor.non_cooling_skipped = non_cooling;
    predecessor.positive_guard_false_fallthrough_skipped = positive_guard_false;
    predecessor.predecessor_dehumidification_control_type = None;
    predecessor.predecessor_dehumidification_control_none_case_completed_skip = false;
    predecessor.dehumidification_control_none_case_completed_skip = false;
    predecessor
}