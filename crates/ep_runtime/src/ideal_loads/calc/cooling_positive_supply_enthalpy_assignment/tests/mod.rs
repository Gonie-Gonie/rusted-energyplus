mod release_corruption;

use super::*;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE_ORDER,
};
use crate::psychrometrics::energyplus_psy_h_fn_tdb_w;

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
    humidity_ratio: f64,
) -> PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot {
    let unit_off = matches!(route, Route::UnitOff);
    let non_cooling = matches!(route, Route::NonCooling);
    let guard_false = matches!(route, Route::GuardFalse);
    let assigned = matches!(route, Route::Assigned);
    let cooling = guard_false || assigned;

    PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot {
        source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE_ORDER,
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
        supply_humidity_ratio_mixed_air_assignment_executed: assigned,
        mixed_air_humidity_ratio_read: assigned,
        mixed_air_humidity_ratio: assigned.then_some(humidity_ratio),
        supply_humidity_ratio_assignment_performed: assigned,
        assigned_supply_humidity_ratio: assigned.then_some(humidity_ratio),
    }
}

fn advance(
    state: &mut PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentRuntimeState,
    route: Route,
    ordinal: usize,
    supply_temperature_c: f64,
    supply_humidity_ratio: f64,
) -> PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot {
    advance_cooling_positive_supply_enthalpy_assignment_state(
        state,
        predecessor(route, ordinal, supply_humidity_ratio),
        matches!(route, Route::Assigned).then_some(
            PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentActiveInput {
                supply_temperature_c,
                supply_humidity_ratio,
            },
        ),
    )
}

#[test]
fn source_order_is_the_exact_four_site_enthalpy_assignment_slice() {
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER,
        &[
            "read-purchased-air-supply-temperature-for-enthalpy",
            "read-purchased-air-supply-humidity-ratio-for-enthalpy",
            "evaluate-psy-h-fn-tdb-w",
            "assign-local-supply-enthalpy",
        ]
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2191"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2195"
    );
}

#[test]
fn pure_transition_matches_canonical_psychrometric_ieee_behavior_bit_exactly() {
    for (supply_temperature_c, supply_humidity_ratio) in [
        (15.0, 0.005),
        (24.0, 0.0),
        (24.0, -0.0),
        (24.0, -1.0),
        (24.0, f64::NEG_INFINITY),
        (24.0, f64::INFINITY),
        (f64::INFINITY, 0.008),
        (f64::NEG_INFINITY, 0.008),
        (f64::from_bits(0x7ff8_0000_0000_00a1), 0.008),
        (24.0, f64::from_bits(0x7ff8_0000_0000_00b2)),
        (f64::MAX, f64::MAX),
    ] {
        let mut state =
            PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentRuntimeState::new(
                ep_model::IdealLoadsAirSystemId(3),
            );
        let snapshot = advance(
            &mut state,
            Route::Assigned,
            1,
            supply_temperature_c,
            supply_humidity_ratio,
        );
        let expected =
            energyplus_psy_h_fn_tdb_w(supply_temperature_c, supply_humidity_ratio);

        assert_eq!(
            snapshot.supply_temperature_c.map(f64::to_bits),
            Some(supply_temperature_c.to_bits())
        );
        assert_eq!(
            snapshot.supply_humidity_ratio.map(f64::to_bits),
            Some(supply_humidity_ratio.to_bits())
        );
        assert_eq!(
            snapshot
                .psychrometric_supply_enthalpy_result_j_per_kg
                .map(f64::to_bits),
            Some(expected.to_bits())
        );
        assert_eq!(
            snapshot.supply_enthalpy_j_per_kg.map(f64::to_bits),
            Some(expected.to_bits())
        );
    }
}

#[test]
fn humidity_floor_and_source_grouping_are_locked_by_bits() {
    let at_floor = energyplus_psy_h_fn_tdb_w(24.0, 1.0e-5);
    for humidity_ratio in [
        f64::NEG_INFINITY,
        -1.0,
        -0.0,
        0.0,
        f64::from_bits(1),
        f64::from_bits(1.0e-5_f64.to_bits() - 1),
        1.0e-5,
    ] {
        assert_eq!(
            energyplus_psy_h_fn_tdb_w(24.0, humidity_ratio).to_bits(),
            at_floor.to_bits()
        );
    }
    assert_eq!(at_floor.to_bits(), 0x40d7_9367_6523_7048);

    let canonical = energyplus_psy_h_fn_tdb_w(15.0, 0.005);
    let legacy_regrouped: f64 =
        1000.0 * (1.004_84 * 15.0 + 0.005 * (2500.94 + 1.858_95 * 15.0));
    assert_eq!(canonical.to_bits(), 0x40db_112e_28f5_c290);
    assert_eq!(legacy_regrouped.to_bits(), 0x40db_112e_28f5_c28f);
    assert_ne!(canonical.to_bits(), legacy_regrouped.to_bits());
}

#[test]
fn exact_validator_accepts_finite_release_domain_including_negative_zero_humidity() {
    for (temperature, humidity_ratio) in [
        (15.0, 0.005),
        (24.0, 0.0),
        (24.0, -0.0),
        (-40.0, f64::MIN_POSITIVE),
        (100.0, 0.2),
    ] {
        let mut state =
            PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentRuntimeState::new(
                ep_model::IdealLoadsAirSystemId(3),
            );
        let snapshot = advance(
            &mut state,
            Route::Assigned,
            1,
            temperature,
            humidity_ratio,
        );
        assert!(
            cooling_positive_supply_enthalpy_assignment_snapshot_is_exact_direct_release(snapshot)
        );
    }
}

