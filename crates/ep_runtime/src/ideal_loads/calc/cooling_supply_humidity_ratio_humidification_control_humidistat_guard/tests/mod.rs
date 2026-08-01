use ep_model::{
    DehumidificationControlType, HumidificationControlType, IdealLoadsAirSystemId, ZoneId,
};

use super::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_CONTROL_HUMIDISTAT_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardRuntimeState as State,
    advance_cooling_supply_humidity_ratio_humidification_control_humidistat_guard_state as advance,
};
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardSnapshot as Predecessor,
};

mod release;

#[test]
fn none_false_and_humidistat_true_use_dynamic_three_site_contract() {
    let mut state = State::new(IdealLoadsAirSystemId(0));
    let direct = advance(
        &mut state,
        active_predecessor(1, DehumidificationControlType::None, true),
        HumidificationControlType::None,
    )
    .expect("direct None route");
    assert_eq!(
        direct.source_order,
        PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_CONTROL_HUMIDISTAT_GUARD_SOURCE_ORDER
    );
    assert!(direct.humidification_control_type_read);
    assert_eq!(
        direct.humidification_control_type,
        Some(HumidificationControlType::None)
    );
    assert_eq!(direct.humidification_control_type_humidistat, Some(false));
    assert!(!direct.humidification_control_body_entered);
    assert!(direct.humidification_control_guard_false_fallthrough);
    assert_eq!(state.source_site_execution_count, 2);

    let private = advance(
        &mut state,
        active_predecessor(2, DehumidificationControlType::Humidistat, true),
        HumidificationControlType::Humidistat,
    )
    .expect("private Humidistat route");
    assert_eq!(private.humidification_control_type_humidistat, Some(true));
    assert!(private.humidification_control_body_entered);
    assert!(!private.humidification_control_guard_false_fallthrough);
    assert_eq!(state.humidification_control_type_read_count, 2);
    assert_eq!(
        state.humidification_control_type_humidistat_comparison_count,
        2
    );
    assert_eq!(state.humidification_control_body_entry_count, 1);
    assert_eq!(
        state.humidification_control_guard_false_fallthrough_count,
        1
    );
    assert_eq!(state.source_site_execution_count, 5);
}

#[test]
fn all_cp369_selector_routes_evaluate_the_control_only_after_heat_entry() {
    for selector in [
        DehumidificationControlType::None,
        DehumidificationControlType::ConstantSensibleHeatRatio,
        DehumidificationControlType::Humidistat,
        DehumidificationControlType::ConstantSupplyHumidityRatio,
    ] {
        let mut state = State::new(IdealLoadsAirSystemId(0));
        let snapshot = advance(
            &mut state,
            active_predecessor(1, selector, true),
            HumidificationControlType::ConstantSupplyHumidityRatio,
        )
        .expect("typed completed route");
        assert!(snapshot.humidification_control_type_read);
        assert_eq!(snapshot.humidification_control_type_humidistat, Some(false));
        assert_eq!(state.source_site_execution_count, 2);
    }
}

#[test]
fn u_n_p_and_cp369_heat_false_skip_every_cp370_site() {
    let predecessors = [
        inactive_predecessor(1, true, false, false),
        inactive_predecessor(1, false, true, false),
        inactive_predecessor(1, false, false, true),
        active_predecessor(1, DehumidificationControlType::None, false),
    ];
    for predecessor in predecessors {
        for control in [
            HumidificationControlType::None,
            HumidificationControlType::Humidistat,
        ] {
            let mut state = State::new(IdealLoadsAirSystemId(0));
            let snapshot = advance(&mut state, predecessor, control).expect("skipped route");
            assert!(!snapshot.humidification_control_type_read);
            assert_eq!(snapshot.humidification_control_type, None);
            assert_eq!(snapshot.humidification_control_type_humidistat, None);
            assert!(!snapshot.humidification_control_body_entered);
            assert!(!snapshot.humidification_control_guard_false_fallthrough);
            assert_eq!(state.source_site_execution_count, 0);
        }
    }
}

#[test]
fn malformed_cp369_lineage_is_rejected_without_mutation() {
    let mut predecessor = active_predecessor(1, DehumidificationControlType::None, true);
    predecessor.heating_on = None;
    let mut state = State::new(IdealLoadsAirSystemId(0));
    let before = state.clone();
    assert!(advance(&mut state, predecessor, HumidificationControlType::None).is_none());
    assert_eq!(state, before);
}

#[test]
fn active_counter_overflow_matrix_is_transactional() {
    let predecessor = active_predecessor(1, DehumidificationControlType::None, true);
    let mutators: [fn(&mut State); 6] = [
        |state| state.transition_count = usize::MAX,
        |state| state.humidification_control_type_read_count = usize::MAX,
        |state| {
            state.humidification_control_type_humidistat_comparison_count = usize::MAX;
        },
        |state| state.humidification_control_body_entry_count = usize::MAX,
        |state| state.witnessed_humidification_control_body_entry_count = usize::MAX,
        |state| state.source_site_execution_count = usize::MAX - 2,
    ];
    for mutate in mutators {
        let mut state = State::new(IdealLoadsAirSystemId(0));
        mutate(&mut state);
        let before = state.clone();
        assert!(
            advance(
                &mut state,
                predecessor,
                HumidificationControlType::Humidistat,
            )
            .is_none()
        );
        assert_eq!(state, before);
    }
}

fn active_predecessor(
    ordinal: usize,
    selector: DehumidificationControlType,
    heating_on: bool,
) -> Predecessor {
    Predecessor {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_SOURCE_ORDER,
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
        heating_on: Some(heating_on),
        cooling_supply_humidity_ratio_humidification_body_entered: heating_on,
        heating_on_guard_false_fallthrough: !heating_on,
    }
}

fn inactive_predecessor(
    ordinal: usize,
    unit_off: bool,
    non_cooling: bool,
    positive_guard_false: bool,
) -> Predecessor {
    let mut predecessor = active_predecessor(ordinal, DehumidificationControlType::None, true);
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
    predecessor.heating_on_read = false;
    predecessor.heating_on = None;
    predecessor.cooling_supply_humidity_ratio_humidification_body_entered = false;
    predecessor.heating_on_guard_false_fallthrough = false;
    predecessor
}
