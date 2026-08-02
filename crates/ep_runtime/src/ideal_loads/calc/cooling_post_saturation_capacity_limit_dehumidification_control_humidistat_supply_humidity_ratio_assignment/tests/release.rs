//! CP395 public-release lifecycle and transactional tests.

use super::*;
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment::tests::release_corruption::completed_cp391_case;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatCaseEntrySnapshot as Cp394,
    PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_break,
    advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment,
    advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry,
};
use ep_model::{DehumidificationControlType, IdealLoadsAirSystem, IdealLoadsAirSystemId};

#[test]
fn direct_release_skips_humidistat_assignment_and_retains_lifecycle_metadata() {
    let (mut runtime, system, predecessor) = completed_cp394_case();
    let snapshot = advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment(
        &mut runtime,
        &system,
        predecessor,
    )
    .expect("CP395 direct release");
    assert!(
        cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment_snapshot_is_exact_direct_release(
            snapshot,
        )
    );
    assert!(
        !snapshot.dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed
    );
    assert_eq!(snapshot.resulting_supply_humidity_ratio, None);

    let summary = purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment_lifecycle_summary(
        &runtime,
        system.id,
    )
    .expect("CP395 lifecycle");
    assert_eq!(summary.state.latest, Some(snapshot));
    assert_eq!(summary.state.transition_count, 1);
    assert_eq!(summary.state.inactive_transition_count, 1);
    assert_eq!(summary.state.source_site_execution_count, 0);
    assert_eq!(
        runtime.cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment_latest_witness(system.id),
        Some(snapshot),
    );
}

#[test]
fn identity_selector_predecessor_replay_and_overflow_fail_transactionally() {
    let (runtime, system, predecessor) = completed_cp394_case();

    let mut wrong_identity_runtime = runtime.clone();
    let mut wrong_identity = system.clone();
    wrong_identity.id = IdealLoadsAirSystemId(system.id.0 + 1);
    assert_rejected_unchanged(&mut wrong_identity_runtime, &wrong_identity, predecessor);

    let mut selector_runtime = runtime.clone();
    let mut wrong_selector = system.clone();
    wrong_selector.dehumidification_control_type = DehumidificationControlType::Humidistat;
    assert_rejected_unchanged(&mut selector_runtime, &wrong_selector, predecessor);

    let mut forged_runtime = runtime.clone();
    let mut forged = predecessor;
    forged.source = "forged CP394 source";
    assert_rejected_unchanged(&mut forged_runtime, &system, forged);

    let mut overflow_runtime = runtime.clone();
    overflow_runtime
        .units
        .get_mut(&system.id)
        .expect("selected unit")
        .calc_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment
        .transition_count = usize::MAX;
    assert_rejected_unchanged(&mut overflow_runtime, &system, predecessor);

    let mut replay_runtime = runtime;
    assert!(
        advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment(
            &mut replay_runtime,
            &system,
            predecessor,
        )
        .is_ok()
    );
    assert_rejected_unchanged(&mut replay_runtime, &system, predecessor);
}

fn completed_cp394_case() -> (PurchasedAirRuntimeState, IdealLoadsAirSystem, Cp394) {
    let (mut runtime, system, cp391) = completed_cp391_case();
    let cp392 = advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment(
        &mut runtime,
        &system,
        cp391,
    )
    .expect("CP392");
    let cp393 = advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_break(
        &mut runtime,
        &system,
        cp392,
    )
    .expect("CP393");
    let cp394 = advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry(
        &mut runtime,
        &system,
        cp393,
    )
    .expect("CP394");
    (runtime, system, cp394)
}

fn assert_rejected_unchanged(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Cp394,
) {
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment(
            runtime,
            system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(*runtime, before);
}
