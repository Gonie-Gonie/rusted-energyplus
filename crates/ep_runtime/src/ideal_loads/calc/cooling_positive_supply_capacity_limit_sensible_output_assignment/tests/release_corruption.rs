use super::release_fixture::{active_case, completed_cp338_case};
use super::super::*;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot,
    PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_assignment,
};

fn assert_rejected_transactionally(
    runtime: &mut PurchasedAirRuntimeState,
    system: &ep_model::IdealLoadsAirSystem,
    predecessor: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot,
) {
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_assignment(
            runtime,
            system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(*runtime, before);
}

#[test]
fn public_active_release_uses_only_retained_cp330_cp329_and_cp336_operands() {
    let (mut runtime, system, predecessor) = active_case();
    assert!(predecessor.capacity_limit_cp_air_assignment_executed);
    let unit = runtime.units.get(&system.id).expect("known unit");
    let supply_mass_flow = unit
        .calc_cooling_supply_mass_flow_positive_guard
        .latest
        .and_then(|snapshot| snapshot.supply_mass_flow_rate_kg_per_s)
        .expect("CP330 flow");
    let mixed_air_enthalpy = unit
        .calc_cooling_mixed_air_call
        .latest
        .and_then(|snapshot| snapshot.mixed_air_enthalpy_projection_j_per_kg)
        .expect("CP329 enthalpy");
    let supply_enthalpy = unit
        .calc_cooling_positive_supply_enthalpy_assignment
        .latest
        .and_then(|snapshot| snapshot.supply_enthalpy_j_per_kg)
        .expect("CP336 enthalpy");
    let difference = mixed_air_enthalpy - supply_enthalpy;
    let expected = supply_mass_flow * difference;

    let snapshot =
        advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_assignment(
            &mut runtime,
            &system,
            predecessor,
        )
        .expect("CP339");
    assert!(
        cooling_positive_supply_capacity_limit_sensible_output_assignment_snapshot_is_exact_direct_release(
            snapshot,
        )
    );
    assert_eq!(
        snapshot.supply_mass_flow_rate_kg_per_s.map(f64::to_bits),
        Some(supply_mass_flow.to_bits())
    );
    assert_eq!(
        snapshot.mixed_air_enthalpy_j_per_kg.map(f64::to_bits),
        Some(mixed_air_enthalpy.to_bits())
    );
    assert_eq!(
        snapshot.supply_enthalpy_j_per_kg.map(f64::to_bits),
        Some(supply_enthalpy.to_bits())
    );
    assert_eq!(
        snapshot
            .mixed_air_minus_supply_enthalpy_j_per_kg
            .map(f64::to_bits),
        Some(difference.to_bits())
    );
    assert_eq!(
        snapshot.cooling_sensible_output_w.map(f64::to_bits),
        Some(expected.to_bits())
    );
    let state = &runtime
        .units
        .get(&system.id)
        .expect("known unit")
        .calc_cooling_positive_supply_capacity_limit_sensible_output_assignment;
    assert_eq!(state.transition_count, 1);
    assert_eq!(state.capacity_limit_sensible_output_assignment_count, 1);
    assert_eq!(state.source_site_execution_count, 6);
    assert!(
        completed_direct_cooling_positive_supply_capacity_limit_sensible_output_assignment_is_consistent(
            &runtime,
            runtime.units.get(&system.id).expect("known unit"),
            &system,
            snapshot,
            runtime
                .cooling_positive_supply_capacity_limit_sensible_output_assignment_latest_witness(
                    system.id,
                ),
        )
    );

    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_assignment(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before);
}

#[test]
fn public_release_preserves_all_four_complete_null_skip_routes() {
    for (demand, availability, capacity, unit_off, non_cooling, positive_false, capacity_false) in [
        (-1_000.0, 0.0, true, true, false, false, false),
        (1.0, 1.0, true, false, true, false, false),
        (-1.0e-40, 1.0, true, false, false, true, false),
        (-1_000.0, 1.0, false, false, false, false, true),
    ] {
        let (mut runtime, system, predecessor) =
            completed_cp338_case(demand, availability, capacity, 0.008);
        let snapshot =
            advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_assignment(
                &mut runtime,
                &system,
                predecessor,
            )
            .expect("skipped CP339");

        assert_eq!(snapshot.unit_off_skipped, unit_off);
        assert_eq!(snapshot.non_cooling_skipped, non_cooling);
        assert_eq!(
            snapshot.positive_guard_false_fallthrough_skipped,
            positive_false
        );
        assert_eq!(
            snapshot.capacity_limit_guard_false_fallthrough_skipped,
            capacity_false
        );
        assert!(!snapshot.capacity_limit_sensible_output_assignment_executed);
        assert!(!snapshot.supply_mass_flow_rate_read);
        assert!(snapshot.supply_mass_flow_rate_kg_per_s.is_none());
        assert!(!snapshot.mixed_air_enthalpy_read);
        assert!(snapshot.mixed_air_enthalpy_j_per_kg.is_none());
        assert!(!snapshot.supply_enthalpy_read);
        assert!(snapshot.supply_enthalpy_j_per_kg.is_none());
        assert!(!snapshot.enthalpy_difference_calculated);
        assert!(
            snapshot
                .mixed_air_minus_supply_enthalpy_j_per_kg
                .is_none()
        );
        assert!(!snapshot.cooling_sensible_output_calculated);
        assert!(snapshot.calculated_cooling_sensible_output_w.is_none());
        assert!(!snapshot.cooling_sensible_output_assigned);
        assert!(snapshot.cooling_sensible_output_w.is_none());
        assert_eq!(
            runtime
                .units
                .get(&system.id)
                .expect("known unit")
                .calc_cooling_positive_supply_capacity_limit_sensible_output_assignment
                .source_site_execution_count,
            0
        );
    }
}

#[test]
fn supplied_public_and_private_cp338_drift_is_transactional() {
    let (runtime, system, predecessor) = active_case();

    let mut forged = predecessor;
    forged.source = "forged";
    let mut supplied = runtime.clone();
    assert_rejected_transactionally(&mut supplied, &system, forged);

    let mut public = runtime.clone();
    public
        .units
        .get_mut(&system.id)
        .expect("known unit")
        .calc_cooling_positive_supply_capacity_limit_cp_air_assignment
        .latest
        .as_mut()
        .expect("CP338 latest")
        .cp_air_assigned = false;
    assert_rejected_transactionally(&mut public, &system, predecessor);

    let mut private = runtime;
    let mut witness = private
        .cooling_positive_supply_capacity_limit_cp_air_assignment_latest_witness(system.id)
        .expect("CP338 witness");
    witness.source = "forged-private";
    private.set_cooling_positive_supply_capacity_limit_cp_air_assignment_latest_witness(
        system.id,
        witness,
    );
    assert_rejected_transactionally(&mut private, &system, predecessor);
}

#[test]
fn active_cp330_public_and_private_operand_drift_is_transactional() {
    let (runtime, system, predecessor) = active_case();

    let mut public = runtime.clone();
    let value = &mut public
        .units
        .get_mut(&system.id)
        .expect("known unit")
        .calc_cooling_supply_mass_flow_positive_guard
        .latest
        .as_mut()
        .expect("CP330 latest")
        .supply_mass_flow_rate_kg_per_s;
    *value = value.map(|value| value + 0.001);
    assert_rejected_transactionally(&mut public, &system, predecessor);

    let mut private = runtime;
    let mut witness = private
        .cooling_supply_mass_flow_positive_guard_latest_witness(system.id)
        .expect("CP330 witness");
    witness.supply_mass_flow_rate_kg_per_s = witness
        .supply_mass_flow_rate_kg_per_s
        .map(|value| value + 0.001);
    private.set_cooling_supply_mass_flow_positive_guard_latest_witness(
        system.id,
        witness,
    );
    assert_rejected_transactionally(&mut private, &system, predecessor);
}

#[test]
fn active_cp329_public_and_private_operand_drift_is_transactional() {
    let (runtime, system, predecessor) = active_case();

    let mut public = runtime.clone();
    let value = &mut public
        .units
        .get_mut(&system.id)
        .expect("known unit")
        .calc_cooling_mixed_air_call
        .latest
        .as_mut()
        .expect("CP329 latest")
        .mixed_air_enthalpy_projection_j_per_kg;
    *value = value.map(|value| value + 1.0);
    assert_rejected_transactionally(&mut public, &system, predecessor);

    let mut private = runtime;
    let mut witness = private
        .cooling_mixed_air_call_latest_witness(system.id)
        .expect("CP329 witness");
    witness.mixed_air_enthalpy_projection_j_per_kg = witness
        .mixed_air_enthalpy_projection_j_per_kg
        .map(|value| value + 1.0);
    private.set_cooling_mixed_air_call_latest_witness(system.id, witness);
    assert_rejected_transactionally(&mut private, &system, predecessor);
}

#[test]
fn active_cp336_public_and_private_operand_drift_is_transactional() {
    let (runtime, system, predecessor) = active_case();

    let mut public = runtime.clone();
    let value = &mut public
        .units
        .get_mut(&system.id)
        .expect("known unit")
        .calc_cooling_positive_supply_enthalpy_assignment
        .latest
        .as_mut()
        .expect("CP336 latest")
        .supply_enthalpy_j_per_kg;
    *value = value.map(|value| value + 1.0);
    assert_rejected_transactionally(&mut public, &system, predecessor);

    let mut private = runtime;
    let mut witness = private
        .cooling_positive_supply_enthalpy_assignment_latest_witness(system.id)
        .expect("CP336 witness");
    witness.supply_enthalpy_j_per_kg =
        witness.supply_enthalpy_j_per_kg.map(|value| value + 1.0);
    private.set_cooling_positive_supply_enthalpy_assignment_latest_witness(
        system.id,
        witness,
    );
    assert_rejected_transactionally(&mut private, &system, predecessor);
}

#[test]
fn every_active_counter_overflow_is_preflighted_transactionally() {
    for counter in 0..10 {
        let (mut runtime, system, predecessor) = active_case();
        let state = &mut runtime
            .units
            .get_mut(&system.id)
            .expect("known unit")
            .calc_cooling_positive_supply_capacity_limit_sensible_output_assignment;
        match counter {
            0 => state.transition_count = usize::MAX,
            1 => state.capacity_limit_sensible_output_assignment_count = usize::MAX,
            2 => state.source_site_execution_count = usize::MAX - 5,
            3 => state.supply_mass_flow_rate_read_count = usize::MAX,
            4 => state.mixed_air_enthalpy_read_count = usize::MAX,
            5 => state.supply_enthalpy_read_count = usize::MAX,
            6 => state.enthalpy_difference_calculation_count = usize::MAX,
            7 => state.cooling_sensible_output_calculation_count = usize::MAX,
            8 => state.cooling_sensible_output_assignment_write_count = usize::MAX,
            9 => {
                state.witnessed_capacity_limit_sensible_output_assignment_count =
                    usize::MAX
            }
            _ => unreachable!(),
        }
        let unit = runtime.units.get(&system.id).expect("known unit");
        assert!(
            !super::super::release::
                next_capacity_limit_sensible_output_assignment_transition_fits_for_test(
                    unit,
                    predecessor,
                )
        );
        assert_rejected_transactionally(&mut runtime, &system, predecessor);
    }
}

#[test]
fn every_skip_route_counter_overflow_is_preflighted_transactionally() {
    for (demand, availability, capacity, counter) in [
        (-1_000.0, 0.0, true, 0),
        (1.0, 1.0, true, 1),
        (-1.0e-40, 1.0, true, 2),
        (-1.0e-40, 1.0, true, 3),
        (-1_000.0, 1.0, false, 4),
        (-1_000.0, 1.0, false, 5),
    ] {
        let (mut runtime, system, predecessor) =
            completed_cp338_case(demand, availability, capacity, 0.008);
        let state = &mut runtime
            .units
            .get_mut(&system.id)
            .expect("known unit")
            .calc_cooling_positive_supply_capacity_limit_sensible_output_assignment;
        match counter {
            0 => state.unit_off_skip_count = usize::MAX,
            1 => state.non_cooling_skip_count = usize::MAX,
            2 => state.positive_guard_false_fallthrough_skip_count = usize::MAX,
            3 => state.witnessed_positive_guard_false_fallthrough_skip_count = usize::MAX,
            4 => state.capacity_limit_guard_false_fallthrough_skip_count = usize::MAX,
            5 => {
                state.witnessed_capacity_limit_guard_false_fallthrough_skip_count =
                    usize::MAX
            }
            _ => unreachable!(),
        }
        let unit = runtime.units.get(&system.id).expect("known unit");
        assert!(
            !super::super::release::
                next_capacity_limit_sensible_output_assignment_transition_fits_for_test(
                    unit,
                    predecessor,
                )
        );
        assert_rejected_transactionally(&mut runtime, &system, predecessor);
    }
}

#[test]
fn route_partition_product_corruption_and_post_commit_drift_are_detected() {
    let (mut runtime, system, predecessor) = active_case();
    {
        let state = &mut runtime
            .units
            .get_mut(&system.id)
            .expect("known unit")
            .calc_cooling_positive_supply_capacity_limit_sensible_output_assignment;
        state.capacity_limit_sensible_output_assignment_count = usize::MAX / 6 + 1;
        state.source_site_execution_count = 0;
    }
    let unit = runtime.units.get(&system.id).expect("known unit");
    assert!(
        !super::super::release::
            pending_capacity_limit_sensible_output_assignment_state_is_consistent_for_test(
                unit,
                predecessor,
                None,
            )
    );
    assert_rejected_transactionally(&mut runtime, &system, predecessor);

    let (mut runtime, system, predecessor) = active_case();
    let snapshot =
        advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_assignment(
            &mut runtime,
            &system,
            predecessor,
        )
        .expect("CP339");
    let mut public = runtime.clone();
    public
        .units
        .get_mut(&system.id)
        .expect("known unit")
        .calc_cooling_positive_supply_capacity_limit_sensible_output_assignment
        .latest
        .as_mut()
        .expect("CP339 latest")
        .cooling_sensible_output_assigned = false;
    assert!(
        !completed_direct_cooling_positive_supply_capacity_limit_sensible_output_assignment_is_consistent(
            &public,
            public.units.get(&system.id).expect("known unit"),
            &system,
            snapshot,
            public
                .cooling_positive_supply_capacity_limit_sensible_output_assignment_latest_witness(
                    system.id,
                ),
        )
    );

    let lifecycle =
        purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_assignment_lifecycle_summary(
            &runtime,
            system.id,
        )
        .expect("CP339 lifecycle");
    assert_eq!(lifecycle.state.latest, Some(snapshot));
}
