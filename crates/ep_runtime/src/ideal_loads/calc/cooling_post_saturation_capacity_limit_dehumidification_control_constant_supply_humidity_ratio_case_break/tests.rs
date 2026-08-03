//! CP409 boundary, route, carrier, accounting, corruption, and overflow tests.

use ep_model::{DehumidificationControlType as D, IdealLoadsAirSystemId, ZoneId};

use super::release::{
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_break_snapshot_is_exact_direct_release,
    test_counts_are_exact,
};
use super::transition::routes::{
    RetainedRoute, logical_route_index, predecessor_index_is_active, predecessor_index_is_public,
};
use super::transition::{test_increment_counts, test_next_transition_fits};
use super::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_FIRST_EXCLUDED_SOURCE as EXCLUDED,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_SOURCE as SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_SOURCE_ORDER as ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCaseBreakRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCaseBreakSnapshot as Snapshot,
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_break_snapshot_route as snapshot_route,
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_break_snapshots_match_bit_exact as snapshots_match_bit_exact,
};

#[test]
fn cp409_boundary_and_single_break_site_are_exact() {
    assert_eq!(SOURCE, "EnergyPlus 26.1 PurchasedAirManager.cc:2306");
    assert_eq!(EXCLUDED, "EnergyPlus 26.1 PurchasedAirManager.cc:2308");
    assert_eq!(
        ORDER,
        &[
            "exit-purchased-air-post-saturation-capacity-limit-dehumidification-control-none-or-constant-supply-humidity-ratio-shared-case-via-break"
        ],
    );
}

fn all_routes() -> Vec<RetainedRoute> {
    let mut routes = Vec::new();
    for predecessor_index in 0..30 {
        if predecessor_index_is_active(predecessor_index) {
            routes.push(RetainedRoute {
                predecessor_index,
                predecessor_guard_false_fallthrough: true,
                predecessor_maximum_capacity_assignment_executed: false,
                active: true,
            });
            routes.push(RetainedRoute {
                predecessor_index,
                predecessor_guard_false_fallthrough: false,
                predecessor_maximum_capacity_assignment_executed: true,
                active: true,
            });
        } else {
            routes.push(RetainedRoute {
                predecessor_index,
                predecessor_guard_false_fallthrough: false,
                predecessor_maximum_capacity_assignment_executed: false,
                active: false,
            });
        }
    }
    routes
}

#[test]
fn cp409_preserves_thirty_six_routes_and_breaks_on_both_guard_outcomes() {
    let routes = all_routes();
    assert_eq!(routes.len(), 36);
    assert_eq!(
        routes
            .iter()
            .copied()
            .map(logical_route_index)
            .collect::<Vec<_>>(),
        (0..36).collect::<Vec<_>>(),
    );
    assert_eq!(
        routes
            .iter()
            .copied()
            .filter(|route| route.active)
            .map(logical_route_index)
            .collect::<Vec<_>>(),
        [20, 21, 22, 23, 26, 27, 28, 29, 31, 32, 34, 35],
    );
    assert_eq!(
        routes
            .iter()
            .filter(|route| predecessor_index_is_public(route.predecessor_index))
            .count(),
        13,
    );
    assert_eq!(
        routes
            .iter()
            .copied()
            .filter(|route| {
                route.active && predecessor_index_is_public(route.predecessor_index)
            })
            .map(logical_route_index)
            .collect::<Vec<_>>(),
        [20, 21, 26, 27],
    );
}

fn exhaustive_state() -> State {
    let mut state = State::new(IdealLoadsAirSystemId(409));
    for route in all_routes() {
        assert!(test_next_transition_fits(&state, route));
        state.transition_count += 1;
        test_increment_counts(&mut state, route);
    }
    state
}

#[test]
fn exhaustive_accounting_is_36_24_12_12() {
    let state = exhaustive_state();
    assert!(test_counts_are_exact(&state));
    assert_eq!(state.transition_count, 36);
    assert_eq!(state.inactive_transition_count, 24);
    assert_eq!(state.predecessor_guard_false_fallthrough_count, 6);
    assert_eq!(state.predecessor_maximum_capacity_assignment_count, 6);
    assert_eq!(
        state.dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_break_count,
        12,
    );
    assert_eq!(state.source_site_execution_count, 12);
    for index in 0..30 {
        if predecessor_index_is_active(index) {
            assert_eq!(state.predecessor_route_counts[index], 2);
            assert_eq!(
                state.predecessor_guard_false_fallthrough_route_counts[index],
                1
            );
            assert_eq!(
                state.predecessor_maximum_capacity_assignment_route_counts[index],
                1
            );
        } else {
            assert_eq!(state.predecessor_route_counts[index], 1);
            assert_eq!(
                state.predecessor_guard_false_fallthrough_route_counts[index],
                0
            );
            assert_eq!(
                state.predecessor_maximum_capacity_assignment_route_counts[index],
                0
            );
        }
    }
}

