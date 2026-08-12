//! CP418 boundary, exhaustive route, forgery, preservation, and overflow tests.

use super::release::cp417_shape_for_test;
use super::transition::predecessor_route;
use super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchEntryRuntimeState as State,
    advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_state as advance,
    advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_state_with_validated_route as advance_with_validated_route,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_snapshot_is_exact,
};
use crate::ideal_loads::calc::cooling_mixed_air_call::tests::{
    Route as MixedAirRoute, active_input as mixed_air_active_input,
    predecessor as mixed_air_predecessor,
};
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard::tests::{
    all_routes, predecessor_for_outcome,
};
use crate::ideal_loads::calc::{
    PurchasedAirCalcCoolingMixedAirCallRuntimeState as MixedAirState,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyEnthalpyAssignmentRuntimeState as Cp417State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioAssignmentRuntimeState as Cp416State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationGuardRuntimeState as Cp413State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationAssignmentRuntimeState as Cp414State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationMixedAirLimitRuntimeState as Cp415State,
    advance_cooling_mixed_air_call_state,
    advance_cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment_state as advance_cp417,
    advance_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment_state as advance_cp416,
    advance_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_state as advance_cp413,
    advance_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_state as advance_cp414,
    advance_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_state as advance_cp415,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingMixedAirCallSnapshot as MixedAirOwner,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchEntrySnapshot as Snapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyEnthalpyAssignmentSnapshot as Predecessor,
    cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment_snapshots_match_bit_exact,
};

#[test]
fn cp418_boundary_and_sole_site_are_exact() {
    assert_eq!(
        super::PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_ENTRY_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2327",
    );
    assert_eq!(
        super::PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_ENTRY_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2330",
    );
    assert_eq!(
        super::PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_ENTRY_SOURCE_ORDER,
        &["enter-post-saturation-capacity-limit-dehumidification-guard-else-branch-after-guard-false-fallthrough"],
    );
}

