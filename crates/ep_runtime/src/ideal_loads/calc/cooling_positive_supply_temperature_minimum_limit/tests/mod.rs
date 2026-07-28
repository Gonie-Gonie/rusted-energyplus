mod release_corruption;

use super::*;
use crate::ideal_loads::PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot;

#[derive(Clone, Copy)]
enum Route {
    UnitOff,
    NonCooling,
    GuardFalse,
    Limited,
}

fn predecessor(
    route: Route,
    ordinal: usize,
    supply_temperature_c: f64,
) -> PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot {
    let unit_off = matches!(route, Route::UnitOff);
    let non_cooling = matches!(route, Route::NonCooling);
    let guard_false = matches!(route, Route::GuardFalse);
    let assigned = matches!(route, Route::Limited);
    let cooling = guard_false || assigned;
    let cp_air = 1_000.0;
    let supply_flow = 0.1;
    let denominator = cp_air * supply_flow;
    let zone_temperature = 25.0;
    let quotient = supply_temperature_c - zone_temperature;
    let cooling_load = quotient * denominator;

    PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot {
        source:
            crate::ideal_loads::PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE,
        first_excluded_source:
            crate::ideal_loads::PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order:
            crate::ideal_loads::PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER,
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
        supply_temperature_assignment_executed: assigned,
        zone_cooling_setpoint_load_read: assigned,
        zone_cooling_setpoint_load_w: assigned.then_some(cooling_load),
        cp_air_read: assigned,
        cp_air_j_per_kg_k: assigned.then_some(cp_air),
        supply_mass_flow_rate_read: assigned,
        supply_mass_flow_rate_kg_per_s: assigned.then_some(supply_flow),
        cp_air_times_supply_mass_flow_rate_calculated: assigned,
        cp_air_times_supply_mass_flow_rate_w_per_k: assigned.then_some(denominator),
        zone_cooling_setpoint_load_over_denominator_calculated: assigned,
        zone_cooling_setpoint_load_over_denominator_c: assigned.then_some(quotient),
        zone_node_temperature_read: assigned,
        zone_node_temperature_c: assigned.then_some(zone_temperature),
        supply_temperature_calculated: assigned,
        calculated_supply_temperature_c: assigned.then_some(supply_temperature_c),
        supply_temperature_assigned: assigned,
        supply_temperature_c: assigned.then_some(supply_temperature_c),
    }
}

fn advance(
    state: &mut PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitRuntimeState,
    route: Route,
    ordinal: usize,
    left: f64,
    right: f64,
) -> PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitSnapshot {
    advance_cooling_positive_supply_temperature_minimum_limit_state(
        state,
        predecessor(route, ordinal, left),
        matches!(route, Route::Limited).then_some(
            PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitActiveInput {
                supply_temperature_before_minimum_limit_c: left,
                minimum_cooling_supply_air_temperature_c: right,
            },
        ),
    )
}

#[test]
fn positive_route_executes_exact_four_sites_and_retains_each_operand_and_result() {
    let mut state =
        PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitRuntimeState::new(
            ep_model::IdealLoadsAirSystemId(3),
        );
    let snapshot = advance(&mut state, Route::Limited, 1, 12.5, 14.0);

    assert!(
        cooling_positive_supply_temperature_minimum_limit_snapshot_is_exact_direct_release(
            snapshot
        )
    );
    assert_eq!(
        snapshot.source_order,
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MINIMUM_LIMIT_SOURCE_ORDER
    );
    assert_eq!(
        snapshot
            .supply_temperature_before_minimum_limit_c
            .map(f64::to_bits),
        Some(12.5_f64.to_bits())
    );
    assert_eq!(
        snapshot
            .minimum_cooling_supply_air_temperature_c
            .map(f64::to_bits),
        Some(14.0_f64.to_bits())
    );
    assert_eq!(
        snapshot.maximum_supply_temperature_c.map(f64::to_bits),
        Some(14.0_f64.to_bits())
    );
    assert_eq!(
        snapshot.assigned_supply_temperature_c.map(f64::to_bits),
        Some(14.0_f64.to_bits())
    );
    assert_eq!(state.source_site_execution_count, 4);
    assert_eq!(state.supply_temperature_for_maximum_read_count, 1);
    assert_eq!(
        state.minimum_cooling_supply_air_temperature_for_maximum_read_count,
        1
    );
    assert_eq!(
        state.source_shaped_two_argument_maximum_evaluation_count,
        1
    );
    assert_eq!(state.supply_temperature_assignment_count, 1);
}

