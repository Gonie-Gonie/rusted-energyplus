use ep_model::{
    DehumidificationControlType, HumidificationControlType, IdealLoadsAirSystemId, ZoneId,
};

use super::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardRuntimeState as State,
    advance_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_state as advance,
};
use crate::ideal_loads::calc::{
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardRuntimeState as Cp370State,
    advance_cooling_supply_humidity_ratio_humidification_control_humidistat_guard_state as advance_cp370,
};
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_HUMIDIFICATION_FLOW_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardSnapshot as Predecessor,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardSnapshot as Cp369Snapshot,
};

mod release;

#[test]
fn source_order_is_exact_cp320_short_circuit_slice() {
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_SOURCE_ORDER,
        &PURCHASED_AIR_CALC_COOLING_HUMIDIFICATION_FLOW_SOURCE_ORDER[6..11],
    );
}

#[test]
fn direct_cp370_outer_false_skips_every_cp371_site() {
    let predecessor = active_cp370(
        1,
        DehumidificationControlType::None,
        true,
        HumidificationControlType::None,
    );
    let mut state = State::new(IdealLoadsAirSystemId(0));
    let snapshot = advance(
        &mut state,
        predecessor,
        DehumidificationControlType::None,
    )
    .expect("outer-false route");

    assert!(!snapshot.dehumidification_control_type_first_read);
    assert_eq!(snapshot.first_dehumidification_control_type, None);
    assert_eq!(snapshot.dehumidification_control_type_humidistat, None);
    assert!(!snapshot.dehumidification_control_type_second_read);
    assert_eq!(snapshot.second_dehumidification_control_type, None);
    assert_eq!(snapshot.dehumidification_control_type_none, None);
    assert!(!snapshot.dehumidification_control_body_entered);
    assert!(!snapshot.dehumidification_control_guard_false_fallthrough);
    assert_eq!(state.source_site_execution_count, 0);
}

#[test]
fn none_uses_all_five_sites_and_enters_the_body() {
    let predecessor = active_cp370(
        1,
        DehumidificationControlType::None,
        true,
        HumidificationControlType::Humidistat,
    );
    let mut state = State::new(IdealLoadsAirSystemId(0));
    let snapshot = advance(
        &mut state,
        predecessor,
        DehumidificationControlType::None,
    )
    .expect("None second-disjunct route");

    assert!(snapshot.dehumidification_control_type_first_read);
    assert_eq!(
        snapshot.first_dehumidification_control_type,
        Some(DehumidificationControlType::None),
    );
    assert_eq!(snapshot.dehumidification_control_type_humidistat, Some(false));
    assert!(snapshot.dehumidification_control_type_second_read);
    assert_eq!(
        snapshot.second_dehumidification_control_type,
        Some(DehumidificationControlType::None),
    );
    assert_eq!(snapshot.dehumidification_control_type_none, Some(true));
    assert!(snapshot.dehumidification_control_body_entered);
    assert!(!snapshot.dehumidification_control_guard_false_fallthrough);
    assert_eq!(state.dehumidification_control_type_first_read_count, 1);
    assert_eq!(state.dehumidification_control_type_second_read_count, 1);
    assert_eq!(state.dehumidification_control_type_none_match_count, 1);
    assert_eq!(state.dehumidification_control_body_entry_count, 1);
    assert_eq!(state.source_site_execution_count, 5);
}

#[test]
fn humidistat_short_circuits_after_three_sites() {
    let predecessor = active_cp370(
        1,
        DehumidificationControlType::Humidistat,
        true,
        HumidificationControlType::Humidistat,
    );
    let mut state = State::new(IdealLoadsAirSystemId(0));
    let snapshot = advance(
        &mut state,
        predecessor,
        DehumidificationControlType::Humidistat,
    )
    .expect("Humidistat first-disjunct route");

    assert_eq!(snapshot.dehumidification_control_type_humidistat, Some(true));
    assert!(!snapshot.dehumidification_control_type_second_read);
    assert_eq!(snapshot.second_dehumidification_control_type, None);
    assert_eq!(snapshot.dehumidification_control_type_none, None);
    assert!(snapshot.dehumidification_control_body_entered);
    assert_eq!(state.dehumidification_control_type_humidistat_match_count, 1);
    assert_eq!(state.dehumidification_control_type_second_read_count, 0);
    assert_eq!(state.source_site_execution_count, 3);
}

#[test]
fn rejected_control_enums_use_four_sites_and_fall_through() {
    for control in [
        DehumidificationControlType::ConstantSensibleHeatRatio,
        DehumidificationControlType::ConstantSupplyHumidityRatio,
    ] {
        let predecessor = active_cp370(1, control, true, HumidificationControlType::Humidistat);
        let mut state = State::new(IdealLoadsAirSystemId(0));
        let snapshot = advance(&mut state, predecessor, control).expect("rejected enum route");

        assert_eq!(snapshot.dehumidification_control_type_humidistat, Some(false));
        assert!(snapshot.dehumidification_control_type_second_read);
        assert_eq!(snapshot.dehumidification_control_type_none, Some(false));
        assert!(!snapshot.dehumidification_control_body_entered);
        assert!(snapshot.dehumidification_control_guard_false_fallthrough);
        assert_eq!(state.dehumidification_control_guard_false_fallthrough_count, 1);
        assert_eq!(state.source_site_execution_count, 4);
    }
}

