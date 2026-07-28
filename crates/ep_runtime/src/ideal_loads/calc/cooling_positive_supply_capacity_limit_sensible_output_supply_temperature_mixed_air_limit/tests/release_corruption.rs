use super::public_release::{
    assert_rejected_transactionally, completed_cp343_case,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitError,
    advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit,
    cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_snapshot_is_exact_direct_release,
};

#[test]
fn snapshot_validation_has_no_new_left_or_result_finite_gate() {
    for (left, right, expected) in [
        (f64::from_bits(0x7ff8_0000_0000_0344), 22.0, 22.0),
        (f64::INFINITY, 22.0, 22.0),
        (f64::NEG_INFINITY, 22.0, f64::NEG_INFINITY),
    ] {
        let mut state =
            super::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitRuntimeState::new(
                ep_model::IdealLoadsAirSystemId(3),
            );
        let snapshot = super::advance(&mut state, super::Route::Limited, 1, left, right);
        assert_eq!(
            snapshot.resulting_supply_temperature_c.map(f64::to_bits),
            Some(expected.to_bits())
        );
        assert!(
            cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_snapshot_is_exact_direct_release(
                snapshot,
            )
        );
    }
}

#[test]
fn direct_snapshot_validation_rejects_nonfinite_right_and_value_or_provenance_drift() {
    let mut state =
        super::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitRuntimeState::new(
            ep_model::IdealLoadsAirSystemId(3),
        );
    let snapshot = super::advance(&mut state, super::Route::Limited, 1, 24.0, 22.0);
    assert!(
        cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_snapshot_is_exact_direct_release(
            snapshot,
        )
    );

    let mut forged = snapshot;
    forged.source = "forged-cp344";
    assert!(
        !cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_snapshot_is_exact_direct_release(
            forged,
        )
    );

    let mut forged = snapshot;
    forged.supply_temperature_before_mixed_air_limit_c = Some(24.0_f64.next_up());
    assert!(
        !cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_snapshot_is_exact_direct_release(
            forged,
        )
    );

    let mut forged = snapshot;
    forged.minimum_supply_temperature_c = Some(22.0_f64.next_up());
    assert!(
        !cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_snapshot_is_exact_direct_release(
            forged,
        )
    );

    let mut nonfinite_state =
        super::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitRuntimeState::new(
            ep_model::IdealLoadsAirSystemId(3),
        );
    let right_nan = f64::from_bits(0x7ff8_0000_0000_0444);
    let nonfinite =
        super::advance(&mut nonfinite_state, super::Route::Limited, 1, 24.0, right_nan);
    assert_eq!(
        nonfinite
            .resulting_supply_temperature_c
            .expect("pure result")
            .to_bits(),
        right_nan.to_bits()
    );
    assert!(
        !cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_snapshot_is_exact_direct_release(
            nonfinite,
        )
    );
}

#[test]
fn supplied_public_and_private_cp343_drift_is_rejected_transactionally() {
    let (runtime, system, predecessor) = completed_cp343_case(-100_000.0, 1.0, true);

    let mut supplied = runtime.clone();
    let mut forged = predecessor;
    forged.resulting_supply_temperature_c =
        forged.resulting_supply_temperature_c.map(next_bits);
    assert_rejected_transactionally(&mut supplied, &system, forged);

    let mut public = runtime.clone();
    public
        .units
        .get_mut(&system.id)
        .expect("known unit")
        .calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment
        .latest
        .as_mut()
        .expect("CP343 latest")
        .resulting_supply_temperature_c = predecessor
        .resulting_supply_temperature_c
        .map(next_bits);
    assert_rejected_transactionally(&mut public, &system, predecessor);

    let mut private = runtime;
    let mut witness = private
        .cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_latest_witness(
            system.id,
        )
        .expect("CP343 witness");
    witness.resulting_supply_temperature_c =
        witness.resulting_supply_temperature_c.map(next_bits);
    private
        .set_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_latest_witness(
            system.id,
            witness,
        );
    assert_rejected_transactionally(&mut private, &system, predecessor);
}

#[test]
fn cp329_public_and_private_owner_drift_is_rejected_transactionally() {
    let (runtime, system, predecessor) = completed_cp343_case(-100_000.0, 1.0, true);

    let mut public = runtime.clone();
    let latest = public
        .units
        .get_mut(&system.id)
        .expect("known unit")
        .calc_cooling_mixed_air_call
        .latest
        .as_mut()
        .expect("CP329 latest");
    latest.mixed_air_temperature_c = latest.mixed_air_temperature_c.map(next_bits);
    assert_rejected_transactionally(&mut public, &system, predecessor);

    let mut private = runtime;
    let mut witness = private
        .cooling_mixed_air_call_latest_witness(system.id)
        .expect("CP329 witness");
    witness.mixed_air_temperature_c = witness.mixed_air_temperature_c.map(next_bits);
    private.set_cooling_mixed_air_call_latest_witness(system.id, witness);
    assert_rejected_transactionally(&mut private, &system, predecessor);
}