#[test]
fn exhaustive_54_outcomes_49_inactive_five_entries_and_eight_arrays_are_exact() {
    let routes = all_routes();
    assert_eq!(routes.len(), 36);
    let system = ep_model::IdealLoadsAirSystemId(412);
    let mut cp413_state = Cp413State::new(system);
    let mut cp414_state = Cp414State::new(system);
    let mut cp415_state = Cp415State::new(system);
    let mut cp416_state = Cp416State::new(system);
    let mut cp417_state = Cp417State::new(system);
    let mut state = State::new(system);
    let mut expected_predecessor = [0usize; 36];
    let mut expected_guard_false = [0usize; 36];
    let mut expected_guard_body = [0usize; 36];
    let mut expected_saturation_assignment = [0usize; 36];
    let mut expected_mixed_air_limit = [0usize; 36];
    let mut expected_humidity_assignment = [0usize; 36];
    let mut expected_enthalpy_assignment = [0usize; 36];
    let mut expected_else_entry = [0usize; 36];
    let mut entry_outcomes = Vec::new();
    let mut public_entry_outcomes = Vec::new();
    let mut snapshots = Vec::new();
    let mut ordinal = 0usize;

    for route in routes {
        let outcomes: &[bool] = if route.active {
            &[false, true]
        } else {
            &[false]
        };
        for &body_entered in outcomes {
            let conceptual_index = ordinal;
            ordinal += 1;
            let cp412 = predecessor_for_outcome(route, ordinal, body_entered);
            let cp413 = advance_cp413(&mut cp413_state, cp412).expect("valid CP413 outcome");
            let cp414 =
                advance_cp414(&mut cp414_state, cp413, 91_325.0).expect("valid CP414 outcome");
            let owner = body_entered.then(|| matching_mixed_air_owner(cp414, 17.0));
            let cp415 = advance_cp415(&mut cp415_state, cp414, owner).expect("valid CP415 outcome");
            let cp416 = advance_cp416(&mut cp416_state, cp415).expect("valid CP416 outcome");
            let cp417 = advance_cp417(&mut cp417_state, cp416).expect("valid CP417 outcome");
            let cp418_route = predecessor_route(cp417).expect("valid CP418 route");
            let snapshot = advance(&mut state, cp417).expect("valid CP418 outcome");
            let index = cp418_route.logical_index;

            expected_predecessor[index] += 1;
            if cp418_route.predecessor_guard_false_fallthrough {
                expected_guard_false[index] += 1;
            }
            if cp418_route.predecessor_guard_body_entered {
                expected_guard_body[index] += 1;
            }
            if cp418_route.predecessor_saturation_temperature_assignment_executed {
                expected_saturation_assignment[index] += 1;
            }
            if cp418_route.predecessor_saturation_temperature_mixed_air_limit_executed {
                expected_mixed_air_limit[index] += 1;
            }
            if cp418_route.predecessor_supply_humidity_ratio_assignment_executed {
                expected_humidity_assignment[index] += 1;
            }
            if cp418_route.predecessor_supply_enthalpy_assignment_executed {
                expected_enthalpy_assignment[index] += 1;
            }
            if cp418_route.active {
                expected_else_entry[index] += 1;
                entry_outcomes.push(conceptual_index);
                if matches!(route.predecessor_index, 0..=8 | 20 | 24) {
                    public_entry_outcomes.push(conceptual_index);
                }
            }

            assert_snapshot_bit_preserves_cp417(snapshot, cp417, cp418_route.active);
            snapshots.push(snapshot);
        }
    }

    assert_eq!(ordinal, 54);
    assert_eq!(entry_outcomes, [4, 7, 10, 13, 16]);
    assert_eq!(public_entry_outcomes, [4, 7]);
    assert_eq!(state.transition_count, 54);
    assert_eq!(state.inactive_transition_count, 49);
    assert_eq!(
        state.predecessor_supply_temperature_saturation_assignment_count,
        18
    );
    assert_eq!(
        state.predecessor_supply_temperature_saturation_mixed_air_limit_count,
        18
    );
    assert_eq!(state.predecessor_supply_humidity_ratio_assignment_count, 18);
    assert_eq!(state.predecessor_supply_enthalpy_assignment_count, 18);
    assert_eq!(state.dehumidification_guard_else_branch_entry_count, 5);
    assert_eq!(state.source_site_execution_count, 5);
    assert_eq!(state.cp417_supply_humidity_ratio_state_owner_count, 36);
    assert_eq!(state.unchanged_supply_humidity_ratio_preservation_count, 36);
    assert_eq!(state.cp417_supply_enthalpy_state_owner_count, 41);
    assert_eq!(state.unchanged_supply_enthalpy_preservation_count, 41);
    assert_eq!(state.cp417_supply_temperature_state_owner_count, 51);
    assert_eq!(state.unchanged_supply_temperature_preservation_count, 51);
    assert_eq!(state.predecessor_route_counts, expected_predecessor);
    assert_eq!(
        state.predecessor_guard_false_fallthrough_route_counts,
        expected_guard_false
    );
    assert_eq!(
        state.predecessor_guard_body_entry_route_counts,
        expected_guard_body
    );
    assert_eq!(
        state.predecessor_supply_temperature_saturation_assignment_route_counts,
        expected_saturation_assignment
    );
    assert_eq!(
        state.predecessor_supply_temperature_mixed_air_limit_route_counts,
        expected_mixed_air_limit
    );
    assert_eq!(
        state.predecessor_supply_humidity_ratio_assignment_route_counts,
        expected_humidity_assignment
    );
    assert_eq!(
        state.predecessor_supply_enthalpy_assignment_route_counts,
        expected_enthalpy_assignment
    );
    assert_eq!(
        state.dehumidification_guard_else_branch_entry_route_counts,
        expected_else_entry
    );
    assert_eq!(nonzero_indices(&expected_else_entry), [4, 7, 10, 13, 16]);
    assert_eq!(
        snapshots
            .iter()
            .filter(|snapshot| snapshot.resulting_supply_humidity_ratio.is_some())
            .count(),
        36
    );
    assert_eq!(
        snapshots
            .iter()
            .filter(|snapshot| snapshot.resulting_supply_enthalpy_j_per_kg.is_some())
            .count(),
        41
    );
    assert_eq!(
        snapshots
            .iter()
            .filter(|snapshot| snapshot.resulting_supply_temperature_c.is_some())
            .count(),
        51
    );
    for index in [4, 23] {
        assert!(cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_snapshot_is_exact(
            snapshots[index],
        ));
    }
}

