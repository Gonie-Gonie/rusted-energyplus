//! CP392 public-release fail-closed and transactional tests.

use super::*;
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_total_output_guard::completed_cp382_case_for_cp384_test;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitSnapshot as Cp391,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyHumidityRatioAssignmentError as Error,
    PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment,
    advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment,
    advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment,
    advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit,
    advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit,
    advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_control_switch,
    advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard,
    advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment,
    advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment,
};
use ep_model::{
    DehumidificationControlType, IdealLoadsAirSystem, IdealLoadsAirSystemId, IdealLoadsLimit,
};

#[test]
fn identity_subset_and_initialization_failures_preserve_cp391_state_and_witness() {
    let (runtime, system, predecessor) = completed_cp391_case();

    let mut identity_runtime = runtime.clone();
    let mut wrong_identity = system.clone();
    wrong_identity.id = IdealLoadsAirSystemId(system.id.0 + 1);
    assert_rejected_transactionally(
        &mut identity_runtime,
        &wrong_identity,
        predecessor,
        |error| matches!(error, Error::SystemIdentityMismatch { .. }),
    );

    let mut subset_runtime = runtime.clone();
    let mut outside_subset = system.clone();
    outside_subset.cooling_limit = IdealLoadsLimit::LimitCapacity;
    outside_subset.maximum_total_cooling_capacity_w = None;
    assert_rejected_transactionally(&mut subset_runtime, &outside_subset, predecessor, |error| {
        matches!(error, Error::SystemOutsideDirectSubset { .. })
    });

    let mut selector_runtime = runtime.clone();
    let mut wrong_selector = system.clone();
    wrong_selector.dehumidification_control_type =
        DehumidificationControlType::ConstantSensibleHeatRatio;
    assert_rejected_transactionally(
        &mut selector_runtime,
        &wrong_selector,
        predecessor,
        |error| {
            matches!(
                error,
                Error::DehumidificationControlTypeOutsideDirectSubset { .. }
            )
        },
    );

    let mut initialization_runtime = runtime;
    initialization_runtime
        .units
        .get_mut(&system.id)
        .expect("known unit")
        .topology_completed = false;
    assert_rejected_transactionally(&mut initialization_runtime, &system, predecessor, |error| {
        matches!(error, Error::InitializationNotReady { .. })
    });
}

#[test]
fn supplied_latest_and_witness_predecessor_corruption_is_transactional() {
    let (runtime, system, predecessor) = completed_cp391_case();

    let mut supplied_runtime = runtime.clone();
    let mut forged_argument = predecessor;
    forged_argument.source = "forged CP391 source";
    assert_rejected_transactionally(&mut supplied_runtime, &system, forged_argument, |error| {
        matches!(error, Error::PredecessorOutsideDirectSubset { .. })
    });

    let mut latest_runtime = runtime.clone();
    latest_runtime
        .units
        .get_mut(&system.id)
        .expect("known unit")
        .calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit
        .latest
        .as_mut()
        .expect("CP391 latest")
        .source = "forged CP391 latest source";
    assert_rejected_transactionally(&mut latest_runtime, &system, predecessor, |error| {
        matches!(
                error,
                Error::CoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitSnapshotMismatch { .. }
            )
    });

    let mut witness_runtime = runtime;
    let mut witness = witness_runtime
        .cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_latest_witness(system.id)
        .expect("CP391 witness");
    witness.source = "forged CP391 witness source";
    witness_runtime.set_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_latest_witness(
        system.id,
        witness,
    );
    assert_rejected_transactionally(&mut witness_runtime, &system, predecessor, |error| {
        matches!(
                error,
                Error::CoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitSnapshotMismatch { .. }
            )
    });
}

