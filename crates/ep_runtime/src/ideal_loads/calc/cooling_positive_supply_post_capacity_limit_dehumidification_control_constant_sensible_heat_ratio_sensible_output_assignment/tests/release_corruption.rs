use super::super::{
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentRetainedRoute as Route,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentRuntimeState as State,
    advance_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_state as advance,
    release::{active_input_from_retained_owners_for_test, next_transition_fits_for_test},
};
use super::{active_input, completed_cp349_case, predecessor};
use crate::ideal_loads::advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment;
use crate::psychrometrics::energyplus_psy_cp_air_fn_w;

const Q: Route = Route::DehumidificationControlConstantSensibleHeatRatioSensibleOutputAssigned;

fn assert_rejected_transactionally(
    runtime: &mut crate::ideal_loads::PurchasedAirRuntimeState,
    system: &ep_model::IdealLoadsAirSystem,
    predecessor:
        crate::ideal_loads::PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentSnapshot,
) {
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment(
            runtime,
            system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(&*runtime, &before);
}

fn private_active_predecessor(
    mut predecessor:
        crate::ideal_loads::PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentSnapshot,
    runtime: &crate::ideal_loads::PurchasedAirRuntimeState,
) -> Option<
    crate::ideal_loads::PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentSnapshot,
>{
    let humidity = runtime
        .units
        .get(&predecessor.system)?
        .calc_cooling_mixed_air_call
        .latest?
        .mixed_air_humidity_ratio?;
    let cp_air = energyplus_psy_cp_air_fn_w(humidity);
    predecessor.predecessor_dehumidification_control_type =
        Some(ep_model::DehumidificationControlType::ConstantSensibleHeatRatio);
    predecessor.predecessor_dehumidification_control_none_case_completed = false;
    predecessor.predecessor_dehumidification_control_none_case_completed_skip = false;
    predecessor.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered =
        true;
    predecessor.dehumidification_control_none_case_completed_skip = false;
    predecessor.dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_executed =
        true;
    predecessor.mixed_air_humidity_ratio_read = true;
    predecessor.mixed_air_humidity_ratio = Some(humidity);
    predecessor.psychrometric_cp_air_evaluated = true;
    predecessor.psychrometric_cp_air_result_j_per_kg_k = Some(cp_air);
    predecessor.cp_air_assigned = true;
    predecessor.cp_air_j_per_kg_k = Some(cp_air);
    Some(predecessor)
}

#[test]
fn supplied_latest_and_private_cp349_corruption_are_transactional() {
    let completed = completed_cp349_case(-100_000.0, 1.0, true);
    assert!(completed.is_some());
    let Some((mut runtime, system, mut predecessor)) = completed else {
        return;
    };
    predecessor.source_order = &[];
    assert_rejected_transactionally(&mut runtime, &system, predecessor);

    let completed = completed_cp349_case(-100_000.0, 1.0, true);
    assert!(completed.is_some());
    let Some((mut runtime, system, predecessor)) = completed else {
        return;
    };
    if let Some(unit) = runtime.units.get_mut(&system.id)
        && let Some(latest) = unit
            .calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment
            .latest
            .as_mut()
    {
        latest.parent_call_ordinal += 1;
    }
    assert_rejected_transactionally(&mut runtime, &system, predecessor);

    let completed = completed_cp349_case(-100_000.0, 1.0, true);
    assert!(completed.is_some());
    let Some((mut runtime, system, predecessor)) = completed else {
        return;
    };
    let mut forged = predecessor;
    forged.dehumidification_control_none_case_completed_skip = false;
    runtime
        .set_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_latest_witness(
            system.id,
            forged,
        );
    assert_rejected_transactionally(&mut runtime, &system, predecessor);
}

#[test]
fn identity_replay_and_public_counter_overflow_are_transactional() {
    let completed = completed_cp349_case(-1_000.0, 1.0, false);
    assert!(completed.is_some());
    let Some((mut runtime, system, predecessor)) = completed else {
        return;
    };
    let mut wrong_system = system.clone();
    wrong_system.id = ep_model::IdealLoadsAirSystemId(system.id.0 + 1);
    assert_rejected_transactionally(&mut runtime, &wrong_system, predecessor);

    let snapshot =
        advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment(
            &mut runtime,
            &system,
            predecessor,
        );
    assert!(snapshot.is_ok());
    assert_rejected_transactionally(&mut runtime, &system, predecessor);

    let completed = completed_cp349_case(-1_000.0, 1.0, false);
    assert!(completed.is_some());
    let Some((mut runtime, system, predecessor)) = completed else {
        return;
    };
    if let Some(unit) = runtime.units.get_mut(&system.id) {
        unit.calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment
            .dehumidification_control_none_case_completed_skip_count = usize::MAX;
    }
    assert_rejected_transactionally(&mut runtime, &system, predecessor);
}

#[test]
fn private_active_owner_helper_selects_g_f_from_cp334_and_l_from_cp344() {
    for (demand, capacity, expected_route) in [
        (-1_000.0, false, "g"),
        (-1_000.0, true, "f"),
        (-100_000.0, true, "l"),
    ] {
        let completed = completed_cp349_case(demand, 1.0, capacity);
        assert!(completed.is_some());
        let Some((runtime, system, predecessor)) = completed else {
            return;
        };
        let private = private_active_predecessor(predecessor, &runtime);
        assert!(private.is_some());
        let Some(private) = private else {
            return;
        };
        let Some(unit) = runtime.units.get(&system.id) else {
            return;
        };
        let input = active_input_from_retained_owners_for_test(&runtime, unit, &system, private);
        assert!(input.is_some(), "{expected_route}");
        let Some(input) = input else {
            return;
        };
        let expected_supply = if expected_route == "l" {
            unit.calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit
                .latest
                .and_then(|snapshot| snapshot.resulting_supply_temperature_c)
        } else {
            unit.calc_cooling_positive_supply_temperature_mixed_air_limit
                .latest
                .and_then(|snapshot| snapshot.assigned_supply_temperature_c)
        };
        assert_eq!(
            Some(input.supply_temperature_c.to_bits()),
            expected_supply.map(f64::to_bits)
        );
        let mut state = State::new(system.id);
        let snapshot = advance(&mut state, private, Some(input));
        assert!(snapshot.is_some());
        assert_eq!(
            state
                .dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_count,
            1
        );
        assert_eq!(state.source_site_execution_count, 8);
    }
}

#[test]
fn wrong_owner_witnesses_and_provenance_are_rejected() {
    let completed = completed_cp349_case(-1_000.0, 1.0, false);
    assert!(completed.is_some());
    let Some((mut runtime, system, predecessor)) = completed else {
        return;
    };
    let private = private_active_predecessor(predecessor, &runtime);
    assert!(private.is_some());
    let Some(private) = private else {
        return;
    };
    let flow_witness = runtime.cooling_supply_mass_flow_positive_guard_latest_witness(system.id);
    assert!(flow_witness.is_some());
    let Some(mut flow_witness) = flow_witness else {
        return;
    };
    flow_witness.supply_mass_flow_rate_kg_per_s = flow_witness
        .supply_mass_flow_rate_kg_per_s
        .map(|value| f64::from_bits(value.to_bits() ^ 1));
    runtime.set_cooling_supply_mass_flow_positive_guard_latest_witness(system.id, flow_witness);
    let Some(unit) = runtime.units.get(&system.id) else {
        return;
    };
    assert!(active_input_from_retained_owners_for_test(&runtime, unit, &system, private).is_none());

    let completed = completed_cp349_case(-100_000.0, 1.0, true);
    assert!(completed.is_some());
    let Some((mut runtime, system, predecessor)) = completed else {
        return;
    };
    let private = private_active_predecessor(predecessor, &runtime);
    assert!(private.is_some());
    let Some(private) = private else {
        return;
    };
    if let Some(unit) = runtime.units.get_mut(&system.id)
        && let Some(provenance) = unit
            .calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment
            .latest
            .as_mut()
    {
        provenance.capacity_limit_guard_false_fallthrough_skipped = true;
    }
    let Some(unit) = runtime.units.get(&system.id) else {
        return;
    };
    assert!(active_input_from_retained_owners_for_test(&runtime, unit, &system, private).is_none());
}

#[test]
fn coordinated_owner_and_cp349_corruption_is_rejected() {
    let completed = completed_cp349_case(-100_000.0, 1.0, true);
    assert!(completed.is_some());
    let Some((mut runtime, system, predecessor)) = completed else {
        return;
    };
    let private = private_active_predecessor(predecessor, &runtime);
    assert!(private.is_some());
    let Some(private) = private else {
        return;
    };
    let Some(unit) = runtime.units.get(&system.id) else {
        return;
    };
    assert!(active_input_from_retained_owners_for_test(&runtime, unit, &system, private).is_some());
    let forged = {
        let Some(unit) = runtime.units.get_mut(&system.id) else {
            return;
        };
        let Some(latest) = unit
            .calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit
            .latest
            .as_mut()
        else {
            return;
        };
        latest.resulting_supply_temperature_c = latest
            .resulting_supply_temperature_c
            .map(|value| f64::from_bits(value.to_bits() ^ 1));
        *latest
    };
    runtime
        .set_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_latest_witness(
            system.id,
            forged,
        );
    let Some(unit) = runtime.units.get(&system.id) else {
        return;
    };
    assert!(active_input_from_retained_owners_for_test(&runtime, unit, &system, private).is_none());

    let completed = completed_cp349_case(-100_000.0, 1.0, true);
    assert!(completed.is_some());
    let Some((mut runtime, system, predecessor)) = completed else {
        return;
    };
    let private = private_active_predecessor(predecessor, &runtime);
    assert!(private.is_some());
    let Some(private) = private else {
        return;
    };
    let Some(unit) = runtime.units.get(&system.id) else {
        return;
    };
    assert!(active_input_from_retained_owners_for_test(&runtime, unit, &system, private).is_some());
    let forged = {
        let Some(unit) = runtime.units.get_mut(&system.id) else {
            return;
        };
        let Some(latest) = unit
            .calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment
            .latest
            .as_mut()
        else {
            return;
        };
        latest.capacity_limit_guard_false_fallthrough_skipped = true;
        latest.capacity_limit_sensible_output_guard_false_fallthrough = false;
        latest.capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed = false;
        *latest
    };
    runtime
        .set_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_latest_witness(
            system.id,
            forged,
        );
    let Some(unit) = runtime.units.get(&system.id) else {
        return;
    };
    assert!(active_input_from_retained_owners_for_test(&runtime, unit, &system, private).is_none());

    let completed = completed_cp349_case(-100_000.0, 1.0, true);
    assert!(completed.is_some());
    let Some((mut runtime, system, predecessor)) = completed else {
        return;
    };
    let private = private_active_predecessor(predecessor, &runtime);
    assert!(private.is_some());
    let Some(private) = private else {
        return;
    };
    let Some(unit) = runtime.units.get(&system.id) else {
        return;
    };
    assert!(active_input_from_retained_owners_for_test(&runtime, unit, &system, private).is_some());
    let forged = {
        let Some(unit) = runtime.units.get_mut(&system.id) else {
            return;
        };
        let Some(latest) = unit
            .calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment
            .latest
            .as_mut()
        else {
            return;
        };
        latest.cp_air_j_per_kg_k = Some(1006.0);
        *latest
    };
    runtime
        .set_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_latest_witness(
            system.id,
            forged,
        );
    let Some(unit) = runtime.units.get(&system.id) else {
        return;
    };
    assert!(active_input_from_retained_owners_for_test(&runtime, unit, &system, private).is_none());
}

#[test]
fn every_active_counter_overflow_rejects_before_mutation() {
    let pending = State::new(ep_model::IdealLoadsAirSystemId(7));
    assert!(next_transition_fits_for_test(
        &pending,
        predecessor(Q, 1),
        Some(active_input(1.0, 25.0, 15.0)),
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
                    Some(active_input(1.0, 25.0, 15.0)),
                )
                .is_none()
            );
            assert_eq!(state, before);
        }};
    }
    reject_overflow!(transition_count);
    reject_overflow!(source_site_execution_count);
    reject_overflow!(
        dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_count
    );
    reject_overflow!(supply_mass_flow_rate_read_count);
    reject_overflow!(cp_air_read_count);
    reject_overflow!(supply_mass_flow_rate_times_cp_air_calculation_count);
    reject_overflow!(mixed_air_temperature_read_count);
    reject_overflow!(supply_temperature_read_count);
    reject_overflow!(mixed_air_minus_supply_temperature_calculation_count);
    reject_overflow!(cooling_sensible_output_calculation_count);
    reject_overflow!(cooling_sensible_output_assignment_write_count);
}
