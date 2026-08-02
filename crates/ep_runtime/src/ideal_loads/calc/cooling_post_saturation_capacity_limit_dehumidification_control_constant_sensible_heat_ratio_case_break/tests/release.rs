//! CP393 public-release lifecycle and transactional tests.

use super::*;
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment::tests::release_corruption::completed_cp391_case;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyHumidityRatioAssignmentSnapshot as Cp392,
    PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment,
};
use ep_model::{IdealLoadsAirSystem, IdealLoadsAirSystemId};

#[test]
fn direct_release_skips_break_and_retains_lifecycle_metadata() {
    let (mut runtime, system, predecessor) = completed_cp392_case();
    let snapshot = advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_break(
        &mut runtime,
        &system,
        predecessor,
    )
    .expect("CP393 direct release");
    assert!(cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_break_snapshot_is_exact_direct_release(snapshot));
    assert!(!snapshot.dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break);
    let summary = purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_break_lifecycle_summary(
        &runtime,
        system.id,
    )
    .expect("CP393 lifecycle");
    assert_eq!(summary.state.latest, Some(snapshot));
    assert_eq!(summary.state.source_site_execution_count, 0);
}

#[test]
fn identity_predecessor_replay_and_overflow_fail_transactionally() {
    let (runtime, system, predecessor) = completed_cp392_case();

    let mut wrong_identity_runtime = runtime.clone();
    let mut wrong_identity = system.clone();
    wrong_identity.id = IdealLoadsAirSystemId(system.id.0 + 1);
    assert_rejected_unchanged(&mut wrong_identity_runtime, &wrong_identity, predecessor);

    let mut forged_runtime = runtime.clone();
    let mut forged = predecessor;
    forged.source = "forged CP392 source";
    assert_rejected_unchanged(&mut forged_runtime, &system, forged);

    let mut overflow_runtime = runtime.clone();
    overflow_runtime
        .units
        .get_mut(&system.id)
        .expect("selected unit")
        .calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_break
        .transition_count = usize::MAX;
    assert_rejected_unchanged(&mut overflow_runtime, &system, predecessor);

    let mut replay_runtime = runtime;
    assert!(advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_break(
        &mut replay_runtime,
        &system,
        predecessor,
    )
    .is_ok());
    assert_rejected_unchanged(&mut replay_runtime, &system, predecessor);
}

fn completed_cp392_case() -> (PurchasedAirRuntimeState, IdealLoadsAirSystem, Cp392) {
    let (mut runtime, system, cp391) = completed_cp391_case();
    let cp392 = advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment(
        &mut runtime,
        &system,
        cp391,
    )
    .expect("CP392");
    (runtime, system, cp392)
}

fn assert_rejected_unchanged(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Cp392,
) {
    let before = runtime.clone();
    assert!(advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_break(
        runtime,
        system,
        predecessor,
    )
    .is_err());
    assert_eq!(*runtime, before);
}