#[test]
fn outer_guard_false_is_distinct_from_later_saturation_guard_false() {
    let outer_false = predecessor_fixture(4, false, false);
    let outer_snapshot =
        advance(&mut State::new(outer_false.system), outer_false).expect("CP418 entry");
    assert!(outer_false.predecessor_dehumidification_guard_false_fallthrough);
    assert!(!outer_false.saturation_supply_humidity_ratio_guard_false_fallthrough);
    assert!(
        outer_snapshot.post_saturation_capacity_limit_dehumidification_guard_else_branch_entered
    );

    let later_false = predecessor_fixture(20, false, true);
    let later_snapshot =
        advance(&mut State::new(later_false.system), later_false).expect("CP418 skip");
    assert!(!later_false.predecessor_dehumidification_guard_false_fallthrough);
    assert!(later_false.saturation_supply_humidity_ratio_guard_false_fallthrough);
    assert!(
        !later_snapshot.post_saturation_capacity_limit_dehumidification_guard_else_branch_entered
    );
}

#[test]
fn validated_route_advance_matches_cold_recursive_advance_bit_exact() {
    for predecessor in [
        predecessor_fixture(4, false, false),
        predecessor_fixture(20, true, true),
    ] {
        let route = predecessor_route(predecessor).expect("validated predecessor route");
        let mut recursive_state = State::new(predecessor.system);
        let mut validated_state = State::new(predecessor.system);
        let recursive = advance(&mut recursive_state, predecessor).expect("recursive CP418");
        let validated = advance_with_validated_route(&mut validated_state, predecessor, route)
            .expect("validated-route CP418");

        assert!(
            crate::ideal_loads::cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_snapshots_match_bit_exact(
                recursive,
                validated,
            )
        );
        assert_eq!(recursive_state, validated_state);
    }
}

#[test]
fn validated_route_advance_rejects_forged_route_transactionally() {
    let predecessor = predecessor_fixture(4, false, false);
    let route = predecessor_route(predecessor).expect("entry route");
    let mut forgeries = [route, route];
    forgeries[0].active = false;
    forgeries[1].logical_index = 5;

    for forged in forgeries {
        let mut state = State::new(predecessor.system);
        let before = state.clone();
        assert!(advance_with_validated_route(&mut state, predecessor, forged).is_none());
        assert_eq!(state, before);
    }
}

#[test]
fn public_release_hot_path_has_no_recursive_exact_route_derivation() {
    let source = include_str!("release.rs");
    let start = source
        .find(
            "pub fn advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry",
        )
        .expect("CP418 public release");
    let end = source[start..]
        .find(
            "pub(in crate::ideal_loads) fn private_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_characterization",
        )
        .map(|offset| start + offset)
        .expect("end of CP418 public release");
    let hot_path = &source[start..end];

    assert!(hot_path.contains("committed_latest_route"));
    assert!(hot_path.contains("advance_with_validated_route"));
    assert!(!hot_path.contains("predecessor_route("));
    assert!(!hot_path.contains("snapshot_is_exact"));
    assert!(!hot_path.contains("advance(&mut"));
}

#[test]
fn committed_route_accessor_is_nonrecursive_and_checks_committed_shape() {
    let source = include_str!("release.rs");
    let start = source
        .find("fn cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_committed_latest_route")
        .expect("CP418 committed route accessor");
    let end = source[start..]
        .find("fn pending_state_is_consistent")
        .map(|offset| start + offset)
        .expect("end of accessor");
    let accessor = &source[start..end];
    assert!(accessor.contains("snapshot_matches_validated_predecessor"));
    assert!(accessor.contains("committed_route_counts_match"));
    assert!(!accessor.contains("snapshot_is_exact"));
    assert!(!accessor.contains("predecessor_route("));
    assert!(!accessor.contains("completed_direct"));
}