#[test]
fn cp334_is_recursive_corroboration_not_a_substitute_owner() {
    let (mut runtime, system, predecessor) = completed_cp343_case(-100_000.0, 1.0, true);

    // Forge both CP334 copies together so their private/public parity remains
    // intact. CP344 still rejects because recursive CP343 completion checks
    // CP334's copied MixedAirTemp against the actual CP329 source owner.
    let forged_temperature = {
        let latest = runtime
            .units
            .get_mut(&system.id)
            .expect("known unit")
            .calc_cooling_positive_supply_temperature_mixed_air_limit
            .latest
            .as_mut()
            .expect("CP334 latest");
        let forged = latest
            .mixed_air_temperature_c
            .map(next_bits)
            .expect("CP334 mixed temperature");
        latest.mixed_air_temperature_c = Some(forged);
        forged
    };
    let mut witness = runtime
        .cooling_positive_supply_temperature_mixed_air_limit_latest_witness(system.id)
        .expect("CP334 witness");
    witness.mixed_air_temperature_c = Some(forged_temperature);
    runtime.set_cooling_positive_supply_temperature_mixed_air_limit_latest_witness(
        system.id,
        witness,
    );

    assert_rejected_transactionally(&mut runtime, &system, predecessor);
}

#[test]
fn nonfinite_cp329_right_owner_is_rejected_before_mutation() {
    let (mut runtime, system, predecessor) = completed_cp343_case(-100_000.0, 1.0, true);
    let right_nan = f64::from_bits(0x7ff8_0000_0000_0444);
    runtime
        .units
        .get_mut(&system.id)
        .expect("known unit")
        .calc_cooling_mixed_air_call
        .latest
        .as_mut()
        .expect("CP329 latest")
        .mixed_air_temperature_c = Some(right_nan);
    let before = runtime
        .units
        .get(&system.id)
        .expect("known unit")
        .calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit
        .clone();
    let error =
        advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit(
            &mut runtime,
            &system,
            predecessor,
        )
        .expect_err("nonfinite CP329 owner must fail");
    assert_eq!(
        error,
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitError::InvalidMixedAirTemperature {
            system: system.id,
        }
    );
    assert_eq!(
        runtime
            .units
            .get(&system.id)
            .expect("known unit")
            .calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit,
        before
    );
}

#[test]
fn every_limit_counter_increment_is_preflighted_transactionally() {
    for counter in 0..8 {
        let (mut runtime, system, predecessor) =
            completed_cp343_case(-100_000.0, 1.0, true);
        let state = &mut runtime
            .units
            .get_mut(&system.id)
            .expect("known unit")
            .calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit;
        match counter {
            0 => state.transition_count = usize::MAX,
            1 => {
                state.capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count =
                    usize::MAX
            }
            2 => state.source_site_execution_count = usize::MAX - 3,
            3 => state.supply_temperature_for_minimum_read_count = usize::MAX,
            4 => state.mixed_air_temperature_for_minimum_read_count = usize::MAX,
            5 => state.source_shaped_two_argument_minimum_evaluation_count = usize::MAX,
            6 => state.supply_temperature_assignment_write_count = usize::MAX,
            7 => {
                state
                    .witnessed_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count =
                    usize::MAX
            }
            _ => unreachable!(),
        }
        let unit = runtime.units.get(&system.id).expect("known unit");
        assert!(
            !super::super::release::next_supply_temperature_mixed_air_limit_transition_fits_for_test(
                unit,
                predecessor,
            )
        );
        assert_rejected_transactionally(&mut runtime, &system, predecessor);
    }
}

#[test]
fn every_nonlimit_route_counter_increment_is_preflighted_transactionally() {
    for (demand, availability, capacity_limit, counter) in [
        (-1_000.0, 0.0, true, 0),
        (1.0, 1.0, true, 1),
        (-1.0e-40, 1.0, true, 2),
        (-1.0e-40, 1.0, true, 3),
        (-1_000.0, 1.0, false, 4),
        (-1_000.0, 1.0, false, 5),
        (-1_000.0, 1.0, true, 6),
        (-1_000.0, 1.0, true, 7),
    ] {
        let (mut runtime, system, predecessor) =
            completed_cp343_case(demand, availability, capacity_limit);
        let state = &mut runtime
            .units
            .get_mut(&system.id)
            .expect("known unit")
            .calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit;
        match counter {
            0 => state.unit_off_skip_count = usize::MAX,
            1 => state.non_cooling_skip_count = usize::MAX,
            2 => state.positive_guard_false_fallthrough_skip_count = usize::MAX,
            3 => state.witnessed_positive_guard_false_fallthrough_skip_count = usize::MAX,
            4 => state.capacity_limit_guard_false_fallthrough_skip_count = usize::MAX,
            5 => state.witnessed_capacity_limit_guard_false_fallthrough_skip_count = usize::MAX,
            6 => state.capacity_limit_sensible_output_guard_false_fallthrough_count = usize::MAX,
            7 => {
                state.witnessed_capacity_limit_sensible_output_guard_false_fallthrough_count =
                    usize::MAX
            }
            _ => unreachable!(),
        }
        let unit = runtime.units.get(&system.id).expect("known unit");
        assert!(
            !super::super::release::next_supply_temperature_mixed_air_limit_transition_fits_for_test(
                unit,
                predecessor,
            )
        );
        assert_rejected_transactionally(&mut runtime, &system, predecessor);
    }
}

fn next_bits(value: f64) -> f64 {
    f64::from_bits(value.to_bits().wrapping_add(1))
}
