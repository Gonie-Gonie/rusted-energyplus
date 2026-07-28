mod public_release;
mod release_corruption;

use ep_model::DehumidificationControlType;

use super::*;
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot,
};

#[derive(Clone, Copy)]
enum PredecessorRoute {
    UnitOff,
    NonCooling,
    PositiveGuardFalse,
    CapacityGuardFalse,
    SensibleGuardFalse,
    TemperatureLimited,
}

fn predecessor(
    route: PredecessorRoute,
    ordinal: usize,
) -> PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot {
    let unit_off = matches!(route, PredecessorRoute::UnitOff);
    let non_cooling = matches!(route, PredecessorRoute::NonCooling);
    let positive_false = matches!(route, PredecessorRoute::PositiveGuardFalse);
    let g = matches!(route, PredecessorRoute::CapacityGuardFalse);
    let f = matches!(route, PredecessorRoute::SensibleGuardFalse);
    let l = matches!(route, PredecessorRoute::TemperatureLimited);
    let cooling = positive_false || g || f || l;
    let positive = g || f || l;
    let capacity_body = f || l;
    let active = g || f || l;

    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot {
        source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE_ORDER,
        system: ep_model::IdealLoadsAirSystemId(3),
        parent_call_ordinal: ordinal,
        controlled_zone: ep_model::ZoneId(7),
        unit_body_entered: !unit_off,
        predecessor_cooling_body_entered: cooling,
        predecessor_no_outdoor_air_fallback_entered: cooling,
        predecessor_positive_supply_mass_flow_body_entered: positive,
        predecessor_active_guard_false_fallthrough: positive_false,
        predecessor_capacity_limit_guard_evaluated: positive,
        predecessor_capacity_limit_body_entered: capacity_body,
        predecessor_active_capacity_limit_guard_false_fallthrough: g,
        predecessor_capacity_limit_cp_air_assignment_executed: capacity_body,
        predecessor_capacity_limit_sensible_output_assignment_executed: capacity_body,
        predecessor_capacity_limit_sensible_output_guard_evaluated: capacity_body,
        predecessor_capacity_limit_sensible_output_guard_false_fallthrough: f,
        predecessor_capacity_limit_sensible_output_adjustment_body_entered: l,
        predecessor_capacity_limit_sensible_output_maximum_capacity_assignment_executed: l,
        predecessor_capacity_limit_sensible_output_supply_enthalpy_assignment_executed: l,
        predecessor_capacity_limit_sensible_output_supply_temperature_assignment_executed: l,
        unit_off_skipped: unit_off,
        non_cooling_skipped: non_cooling,
        positive_guard_false_fallthrough_skipped: positive_false,
        capacity_limit_guard_false_fallthrough_skipped: g,
        capacity_limit_sensible_output_guard_false_fallthrough: f,
        capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed: l,
        post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_executed: active,
        mixed_air_humidity_ratio_read: active,
        mixed_air_humidity_ratio: active.then_some(0.008),
        supply_humidity_ratio_assignment_performed: active,
        assigned_supply_humidity_ratio: active.then_some(0.008),
    }
}

fn input(
    selector: DehumidificationControlType,
) -> PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchActiveInput
{
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchActiveInput {
        dehumidification_control_type: selector,
    }
}

#[test]
fn source_boundary_and_exact_two_cp346_sites_are_stable() {
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2209"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2211"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_SOURCE_ORDER,
        [
            "read-purchased-air-dehumidification-control-type",
            "dispatch-dehumidification-control-switch",
        ]
    );
}

#[test]
fn inherited_u_n_p_routes_execute_no_cp346_site() {
    for (index, route) in [
        PredecessorRoute::UnitOff,
        PredecessorRoute::NonCooling,
        PredecessorRoute::PositiveGuardFalse,
    ]
    .into_iter()
    .enumerate()
    {
        let mut state =
            PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchRuntimeState::new(
                ep_model::IdealLoadsAirSystemId(3),
            );
        let snapshot =
            advance_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_state(
                &mut state,
                predecessor(route, index + 1),
                None,
            )
            .expect("valid skipped CP346 transition");
        assert!(!snapshot.dehumidification_control_type_read);
        assert!(snapshot.dehumidification_control_type.is_none());
        assert!(!snapshot.dehumidification_control_switch_dispatched);
        assert_eq!(state.source_site_execution_count, 0);
    }
}

