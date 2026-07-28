mod public_release;

use super::*;
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot,
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
) -> PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot {
    let unit_off = matches!(route, Route::UnitOff);
    let non_cooling = matches!(route, Route::NonCooling);
    let positive_false = matches!(route, Route::PositiveGuardFalse);
    let capacity_false = matches!(route, Route::CapacityGuardFalse);
    let guard_false = matches!(route, Route::SensibleGuardFalse);
    let assigned = matches!(route, Route::Assigned);
    let cooling = positive_false || capacity_false || guard_false || assigned;
    let positive_body = capacity_false || guard_false || assigned;
    let active = guard_false || assigned;

    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot {
        source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_SOURCE_ORDER,
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
        unit_off_skipped: unit_off,
        non_cooling_skipped: non_cooling,
        positive_guard_false_fallthrough_skipped: positive_false,
        capacity_limit_guard_false_fallthrough_skipped: capacity_false,
        capacity_limit_sensible_output_guard_evaluated: active,
        cooling_sensible_output_read: active,
        cooling_sensible_output_w: active.then_some(cooling_sensible_output_w),
        maximum_total_cooling_capacity_read: active,
        maximum_total_cooling_capacity_w: active
            .then_some(maximum_total_cooling_capacity_w),
        cooling_sensible_output_maximum_capacity_comparison_evaluated: active,
        cooling_sensible_output_at_or_above_maximum_capacity: if active {
            Some(assigned)
        } else {
            None
        },
        capacity_limit_sensible_output_guard_false_fallthrough: guard_false,
        capacity_limit_sensible_output_adjustment_body_entered: assigned,
    }
}

fn advance(
    state: &mut PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentRuntimeState,
    route: Route,
    ordinal: usize,
    cooling_sensible_output_w: f64,
    maximum_total_cooling_capacity_w: f64,
) -> PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot
{
    advance_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_state(
        state,
        predecessor(
            route,
            ordinal,
            cooling_sensible_output_w,
            maximum_total_cooling_capacity_w,
        ),
    )
}

#[test]
fn source_boundary_and_exact_two_sites_are_stable() {
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2199"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2200"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE_ORDER,
        &[
            "read-retained-maximum-total-cooling-capacity-for-sensible-output-assignment",
            "assign-local-cooling-sensible-output-from-maximum-total-cooling-capacity",
        ]
    );
}

#[test]
fn all_six_routes_have_exact_local_value_shapes() {
    for route in [
        Route::UnitOff,
        Route::NonCooling,
        Route::PositiveGuardFalse,
        Route::CapacityGuardFalse,
        Route::SensibleGuardFalse,
        Route::Assigned,
    ] {
        let mut state =
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentRuntimeState::new(
                ep_model::IdealLoadsAirSystemId(3),
            );
        let snapshot = advance(&mut state, route, 1, 99.0, 100.0);
        assert!(
            cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_snapshot_is_exact_direct_release(
                snapshot,
            )
        );

        if matches!(route, Route::SensibleGuardFalse) {
            assert_eq!(
                snapshot.preexisting_cooling_sensible_output_w,
                Some(99.0)
            );
            assert!(!snapshot.maximum_total_cooling_capacity_read);
            assert!(snapshot.maximum_total_cooling_capacity_w.is_none());
            assert!(!snapshot.cooling_sensible_output_assigned);
            assert!(snapshot.assigned_cooling_sensible_output_w.is_none());
            assert_eq!(snapshot.resulting_cooling_sensible_output_w, Some(99.0));
        } else if matches!(route, Route::Assigned) {
            assert_eq!(
                snapshot.preexisting_cooling_sensible_output_w,
                Some(99.0)
            );
            assert!(snapshot.maximum_total_cooling_capacity_read);
            assert_eq!(snapshot.maximum_total_cooling_capacity_w, Some(100.0));
            assert!(snapshot.cooling_sensible_output_assigned);
            assert_eq!(snapshot.assigned_cooling_sensible_output_w, Some(100.0));
            assert_eq!(
                snapshot.resulting_cooling_sensible_output_w,
                Some(100.0)
            );
        } else {
            assert!(snapshot.preexisting_cooling_sensible_output_w.is_none());
            assert!(!snapshot.maximum_total_cooling_capacity_read);
            assert!(snapshot.maximum_total_cooling_capacity_w.is_none());
            assert!(!snapshot.cooling_sensible_output_assigned);
            assert!(snapshot.assigned_cooling_sensible_output_w.is_none());
            assert!(snapshot.resulting_cooling_sensible_output_w.is_none());
        }
    }
}

