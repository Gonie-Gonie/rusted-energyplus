//! CP386 source boundary, 30-route refinement, and corruption tests.

use ep_model::DehumidificationControlType as D;

use super::*;
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_total_output_guard::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputGuardRuntimeState as Cp383State,
    active_input_for_cp384_test as cp383_active_input,
    advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard_state as advance_cp383,
    predecessor_for_cp384_test as cp382_predecessor,
};
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputMaximumCapacityAssignmentRuntimeState as Cp384State,
    advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment_state as advance_cp384,
};
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentActiveOperands as Cp385Operands,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentRetainedInput as Cp385Input,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentRuntimeState as Cp385State,
    advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_state as advance_cp385,
};
use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentSnapshot;

fn predecessor(
    inherited: usize,
    outcome: usize,
    assignment: bool,
    ordinal: usize,
) -> PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentSnapshot
{
    let cp382 = cp382_predecessor(inherited, outcome, ordinal);
    let mut cp383_state = Cp383State::new(cp382.system);
    let cp383_input = (outcome == 1).then(|| {
        cp383_active_input(cp382, if assignment { 99.0 } else { 100.0 })
            .expect("active CP383 input")
    });
    let cp383 = advance_cp383(&mut cp383_state, cp382, cp383_input).expect("CP383");
    let mut cp384_state = Cp384State::new(cp383.system);
    let cp384 = advance_cp384(&mut cp384_state, cp383).expect("CP384");
    let cp385_input = cp384
        .predecessor_dehumidification_total_output_capacity_guard_evaluated
        .then(|| Cp385Input {
            preexisting_supply_enthalpy_j_per_kg: f64::from_bits(0x40e4_86a0_0000_0001),
            active_operands: cp384
                .dehumidification_total_output_maximum_capacity_assignment_executed
                .then(|| Cp385Operands {
                    mixed_air_enthalpy_j_per_kg: 50_000.0,
                    cooling_total_output_w: cp384
                        .resulting_cooling_total_output_w
                        .expect("CP384 output"),
                    supply_mass_flow_rate_kg_per_s: 2.0,
                }),
        });
    let mut cp385_state = Cp385State::new(cp384.system);
    advance_cp385(&mut cp385_state, cp384, cp385_input).expect("CP385")
}

const fn input(selector: D) -> PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlSwitchActiveInput {
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlSwitchActiveInput {
        dehumidification_control_type: selector,
    }
}

#[test]
fn cp386_boundaries_and_physical_two_site_order_are_exact() {
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2272",
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_FIRST_EXCLUDED_LEXICAL_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2273",
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2277",
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_SOURCE_ORDER,
        [
            "read-purchased-air-dehumidification-control-type",
            "dispatch-dehumidification-control-switch",
        ],
    );
}