#[test]
fn marker_and_predecessor_forgery_are_rejected() {
    let predecessor = predecessor_fixture(4, false, false);
    let snapshot = advance(&mut State::new(predecessor.system), predecessor).expect("CP418 entry");
    assert!(cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_snapshot_is_exact(snapshot));

    let mut marker_forged = snapshot;
    marker_forged.post_saturation_capacity_limit_dehumidification_guard_else_branch_entered = false;
    assert!(!cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_snapshot_is_exact(marker_forged));

    let mut lineage_forged = snapshot;
    lineage_forged.predecessor_dehumidification_guard_false_fallthrough = false;
    assert!(!cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_snapshot_is_exact(lineage_forged));

    let mut terminal_forged = snapshot;
    terminal_forged.resulting_supply_temperature_c = terminal_forged
        .resulting_supply_temperature_c
        .map(|value| f64::from_bits(value.to_bits() ^ 1));
    assert!(!cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_snapshot_is_exact(terminal_forged));
}

#[test]
fn entry_counter_overflow_is_transactional() {
    let predecessor = predecessor_fixture(4, false, false);
    let route = predecessor_route(predecessor).expect("entry route");
    assert!(route.active);
    let setters: &[fn(&mut State, usize)] = &[
        |state, _| state.transition_count = usize::MAX,
        |state, index| state.predecessor_route_counts[index] = usize::MAX,
        |state, _| state.dehumidification_guard_else_branch_entry_count = usize::MAX,
        |state, index| {
            state.dehumidification_guard_else_branch_entry_route_counts[index] = usize::MAX
        },
        |state, _| state.source_site_execution_count = usize::MAX,
        |state, _| state.cp417_supply_temperature_state_owner_count = usize::MAX,
        |state, _| state.unchanged_supply_temperature_preservation_count = usize::MAX,
    ];
    assert_overflows_transactionally(predecessor, route.logical_index, setters);
}

#[test]
fn deep_inactive_counter_overflow_is_transactional() {
    let predecessor = predecessor_fixture(20, true, true);
    let route = predecessor_route(predecessor).expect("deep inactive route");
    assert!(!route.active && route.predecessor_supply_enthalpy_assignment_executed);
    #[rustfmt::skip]
    let setters: &[fn(&mut State, usize)] = &[
        |state, _| state.transition_count = usize::MAX,
        |state, index| state.predecessor_route_counts[index] = usize::MAX,
        |state, index| state.predecessor_guard_body_entry_route_counts[index] = usize::MAX,
        |state, _| state.predecessor_supply_temperature_saturation_assignment_count = usize::MAX,
        |state, index| state.predecessor_supply_temperature_saturation_assignment_route_counts[index] = usize::MAX,
        |state, _| state.predecessor_supply_temperature_saturation_mixed_air_limit_count = usize::MAX,
        |state, index| state.predecessor_supply_temperature_mixed_air_limit_route_counts[index] = usize::MAX,
        |state, _| state.predecessor_supply_humidity_ratio_assignment_count = usize::MAX,
        |state, index| state.predecessor_supply_humidity_ratio_assignment_route_counts[index] = usize::MAX,
        |state, _| state.predecessor_supply_enthalpy_assignment_count = usize::MAX,
        |state, index| state.predecessor_supply_enthalpy_assignment_route_counts[index] = usize::MAX,
        |state, _| state.inactive_transition_count = usize::MAX,
        |state, _| state.cp417_supply_humidity_ratio_state_owner_count = usize::MAX,
        |state, _| state.unchanged_supply_humidity_ratio_preservation_count = usize::MAX,
        |state, _| state.cp417_supply_enthalpy_state_owner_count = usize::MAX,
        |state, _| state.unchanged_supply_enthalpy_preservation_count = usize::MAX,
        |state, _| state.cp417_supply_temperature_state_owner_count = usize::MAX,
        |state, _| state.unchanged_supply_temperature_preservation_count = usize::MAX,
    ];
    assert_overflows_transactionally(predecessor, route.logical_index, setters);
}