#[test]
fn false_route_preserves_nan_and_signed_zero_bits_without_rhs_or_write() {
    for value in [
        f64::from_bits(0x7ff8_0000_0000_0341),
        0.0,
        -0.0,
    ] {
        let mut state =
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentRuntimeState::new(
                ep_model::IdealLoadsAirSystemId(3),
            );
        let snapshot =
            advance(&mut state, Route::SensibleGuardFalse, 1, value, 100.0);

        assert_eq!(
            snapshot
                .preexisting_cooling_sensible_output_w
                .map(f64::to_bits),
            Some(value.to_bits())
        );
        assert_eq!(
            snapshot
                .resulting_cooling_sensible_output_w
                .map(f64::to_bits),
            Some(value.to_bits())
        );
        assert!(!snapshot.maximum_total_cooling_capacity_read);
        assert!(snapshot.maximum_total_cooling_capacity_w.is_none());
        assert!(!snapshot.cooling_sensible_output_assigned);
        assert!(snapshot.assigned_cooling_sensible_output_w.is_none());
        assert_eq!(state.source_site_execution_count, 0);
    }
}

#[test]
fn true_route_replaces_positive_infinity_with_finite_maximum_bit_exact() {
    let mut state =
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentRuntimeState::new(
            ep_model::IdealLoadsAirSystemId(3),
        );
    let maximum = f64::from_bits(100.0_f64.to_bits() + 7);
    let snapshot = advance(
        &mut state,
        Route::Assigned,
        1,
        f64::INFINITY,
        maximum,
    );

    assert_eq!(
        snapshot
            .preexisting_cooling_sensible_output_w
            .map(f64::to_bits),
        Some(f64::INFINITY.to_bits())
    );
    for value in [
        snapshot.maximum_total_cooling_capacity_w,
        snapshot.assigned_cooling_sensible_output_w,
        snapshot.resulting_cooling_sensible_output_w,
    ] {
        assert_eq!(value.map(f64::to_bits), Some(maximum.to_bits()));
    }
    assert_eq!(state.source_site_execution_count, 2);
}

#[test]
fn pure_transition_bit_copies_arbitrary_rhs_payload_without_normalization() {
    for rhs in [
        f64::from_bits(0x7ff8_0000_0000_0341),
        f64::NEG_INFINITY,
        -0.0,
        f64::from_bits(1),
    ] {
        let mut state =
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentRuntimeState::new(
                ep_model::IdealLoadsAirSystemId(3),
            );
        let snapshot = advance(&mut state, Route::Assigned, 1, 1.0, rhs);
        assert_eq!(
            snapshot.maximum_total_cooling_capacity_w.map(f64::to_bits),
            Some(rhs.to_bits())
        );
        assert_eq!(
            snapshot
                .assigned_cooling_sensible_output_w
                .map(f64::to_bits),
            Some(rhs.to_bits())
        );
        assert_eq!(
            snapshot
                .resulting_cooling_sensible_output_w
                .map(f64::to_bits),
            Some(rhs.to_bits())
        );
        if !rhs.is_finite() || rhs <= 0.0 {
            assert!(
                !cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_snapshot_is_exact_direct_release(
                    snapshot,
                ),
                "pure out-of-domain bit-copy characterization is not public release reachability"
            );
        }
    }
}

#[test]
fn counters_partition_six_routes_and_apply_two_m_identity() {
    let mut state =
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentRuntimeState::new(
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
        advance(&mut state, route, ordinal, 99.0, 100.0);
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
        state.capacity_limit_sensible_output_guard_false_fallthrough_count,
        1
    );
    assert_eq!(
        state.capacity_limit_sensible_output_maximum_capacity_assignment_count,
        1
    );
    assert_eq!(state.source_site_execution_count, 2);
    assert_eq!(state.maximum_total_cooling_capacity_read_count, 1);
    assert_eq!(state.cooling_sensible_output_assignment_write_count, 1);
}

#[test]
fn matcher_rejects_ieee_payload_and_assigned_projection_drift() {
    let mut state =
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentRuntimeState::new(
            ep_model::IdealLoadsAirSystemId(3),
        );
    let snapshot = advance(
        &mut state,
        Route::Assigned,
        1,
        f64::INFINITY,
        f64::from_bits(0x7ff8_0000_0000_0341),
    );
    assert!(super::release::snapshots_match_bit_exact(
        snapshot, snapshot,
    ));

    let mut forged = snapshot;
    forged.assigned_cooling_sensible_output_w =
        Some(f64::from_bits(0x7ff8_0000_0000_0342));
    assert!(!super::release::snapshots_match_bit_exact(
        snapshot, forged,
    ));
    assert!(
        !cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_snapshot_is_exact_direct_release(
            forged,
        )
    );
}
