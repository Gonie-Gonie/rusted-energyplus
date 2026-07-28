mod ieee;
mod release_corruption;
mod release_edge_cases;
pub(in crate::ideal_loads::calc) mod release_fixture;

use super::*;
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_CP_AIR_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_CP_AIR_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot,
    cooling_positive_supply_capacity_limit_cp_air_assignment_snapshot_is_exact_direct_release,
};
use crate::psychrometrics::energyplus_psy_cp_air_fn_w;

#[derive(Clone, Copy)]
enum Route {
    UnitOff,
    NonCooling,
    PositiveGuardFalse,
    CapacityGuardFalse,
    Assigned,
}

fn predecessor(
    route: Route,
    ordinal: usize,
) -> PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot {
    let unit_off = matches!(route, Route::UnitOff);
    let non_cooling = matches!(route, Route::NonCooling);
    let positive_guard_false = matches!(route, Route::PositiveGuardFalse);
    let capacity_guard_false = matches!(route, Route::CapacityGuardFalse);
    let assigned = matches!(route, Route::Assigned);
    let positive_body = capacity_guard_false || assigned;
    let cooling = positive_guard_false || positive_body;
    let humidity_ratio = assigned.then_some(0.008);
    let cp_air = humidity_ratio.map(energyplus_psy_cp_air_fn_w);
    let snapshot =
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot {
            source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_CP_AIR_ASSIGNMENT_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
            source_order:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_CP_AIR_ASSIGNMENT_SOURCE_ORDER,
            system: ep_model::IdealLoadsAirSystemId(3),
            parent_call_ordinal: ordinal,
            controlled_zone: ep_model::ZoneId(4),
            unit_body_entered: !unit_off,
            predecessor_cooling_body_entered: cooling,
            predecessor_no_outdoor_air_fallback_entered: cooling,
            predecessor_positive_supply_mass_flow_body_entered: positive_body,
            predecessor_active_guard_false_fallthrough: positive_guard_false,
            predecessor_capacity_limit_guard_evaluated: positive_body,
            predecessor_capacity_limit_body_entered: assigned,
            predecessor_active_capacity_limit_guard_false_fallthrough: capacity_guard_false,
            unit_off_skipped: unit_off,
            non_cooling_skipped: non_cooling,
            positive_guard_false_fallthrough_skipped: positive_guard_false,
            capacity_limit_guard_false_fallthrough_skipped: capacity_guard_false,
            capacity_limit_cp_air_assignment_executed: assigned,
            mixed_air_humidity_ratio_read: assigned,
            mixed_air_humidity_ratio: humidity_ratio,
            psychrometric_cp_air_evaluated: assigned,
            psychrometric_cp_air_result_j_per_kg_k: cp_air,
            cp_air_assigned: assigned,
            cp_air_j_per_kg_k: cp_air,
        };
    assert!(
        cooling_positive_supply_capacity_limit_cp_air_assignment_snapshot_is_exact_direct_release(
            snapshot,
        )
    );
    snapshot
}

fn advance(
    state: &mut PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentRuntimeState,
    route: Route,
    ordinal: usize,
    operands: [f64; 3],
) -> PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot {
    let active_input = matches!(route, Route::Assigned).then_some(
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentActiveInput {
            supply_mass_flow_rate_kg_per_s: operands[0],
            mixed_air_enthalpy_j_per_kg: operands[1],
            supply_enthalpy_j_per_kg: operands[2],
        },
    );
    advance_cooling_positive_supply_capacity_limit_sensible_output_assignment_state(
        state,
        predecessor(route, ordinal),
        active_input,
    )
}

#[test]
fn source_boundary_and_exact_six_sites_are_stable() {
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2197"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2198"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER,
        &[
            "read-retained-supply-mass-flow-rate-for-sensible-output-product",
            "read-retained-mixed-air-enthalpy-for-sensible-output-difference",
            "read-retained-supply-enthalpy-for-sensible-output-difference",
            "calculate-mixed-air-enthalpy-minus-supply-enthalpy",
            "calculate-supply-mass-flow-rate-times-enthalpy-difference",
            "assign-local-cooling-sensible-output",
        ]
    );
}

#[test]
fn active_assignment_executes_six_sites_in_source_grouping() {
    let operands = [2.5, 48_000.0, 32_000.0];
    let mut state =
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentRuntimeState::new(
            ep_model::IdealLoadsAirSystemId(3),
        );
    let snapshot = advance(&mut state, Route::Assigned, 1, operands);
    let difference = operands[1] - operands[2];
    let expected = operands[0] * difference;

    assert!(
        cooling_positive_supply_capacity_limit_sensible_output_assignment_snapshot_is_exact_direct_release(
            snapshot,
        )
    );
    assert_eq!(
        snapshot
            .mixed_air_minus_supply_enthalpy_j_per_kg
            .map(f64::to_bits),
        Some(difference.to_bits())
    );
    assert_eq!(
        snapshot.cooling_sensible_output_w.map(f64::to_bits),
        Some(expected.to_bits())
    );
    assert_eq!(state.source_site_execution_count, 6);
    assert_eq!(state.supply_mass_flow_rate_read_count, 1);
    assert_eq!(state.mixed_air_enthalpy_read_count, 1);
    assert_eq!(state.supply_enthalpy_read_count, 1);
    assert_eq!(state.enthalpy_difference_calculation_count, 1);
    assert_eq!(state.cooling_sensible_output_calculation_count, 1);
    assert_eq!(state.cooling_sensible_output_assignment_write_count, 1);
}

