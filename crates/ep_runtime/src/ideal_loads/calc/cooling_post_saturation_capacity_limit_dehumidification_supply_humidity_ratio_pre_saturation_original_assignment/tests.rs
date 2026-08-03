//! CP411 boundary, route, copy, corruption, IEEE, and overflow tests.

use ep_model::{DehumidificationControlType as D, IdealLoadsAirSystemId, ZoneId};

use super::release::{
    cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment_snapshot_is_exact_direct_release,
    test_counts_are_exact,
};
use super::transition::routes::{
    logical_route_index, predecessor_index_is_public, predecessor_index_is_split, route_is_active,
    RetainedRoute,
};
use super::transition::test_next_transition_fits;
use super::{
    advance_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment_state as advance,
    cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment_snapshot_route as snapshot_route,
    cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment_snapshots_match_bit_exact as snapshots_match_bit_exact,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioPreSaturationOriginalAssignmentRuntimeState as State,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_FIRST_EXCLUDED_SOURCE as EXCLUDED,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE as SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE_ORDER as ORDER,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlDefaultCaseBreakSnapshot as Predecessor,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_DEFAULT_CASE_BREAK_FIRST_EXCLUDED_SOURCE as PREDECESSOR_EXCLUDED,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_DEFAULT_CASE_BREAK_SOURCE as PREDECESSOR_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_DEFAULT_CASE_BREAK_SOURCE_ORDER as PREDECESSOR_ORDER,
};

#[test]
fn cp411_boundary_and_two_copy_sites_are_exact() {
    assert_eq!(SOURCE, "EnergyPlus 26.1 PurchasedAirManager.cc:2313");
    assert_eq!(EXCLUDED, "EnergyPlus 26.1 PurchasedAirManager.cc:2314");
    assert_eq!(
        ORDER,
        &[
            "read-purchased-air-supply-humidity-ratio-before-saturation-limit",
            "assign-local-original-supply-humidity-ratio-before-saturation-limit",
        ]
    );
}

pub(super) fn all_routes() -> Vec<RetainedRoute> {
    let mut routes = Vec::new();
    for predecessor_index in 0..30 {
        let active = matches!(predecessor_index, 18..=29);
        if predecessor_index_is_split(predecessor_index) {
            routes.push(RetainedRoute {
                predecessor_index,
                predecessor_guard_false_fallthrough: true,
                predecessor_maximum_capacity_assignment_executed: false,
                active,
            });
            routes.push(RetainedRoute {
                predecessor_index,
                predecessor_guard_false_fallthrough: false,
                predecessor_maximum_capacity_assignment_executed: true,
                active,
            });
        } else {
            routes.push(RetainedRoute {
                predecessor_index,
                predecessor_guard_false_fallthrough: false,
                predecessor_maximum_capacity_assignment_executed: false,
                active,
            });
        }
    }
    routes
}

#[test]
fn cp411_route_partition_is_18_active_with_four_public_and_fourteen_private() {
    let routes = all_routes();
    assert_eq!(routes.len(), 36);
    assert_eq!(
        routes
            .iter()
            .copied()
            .map(logical_route_index)
            .collect::<Vec<_>>(),
        (0..36).collect::<Vec<_>>()
    );
    assert_eq!(
        routes
            .iter()
            .copied()
            .filter(|route| route_is_active(*route))
            .map(logical_route_index)
            .collect::<Vec<_>>(),
        (18..36).collect::<Vec<_>>()
    );
    assert_eq!(
        routes
            .iter()
            .filter(|route| predecessor_index_is_public(route.predecessor_index))
            .count(),
        13
    );
    assert_eq!(
        routes
            .iter()
            .filter(|route| predecessor_index_is_public(route.predecessor_index)
                && route_is_active(**route))
            .count(),
        4
    );
    assert_eq!(
        routes
            .iter()
            .filter(
                |route| !predecessor_index_is_public(route.predecessor_index)
                    && route_is_active(**route)
            )
            .count(),
        14
    );
}

