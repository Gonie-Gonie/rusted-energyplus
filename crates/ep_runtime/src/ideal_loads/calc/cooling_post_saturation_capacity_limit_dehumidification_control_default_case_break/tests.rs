//! CP410 boundary, route, carrier, accounting, corruption, and overflow tests.

use ep_model::{DehumidificationControlType as D, IdealLoadsAirSystemId, ZoneId};

use super::release::{
    cooling_post_saturation_capacity_limit_dehumidification_control_default_case_break_snapshot_is_exact_direct_release,
    test_counts_are_exact,
};
use super::transition::routes::{
    RetainedRoute, logical_route_index, predecessor_index_is_public, predecessor_index_is_split,
};
use super::transition::{test_increment_counts, test_next_transition_fits};
use super::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_DEFAULT_CASE_BREAK_FIRST_EXCLUDED_SOURCE as EXCLUDED,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_DEFAULT_CASE_BREAK_SOURCE as SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_DEFAULT_CASE_BREAK_SOURCE_ORDER as ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlDefaultCaseBreakRuntimeState as State,
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_default_case_break_state as advance,
    cooling_post_saturation_capacity_limit_dehumidification_control_default_case_break_snapshot_route as snapshot_route,
    cooling_post_saturation_capacity_limit_dehumidification_control_default_case_break_snapshots_match_bit_exact as snapshots_match_bit_exact,
};
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_FIRST_EXCLUDED_SOURCE as PREDECESSOR_EXCLUDED,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_SOURCE as PREDECESSOR_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_SOURCE_ORDER as PREDECESSOR_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCaseBreakSnapshot as Predecessor,
};

#[test]
fn cp410_boundary_and_single_default_break_site_are_exact() {
    assert_eq!(SOURCE, "EnergyPlus 26.1 PurchasedAirManager.cc:2308");
    assert_eq!(EXCLUDED, "EnergyPlus 26.1 PurchasedAirManager.cc:2313");
    assert_eq!(
        ORDER,
        &[
            "exit-purchased-air-post-saturation-capacity-limit-dehumidification-control-default-case-via-break"
        ],
    );
}

fn all_routes() -> Vec<RetainedRoute> {
    let mut routes = Vec::new();
    for predecessor_index in 0..30 {
        if predecessor_index_is_split(predecessor_index) {
            routes.push(RetainedRoute {
                predecessor_index,
                predecessor_guard_false_fallthrough: true,
                predecessor_maximum_capacity_assignment_executed: false,
                predecessor_shared_case_break_executed: true,
            });
            routes.push(RetainedRoute {
                predecessor_index,
                predecessor_guard_false_fallthrough: false,
                predecessor_maximum_capacity_assignment_executed: true,
                predecessor_shared_case_break_executed: true,
            });
        } else {
            routes.push(RetainedRoute {
                predecessor_index,
                predecessor_guard_false_fallthrough: false,
                predecessor_maximum_capacity_assignment_executed: false,
                predecessor_shared_case_break_executed: false,
            });
        }
    }
    routes
}

#[test]
fn cp410_preserves_thirty_six_routes_without_an_untyped_default_route() {
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
            .filter(|route| route.predecessor_shared_case_break_executed)
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
}

fn exhaustive_state() -> State {
    let mut state = State::new(IdealLoadsAirSystemId(410));
    for route in all_routes() {
        assert!(test_next_transition_fits(&state, route));
        state.transition_count += 1;
        test_increment_counts(&mut state, route);
    }
    state
}

