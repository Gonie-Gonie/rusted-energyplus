mod public_release;

use super::*;
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot,
};
use crate::psychrometrics::energyplus_psy_tdb_fn_h_w;

#[derive(Clone, Copy)]
enum Route {
    UnitOff,
    NonCooling,
    PositiveGuardFalse,
    CapacityGuardFalse,
    SensibleGuardFalse,
    Assigned,
}

fn predecessor(
    route: Route,
    ordinal: usize,
) -> PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot
{
    let unit_off = matches!(route, Route::UnitOff);
    let non_cooling = matches!(route, Route::NonCooling);
    let positive_false = matches!(route, Route::PositiveGuardFalse);
    let capacity_false = matches!(route, Route::CapacityGuardFalse);
    let guard_false = matches!(route, Route::SensibleGuardFalse);
    let assigned = matches!(route, Route::Assigned);
    let cooling = positive_false || capacity_false || guard_false || assigned;
    let positive_body = capacity_false || guard_false || assigned;
    let capacity_body = guard_false || assigned;
    let mixed_air = 42_000.0;
    let output = 10_000.0;
    let flow = 2.0;
    let quotient = output / flow;
    let calculated = mixed_air - quotient;
    let preexisting = 33_000.0;

    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot {
        source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER,
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
        unit_off_skipped: unit_off,
        non_cooling_skipped: non_cooling,
        positive_guard_false_fallthrough_skipped: positive_false,
        capacity_limit_guard_false_fallthrough_skipped: capacity_false,
        capacity_limit_sensible_output_guard_false_fallthrough: guard_false,
        capacity_limit_sensible_output_supply_enthalpy_assignment_executed: assigned,
        preexisting_supply_enthalpy_j_per_kg: capacity_body.then_some(preexisting),
        mixed_air_enthalpy_read: assigned,
        mixed_air_enthalpy_j_per_kg: assigned.then_some(mixed_air),
        cooling_sensible_output_read: assigned,
        cooling_sensible_output_w: assigned.then_some(output),
        supply_mass_flow_rate_read: assigned,
        supply_mass_flow_rate_kg_per_s: assigned.then_some(flow),
        specific_cooling_output_calculated: assigned,
        specific_cooling_output_j_per_kg: assigned.then_some(quotient),
        supply_enthalpy_calculated: assigned,
        calculated_supply_enthalpy_j_per_kg: assigned.then_some(calculated),
        supply_enthalpy_assigned: assigned,
        assigned_supply_enthalpy_j_per_kg: assigned.then_some(calculated),
        resulting_supply_enthalpy_j_per_kg: if assigned {
            Some(calculated)
        } else if guard_false {
            Some(preexisting)
        } else {
            None
        },
    }
}

fn retained_input(
    route: Route,
    preexisting_supply_temperature_c: f64,
    operands:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentActiveOperands,
) -> Option<
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentRetainedInput,
>{
    if matches!(route, Route::SensibleGuardFalse | Route::Assigned) {
        Some(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentRetainedInput {
                preexisting_supply_temperature_c,
                active_operands: matches!(route, Route::Assigned).then_some(operands),
            },
        )
    } else {
        None
    }
}

fn advance(
    state: &mut PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentRuntimeState,
    route: Route,
    ordinal: usize,
    preexisting_supply_temperature_c: f64,
    operands:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentActiveOperands,
) -> PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentSnapshot
{
    advance_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_state(
        state,
        predecessor(route, ordinal),
        retained_input(route, preexisting_supply_temperature_c, operands),
    )
}

fn ordinary_operands(
) -> PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentActiveOperands
{
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentActiveOperands {
        supply_enthalpy_j_per_kg: 37_000.0,
        supply_humidity_ratio: 0.008,
    }
}

#[test]
fn source_boundary_and_exact_four_sites_are_stable() {
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2201"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2203"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER,
        &[
            "read-local-supply-enthalpy-for-dry-bulb-inversion",
            "read-purchased-air-supply-humidity-ratio-for-dry-bulb-inversion",
            "evaluate-psy-tdb-fn-h-w",
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
        Route::Assigned,
    ] {
        let mut state =
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentRuntimeState::new(
                ep_model::IdealLoadsAirSystemId(3),
            );
        let snapshot = advance(&mut state, route, 1, 24.0, ordinary_operands());
        assert!(
            cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_snapshot_is_exact_direct_release(
                snapshot,
            )
        );
        if matches!(route, Route::SensibleGuardFalse) {
            assert_eq!(snapshot.preexisting_supply_temperature_c, Some(24.0));
            assert_eq!(snapshot.resulting_supply_temperature_c, Some(24.0));
            assert!(!snapshot.supply_enthalpy_for_dry_bulb_inversion_read);
            assert!(!snapshot.supply_temperature_assigned);
        } else if matches!(route, Route::Assigned) {
            let expected = energyplus_psy_tdb_fn_h_w(37_000.0, 0.008);
            assert_eq!(
                snapshot.resulting_supply_temperature_c.map(f64::to_bits),
                Some(expected.to_bits())
            );
            assert!(snapshot.supply_enthalpy_for_dry_bulb_inversion_read);
            assert!(snapshot.supply_humidity_ratio_for_dry_bulb_inversion_read);
            assert!(snapshot.psychrometric_supply_temperature_evaluated);
            assert!(snapshot.supply_temperature_assigned);
        } else {
            assert!(snapshot.preexisting_supply_temperature_c.is_none());
            assert!(snapshot.resulting_supply_temperature_c.is_none());
        }
    }
}

