mod public_release;
mod committed;
pub(in crate::ideal_loads::calc) mod release_fixture;

use super::*;
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot,
};

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
    cooling_sensible_output_w: f64,
) -> PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot {
    let unit_off = matches!(route, Route::UnitOff);
    let non_cooling = matches!(route, Route::NonCooling);
    let positive_false = matches!(route, Route::PositiveGuardFalse);
    let capacity_false = matches!(route, Route::CapacityGuardFalse);
    let assigned = matches!(route, Route::Assigned);
    let cooling = positive_false || capacity_false || assigned;
    let positive_body = capacity_false || assigned;
    let value = assigned.then_some(cooling_sensible_output_w);

    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot {
        source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER,
        system: ep_model::IdealLoadsAirSystemId(3),
        parent_call_ordinal: ordinal,
        controlled_zone: ep_model::ZoneId(4),
        unit_body_entered: !unit_off,
        predecessor_cooling_body_entered: cooling,
        predecessor_no_outdoor_air_fallback_entered: cooling,
        predecessor_positive_supply_mass_flow_body_entered: positive_body,
        predecessor_active_guard_false_fallthrough: positive_false,
        predecessor_capacity_limit_guard_evaluated: positive_body,
        predecessor_capacity_limit_body_entered: assigned,
        predecessor_active_capacity_limit_guard_false_fallthrough: capacity_false,
        predecessor_capacity_limit_cp_air_assignment_executed: assigned,
        unit_off_skipped: unit_off,
        non_cooling_skipped: non_cooling,
        positive_guard_false_fallthrough_skipped: positive_false,
        capacity_limit_guard_false_fallthrough_skipped: capacity_false,
        capacity_limit_sensible_output_assignment_executed: assigned,
        supply_mass_flow_rate_read: assigned,
        supply_mass_flow_rate_kg_per_s: assigned.then_some(1.0),
        mixed_air_enthalpy_read: assigned,
        mixed_air_enthalpy_j_per_kg: value,
        supply_enthalpy_read: assigned,
        supply_enthalpy_j_per_kg: assigned.then_some(0.0),
        enthalpy_difference_calculated: assigned,
        mixed_air_minus_supply_enthalpy_j_per_kg: value,
        cooling_sensible_output_calculated: assigned,
        calculated_cooling_sensible_output_w: value,
        cooling_sensible_output_assigned: assigned,
        cooling_sensible_output_w: value,
    }
}

fn advance(
    state: &mut PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardRuntimeState,
    route: Route,
    ordinal: usize,
    cooling_sensible_output_w: f64,
    maximum_total_cooling_capacity_w: f64,
) -> PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot {
    let active_input = matches!(route, Route::Assigned).then_some(
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardActiveInput {
            cooling_sensible_output_w,
            maximum_total_cooling_capacity_w,
        },
    );
    advance_cooling_positive_supply_capacity_limit_sensible_output_guard_state(
        state,
        predecessor(route, ordinal, cooling_sensible_output_w),
        active_input,
    )
}

#[test]
fn source_boundary_and_exact_four_sites_are_stable() {
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2198"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2199"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_SOURCE_ORDER,
        &[
            "read-retained-cooling-sensible-output-for-maximum-capacity-comparison",
            "read-retained-maximum-total-cooling-capacity-for-sensible-output-comparison",
            "compare-cooling-sensible-output-greater-than-or-equal-to-maximum-total-cooling-capacity",
            "enter-cooling-capacity-adjustment-body-if-comparison-satisfied",
        ]
    );
}

#[test]
fn pure_transition_preserves_raw_ieee_greater_than_or_equal_behavior() {
    for (output, maximum, expected) in [
        (f64::NEG_INFINITY, 100.0, false),
        (99.0, 100.0, false),
        (100.0, 100.0, true),
        (f64::INFINITY, 100.0, true),
        (f64::from_bits(0x7ff8_0000_0000_00a1), 100.0, false),
        (0.0, -0.0, true),
        (-0.0, 0.0, true),
    ] {
        let mut state =
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardRuntimeState::new(
                ep_model::IdealLoadsAirSystemId(3),
            );
        let snapshot = advance(&mut state, Route::Assigned, 1, output, maximum);

        assert!(
            cooling_positive_supply_capacity_limit_sensible_output_guard_snapshot_is_exact_direct_release(
                snapshot,
            )
        );
        assert_eq!(
            snapshot.cooling_sensible_output_w.map(f64::to_bits),
            Some(output.to_bits())
        );
        assert_eq!(
            snapshot
                .maximum_total_cooling_capacity_w
                .map(f64::to_bits),
            Some(maximum.to_bits())
        );
        assert_eq!(
            snapshot.cooling_sensible_output_at_or_above_maximum_capacity,
            Some(expected)
        );
        assert_eq!(
            snapshot.capacity_limit_sensible_output_adjustment_body_entered,
            expected
        );
        assert_eq!(
            snapshot.capacity_limit_sensible_output_guard_false_fallthrough,
            !expected
        );
        assert_eq!(
            state.source_site_execution_count,
            3 + usize::from(expected)
        );
    }
}