#[test]
fn exhaustive_accounting_is_36_36_0_0_with_six_plus_six_predecessor_branches() {
    let state = exhaustive_state();
    assert!(test_counts_are_exact(&state));
    assert_eq!(state.transition_count, 36);
    assert_eq!(state.inactive_transition_count, 36);
    assert_eq!(state.predecessor_guard_false_fallthrough_count, 6);
    assert_eq!(state.predecessor_maximum_capacity_assignment_count, 6);
    assert_eq!(state.dehumidification_control_default_case_break_count, 0);
    assert_eq!(state.source_site_execution_count, 0);
    for index in 0..30 {
        if predecessor_index_is_split(index) {
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
fn both_reconverged_public_predecessors_skip_default_and_preserve_carriers() {
    let humidity = f64::from_bits(0x7ff8_0000_0000_0410);
    let enthalpy = -0.0_f64;
    let temperature = f64::INFINITY;
    for maximum in [false, true] {
        let mut predecessor = shared_predecessor(maximum);
        predecessor.predecessor_cp408_resulting_supply_humidity_ratio = Some(humidity);
        predecessor.resulting_supply_humidity_ratio = Some(humidity);
        predecessor.predecessor_cp408_resulting_supply_enthalpy_j_per_kg = Some(enthalpy);
        predecessor.resulting_supply_enthalpy_j_per_kg = Some(enthalpy);
        predecessor.predecessor_cp408_resulting_supply_temperature_c = Some(temperature);
        predecessor.resulting_supply_temperature_c = Some(temperature);
        let mut state = State::new(predecessor.system);
        let Some(snapshot) = advance(&mut state, predecessor) else {
            assert!(advance(&mut State::new(predecessor.system), predecessor).is_some());
            continue;
        };
        assert!(
            snapshot.predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_exited_via_break
        );
        assert!(!snapshot.dehumidification_control_default_case_exited_via_break);
        assert!(snapshot_route(snapshot).is_some());
        assert!(cooling_post_saturation_capacity_limit_dehumidification_control_default_case_break_snapshot_is_exact_direct_release(snapshot));
        assert_eq!(
            snapshot.resulting_supply_humidity_ratio.map(f64::to_bits),
            Some(humidity.to_bits())
        );
        assert_eq!(state.inactive_transition_count, 1);
        assert_eq!(state.dehumidification_control_default_case_break_count, 0);
        assert_eq!(state.source_site_execution_count, 0);
    }
}

#[test]
fn default_execution_and_carrier_corruption_fail_closed() {
    let predecessor = shared_predecessor(false);
    let mut state = State::new(predecessor.system);
    let Some(snapshot) = advance(&mut state, predecessor) else {
        assert!(advance(&mut State::new(predecessor.system), predecessor).is_some());
        return;
    };

    let mut forged = snapshot;
    forged.dehumidification_control_default_case_exited_via_break = true;
    assert!(snapshot_route(forged).is_none());

    let mut forged = snapshot;
    forged
        .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_exited_via_break = false;
    assert!(snapshot_route(forged).is_none());

    let mut drift = snapshot;
    drift.resulting_supply_temperature_c = Some(f64::from_bits(
        snapshot
            .resulting_supply_temperature_c
            .map_or(0, f64::to_bits)
            ^ 1,
    ));
    assert!(snapshot_route(drift).is_none());
    assert!(!snapshots_match_bit_exact(snapshot, drift));
}

#[test]
fn inactive_predecessor_is_preserved_as_an_inactive_default_skip() {
    let predecessor = inactive_predecessor();
    let mut state = State::new(predecessor.system);
    let Some(snapshot) = advance(&mut state, predecessor) else {
        assert!(advance(&mut State::new(predecessor.system), predecessor).is_some());
        return;
    };
    let Some(route) = snapshot_route(snapshot) else {
        assert!(snapshot_route(snapshot).is_some());
        return;
    };
    assert_eq!(logical_route_index(route), 0);
    assert!(!route.predecessor_shared_case_break_executed);
    assert!(!snapshot.dehumidification_control_default_case_exited_via_break);
}

#[test]
fn every_incremented_counter_and_zero_source_invariant_are_transactional() {
    const INACTIVE: RetainedRoute = RetainedRoute {
        predecessor_index: 0,
        predecessor_guard_false_fallthrough: false,
        predecessor_maximum_capacity_assignment_executed: false,
        predecessor_shared_case_break_executed: false,
    };
    const GUARD_FALSE: RetainedRoute = RetainedRoute {
        predecessor_index: 20,
        predecessor_guard_false_fallthrough: true,
        predecessor_maximum_capacity_assignment_executed: false,
        predecessor_shared_case_break_executed: true,
    };
    const MAXIMUM: RetainedRoute = RetainedRoute {
        predecessor_index: 20,
        predecessor_guard_false_fallthrough: false,
        predecessor_maximum_capacity_assignment_executed: true,
        predecessor_shared_case_break_executed: true,
    };
    type Mutation = (RetainedRoute, fn(&mut State));
    let mutations: &[Mutation] = &[
        (INACTIVE, |state| state.transition_count = usize::MAX),
        (INACTIVE, |state| {
            state.inactive_transition_count = usize::MAX
        }),
        (INACTIVE, |state| {
            state.predecessor_route_counts[0] = usize::MAX
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
        (INACTIVE, |state| {
            state.dehumidification_control_default_case_break_count = 1
        }),
        (INACTIVE, |state| state.source_site_execution_count = 1),
    ];
    for (route, mutate) in mutations {
        let mut state = State::new(IdealLoadsAirSystemId(410));
        mutate(&mut state);
        let before = state.clone();
        assert!(!test_next_transition_fits(&state, *route));
        assert_eq!(state, before);
    }
}

fn inactive_predecessor() -> Predecessor {
    Predecessor {
        source: PREDECESSOR_SOURCE,
        first_excluded_source: PREDECESSOR_EXCLUDED,
        source_order: PREDECESSOR_ORDER,
        system: IdealLoadsAirSystemId(410),
        parent_call_ordinal: 1,
        controlled_zone: ZoneId(410),
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

fn shared_predecessor(maximum: bool) -> Predecessor {
    let mut snapshot = inactive_predecessor();
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

mod exhaustive;