#[test]
fn cp386_has_eighteen_inactive_and_twelve_lineage_constrained_active_routes() {
    let system = predecessor(0, 0, false, 1).system;
    let mut state = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlSwitchRuntimeState::new(system);
    let mut snapshots = Vec::new();
    let mut ordinal = 1;

    for inherited in 0..3 {
        let predecessor = predecessor(inherited, 0, false, ordinal);
        snapshots.push(
            advance_cooling_post_saturation_capacity_limit_dehumidification_control_switch_state(
                &mut state,
                predecessor,
                None,
            )
            .expect("inactive base route"),
        );
        ordinal += 1;
    }
    for inherited in 3..8 {
        for (outcome, assignment) in [(0, false), (2, false), (1, false)] {
            let predecessor = predecessor(inherited, outcome, assignment, ordinal);
            snapshots.push(
                advance_cooling_post_saturation_capacity_limit_dehumidification_control_switch_state(
                    &mut state,
                    predecessor,
                    None,
                )
                .expect("inactive lineage route"),
            );
            ordinal += 1;
        }
    }

    let selectors = [
        D::ConstantSensibleHeatRatio,
        D::Humidistat,
        D::None,
        D::ConstantSupplyHumidityRatio,
    ];
    for inherited in [3, 4] {
        for selector in selectors {
            let predecessor = predecessor(inherited, 1, true, ordinal);
            snapshots.push(
                advance_cooling_post_saturation_capacity_limit_dehumidification_control_switch_state(
                    &mut state,
                    predecessor,
                    Some(input(selector)),
                )
                .expect("unconstrained H/F active route"),
            );
            ordinal += 1;
        }
    }
    for (inherited, selectors) in [
        (5, &[D::Humidistat][..]),
        (6, &[D::None][..]),
        (
            7,
            &[
                D::ConstantSensibleHeatRatio,
                D::ConstantSupplyHumidityRatio,
            ][..],
        ),
    ] {
        for selector in selectors {
            let predecessor = predecessor(inherited, 1, true, ordinal);
            snapshots.push(
                advance_cooling_post_saturation_capacity_limit_dehumidification_control_switch_state(
                    &mut state,
                    predecessor,
                    Some(input(*selector)),
                )
                .expect("constrained active route"),
            );
            ordinal += 1;
        }
    }

    assert_eq!(snapshots.len(), 30);
    assert_eq!(state.transition_count, 30);
    assert_eq!(state.inactive_transition_count, 18);
    assert_eq!(state.dehumidification_control_switch_count, 12);
    assert_eq!(state.source_site_execution_count, 24);
    assert_eq!(state.dehumidification_control_type_read_count, 12);
    assert_eq!(state.dehumidification_control_switch_dispatch_count, 12);
    assert_eq!(state.predecessor_route_counts.iter().sum::<usize>(), 30);
    assert_eq!(
        state.dehumidification_control_constant_sensible_heat_ratio_case_selection_count,
        3,
    );
    assert_eq!(state.dehumidification_control_humidistat_case_selection_count, 3);
    assert_eq!(state.dehumidification_control_none_case_selection_count, 3);
    assert_eq!(
        state.dehumidification_control_constant_supply_humidity_ratio_case_selection_count,
        3,
    );
    assert_eq!(
        snapshots
            .iter()
            .filter(|snapshot| {
                cooling_post_saturation_capacity_limit_dehumidification_control_switch_snapshot_is_exact_direct_release(
                    **snapshot,
                )
            })
            .count(),
        11,
    );
    for snapshot in snapshots {
        assert_eq!(
            snapshot
                .predecessor_resulting_supply_enthalpy_j_per_kg
                .map(f64::to_bits),
            snapshot.resulting_supply_enthalpy_j_per_kg.map(f64::to_bits),
        );
    }
}

#[test]
fn invalid_lineage_selector_pairs_and_activity_mismatches_are_atomic() {
    for (predecessor, active_input) in [
        (predecessor(3, 1, true, 1), None),
        (predecessor(0, 0, false, 1), Some(input(D::None))),
        (predecessor(5, 1, true, 1), Some(input(D::None))),
        (predecessor(6, 1, true, 1), Some(input(D::Humidistat))),
        (predecessor(7, 1, true, 1), Some(input(D::None))),
    ] {
        let mut state = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlSwitchRuntimeState::new(predecessor.system);
        let before = state.clone();
        assert!(
            advance_cooling_post_saturation_capacity_limit_dehumidification_control_switch_state(
                &mut state,
                predecessor,
                active_input,
            )
            .is_none(),
        );
        assert_eq!(state, before);
    }
}