#[test]
fn upstream_skip_routes_skip_every_cp371_site() {
    let predecessors = [
        inactive_cp369(1, true, false, false),
        inactive_cp369(1, false, true, false),
        inactive_cp369(1, false, false, true),
        active_cp369(1, DehumidificationControlType::None, false),
    ];
    for cp369 in predecessors {
        let predecessor = cp370_from(cp369, HumidificationControlType::Humidistat);
        let mut state = State::new(IdealLoadsAirSystemId(0));
        let snapshot = advance(
            &mut state,
            predecessor,
            DehumidificationControlType::None,
        )
        .expect("upstream skip route");
        assert!(!snapshot.dehumidification_control_type_first_read);
        assert!(!snapshot.dehumidification_control_type_second_read);
        assert!(!snapshot.dehumidification_control_body_entered);
        assert!(!snapshot.dehumidification_control_guard_false_fallthrough);
        assert_eq!(state.source_site_execution_count, 0);
    }
}

#[test]
fn malformed_cp370_lineage_is_rejected_without_mutation() {
    let mut predecessor = active_cp370(
        1,
        DehumidificationControlType::None,
        true,
        HumidificationControlType::Humidistat,
    );
    predecessor.humidification_control_type = None;
    let mut state = State::new(IdealLoadsAirSystemId(0));
    let before = state.clone();
    assert!(
        advance(
            &mut state,
            predecessor,
            DehumidificationControlType::None,
        )
        .is_none()
    );
    assert_eq!(state, before);
}

#[test]
fn active_counter_overflow_matrix_is_transactional() {
    let none_predecessor = active_cp370(
        1,
        DehumidificationControlType::None,
        true,
        HumidificationControlType::Humidistat,
    );
    let none_mutators: [fn(&mut State); 12] = [
        |state| state.transition_count = usize::MAX,
        |state| state.dehumidification_control_none_case_completed_skip_count = usize::MAX,
        |state| state.heating_on_read_count = usize::MAX,
        |state| state.humidification_control_body_entry_count = usize::MAX,
        |state| state.dehumidification_control_type_first_read_count = usize::MAX,
        |state| state.dehumidification_control_type_humidistat_comparison_count = usize::MAX,
        |state| state.dehumidification_control_type_second_read_count = usize::MAX,
        |state| state.dehumidification_control_type_none_comparison_count = usize::MAX,
        |state| state.dehumidification_control_type_none_match_count = usize::MAX,
        |state| state.dehumidification_control_body_entry_count = usize::MAX,
        |state| state.source_site_execution_count = usize::MAX - 4,
        |state| state.humidification_control_type_read_count = usize::MAX,
    ];
    for mutate in none_mutators {
        assert_overflow_is_transactional(
            none_predecessor,
            DehumidificationControlType::None,
            mutate,
        );
    }

    let humidistat_predecessor = active_cp370(
        1,
        DehumidificationControlType::Humidistat,
        true,
        HumidificationControlType::Humidistat,
    );
    assert_overflow_is_transactional(
        humidistat_predecessor,
        DehumidificationControlType::Humidistat,
        |state| state.dehumidification_control_type_humidistat_match_count = usize::MAX,
    );

    let rejected_predecessor = active_cp370(
        1,
        DehumidificationControlType::ConstantSensibleHeatRatio,
        true,
        HumidificationControlType::Humidistat,
    );
    assert_overflow_is_transactional(
        rejected_predecessor,
        DehumidificationControlType::ConstantSensibleHeatRatio,
        |state| state.dehumidification_control_guard_false_fallthrough_count = usize::MAX,
    );
}

fn assert_overflow_is_transactional(
    predecessor: Predecessor,
    control: DehumidificationControlType,
    mutate: fn(&mut State),
) {
    let mut state = State::new(IdealLoadsAirSystemId(0));
    mutate(&mut state);
    let before = state.clone();
    assert!(advance(&mut state, predecessor, control).is_none());
    assert_eq!(state, before);
}

fn active_cp370(
    ordinal: usize,
    selector: DehumidificationControlType,
    heating_on: bool,
    humidification_control: HumidificationControlType,
) -> Predecessor {
    cp370_from(
        active_cp369(ordinal, selector, heating_on),
        humidification_control,
    )
}

fn cp370_from(
    predecessor: Cp369Snapshot,
    humidification_control: HumidificationControlType,
) -> Predecessor {
    let mut state = Cp370State::new(predecessor.system);
    advance_cp370(&mut state, predecessor, humidification_control).expect("valid CP370 fixture")
}

fn active_cp369(
    ordinal: usize,
    selector: DehumidificationControlType,
    heating_on: bool,
) -> Cp369Snapshot {
    Cp369Snapshot {
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

fn inactive_cp369(
    ordinal: usize,
    unit_off: bool,
    non_cooling: bool,
    positive_guard_false: bool,
) -> Cp369Snapshot {
    let mut predecessor = active_cp369(ordinal, DehumidificationControlType::None, true);
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