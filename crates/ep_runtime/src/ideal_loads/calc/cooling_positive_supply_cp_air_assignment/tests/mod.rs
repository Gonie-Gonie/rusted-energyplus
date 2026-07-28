mod release_corruption;

use super::*;
use crate::ideal_loads::PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot;
use crate::psychrometrics::energyplus_psy_cp_air_fn_w;

#[derive(Clone, Copy)]
enum Route {
    UnitOff,
    NonCooling,
    GuardFalse,
    Assigned,
}

fn predecessor(
    route: Route,
    ordinal: usize,
) -> PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot {
    let unit_off = matches!(route, Route::UnitOff);
    let non_cooling = matches!(route, Route::NonCooling);
    let guard_false = matches!(route, Route::GuardFalse);
    let assigned = matches!(route, Route::Assigned);
    let cooling = guard_false || assigned;
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot {
        source: crate::ideal_loads::PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_SOURCE,
        first_excluded_source:
            crate::ideal_loads::PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_FIRST_EXCLUDED_SOURCE,
        source_order:
            crate::ideal_loads::PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_SOURCE_ORDER,
        system: ep_model::IdealLoadsAirSystemId(3),
        parent_call_ordinal: ordinal,
        controlled_zone: ep_model::ZoneId(4),
        unit_body_entered: !unit_off,
        predecessor_cooling_call_executed: cooling,
        predecessor_zero_flow_reset_body_entered: guard_false,
        predecessor_active_guard_false_fallthrough: assigned,
        predecessor_no_outdoor_air_fallback_entered: cooling,
        unit_off_skipped: unit_off,
        non_cooling_skipped: non_cooling,
        cooling_body_entered: cooling,
        supply_mass_flow_rate_read: cooling,
        supply_mass_flow_rate_kg_per_s: if assigned {
            Some(0.25)
        } else if guard_false {
            Some(0.0)
        } else {
            None
        },
        supply_mass_flow_rate_strictly_positive_comparison_evaluated: cooling,
        supply_mass_flow_rate_strictly_positive: if assigned {
            Some(true)
        } else if guard_false {
            Some(false)
        } else {
            None
        },
        positive_supply_mass_flow_body_entered: assigned,
        active_guard_false_fallthrough: guard_false,
    }
}

fn advance(
    state: &mut PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentRuntimeState,
    route: Route,
    ordinal: usize,
    humidity_ratio: f64,
) -> PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentSnapshot {
    let active_input = matches!(route, Route::Assigned).then_some(
        PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentActiveInput {
            zone_humidity_ratio: humidity_ratio,
        },
    );
    advance_cooling_positive_supply_cp_air_assignment_state(
        state,
        predecessor(route, ordinal),
        active_input,
    )
}

#[test]
fn positive_route_executes_exact_three_sites_and_assigns_canonical_cp_air() {
    let humidity_ratio = 0.008;
    let mut state = PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentRuntimeState::new(
        ep_model::IdealLoadsAirSystemId(3),
    );
    let snapshot = advance(&mut state, Route::Assigned, 1, humidity_ratio);
    let expected = energyplus_psy_cp_air_fn_w(humidity_ratio);

    assert!(cooling_positive_supply_cp_air_assignment_snapshot_is_exact_direct_release(snapshot));
    assert_eq!(
        snapshot.source_order,
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CP_AIR_ASSIGNMENT_SOURCE_ORDER
    );
    assert!(snapshot.zone_humidity_ratio_read);
    assert_eq!(
        snapshot.zone_humidity_ratio.map(f64::to_bits),
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
fn canonical_scalar_characterization_preserves_raw_humidity_classes() {
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
        let mut state = PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentRuntimeState::new(
            ep_model::IdealLoadsAirSystemId(3),
        );
        let snapshot = advance(&mut state, Route::Assigned, 1, humidity_ratio);
        let expected = energyplus_psy_cp_air_fn_w(humidity_ratio);

        assert_eq!(
            snapshot.zone_humidity_ratio.map(f64::to_bits),
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
fn skipped_routes_do_not_validate_zone_humidity() {
    for (route, unit_off, non_cooling, guard_false) in [
        (Route::UnitOff, true, false, false),
        (Route::NonCooling, false, true, false),
        (Route::GuardFalse, false, false, true),
    ] {
        let mut state = PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentRuntimeState::new(
            ep_model::IdealLoadsAirSystemId(3),
        );
        let snapshot = advance(&mut state, route, 1, f64::from_bits(0x7ff8_0000_0000_00ff));

        assert!(
            cooling_positive_supply_cp_air_assignment_snapshot_is_exact_direct_release(snapshot)
        );
        assert_eq!(snapshot.unit_off_skipped, unit_off);
        assert_eq!(snapshot.non_cooling_skipped, non_cooling);
        assert_eq!(
            snapshot.positive_guard_false_fallthrough_skipped,
            guard_false
        );
        assert!(!snapshot.zone_humidity_ratio_read);
        assert!(snapshot.zone_humidity_ratio.is_none());
        assert!(!snapshot.psychrometric_cp_air_evaluated);
        assert!(!snapshot.cp_air_assigned);
        assert_eq!(state.source_site_execution_count, 0);
    }
}

#[test]
fn counters_partition_all_four_routes_and_count_three_sites_per_assignment() {
    let mut state = PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentRuntimeState::new(
        ep_model::IdealLoadsAirSystemId(3),
    );
    for (ordinal, route) in [
        Route::UnitOff,
        Route::NonCooling,
        Route::GuardFalse,
        Route::Assigned,
    ]
    .into_iter()
    .enumerate()
    {
        advance(&mut state, route, ordinal + 1, 0.008);
    }

    assert_eq!(state.transition_count, 4);
    assert_eq!(state.unit_off_skip_count, 1);
    assert_eq!(state.non_cooling_skip_count, 1);
    assert_eq!(state.positive_guard_false_fallthrough_skip_count, 1);
    assert_eq!(state.cp_air_assignment_count, 1);
    assert_eq!(state.source_site_execution_count, 3);
    assert_eq!(state.zone_humidity_ratio_read_count, 1);
    assert_eq!(state.psychrometric_cp_air_evaluation_count, 1);
    assert_eq!(state.cp_air_assignment_write_count, 1);
}

#[test]
fn bit_exact_matcher_rejects_signed_zero_and_result_corruption() {
    let mut state = PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentRuntimeState::new(
        ep_model::IdealLoadsAirSystemId(3),
    );
    let snapshot = advance(&mut state, Route::Assigned, 1, 0.0);
    let mut negative_zero = snapshot;
    negative_zero.zone_humidity_ratio = Some(-0.0);
    assert_eq!(snapshot, negative_zero);
    assert!(!super::release::snapshots_match_bit_exact(
        snapshot,
        negative_zero
    ));

    let mut forged_result = snapshot;
    forged_result.psychrometric_cp_air_result_j_per_kg_k = forged_result
        .psychrometric_cp_air_result_j_per_kg_k
        .map(|value| f64::from_bits(value.to_bits() + 1));
    assert!(
        !cooling_positive_supply_cp_air_assignment_snapshot_is_exact_direct_release(forged_result)
    );
}