#[test]
fn compressed_snapshot_accepts_both_reconverged_public_break_routes() {
    let guard_false = shared_snapshot(false);
    let maximum = shared_snapshot(true);
    let Some(guard_route) = snapshot_route(guard_false) else {
        assert!(
            snapshot_route(guard_false).is_some(),
            "guard-false CP409 route must be retained"
        );
        return;
    };
    let Some(maximum_route) = snapshot_route(maximum) else {
        assert!(
            snapshot_route(maximum).is_some(),
            "maximum CP409 route must be retained"
        );
        return;
    };
    assert_eq!(logical_route_index(guard_route), 20);
    assert_eq!(logical_route_index(maximum_route), 21);
    assert!(guard_route.active && maximum_route.active);
    assert!(cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_break_snapshot_is_exact_direct_release(guard_false));
    assert!(cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_break_snapshot_is_exact_direct_release(maximum));
}

#[test]
fn compressed_terminal_carriers_preserve_arbitrary_ieee_bits() {
    let mut snapshot = shared_snapshot(false);
    let humidity = f64::from_bits(0x7ff8_0000_0000_0409);
    let enthalpy = -0.0_f64;
    let temperature = f64::INFINITY;
    snapshot.predecessor_cp408_resulting_supply_humidity_ratio = Some(humidity);
    snapshot.resulting_supply_humidity_ratio = Some(humidity);
    snapshot.predecessor_cp408_resulting_supply_enthalpy_j_per_kg = Some(enthalpy);
    snapshot.resulting_supply_enthalpy_j_per_kg = Some(enthalpy);
    snapshot.predecessor_cp408_resulting_supply_temperature_c = Some(temperature);
    snapshot.resulting_supply_temperature_c = Some(temperature);
    assert!(snapshot_route(snapshot).is_some());

    let mut drift = snapshot;
    drift.resulting_supply_humidity_ratio = Some(f64::from_bits(humidity.to_bits() ^ 1));
    assert!(snapshot_route(drift).is_none());
    assert!(!snapshots_match_bit_exact(snapshot, drift));
}

#[test]
fn inactive_route_and_corruption_are_fail_closed() {
    let inactive = inactive_snapshot();
    let Some(route) = snapshot_route(inactive) else {
        assert!(
            snapshot_route(inactive).is_some(),
            "inactive CP409 route must be retained"
        );
        return;
    };
    assert_eq!(logical_route_index(route), 0);
    assert!(!route.active);

    let mut forged = shared_snapshot(false);
    forged
        .dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_exited_via_break = false;
    assert!(snapshot_route(forged).is_none());

    let mut forged = shared_snapshot(false);
    forged
        .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed = true;
    assert!(snapshot_route(forged).is_none());
}

#[test]
fn every_incremented_counter_is_checked_transactionally_for_overflow() {
    const INACTIVE: RetainedRoute = RetainedRoute {
        predecessor_index: 0,
        predecessor_guard_false_fallthrough: false,
        predecessor_maximum_capacity_assignment_executed: false,
        active: false,
    };
    const GUARD_FALSE: RetainedRoute = RetainedRoute {
        predecessor_index: 20,
        predecessor_guard_false_fallthrough: true,
        predecessor_maximum_capacity_assignment_executed: false,
        active: true,
    };
    const MAXIMUM: RetainedRoute = RetainedRoute {
        predecessor_index: 20,
        predecessor_guard_false_fallthrough: false,
        predecessor_maximum_capacity_assignment_executed: true,
        active: true,
    };
    type Mutation = (RetainedRoute, fn(&mut State));
    let mutations: &[Mutation] = &[
        (INACTIVE, |state| state.transition_count = usize::MAX),
        (INACTIVE, |state| {
            state.predecessor_route_counts[0] = usize::MAX
        }),
        (INACTIVE, |state| {
            state.inactive_transition_count = usize::MAX
        }),
        (GUARD_FALSE, |state| {
            state.predecessor_guard_false_fallthrough_count = usize::MAX
        }),
        (GUARD_FALSE, |state| {
            state.predecessor_guard_false_fallthrough_route_counts[20] = usize::MAX
        }),
        (MAXIMUM, |state| {
            state.predecessor_maximum_capacity_assignment_count = usize::MAX
        }),
        (MAXIMUM, |state| {
            state.predecessor_maximum_capacity_assignment_route_counts[20] = usize::MAX
        }),
        (GUARD_FALSE, |state| {
            state.dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_break_count = usize::MAX
        }),
        (MAXIMUM, |state| {
            state.source_site_execution_count = usize::MAX
        }),
    ];
    for (route, mutate) in mutations {
        let mut state = State::new(IdealLoadsAirSystemId(409));
        mutate(&mut state);
        let before = state.clone();
        assert!(!test_next_transition_fits(&state, *route));
        assert_eq!(state, before);
    }
}

