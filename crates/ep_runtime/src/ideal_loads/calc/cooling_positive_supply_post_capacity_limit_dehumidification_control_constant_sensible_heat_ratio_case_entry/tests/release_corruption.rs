use super::super::{
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseEntryRetainedRoute as Route,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseEntryRuntimeState,
    advance_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry_state,
    completed_direct_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry_is_consistent,
    release::next_transition_fits_for_test,
};
use super::{completed_cp347_case, private_cp347_case};
use crate::ideal_loads::advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry;

fn assert_rejected_transactionally(
    runtime: &mut crate::ideal_loads::PurchasedAirRuntimeState,
    system: &ep_model::IdealLoadsAirSystem,
    predecessor: crate::ideal_loads::PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseSnapshot,
) {
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry(
            runtime,
            system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(&*runtime, &before);
}

#[test]
fn supplied_retained_and_private_cp347_drift_are_rejected_transactionally() {
    let completed = completed_cp347_case(-100_000.0, 1.0, true);
    assert!(completed.is_some());
    let Some((mut runtime, system, mut predecessor)) = completed else {
        return;
    };
    predecessor.dehumidification_control_none_case_exited_via_break = false;
    assert_rejected_transactionally(&mut runtime, &system, predecessor);

    let completed = completed_cp347_case(-100_000.0, 1.0, true);
    assert!(completed.is_some());
    let Some((mut runtime, system, predecessor)) = completed else {
        return;
    };
    if let Some(unit) = runtime.units.get_mut(&system.id)
        && let Some(latest) = unit
            .calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case
            .latest
            .as_mut()
    {
        latest.source_order = &[];
    }
    assert_rejected_transactionally(&mut runtime, &system, predecessor);

    let completed = completed_cp347_case(-100_000.0, 1.0, true);
    assert!(completed.is_some());
    let Some((mut runtime, system, predecessor)) = completed else {
        return;
    };
    let mut drift = predecessor;
    drift.parent_call_ordinal += 1;
    runtime
        .set_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_latest_witness(
            system.id,
            drift,
        );
    assert_rejected_transactionally(&mut runtime, &system, predecessor);
}

#[test]
fn premature_private_cp348_witness_is_rejected_transactionally() {
    let completed = completed_cp347_case(-100_000.0, 1.0, true);
    assert!(completed.is_some());
    let Some((mut runtime, system, predecessor)) = completed else {
        return;
    };
    let mut forged_state =
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseEntryRuntimeState::new(
            system.id,
        );
    let forged =
        advance_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry_state(
            &mut forged_state,
            predecessor,
        );
    assert!(forged.is_some());
    let Some(forged) = forged else {
        return;
    };
    runtime
        .set_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry_latest_witness(
            system.id,
            forged,
        );
    assert_rejected_transactionally(&mut runtime, &system, predecessor);
}

#[test]
fn completed_private_route_and_witness_drift_fail_consistency() {
    let completed = completed_cp347_case(-100_000.0, 1.0, true);
    assert!(completed.is_some());
    let Some((mut runtime, system, predecessor)) = completed else {
        return;
    };
    let snapshot =
        advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry(
            &mut runtime,
            &system,
            predecessor,
        );
    assert!(snapshot.is_ok());
    let Ok(snapshot) = snapshot else {
        return;
    };
    if let Some(unit) = runtime.units.get_mut(&system.id) {
        unit.calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry
            .latest_route =
            Some(Route::DehumidificationControlConstantSensibleHeatRatioCaseEntered);
    }
    let Some(unit) = runtime.units.get(&system.id) else {
        return;
    };
    assert!(
        !completed_direct_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry_is_consistent(
            &runtime,
            unit,
            &system,
            snapshot,
            runtime
                .cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry_latest_witness(
                    system.id,
                ),
        )
    );
}

#[test]
fn every_public_counter_increment_is_preflighted() {
    let completed = completed_cp347_case(-100_000.0, 1.0, true);
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
                .calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry
                .clone();
            state.$field = usize::MAX;
            assert!(!next_transition_fits_for_test(&state, predecessor));
        }};
    }
    overflow!(transition_count);
    overflow!(dehumidification_control_none_case_completed_skip_count);
}

#[test]
fn every_private_entry_counter_increment_is_preflighted() {
    let predecessor =
        private_cp347_case(ep_model::DehumidificationControlType::ConstantSensibleHeatRatio);
    assert!(predecessor.is_some());
    let Some(predecessor) = predecessor else {
        return;
    };
    macro_rules! overflow {
        ($field:ident) => {{
            let mut state =
                PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseEntryRuntimeState::new(
                    predecessor.system,
                );
            state.$field = usize::MAX;
            let before = state.clone();
            assert!(
                advance_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry_state(
                    &mut state,
                    predecessor,
                )
                .is_none()
            );
            assert_eq!(state, before);
        }};
    }
    overflow!(transition_count);
    overflow!(dehumidification_control_constant_sensible_heat_ratio_case_entry_count);
    overflow!(source_site_execution_count);
    overflow!(dehumidification_control_constant_sensible_heat_ratio_case_entry_site_count);
}
