mod release_corruption;

use super::*;
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot,
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
) -> PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot {
    let unit_off = matches!(route, Route::UnitOff);
    let non_cooling = matches!(route, Route::NonCooling);
    let positive_guard_false = matches!(route, Route::PositiveGuardFalse);
    let capacity_guard_false = matches!(route, Route::CapacityGuardFalse);
    let assigned = matches!(route, Route::Assigned);
    let active = capacity_guard_false || assigned;
    let cooling = positive_guard_false || active;
    let first_limit = if capacity_guard_false {
        Some(ep_model::IdealLoadsLimit::NoLimit)
    } else if assigned {
        Some(ep_model::IdealLoadsLimit::LimitCapacity)
    } else {
        None
    };

    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_SOURCE_ORDER,
        system: ep_model::IdealLoadsAirSystemId(3),
        parent_call_ordinal: ordinal,
        controlled_zone: ep_model::ZoneId(4),
        unit_body_entered: !unit_off,
        predecessor_cooling_body_entered: cooling,
        predecessor_no_outdoor_air_fallback_entered: cooling,
        predecessor_positive_supply_mass_flow_body_entered: active,
        predecessor_active_guard_false_fallthrough: positive_guard_false,
        unit_off_skipped: unit_off,
        non_cooling_skipped: non_cooling,
        positive_guard_false_fallthrough_skipped: positive_guard_false,
        capacity_limit_guard_evaluated: active,
        first_cooling_limit_read: active,
        first_cooling_limit: first_limit,
        cooling_limit_capacity_comparison_evaluated: active,
        cooling_limit_capacity: active.then_some(assigned),
        second_cooling_limit_read: capacity_guard_false,
        second_cooling_limit: capacity_guard_false.then_some(ep_model::IdealLoadsLimit::NoLimit),
        cooling_limit_flow_rate_and_capacity_comparison_evaluated: capacity_guard_false,
        cooling_limit_flow_rate_and_capacity: capacity_guard_false.then_some(false),
        cooling_limit_condition_satisfied: active.then_some(assigned),
        cooling_limit_rejected: capacity_guard_false,
        capacity_limit_body_entered: assigned,
        active_guard_false_fallthrough: capacity_guard_false,
    }
}

fn advance(
    state: &mut PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentRuntimeState,
    route: Route,
    ordinal: usize,
    humidity_ratio: f64,
) -> PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot {
    let active_input = matches!(route, Route::Assigned).then_some(
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentActiveInput {
            mixed_air_humidity_ratio: humidity_ratio,
        },
    );
    advance_cooling_positive_supply_capacity_limit_cp_air_assignment_state(
        state,
        predecessor(route, ordinal),
        active_input,
    )
}

#[test]
fn source_boundary_and_exact_three_sites_are_stable() {
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_CP_AIR_ASSIGNMENT_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2196"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2197"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_CP_AIR_ASSIGNMENT_SOURCE_ORDER,
        &[
            "read-purchased-air-mixed-air-humidity-ratio",
            "evaluate-psy-cp-air-fn-w",
            "assign-local-cp-air",
        ]
    );
}

#[test]
fn capacity_body_executes_three_sites_and_assigns_canonical_cp_air() {
    let humidity_ratio = 0.008;
    let mut state =
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentRuntimeState::new(
            ep_model::IdealLoadsAirSystemId(3),
        );
    let snapshot = advance(&mut state, Route::Assigned, 1, humidity_ratio);
    let expected = energyplus_psy_cp_air_fn_w(humidity_ratio);

    assert!(
        cooling_positive_supply_capacity_limit_cp_air_assignment_snapshot_is_exact_direct_release(
            snapshot,
        )
    );
    assert!(snapshot.mixed_air_humidity_ratio_read);
    assert_eq!(
        snapshot.mixed_air_humidity_ratio.map(f64::to_bits),
        Some(humidity_ratio.to_bits())
    );
    assert!(snapshot.psychrometric_cp_air_evaluated);
    assert_eq!(
        snapshot
            .psychrometric_cp_air_result_j_per_kg_k
            .map(f64::to_bits),
        Some(expected.to_bits())
    );
    assert!(snapshot.cp_air_assigned);
    assert_eq!(
        snapshot.cp_air_j_per_kg_k.map(f64::to_bits),
        Some(expected.to_bits())
    );
    assert_eq!(state.source_site_execution_count, 3);
}

