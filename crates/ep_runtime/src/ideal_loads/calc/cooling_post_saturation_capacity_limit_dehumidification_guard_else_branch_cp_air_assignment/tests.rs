//! CP419 boundary, exhaustive route, forgery, preservation, and overflow tests.

mod committed;
mod edge_cases;
mod overflow;

use super::release::cp418_shape_for_test;
use super::transition::predecessor_route;
use super::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchCpAirAssignmentActiveInput as ActiveInput;
use super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchCpAirAssignmentRuntimeState as State,
    advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_state as advance,
    advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_state_with_validated_route as advance_with_validated_route,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_snapshot_has_exact_cp418_prefix_and_local_assignment,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_snapshot_is_exact,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_snapshot_is_exact_direct_release,
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
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchEntryRuntimeState as Cp418State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioAssignmentRuntimeState as Cp416State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationGuardRuntimeState as Cp413State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationAssignmentRuntimeState as Cp414State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationMixedAirLimitRuntimeState as Cp415State,
    advance_cooling_mixed_air_call_state,
    advance_cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment_state as advance_cp417,
    advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_state as advance_cp418,
    advance_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment_state as advance_cp416,
    advance_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_state as advance_cp413,
    advance_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_state as advance_cp414,
    advance_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_state as advance_cp415,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingMixedAirCallSnapshot as MixedAirOwner,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchCpAirAssignmentSnapshot as Snapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchEntrySnapshot as Predecessor,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_snapshots_match_bit_exact,
};

#[test]
fn cp419_boundary_and_three_sites_are_exact() {
    assert_eq!(
        super::PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_CP_AIR_ASSIGNMENT_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2330",
    );
    assert_eq!(
        super::PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2331",
    );
    assert_eq!(
        super::PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_CP_AIR_ASSIGNMENT_SOURCE_ORDER,
        &[
            "read-purchased-air-mixed-air-humidity-ratio-for-post-saturation-capacity-limit-dehumidification-guard-else-branch-cp-air",
            "evaluate-psy-cp-air-fn-w-for-post-saturation-capacity-limit-dehumidification-guard-else-branch-cp-air",
            "assign-local-cp-air-for-post-saturation-capacity-limit-dehumidification-guard-else-branch",
        ],
    );
}

#[test]
fn exhaustive_54_outcomes_49_inactive_five_assignments_and_nine_arrays_are_exact() {
    let routes = all_routes();
    assert_eq!(routes.len(), 36);
    let system = ep_model::IdealLoadsAirSystemId(412);
    let mut cp413_state = Cp413State::new(system);
    let mut cp414_state = Cp414State::new(system);
    let mut cp415_state = Cp415State::new(system);
    let mut cp416_state = Cp416State::new(system);
    let mut cp417_state = Cp417State::new(system);
    let mut cp418_state = Cp418State::new(system);
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
            let cp418 = advance_cp418(&mut cp418_state, cp417).expect("valid CP418 outcome");
            let cp419_route = predecessor_route(cp418).expect("valid CP419 route");
            let snapshot =
                advance(&mut state, cp418, active_input(cp418)).expect("valid CP419 outcome");
            let index = cp419_route.logical_index;

            expected_predecessor[index] += 1;
            if cp419_route.predecessor_guard_false_fallthrough {
                expected_guard_false[index] += 1;
            }
            if cp419_route.predecessor_guard_body_entered {
                expected_guard_body[index] += 1;
            }
            if cp419_route.predecessor_saturation_temperature_assignment_executed {
                expected_saturation_assignment[index] += 1;
            }
            if cp419_route.predecessor_saturation_temperature_mixed_air_limit_executed {
                expected_mixed_air_limit[index] += 1;
            }
            if cp419_route.predecessor_supply_humidity_ratio_assignment_executed {
                expected_humidity_assignment[index] += 1;
            }
            if cp419_route.predecessor_supply_enthalpy_assignment_executed {
                expected_enthalpy_assignment[index] += 1;
            }
            if cp419_route.active {
                expected_else_entry[index] += 1;
                entry_outcomes.push(conceptual_index);
                if matches!(route.predecessor_index, 0..=8 | 20 | 24) {
                    public_entry_outcomes.push(conceptual_index);
                }
            }

            assert_snapshot_bit_preserves_cp418(snapshot, cp418, cp419_route.active);
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
    assert_eq!(
        state.predecessor_dehumidification_guard_else_branch_entry_count,
        5
    );
    assert_eq!(
        state.dehumidification_guard_else_branch_cp_air_assignment_count,
        5
    );
    assert_eq!(state.source_site_execution_count, 15);
    assert_eq!(state.cp418_supply_humidity_ratio_state_owner_count, 36);
    assert_eq!(state.unchanged_supply_humidity_ratio_preservation_count, 36);
    assert_eq!(state.cp418_supply_enthalpy_state_owner_count, 41);
    assert_eq!(state.unchanged_supply_enthalpy_preservation_count, 41);
    assert_eq!(state.cp418_supply_temperature_state_owner_count, 51);
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
        state.predecessor_dehumidification_guard_else_branch_entry_route_counts,
        expected_else_entry
    );
    assert_eq!(
        state.dehumidification_guard_else_branch_cp_air_assignment_route_counts,
        expected_else_entry
    );
    assert_eq!(state.cp419_psychrometric_cp_air_state_owner_count, 5);
    assert_eq!(
        state.cp329_retained_mixed_air_humidity_ratio_owned_read_count,
        5
    );
    assert_eq!(state.mixed_air_humidity_ratio_for_cp_air_read_count, 5);
    assert_eq!(state.psychrometric_cp_air_evaluation_count, 5);
    assert_eq!(state.cp_air_assignment_write_count, 5);
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
        assert!(cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_snapshot_is_exact(
            snapshots[index],
        ));
    }
}

