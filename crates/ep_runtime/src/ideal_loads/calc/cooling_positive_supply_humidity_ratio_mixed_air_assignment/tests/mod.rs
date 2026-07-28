mod release_corruption;

use super::*;
use crate::ideal_loads::PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot;

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
) -> PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot {
    let unit_off = matches!(route, Route::UnitOff);
    let non_cooling = matches!(route, Route::NonCooling);
    let guard_false = matches!(route, Route::GuardFalse);
    let assigned = matches!(route, Route::Assigned);
    let cooling = guard_false || assigned;

    PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot {
        source:
            crate::ideal_loads::PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE,
        first_excluded_source:
            crate::ideal_loads::PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
        source_order:
            crate::ideal_loads::PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER,
        system: ep_model::IdealLoadsAirSystemId(3),
        parent_call_ordinal: ordinal,
        controlled_zone: ep_model::ZoneId(4),
        unit_body_entered: !unit_off,
        predecessor_cooling_body_entered: cooling,
        predecessor_no_outdoor_air_fallback_entered: cooling,
        predecessor_positive_supply_mass_flow_body_entered: assigned,
        predecessor_active_guard_false_fallthrough: guard_false,
        unit_off_skipped: unit_off,
        non_cooling_skipped: non_cooling,
        positive_guard_false_fallthrough_skipped: guard_false,
        supply_temperature_mixed_air_limit_executed: assigned,
        supply_temperature_for_minimum_read: assigned,
        supply_temperature_before_mixed_air_limit_c: assigned.then_some(18.0),
        mixed_air_temperature_for_minimum_read: assigned,
        mixed_air_temperature_c: assigned.then_some(17.0),
        source_shaped_two_argument_minimum_evaluated: assigned,
        minimum_supply_temperature_c: assigned.then_some(17.0),
        supply_temperature_assignment_performed: assigned,
        assigned_supply_temperature_c: assigned.then_some(17.0),
    }
}

fn advance(
    state: &mut PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentRuntimeState,
    route: Route,
    ordinal: usize,
    humidity_ratio: f64,
) -> PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot {
    advance_cooling_positive_supply_humidity_ratio_mixed_air_assignment_state(
        state,
        predecessor(route, ordinal),
        matches!(route, Route::Assigned).then_some(
            PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentActiveInput {
                mixed_air_humidity_ratio: humidity_ratio,
            },
        ),
    )
}

#[test]
fn source_order_is_the_exact_two_site_assignment_slice() {
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE_ORDER,
        &[
            "read-purchased-air-mixed-air-humidity-ratio",
            "assign-purchased-air-supply-humidity-ratio",
        ]
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2190"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2191"
    );
}

#[test]
fn pure_transition_copies_every_ieee_bit_pattern_without_normalization() {
    for humidity_ratio in [
        0.012,
        0.0,
        -0.0,
        -1.0,
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::from_bits(0x7ff8_0000_0000_00a1),
        f64::from_bits(0xfff8_0000_0000_00b2),
    ] {
        let mut state =
            PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentRuntimeState::new(
                ep_model::IdealLoadsAirSystemId(3),
            );
        let snapshot = advance(&mut state, Route::Assigned, 1, humidity_ratio);
        assert_eq!(
            snapshot.mixed_air_humidity_ratio.map(f64::to_bits),
            Some(humidity_ratio.to_bits())
        );
        assert_eq!(
            snapshot.assigned_supply_humidity_ratio.map(f64::to_bits),
            Some(humidity_ratio.to_bits())
        );
    }
}

#[test]
fn exact_validator_accepts_finite_nonnegative_values_including_negative_zero() {
    for humidity_ratio in [0.0, -0.0, f64::MIN_POSITIVE, 0.012, f64::MAX] {
        let mut state =
            PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentRuntimeState::new(
                ep_model::IdealLoadsAirSystemId(3),
            );
        let snapshot = advance(&mut state, Route::Assigned, 1, humidity_ratio);
        assert!(
            cooling_positive_supply_humidity_ratio_mixed_air_assignment_snapshot_is_exact_direct_release(
                snapshot
            )
        );
    }
}

#[test]
fn exact_validator_rejects_negative_and_nonfinite_active_values() {
    for humidity_ratio in [
        -f64::MIN_POSITIVE,
        -1.0,
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::from_bits(0x7ff8_0000_0000_00a1),
    ] {
        let mut state =
            PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentRuntimeState::new(
                ep_model::IdealLoadsAirSystemId(3),
            );
        let snapshot = advance(&mut state, Route::Assigned, 1, humidity_ratio);
        assert!(
            !cooling_positive_supply_humidity_ratio_mixed_air_assignment_snapshot_is_exact_direct_release(
                snapshot
            )
        );
    }
}

