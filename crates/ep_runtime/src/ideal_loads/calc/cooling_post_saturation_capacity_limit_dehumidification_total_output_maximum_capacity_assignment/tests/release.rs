//! CP384 public release, retained-owner, and commit-atomicity tests.

use super::super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputMaximumCapacityAssignmentError as Error,
    advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment,
    completed_direct_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment_is_consistent,
};
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_total_output_guard::{
    advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard,
    completed_cp382_case_for_cp384_test,
};
use crate::ideal_loads::*;

#[test]
fn cp384_public_direct_assigns_only_from_retained_cp383_bits() {
    let (mut runtime, system, cp383) = completed_cp383_case(true);
    let snapshot = advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment(
        &mut runtime,
        &system,
        cp383,
    )
    .expect("CP384 direct release");
    assert!(cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment_snapshot_is_exact_direct_release(snapshot));
    if cp383.dehumidification_total_output_capacity_adjustment_body_entered {
        let expected = cp383.maximum_total_cooling_capacity_w.map(f64::to_bits);
        assert!(snapshot.cp383_retained_maximum_total_cooling_capacity_owned_read);
        assert!(snapshot.cooling_total_output_assigned);
        assert_eq!(snapshot.maximum_total_cooling_capacity_w.map(f64::to_bits), expected);
        assert_eq!(snapshot.assigned_cooling_total_output_w.map(f64::to_bits), expected);
        assert_eq!(snapshot.resulting_cooling_total_output_w.map(f64::to_bits), expected);
    } else if cp383.dehumidification_total_output_capacity_guard_false_fallthrough {
        let expected = cp383.cooling_total_output_w.map(f64::to_bits);
        assert!(!snapshot.maximum_total_cooling_capacity_read);
        assert!(!snapshot.cooling_total_output_assigned);
        assert_eq!(snapshot.resulting_cooling_total_output_w.map(f64::to_bits), expected);
    }

    let unit = runtime.units.get(&system.id).expect("selected unit");
    let witness = runtime
        .cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment_latest_witness(system.id);
    assert!(completed_direct_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment_is_consistent(
        &runtime, unit, &system, snapshot, witness,
    ));
    assert!(cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment_latest_metadata_is_consistent(unit, 1));
    let summary = purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment_lifecycle_summary(&runtime, system.id)
        .expect("CP384 summary");
    assert_eq!(summary.state.latest, Some(snapshot));
}

#[test]
fn cp384_public_direct_inactive_predecessor_skips_line_2269() {
    let (mut runtime, system, cp383) = completed_cp383_case(false);
    let snapshot = advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment(
        &mut runtime,
        &system,
        cp383,
    )
    .expect("CP384 skip release");
    assert!(!snapshot.dehumidification_total_output_maximum_capacity_assignment_executed);
    assert!(snapshot.maximum_total_cooling_capacity_w.is_none());
    assert!(snapshot.assigned_cooling_total_output_w.is_none());
    let state = &runtime
        .units
        .get(&system.id)
        .expect("selected unit")
        .calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment;
    assert_eq!(state.source_site_execution_count, 0);
}

#[test]
fn cp384_rejects_supplied_cp383_drift_transactionally() {
    let (mut runtime, system, cp383) = completed_cp383_case(true);
    let mut forged = cp383;
    forged.maximum_total_cooling_capacity_w = forged
        .maximum_total_cooling_capacity_w
        .map(|value| f64::from_bits(value.to_bits() ^ 1));
    let before = runtime.clone();
    assert!(matches!(
        advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment(
            &mut runtime,
            &system,
            forged,
        ),
        Err(
            Error::CoolingPostSaturationCapacityLimitDehumidificationTotalOutputGuardSnapshotMismatch { .. }
                | Error::PredecessorOutsideDirectSubset { .. }
        )
    ));
    assert_eq!(runtime, before);
}

#[test]
fn cp384_duplicate_public_release_is_rejected_transactionally() {
    let (mut runtime, system, cp383) = completed_cp383_case(true);
    advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment(
        &mut runtime,
        &system,
        cp383,
    )
    .expect("first CP384 release");
    let before = runtime.clone();
    assert!(matches!(
        advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment(
            &mut runtime,
            &system,
            cp383,
        ),
        Err(Error::PredecessorCallOrder { .. } | Error::RuntimeStateInvariantViolation { .. })
    ));
    assert_eq!(runtime, before);
}

fn completed_cp383_case(
    capacity_limit: bool,
) -> (
    PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputGuardSnapshot,
) {
    let (mut runtime, system, cp382) = completed_cp382_case_for_cp384_test(capacity_limit);
    let cp383 = advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard(
        &mut runtime,
        &system,
        cp382,
    )
    .expect("CP383");
    (runtime, system, cp383)
}