#[test]
fn exact_validator_rejects_nonfinite_or_negative_active_domain_and_corruption() {
    for (temperature, humidity_ratio) in [
        (f64::INFINITY, 0.008),
        (f64::NEG_INFINITY, 0.008),
        (f64::NAN, 0.008),
        (24.0, -f64::MIN_POSITIVE),
        (24.0, -1.0),
        (24.0, f64::INFINITY),
        (24.0, f64::NAN),
        (f64::MAX, f64::MAX),
    ] {
        let mut state =
            PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentRuntimeState::new(
                ep_model::IdealLoadsAirSystemId(3),
            );
        let snapshot = advance(
            &mut state,
            Route::Assigned,
            1,
            temperature,
            humidity_ratio,
        );
        assert!(
            !cooling_positive_supply_enthalpy_assignment_snapshot_is_exact_direct_release(snapshot)
        );
    }

    let mut state =
        PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentRuntimeState::new(
            ep_model::IdealLoadsAirSystemId(3),
        );
    let exact = advance(&mut state, Route::Assigned, 1, 15.0, 0.005);
    let mut wrong_result = exact;
    wrong_result.psychrometric_supply_enthalpy_result_j_per_kg =
        wrong_result.psychrometric_supply_enthalpy_result_j_per_kg.map(|value| {
            f64::from_bits(value.to_bits() + 1)
        });
    assert!(
        !cooling_positive_supply_enthalpy_assignment_snapshot_is_exact_direct_release(wrong_result)
    );
    let mut wrong_assignment = exact;
    wrong_assignment.supply_enthalpy_j_per_kg = wrong_assignment
        .supply_enthalpy_j_per_kg
        .map(|value| f64::from_bits(value.to_bits() + 1));
    assert!(
        !cooling_positive_supply_enthalpy_assignment_snapshot_is_exact_direct_release(
            wrong_assignment
        )
    );
}

#[test]
fn skipped_routes_enforce_the_complete_null_operand_firewall() {
    for (route, unit_off, non_cooling, guard_false) in [
        (Route::UnitOff, true, false, false),
        (Route::NonCooling, false, true, false),
        (Route::GuardFalse, false, false, true),
    ] {
        let mut state =
            PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentRuntimeState::new(
                ep_model::IdealLoadsAirSystemId(3),
            );
        let snapshot = advance(&mut state, route, 1, f64::NAN, f64::NAN);
        assert!(
            cooling_positive_supply_enthalpy_assignment_snapshot_is_exact_direct_release(snapshot)
        );
        assert_eq!(snapshot.unit_off_skipped, unit_off);
        assert_eq!(snapshot.non_cooling_skipped, non_cooling);
        assert_eq!(
            snapshot.positive_guard_false_fallthrough_skipped,
            guard_false
        );
        assert!(!snapshot.supply_temperature_for_enthalpy_read);
        assert!(snapshot.supply_temperature_c.is_none());
        assert!(!snapshot.supply_humidity_ratio_for_enthalpy_read);
        assert!(snapshot.supply_humidity_ratio.is_none());
        assert!(!snapshot.psychrometric_supply_enthalpy_evaluated);
        assert!(
            snapshot
                .psychrometric_supply_enthalpy_result_j_per_kg
                .is_none()
        );
        assert!(!snapshot.supply_enthalpy_assigned);
        assert!(snapshot.supply_enthalpy_j_per_kg.is_none());
        assert_eq!(state.source_site_execution_count, 0);
    }
}

#[test]
fn counters_partition_routes_and_count_four_sites_per_assignment() {
    let mut state =
        PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentRuntimeState::new(
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
        advance(&mut state, route, ordinal + 1, 15.0, 0.005);
    }

    assert_eq!(state.transition_count, 4);
    assert_eq!(state.unit_off_skip_count, 1);
    assert_eq!(state.non_cooling_skip_count, 1);
    assert_eq!(state.positive_guard_false_fallthrough_skip_count, 1);
    assert_eq!(state.supply_enthalpy_assignment_count, 1);
    assert_eq!(state.source_site_execution_count, 4);
    assert_eq!(state.supply_temperature_for_enthalpy_read_count, 1);
    assert_eq!(state.supply_humidity_ratio_for_enthalpy_read_count, 1);
    assert_eq!(state.psychrometric_supply_enthalpy_evaluation_count, 1);
    assert_eq!(state.supply_enthalpy_assignment_write_count, 1);
}

#[test]
fn bit_exact_snapshot_matching_detects_signed_zero_and_nan_payload_drift() {
    let mut state =
        PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentRuntimeState::new(
            ep_model::IdealLoadsAirSystemId(3),
        );
    let negative_zero = advance(&mut state, Route::Assigned, 1, 24.0, -0.0);
    let mut positive_zero = negative_zero;
    positive_zero.supply_humidity_ratio = Some(0.0);
    assert!(!super::release::snapshots_match_bit_exact(
        negative_zero,
        positive_zero
    ));

    let mut first_payload = negative_zero;
    first_payload.supply_temperature_c = Some(f64::from_bits(0x7ff8_0000_0000_00a1));
    let mut second_payload = first_payload;
    second_payload.supply_temperature_c = Some(f64::from_bits(0x7ff8_0000_0000_00b2));
    assert!(!super::release::snapshots_match_bit_exact(
        first_payload,
        second_payload
    ));
}