#[test]
fn active_copy_preserves_nan_payload_and_all_terminal_carriers() {
    let route = all_routes()
        .into_iter()
        .find(|route| route.predecessor_index == 20 && route.predecessor_guard_false_fallthrough);
    assert!(route.is_some(), "active guard-false route must exist");
    let Some(route) = route else {
        return;
    };
    let mut predecessor = predecessor_for_route(route, 1);
    let humidity = f64::from_bits(0x7ff8_0000_0000_0411);
    let enthalpy = -0.0_f64;
    let temperature = f64::INFINITY;
    predecessor.predecessor_cp409_resulting_supply_humidity_ratio = Some(humidity);
    predecessor.resulting_supply_humidity_ratio = Some(humidity);
    predecessor.predecessor_cp409_resulting_supply_enthalpy_j_per_kg = Some(enthalpy);
    predecessor.resulting_supply_enthalpy_j_per_kg = Some(enthalpy);
    predecessor.predecessor_cp409_resulting_supply_temperature_c = Some(temperature);
    predecessor.resulting_supply_temperature_c = Some(temperature);
    let mut state = State::new(predecessor.system);
    let snapshot = advance(&mut state, predecessor);
    assert!(snapshot.is_some(), "valid active CP410 snapshot");
    let Some(snapshot) = snapshot else {
        return;
    };
    for value in [
        snapshot.predecessor_cp410_resulting_supply_humidity_ratio,
        snapshot.purchased_air_supply_humidity_ratio_before_saturation_check,
        snapshot.assigned_supply_humidity_ratio_original,
        snapshot.resulting_supply_humidity_ratio_original,
        snapshot.resulting_supply_humidity_ratio,
    ] {
        assert_eq!(value.map(f64::to_bits), Some(humidity.to_bits()));
    }
    assert_eq!(
        snapshot
            .resulting_supply_enthalpy_j_per_kg
            .map(f64::to_bits),
        Some(enthalpy.to_bits())
    );
    assert_eq!(
        snapshot.resulting_supply_temperature_c.map(f64::to_bits),
        Some(temperature.to_bits())
    );
    assert!(cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment_snapshot_is_exact_direct_release(snapshot));
}

#[test]
fn option_presence_cannot_forge_reachability() {
    let inactive = all_routes()[0];
    let mut forged = predecessor_for_route(inactive, 1);
    forged.predecessor_cp409_resulting_supply_humidity_ratio = Some(0.01);
    forged.resulting_supply_humidity_ratio = Some(0.01);
    let before = State::new(forged.system);
    let mut state = before.clone();
    assert!(advance(&mut state, forged).is_none());
    assert_eq!(state, before);

    let active = all_routes()
        .into_iter()
        .find(|route| route.predecessor_index == 18);
    assert!(active.is_some(), "active route must exist");
    let Some(active) = active else {
        return;
    };
    let mut missing = predecessor_for_route(active, 1);
    missing.predecessor_cp409_resulting_supply_humidity_ratio = None;
    missing.resulting_supply_humidity_ratio = None;
    assert!(advance(&mut State::new(missing.system), missing).is_none());
}

#[test]
fn local_or_terminal_bit_drift_fails_closed() {
    let route = all_routes()
        .into_iter()
        .find(|route| route.predecessor_index == 18);
    assert!(route.is_some(), "active route must exist");
    let Some(route) = route else {
        return;
    };
    let predecessor = predecessor_for_route(route, 1);
    let snapshot = advance(&mut State::new(predecessor.system), predecessor);
    assert!(snapshot.is_some(), "valid active CP410 snapshot");
    let Some(snapshot) = snapshot else {
        return;
    };
    let mut drift = snapshot;
    drift.assigned_supply_humidity_ratio_original = drift
        .assigned_supply_humidity_ratio_original
        .map(|value| f64::from_bits(value.to_bits() ^ 1));
    assert!(snapshot_route(drift).is_none());
    assert!(!snapshots_match_bit_exact(snapshot, drift));
    let mut drift = snapshot;
    drift.resulting_supply_temperature_c = drift
        .resulting_supply_temperature_c
        .map(|value| f64::from_bits(value.to_bits() ^ 1));
    assert!(snapshot_route(drift).is_none());
}

