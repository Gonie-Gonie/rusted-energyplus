use crate::ideal_loads::calc::cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment::tests::release_fixture::{
    completed_cp341_case, completed_cp341_case_with_zone_temperature,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot,
    PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment,
};

fn assert_rejected_transactionally(
    runtime: &mut PurchasedAirRuntimeState,
    system: &ep_model::IdealLoadsAirSystem,
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot,
) {
    let selected = predecessor.system;
    let before_state = runtime
        .units
        .get(&selected)
        .expect("known unit")
        .calc_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment
        .clone();
    let before_witness = runtime
        .cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_latest_witness(
            selected,
        );
    assert!(
        advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment(
            runtime,
            system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(
        runtime
            .units
            .get(&selected)
            .expect("known unit")
            .calc_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment,
        before_state
    );
    assert_eq!(
        runtime
            .cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_latest_witness(
                selected,
            ),
        before_witness
    );
}

#[test]
fn public_false_route_preserves_retained_cp339_supply_enthalpy_without_sites() {
    let (mut runtime, system, predecessor) = completed_cp341_case(-1_000.0, 1.0, true);
    assert!(predecessor.capacity_limit_sensible_output_guard_false_fallthrough);
    let retained = runtime
        .units
        .get(&system.id)
        .expect("known unit")
        .calc_cooling_positive_supply_capacity_limit_sensible_output_assignment
        .latest
        .and_then(|snapshot| snapshot.supply_enthalpy_j_per_kg)
        .expect("CP339 supply enthalpy");
    let snapshot =
        advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment(
            &mut runtime,
            &system,
            predecessor,
        )
        .expect("CP342 false route");
    assert_eq!(
        snapshot
            .preexisting_supply_enthalpy_j_per_kg
            .map(f64::to_bits),
        Some(retained.to_bits())
    );
    assert_eq!(
        snapshot
            .resulting_supply_enthalpy_j_per_kg
            .map(f64::to_bits),
        Some(retained.to_bits())
    );
    assert!(!snapshot.mixed_air_enthalpy_read);
    assert!(!snapshot.cooling_sensible_output_read);
    assert!(!snapshot.supply_mass_flow_rate_read);
    assert!(!snapshot.supply_enthalpy_assigned);
}

#[test]
fn public_true_route_uses_only_retained_cp339_and_cp341_operands() {
    let (mut runtime, system, predecessor) = completed_cp341_case(-100_000.0, 1.0, true);
    assert!(predecessor.capacity_limit_sensible_output_maximum_capacity_assignment_executed);
    let cp339 = runtime
        .units
        .get(&system.id)
        .expect("known unit")
        .calc_cooling_positive_supply_capacity_limit_sensible_output_assignment
        .latest
        .expect("CP339");
    let mixed = cp339.mixed_air_enthalpy_j_per_kg.expect("mixed");
    let flow = cp339.supply_mass_flow_rate_kg_per_s.expect("flow");
    let output = predecessor
        .resulting_cooling_sensible_output_w
        .expect("CP341 result");
    let expected_quotient = output / flow;
    let expected = mixed - expected_quotient;
    let snapshot =
        advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment(
            &mut runtime,
            &system,
            predecessor,
        )
        .expect("CP342");
    assert_eq!(
        snapshot.specific_cooling_output_j_per_kg.map(f64::to_bits),
        Some(expected_quotient.to_bits())
    );
    assert_eq!(
        snapshot
            .resulting_supply_enthalpy_j_per_kg
            .map(f64::to_bits),
        Some(expected.to_bits())
    );
    assert_rejected_transactionally(&mut runtime, &system, predecessor);
}

#[test]
fn full_public_cp339_nan_chain_skips_arithmetic_and_preserves_supply_enthalpy() {
    let (mut runtime, system, predecessor) =
        completed_cp341_case_with_zone_temperature(-f64::MAX, 1.0, true, 0.008, 13.000_02);
    assert!(
        predecessor
            .resulting_cooling_sensible_output_w
            .is_some_and(f64::is_nan)
    );
    assert!(predecessor.capacity_limit_sensible_output_guard_false_fallthrough);
    let retained = runtime
        .units
        .get(&system.id)
        .expect("known unit")
        .calc_cooling_positive_supply_capacity_limit_sensible_output_assignment
        .latest
        .and_then(|snapshot| snapshot.supply_enthalpy_j_per_kg)
        .expect("CP339 supply enthalpy");
    let snapshot =
        advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment(
            &mut runtime,
            &system,
            predecessor,
        )
        .expect("CP342 NaN false route");
    assert_eq!(
        snapshot
            .preexisting_supply_enthalpy_j_per_kg
            .map(f64::to_bits),
        Some(retained.to_bits())
    );
    assert_eq!(
        snapshot
            .resulting_supply_enthalpy_j_per_kg
            .map(f64::to_bits),
        Some(retained.to_bits())
    );
    assert!(snapshot.cooling_sensible_output_w.is_none());
    assert!(snapshot.specific_cooling_output_j_per_kg.is_none());
}

#[test]
fn full_public_cp339_positive_infinity_chain_uses_cp341_finite_maximum() {
    let (mut runtime, system, predecessor) = completed_cp341_case(-f64::MAX, 1.0, true);
    let cp339 = runtime
        .units
        .get(&system.id)
        .expect("known unit")
        .calc_cooling_positive_supply_capacity_limit_sensible_output_assignment
        .latest
        .expect("CP339");
    assert_eq!(cp339.cooling_sensible_output_w, Some(f64::INFINITY));
    assert!(
        cp339
            .supply_mass_flow_rate_kg_per_s
            .is_some_and(|flow| flow.is_finite() && flow > 0.0)
    );
    let retained_maximum = predecessor
        .resulting_cooling_sensible_output_w
        .expect("CP341 finite maximum");
    assert!(retained_maximum.is_finite() && retained_maximum > 0.0);
    let flow = cp339
        .supply_mass_flow_rate_kg_per_s
        .expect("CP339 finite positive flow");
    let mixed_air = cp339
        .mixed_air_enthalpy_j_per_kg
        .expect("CP339 mixed-air enthalpy");
    let expected_quotient = retained_maximum / flow;
    let expected_supply_enthalpy = mixed_air - expected_quotient;
    let snapshot =
        advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment(
            &mut runtime,
            &system,
            predecessor,
        )
        .expect("CP342 finite-maximum assignment route");
    assert_eq!(
        snapshot.cooling_sensible_output_w.map(f64::to_bits),
        Some(retained_maximum.to_bits())
    );
    assert_eq!(snapshot.supply_mass_flow_rate_kg_per_s, Some(flow));
    assert_eq!(
        snapshot.specific_cooling_output_j_per_kg.map(f64::to_bits),
        Some(expected_quotient.to_bits())
    );
    assert_eq!(
        snapshot
            .resulting_supply_enthalpy_j_per_kg
            .map(f64::to_bits),
        Some(expected_supply_enthalpy.to_bits())
    );
}

#[test]
fn all_four_inherited_skips_have_no_cp342_values() {
    for (demand, availability, capacity) in [
        (-1_000.0, 0.0, true),
        (1.0, 1.0, true),
        (-1.0e-40, 1.0, true),
        (-1_000.0, 1.0, false),
    ] {
        let (mut runtime, system, predecessor) =
            completed_cp341_case(demand, availability, capacity);
        let snapshot =
            advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment(
                &mut runtime,
                &system,
                predecessor,
            )
            .expect("CP342 inherited skip");
        assert!(snapshot.preexisting_supply_enthalpy_j_per_kg.is_none());
        assert!(snapshot.mixed_air_enthalpy_j_per_kg.is_none());
        assert!(snapshot.cooling_sensible_output_w.is_none());
        assert!(snapshot.supply_mass_flow_rate_kg_per_s.is_none());
        assert!(snapshot.resulting_supply_enthalpy_j_per_kg.is_none());
    }
}

#[test]
fn supplied_and_retained_cp341_and_cp339_drift_are_transactional() {
    let (runtime, system, predecessor) = completed_cp341_case(-100_000.0, 1.0, true);
    let mut supplied = runtime.clone();
    let mut forged = predecessor;
    forged.source = "forged-cp341";
    assert_rejected_transactionally(&mut supplied, &system, forged);

    let mut cp341_public = runtime.clone();
    cp341_public
        .units
        .get_mut(&system.id)
        .expect("known unit")
        .calc_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment
        .latest
        .as_mut()
        .expect("CP341 latest")
        .source = "forged-cp341-public";
    assert_rejected_transactionally(&mut cp341_public, &system, predecessor);

    let mut cp341_private = runtime.clone();
    let mut witness = cp341_private
        .cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_latest_witness(
            system.id,
        )
        .expect("CP341 witness");
    witness.resulting_cooling_sensible_output_w = Some(123.0);
    cp341_private
        .set_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_latest_witness(
            system.id,
            witness,
        );
    assert_rejected_transactionally(&mut cp341_private, &system, predecessor);

    let mut cp339_public = runtime.clone();
    cp339_public
        .units
        .get_mut(&system.id)
        .expect("known unit")
        .calc_cooling_positive_supply_capacity_limit_sensible_output_assignment
        .latest
        .as_mut()
        .expect("CP339 latest")
        .source = "forged-cp339-public";
    assert_rejected_transactionally(&mut cp339_public, &system, predecessor);

    let mut cp339_private = runtime;
    let mut witness = cp339_private
        .cooling_positive_supply_capacity_limit_sensible_output_assignment_latest_witness(system.id)
        .expect("CP339 witness");
    witness.supply_mass_flow_rate_kg_per_s = Some(123.0);
    cp339_private
        .set_cooling_positive_supply_capacity_limit_sensible_output_assignment_latest_witness(
            system.id, witness,
        );
    assert_rejected_transactionally(&mut cp339_private, &system, predecessor);
}

#[test]
fn assignment_counter_overflow_is_preflighted_transactionally() {
    for counter in 0..10 {
        let (mut runtime, system, predecessor) = completed_cp341_case(-100_000.0, 1.0, true);
        let state = &mut runtime
            .units
            .get_mut(&system.id)
            .expect("known unit")
            .calc_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment;
        match counter {
            0 => state.transition_count = usize::MAX,
            1 => state.capacity_limit_sensible_output_supply_enthalpy_assignment_count = usize::MAX,
            2 => state.source_site_execution_count = usize::MAX - 5,
            3 => state.mixed_air_enthalpy_read_count = usize::MAX,
            4 => state.cooling_sensible_output_read_count = usize::MAX,
            5 => state.supply_mass_flow_rate_read_count = usize::MAX,
            6 => state.specific_cooling_output_calculation_count = usize::MAX,
            7 => state.supply_enthalpy_calculation_count = usize::MAX,
            8 => state.supply_enthalpy_assignment_write_count = usize::MAX,
            9 => {
                state.witnessed_capacity_limit_sensible_output_supply_enthalpy_assignment_count =
                    usize::MAX
            }
            _ => unreachable!(),
        }
        let unit = runtime.units.get(&system.id).expect("known unit");
        assert!(
            !super::super::release::next_supply_enthalpy_assignment_transition_fits_for_test(
                unit,
                predecessor,
            )
        );
        assert_rejected_transactionally(&mut runtime, &system, predecessor);
    }
}

#[test]
fn every_nonassignment_route_counter_increment_is_preflighted_transactionally() {
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
            completed_cp341_case(demand, availability, capacity_limit);
        let state = &mut runtime
            .units
            .get_mut(&system.id)
            .expect("known unit")
            .calc_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment;
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
            !super::super::release::next_supply_enthalpy_assignment_transition_fits_for_test(
                unit,
                predecessor,
            )
        );
        assert_rejected_transactionally(&mut runtime, &system, predecessor);
    }
}
