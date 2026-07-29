use super::super::{
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitRetainedRoute as Route,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitRuntimeState as State,
    advance_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_state as advance,
    release::{
        active_operands_from_retained_owners_for_test, next_transition_fits_for_test,
    },
};
use super::{
    active_operands, completed_cp352_case, predecessor, private_active_predecessor,
    retained_supply_temperature,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyEnthalpyAssignmentSnapshot as Predecessor,
    advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit,
};

const Q: Route =
    Route::DehumidificationControlConstantSensibleHeatRatioOverdryingLimitExecuted;

fn assert_rejected_transactionally(
    runtime: &mut crate::ideal_loads::PurchasedAirRuntimeState,
    system: &ep_model::IdealLoadsAirSystem,
    predecessor: Predecessor,
) {
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit(
            runtime,
            system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(&*runtime, &before);
}

#[test]
fn supplied_latest_witness_replay_and_overflow_reject_transactionally() {
    let completed = completed_cp352_case(-100_000.0, 1.0, true);
    assert!(completed.is_some());
    let Some((mut runtime, system, mut predecessor)) = completed else {
        return;
    };
    predecessor.source_order = &[];
    assert_rejected_transactionally(&mut runtime, &system, predecessor);

    let completed = completed_cp352_case(-1_000.0, 1.0, false);
    assert!(completed.is_some());
    let Some((mut runtime, system, predecessor)) = completed else {
        return;
    };
    let mut forged = predecessor;
    forged.dehumidification_control_none_case_completed_skip = false;
    runtime
        .set_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_latest_witness(
            system.id,
            forged,
        );
    assert_rejected_transactionally(&mut runtime, &system, predecessor);

    let completed = completed_cp352_case(-1_000.0, 1.0, false);
    assert!(completed.is_some());
    let Some((mut runtime, system, predecessor)) = completed else {
        return;
    };
    assert!(
        advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_ok()
    );
    assert_rejected_transactionally(&mut runtime, &system, predecessor);

    let completed = completed_cp352_case(-1_000.0, 1.0, false);
    assert!(completed.is_some());
    let Some((mut runtime, system, predecessor)) = completed else {
        return;
    };
    if let Some(unit) = runtime.units.get_mut(&system.id) {
        unit.calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit
            .dehumidification_control_none_case_completed_skip_count = usize::MAX;
    }
    assert_rejected_transactionally(&mut runtime, &system, predecessor);
}

#[test]
fn private_active_operands_resolve_cp352_and_selected_temperature_owner() {
    for capacity_limit in [false, true] {
        let completed = completed_cp352_case(-100_000.0, 1.0, capacity_limit);
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
        assert_eq!(
            operands
                .supply_enthalpy_before_overdrying_limit_j_per_kg
                .to_bits(),
            private
                .resulting_supply_enthalpy_j_per_kg
                .map(f64::to_bits)
                .unwrap_or_default()
        );
        assert_eq!(
            operands.supply_temperature_c.to_bits(),
            retained_supply_temperature(&runtime, system.id)
                .map(f64::to_bits)
                .unwrap_or_default()
        );
        let mut state = State::new(system.id);
        assert!(advance(&mut state, private, Some(operands)).is_some());
    }
}

#[test]
fn coordinated_private_cp352_and_provenance_owner_forgeries_are_rejected() {
    let completed = completed_cp352_case(-100_000.0, 1.0, false);
    assert!(completed.is_some());
    let Some((runtime, system, direct)) = completed else {
        return;
    };
    let private = private_active_predecessor(direct, &runtime, &system);
    assert!(private.is_some());
    let Some(mut forged_private) = private else {
        return;
    };
    forged_private.mixed_air_enthalpy_j_per_kg = forged_private
        .mixed_air_enthalpy_j_per_kg
        .map(|value| value + 1.0);
    for value in [
        &mut forged_private.calculated_supply_enthalpy_j_per_kg,
        &mut forged_private.assigned_supply_enthalpy_j_per_kg,
        &mut forged_private.resulting_supply_enthalpy_j_per_kg,
    ] {
        *value = value.map(|value| value + 1.0);
    }
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

    let completed = completed_cp352_case(-100_000.0, 1.0, false);
    assert!(completed.is_some());
    let Some((mut runtime, system, direct)) = completed else {
        return;
    };
    let private = private_active_predecessor(direct, &runtime, &system);
    assert!(private.is_some());
    let Some(private) = private else {
        return;
    };
    let forged_provenance = {
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
        latest.mixed_air_humidity_ratio = latest
            .mixed_air_humidity_ratio
            .map(|value| f64::from_bits(value.to_bits().wrapping_add(1)));
        *latest
    };
    runtime
        .set_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_latest_witness(
            system.id,
            forged_provenance,
        );
    let Some(unit) = runtime.units.get(&system.id) else {
        return;
    };
    assert!(
        active_operands_from_retained_owners_for_test(&runtime, unit, &system, private)
            .is_none()
    );

    let completed = completed_cp352_case(-100_000.0, 1.0, false);
    assert!(completed.is_some());
    let Some((mut runtime, system, direct)) = completed else {
        return;
    };
    let private = private_active_predecessor(direct, &runtime, &system);
    assert!(private.is_some());
    let Some(private) = private else {
        return;
    };
    let capacity_owner_selected = runtime
        .units
        .get(&system.id)
        .and_then(|unit| {
            unit.calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment
                .latest
        })
        .is_some_and(|snapshot| {
            snapshot
                .capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed
        });
    if capacity_owner_selected {
        let forged_owner = {
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
                .map(|value| f64::from_bits(value.to_bits().wrapping_add(1)));
            *latest
        };
        runtime
            .set_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_latest_witness(
                system.id,
                forged_owner,
            );
    } else {
        let forged_owner = {
            let Some(unit) = runtime.units.get_mut(&system.id) else {
                return;
            };
            let Some(latest) = unit
                .calc_cooling_positive_supply_temperature_mixed_air_limit
                .latest
                .as_mut()
            else {
                return;
            };
            latest.assigned_supply_temperature_c = latest
                .assigned_supply_temperature_c
                .map(|value| f64::from_bits(value.to_bits().wrapping_add(1)));
            *latest
        };
        runtime.set_cooling_positive_supply_temperature_mixed_air_limit_latest_witness(
            system.id,
            forged_owner,
        );
    }
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
        Some(active_operands(40_000.0, 12.0)),
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
                    Some(active_operands(40_000.0, 12.0)),
                )
                .is_none()
            );
            assert_eq!(state, before);
        }};
    }
    reject_overflow!(transition_count);
    reject_overflow!(source_site_execution_count);
    reject_overflow!(
        dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_count
    );
    reject_overflow!(supply_enthalpy_for_overdrying_limit_maximum_read_count);
    reject_overflow!(supply_temperature_for_minimum_humidity_ratio_enthalpy_read_count);
    reject_overflow!(psychrometric_minimum_supply_enthalpy_evaluation_count);
    reject_overflow!(source_shaped_two_argument_maximum_evaluation_count);
    reject_overflow!(supply_enthalpy_assignment_write_count);
    reject_overflow!(
        witnessed_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_count
    );
}
