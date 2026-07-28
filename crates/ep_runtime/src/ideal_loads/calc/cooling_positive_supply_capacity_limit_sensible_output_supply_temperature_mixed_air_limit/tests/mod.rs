mod public_release;
mod release_corruption;

use super::*;
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentSnapshot,
};

#[derive(Clone, Copy)]
enum Route {
    UnitOff,
    NonCooling,
    PositiveGuardFalse,
    CapacityGuardFalse,
    SensibleGuardFalse,
    Limited,
}

fn predecessor(
    route: Route,
    ordinal: usize,
    resulting_supply_temperature_c: f64,
) -> PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentSnapshot
{
    let unit_off = matches!(route, Route::UnitOff);
    let non_cooling = matches!(route, Route::NonCooling);
    let positive_false = matches!(route, Route::PositiveGuardFalse);
    let capacity_false = matches!(route, Route::CapacityGuardFalse);
    let guard_false = matches!(route, Route::SensibleGuardFalse);
    let assigned = matches!(route, Route::Limited);
    let cooling = positive_false || capacity_false || guard_false || assigned;
    let positive_body = capacity_false || guard_false || assigned;
    let capacity_body = guard_false || assigned;

    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentSnapshot {
        source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER,
        system: ep_model::IdealLoadsAirSystemId(3),
        parent_call_ordinal: ordinal,
        controlled_zone: ep_model::ZoneId(4),
        unit_body_entered: !unit_off,
        predecessor_cooling_body_entered: cooling,
        predecessor_no_outdoor_air_fallback_entered: cooling,
        predecessor_positive_supply_mass_flow_body_entered: positive_body,
        predecessor_active_guard_false_fallthrough: positive_false,
        predecessor_capacity_limit_guard_evaluated: positive_body,
        predecessor_capacity_limit_body_entered: capacity_body,
        predecessor_active_capacity_limit_guard_false_fallthrough: capacity_false,
        predecessor_capacity_limit_cp_air_assignment_executed: capacity_body,
        predecessor_capacity_limit_sensible_output_assignment_executed: capacity_body,
        predecessor_capacity_limit_sensible_output_guard_evaluated: capacity_body,
        predecessor_capacity_limit_sensible_output_guard_false_fallthrough: guard_false,
        predecessor_capacity_limit_sensible_output_adjustment_body_entered: assigned,
        predecessor_capacity_limit_sensible_output_maximum_capacity_assignment_executed: assigned,
        predecessor_capacity_limit_sensible_output_supply_enthalpy_assignment_executed: assigned,
        unit_off_skipped: unit_off,
        non_cooling_skipped: non_cooling,
        positive_guard_false_fallthrough_skipped: positive_false,
        capacity_limit_guard_false_fallthrough_skipped: capacity_false,
        capacity_limit_sensible_output_guard_false_fallthrough: guard_false,
        capacity_limit_sensible_output_supply_temperature_assignment_executed: assigned,
        preexisting_supply_temperature_c: capacity_body.then_some(24.0),
        supply_enthalpy_for_dry_bulb_inversion_read: assigned,
        supply_enthalpy_j_per_kg: assigned.then_some(37_000.0),
        supply_humidity_ratio_for_dry_bulb_inversion_read: assigned,
        supply_humidity_ratio: assigned.then_some(0.008),
        psychrometric_supply_temperature_evaluated: assigned,
        psychrometric_supply_temperature_result_c: assigned
            .then_some(resulting_supply_temperature_c),
        supply_temperature_assigned: assigned,
        assigned_supply_temperature_c: assigned.then_some(resulting_supply_temperature_c),
        resulting_supply_temperature_c: capacity_body
            .then_some(resulting_supply_temperature_c),
    }
}

fn retained_input(
    route: Route,
    supply_temperature_c: f64,
    mixed_air_temperature_c: f64,
) -> Option<
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitRetainedInput,
> {
    matches!(route, Route::SensibleGuardFalse | Route::Limited).then_some(
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitRetainedInput {
            preexisting_supply_temperature_c: supply_temperature_c,
            active_operands: matches!(route, Route::Limited).then_some(
                PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitActiveOperands {
                    mixed_air_temperature_c,
                },
            ),
        },
    )
}

fn advance(
    state: &mut PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitRuntimeState,
    route: Route,
    ordinal: usize,
    supply_temperature_c: f64,
    mixed_air_temperature_c: f64,
) -> PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot
{
    advance_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_state(
        state,
        predecessor(route, ordinal, supply_temperature_c),
        retained_input(route, supply_temperature_c, mixed_air_temperature_c),
    )
}

#[test]
fn source_boundary_and_exact_four_cp344_site_labels_are_stable() {
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2203"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2208"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER,
        &[
            "read-purchased-air-supply-temperature-for-minimum",
            "read-purchased-air-mixed-air-temperature-for-minimum",
            "apply-source-shaped-two-argument-minimum",
            "assign-purchased-air-supply-temperature",
        ]
    );
}

