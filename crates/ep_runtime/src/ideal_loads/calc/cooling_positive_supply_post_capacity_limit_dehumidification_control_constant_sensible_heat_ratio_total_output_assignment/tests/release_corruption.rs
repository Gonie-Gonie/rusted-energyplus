use super::super::{
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioTotalOutputAssignmentRetainedRoute as Route,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioTotalOutputAssignmentRuntimeState as State,
    advance_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment_state as advance,
    release::{active_input_from_retained_owner_for_test, next_transition_fits_for_test},
};
use super::{
    active_input, completed_cp350_case, predecessor, private_active_predecessor,
};
use crate::ideal_loads::advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment;

const Q: Route = Route::DehumidificationControlConstantSensibleHeatRatioTotalOutputAssigned;

fn assert_rejected_transactionally(
    runtime: &mut crate::ideal_loads::PurchasedAirRuntimeState,
    system: &ep_model::IdealLoadsAirSystem,
    predecessor:
        crate::ideal_loads::PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentSnapshot,
) {
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment(
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
    let completed = completed_cp350_case(-100_000.0, 1.0, true);
    assert!(completed.is_some());
    let Some((mut runtime, system, mut predecessor)) = completed else {
        return;
    };
    predecessor.source_order = &[];
    assert_rejected_transactionally(&mut runtime, &system, predecessor);

    let completed = completed_cp350_case(-100_000.0, 1.0, true);
    assert!(completed.is_some());
    let Some((mut runtime, system, predecessor)) = completed else {
        return;
    };
    let mut forged = predecessor;
    forged.dehumidification_control_none_case_completed_skip = false;
    runtime
        .set_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_latest_witness(
            system.id,
            forged,
        );
    assert_rejected_transactionally(&mut runtime, &system, predecessor);

    let completed = completed_cp350_case(-1_000.0, 1.0, false);
    assert!(completed.is_some());
    let Some((mut runtime, system, predecessor)) = completed else {
        return;
    };
    let mut wrong_system = system.clone();
    wrong_system.id = ep_model::IdealLoadsAirSystemId(system.id.0 + 1);
    assert_rejected_transactionally(&mut runtime, &wrong_system, predecessor);
    assert!(
        advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_ok()
    );
    assert_rejected_transactionally(&mut runtime, &system, predecessor);

    let completed = completed_cp350_case(-1_000.0, 1.0, false);
    assert!(completed.is_some());
    let Some((mut runtime, system, predecessor)) = completed else {
        return;
    };
    if let Some(unit) = runtime.units.get_mut(&system.id) {
        unit.calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment
            .dehumidification_control_none_case_completed_skip_count = usize::MAX;
    }
    assert_rejected_transactionally(&mut runtime, &system, predecessor);
}

#[test]
fn private_active_owner_requires_retained_cp350_recursive_proof() {
    let completed = completed_cp350_case(-1_000.0, 1.0, false);
    assert!(completed.is_some());
    let Some((runtime, system, direct)) = completed else {
        return;
    };
    let private = private_active_predecessor(direct, &runtime);
    assert!(private.is_some());
    let Some(private) = private else {
        return;
    };
    let Some(unit) = runtime.units.get(&system.id) else {
        return;
    };
    let input = active_input_from_retained_owner_for_test(&runtime, unit, &system, private);
    assert!(input.is_some());
    let Some(input) = input else {
        return;
    };
    assert_eq!(
        input.cooling_sensible_heat_ratio.to_bits(),
        system.cooling_sensible_heat_ratio.to_bits()
    );
    let mut state = State::new(system.id);
    let snapshot = advance(&mut state, private, Some(input));
    assert!(snapshot.is_some());
    let Some(snapshot) = snapshot else {
        return;
    };
    assert_eq!(
        snapshot.cooling_sensible_output_w.map(f64::to_bits),
        private.cooling_sensible_output_w.map(f64::to_bits)
    );
}

#[test]
fn coordinated_cp350_counterfactual_corruption_is_rejected() {
    let completed = completed_cp350_case(-1_000.0, 1.0, false);
    assert!(completed.is_some());
    let Some((runtime, system, direct)) = completed else {
        return;
    };
    let private = private_active_predecessor(direct, &runtime);
    assert!(private.is_some());
    let Some(mut private) = private else {
        return;
    };
    let Some(unit) = runtime.units.get(&system.id) else {
        return;
    };
    assert!(active_input_from_retained_owner_for_test(&runtime, unit, &system, private).is_some());

    let Some(cp_air) = private.cp_air_j_per_kg_k else {
        return;
    };
    let Some(mixed) = private.mixed_air_temperature_c else {
        return;
    };
    let Some(supply) = private.supply_temperature_c else {
        return;
    };
    let forged_flow = private
        .supply_mass_flow_rate_kg_per_s
        .map(|value| value * 2.0);
    let Some(forged_flow) = forged_flow else {
        return;
    };
    let first = forged_flow * cp_air;
    let difference = mixed - supply;
    let forged_output = first * difference;
    private.supply_mass_flow_rate_kg_per_s = Some(forged_flow);
    private.supply_mass_flow_rate_times_cp_air_w_per_k = Some(first);
    private.mixed_air_minus_supply_temperature_k = Some(difference);
    private.calculated_cooling_sensible_output_w = Some(forged_output);
    private.cooling_sensible_output_w = Some(forged_output);
    assert!(active_input_from_retained_owner_for_test(&runtime, unit, &system, private).is_none());

    let completed = completed_cp350_case(-1_000.0, 1.0, false);
    assert!(completed.is_some());
    let Some((mut runtime, system, direct)) = completed else {
        return;
    };
    let private = private_active_predecessor(direct, &runtime);
    assert!(private.is_some());
    let Some(private) = private else {
        return;
    };
    let coordinated = {
        let Some(unit) = runtime.units.get_mut(&system.id) else {
            return;
        };
        let Some(latest) = unit
            .calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment
            .latest
            .as_mut()
        else {
            return;
        };
        latest.parent_call_ordinal += 1;
        *latest
    };
    runtime
        .set_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_latest_witness(
            system.id,
            coordinated,
        );
    let Some(unit) = runtime.units.get(&system.id) else {
        return;
    };
    assert!(active_input_from_retained_owner_for_test(&runtime, unit, &system, private).is_none());
}

#[test]
fn model_owned_ratio_is_bit_exact_without_line_local_range_revalidation() {
    let completed = completed_cp350_case(-1_000.0, 1.0, false);
    assert!(completed.is_some());
    let Some((runtime, mut system, direct)) = completed else {
        return;
    };
    let private = private_active_predecessor(direct, &runtime);
    assert!(private.is_some());
    let Some(private) = private else {
        return;
    };
    let Some(unit) = runtime.units.get(&system.id) else {
        return;
    };
    for ratio in [0.0, -0.0, 2.0, f64::INFINITY, f64::NAN] {
        system.cooling_sensible_heat_ratio = ratio;
        let input =
            active_input_from_retained_owner_for_test(&runtime, unit, &system, private);
        assert!(input.is_some());
        let Some(input) = input else {
            return;
        };
        assert_eq!(input.cooling_sensible_heat_ratio.to_bits(), ratio.to_bits());
    }
}

#[test]
fn every_active_counter_overflow_rejects_before_mutation() {
    let pending = State::new(ep_model::IdealLoadsAirSystemId(7));
    assert!(next_transition_fits_for_test(
        &pending,
        predecessor(Q, 1),
        Some(active_input(0.7)),
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
                    Some(active_input(0.7)),
                )
                .is_none()
            );
            assert_eq!(state, before);
        }};
    }
    reject_overflow!(transition_count);
    reject_overflow!(source_site_execution_count);
    reject_overflow!(
        dehumidification_control_constant_sensible_heat_ratio_total_output_assignment_count
    );
    reject_overflow!(cooling_sensible_output_read_count);
    reject_overflow!(cooling_sensible_heat_ratio_read_count);
    reject_overflow!(cooling_total_output_calculation_count);
    reject_overflow!(cooling_total_output_assignment_write_count);
}
