//! CP396 public-release lifecycle and transactional tests.

use super::*;
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment::tests::release_corruption::completed_cp391_case;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatSupplyHumidityRatioAssignmentSnapshot as Cp395,
    PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_break,
    advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment,
    advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry,
    advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment,
};
use ep_model::{DehumidificationControlType, IdealLoadsAirSystem, IdealLoadsAirSystemId};

#[test]
fn direct_release_skips_break_and_retains_lifecycle_metadata() {
    let (mut runtime, system, predecessor) = completed_cp395_case();
    let snapshot = advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_break(
        &mut runtime,
        &system,
        predecessor,
    )
    .expect("CP396 direct release");
    assert!(
        cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_break_snapshot_is_exact_direct_release(
            snapshot,
        )
    );
    assert!(!snapshot.dehumidification_control_humidistat_case_exited_via_break);
    assert_eq!(
        snapshot.resulting_supply_humidity_ratio.map(f64::to_bits),
        predecessor
            .resulting_supply_humidity_ratio
            .map(f64::to_bits),
    );
    assert_eq!(
        snapshot
            .resulting_supply_enthalpy_j_per_kg
            .map(f64::to_bits),
        predecessor
            .resulting_supply_enthalpy_j_per_kg
            .map(f64::to_bits),
    );
    assert_eq!(
        snapshot.resulting_supply_temperature_c.map(f64::to_bits),
        predecessor.resulting_supply_temperature_c.map(f64::to_bits),
    );

    let summary = purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_break_lifecycle_summary(
        &runtime,
        system.id,
    )
    .expect("CP396 lifecycle");
    assert_eq!(summary.state.latest, Some(snapshot));
    assert_eq!(summary.state.transition_count, 1);
    assert_eq!(summary.state.inactive_transition_count, 1);
    assert_eq!(
        summary
            .state
            .dehumidification_control_humidistat_case_break_count,
        0
    );
    assert_eq!(summary.state.source_site_execution_count, 0);
}

#[test]
fn identity_selector_predecessor_replay_and_overflow_fail_transactionally() {
    let (runtime, system, predecessor) = completed_cp395_case();

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
    forged.source = "forged CP395 source";
    assert_rejected_unchanged(&mut forged_runtime, &system, forged);

    let mut overflow_runtime = runtime.clone();
    overflow_runtime
        .units
        .get_mut(&system.id)
        .expect("selected unit")
        .calc_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_break
        .transition_count = usize::MAX;
    assert_rejected_unchanged(&mut overflow_runtime, &system, predecessor);

    let mut replay_runtime = runtime;
    assert!(
        advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_break(
            &mut replay_runtime,
            &system,
            predecessor,
        )
        .is_ok()
    );
    assert_rejected_unchanged(&mut replay_runtime, &system, predecessor);
}

#[test]
fn cp396_bounded_prefix_rejects_cp395_local_corruption_transactionally() {
    let (runtime, system, predecessor) = completed_cp395_case();

    let mut bad_counter = runtime.clone();
    bad_counter
        .units
        .get_mut(&system.id)
        .expect("selected unit")
        .calc_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment
        .inactive_transition_count += 1;
    assert_rejected_unchanged(&mut bad_counter, &system, predecessor);

    let mut bad_route = runtime.clone();
    bad_route
        .units
        .get_mut(&system.id)
        .expect("selected unit")
        .calc_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment
        .predecessor_route_counts[0] += 1;
    assert_rejected_unchanged(&mut bad_route, &system, predecessor);

    let mut bad_latest = runtime.clone();
    bad_latest
        .units
        .get_mut(&system.id)
        .expect("selected unit")
        .calc_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment
        .latest
        .as_mut()
        .expect("CP395 latest")
        .source = "forged CP395 latest source";
    assert_rejected_unchanged(&mut bad_latest, &system, predecessor);

    let mut bad_witness = runtime.clone();
    let mut forged_witness = predecessor;
    forged_witness.source = "forged CP395 witness source";
    bad_witness.set_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment_latest_witness(
        system.id,
        forged_witness,
    );
    assert_rejected_unchanged(&mut bad_witness, &system, predecessor);

    let mut bad_argument_runtime = runtime;
    let mut bad_argument = predecessor;
    bad_argument.source = "forged passed CP395 source";
    assert_rejected_unchanged(&mut bad_argument_runtime, &system, bad_argument);
}

pub(in crate::ideal_loads::calc) fn completed_cp395_case()
-> (PurchasedAirRuntimeState, IdealLoadsAirSystem, Cp395) {
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
    let cp395 = advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment(
        &mut runtime,
        &system,
        cp394,
    )
    .expect("CP395");
    (runtime, system, cp395)
}

fn assert_rejected_unchanged(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Cp395,
) {
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_break(
            runtime,
            system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(*runtime, before);
}
