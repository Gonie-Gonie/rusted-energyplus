use ep_model::DehumidificationControlType;

use super::super::{
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentRetainedRoute as Route,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentRuntimeState,
    advance_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_state,
    completed_direct_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_is_consistent,
    release::{next_transition_fits_for_test, snapshots_match_bit_exact_for_test},
};
use super::{completed_cp348_case, owner_input, private_cp348_case};
use crate::ideal_loads::advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment;

fn assert_rejected_transactionally(
    runtime: &mut crate::ideal_loads::PurchasedAirRuntimeState,
    system: &ep_model::IdealLoadsAirSystem,
    predecessor: crate::ideal_loads::PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseEntrySnapshot,
) {
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment(
            runtime,
            system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(&*runtime, &before);
}

#[test]
fn supplied_retained_and_private_cp348_drift_are_rejected_transactionally() {
    let completed = completed_cp348_case(-100_000.0, 1.0, true);
    assert!(completed.is_some());
    let Some((mut runtime, system, mut predecessor)) = completed else {
        return;
    };
    predecessor.source_order = &[];
    assert_rejected_transactionally(&mut runtime, &system, predecessor);

    let completed = completed_cp348_case(-100_000.0, 1.0, true);
    assert!(completed.is_some());
    let Some((mut runtime, system, predecessor)) = completed else {
        return;
    };
    if let Some(unit) = runtime.units.get_mut(&system.id)
        && let Some(latest) = unit
            .calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry
            .latest
            .as_mut()
    {
        latest.parent_call_ordinal += 1;
    }
    assert_rejected_transactionally(&mut runtime, &system, predecessor);

    let completed = completed_cp348_case(-100_000.0, 1.0, true);
    assert!(completed.is_some());
    let Some((mut runtime, system, predecessor)) = completed else {
        return;
    };
    let mut drift = predecessor;
    drift.dehumidification_control_none_case_completed_skip = false;
    runtime
        .set_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry_latest_witness(
            system.id,
            drift,
        );
    assert_rejected_transactionally(&mut runtime, &system, predecessor);
}

#[test]
fn premature_cp349_witness_and_counter_drift_are_rejected_transactionally() {
    let completed = completed_cp348_case(-100_000.0, 1.0, true);
    assert!(completed.is_some());
    let Some((mut runtime, system, predecessor)) = completed else {
        return;
    };
    let mut forged_state =
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentRuntimeState::new(
            system.id,
        );
    let forged =
        advance_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_state(
            &mut forged_state,
            predecessor,
            None,
        );
    assert!(forged.is_some());
    let Some(forged) = forged else {
        return;
    };
    runtime
        .set_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_latest_witness(
            system.id,
            forged,
        );
    assert_rejected_transactionally(&mut runtime, &system, predecessor);

    let completed = completed_cp348_case(-100_000.0, 1.0, true);
    assert!(completed.is_some());
    let Some((mut runtime, system, predecessor)) = completed else {
        return;
    };
    if let Some(unit) = runtime.units.get_mut(&system.id) {
        unit.calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment
            .dehumidification_control_none_case_completed_skip_count = 1;
    }
    assert_rejected_transactionally(&mut runtime, &system, predecessor);
}

#[test]
fn completed_route_and_private_witness_drift_fail_consistency() {
    let completed = completed_cp348_case(-100_000.0, 1.0, true);
    assert!(completed.is_some());
    let Some((mut runtime, system, predecessor)) = completed else {
        return;
    };
    let snapshot =
        advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment(
            &mut runtime,
            &system,
            predecessor,
        );
    assert!(snapshot.is_ok());
    let Ok(snapshot) = snapshot else {
        return;
    };
    if let Some(unit) = runtime.units.get_mut(&system.id) {
        unit.calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment
            .latest_route =
            Some(Route::DehumidificationControlConstantSensibleHeatRatioCpAirAssigned);
    }
    let Some(unit) = runtime.units.get(&system.id) else {
        return;
    };
    assert!(
        !completed_direct_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_is_consistent(
            &runtime,
            unit,
            &system,
            snapshot,
            runtime
                .cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_latest_witness(
                    system.id,
                ),
        )
    );

    let completed = completed_cp348_case(-100_000.0, 1.0, true);
    assert!(completed.is_some());
    let Some((mut runtime, system, predecessor)) = completed else {
        return;
    };
    let snapshot =
        advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment(
            &mut runtime,
            &system,
            predecessor,
        );
    assert!(snapshot.is_ok());
    let Ok(snapshot) = snapshot else {
        return;
    };
    let mut drift = snapshot;
    drift.source_order = &[];
    runtime
        .set_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_latest_witness(
            system.id,
            drift,
        );
    let Some(unit) = runtime.units.get(&system.id) else {
        return;
    };
    assert!(
        !completed_direct_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_is_consistent(
            &runtime,
            unit,
            &system,
            snapshot,
            Some(drift),
        )
    );
}

#[test]
fn private_numeric_snapshot_and_witness_matching_is_bit_exact() {
    let private = private_cp348_case(DehumidificationControlType::ConstantSensibleHeatRatio);
    assert!(private.is_some());
    let Some((runtime, system, predecessor)) = private else {
        return;
    };
    let input = owner_input(&runtime, system.id, predecessor);
    assert!(input.is_some());
    let mut state =
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentRuntimeState::new(
            system.id,
        );
    let snapshot =
        advance_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_state(
            &mut state,
            predecessor,
            input,
        );
    assert!(snapshot.is_some());
    let Some(snapshot) = snapshot else {
        return;
    };
    assert!(snapshots_match_bit_exact_for_test(snapshot, snapshot));
    for field in ["operand", "result", "assignment"] {
        let mut drift = snapshot;
        match field {
            "operand" => {
                drift.mixed_air_humidity_ratio = drift
                    .mixed_air_humidity_ratio
                    .map(|value| f64::from_bits(value.to_bits() ^ 1));
            }
            "result" => {
                drift.psychrometric_cp_air_result_j_per_kg_k = drift
                    .psychrometric_cp_air_result_j_per_kg_k
                    .map(|value| f64::from_bits(value.to_bits() ^ 1));
            }
            _ => {
                drift.cp_air_j_per_kg_k = drift
                    .cp_air_j_per_kg_k
                    .map(|value| f64::from_bits(value.to_bits() ^ 1));
            }
        }
        assert!(!snapshots_match_bit_exact_for_test(snapshot, drift));
    }
}

#[test]
fn every_public_conditional_counter_increment_is_preflighted() {
    let completed = completed_cp348_case(-100_000.0, 1.0, true);
    assert!(completed.is_some());
    let Some((runtime, system, predecessor)) = completed else {
        return;
    };
    let Some(baseline) = runtime.units.get(&system.id) else {
        return;
    };
    macro_rules! overflow {
        ($field:ident) => {{
            let mut state = baseline
                .calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment
                .clone();
            state.$field = usize::MAX;
            assert!(!next_transition_fits_for_test(&state, predecessor, None));
        }};
    }
    overflow!(transition_count);
    overflow!(dehumidification_control_none_case_completed_skip_count);
    overflow!(witnessed_dehumidification_control_none_case_completed_skip_count);
}

#[test]
fn every_skip_route_counter_increment_is_preflighted() {
    macro_rules! overflow_route {
        ($predecessor:expr, $field:ident) => {{
            let predecessor = $predecessor;
            let mut state =
                PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentRuntimeState::new(
                    predecessor.system,
                );
            state.$field = usize::MAX;
            let before = state.clone();
            assert!(
                advance_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_state(
                    &mut state,
                    predecessor,
                    None,
                )
                .is_none()
            );
            assert_eq!(state, before);
        }};
    }

    let unit_off = completed_cp348_case(-1_000.0, 0.0, true);
    let non_cooling = completed_cp348_case(1.0, 1.0, true);
    let positive_false = completed_cp348_case(-1.0e-40, 1.0, true);
    let humidistat = private_cp348_case(DehumidificationControlType::Humidistat);
    let constant_supply =
        private_cp348_case(DehumidificationControlType::ConstantSupplyHumidityRatio);
    assert!(unit_off.is_some());
    assert!(non_cooling.is_some());
    assert!(positive_false.is_some());
    assert!(humidistat.is_some());
    assert!(constant_supply.is_some());
    let Some((_, _, unit_off)) = unit_off else {
        return;
    };
    let Some((_, _, non_cooling)) = non_cooling else {
        return;
    };
    let Some((_, _, positive_false)) = positive_false else {
        return;
    };
    let Some((_, _, humidistat)) = humidistat else {
        return;
    };
    let Some((_, _, constant_supply)) = constant_supply else {
        return;
    };

    overflow_route!(unit_off, unit_off_skip_count);
    overflow_route!(non_cooling, non_cooling_skip_count);
    overflow_route!(positive_false, positive_guard_false_fallthrough_skip_count);
    overflow_route!(
        positive_false,
        witnessed_positive_guard_false_fallthrough_skip_count
    );
    overflow_route!(
        humidistat,
        dehumidification_control_humidistat_case_selected_skip_count
    );
    overflow_route!(
        humidistat,
        witnessed_dehumidification_control_humidistat_case_selected_skip_count
    );
    overflow_route!(
        constant_supply,
        dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count
    );
    overflow_route!(
        constant_supply,
        witnessed_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count
    );
}

#[test]
fn every_private_k_counter_increment_is_preflighted() {
    let private = private_cp348_case(DehumidificationControlType::ConstantSensibleHeatRatio);
    assert!(private.is_some());
    let Some((runtime, system, predecessor)) = private else {
        return;
    };
    let input = owner_input(&runtime, system.id, predecessor);
    assert!(input.is_some());
    macro_rules! overflow {
        ($field:ident) => {{
            let mut state =
                PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentRuntimeState::new(
                    system.id,
                );
            state.$field = usize::MAX;
            let before = state.clone();
            assert!(
                advance_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_state(
                    &mut state,
                    predecessor,
                    input,
                )
                .is_none()
            );
            assert_eq!(state, before);
        }};
    }
    overflow!(transition_count);
    overflow!(dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_count);
    overflow!(source_site_execution_count);
    overflow!(mixed_air_humidity_ratio_read_count);
    overflow!(psychrometric_cp_air_evaluation_count);
    overflow!(cp_air_assignment_write_count);
    overflow!(
        witnessed_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_count
    );
}
