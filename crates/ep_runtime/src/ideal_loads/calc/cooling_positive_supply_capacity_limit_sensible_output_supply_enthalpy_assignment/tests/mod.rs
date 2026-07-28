mod public_release;
pub(in crate::ideal_loads::calc) mod release_fixture;

use super::*;
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot,
};

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
    cooling_sensible_output_w: f64,
    maximum_total_cooling_capacity_w: f64,
) -> PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot
{
    let unit_off = matches!(route, Route::UnitOff);
    let non_cooling = matches!(route, Route::NonCooling);
    let positive_false = matches!(route, Route::PositiveGuardFalse);
    let capacity_false = matches!(route, Route::CapacityGuardFalse);
    let guard_false = matches!(route, Route::SensibleGuardFalse);
    let assigned = matches!(route, Route::Assigned);
    let cooling = positive_false || capacity_false || guard_false || assigned;
    let positive_body = capacity_false || guard_false || assigned;
    let active = guard_false || assigned;

    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot {
        source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE_ORDER,
        system: ep_model::IdealLoadsAirSystemId(3),
        parent_call_ordinal: ordinal,
        controlled_zone: ep_model::ZoneId(4),
        unit_body_entered: !unit_off,
        predecessor_cooling_body_entered: cooling,
        predecessor_no_outdoor_air_fallback_entered: cooling,
        predecessor_positive_supply_mass_flow_body_entered: positive_body,
        predecessor_active_guard_false_fallthrough: positive_false,
        predecessor_capacity_limit_guard_evaluated: positive_body,
        predecessor_capacity_limit_body_entered: active,
        predecessor_active_capacity_limit_guard_false_fallthrough: capacity_false,
        predecessor_capacity_limit_cp_air_assignment_executed: active,
        predecessor_capacity_limit_sensible_output_assignment_executed: active,
        predecessor_capacity_limit_sensible_output_guard_evaluated: active,
        predecessor_capacity_limit_sensible_output_guard_false_fallthrough: guard_false,
        predecessor_capacity_limit_sensible_output_adjustment_body_entered: assigned,
        unit_off_skipped: unit_off,
        non_cooling_skipped: non_cooling,
        positive_guard_false_fallthrough_skipped: positive_false,
        capacity_limit_guard_false_fallthrough_skipped: capacity_false,
        capacity_limit_sensible_output_guard_false_fallthrough: guard_false,
        capacity_limit_sensible_output_maximum_capacity_assignment_executed: assigned,
        preexisting_cooling_sensible_output_w: active.then_some(cooling_sensible_output_w),
        maximum_total_cooling_capacity_read: assigned,
        maximum_total_cooling_capacity_w: assigned
            .then_some(maximum_total_cooling_capacity_w),
        cooling_sensible_output_assigned: assigned,
        assigned_cooling_sensible_output_w: assigned
            .then_some(maximum_total_cooling_capacity_w),
        resulting_cooling_sensible_output_w: if assigned {
            Some(maximum_total_cooling_capacity_w)
        } else if guard_false {
            Some(cooling_sensible_output_w)
        } else {
            None
        },
    }
}

fn retained_input(
    route: Route,
    preexisting_supply_enthalpy_j_per_kg: f64,
    operands:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentActiveOperands,
) -> Option<
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentRetainedInput,
>{
    if matches!(route, Route::SensibleGuardFalse | Route::Assigned) {
        Some(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentRetainedInput {
                preexisting_supply_enthalpy_j_per_kg,
                active_operands: matches!(route, Route::Assigned).then_some(operands),
            },
        )
    } else {
        None
    }
}

fn advance(
    state: &mut PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentRuntimeState,
    route: Route,
    ordinal: usize,
    preexisting_supply_enthalpy_j_per_kg: f64,
    operands:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentActiveOperands,
) -> PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot
{
    advance_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_state(
        state,
        predecessor(
            route,
            ordinal,
            operands.cooling_sensible_output_w,
            operands.cooling_sensible_output_w,
        ),
        retained_input(route, preexisting_supply_enthalpy_j_per_kg, operands),
    )
}

fn ordinary_operands(
) -> PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentActiveOperands
{
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentActiveOperands {
        mixed_air_enthalpy_j_per_kg: 42_000.0,
        cooling_sensible_output_w: 10_000.0,
        supply_mass_flow_rate_kg_per_s: 2.0,
    }
}

