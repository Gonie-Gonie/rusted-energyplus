mod release_corruption;

use super::*;
use crate::ideal_loads::PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentSnapshot;

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
    cp_air_j_per_kg_k: f64,
) -> PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentSnapshot {
    let unit_off = matches!(route, Route::UnitOff);
    let non_cooling = matches!(route, Route::NonCooling);
    let guard_false = matches!(route, Route::GuardFalse);
    let assigned = matches!(route, Route::Assigned);
    let cooling = guard_false || assigned;
    PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentSnapshot {
        source:
            crate::ideal_loads::PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CP_AIR_ASSIGNMENT_SOURCE,
        first_excluded_source:
            crate::ideal_loads::PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order:
            crate::ideal_loads::PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CP_AIR_ASSIGNMENT_SOURCE_ORDER,
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
        cp_air_assignment_executed: assigned,
        zone_humidity_ratio_read: assigned,
        zone_humidity_ratio: assigned.then_some(0.0),
        psychrometric_cp_air_evaluated: assigned,
        psychrometric_cp_air_result_j_per_kg_k: assigned.then_some(cp_air_j_per_kg_k),
        cp_air_assigned: assigned,
        cp_air_j_per_kg_k: assigned.then_some(cp_air_j_per_kg_k),
    }
}

fn advance(
    state: &mut PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentRuntimeState,
    route: Route,
    ordinal: usize,
    active_input: PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentActiveInput,
) -> PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot {
    advance_cooling_positive_supply_temperature_assignment_state(
        state,
        predecessor(route, ordinal, active_input.cp_air_j_per_kg_k),
        matches!(route, Route::Assigned).then_some(active_input),
    )
}

fn ordinary_active_input() -> PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentActiveInput
{
    PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentActiveInput {
        zone_cooling_setpoint_load_w: -1_000.0,
        cp_air_j_per_kg_k: 1_010.0,
        supply_mass_flow_rate_kg_per_s: 0.25,
        zone_node_temperature_c: 25.0,
    }
}

fn recalculate_derived_fields(
    mut snapshot: PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot,
) -> PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot {
    let denominator = snapshot.cp_air_j_per_kg_k.expect("CpAir")
        * snapshot
            .supply_mass_flow_rate_kg_per_s
            .expect("supply mass flow");
    let quotient = snapshot
        .zone_cooling_setpoint_load_w
        .expect("cooling setpoint load")
        / denominator;
    let calculated = quotient
        + snapshot
            .zone_node_temperature_c
            .expect("Zone-node temperature");
    snapshot.cp_air_times_supply_mass_flow_rate_w_per_k = Some(denominator);
    snapshot.zone_cooling_setpoint_load_over_denominator_c = Some(quotient);
    snapshot.calculated_supply_temperature_c = Some(calculated);
    snapshot.supply_temperature_c = Some(calculated);
    snapshot
}

#[test]
fn positive_route_executes_exact_eight_sites_and_retains_each_ieee_intermediate() {
    let input = ordinary_active_input();
    let mut state = PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentRuntimeState::new(
        ep_model::IdealLoadsAirSystemId(3),
    );
    let snapshot = advance(&mut state, Route::Assigned, 1, input);
    let denominator = input.cp_air_j_per_kg_k * input.supply_mass_flow_rate_kg_per_s;
    let quotient = input.zone_cooling_setpoint_load_w / denominator;
    let expected = quotient + input.zone_node_temperature_c;

    assert!(
        cooling_positive_supply_temperature_assignment_snapshot_is_exact_direct_release(snapshot)
    );
    assert_eq!(
        snapshot.source_order,
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER
    );
    assert_eq!(
        snapshot
            .cp_air_times_supply_mass_flow_rate_w_per_k
            .map(f64::to_bits),
        Some(denominator.to_bits())
    );
    assert_eq!(
        snapshot
            .zone_cooling_setpoint_load_over_denominator_c
            .map(f64::to_bits),
        Some(quotient.to_bits())
    );
    assert_eq!(
        snapshot.supply_temperature_c.map(f64::to_bits),
        Some(expected.to_bits())
    );
    assert_eq!(state.source_site_execution_count, 8);
}