#[test]
fn representative_counter_overflow_is_transactional() {
    let inactive = all_routes()[0];
    let active = all_routes()
        .into_iter()
        .find(|route| route.predecessor_index == 20 && route.predecessor_guard_false_fallthrough);
    assert!(active.is_some(), "active guard-false route must exist");
    let Some(active) = active else {
        return;
    };
    let maximum = all_routes()
        .into_iter()
        .find(|route| {
            route.predecessor_index == 20 && route.predecessor_maximum_capacity_assignment_executed
        });
    assert!(maximum.is_some(), "active maximum route must exist");
    let Some(maximum) = maximum else {
        return;
    };
    type Mutation = (RetainedRoute, fn(&mut State));
    let mutations: &[Mutation] = &[
        (inactive, |state| state.transition_count = usize::MAX),
        (inactive, |state| {
            state.inactive_transition_count = usize::MAX
        }),
        (inactive, |state| {
            state.predecessor_route_counts[0] = usize::MAX
        }),
        (active, |state| {
            state.predecessor_guard_false_fallthrough_count = usize::MAX
        }),
        (active, |state| {
            state.predecessor_guard_false_fallthrough_route_counts[20] = usize::MAX
        }),
        (maximum, |state| {
            state.predecessor_maximum_capacity_assignment_count = usize::MAX
        }),
        (maximum, |state| {
            state.predecessor_maximum_capacity_assignment_route_counts[20] = usize::MAX
        }),
        (active, |state| {
            state.supply_humidity_ratio_pre_saturation_original_assignment_count = usize::MAX
        }),
        (active, |state| {
            state.supply_humidity_ratio_pre_saturation_original_assignment_route_counts[20] =
                usize::MAX
        }),
        (active, |state| {
            state.source_site_execution_count = usize::MAX
        }),
        (active, |state| {
            state.cp410_supply_humidity_ratio_state_owner_count = usize::MAX
        }),
        (active, |state| {
            state.unchanged_supply_humidity_ratio_preservation_count = usize::MAX
        }),
        (active, |state| {
            state.cp410_supply_enthalpy_state_owner_count = usize::MAX
        }),
        (active, |state| {
            state.unchanged_supply_enthalpy_preservation_count = usize::MAX
        }),
        (active, |state| {
            state.cp410_supply_temperature_state_owner_count = usize::MAX
        }),
        (active, |state| {
            state.unchanged_supply_temperature_preservation_count = usize::MAX
        }),
        (active, |state| {
            state.cp410_retained_supply_humidity_ratio_owned_read_count = usize::MAX
        }),
        (active, |state| {
            state.purchased_air_supply_humidity_ratio_before_saturation_limit_read_count =
                usize::MAX
        }),
        (active, |state| {
            state.local_supply_humidity_ratio_original_assignment_write_count = usize::MAX
        }),
    ];
    for (route, mutate) in mutations {
        let mut state = State::new(IdealLoadsAirSystemId(411));
        mutate(&mut state);
        let before = state.clone();
        assert!(!test_next_transition_fits(&state, *route));
        assert!(advance(&mut state, predecessor_for_route(*route, 1)).is_none());
        assert_eq!(state, before);
    }
}

pub(super) fn base_predecessor() -> Predecessor {
    Predecessor {
        source: PREDECESSOR_SOURCE,
        first_excluded_source: PREDECESSOR_EXCLUDED,
        source_order: PREDECESSOR_ORDER,
        system: IdealLoadsAirSystemId(411),
        parent_call_ordinal: 1,
        controlled_zone: ZoneId(411),
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
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_exited_via_break: false,
        predecessor_cp409_resulting_supply_humidity_ratio: None,
        predecessor_cp409_resulting_supply_enthalpy_j_per_kg: None,
        predecessor_cp409_resulting_supply_temperature_c: None,
        dehumidification_control_default_case_exited_via_break: false,
        resulting_supply_humidity_ratio: None,
        resulting_supply_enthalpy_j_per_kg: None,
        resulting_supply_temperature_c: None,
    }
}

pub(super) fn predecessor_for_route(route: RetainedRoute, ordinal: usize) -> Predecessor {
    let index = route.predecessor_index;
    let mut snapshot = base_predecessor();
    snapshot.parent_call_ordinal = ordinal;
    snapshot.unit_off_skipped = index == 0;
    snapshot.non_cooling_skipped = index == 1;
    snapshot.positive_guard_false_fallthrough_skipped = index == 2;
    if index >= 3 {
        set_lineage(&mut snapshot, lineage(index));
        set_stage(&mut snapshot, if index < 18 { (index - 3) % 3 } else { 3 });
    }
    if index >= 18 {
        set_switch_case(&mut snapshot, index, route);
    }
    set_carriers(&mut snapshot, index, ordinal);
    snapshot
}