#[test]
fn source_boundary_and_exact_six_sites_are_stable() {
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2200"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2201"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER,
        &[
            "read-retained-mixed-air-enthalpy-for-supply-enthalpy-difference",
            "read-retained-cooling-sensible-output-for-specific-cooling-output-division",
            "read-retained-supply-mass-flow-rate-for-specific-cooling-output-division",
            "calculate-cooling-sensible-output-divided-by-supply-mass-flow-rate",
            "calculate-mixed-air-enthalpy-minus-specific-cooling-output",
            "assign-local-supply-enthalpy",
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
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentRuntimeState::new(
                ep_model::IdealLoadsAirSystemId(3),
            );
        let snapshot = advance(&mut state, route, 1, 33_000.0, ordinary_operands());
        assert!(
            cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_snapshot_is_exact_direct_release(
                snapshot,
            )
        );
        if matches!(route, Route::SensibleGuardFalse) {
            assert_eq!(
                snapshot.preexisting_supply_enthalpy_j_per_kg,
                Some(33_000.0)
            );
            assert_eq!(snapshot.resulting_supply_enthalpy_j_per_kg, Some(33_000.0));
            assert!(!snapshot.mixed_air_enthalpy_read);
            assert!(!snapshot.supply_enthalpy_assigned);
        } else if matches!(route, Route::Assigned) {
            assert_eq!(snapshot.specific_cooling_output_j_per_kg, Some(5_000.0));
            assert_eq!(snapshot.resulting_supply_enthalpy_j_per_kg, Some(37_000.0));
            assert!(snapshot.mixed_air_enthalpy_read);
            assert!(snapshot.cooling_sensible_output_read);
            assert!(snapshot.supply_mass_flow_rate_read);
            assert!(snapshot.supply_enthalpy_assigned);
        } else {
            assert!(snapshot.preexisting_supply_enthalpy_j_per_kg.is_none());
            assert!(snapshot.resulting_supply_enthalpy_j_per_kg.is_none());
        }
    }
}

#[test]
fn arithmetic_divides_then_subtracts_without_reassociation() {
    let operands =
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentActiveOperands {
            mixed_air_enthalpy_j_per_kg: 1.000_000_000_000_000_2,
            cooling_sensible_output_w: 1.0e308,
            supply_mass_flow_rate_kg_per_s: 1.0e308,
        };
    let expected = operands.mixed_air_enthalpy_j_per_kg
        - operands.cooling_sensible_output_w / operands.supply_mass_flow_rate_kg_per_s;
    let reassociated = (operands.mixed_air_enthalpy_j_per_kg
        * operands.supply_mass_flow_rate_kg_per_s
        - operands.cooling_sensible_output_w)
        / operands.supply_mass_flow_rate_kg_per_s;
    assert_ne!(expected.to_bits(), reassociated.to_bits());

    let mut state =
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentRuntimeState::new(
            ep_model::IdealLoadsAirSystemId(3),
        );
    let snapshot = advance(&mut state, Route::Assigned, 1, 3.0, operands);
    assert_eq!(
        snapshot
            .calculated_supply_enthalpy_j_per_kg
            .map(f64::to_bits),
        Some(expected.to_bits())
    );
}

#[test]
fn pure_ieee_transition_keeps_positive_zero_and_negative_infinity() {
    for (operands, expected_quotient, expected_result) in [
        (
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentActiveOperands {
                mixed_air_enthalpy_j_per_kg: 42_000.0,
                cooling_sensible_output_w: 10_000.0,
                supply_mass_flow_rate_kg_per_s: f64::INFINITY,
            },
            0.0,
            42_000.0,
        ),
        (
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentActiveOperands {
                mixed_air_enthalpy_j_per_kg: 42_000.0,
                cooling_sensible_output_w: f64::MAX,
                supply_mass_flow_rate_kg_per_s: f64::MIN_POSITIVE,
            },
            f64::INFINITY,
            f64::NEG_INFINITY,
        ),
    ] {
        let mut state =
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentRuntimeState::new(
                ep_model::IdealLoadsAirSystemId(3),
            );
        let snapshot = advance(&mut state, Route::Assigned, 1, 33_000.0, operands);
        assert_eq!(
            snapshot.specific_cooling_output_j_per_kg.map(f64::to_bits),
            Some(expected_quotient.to_bits())
        );
        assert_eq!(
            snapshot.resulting_supply_enthalpy_j_per_kg.map(f64::to_bits),
            Some(expected_result.to_bits())
        );
    }
}

#[test]
fn guard_false_preserves_arbitrary_preexisting_bits_without_sites() {
    for preexisting in [f64::from_bits(0x7ff8_0000_0000_0342), 0.0, -0.0] {
        let mut state =
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentRuntimeState::new(
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
            snapshot
                .preexisting_supply_enthalpy_j_per_kg
                .map(f64::to_bits),
            Some(preexisting.to_bits())
        );
        assert_eq!(
            snapshot
                .resulting_supply_enthalpy_j_per_kg
                .map(f64::to_bits),
            Some(preexisting.to_bits())
        );
        assert_eq!(state.source_site_execution_count, 0);
    }
}

#[test]
fn counters_partition_routes_and_apply_six_h_identity() {
    let mut state =
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentRuntimeState::new(
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
        advance(&mut state, route, ordinal, 33_000.0, ordinary_operands());
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
        state.capacity_limit_sensible_output_supply_enthalpy_assignment_count,
        1
    );
    assert_eq!(state.source_site_execution_count, 6);
    assert_eq!(state.mixed_air_enthalpy_read_count, 1);
    assert_eq!(state.cooling_sensible_output_read_count, 1);
    assert_eq!(state.supply_mass_flow_rate_read_count, 1);
    assert_eq!(state.specific_cooling_output_calculation_count, 1);
    assert_eq!(state.supply_enthalpy_calculation_count, 1);
    assert_eq!(state.supply_enthalpy_assignment_write_count, 1);
}