#[test]
fn all_four_skipped_routes_are_complete_null_and_execute_no_sites() {
    for (route, unit_off, non_cooling, positive_false, capacity_false) in [
        (Route::UnitOff, true, false, false, false),
        (Route::NonCooling, false, true, false, false),
        (Route::PositiveGuardFalse, false, false, true, false),
        (Route::CapacityGuardFalse, false, false, false, true),
    ] {
        let mut state =
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentRuntimeState::new(
                ep_model::IdealLoadsAirSystemId(3),
            );
        let snapshot = advance(
            &mut state,
            route,
            1,
            [
                f64::from_bits(0x7ff8_0000_0000_00a1),
                f64::NEG_INFINITY,
                f64::INFINITY,
            ],
        );

        assert!(
            cooling_positive_supply_capacity_limit_sensible_output_assignment_snapshot_is_exact_direct_release(
                snapshot,
            )
        );
        assert_eq!(snapshot.unit_off_skipped, unit_off);
        assert_eq!(snapshot.non_cooling_skipped, non_cooling);
        assert_eq!(
            snapshot.positive_guard_false_fallthrough_skipped,
            positive_false
        );
        assert_eq!(
            snapshot.capacity_limit_guard_false_fallthrough_skipped,
            capacity_false
        );
        assert!(!snapshot.supply_mass_flow_rate_read);
        assert!(snapshot.supply_mass_flow_rate_kg_per_s.is_none());
        assert!(!snapshot.mixed_air_enthalpy_read);
        assert!(snapshot.mixed_air_enthalpy_j_per_kg.is_none());
        assert!(!snapshot.supply_enthalpy_read);
        assert!(snapshot.supply_enthalpy_j_per_kg.is_none());
        assert!(!snapshot.enthalpy_difference_calculated);
        assert!(!snapshot.cooling_sensible_output_calculated);
        assert!(!snapshot.cooling_sensible_output_assigned);
        assert_eq!(state.source_site_execution_count, 0);
    }
}

#[test]
fn counters_partition_five_routes_and_count_each_site_once_per_assignment() {
    let mut state =
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentRuntimeState::new(
            ep_model::IdealLoadsAirSystemId(3),
        );
    for (ordinal, route) in [
        Route::UnitOff,
        Route::NonCooling,
        Route::PositiveGuardFalse,
        Route::CapacityGuardFalse,
        Route::Assigned,
    ]
    .into_iter()
    .enumerate()
    {
        advance(&mut state, route, ordinal + 1, [2.0, 3.0, 1.0]);
    }

    assert_eq!(state.transition_count, 5);
    assert_eq!(state.unit_off_skip_count, 1);
    assert_eq!(state.non_cooling_skip_count, 1);
    assert_eq!(state.positive_guard_false_fallthrough_skip_count, 1);
    assert_eq!(
        state.capacity_limit_guard_false_fallthrough_skip_count,
        1
    );
    assert_eq!(state.capacity_limit_sensible_output_assignment_count, 1);
    assert_eq!(state.source_site_execution_count, 6);
    assert_eq!(state.supply_mass_flow_rate_read_count, 1);
    assert_eq!(state.mixed_air_enthalpy_read_count, 1);
    assert_eq!(state.supply_enthalpy_read_count, 1);
    assert_eq!(state.enthalpy_difference_calculation_count, 1);
    assert_eq!(state.cooling_sensible_output_calculation_count, 1);
    assert_eq!(state.cooling_sensible_output_assignment_write_count, 1);
}

#[test]
fn exact_predicate_rejects_derived_value_and_route_corruption() {
    let mut state =
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentRuntimeState::new(
            ep_model::IdealLoadsAirSystemId(3),
        );
    let snapshot = advance(&mut state, Route::Assigned, 1, [2.0, 3.0, 1.0]);

    let mut forged_product = snapshot;
    forged_product.calculated_cooling_sensible_output_w = forged_product
        .calculated_cooling_sensible_output_w
        .map(|value| f64::from_bits(value.to_bits() + 1));
    assert!(
        !cooling_positive_supply_capacity_limit_sensible_output_assignment_snapshot_is_exact_direct_release(
            forged_product,
        )
    );

    let mut forged_route = snapshot;
    forged_route.capacity_limit_guard_false_fallthrough_skipped = true;
    assert!(
        !cooling_positive_supply_capacity_limit_sensible_output_assignment_snapshot_is_exact_direct_release(
            forged_route,
        )
    );
}