fn lineage(index: usize) -> usize {
    match index {
        3..=17 => (index - 3) / 3,
        18..=21 => 0,
        22..=25 => 1,
        26 => 2,
        27 => 3,
        28..=29 => 4,
        _ => 0,
    }
}

fn set_lineage(snapshot: &mut Predecessor, lineage: usize) {
    snapshot.heating_availability_guard_false_fallthrough = lineage == 0;
    snapshot.humidification_control_guard_false_fallthrough = lineage == 1;
    snapshot.dehumidification_control_humidistat_maximum_assignment_executed = lineage == 2;
    snapshot.dehumidification_control_none_maximum_assignment_executed = lineage == 3;
    snapshot.dehumidification_control_guard_false_fallthrough = lineage == 4;
}

fn set_stage(snapshot: &mut Predecessor, stage: usize) {
    snapshot.predecessor_capacity_limit_guard_evaluated = true;
    if stage == 0 {
        snapshot.predecessor_active_capacity_limit_guard_false_fallthrough = true;
        return;
    }
    snapshot.predecessor_capacity_limit_body_entered = true;
    snapshot.predecessor_dehumidification_guard_evaluated = true;
    if stage == 1 {
        snapshot.predecessor_dehumidification_guard_false_fallthrough = true;
        return;
    }
    snapshot.predecessor_dehumidification_body_entered = true;
    snapshot.predecessor_dehumidification_total_output_assignment_executed = true;
    snapshot.predecessor_dehumidification_total_output_capacity_guard_evaluated = true;
    if stage == 2 {
        snapshot.predecessor_dehumidification_total_output_capacity_guard_false_fallthrough = true;
        snapshot.dehumidification_total_output_capacity_guard_false_fallthrough = true;
        return;
    }
    snapshot.predecessor_dehumidification_total_output_capacity_adjustment_body_entered = true;
    snapshot.dehumidification_total_output_maximum_capacity_assignment_executed = true;
}

fn set_switch_case(snapshot: &mut Predecessor, index: usize, route: RetainedRoute) {
    let selector = match index {
        18 | 22 | 28 => D::ConstantSensibleHeatRatio,
        19 | 23 | 26 => D::Humidistat,
        20 | 24 | 27 => D::None,
        _ => D::ConstantSupplyHumidityRatio,
    };
    let constant_shr = matches!(index, 18 | 22 | 28);
    let humidistat = matches!(index, 19 | 23 | 26);
    let shared = matches!(index, 20 | 21 | 24 | 25 | 27 | 29);
    snapshot.predecessor_supply_enthalpy_assignment_executed = true;
    snapshot.predecessor_dehumidification_control_type_read = true;
    snapshot.predecessor_dehumidification_control_type = Some(selector);
    snapshot.predecessor_dehumidification_control_switch_dispatched = true;
    snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered =
        constant_shr;
    snapshot
        .predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break =
        constant_shr;
    snapshot.predecessor_dehumidification_control_humidistat_case_entered = humidistat;
    snapshot.predecessor_dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed = humidistat;
    snapshot.predecessor_dehumidification_control_humidistat_case_exited_via_break = humidistat;
    snapshot.predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered = shared;
    snapshot.predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough = route.predecessor_guard_false_fallthrough;
    snapshot.predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed = route.predecessor_maximum_capacity_assignment_executed;
    snapshot.predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_exited_via_break = shared;
}

fn set_carriers(snapshot: &mut Predecessor, index: usize, ordinal: usize) {
    let ordinal = ordinal as u64;
    let humidity =
        matches!(index, 18..=29).then(|| f64::from_bits(0x3f80_0000_0000_0000 + ordinal));
    let enthalpy = matches!(index, 5 | 8 | 11 | 14 | 17..=29)
        .then(|| f64::from_bits(0xc0e0_0000_0000_0000 + ordinal));
    let temperature = (index >= 3).then(|| f64::from_bits(0x7ff8_0000_0000_1000 + ordinal));
    snapshot.predecessor_cp409_resulting_supply_humidity_ratio = humidity;
    snapshot.predecessor_cp409_resulting_supply_enthalpy_j_per_kg = enthalpy;
    snapshot.predecessor_cp409_resulting_supply_temperature_c = temperature;
    snapshot.resulting_supply_humidity_ratio = humidity;
    snapshot.resulting_supply_enthalpy_j_per_kg = enthalpy;
    snapshot.resulting_supply_temperature_c = temperature;
}

mod exhaustive;