#[test]
fn source_shaped_maximum_is_left_biased_for_ties_and_unordered_comparisons() {
    let left_nan = f64::from_bits(0x7ff8_0000_0000_00a1);
    let right_nan = f64::from_bits(0x7ff8_0000_0000_00b2);

    for (left, right, expected) in [
        (12.0, 14.0, 14.0),
        (14.0, 12.0, 14.0),
        (-0.0, 0.0, -0.0),
        (0.0, -0.0, 0.0),
        (f64::NEG_INFINITY, 1.0, 1.0),
        (f64::INFINITY, 1.0, f64::INFINITY),
        (f64::NEG_INFINITY, f64::INFINITY, f64::INFINITY),
        (left_nan, 1.0, left_nan),
        (1.0, right_nan, 1.0),
        (left_nan, right_nan, left_nan),
    ] {
        assert_eq!(
            source_shaped_two_argument_maximum(left, right).to_bits(),
            expected.to_bits()
        );
    }
}

#[test]
fn exact_validator_admits_any_left_ieee_value_but_requires_finite_typed_minimum() {
    let left_nan = f64::from_bits(0x7ff8_0000_0000_00a1);
    let mut state =
        PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitRuntimeState::new(
            ep_model::IdealLoadsAirSystemId(3),
        );
    let left_nan_snapshot = advance(&mut state, Route::Limited, 1, left_nan, 14.0);
    assert!(
        cooling_positive_supply_temperature_minimum_limit_snapshot_is_exact_direct_release(
            left_nan_snapshot
        )
    );
    assert_eq!(
        left_nan_snapshot
            .assigned_supply_temperature_c
            .map(f64::to_bits),
        Some(left_nan.to_bits())
    );

    for right in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let mut state =
            PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitRuntimeState::new(
                ep_model::IdealLoadsAirSystemId(3),
            );
        let snapshot = advance(&mut state, Route::Limited, 1, 12.0, right);
        assert!(
            !cooling_positive_supply_temperature_minimum_limit_snapshot_is_exact_direct_release(
                snapshot
            )
        );
    }
}

#[test]
fn skipped_routes_execute_no_sites_or_operand_reads() {
    for (route, unit_off, non_cooling, guard_false) in [
        (Route::UnitOff, true, false, false),
        (Route::NonCooling, false, true, false),
        (Route::GuardFalse, false, false, true),
    ] {
        let mut state =
            PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitRuntimeState::new(
                ep_model::IdealLoadsAirSystemId(3),
            );
        let snapshot = advance(&mut state, route, 1, f64::NAN, f64::NAN);

        assert!(
            cooling_positive_supply_temperature_minimum_limit_snapshot_is_exact_direct_release(
                snapshot
            )
        );
        assert_eq!(snapshot.unit_off_skipped, unit_off);
        assert_eq!(snapshot.non_cooling_skipped, non_cooling);
        assert_eq!(
            snapshot.positive_guard_false_fallthrough_skipped,
            guard_false
        );
        assert!(!snapshot.supply_temperature_for_maximum_read);
        assert!(
            snapshot
                .supply_temperature_before_minimum_limit_c
                .is_none()
        );
        assert!(!snapshot.minimum_cooling_supply_air_temperature_for_maximum_read);
        assert!(
            snapshot
                .minimum_cooling_supply_air_temperature_c
                .is_none()
        );
        assert!(!snapshot.source_shaped_two_argument_maximum_evaluated);
        assert!(snapshot.maximum_supply_temperature_c.is_none());
        assert!(!snapshot.supply_temperature_assignment_performed);
        assert!(snapshot.assigned_supply_temperature_c.is_none());
        assert_eq!(state.source_site_execution_count, 0);
    }
}

