//! CP381 public release, retained-owner, and commit-atomicity tests.

use super::super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardError as Error,
    advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_guard,
    completed_direct_cooling_post_saturation_capacity_limit_dehumidification_guard_is_consistent,
};
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_guard::completed_cp380_case_for_cp381_test;
use crate::ideal_loads::{
    cooling_post_saturation_capacity_limit_dehumidification_guard_latest_metadata_is_consistent,
    cooling_post_saturation_capacity_limit_dehumidification_guard_snapshot_is_exact_direct_release,
    purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_guard_lifecycle_summary,
};

#[test]
fn cp381_public_direct_reads_retained_cp378_cp379_and_cp329_owners() {
    let (mut runtime, system, cp380) =
        completed_cp380_case_for_cp381_test(true).expect("active CP380 fixture");
    assert!(cp380.capacity_limit_body_entered);
    let unit = runtime.units.get(&system.id).expect("selected unit");
    let expected_supply = unit
        .calc_cooling_supply_humidity_ratio_saturation_limit_assignment
        .latest
        .and_then(|snapshot| snapshot.resulting_supply_humidity_ratio)
        .expect("CP378 supply owner");
    let expected_mixed = unit
        .calc_cooling_mixed_air_call
        .latest
        .and_then(|snapshot| snapshot.mixed_air_humidity_ratio)
        .expect("CP329 mixed-air owner");

    let snapshot =
        advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_guard(
            &mut runtime,
            &system,
            cp380,
        )
        .expect("CP381 direct release");
    assert!(
        cooling_post_saturation_capacity_limit_dehumidification_guard_snapshot_is_exact_direct_release(snapshot)
    );
    assert!(snapshot.dehumidification_guard_evaluated);
    assert!(snapshot.cp378_supply_humidity_ratio_saturation_limit_owned_read);
    assert!(snapshot.cp379_same_call_supply_humidity_ratio_bit_corroborated);
    assert!(snapshot.cp329_mixed_air_humidity_ratio_owned_read);
    assert_eq!(
        snapshot.supply_humidity_ratio.map(f64::to_bits),
        Some(expected_supply.to_bits()),
    );
    assert_eq!(
        snapshot.mixed_air_humidity_ratio.map(f64::to_bits),
        Some(expected_mixed.to_bits()),
    );
    assert_eq!(
        snapshot.supply_humidity_ratio_strictly_less_than_mixed_air_humidity_ratio,
        Some(expected_supply < expected_mixed),
    );

    let unit = runtime.units.get(&system.id).expect("selected unit");
    let witness = runtime
        .cooling_post_saturation_capacity_limit_dehumidification_guard_latest_witness(system.id);
    assert!(
        completed_direct_cooling_post_saturation_capacity_limit_dehumidification_guard_is_consistent(
            &runtime, unit, &system, snapshot, witness,
        )
    );
    assert!(
        cooling_post_saturation_capacity_limit_dehumidification_guard_latest_metadata_is_consistent(
            unit, 1,
        )
    );
    let summary =
        purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_guard_lifecycle_summary(
            &runtime,
            system.id,
        )
        .expect("CP381 summary");
    assert_eq!(summary.state.transition_count, 1);
    assert_eq!(summary.state.latest, Some(snapshot));
}

#[test]
fn cp381_public_direct_outer_false_skips_all_line_2266_sites() {
    let (mut runtime, system, cp380) =
        completed_cp380_case_for_cp381_test(false).expect("outer-false CP380 fixture");
    assert!(cp380.active_guard_false_fallthrough);
    let snapshot =
        advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_guard(
            &mut runtime,
            &system,
            cp380,
        )
        .expect("CP381 skipped direct release");
    assert!(!snapshot.dehumidification_guard_evaluated);
    assert!(!snapshot.purchased_air_supply_humidity_ratio_read);
    assert!(snapshot.supply_humidity_ratio.is_none());
    assert!(!snapshot.purchased_air_mixed_air_humidity_ratio_read);
    assert!(snapshot.mixed_air_humidity_ratio.is_none());
    assert!(
        snapshot
            .supply_humidity_ratio_strictly_less_than_mixed_air_humidity_ratio
            .is_none()
    );
    assert!(!snapshot.dehumidification_body_entered);
    assert!(!snapshot.dehumidification_guard_false_fallthrough);
    let state = &runtime
        .units
        .get(&system.id)
        .expect("selected unit")
        .calc_cooling_post_saturation_capacity_limit_dehumidification_guard;
    assert_eq!(state.dehumidification_guard_evaluation_count, 0);
    assert_eq!(state.source_site_execution_count, 0);
}

#[test]
fn cp381_rejects_each_retained_owner_witness_drift_transactionally() {
    for owner in 0..3 {
        let (mut runtime, system, cp380) =
            completed_cp380_case_for_cp381_test(true).expect("active CP380 fixture");
        match owner {
            0 => {
                let mut forged = runtime
                    .cooling_supply_humidity_ratio_saturation_limit_assignment_latest_witness(
                        system.id,
                    )
                    .expect("CP378 witness");
                forged.resulting_supply_humidity_ratio = forged
                    .resulting_supply_humidity_ratio
                    .map(|value| f64::from_bits(value.to_bits() ^ 1));
                runtime
                    .set_cooling_supply_humidity_ratio_saturation_limit_assignment_latest_witness(
                        system.id, forged,
                    );
            }
            1 => {
                let mut forged = runtime
                    .cooling_supply_enthalpy_post_saturation_assignment_latest_witness(system.id)
                    .expect("CP379 witness");
                forged.supply_humidity_ratio = forged
                    .supply_humidity_ratio
                    .map(|value| f64::from_bits(value.to_bits() ^ 1));
                runtime.set_cooling_supply_enthalpy_post_saturation_assignment_latest_witness(
                    system.id, forged,
                );
            }
            2 => {
                let mut forged = runtime
                    .cooling_mixed_air_call_latest_witness(system.id)
                    .expect("CP329 witness");
                forged.mixed_air_humidity_ratio = forged
                    .mixed_air_humidity_ratio
                    .map(|value| f64::from_bits(value.to_bits() ^ 1));
                runtime.set_cooling_mixed_air_call_latest_witness(system.id, forged);
            }
            _ => unreachable!(),
        }
        let before = runtime.clone();
        let error =
            advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_guard(
                &mut runtime,
                &system,
                cp380,
            )
            .expect_err("owner witness drift must fail closed");
        if owner == 2 {
            assert!(matches!(
                error,
                Error::MixedAirHumidityRatioOwnerLineageMismatch { .. }
            ));
        } else {
            assert!(matches!(
                error,
                Error::CoolingPostSaturationCapacityLimitGuardSnapshotMismatch { .. }
                    | Error::SupplyHumidityRatioOwnerLineageMismatch { .. }
            ));
        }
        assert_eq!(runtime, before);
    }
}

#[test]
fn cp381_duplicate_public_release_is_rejected_transactionally() {
    let (mut runtime, system, cp380) =
        completed_cp380_case_for_cp381_test(true).expect("active CP380 fixture");
    advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_guard(
        &mut runtime,
        &system,
        cp380,
    )
    .expect("first CP381 release");
    let before = runtime.clone();
    assert!(matches!(
        advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_guard(
            &mut runtime,
            &system,
            cp380,
        ),
        Err(Error::PredecessorCallOrder { .. } | Error::RuntimeStateInvariantViolation { .. })
    ));
    assert_eq!(runtime, before);
}