#[test]
fn later_guard_false_counter_overflow_is_transactional() {
    let predecessor = predecessor_fixture(20, false, true);
    let route = predecessor_route(predecessor).expect("later guard-false route");
    assert!(!route.active && route.predecessor_guard_false_fallthrough);
    let setters: &[fn(&mut State, usize)] = &[
        |state, index| state.predecessor_guard_false_fallthrough_route_counts[index] = usize::MAX,
        |state, _| state.inactive_transition_count = usize::MAX,
        |state, _| state.cp417_supply_humidity_ratio_state_owner_count = usize::MAX,
        |state, _| state.unchanged_supply_humidity_ratio_preservation_count = usize::MAX,
        |state, _| state.cp417_supply_enthalpy_state_owner_count = usize::MAX,
        |state, _| state.unchanged_supply_enthalpy_preservation_count = usize::MAX,
        |state, _| state.cp417_supply_temperature_state_owner_count = usize::MAX,
        |state, _| state.unchanged_supply_temperature_preservation_count = usize::MAX,
    ];
    assert_overflows_transactionally(predecessor, route.logical_index, setters);
}

fn assert_snapshot_bit_preserves_cp417(
    snapshot: Snapshot,
    predecessor: Predecessor,
    entered: bool,
) {
    assert_eq!(
        snapshot.post_saturation_capacity_limit_dehumidification_guard_else_branch_entered,
        entered,
    );
    assert!(cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment_snapshots_match_bit_exact(
        cp417_shape_for_test(snapshot),
        predecessor,
    ));
    assert!(option_bits_equal(
        snapshot.resulting_supply_humidity_ratio,
        predecessor.resulting_supply_humidity_ratio
    ));
    assert!(option_bits_equal(
        snapshot.resulting_supply_enthalpy_j_per_kg,
        predecessor.resulting_supply_enthalpy_j_per_kg
    ));
    assert!(option_bits_equal(
        snapshot.resulting_supply_temperature_c,
        predecessor.resulting_supply_temperature_c
    ));
}

fn predecessor_fixture(
    predecessor_index: usize,
    body_entered: bool,
    prefer_guard_false_route: bool,
) -> Predecessor {
    let route = all_routes()
        .into_iter()
        .find(|route| {
            route.predecessor_index == predecessor_index
                && (!matches!(predecessor_index, 20 | 21 | 24 | 25 | 27 | 29)
                    || route.predecessor_guard_false_fallthrough == prefer_guard_false_route)
        })
        .expect("requested route");
    let cp412 = predecessor_for_outcome(route, 1, body_entered);
    let cp413 = advance_cp413(&mut Cp413State::new(cp412.system), cp412).expect("CP413");
    let cp414 = advance_cp414(&mut Cp414State::new(cp413.system), cp413, 91_325.0).expect("CP414");
    let owner = body_entered.then(|| matching_mixed_air_owner(cp414, 17.0));
    let cp415 = advance_cp415(&mut Cp415State::new(cp414.system), cp414, owner).expect("CP415");
    let cp416 = advance_cp416(&mut Cp416State::new(cp415.system), cp415).expect("CP416");
    advance_cp417(&mut Cp417State::new(cp416.system), cp416).expect("CP417")
}

fn matching_mixed_air_owner(
    predecessor: crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationAssignmentSnapshot,
    mixed_air_temperature_c: f64,
) -> MixedAirOwner {
    let mixed_predecessor = mixed_air_predecessor(MixedAirRoute::CoolingFallthrough);
    let mut owner = advance_cooling_mixed_air_call_state(
        &mut MixedAirState::new(mixed_predecessor.system),
        mixed_predecessor,
        Some(mixed_air_active_input(0.25)),
    );
    owner.system = predecessor.system;
    owner.parent_call_ordinal = predecessor.parent_call_ordinal;
    owner.controlled_zone = predecessor.controlled_zone;
    owner.mixed_air_temperature_c = Some(mixed_air_temperature_c);
    owner
}

fn assert_overflows_transactionally(
    predecessor: Predecessor,
    logical_index: usize,
    setters: &[fn(&mut State, usize)],
) {
    for set_overflow in setters {
        let mut state = State::new(predecessor.system);
        set_overflow(&mut state, logical_index);
        let before = state.clone();
        assert!(advance(&mut state, predecessor).is_none());
        assert_eq!(state, before);
    }
}

fn nonzero_indices(values: &[usize; 36]) -> Vec<usize> {
    values
        .iter()
        .enumerate()
        .filter_map(|(index, value)| (*value != 0).then_some(index))
        .collect()
}

fn option_bits_equal(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