#[test]
fn predecessor_metadata_and_enthalpy_corruption_are_rejected_atomically() {
    let clean = predecessor(3, 1, true, 1);
    let mut cases = Vec::new();
    let mut bad_source = clean;
    bad_source.source = "wrong";
    cases.push(bad_source);
    let mut bad_payload = clean;
    bad_payload.resulting_supply_enthalpy_j_per_kg = Some(f64::from_bits(
        clean
            .resulting_supply_enthalpy_j_per_kg
            .expect("active enthalpy")
            .to_bits()
            ^ 1,
    ));
    cases.push(bad_payload);

    for predecessor in cases {
        let mut state = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlSwitchRuntimeState::new(predecessor.system);
        let before = state.clone();
        assert!(
            advance_cooling_post_saturation_capacity_limit_dehumidification_control_switch_state(
                &mut state,
                predecessor,
                Some(input(D::None)),
            )
            .is_none(),
        );
        assert_eq!(state, before);
    }
}

#[test]
fn all_ten_inherited_control_flags_are_revalidated_across_u_k_x_f_m_routes() {
    let routes = [
        predecessor(0, 0, false, 1),
        predecessor(3, 0, false, 1),
        predecessor(3, 2, false, 1),
        predecessor(3, 1, false, 1),
        predecessor(3, 1, true, 1),
    ];
    for predecessor in routes {
        for flag in 0..10 {
            let mut corrupted = predecessor;
            match flag {
                0 => {
                    corrupted.predecessor_capacity_limit_guard_evaluated =
                        !corrupted.predecessor_capacity_limit_guard_evaluated;
                }
                1 => {
                    corrupted.predecessor_capacity_limit_body_entered =
                        !corrupted.predecessor_capacity_limit_body_entered;
                }
                2 => {
                    corrupted.predecessor_active_capacity_limit_guard_false_fallthrough =
                        !corrupted.predecessor_active_capacity_limit_guard_false_fallthrough;
                }
                3 => {
                    corrupted.predecessor_dehumidification_guard_evaluated =
                        !corrupted.predecessor_dehumidification_guard_evaluated;
                }
                4 => {
                    corrupted.predecessor_dehumidification_body_entered =
                        !corrupted.predecessor_dehumidification_body_entered;
                }
                5 => {
                    corrupted.predecessor_dehumidification_guard_false_fallthrough =
                        !corrupted.predecessor_dehumidification_guard_false_fallthrough;
                }
                6 => {
                    corrupted.predecessor_dehumidification_total_output_assignment_executed =
                        !corrupted.predecessor_dehumidification_total_output_assignment_executed;
                }
                7 => {
                    corrupted.predecessor_dehumidification_total_output_capacity_guard_evaluated =
                        !corrupted
                            .predecessor_dehumidification_total_output_capacity_guard_evaluated;
                }
                8 => {
                    corrupted
                        .predecessor_dehumidification_total_output_capacity_adjustment_body_entered =
                        !corrupted
                            .predecessor_dehumidification_total_output_capacity_adjustment_body_entered;
                }
                9 => {
                    corrupted
                        .predecessor_dehumidification_total_output_capacity_guard_false_fallthrough =
                        !corrupted
                            .predecessor_dehumidification_total_output_capacity_guard_false_fallthrough;
                }
                _ => unreachable!(),
            }
            let mut state = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlSwitchRuntimeState::new(corrupted.system);
            let active_input = predecessor
                .supply_enthalpy_assignment_executed
                .then_some(input(D::None));
            assert!(
                advance_cooling_post_saturation_capacity_limit_dehumidification_control_switch_state(
                    &mut state,
                    corrupted,
                    active_input,
                )
                .is_none(),
                "route accepted inverted inherited control flag {flag}",
            );
            assert_eq!(state.transition_count, 0);
        }
    }
}

#[test]
fn overflow_rejects_before_mutation() {
    let predecessor = predecessor(3, 1, true, 1);
    let mut state = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlSwitchRuntimeState::new(predecessor.system);
    state.source_site_execution_count = usize::MAX;
    let before = state.clone();
    assert!(
        advance_cooling_post_saturation_capacity_limit_dehumidification_control_switch_state(
            &mut state,
            predecessor,
            Some(input(D::None)),
        )
        .is_none(),
    );
    assert_eq!(state, before);
}