#[test]
fn pure_scalar_characterization_preserves_raw_ieee_humidity_classes() {
    for humidity_ratio in [
        f64::NEG_INFINITY,
        -1.0,
        -0.0,
        0.0,
        f64::from_bits(1),
        1.0e-5,
        0.008,
        f64::INFINITY,
        f64::from_bits(0x7ff8_0000_0000_00a1),
    ] {
        let mut state =
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentRuntimeState::new(
                ep_model::IdealLoadsAirSystemId(3),
            );
        let snapshot = advance(&mut state, Route::Assigned, 1, humidity_ratio);
        let expected = energyplus_psy_cp_air_fn_w(humidity_ratio);

        assert_eq!(
            snapshot.mixed_air_humidity_ratio.map(f64::to_bits),
            Some(humidity_ratio.to_bits())
        );
        assert_eq!(
            snapshot
                .psychrometric_cp_air_result_j_per_kg_k
                .map(f64::to_bits),
            Some(expected.to_bits())
        );
        assert_eq!(
            snapshot.cp_air_j_per_kg_k.map(f64::to_bits),
            Some(expected.to_bits())
        );
    }
}

#[test]
fn all_four_skipped_routes_execute_no_cp338_sites_or_scalar_work() {
    for (route, unit_off, non_cooling, positive_false, capacity_false) in [
        (Route::UnitOff, true, false, false, false),
        (Route::NonCooling, false, true, false, false),
        (Route::PositiveGuardFalse, false, false, true, false),
        (Route::CapacityGuardFalse, false, false, false, true),
    ] {
        let mut state =
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentRuntimeState::new(
                ep_model::IdealLoadsAirSystemId(3),
            );
        let snapshot = advance(
            &mut state,
            route,
            1,
            f64::from_bits(0x7ff8_0000_0000_00ff),
        );

        assert!(
            cooling_positive_supply_capacity_limit_cp_air_assignment_snapshot_is_exact_direct_release(
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
        assert!(!snapshot.mixed_air_humidity_ratio_read);
        assert!(snapshot.mixed_air_humidity_ratio.is_none());
        assert!(!snapshot.psychrometric_cp_air_evaluated);
        assert!(!snapshot.cp_air_assigned);
        assert_eq!(state.source_site_execution_count, 0);
    }
}

#[test]
fn counters_partition_all_five_routes_and_count_three_sites_per_assignment() {
    let mut state =
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentRuntimeState::new(
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
        advance(&mut state, route, ordinal + 1, 0.008);
    }

    assert_eq!(state.transition_count, 5);
    assert_eq!(state.unit_off_skip_count, 1);
    assert_eq!(state.non_cooling_skip_count, 1);
    assert_eq!(state.positive_guard_false_fallthrough_skip_count, 1);
    assert_eq!(
        state.capacity_limit_guard_false_fallthrough_skip_count,
        1
    );
    assert_eq!(state.capacity_limit_cp_air_assignment_count, 1);
    assert_eq!(state.source_site_execution_count, 3);
    assert_eq!(state.mixed_air_humidity_ratio_read_count, 1);
    assert_eq!(state.psychrometric_cp_air_evaluation_count, 1);
    assert_eq!(state.cp_air_assignment_write_count, 1);
}

#[test]
fn exact_predicate_and_bit_matcher_reject_corruption_and_signed_zero_drift() {
    let mut state =
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentRuntimeState::new(
            ep_model::IdealLoadsAirSystemId(3),
        );
    let snapshot = advance(&mut state, Route::Assigned, 1, 0.0);

    let mut negative_zero = snapshot;
    negative_zero.mixed_air_humidity_ratio = Some(-0.0);
    assert_eq!(snapshot, negative_zero);
    assert!(!super::release::snapshots_match_bit_exact(
        snapshot,
        negative_zero,
    ));

    let mut forged_result = snapshot;
    forged_result.psychrometric_cp_air_result_j_per_kg_k = forged_result
        .psychrometric_cp_air_result_j_per_kg_k
        .map(|value| f64::from_bits(value.to_bits() + 1));
    assert!(
        !cooling_positive_supply_capacity_limit_cp_air_assignment_snapshot_is_exact_direct_release(
            forged_result,
        )
    );

    let mut forged_route = snapshot;
    forged_route.capacity_limit_guard_false_fallthrough_skipped = true;
    assert!(
        !cooling_positive_supply_capacity_limit_cp_air_assignment_snapshot_is_exact_direct_release(
            forged_route,
        )
    );
}