#[test]
fn skipped_routes_enforce_the_null_operand_firewall() {
    for (route, unit_off, non_cooling, guard_false) in [
        (Route::UnitOff, true, false, false),
        (Route::NonCooling, false, true, false),
        (Route::GuardFalse, false, false, true),
    ] {
        let mut state =
            PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentRuntimeState::new(
                ep_model::IdealLoadsAirSystemId(3),
            );
        let snapshot = advance(&mut state, route, 1, f64::NAN);
        assert!(
            cooling_positive_supply_humidity_ratio_mixed_air_assignment_snapshot_is_exact_direct_release(
                snapshot
            )
        );
        assert_eq!(snapshot.unit_off_skipped, unit_off);
        assert_eq!(snapshot.non_cooling_skipped, non_cooling);
        assert_eq!(
            snapshot.positive_guard_false_fallthrough_skipped,
            guard_false
        );
        assert!(!snapshot.mixed_air_humidity_ratio_read);
        assert!(snapshot.mixed_air_humidity_ratio.is_none());
        assert!(!snapshot.supply_humidity_ratio_assignment_performed);
        assert!(snapshot.assigned_supply_humidity_ratio.is_none());
        assert_eq!(state.source_site_execution_count, 0);
    }
}

#[test]
fn counters_partition_all_four_routes_and_count_two_sites_per_assignment() {
    let mut state =
        PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentRuntimeState::new(
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
        advance(&mut state, route, ordinal + 1, 0.012);
    }

    assert_eq!(state.transition_count, 4);
    assert_eq!(state.unit_off_skip_count, 1);
    assert_eq!(state.non_cooling_skip_count, 1);
    assert_eq!(state.positive_guard_false_fallthrough_skip_count, 1);
    assert_eq!(state.supply_humidity_ratio_mixed_air_assignment_count, 1);
    assert_eq!(state.source_site_execution_count, 2);
    assert_eq!(state.mixed_air_humidity_ratio_read_count, 1);
    assert_eq!(state.supply_humidity_ratio_assignment_count, 1);
}

#[test]
fn bit_exact_snapshot_matching_detects_signed_zero_and_nan_payload_drift() {
    let mut state =
        PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentRuntimeState::new(
            ep_model::IdealLoadsAirSystemId(3),
        );
    let negative_zero = advance(&mut state, Route::Assigned, 1, -0.0);
    let mut positive_zero = negative_zero;
    positive_zero.mixed_air_humidity_ratio = Some(0.0);
    positive_zero.assigned_supply_humidity_ratio = Some(0.0);
    assert!(!super::release::snapshots_match_bit_exact(
        negative_zero,
        positive_zero
    ));

    let mut first_payload = negative_zero;
    first_payload.mixed_air_humidity_ratio = Some(f64::from_bits(0x7ff8_0000_0000_00a1));
    first_payload.assigned_supply_humidity_ratio = Some(f64::from_bits(0x7ff8_0000_0000_00a1));
    let mut second_payload = first_payload;
    second_payload.mixed_air_humidity_ratio = Some(f64::from_bits(0x7ff8_0000_0000_00b2));
    second_payload.assigned_supply_humidity_ratio = Some(f64::from_bits(0x7ff8_0000_0000_00b2));
    assert!(!super::release::snapshots_match_bit_exact(
        first_payload,
        second_payload
    ));
}

#[test]
fn exact_validator_rejects_assignment_and_null_firewall_corruption() {
    let mut state =
        PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentRuntimeState::new(
            ep_model::IdealLoadsAirSystemId(3),
        );
    let exact = advance(&mut state, Route::Assigned, 1, 0.012);
    let mut wrong_assignment = exact;
    wrong_assignment.assigned_supply_humidity_ratio = Some(0.013);
    assert!(
        !cooling_positive_supply_humidity_ratio_mixed_air_assignment_snapshot_is_exact_direct_release(
            wrong_assignment
        )
    );

    let mut state =
        PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentRuntimeState::new(
            ep_model::IdealLoadsAirSystemId(3),
        );
    let mut skipped = advance(&mut state, Route::GuardFalse, 1, 0.012);
    skipped.mixed_air_humidity_ratio_read = true;
    skipped.mixed_air_humidity_ratio = Some(0.012);
    assert!(
        !cooling_positive_supply_humidity_ratio_mixed_air_assignment_snapshot_is_exact_direct_release(
            skipped
        )
    );
}