#[test]
fn predecessor_and_selector_activity_mismatch_is_rejected_without_mutation() {
    for (predecessor, active_input) in [
        (
            predecessor(PredecessorRoute::CapacityGuardFalse, 1),
            None,
        ),
        (
            predecessor(PredecessorRoute::UnitOff, 1),
            Some(input(DehumidificationControlType::None)),
        ),
    ] {
        let mut state =
            PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchRuntimeState::new(
                ep_model::IdealLoadsAirSystemId(3),
            );
        let before = state.clone();
        assert!(
            advance_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_state(
                &mut state,
                predecessor,
                active_input,
            )
            .is_none()
        );
        assert_eq!(state, before);
    }
}

#[test]
fn every_typed_selector_maps_by_name_to_its_case_without_discriminant_coupling() {
    for (selector, expected_route) in [
        (
            DehumidificationControlType::None,
            PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchRetainedRoute::DehumidificationControlNoneCaseSelected,
        ),
        (
            DehumidificationControlType::ConstantSensibleHeatRatio,
            PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchRetainedRoute::DehumidificationControlConstantSensibleHeatRatioCaseSelected,
        ),
        (
            DehumidificationControlType::Humidistat,
            PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchRetainedRoute::DehumidificationControlHumidistatCaseSelected,
        ),
        (
            DehumidificationControlType::ConstantSupplyHumidityRatio,
            PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchRetainedRoute::DehumidificationControlConstantSupplyHumidityRatioCaseSelected,
        ),
    ] {
        let mut state =
            PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchRuntimeState::new(
                ep_model::IdealLoadsAirSystemId(3),
            );
        let snapshot =
            advance_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_state(
                &mut state,
                predecessor(PredecessorRoute::CapacityGuardFalse, 1),
                Some(input(selector)),
            )
            .expect("valid active CP346 transition");
        assert_eq!(snapshot.dehumidification_control_type, Some(selector));
        assert_eq!(state.latest_route, Some(expected_route));
        assert_eq!(state.dehumidification_control_switch_count, 1);
        assert_eq!(state.source_site_execution_count, 2);
        assert_eq!(state.dehumidification_control_type_read_count, 1);
        assert_eq!(state.dehumidification_control_switch_dispatch_count, 1);
    }
}

#[test]
fn cumulative_state_obeys_t_and_selector_partitions() {
    let mut state =
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchRuntimeState::new(
            ep_model::IdealLoadsAirSystemId(3),
        );
    for (index, route) in [
        PredecessorRoute::UnitOff,
        PredecessorRoute::NonCooling,
        PredecessorRoute::PositiveGuardFalse,
        PredecessorRoute::CapacityGuardFalse,
        PredecessorRoute::SensibleGuardFalse,
        PredecessorRoute::TemperatureLimited,
    ]
    .into_iter()
    .enumerate()
    {
        let active = matches!(
            route,
            PredecessorRoute::CapacityGuardFalse
                | PredecessorRoute::SensibleGuardFalse
                | PredecessorRoute::TemperatureLimited
        );
        advance_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_state(
            &mut state,
            predecessor(route, index + 1),
            active.then(|| input(DehumidificationControlType::None)),
        )
        .expect("valid cumulative CP346 transition");
    }
    let switch_count = state.dehumidification_control_switch_count;
    assert_eq!(
        state.unit_off_skip_count
            + state.non_cooling_skip_count
            + state.positive_guard_false_fallthrough_skip_count
            + switch_count,
        state.transition_count
    );
    assert_eq!(
        state.dehumidification_control_none_case_selection_count
            + state.dehumidification_control_constant_sensible_heat_ratio_case_selection_count
            + state.dehumidification_control_humidistat_case_selection_count
            + state.dehumidification_control_constant_supply_humidity_ratio_case_selection_count,
        switch_count
    );
    assert_eq!(state.source_site_execution_count, 2 * switch_count);
    assert_eq!(state.dehumidification_control_type_read_count, switch_count);
    assert_eq!(
        state.dehumidification_control_switch_dispatch_count,
        switch_count
    );
}