#[test]
fn all_six_routes_have_exact_local_shapes() {
    for route in [
        Route::UnitOff,
        Route::NonCooling,
        Route::PositiveGuardFalse,
        Route::CapacityGuardFalse,
        Route::SensibleGuardFalse,
        Route::Limited,
    ] {
        let mut state =
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitRuntimeState::new(
                ep_model::IdealLoadsAirSystemId(3),
            );
        let snapshot = advance(&mut state, route, 1, 24.0, 22.0);
        assert!(
            cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_snapshot_is_exact_direct_release(
                snapshot,
            )
        );
        if matches!(route, Route::SensibleGuardFalse) {
            assert_eq!(snapshot.preexisting_supply_temperature_c, Some(24.0));
            assert_eq!(snapshot.resulting_supply_temperature_c, Some(24.0));
            assert!(!snapshot.supply_temperature_for_minimum_read);
            assert!(!snapshot.supply_temperature_assignment_performed);
        } else if matches!(route, Route::Limited) {
            assert_eq!(snapshot.supply_temperature_before_mixed_air_limit_c, Some(24.0));
            assert_eq!(snapshot.mixed_air_temperature_c, Some(22.0));
            assert_eq!(snapshot.minimum_supply_temperature_c, Some(22.0));
            assert_eq!(snapshot.resulting_supply_temperature_c, Some(22.0));
            assert!(snapshot.supply_temperature_for_minimum_read);
            assert!(snapshot.mixed_air_temperature_for_minimum_read);
            assert!(snapshot.source_shaped_two_argument_minimum_evaluated);
            assert!(snapshot.supply_temperature_assignment_performed);
        } else {
            assert!(snapshot.preexisting_supply_temperature_c.is_none());
            assert!(snapshot.resulting_supply_temperature_c.is_none());
        }
    }
}

#[test]
fn pure_transition_preserves_source_minimum_signed_zero_nan_payload_and_infinity() {
    let left_nan = f64::from_bits(0x7ff8_0000_0000_0344);
    let right_nan = f64::from_bits(0x7ff8_0000_0000_0444);
    for (left, right, expected_bits) in [
        (12.0, 14.0, 12.0_f64.to_bits()),
        (14.0, 12.0, 12.0_f64.to_bits()),
        (-0.0, 0.0, 0.0_f64.to_bits()),
        (0.0, -0.0, (-0.0_f64).to_bits()),
        (f64::NEG_INFINITY, 1.0, f64::NEG_INFINITY.to_bits()),
        (f64::INFINITY, 1.0, 1.0_f64.to_bits()),
        (1.0, f64::INFINITY, 1.0_f64.to_bits()),
        (1.0, f64::NEG_INFINITY, f64::NEG_INFINITY.to_bits()),
        (left_nan, 1.0, 1.0_f64.to_bits()),
        (1.0, right_nan, right_nan.to_bits()),
        (left_nan, right_nan, right_nan.to_bits()),
    ] {
        let mut state =
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitRuntimeState::new(
                ep_model::IdealLoadsAirSystemId(3),
            );
        let snapshot = advance(&mut state, Route::Limited, 1, left, right);
        assert_eq!(
            snapshot
                .resulting_supply_temperature_c
                .expect("pure result")
                .to_bits(),
            expected_bits
        );
    }
}

#[test]
fn false_route_preserves_arbitrary_cp343_result_bits_without_sites() {
    for preexisting in [f64::from_bits(0x7ff8_0000_0000_0344), 0.0, -0.0] {
        let mut state =
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitRuntimeState::new(
                ep_model::IdealLoadsAirSystemId(3),
            );
        let snapshot = advance(
            &mut state,
            Route::SensibleGuardFalse,
            1,
            preexisting,
            22.0,
        );
        assert_eq!(
            snapshot.preexisting_supply_temperature_c.map(f64::to_bits),
            Some(preexisting.to_bits())
        );
        assert_eq!(
            snapshot.resulting_supply_temperature_c.map(f64::to_bits),
            Some(preexisting.to_bits())
        );
        assert!(snapshot.mixed_air_temperature_c.is_none());
        assert_eq!(state.source_site_execution_count, 0);
    }
}

#[test]
fn counters_partition_routes_and_apply_exact_four_site_identity() {
    let mut state =
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitRuntimeState::new(
            ep_model::IdealLoadsAirSystemId(3),
        );
    for (ordinal, route) in [
        (1, Route::UnitOff),
        (2, Route::NonCooling),
        (3, Route::PositiveGuardFalse),
        (4, Route::CapacityGuardFalse),
        (5, Route::SensibleGuardFalse),
        (6, Route::Limited),
    ] {
        advance(&mut state, route, ordinal, 24.0, 22.0);
    }
    assert_eq!(state.transition_count, 6);
    assert_eq!(state.unit_off_skip_count, 1);
    assert_eq!(state.non_cooling_skip_count, 1);
    assert_eq!(state.positive_guard_false_fallthrough_skip_count, 1);
    assert_eq!(state.capacity_limit_guard_false_fallthrough_skip_count, 1);
    assert_eq!(
        state.capacity_limit_sensible_output_guard_false_fallthrough_count,
        1
    );
    assert_eq!(
        state.capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count,
        1
    );
    assert_eq!(state.source_site_execution_count, 4);
    assert_eq!(state.supply_temperature_for_minimum_read_count, 1);
    assert_eq!(state.mixed_air_temperature_for_minimum_read_count, 1);
    assert_eq!(
        state.source_shaped_two_argument_minimum_evaluation_count,
        1
    );
    assert_eq!(state.supply_temperature_assignment_write_count, 1);
}
