pub(in crate::ideal_loads::calc) mod public_release;
mod release_corruption;

use super::*;
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot,
};

#[derive(Clone, Copy)]
enum Route {
    UnitOff,
    NonCooling,
    PositiveGuardFalse,
    CapacityGuardFalse,
    SensibleGuardFalse,
    TemperatureLimited,
}

fn predecessor(
    route: Route,
    ordinal: usize,
) -> PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot
{
    let unit_off = matches!(route, Route::UnitOff);
    let non_cooling = matches!(route, Route::NonCooling);
    let positive_false = matches!(route, Route::PositiveGuardFalse);
    let g = matches!(route, Route::CapacityGuardFalse);
    let f = matches!(route, Route::SensibleGuardFalse);
    let l = matches!(route, Route::TemperatureLimited);
    let cooling = positive_false || g || f || l;
    let positive = g || f || l;
    let capacity_body = f || l;

    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot {
        source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER,
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
        preexisting_supply_temperature_c: None,
        supply_temperature_for_minimum_read: false,
        supply_temperature_before_mixed_air_limit_c: None,
        mixed_air_temperature_for_minimum_read: false,
        mixed_air_temperature_c: None,
        source_shaped_two_argument_minimum_evaluated: false,
        minimum_supply_temperature_c: None,
        supply_temperature_assignment_performed: false,
        assigned_supply_temperature_c: None,
        resulting_supply_temperature_c: None,
    }
}

fn input(
    bits: u64,
) -> PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentActiveInput
{
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentActiveInput {
        mixed_air_humidity_ratio: f64::from_bits(bits),
    }
}

#[test]
fn source_boundary_and_exact_two_cp345_site_labels_are_stable() {
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2208"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2209"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE_ORDER,
        [
            "read-purchased-air-mixed-air-humidity-ratio",
            "assign-purchased-air-supply-humidity-ratio",
        ]
    );
}

#[test]
fn inherited_u_n_p_routes_execute_no_cp345_source_site() {
    for (ordinal, route) in [Route::UnitOff, Route::NonCooling, Route::PositiveGuardFalse]
        .into_iter()
        .enumerate()
    {
        let mut state =
            PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentRuntimeState::new(
                ep_model::IdealLoadsAirSystemId(3),
            );
        let snapshot =
            advance_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_state(
                &mut state,
                predecessor(route, ordinal + 1),
                None,
            );
        assert!(
            cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_snapshot_is_exact_direct_release(
                snapshot
            )
        );
        assert!(!snapshot.post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_executed);
        assert!(snapshot.mixed_air_humidity_ratio.is_none());
        assert!(snapshot.assigned_supply_humidity_ratio.is_none());
        assert_eq!(state.source_site_execution_count, 0);
    }
}

#[test]
fn g_f_l_routes_collapse_to_one_assignment_route_but_retain_provenance_counts() {
    for (route, expected) in [
        (Route::CapacityGuardFalse, [1, 0, 0]),
        (Route::SensibleGuardFalse, [0, 1, 0]),
        (Route::TemperatureLimited, [0, 0, 1]),
    ] {
        let mut state =
            PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentRuntimeState::new(
                ep_model::IdealLoadsAirSystemId(3),
            );
        let snapshot =
            advance_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_state(
                &mut state,
                predecessor(route, 1),
                Some(input(0x3f_84_7a_e1_47_ae_14_7b)),
            );
        assert!(
            cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_snapshot_is_exact_direct_release(
                snapshot
            )
        );
        assert_eq!(
            state.latest_route,
            Some(
                PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentRetainedRoute::SupplyHumidityRatioMixedAirAssigned
            )
        );
        assert_eq!(
            [
                state.assignment_after_capacity_limit_guard_false_fallthrough_count,
                state
                    .assignment_after_capacity_limit_sensible_output_guard_false_fallthrough_count,
                state
                    .assignment_after_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count,
            ],
            expected
        );
        assert_eq!(
            state.post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_count,
            1
        );
        assert_eq!(state.source_site_execution_count, 2);
        assert_eq!(state.mixed_air_humidity_ratio_read_count, 1);
        assert_eq!(state.supply_humidity_ratio_assignment_count, 1);
    }
}

#[test]
fn pure_transition_copies_every_binary64_payload_without_a_new_numeric_gate() {
    for bits in [
        (-0.0_f64).to_bits(),
        f64::INFINITY.to_bits(),
        f64::NEG_INFINITY.to_bits(),
        0x7ff8_0000_0000_0042,
    ] {
        let mut state =
            PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentRuntimeState::new(
                ep_model::IdealLoadsAirSystemId(3),
            );
        let snapshot =
            advance_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_state(
                &mut state,
                predecessor(Route::CapacityGuardFalse, 1),
                Some(input(bits)),
            );
        assert_eq!(
            snapshot.mixed_air_humidity_ratio.map(f64::to_bits),
            Some(bits)
        );
        assert_eq!(
            snapshot.assigned_supply_humidity_ratio.map(f64::to_bits),
            Some(bits)
        );
        assert!(
            cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_snapshot_is_exact_direct_release(
                snapshot
            )
        );
    }
}

#[test]
fn accumulated_state_obeys_t_partition_r_join_and_two_r_sites() {
    let mut state =
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentRuntimeState::new(
            ep_model::IdealLoadsAirSystemId(3),
        );
    for (index, route) in [
        Route::UnitOff,
        Route::NonCooling,
        Route::PositiveGuardFalse,
        Route::CapacityGuardFalse,
        Route::SensibleGuardFalse,
        Route::TemperatureLimited,
    ]
    .into_iter()
    .enumerate()
    {
        let active = matches!(
            route,
            Route::CapacityGuardFalse | Route::SensibleGuardFalse | Route::TemperatureLimited
        );
        advance_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_state(
            &mut state,
            predecessor(route, index + 1),
            active.then(|| input(0x3f_84_7a_e1_47_ae_14_7b)),
        );
    }
    let r = state.post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_count;
    assert_eq!(state.transition_count, 6);
    assert_eq!(
        state.unit_off_skip_count
            + state.non_cooling_skip_count
            + state.positive_guard_false_fallthrough_skip_count
            + r,
        state.transition_count
    );
    assert_eq!(
        state.assignment_after_capacity_limit_guard_false_fallthrough_count
            + state
                .assignment_after_capacity_limit_sensible_output_guard_false_fallthrough_count
            + state
                .assignment_after_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count,
        r
    );
    assert_eq!(state.source_site_execution_count, 2 * r);
    assert_eq!(state.mixed_air_humidity_ratio_read_count, r);
    assert_eq!(state.supply_humidity_ratio_assignment_count, r);
}