#[test]
fn source_grouping_uses_product_then_division_without_reassociation() {
    let input = PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentActiveInput {
        zone_cooling_setpoint_load_w: 1.0e308,
        cp_air_j_per_kg_k: 1.0e308,
        supply_mass_flow_rate_kg_per_s: 2.0,
        zone_node_temperature_c: 0.0,
    };
    let mut state = PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentRuntimeState::new(
        ep_model::IdealLoadsAirSystemId(3),
    );
    let snapshot = advance(&mut state, Route::Assigned, 1, input);
    let grouped = input.zone_cooling_setpoint_load_w
        / (input.cp_air_j_per_kg_k * input.supply_mass_flow_rate_kg_per_s);
    let reassociated = (input.zone_cooling_setpoint_load_w / input.cp_air_j_per_kg_k)
        / input.supply_mass_flow_rate_kg_per_s;

    assert_eq!(
        snapshot
            .zone_cooling_setpoint_load_over_denominator_c
            .map(f64::to_bits),
        Some(grouped.to_bits())
    );
    assert_ne!(grouped.to_bits(), reassociated.to_bits());
    assert!(
        cooling_positive_supply_temperature_assignment_snapshot_is_exact_direct_release(snapshot)
    );
}

#[test]
fn admitted_operands_preserve_infinite_derived_results_and_signed_zero_quotients() {
    for (supply_flow, expected_quotient) in [
        (f64::from_bits(1), f64::NEG_INFINITY),
        (f64::INFINITY, -0.0),
    ] {
        let input = PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentActiveInput {
            zone_cooling_setpoint_load_w: -1_000.0,
            cp_air_j_per_kg_k: 1_000.0,
            supply_mass_flow_rate_kg_per_s: supply_flow,
            zone_node_temperature_c: 25.0,
        };
        let mut state = PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentRuntimeState::new(
            ep_model::IdealLoadsAirSystemId(3),
        );
        let snapshot = advance(&mut state, Route::Assigned, 1, input);

        assert_eq!(
            snapshot
                .zone_cooling_setpoint_load_over_denominator_c
                .map(f64::to_bits),
            Some(expected_quotient.to_bits())
        );
        assert!(
            cooling_positive_supply_temperature_assignment_snapshot_is_exact_direct_release(
                snapshot
            )
        );
    }
}

#[test]
fn pure_characterization_retains_raw_nan_intermediates_but_direct_validator_rejects_them() {
    let input = PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentActiveInput {
        zone_cooling_setpoint_load_w: -1_000.0,
        cp_air_j_per_kg_k: f64::from_bits(0x7ff8_0000_0000_00a1),
        supply_mass_flow_rate_kg_per_s: 0.25,
        zone_node_temperature_c: 25.0,
    };
    let mut state = PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentRuntimeState::new(
        ep_model::IdealLoadsAirSystemId(3),
    );
    let snapshot = advance(&mut state, Route::Assigned, 1, input);
    let denominator = input.cp_air_j_per_kg_k * input.supply_mass_flow_rate_kg_per_s;
    let quotient = input.zone_cooling_setpoint_load_w / denominator;
    let sum = quotient + input.zone_node_temperature_c;

    assert!(denominator.is_nan());
    assert_eq!(
        snapshot
            .cp_air_times_supply_mass_flow_rate_w_per_k
            .map(f64::to_bits),
        Some(denominator.to_bits())
    );
    assert_eq!(
        snapshot
            .zone_cooling_setpoint_load_over_denominator_c
            .map(f64::to_bits),
        Some(quotient.to_bits())
    );
    assert_eq!(
        snapshot.calculated_supply_temperature_c.map(f64::to_bits),
        Some(sum.to_bits())
    );
    assert_eq!(
        snapshot.supply_temperature_c.map(f64::to_bits),
        Some(sum.to_bits())
    );
    assert!(
        !cooling_positive_supply_temperature_assignment_snapshot_is_exact_direct_release(snapshot)
    );
}