fn inactive_snapshot() -> Snapshot {
    Snapshot {
        source: SOURCE,
        first_excluded_source: EXCLUDED,
        source_order: ORDER,
        system: IdealLoadsAirSystemId(409),
        parent_call_ordinal: 1,
        controlled_zone: ZoneId(409),
        unit_off_skipped: true,
        non_cooling_skipped: false,
        positive_guard_false_fallthrough_skipped: false,
        heating_availability_guard_false_fallthrough: false,
        humidification_control_guard_false_fallthrough: false,
        dehumidification_control_humidistat_maximum_assignment_executed: false,
        dehumidification_control_none_maximum_assignment_executed: false,
        dehumidification_control_guard_false_fallthrough: false,
        predecessor_capacity_limit_guard_evaluated: false,
        predecessor_capacity_limit_body_entered: false,
        predecessor_active_capacity_limit_guard_false_fallthrough: false,
        predecessor_dehumidification_guard_evaluated: false,
        predecessor_dehumidification_body_entered: false,
        predecessor_dehumidification_guard_false_fallthrough: false,
        predecessor_dehumidification_total_output_assignment_executed: false,
        predecessor_dehumidification_total_output_capacity_guard_evaluated: false,
        predecessor_dehumidification_total_output_capacity_adjustment_body_entered: false,
        predecessor_dehumidification_total_output_capacity_guard_false_fallthrough: false,
        dehumidification_total_output_capacity_guard_false_fallthrough: false,
        dehumidification_total_output_maximum_capacity_assignment_executed: false,
        predecessor_supply_enthalpy_assignment_executed: false,
        predecessor_dehumidification_control_type_read: false,
        predecessor_dehumidification_control_type: None,
        predecessor_dehumidification_control_switch_dispatched: false,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered: false,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break: false,
        predecessor_dehumidification_control_humidistat_case_entered: false,
        predecessor_dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed: false,
        predecessor_dehumidification_control_humidistat_case_exited_via_break: false,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered: false,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough: false,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed: false,
        predecessor_cp408_resulting_supply_humidity_ratio: None,
        predecessor_cp408_resulting_supply_enthalpy_j_per_kg: None,
        predecessor_cp408_resulting_supply_temperature_c: None,
        dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_exited_via_break: false,
        resulting_supply_humidity_ratio: None,
        resulting_supply_enthalpy_j_per_kg: None,
        resulting_supply_temperature_c: None,
    }
}

fn shared_snapshot(maximum: bool) -> Snapshot {
    let mut snapshot = inactive_snapshot();
    snapshot.unit_off_skipped = false;
    snapshot.heating_availability_guard_false_fallthrough = true;
    snapshot.predecessor_capacity_limit_guard_evaluated = true;
    snapshot.predecessor_capacity_limit_body_entered = true;
    snapshot.predecessor_dehumidification_guard_evaluated = true;
    snapshot.predecessor_dehumidification_body_entered = true;
    snapshot.predecessor_dehumidification_total_output_assignment_executed = true;
    snapshot.predecessor_dehumidification_total_output_capacity_guard_evaluated = true;
    snapshot.predecessor_dehumidification_total_output_capacity_adjustment_body_entered = true;
    snapshot.dehumidification_total_output_maximum_capacity_assignment_executed = true;
    snapshot.predecessor_supply_enthalpy_assignment_executed = true;
    snapshot.predecessor_dehumidification_control_type_read = true;
    snapshot.predecessor_dehumidification_control_type = Some(D::None);
    snapshot.predecessor_dehumidification_control_switch_dispatched = true;
    snapshot
        .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered = true;
    snapshot
        .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough = !maximum;
    snapshot
        .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed = maximum;
    snapshot.predecessor_cp408_resulting_supply_humidity_ratio = Some(0.008);
    snapshot.predecessor_cp408_resulting_supply_enthalpy_j_per_kg = Some(40_000.0);
    snapshot.predecessor_cp408_resulting_supply_temperature_c = Some(18.0);
    snapshot
        .dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_exited_via_break = true;
    snapshot.resulting_supply_humidity_ratio = Some(0.008);
    snapshot.resulting_supply_enthalpy_j_per_kg = Some(40_000.0);
    snapshot.resulting_supply_temperature_c = Some(18.0);
    snapshot
}
