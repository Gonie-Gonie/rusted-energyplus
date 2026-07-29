use super::super::{
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyEnthalpyAssignmentRetainedRoute as Route,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyEnthalpyAssignmentRuntimeState as State,
    advance_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_state as advance,
    release::{
        active_operands_from_retained_owners_for_test, next_transition_fits_for_test,
    },
};
use super::{
    active_operands, completed_cp351_case, predecessor, private_active_predecessor,
};
use crate::ideal_loads::advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment;

const Q: Route =
    Route::DehumidificationControlConstantSensibleHeatRatioSupplyEnthalpyAssigned;

fn assert_rejected_transactionally(
    runtime: &mut crate::ideal_loads::PurchasedAirRuntimeState,
    system: &ep_model::IdealLoadsAirSystem,
    predecessor:
        crate::ideal_loads::PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioTotalOutputAssignmentSnapshot,
) {
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment(
            runtime,
            system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(&*runtime, &before);
}

#[test]
fn supplied_latest_witness_identity_replay_and_overflow_are_transactional() {
    let completed = completed_cp351_case(-100_000.0, 1.0, true);
    assert!(completed.is_some());
    let Some((mut runtime, system, mut predecessor)) = completed else {
        return;
    };
    predecessor.source_order = &[];
    assert_rejected_transactionally(&mut runtime, &system, predecessor);

    let completed = completed_cp351_case(-1_000.0, 1.0, false);
    assert!(completed.is_some());
    let Some((mut runtime, system, predecessor)) = completed else {
        return;
    };
    let mut forged = predecessor;
    forged.dehumidification_control_none_case_completed_skip = false;
    runtime
        .set_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment_latest_witness(
            system.id,
            forged,
        );
    assert_rejected_transactionally(&mut runtime, &system, predecessor);

    let completed = completed_cp351_case(-1_000.0, 1.0, false);
    assert!(completed.is_some());
    let Some((mut runtime, system, predecessor)) = completed else {
        return;
    };
    assert!(
        advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_ok()
    );
    assert_rejected_transactionally(&mut runtime, &system, predecessor);

    let completed = completed_cp351_case(-1_000.0, 1.0, false);
    assert!(completed.is_some());
    let Some((mut runtime, system, predecessor)) = completed else {
        return;
    };
    if let Some(unit) = runtime.units.get_mut(&system.id) {
        unit.calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment
            .dehumidification_control_none_case_completed_skip_count = usize::MAX;
    }
    assert_rejected_transactionally(&mut runtime, &system, predecessor);
}

#[test]
fn private_active_operands_require_all_three_recursive_owners() {
    let completed = completed_cp351_case(-1_000.0, 1.0, false);
    assert!(completed.is_some());
    let Some((runtime, system, direct)) = completed else {
        return;
    };
    let private = private_active_predecessor(direct, &runtime, &system);
    assert!(private.is_some());
    let Some(private) = private else {
        return;
    };
    let Some(unit) = runtime.units.get(&system.id) else {
        return;
    };
    let operands =
        active_operands_from_retained_owners_for_test(&runtime, unit, &system, private);
    assert!(operands.is_some());
    let Some(operands) = operands else {
        return;
    };
    let Some(mixed_owner) = unit.calc_cooling_mixed_air_call.latest else {
        return;
    };
    let Some(flow_owner) = unit.calc_cooling_supply_mass_flow_positive_guard.latest else {
        return;
    };
    assert_eq!(
        operands.mixed_air_enthalpy_j_per_kg.to_bits(),
        mixed_owner
            .mixed_air_enthalpy_projection_j_per_kg
            .map(f64::to_bits)
            .unwrap_or_default()
    );
    assert_eq!(
        operands.cooling_total_output_w.to_bits(),
        private
            .cooling_total_output_w
            .map(f64::to_bits)
            .unwrap_or_default()
    );
    assert_eq!(
        operands.supply_mass_flow_rate_kg_per_s.to_bits(),
        flow_owner
            .supply_mass_flow_rate_kg_per_s
            .map(f64::to_bits)
            .unwrap_or_default()
    );
    let mut state = State::new(system.id);
    let snapshot = advance(&mut state, private, Some(operands));
    assert!(snapshot.is_some());
}

#[test]
fn coordinated_cp351_cp329_and_cp330_corruption_is_rejected() {
    let completed = completed_cp351_case(-1_000.0, 1.0, false);
    assert!(completed.is_some());
    let Some((runtime, system, direct)) = completed else {
        return;
    };
    let private = private_active_predecessor(direct, &runtime, &system);
    assert!(private.is_some());
    let Some(mut forged_private) = private else {
        return;
    };
    let Some(ratio) = forged_private.cooling_sensible_heat_ratio else {
        return;
    };
    let forged_sensible = forged_private
        .cooling_sensible_output_w
        .map(|value| value * 2.0);
    let Some(forged_sensible) = forged_sensible else {
        return;
    };
    let forged_total = forged_sensible / ratio;
    forged_private.cooling_sensible_output_w = Some(forged_sensible);
    forged_private.calculated_cooling_total_output_w = Some(forged_total);
    forged_private.cooling_total_output_w = Some(forged_total);
    let Some(unit) = runtime.units.get(&system.id) else {
        return;
    };
    assert!(
        active_operands_from_retained_owners_for_test(
            &runtime,
            unit,
            &system,
            forged_private,
        )
        .is_none()
    );

    let completed = completed_cp351_case(-1_000.0, 1.0, false);
    assert!(completed.is_some());
    let Some((mut runtime, system, direct)) = completed else {
        return;
    };
    let private = private_active_predecessor(direct, &runtime, &system);
    assert!(private.is_some());
    let Some(private) = private else {
        return;
    };
    let forged = {
        let Some(unit) = runtime.units.get_mut(&system.id) else {
            return;
        };
        let Some(latest) = unit.calc_cooling_mixed_air_call.latest.as_mut() else {
            return;
        };
        latest.mixed_air_enthalpy_projection_j_per_kg =
            latest.mixed_air_enthalpy_projection_j_per_kg.map(|value| value + 1.0);
        *latest
    };
    runtime.set_cooling_mixed_air_call_latest_witness(system.id, forged);
    let Some(unit) = runtime.units.get(&system.id) else {
        return;
    };
    assert!(
        active_operands_from_retained_owners_for_test(&runtime, unit, &system, private)
            .is_none()
    );

    let completed = completed_cp351_case(-1_000.0, 1.0, false);
    assert!(completed.is_some());
    let Some((mut runtime, system, direct)) = completed else {
        return;
    };
    let private = private_active_predecessor(direct, &runtime, &system);
    assert!(private.is_some());
    let Some(private) = private else {
        return;
    };
    let forged = {
        let Some(unit) = runtime.units.get_mut(&system.id) else {
            return;
        };
        let Some(latest) = unit
            .calc_cooling_supply_mass_flow_positive_guard
            .latest
            .as_mut()
        else {
            return;
        };
        latest.supply_mass_flow_rate_kg_per_s = Some(0.0);
        *latest
    };
    runtime.set_cooling_supply_mass_flow_positive_guard_latest_witness(system.id, forged);
    let Some(unit) = runtime.units.get(&system.id) else {
        return;
    };
    assert!(
        active_operands_from_retained_owners_for_test(&runtime, unit, &system, private)
            .is_none()
    );

    let completed = completed_cp351_case(-1_000.0, 1.0, false);
    assert!(completed.is_some());
    let Some((mut runtime, system, direct)) = completed else {
        return;
    };
    let private = private_active_predecessor(direct, &runtime, &system);
    assert!(private.is_some());
    let Some(private) = private else {
        return;
    };
    let coordinated = {
        let Some(unit) = runtime.units.get_mut(&system.id) else {
            return;
        };
        let Some(latest) = unit
            .calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment
            .latest
            .as_mut()
        else {
            return;
        };
        latest.parent_call_ordinal += 1;
        *latest
    };
    runtime
        .set_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment_latest_witness(
            system.id,
            coordinated,
        );
    let Some(unit) = runtime.units.get(&system.id) else {
        return;
    };
    assert!(
        active_operands_from_retained_owners_for_test(&runtime, unit, &system, private)
            .is_none()
    );
}

#[test]
fn every_active_counter_overflow_rejects_before_mutation() {
    let pending = State::new(ep_model::IdealLoadsAirSystemId(7));
    assert!(next_transition_fits_for_test(
        &pending,
        predecessor(Q, 1),
        Some(active_operands(50_000.0, 10_000.0, 1.5)),
    ));

    macro_rules! reject_overflow {
        ($field:ident) => {{
            let mut state = State::new(ep_model::IdealLoadsAirSystemId(7));
            state.$field = usize::MAX;
            let before = state.clone();
            assert!(
                advance(
                    &mut state,
                    predecessor(Q, 1),
                    Some(active_operands(50_000.0, 10_000.0, 1.5)),
                )
                .is_none()
            );
            assert_eq!(state, before);
        }};
    }
    reject_overflow!(transition_count);
    reject_overflow!(source_site_execution_count);
    reject_overflow!(
        dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_count
    );
    reject_overflow!(mixed_air_enthalpy_read_count);
    reject_overflow!(cooling_total_output_read_count);
    reject_overflow!(supply_mass_flow_rate_read_count);
    reject_overflow!(specific_cooling_output_calculation_count);
    reject_overflow!(supply_enthalpy_calculation_count);
    reject_overflow!(supply_enthalpy_assignment_write_count);
}
