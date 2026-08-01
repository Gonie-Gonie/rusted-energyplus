//! CP383 public release, retained-owner, and commit-atomicity tests.

use super::super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputGuardError as Error,
    advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard,
    completed_direct_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard_is_consistent,
};
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_guard::advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_guard;
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment::advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment;
use crate::ideal_loads::calc::cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard::completed_cp370_case_with_capacity_limit_for_later_test;
use crate::ideal_loads::*;

#[test]
fn cp383_public_direct_reads_cp382_and_cp321_with_cp340_corroboration() {
    let (mut runtime, system, cp382) = completed_cp382_case(true);
    assert!(cp382.dehumidification_total_output_assignment_executed);
    let unit = runtime.units.get(&system.id).expect("selected unit");
    let output = cp382.cooling_total_output_w.expect("CP382 owner");
    let capacity = unit
        .calc_cooling_capacity_zero_flow_reset
        .latest
        .and_then(|snapshot| snapshot.maximum_total_cooling_capacity_w)
        .expect("CP321 owner");
    let cp340_capacity = unit
        .calc_cooling_positive_supply_capacity_limit_sensible_output_guard
        .latest
        .and_then(|snapshot| snapshot.maximum_total_cooling_capacity_w)
        .expect("CP340 corroborator");
    assert_eq!(capacity.to_bits(), cp340_capacity.to_bits());

    let snapshot = advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard(
        &mut runtime,
        &system,
        cp382,
    )
    .expect("CP383 direct release");
    assert!(cooling_post_saturation_capacity_limit_dehumidification_total_output_guard_snapshot_is_exact_direct_release(snapshot));
    assert!(snapshot.cp382_cooling_total_output_owned_read);
    assert!(snapshot.cp321_maximum_total_cooling_capacity_owned_read);
    assert!(snapshot.cp340_same_call_maximum_total_cooling_capacity_bit_corroborated);
    assert_eq!(snapshot.cooling_total_output_w.map(f64::to_bits), Some(output.to_bits()));
    assert_eq!(snapshot.maximum_total_cooling_capacity_w.map(f64::to_bits), Some(capacity.to_bits()));
    assert_eq!(snapshot.cooling_total_output_strictly_greater_than_maximum_total_cooling_capacity, Some(output > capacity));
    assert_eq!(snapshot.dehumidification_total_output_capacity_adjustment_body_entered, output > capacity);

    let unit = runtime.units.get(&system.id).expect("selected unit");
    let witness = runtime
        .cooling_post_saturation_capacity_limit_dehumidification_total_output_guard_latest_witness(system.id);
    assert!(completed_direct_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard_is_consistent(
        &runtime, unit, &system, snapshot, witness,
    ));
    assert!(cooling_post_saturation_capacity_limit_dehumidification_total_output_guard_latest_metadata_is_consistent(unit, 1));
    let summary = purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard_lifecycle_summary(&runtime, system.id)
        .expect("CP383 summary");
    assert_eq!(summary.state.latest, Some(snapshot));
}

#[test]
fn cp383_public_direct_inactive_predecessor_skips_all_sites() {
    let (mut runtime, system, cp382) = completed_cp382_case(false);
    assert!(!cp382.dehumidification_total_output_assignment_executed);
    let snapshot = advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard(
        &mut runtime,
        &system,
        cp382,
    )
    .expect("CP383 skip release");
    assert!(!snapshot.dehumidification_total_output_capacity_guard_evaluated);
    assert!(snapshot.cooling_total_output_w.is_none());
    assert!(snapshot.maximum_total_cooling_capacity_w.is_none());
    let state = &runtime
        .units
        .get(&system.id)
        .expect("selected unit")
        .calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard;
    assert_eq!(state.source_site_execution_count, 0);
}

#[test]
fn cp383_rejects_cp340_capacity_corroboration_drift_transactionally() {
    let (mut runtime, system, cp382) = completed_cp382_case(true);
    let mut forged = runtime
        .cooling_positive_supply_capacity_limit_sensible_output_guard_latest_witness(system.id)
        .expect("CP340 witness");
    forged.maximum_total_cooling_capacity_w = forged
        .maximum_total_cooling_capacity_w
        .map(|value| f64::from_bits(value.to_bits() ^ 1));
    runtime.set_cooling_positive_supply_capacity_limit_sensible_output_guard_latest_witness(
        system.id,
        forged,
    );
    let before = runtime.clone();
    assert!(matches!(
        advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard(
            &mut runtime,
            &system,
            cp382,
        ),
        Err(
            Error::ActiveOperandOwnerLineageMismatch { .. }
                | Error::CoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentSnapshotMismatch { .. }
        )
    ));
    assert_eq!(runtime, before);
}

#[test]
fn cp383_duplicate_public_release_is_rejected_transactionally() {
    let (mut runtime, system, cp382) = completed_cp382_case(true);
    advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard(
        &mut runtime,
        &system,
        cp382,
    )
    .expect("first CP383 release");
    let before = runtime.clone();
    assert!(matches!(
        advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard(
            &mut runtime,
            &system,
            cp382,
        ),
        Err(Error::PredecessorCallOrder { .. } | Error::RuntimeStateInvariantViolation { .. })
    ));
    assert_eq!(runtime, before);
}

fn completed_cp382_case(
    capacity_limit: bool,
) -> (
    PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentSnapshot,
) {
    let (mut runtime, system, cp370) =
        completed_cp370_case_with_capacity_limit_for_later_test(capacity_limit)
            .expect("completed CP370 fixture");
    let cp371 = advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard(&mut runtime, &system, cp370).expect("CP371");
    let cp372 = advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment(&mut runtime, &system, cp371).expect("CP372");
    let cp373 = advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment(&mut runtime, &system, cp372).expect("CP373");
    let cp374 = advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit(&mut runtime, &system, cp373).expect("CP374");
    let cp375 = advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment(&mut runtime, &system, cp374).expect("CP375");
    let cp376 = advance_direct_no_oa_calc_cooling_supply_humidity_ratio_pre_saturation_original_assignment(&mut runtime, &system, cp375).expect("CP376");
    let pressure = if capacity_limit { 300_000.0 } else { 101_325.0 };
    let cp377 = advance_direct_no_oa_calc_cooling_supply_humidity_ratio_saturation_assignment(&mut runtime, &system, cp376, pressure).expect("CP377");
    let cp378 = advance_direct_no_oa_calc_cooling_supply_humidity_ratio_saturation_limit_assignment(&mut runtime, &system, cp377).expect("CP378");
    let cp379 = advance_direct_no_oa_calc_cooling_supply_enthalpy_post_saturation_assignment(&mut runtime, &system, cp378).expect("CP379");
    let cp380 = advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_guard(&mut runtime, &system, cp379).expect("CP380");
    let cp381 = advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_guard(&mut runtime, &system, cp380).expect("CP381");
    let cp382 = advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment(&mut runtime, &system, cp381).expect("CP382");
    (runtime, system, cp382)
}