#[test]
fn counters_partition_all_four_routes_and_count_four_sites_per_limit() {
    let mut state =
        PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitRuntimeState::new(
            ep_model::IdealLoadsAirSystemId(3),
        );
    for (ordinal, route) in [
        Route::UnitOff,
        Route::NonCooling,
        Route::GuardFalse,
        Route::Limited,
    ]
    .into_iter()
    .enumerate()
    {
        advance(&mut state, route, ordinal + 1, 12.0, 14.0);
    }

    assert_eq!(state.transition_count, 4);
    assert_eq!(state.unit_off_skip_count, 1);
    assert_eq!(state.non_cooling_skip_count, 1);
    assert_eq!(state.positive_guard_false_fallthrough_skip_count, 1);
    assert_eq!(state.supply_temperature_minimum_limit_count, 1);
    assert_eq!(state.source_site_execution_count, 4);
    assert_eq!(state.supply_temperature_for_maximum_read_count, 1);
    assert_eq!(
        state.minimum_cooling_supply_air_temperature_for_maximum_read_count,
        1
    );
    assert_eq!(
        state.source_shaped_two_argument_maximum_evaluation_count,
        1
    );
    assert_eq!(state.supply_temperature_assignment_count, 1);
}

#[test]
fn bit_exact_snapshot_matching_detects_signed_zero_and_nan_payload_corruption() {
    let mut state =
        PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitRuntimeState::new(
            ep_model::IdealLoadsAirSystemId(3),
        );
    let negative_zero = advance(&mut state, Route::Limited, 1, -0.0, 0.0);
    let mut positive_zero = negative_zero;
    positive_zero.supply_temperature_before_minimum_limit_c = Some(0.0);
    positive_zero.maximum_supply_temperature_c = Some(0.0);
    positive_zero.assigned_supply_temperature_c = Some(0.0);
    assert!(
        !super::release::snapshots_match_bit_exact(negative_zero, positive_zero)
    );

    let left_nan = f64::from_bits(0x7ff8_0000_0000_00a1);
    let mut state =
        PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitRuntimeState::new(
            ep_model::IdealLoadsAirSystemId(3),
        );
    let first_payload = advance(&mut state, Route::Limited, 1, left_nan, 14.0);
    let second_nan = f64::from_bits(0x7ff8_0000_0000_00b2);
    let mut second_payload = first_payload;
    second_payload.supply_temperature_before_minimum_limit_c = Some(second_nan);
    second_payload.maximum_supply_temperature_c = Some(second_nan);
    second_payload.assigned_supply_temperature_c = Some(second_nan);
    assert!(
        !super::release::snapshots_match_bit_exact(first_payload, second_payload)
    );
}

#[test]
fn exact_validator_rejects_result_and_null_firewall_corruption() {
    let mut state =
        PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitRuntimeState::new(
            ep_model::IdealLoadsAirSystemId(3),
        );
    let exact = advance(&mut state, Route::Limited, 1, 12.0, 14.0);

    let mut wrong_maximum = exact;
    wrong_maximum.maximum_supply_temperature_c = Some(12.0);
    assert!(
        !cooling_positive_supply_temperature_minimum_limit_snapshot_is_exact_direct_release(
            wrong_maximum
        )
    );

    let mut wrong_assignment = exact;
    wrong_assignment.assigned_supply_temperature_c = Some(12.0);
    assert!(
        !cooling_positive_supply_temperature_minimum_limit_snapshot_is_exact_direct_release(
            wrong_assignment
        )
    );

    let mut state =
        PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitRuntimeState::new(
            ep_model::IdealLoadsAirSystemId(3),
        );
    let mut skipped = advance(&mut state, Route::UnitOff, 1, 12.0, 14.0);
    skipped.supply_temperature_before_minimum_limit_c = Some(12.0);
    assert!(
        !cooling_positive_supply_temperature_minimum_limit_snapshot_is_exact_direct_release(
            skipped
        )
    );
}