#[test]
fn outer_guard_false_is_distinct_from_later_saturation_guard_false() {
    let outer_false = predecessor_fixture(4, false, false);
    let outer_snapshot = advance(
        &mut State::new(outer_false.system),
        outer_false,
        active_input(outer_false),
    )
    .expect("CP419 assignment");
    assert!(outer_false.predecessor_dehumidification_guard_false_fallthrough);
    assert!(!outer_false.saturation_supply_humidity_ratio_guard_false_fallthrough);
    assert!(
        outer_snapshot.post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_executed
    );

    let later_false = predecessor_fixture(20, false, true);
    let later_snapshot = advance(
        &mut State::new(later_false.system),
        later_false,
        active_input(later_false),
    )
    .expect("CP419 skip");
    assert!(!later_false.predecessor_dehumidification_guard_false_fallthrough);
    assert!(later_false.saturation_supply_humidity_ratio_guard_false_fallthrough);
    assert!(
        !later_snapshot.post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_executed
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
        let input = active_input(predecessor);
        let recursive = advance(&mut recursive_state, predecessor, input).expect("recursive CP419");
        let validated =
            advance_with_validated_route(&mut validated_state, predecessor, route, input)
                .expect("validated-route CP419");

        assert!(
            crate::ideal_loads::cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_snapshots_match_bit_exact(
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
        assert!(
            advance_with_validated_route(
                &mut state,
                predecessor,
                forged,
                active_input(predecessor),
            )
            .is_none()
        );
        assert_eq!(state, before);
    }
}

#[test]
fn public_release_hot_path_has_no_recursive_exact_route_derivation() {
    let source = include_str!("release.rs");
    let start = source
        .find(
            "pub fn advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment",
        )
        .expect("CP419 public release");
    let end = source[start..]
        .find(
            "pub(in crate::ideal_loads) fn private_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_characterization",
        )
        .map(|offset| start + offset)
        .expect("end of CP419 public release");
    let hot_path = &source[start..end];

    assert!(hot_path.contains("committed_latest_route"));
    assert!(hot_path.contains("cooling_mixed_air_call_committed_latest_mixed_air_humidity_ratio"));
    assert!(hot_path.contains("advance_with_validated_route"));
    assert!(!hot_path.contains("predecessor_route("));
    assert!(!hot_path.contains("snapshot_is_exact"));
    assert!(!hot_path.contains("advance(&mut"));
}

#[test]
fn marker_and_predecessor_forgery_are_rejected() {
    let predecessor = predecessor_fixture(4, false, false);
    let snapshot = advance(
        &mut State::new(predecessor.system),
        predecessor,
        active_input(predecessor),
    )
    .expect("CP419 assignment");
    assert!(cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_snapshot_is_exact(snapshot));

    let mut marker_forged = snapshot;
    marker_forged.post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_executed = false;
    assert!(!cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_snapshot_is_exact(marker_forged));

    let mut lineage_forged = snapshot;
    lineage_forged.predecessor_dehumidification_guard_false_fallthrough = false;
    assert!(!cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_snapshot_is_exact(lineage_forged));

    let mut terminal_forged = snapshot;
    terminal_forged.resulting_supply_temperature_c = terminal_forged
        .resulting_supply_temperature_c
        .map(|value| f64::from_bits(value.to_bits() ^ 1));
    assert!(!cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_snapshot_is_exact(terminal_forged));
}

fn assert_snapshot_bit_preserves_cp418(
    snapshot: Snapshot,
    predecessor: Predecessor,
    entered: bool,
) {
    assert_eq!(
        snapshot.post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_executed,
        entered,
    );
    assert!(cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_snapshots_match_bit_exact(
        cp418_shape_for_test(snapshot),
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
    predecessor_fixture_with_state(predecessor_index, body_entered, prefer_guard_false_route).2
}

fn predecessor_fixture_with_state(
    predecessor_index: usize,
    body_entered: bool,
    prefer_guard_false_route: bool,
) -> (Cp417State, Cp418State, Predecessor) {
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
    let mut cp417_state = Cp417State::new(cp416.system);
    let cp417 = advance_cp417(&mut cp417_state, cp416).expect("CP417");
    let mut state = Cp418State::new(cp417.system);
    let snapshot = advance_cp418(&mut state, cp417).expect("CP418");
    (cp417_state, state, snapshot)
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

pub(super) fn assert_overflows_transactionally(
    predecessor: Predecessor,
    logical_index: usize,
    setters: &[fn(&mut State, usize)],
) {
    for set_overflow in setters {
        let mut state = State::new(predecessor.system);
        set_overflow(&mut state, logical_index);
        let before = state.clone();
        assert!(advance(&mut state, predecessor, active_input(predecessor)).is_none());
        assert_eq!(state, before);
    }
}

fn active_input(predecessor: Predecessor) -> Option<ActiveInput> {
    predecessor
        .post_saturation_capacity_limit_dehumidification_guard_else_branch_entered
        .then_some(ActiveInput {
            mixed_air_humidity_ratio: 0.008,
        })
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

mod successor_fixture;

pub(in crate::ideal_loads::calc) use successor_fixture::cp419_all_snapshots_for_successor_tests;
