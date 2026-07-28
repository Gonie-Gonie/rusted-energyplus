use super::super::*;
use crate::ideal_loads::calc::cooling_positive_supply_capacity_limit_sensible_output_assignment::tests::release_fixture::completed_cp338_case;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot,
    PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_assignment,
    advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_guard,
};

fn completed_cp339_case(
    cooling_demand_w: f64,
    overall_availability: f64,
    capacity_limit: bool,
) -> (
    PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot,
) {
    let (mut runtime, system, predecessor) = completed_cp338_case(
        cooling_demand_w,
        overall_availability,
        capacity_limit,
        0.008,
    );
    let assignment =
        advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_assignment(
            &mut runtime,
            &system,
            predecessor,
        )
        .expect("CP339");
    (runtime, system, assignment)
}

fn active_case() -> (
    PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot,
) {
    completed_cp339_case(-1_000.0, 1.0, true)
}

fn assert_rejected_transactionally(
    runtime: &mut PurchasedAirRuntimeState,
    system: &ep_model::IdealLoadsAirSystem,
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot,
) {
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_guard(
            runtime,
            system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(*runtime, before);
}

#[test]
fn public_active_release_reads_only_retained_cp339_and_cp321_operands() {
    let (mut runtime, system, predecessor) = active_case();
    assert!(predecessor.capacity_limit_sensible_output_assignment_executed);
    let unit = runtime.units.get(&system.id).expect("known unit");
    let retained_output = unit
        .calc_cooling_positive_supply_capacity_limit_sensible_output_assignment
        .latest
        .and_then(|snapshot| snapshot.cooling_sensible_output_w)
        .expect("retained CP339 output");
    let retained_capacity = unit
        .calc_cooling_capacity_zero_flow_reset
        .latest
        .and_then(|snapshot| snapshot.maximum_total_cooling_capacity_w)
        .expect("retained CP321 capacity");
    let expected = retained_output >= retained_capacity;

    let snapshot =
        advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_guard(
            &mut runtime,
            &system,
            predecessor,
        )
        .expect("CP340");
    assert!(
        cooling_positive_supply_capacity_limit_sensible_output_guard_snapshot_is_exact_direct_release(
            snapshot,
        )
    );
    assert_eq!(
        snapshot.cooling_sensible_output_w.map(f64::to_bits),
        Some(retained_output.to_bits())
    );
    assert_eq!(
        snapshot
            .maximum_total_cooling_capacity_w
            .map(f64::to_bits),
        Some(retained_capacity.to_bits())
    );
    assert_eq!(
        snapshot.cooling_sensible_output_at_or_above_maximum_capacity,
        Some(expected)
    );
    assert_eq!(
        snapshot.capacity_limit_sensible_output_adjustment_body_entered,
        expected
    );
    assert_eq!(
        snapshot.capacity_limit_sensible_output_guard_false_fallthrough,
        !expected
    );
    assert!(
        completed_direct_cooling_positive_supply_capacity_limit_sensible_output_guard_is_consistent(
            &runtime,
            runtime.units.get(&system.id).expect("known unit"),
            &system,
            snapshot,
            runtime
                .cooling_positive_supply_capacity_limit_sensible_output_guard_latest_witness(
                    system.id,
                ),
        )
    );

    assert_rejected_transactionally(&mut runtime, &system, predecessor);
}

#[test]
fn public_release_preserves_all_four_complete_skip_routes_without_operand_reads() {
    for (demand, availability, capacity, unit_off, non_cooling, positive_false, capacity_false) in [
        (-1_000.0, 0.0, true, true, false, false, false),
        (1.0, 1.0, true, false, true, false, false),
        (-1.0e-40, 1.0, true, false, false, true, false),
        (-1_000.0, 1.0, false, false, false, false, true),
    ] {
        let (mut runtime, system, predecessor) =
            completed_cp339_case(demand, availability, capacity);
        let snapshot =
            advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_guard(
                &mut runtime,
                &system,
                predecessor,
            )
            .expect("skipped CP340");

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
        assert!(!snapshot.capacity_limit_sensible_output_guard_evaluated);
        assert!(!snapshot.cooling_sensible_output_read);
        assert!(snapshot.cooling_sensible_output_w.is_none());
        assert!(!snapshot.maximum_total_cooling_capacity_read);
        assert!(snapshot.maximum_total_cooling_capacity_w.is_none());
        assert_eq!(
            runtime
                .units
                .get(&system.id)
                .expect("known unit")
                .calc_cooling_positive_supply_capacity_limit_sensible_output_guard
                .source_site_execution_count,
            0
        );
    }
}

#[test]
fn supplied_public_and_private_cp339_drift_is_transactional() {
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
        .calc_cooling_positive_supply_capacity_limit_sensible_output_assignment
        .latest
        .as_mut()
        .expect("CP339 latest")
        .cooling_sensible_output_w = Some(123.0);
    assert_rejected_transactionally(&mut public, &system, predecessor);

    let mut private = runtime;
    let mut witness = private
        .cooling_positive_supply_capacity_limit_sensible_output_assignment_latest_witness(
            system.id,
        )
        .expect("CP339 witness");
    witness.cooling_sensible_output_w = Some(456.0);
    private
        .set_cooling_positive_supply_capacity_limit_sensible_output_assignment_latest_witness(
            system.id,
            witness,
        );
    assert_rejected_transactionally(&mut private, &system, predecessor);
}

#[test]
fn cp321_capacity_and_non_operand_private_bits_are_transactionally_protected() {
    let (runtime, system, predecessor) = active_case();

    let mut public = runtime.clone();
    public
        .units
        .get_mut(&system.id)
        .expect("known unit")
        .calc_cooling_capacity_zero_flow_reset
        .latest
        .as_mut()
        .expect("CP321 latest")
        .maximum_total_cooling_capacity_w = Some(9_999.0);
    assert_rejected_transactionally(&mut public, &system, predecessor);

    let mut private = runtime.clone();
    let mut witness = private
        .cooling_capacity_zero_flow_reset_latest_witness(system.id)
        .expect("CP321 witness");
    witness.maximum_total_cooling_capacity_w = Some(9_999.0);
    private.set_cooling_capacity_zero_flow_reset_latest_witness(system.id, witness);
    assert_rejected_transactionally(&mut private, &system, predecessor);

    let mut signed_zero_private = runtime;
    let mut witness = signed_zero_private
        .cooling_capacity_zero_flow_reset_latest_witness(system.id)
        .expect("CP321 witness");
    let original = witness
        .predecessor_supply_mass_flow_rate_for_humidification_kg_per_s
        .expect("retained CP320 humidity candidate");
    assert_eq!(original, 0.0);
    witness.predecessor_supply_mass_flow_rate_for_humidification_kg_per_s =
        Some(f64::from_bits(original.to_bits() ^ (1_u64 << 63)));
    signed_zero_private
        .set_cooling_capacity_zero_flow_reset_latest_witness(system.id, witness);
    assert_rejected_transactionally(
        &mut signed_zero_private,
        &system,
        predecessor,
    );
}

#[test]
fn forged_public_active_signed_zero_capacities_are_rejected_by_retained_chain() {
    for forged_capacity in [0.0, -0.0] {
        let (mut runtime, system, predecessor) = active_case();
        let mut public = runtime
            .units
            .get(&system.id)
            .expect("known unit")
            .calc_cooling_capacity_zero_flow_reset
            .latest
            .expect("CP321 latest");
        public.maximum_total_cooling_capacity_w = Some(forged_capacity);
        runtime
            .units
            .get_mut(&system.id)
            .expect("known unit")
            .calc_cooling_capacity_zero_flow_reset
            .latest = Some(public);
        runtime.set_cooling_capacity_zero_flow_reset_latest_witness(system.id, public);

        assert_rejected_transactionally(&mut runtime, &system, predecessor);
    }
}

#[test]
fn recursive_cp338_witness_corruption_is_rejected_before_cp340_mutation() {
    let (mut runtime, system, predecessor) = active_case();
    let mut witness = runtime
        .cooling_positive_supply_capacity_limit_cp_air_assignment_latest_witness(system.id)
        .expect("CP338 witness");
    witness.source = "forged-cp338-private";
    runtime.set_cooling_positive_supply_capacity_limit_cp_air_assignment_latest_witness(
        system.id,
        witness,
    );

    assert_rejected_transactionally(&mut runtime, &system, predecessor);
}

#[test]
fn every_active_counter_increment_is_checked_before_public_mutation() {
    for counter in 0..8 {
        let (mut runtime, system, predecessor) = active_case();
        let cooling_sensible_output_w = predecessor
            .cooling_sensible_output_w
            .expect("CP339 output");
        let maximum_total_cooling_capacity_w = runtime
            .units
            .get(&system.id)
            .expect("known unit")
            .calc_cooling_capacity_zero_flow_reset
            .latest
            .and_then(|snapshot| snapshot.maximum_total_cooling_capacity_w)
            .expect("CP321 capacity");
        let active_input =
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardActiveInput {
                cooling_sensible_output_w,
                maximum_total_cooling_capacity_w,
            };
        let body =
            cooling_sensible_output_w >= maximum_total_cooling_capacity_w;
        let state = &mut runtime
            .units
            .get_mut(&system.id)
            .expect("known unit")
            .calc_cooling_positive_supply_capacity_limit_sensible_output_guard;
        match counter {
            0 => state.transition_count = usize::MAX,
            1 => state.capacity_limit_sensible_output_guard_evaluation_count = usize::MAX,
            2 => state.source_site_execution_count = usize::MAX - 2,
            3 => state.cooling_sensible_output_read_count = usize::MAX,
            4 => state.maximum_total_cooling_capacity_read_count = usize::MAX,
            5 => {
                state.cooling_sensible_output_maximum_capacity_comparison_count =
                    usize::MAX
            }
            6 if body => {
                state.capacity_limit_sensible_output_adjustment_body_entry_count =
                    usize::MAX
            }
            6 => {
                state.capacity_limit_sensible_output_guard_false_fallthrough_count =
                    usize::MAX
            }
            7 if body => {
                state.witnessed_capacity_limit_sensible_output_adjustment_body_entry_count =
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
            !super::super::release::
                next_capacity_limit_sensible_output_guard_transition_fits_for_test(
                    unit,
                    predecessor,
                    Some(active_input),
                )
        );
        assert_rejected_transactionally(&mut runtime, &system, predecessor);
    }
}
