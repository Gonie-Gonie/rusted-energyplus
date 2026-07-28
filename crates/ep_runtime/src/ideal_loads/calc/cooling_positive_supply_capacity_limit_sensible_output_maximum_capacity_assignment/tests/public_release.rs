use super::super::*;
use crate::ideal_loads::calc::cooling_positive_supply_capacity_limit_sensible_output_guard::tests::release_fixture::{
    completed_cp340_case, completed_cp340_case_with_zone_temperature,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot,
    PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment,
};

fn assert_rejected_transactionally(
    runtime: &mut PurchasedAirRuntimeState,
    system: &ep_model::IdealLoadsAirSystem,
    predecessor: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot,
) {
    let selected = predecessor.system;
    let before_state = runtime
        .units
        .get(&selected)
        .expect("known unit")
        .calc_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment
        .clone();
    let before_witness = runtime
        .cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_latest_witness(
            selected,
        );
    assert!(
        advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment(
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
            .calc_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment,
        before_state
    );
    assert_eq!(
        runtime
            .cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_latest_witness(
                selected,
            ),
        before_witness
    );
}

fn false_case() -> (
    PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot,
) {
    let case = completed_cp340_case(-1_000.0, 1.0, true);
    assert!(
        case.2
            .capacity_limit_sensible_output_guard_false_fallthrough
    );
    case
}

fn assignment_case() -> (
    PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot,
) {
    let case = completed_cp340_case(-100_000.0, 1.0, true);
    assert!(
        case.2
            .capacity_limit_sensible_output_adjustment_body_entered
    );
    case
}

#[test]
fn public_false_route_preserves_retained_output_and_executes_no_cp341_site() {
    let (mut runtime, system, predecessor) = false_case();
    let expected = predecessor
        .cooling_sensible_output_w
        .expect("CP340 retained output");
    let snapshot =
        advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment(
            &mut runtime,
            &system,
            predecessor,
        )
        .expect("CP341 false route");

    assert!(
        cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_snapshot_is_exact_direct_release(
            snapshot,
        )
    );
    assert_eq!(
        snapshot
            .preexisting_cooling_sensible_output_w
            .map(f64::to_bits),
        Some(expected.to_bits())
    );
    assert_eq!(
        snapshot
            .resulting_cooling_sensible_output_w
            .map(f64::to_bits),
        Some(expected.to_bits())
    );
    assert!(!snapshot.maximum_total_cooling_capacity_read);
    assert!(snapshot.maximum_total_cooling_capacity_w.is_none());
    assert!(!snapshot.cooling_sensible_output_assigned);
    assert!(snapshot.assigned_cooling_sensible_output_w.is_none());
    assert_eq!(
        runtime
            .units
            .get(&system.id)
            .expect("known unit")
            .calc_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment
            .source_site_execution_count,
        0
    );
}

#[test]
fn public_true_route_reads_retained_rhs_and_assigns_result_bit_exact() {
    let (mut runtime, system, predecessor) = assignment_case();
    let expected_preexisting = predecessor
        .cooling_sensible_output_w
        .expect("CP340 retained output");
    let expected_maximum = predecessor
        .maximum_total_cooling_capacity_w
        .expect("CP340 retained maximum");
    let snapshot =
        advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment(
            &mut runtime,
            &system,
            predecessor,
        )
        .expect("CP341 assignment");

    assert_eq!(
        snapshot
            .preexisting_cooling_sensible_output_w
            .map(f64::to_bits),
        Some(expected_preexisting.to_bits())
    );
    assert!(snapshot.maximum_total_cooling_capacity_read);
    for value in [
        snapshot.maximum_total_cooling_capacity_w,
        snapshot.assigned_cooling_sensible_output_w,
        snapshot.resulting_cooling_sensible_output_w,
    ] {
        assert_eq!(
            value.map(f64::to_bits),
            Some(expected_maximum.to_bits())
        );
    }
    assert!(
        completed_direct_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_is_consistent(
            &runtime,
            runtime.units.get(&system.id).expect("known unit"),
            &system,
            snapshot,
            runtime
                .cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_latest_witness(
                    system.id,
                ),
        )
    );
    assert_rejected_transactionally(&mut runtime, &system, predecessor);
}

#[test]
fn full_public_nan_chain_preserves_payload_without_rhs_read_or_write() {
    let (mut runtime, system, predecessor) =
        completed_cp340_case_with_zone_temperature(
            -f64::MAX,
            1.0,
            true,
            0.008,
            13.000_02,
        );
    let retained_nan = predecessor
        .cooling_sensible_output_w
        .expect("CP340 retained CP339 output");
    assert!(retained_nan.is_nan());
    assert!(
        predecessor.capacity_limit_sensible_output_guard_false_fallthrough
    );
    assert!(
        !predecessor.capacity_limit_sensible_output_adjustment_body_entered
    );

    let snapshot =
        advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment(
            &mut runtime,
            &system,
            predecessor,
        )
        .expect("CP341 public NaN false route");

    assert_eq!(
        snapshot
            .preexisting_cooling_sensible_output_w
            .map(f64::to_bits),
        Some(retained_nan.to_bits())
    );
    assert_eq!(
        snapshot
            .resulting_cooling_sensible_output_w
            .map(f64::to_bits),
        Some(retained_nan.to_bits())
    );
    assert!(!snapshot.maximum_total_cooling_capacity_read);
    assert!(snapshot.maximum_total_cooling_capacity_w.is_none());
    assert!(!snapshot.cooling_sensible_output_assigned);
    assert!(snapshot.assigned_cooling_sensible_output_w.is_none());
}

#[test]
fn full_public_positive_infinity_chain_assigns_finite_retained_maximum() {
    let (mut runtime, system, predecessor) =
        completed_cp340_case(-f64::MAX, 1.0, true);
    let retained_output = predecessor
        .cooling_sensible_output_w
        .expect("CP340 retained CP339 output");
    let retained_maximum = predecessor
        .maximum_total_cooling_capacity_w
        .expect("CP340 retained maximum capacity");
    assert_eq!(retained_output, f64::INFINITY);
    assert!(retained_maximum.is_finite() && retained_maximum > 0.0);
    assert!(
        predecessor.capacity_limit_sensible_output_adjustment_body_entered
    );

    let snapshot =
        advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment(
            &mut runtime,
            &system,
            predecessor,
        )
        .expect("CP341 public positive-infinity assignment route");

    assert_eq!(
        snapshot
            .preexisting_cooling_sensible_output_w
            .map(f64::to_bits),
        Some(f64::INFINITY.to_bits())
    );
    assert!(snapshot.maximum_total_cooling_capacity_read);
    assert!(snapshot.cooling_sensible_output_assigned);
    for value in [
        snapshot.maximum_total_cooling_capacity_w,
        snapshot.assigned_cooling_sensible_output_w,
        snapshot.resulting_cooling_sensible_output_w,
    ] {
        assert_eq!(
            value.map(f64::to_bits),
            Some(retained_maximum.to_bits())
        );
    }
}

#[test]
fn public_release_preserves_all_four_complete_skip_routes_without_local_values() {
    for (demand, availability, capacity, unit_off, non_cooling, positive_false, capacity_false) in [
        (-1_000.0, 0.0, true, true, false, false, false),
        (1.0, 1.0, true, false, true, false, false),
        (-1.0e-40, 1.0, true, false, false, true, false),
        (-1_000.0, 1.0, false, false, false, false, true),
    ] {
        let (mut runtime, system, predecessor) =
            completed_cp340_case(demand, availability, capacity);
        let snapshot =
            advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment(
                &mut runtime,
                &system,
                predecessor,
            )
            .expect("skipped CP341");

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
        assert!(snapshot.preexisting_cooling_sensible_output_w.is_none());
        assert!(!snapshot.maximum_total_cooling_capacity_read);
        assert!(snapshot.maximum_total_cooling_capacity_w.is_none());
        assert!(!snapshot.cooling_sensible_output_assigned);
        assert!(snapshot.assigned_cooling_sensible_output_w.is_none());
        assert!(snapshot.resulting_cooling_sensible_output_w.is_none());
    }
}

#[test]
fn supplied_public_and_private_cp340_drift_are_transactional() {
    let (runtime, system, predecessor) = assignment_case();

    let mut supplied = runtime.clone();
    let mut forged = predecessor;
    forged.source = "forged-cp340";
    assert_rejected_transactionally(&mut supplied, &system, forged);

    let mut public = runtime.clone();
    public
        .units
        .get_mut(&system.id)
        .expect("known unit")
        .calc_cooling_positive_supply_capacity_limit_sensible_output_guard
        .latest
        .as_mut()
        .expect("CP340 latest")
        .cooling_sensible_output_w = Some(123.0);
    assert_rejected_transactionally(&mut public, &system, predecessor);

    let mut private = runtime;
    let mut witness = private
        .cooling_positive_supply_capacity_limit_sensible_output_guard_latest_witness(system.id)
        .expect("CP340 witness");
    witness.cooling_sensible_output_w = Some(456.0);
    private
        .set_cooling_positive_supply_capacity_limit_sensible_output_guard_latest_witness(
            system.id,
            witness,
        );
    assert_rejected_transactionally(&mut private, &system, predecessor);
}

#[test]
fn forged_nonpositive_and_nonfinite_active_capacities_are_rejected() {
    for forged_capacity in [
        0.0,
        -0.0,
        -1.0,
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::from_bits(0x7ff8_0000_0000_0341),
    ] {
        let (mut runtime, system, mut predecessor) = assignment_case();
        predecessor.maximum_total_cooling_capacity_w = Some(forged_capacity);
        runtime
            .units
            .get_mut(&system.id)
            .expect("known unit")
            .calc_cooling_positive_supply_capacity_limit_sensible_output_guard
            .latest = Some(predecessor);
        runtime
            .set_cooling_positive_supply_capacity_limit_sensible_output_guard_latest_witness(
                system.id,
                predecessor,
            );
        assert_rejected_transactionally(&mut runtime, &system, predecessor);
    }
}

#[test]
fn recursive_cp339_private_witness_corruption_is_rejected_before_mutation() {
    let (mut runtime, system, predecessor) = assignment_case();
    let mut witness = runtime
        .cooling_positive_supply_capacity_limit_sensible_output_assignment_latest_witness(system.id)
        .expect("CP339 witness");
    witness.source = "forged-cp339-private";
    runtime
        .set_cooling_positive_supply_capacity_limit_sensible_output_assignment_latest_witness(
            system.id,
            witness,
        );
    assert_rejected_transactionally(&mut runtime, &system, predecessor);
}

#[test]
fn every_assignment_counter_increment_is_preflighted_transactionally() {
    for counter in 0..6 {
        let (mut runtime, system, predecessor) = assignment_case();
        let state = &mut runtime
            .units
            .get_mut(&system.id)
            .expect("known unit")
            .calc_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment;
        match counter {
            0 => state.transition_count = usize::MAX,
            1 => {
                state.capacity_limit_sensible_output_maximum_capacity_assignment_count =
                    usize::MAX
            }
            2 => state.source_site_execution_count = usize::MAX - 1,
            3 => state.maximum_total_cooling_capacity_read_count = usize::MAX,
            4 => state.cooling_sensible_output_assignment_write_count = usize::MAX,
            5 => {
                state.witnessed_capacity_limit_sensible_output_maximum_capacity_assignment_count =
                    usize::MAX
            }
            _ => unreachable!(),
        }
        let unit = runtime.units.get(&system.id).expect("known unit");
        assert!(
            !super::super::release::next_maximum_capacity_assignment_transition_fits_for_test(
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
            completed_cp340_case(demand, availability, capacity_limit);
        let state = &mut runtime
            .units
            .get_mut(&system.id)
            .expect("known unit")
            .calc_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment;
        match counter {
            0 => state.unit_off_skip_count = usize::MAX,
            1 => state.non_cooling_skip_count = usize::MAX,
            2 => state.positive_guard_false_fallthrough_skip_count = usize::MAX,
            3 => {
                state.witnessed_positive_guard_false_fallthrough_skip_count =
                    usize::MAX
            }
            4 => state.capacity_limit_guard_false_fallthrough_skip_count = usize::MAX,
            5 => {
                state.witnessed_capacity_limit_guard_false_fallthrough_skip_count =
                    usize::MAX
            }
            6 => {
                state.capacity_limit_sensible_output_guard_false_fallthrough_count =
                    usize::MAX
            }
            7 => {
                state.witnessed_capacity_limit_sensible_output_guard_false_fallthrough_count =
                    usize::MAX
            }
            _ => unreachable!(),
        }
        let unit = runtime.units.get(&system.id).expect("known unit");
        assert!(
            !super::super::release::next_maximum_capacity_assignment_transition_fits_for_test(
                unit,
                predecessor,
            )
        );
        assert_rejected_transactionally(&mut runtime, &system, predecessor);
    }
}