#[test]
fn pure_transition_preserves_helper_floor_nan_and_infinity_semantics() {
    for (enthalpy, humidity) in [
        (40_000.0, -0.0),
        (40_000.0, 0.0),
        (40_000.0, -1.0),
        (40_000.0, f64::NAN),
        (f64::INFINITY, 0.008),
        (f64::NEG_INFINITY, 0.008),
        (40_000.0, f64::INFINITY),
    ] {
        let operands =
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentActiveOperands {
                supply_enthalpy_j_per_kg: enthalpy,
                supply_humidity_ratio: humidity,
            };
        let mut state =
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentRuntimeState::new(
                ep_model::IdealLoadsAirSystemId(3),
            );
        let snapshot = advance(&mut state, Route::Assigned, 1, 24.0, operands);
        let expected = energyplus_psy_tdb_fn_h_w(enthalpy, humidity);
        let actual = snapshot
            .resulting_supply_temperature_c
            .expect("pure result");
        if expected.is_nan() {
            assert!(actual.is_nan());
        } else {
            assert_eq!(actual.to_bits(), expected.to_bits());
        }
    }

    let at_floor = energyplus_psy_tdb_fn_h_w(40_000.0, 1.0e-5);
    assert_eq!(
        energyplus_psy_tdb_fn_h_w(40_000.0, -0.0).to_bits(),
        at_floor.to_bits()
    );
    assert!(energyplus_psy_tdb_fn_h_w(40_000.0, f64::NAN).is_nan());
}

#[test]
fn pure_transition_characterizes_finite_operand_product_and_subtraction_overflow() {
    // Pure defensive characterization only: neither finite operand pair is a
    // new complete-public-chain CP342-to-CP343 reachability claim.
    let product_overflow_humidity = f64::MAX;
    assert!(product_overflow_humidity.is_finite());
    assert!((2.50094e6 * product_overflow_humidity).is_infinite());

    let mut product_state =
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentRuntimeState::new(
            ep_model::IdealLoadsAirSystemId(3),
        );
    let product_snapshot = advance(
        &mut product_state,
        Route::Assigned,
        1,
        24.0,
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentActiveOperands {
            supply_enthalpy_j_per_kg: 0.0,
            supply_humidity_ratio: product_overflow_humidity,
        },
    );
    assert!(
        product_snapshot
            .resulting_supply_temperature_c
            .expect("pure product-overflow result")
            .is_nan()
    );

    let subtraction_overflow_humidity = f64::MAX / (2.0 * 2.50094e6);
    let finite_latent_product = 2.50094e6 * subtraction_overflow_humidity;
    assert!(subtraction_overflow_humidity.is_finite());
    assert!(finite_latent_product.is_finite());
    assert!((-f64::MAX - finite_latent_product).is_infinite());

    let mut subtraction_state =
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentRuntimeState::new(
            ep_model::IdealLoadsAirSystemId(3),
        );
    let subtraction_snapshot = advance(
        &mut subtraction_state,
        Route::Assigned,
        1,
        24.0,
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentActiveOperands {
            supply_enthalpy_j_per_kg: -f64::MAX,
            supply_humidity_ratio: subtraction_overflow_humidity,
        },
    );
    let expected =
        energyplus_psy_tdb_fn_h_w(-f64::MAX, subtraction_overflow_humidity);
    assert!(expected.is_infinite() && expected.is_sign_negative());
    assert_eq!(
        subtraction_snapshot
            .resulting_supply_temperature_c
            .expect("pure subtraction-overflow result")
            .to_bits(),
        expected.to_bits()
    );
}

#[test]
fn false_route_preserves_arbitrary_preexisting_temperature_bits_without_sites() {
    for preexisting in [f64::from_bits(0x7ff8_0000_0000_0343), 0.0, -0.0] {
        let mut state =
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentRuntimeState::new(
                ep_model::IdealLoadsAirSystemId(3),
            );
        let snapshot = advance(
            &mut state,
            Route::SensibleGuardFalse,
            1,
            preexisting,
            ordinary_operands(),
        );
        assert_eq!(
            snapshot.preexisting_supply_temperature_c.map(f64::to_bits),
            Some(preexisting.to_bits())
        );
        assert_eq!(
            snapshot.resulting_supply_temperature_c.map(f64::to_bits),
            Some(preexisting.to_bits())
        );
        assert_eq!(state.source_site_execution_count, 0);
    }
}

#[test]
fn counters_partition_routes_and_apply_four_assignment_identity() {
    let mut state =
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentRuntimeState::new(
            ep_model::IdealLoadsAirSystemId(3),
        );
    for (ordinal, route) in [
        (1, Route::UnitOff),
        (2, Route::NonCooling),
        (3, Route::PositiveGuardFalse),
        (4, Route::CapacityGuardFalse),
        (5, Route::SensibleGuardFalse),
        (6, Route::Assigned),
    ] {
        advance(&mut state, route, ordinal, 24.0, ordinary_operands());
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
        state.capacity_limit_sensible_output_supply_temperature_assignment_count,
        1
    );
    assert_eq!(state.source_site_execution_count, 4);
    assert_eq!(state.supply_enthalpy_for_dry_bulb_inversion_read_count, 1);
    assert_eq!(
        state.supply_humidity_ratio_for_dry_bulb_inversion_read_count,
        1
    );
    assert_eq!(state.psychrometric_supply_temperature_evaluation_count, 1);
    assert_eq!(state.supply_temperature_assignment_write_count, 1);
}