#[test]
fn all_four_inherited_skips_execute_no_cp340_sites() {
    for (route, unit_off, non_cooling, positive_false, capacity_false) in [
        (Route::UnitOff, true, false, false, false),
        (Route::NonCooling, false, true, false, false),
        (Route::PositiveGuardFalse, false, false, true, false),
        (Route::CapacityGuardFalse, false, false, false, true),
    ] {
        let mut state =
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardRuntimeState::new(
                ep_model::IdealLoadsAirSystemId(3),
            );
        let snapshot = advance(
            &mut state,
            route,
            1,
            f64::from_bits(0x7ff8_0000_0000_00a1),
            f64::INFINITY,
        );

        assert!(
            cooling_positive_supply_capacity_limit_sensible_output_guard_snapshot_is_exact_direct_release(
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
        assert!(!snapshot.capacity_limit_sensible_output_guard_evaluated);
        assert!(!snapshot.cooling_sensible_output_read);
        assert!(snapshot.cooling_sensible_output_w.is_none());
        assert!(!snapshot.maximum_total_cooling_capacity_read);
        assert!(snapshot.maximum_total_cooling_capacity_w.is_none());
        assert_eq!(state.source_site_execution_count, 0);
    }
}

#[test]
fn counters_partition_six_routes_and_apply_three_a_plus_e_identity() {
    let mut state =
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardRuntimeState::new(
            ep_model::IdealLoadsAirSystemId(3),
        );
    for (ordinal, route, output, maximum) in [
        (1, Route::UnitOff, 0.0, 100.0),
        (2, Route::NonCooling, 0.0, 100.0),
        (3, Route::PositiveGuardFalse, 0.0, 100.0),
        (4, Route::CapacityGuardFalse, 0.0, 100.0),
        (5, Route::Assigned, 99.0, 100.0),
        (6, Route::Assigned, 100.0, 100.0),
    ] {
        advance(&mut state, route, ordinal, output, maximum);
    }

    assert_eq!(state.transition_count, 6);
    assert_eq!(state.unit_off_skip_count, 1);
    assert_eq!(state.non_cooling_skip_count, 1);
    assert_eq!(state.positive_guard_false_fallthrough_skip_count, 1);
    assert_eq!(
        state.capacity_limit_guard_false_fallthrough_skip_count,
        1
    );
    assert_eq!(
        state.capacity_limit_sensible_output_guard_evaluation_count,
        2
    );
    assert_eq!(
        state.capacity_limit_sensible_output_guard_false_fallthrough_count,
        1
    );
    assert_eq!(
        state.capacity_limit_sensible_output_adjustment_body_entry_count,
        1
    );
    assert_eq!(state.source_site_execution_count, 7);
    assert_eq!(state.cooling_sensible_output_read_count, 2);
    assert_eq!(state.maximum_total_cooling_capacity_read_count, 2);
    assert_eq!(
        state.cooling_sensible_output_maximum_capacity_comparison_count,
        2
    );
}

#[test]
fn exact_predicate_and_matcher_reject_forgery_but_retain_nan_bits() {
    let mut state =
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardRuntimeState::new(
            ep_model::IdealLoadsAirSystemId(3),
        );
    let snapshot = advance(
        &mut state,
        Route::Assigned,
        1,
        f64::from_bits(0x7ff8_0000_0000_00a1),
        100.0,
    );
    assert!(super::release::snapshots_match_bit_exact(
        snapshot, snapshot,
    ));

    let mut forged = snapshot;
    forged.cooling_sensible_output_at_or_above_maximum_capacity = Some(true);
    assert!(
        !cooling_positive_supply_capacity_limit_sensible_output_guard_snapshot_is_exact_direct_release(
            forged,
        )
    );

    let mut different_payload = snapshot;
    different_payload.cooling_sensible_output_w =
        Some(f64::from_bits(0x7ff8_0000_0000_00b2));
    assert!(!super::release::snapshots_match_bit_exact(
        snapshot,
        different_payload,
    ));
}
