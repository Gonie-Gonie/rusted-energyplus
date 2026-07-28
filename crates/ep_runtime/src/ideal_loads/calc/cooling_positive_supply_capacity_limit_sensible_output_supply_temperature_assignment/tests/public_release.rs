use crate::ideal_loads::calc::cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment::tests::release_fixture::{
    completed_cp342_case, completed_cp342_case_with_zone_temperature,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot,
    PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment,
};
use crate::psychrometrics::energyplus_psy_tdb_fn_h_w;

fn assert_rejected_transactionally(
    runtime: &mut PurchasedAirRuntimeState,
    system: &ep_model::IdealLoadsAirSystem,
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot,
) {
    let selected = predecessor.system;
    let before_state = runtime
        .units
        .get(&selected)
        .expect("known unit")
        .calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment
        .clone();
    let before_witness = runtime
        .cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_latest_witness(
            selected,
        );
    assert!(
        advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment(
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
            .calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment,
        before_state
    );
    assert_eq!(
        runtime
            .cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_latest_witness(
                selected,
            ),
        before_witness
    );
}

#[test]
fn public_true_route_uses_cp342_enthalpy_and_cp334_cp335_source_owners() {
    let (mut runtime, system, predecessor) = completed_cp342_case(-100_000.0, 1.0, true);
    assert!(predecessor.capacity_limit_sensible_output_supply_enthalpy_assignment_executed);
    let unit = runtime.units.get(&system.id).expect("known unit");
    let owner_temperature = unit
        .calc_cooling_positive_supply_temperature_mixed_air_limit
        .latest
        .and_then(|snapshot| snapshot.assigned_supply_temperature_c)
        .expect("CP334 temperature");
    let owner_humidity = unit
        .calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment
        .latest
        .and_then(|snapshot| snapshot.assigned_supply_humidity_ratio)
        .expect("CP335 humidity");
    let enthalpy = predecessor
        .resulting_supply_enthalpy_j_per_kg
        .expect("CP342 enthalpy");
    let expected = energyplus_psy_tdb_fn_h_w(enthalpy, owner_humidity);

    let snapshot =
        advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment(
            &mut runtime,
            &system,
            predecessor,
        )
        .expect("CP343");
    assert_eq!(
        snapshot.preexisting_supply_temperature_c.map(f64::to_bits),
        Some(owner_temperature.to_bits())
    );
    assert_eq!(
        snapshot.supply_enthalpy_j_per_kg.map(f64::to_bits),
        Some(enthalpy.to_bits())
    );
    assert_eq!(
        snapshot.supply_humidity_ratio.map(f64::to_bits),
        Some(owner_humidity.to_bits())
    );
    assert_eq!(
        snapshot
            .psychrometric_supply_temperature_result_c
            .map(f64::to_bits),
        Some(expected.to_bits())
    );
    assert_eq!(
        snapshot.assigned_supply_temperature_c.map(f64::to_bits),
        Some(expected.to_bits())
    );
    assert_eq!(
        snapshot.resulting_supply_temperature_c.map(f64::to_bits),
        Some(expected.to_bits())
    );

    assert_rejected_transactionally(&mut runtime, &system, predecessor);
}

#[test]
fn public_sensible_guard_false_preserves_cp334_temperature_without_sites() {
    let (mut runtime, system, predecessor) = completed_cp342_case(-1_000.0, 1.0, true);
    assert!(predecessor.capacity_limit_sensible_output_guard_false_fallthrough);
    let owner_temperature = runtime
        .units
        .get(&system.id)
        .expect("known unit")
        .calc_cooling_positive_supply_temperature_mixed_air_limit
        .latest
        .and_then(|snapshot| snapshot.assigned_supply_temperature_c)
        .expect("CP334 temperature");
    let snapshot =
        advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment(
            &mut runtime,
            &system,
            predecessor,
        )
        .expect("CP343 false route");
    assert_eq!(
        snapshot.preexisting_supply_temperature_c.map(f64::to_bits),
        Some(owner_temperature.to_bits())
    );
    assert_eq!(
        snapshot.resulting_supply_temperature_c.map(f64::to_bits),
        Some(owner_temperature.to_bits())
    );
    assert!(!snapshot.supply_enthalpy_for_dry_bulb_inversion_read);
    assert!(snapshot.supply_enthalpy_j_per_kg.is_none());
    assert!(!snapshot.supply_humidity_ratio_for_dry_bulb_inversion_read);
    assert!(snapshot.supply_humidity_ratio.is_none());
    assert!(!snapshot.psychrometric_supply_temperature_evaluated);
    assert!(!snapshot.supply_temperature_assigned);
    assert_eq!(
        runtime
            .units
            .get(&system.id)
            .expect("known unit")
            .calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment
            .source_site_execution_count,
        0
    );
}

#[test]
fn all_four_inherited_skips_have_no_cp343_values() {
    for (demand, availability, capacity) in [
        (-1_000.0, 0.0, true),
        (1.0, 1.0, true),
        (-1.0e-40, 1.0, true),
        (-1_000.0, 1.0, false),
    ] {
        let (mut runtime, system, predecessor) =
            completed_cp342_case(demand, availability, capacity);
        let snapshot =
            advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment(
                &mut runtime,
                &system,
                predecessor,
            )
            .expect("CP343 inherited skip");
        assert!(snapshot.preexisting_supply_temperature_c.is_none());
        assert!(snapshot.supply_enthalpy_j_per_kg.is_none());
        assert!(snapshot.supply_humidity_ratio.is_none());
        assert!(snapshot.psychrometric_supply_temperature_result_c.is_none());
        assert!(snapshot.assigned_supply_temperature_c.is_none());
        assert!(snapshot.resulting_supply_temperature_c.is_none());
    }
}

#[test]
fn public_zero_humidity_route_uses_source_one_e_minus_five_floor() {
    let (mut runtime, system, predecessor) =
        completed_cp342_case_with_zone_temperature(-100_000.0, 1.0, true, 0.0, 24.0);
    assert!(predecessor.capacity_limit_sensible_output_supply_enthalpy_assignment_executed);
    let enthalpy = predecessor
        .resulting_supply_enthalpy_j_per_kg
        .expect("CP342 enthalpy");
    let snapshot =
        advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment(
            &mut runtime,
            &system,
            predecessor,
        )
        .expect("CP343 zero humidity");
    assert_eq!(snapshot.supply_humidity_ratio, Some(0.0));
    assert_eq!(
        snapshot.resulting_supply_temperature_c.map(f64::to_bits),
        Some(energyplus_psy_tdb_fn_h_w(enthalpy, 1.0e-5).to_bits())
    );
}

#[test]
fn predecessor_owner_and_corroborating_witness_drift_are_transactional() {
    let (runtime, system, predecessor) = completed_cp342_case(-100_000.0, 1.0, true);

    let mut supplied = runtime.clone();
    let mut forged = predecessor;
    forged.source = "forged-cp342";
    assert_rejected_transactionally(&mut supplied, &system, forged);

    let mut cp342_public = runtime.clone();
    cp342_public
        .units
        .get_mut(&system.id)
        .expect("known unit")
        .calc_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment
        .latest
        .as_mut()
        .expect("CP342 latest")
        .source = "forged-cp342-public";
    assert_rejected_transactionally(&mut cp342_public, &system, predecessor);

    let mut cp342_private = runtime.clone();
    let mut witness = cp342_private
        .cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_latest_witness(
            system.id,
        )
        .expect("CP342 witness");
    witness.resulting_supply_enthalpy_j_per_kg =
        witness.resulting_supply_enthalpy_j_per_kg.map(next_bits);
    cp342_private
        .set_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_latest_witness(
            system.id,
            witness,
        );
    assert_rejected_transactionally(&mut cp342_private, &system, predecessor);

    let mut cp334_public = runtime.clone();
    let latest = cp334_public
        .units
        .get_mut(&system.id)
        .expect("known unit")
        .calc_cooling_positive_supply_temperature_mixed_air_limit
        .latest
        .as_mut()
        .expect("CP334 latest");
    latest.assigned_supply_temperature_c = latest.assigned_supply_temperature_c.map(next_bits);
    assert_rejected_transactionally(&mut cp334_public, &system, predecessor);

    let mut cp334_private = runtime.clone();
    let mut witness = cp334_private
        .cooling_positive_supply_temperature_mixed_air_limit_latest_witness(system.id)
        .expect("CP334 witness");
    witness.assigned_supply_temperature_c = witness.assigned_supply_temperature_c.map(next_bits);
    cp334_private
        .set_cooling_positive_supply_temperature_mixed_air_limit_latest_witness(system.id, witness);
    assert_rejected_transactionally(&mut cp334_private, &system, predecessor);

    let mut cp335_public = runtime.clone();
    let latest = cp335_public
        .units
        .get_mut(&system.id)
        .expect("known unit")
        .calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment
        .latest
        .as_mut()
        .expect("CP335 latest");
    latest.assigned_supply_humidity_ratio = latest.assigned_supply_humidity_ratio.map(next_bits);
    assert_rejected_transactionally(&mut cp335_public, &system, predecessor);

    let mut cp335_private = runtime.clone();
    let mut witness = cp335_private
        .cooling_positive_supply_humidity_ratio_mixed_air_assignment_latest_witness(system.id)
        .expect("CP335 witness");
    witness.assigned_supply_humidity_ratio = witness.assigned_supply_humidity_ratio.map(next_bits);
    cp335_private.set_cooling_positive_supply_humidity_ratio_mixed_air_assignment_latest_witness(
        system.id, witness,
    );
    assert_rejected_transactionally(&mut cp335_private, &system, predecessor);

    let mut cp336_public = runtime.clone();
    let latest = cp336_public
        .units
        .get_mut(&system.id)
        .expect("known unit")
        .calc_cooling_positive_supply_enthalpy_assignment
        .latest
        .as_mut()
        .expect("CP336 latest");
    latest.supply_temperature_c = latest.supply_temperature_c.map(next_bits);
    assert_rejected_transactionally(&mut cp336_public, &system, predecessor);

    let mut cp336_private = runtime;
    let mut witness = cp336_private
        .cooling_positive_supply_enthalpy_assignment_latest_witness(system.id)
        .expect("CP336 witness");
    witness.supply_humidity_ratio = witness.supply_humidity_ratio.map(next_bits);
    cp336_private
        .set_cooling_positive_supply_enthalpy_assignment_latest_witness(system.id, witness);
    assert_rejected_transactionally(&mut cp336_private, &system, predecessor);
}

#[test]
fn assignment_counter_overflow_is_preflighted_transactionally() {
    for counter in 0..8 {
        let (mut runtime, system, predecessor) = completed_cp342_case(-100_000.0, 1.0, true);
        let state = &mut runtime
            .units
            .get_mut(&system.id)
            .expect("known unit")
            .calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment;
        match counter {
            0 => state.transition_count = usize::MAX,
            1 => {
                state.capacity_limit_sensible_output_supply_temperature_assignment_count =
                    usize::MAX
            }
            2 => state.source_site_execution_count = usize::MAX - 3,
            3 => state.supply_enthalpy_for_dry_bulb_inversion_read_count = usize::MAX,
            4 => state.supply_humidity_ratio_for_dry_bulb_inversion_read_count = usize::MAX,
            5 => state.psychrometric_supply_temperature_evaluation_count = usize::MAX,
            6 => state.supply_temperature_assignment_write_count = usize::MAX,
            7 => {
                state.witnessed_capacity_limit_sensible_output_supply_temperature_assignment_count =
                    usize::MAX
            }
            _ => unreachable!(),
        }
        let unit = runtime.units.get(&system.id).expect("known unit");
        assert!(
            !super::super::release::next_supply_temperature_assignment_transition_fits_for_test(
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
            completed_cp342_case(demand, availability, capacity_limit);
        let state = &mut runtime
            .units
            .get_mut(&system.id)
            .expect("known unit")
            .calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment;
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
            !super::super::release::next_supply_temperature_assignment_transition_fits_for_test(
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