#[test]
fn call_order_and_replay_failures_preserve_cp391_state_and_witness() {
    let (runtime, system, predecessor) = completed_cp391_case();

    let mut call_order_runtime = runtime.clone();
    call_order_runtime
        .units
        .get_mut(&system.id)
        .expect("known unit")
        .calc_entry
        .call_count += 1;
    assert_rejected_transactionally(&mut call_order_runtime, &system, predecessor, |error| {
        matches!(
            error,
            Error::CoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitSnapshotMismatch { .. }
                | Error::PredecessorCallOrder { .. }
                | Error::RuntimeStateInvariantViolation { .. }
        )
    });

    let mut replay_runtime = runtime;
    let released = advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment(
        &mut replay_runtime,
        &system,
        predecessor,
    )
    .expect("first CP392 release");
    assert!(cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment_snapshot_is_exact_direct_release(
        released,
    ));
    assert_rejected_transactionally(&mut replay_runtime, &system, predecessor, |error| {
        matches!(
            error,
            Error::PredecessorCallOrder { .. } | Error::RuntimeStateInvariantViolation { .. }
        )
    });
}

#[test]
fn public_release_counter_overflow_preserves_cp391_state_and_witness() {
    let (mut runtime, system, predecessor) = completed_cp391_case();
    runtime
        .units
        .get_mut(&system.id)
        .expect("known unit")
        .calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment
        .transition_count = usize::MAX;

    assert_rejected_transactionally(&mut runtime, &system, predecessor, |error| {
        matches!(error, Error::RuntimeStateInvariantViolation { .. })
    });
}

fn completed_cp391_case() -> (PurchasedAirRuntimeState, IdealLoadsAirSystem, Cp391) {
    let (mut runtime, system, cp382) = completed_cp382_case_for_cp384_test(false);
    let cp383 = advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard(
        &mut runtime,
        &system,
        cp382,
    )
    .expect("CP383");
    let cp384 = advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment(
        &mut runtime,
        &system,
        cp383,
    )
    .expect("CP384");
    let cp385 = advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment(
        &mut runtime,
        &system,
        cp384,
    )
    .expect("CP385");
    let cp386 = advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_control_switch(
        &mut runtime,
        &system,
        cp385,
    )
    .expect("CP386");
    let cp387 = advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment(
        &mut runtime,
        &system,
        cp386,
    )
    .expect("CP387");
    let cp388 = advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment(
        &mut runtime,
        &system,
        cp387,
    )
    .expect("CP388");
    let cp389 = advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment(
        &mut runtime,
        &system,
        cp388,
    )
    .expect("CP389");
    let cp390 = advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit(
        &mut runtime,
        &system,
        cp389,
    )
    .expect("CP390");
    let cp391 = advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit(
        &mut runtime,
        &system,
        cp390,
    )
    .expect("CP391");
    (runtime, system, cp391)
}

fn assert_rejected_transactionally(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Cp391,
    expected: impl FnOnce(&Error) -> bool,
) {
    let selected = predecessor.system;
    let before_state = runtime
        .units
        .get(&selected)
        .expect("known unit")
        .calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment
        .clone();
    let before_witness = runtime.cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment_latest_witness(selected);

    let error = advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment(
        runtime,
        system,
        predecessor,
    )
    .expect_err("CP392 release must fail");
    assert!(expected(&error), "unexpected CP392 error: {error:?}");

    let after_state = &runtime
        .units
        .get(&selected)
        .expect("known unit")
        .calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment;
    assert_eq!(after_state, &before_state);
    let after_witness = runtime.cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment_latest_witness(selected);
    assert_witness_unchanged(before_witness, after_witness);
}

fn assert_witness_unchanged(
    before: Option<
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyHumidityRatioAssignmentSnapshot,
    >,
    after: Option<
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyHumidityRatioAssignmentSnapshot,
    >,
) {
    assert_eq!(
        before.is_some(),
        after.is_some(),
        "CP392 witness presence changed after rejected release"
    );
    if let (Some(before), Some(after)) = (before, after) {
        assert!(cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment_snapshots_match_bit_exact(
            before,
            after,
        ));
    }
}