#[test]
fn skipped_routes_execute_no_sites_or_operand_reads() {
    for (route, unit_off, non_cooling, guard_false) in [
        (Route::UnitOff, true, false, false),
        (Route::NonCooling, false, true, false),
        (Route::GuardFalse, false, false, true),
    ] {
        let mut state = PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentRuntimeState::new(
            ep_model::IdealLoadsAirSystemId(3),
        );
        let snapshot = advance(&mut state, route, 1, ordinary_active_input());

        assert!(
            cooling_positive_supply_temperature_assignment_snapshot_is_exact_direct_release(
                snapshot
            )
        );
        assert_eq!(snapshot.unit_off_skipped, unit_off);
        assert_eq!(snapshot.non_cooling_skipped, non_cooling);
        assert_eq!(
            snapshot.positive_guard_false_fallthrough_skipped,
            guard_false
        );
        assert!(!snapshot.zone_cooling_setpoint_load_read);
        assert!(!snapshot.cp_air_read);
        assert!(!snapshot.supply_mass_flow_rate_read);
        assert!(!snapshot.zone_node_temperature_read);
        assert_eq!(state.source_site_execution_count, 0);
    }
}

#[test]
fn counters_partition_all_four_routes_and_count_eight_sites_per_assignment() {
    let mut state = PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentRuntimeState::new(
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
        advance(&mut state, route, ordinal + 1, ordinary_active_input());
    }

    assert_eq!(state.transition_count, 4);
    assert_eq!(state.unit_off_skip_count, 1);
    assert_eq!(state.non_cooling_skip_count, 1);
    assert_eq!(state.positive_guard_false_fallthrough_skip_count, 1);
    assert_eq!(state.supply_temperature_assignment_count, 1);
    assert_eq!(state.source_site_execution_count, 8);
    assert_eq!(state.zone_cooling_setpoint_load_read_count, 1);
    assert_eq!(state.cp_air_read_count, 1);
    assert_eq!(state.supply_mass_flow_rate_read_count, 1);
    assert_eq!(
        state.cp_air_times_supply_mass_flow_rate_calculation_count,
        1
    );
    assert_eq!(
        state.zone_cooling_setpoint_load_over_denominator_calculation_count,
        1
    );
    assert_eq!(state.zone_node_temperature_read_count, 1);
    assert_eq!(state.supply_temperature_calculation_count, 1);
    assert_eq!(state.supply_temperature_assignment_write_count, 1);
}

#[test]
fn standalone_exact_validator_rejects_nonfinite_or_nonpositive_source_operands() {
    let mut state = PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentRuntimeState::new(
        ep_model::IdealLoadsAirSystemId(3),
    );
    let exact = advance(&mut state, Route::Assigned, 1, ordinary_active_input());

    for forged in [
        {
            let mut forged = exact;
            forged.zone_cooling_setpoint_load_w = Some(f64::NAN);
            recalculate_derived_fields(forged)
        },
        {
            let mut forged = exact;
            forged.cp_air_j_per_kg_k = Some(f64::INFINITY);
            recalculate_derived_fields(forged)
        },
        {
            let mut forged = exact;
            forged.cp_air_j_per_kg_k = Some(0.0);
            recalculate_derived_fields(forged)
        },
        {
            let mut forged = exact;
            forged.supply_mass_flow_rate_kg_per_s = Some(0.0);
            recalculate_derived_fields(forged)
        },
        {
            let mut forged = exact;
            forged.zone_node_temperature_c = Some(f64::NAN);
            recalculate_derived_fields(forged)
        },
    ] {
        assert!(
            !cooling_positive_supply_temperature_assignment_snapshot_is_exact_direct_release(
                forged
            )
        );
    }
}
