//! CP398 public-release lifecycle and transactional tests.

use super::*;
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment::tests::release_corruption::completed_cp391_case;
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment::tests::fixtures as cp395_fixtures;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatCaseBreakRuntimeState as Cp396State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlNoneCaseEntryRuntimeState as Cp397State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlNoneCaseEntrySnapshot as Cp397,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatSupplyHumidityRatioAssignmentRuntimeState as Cp395State,
    PurchasedAirRuntimeState,
};
use crate::ideal_loads::calc::{
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_break_state,
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_none_case_entry_state,
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment_state,
};
use ep_model::{DehumidificationControlType, IdealLoadsAirSystem, IdealLoadsAirSystemId};

#[test]
fn direct_release_enters_shared_case_via_none_fallthrough_and_retains_metadata() {
    let (mut runtime, system, predecessor) = completed_cp397_case();
    let snapshot = advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry(
        &mut runtime,
        &system,
        predecessor,
    )
    .expect("CP398 direct release");
    assert!(
        cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry_snapshot_is_exact_direct_release(
            snapshot,
        )
    );
    assert!(snapshot.predecessor_dehumidification_control_none_case_entered);
    assert!(
        snapshot
            .dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered
    );
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

    let summary = purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry_lifecycle_summary(
        &runtime,
        system.id,
    )
    .expect("CP398 lifecycle");
    assert_eq!(summary.state.latest, Some(snapshot));
    assert_eq!(summary.state.transition_count, 1);
    assert_eq!(summary.state.inactive_transition_count, 0);
    assert_eq!(
        summary
            .state
            .dehumidification_control_constant_supply_humidity_ratio_case_entry_count,
        1
    );
    assert_eq!(summary.state.source_site_execution_count, 1);
}

#[test]
fn identity_selector_predecessor_replay_and_overflow_fail_transactionally() {
    let (runtime, system, predecessor) = completed_cp397_case();

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
    forged.source = "forged CP397 source";
    assert_rejected_unchanged(&mut forged_runtime, &system, forged);

    let mut overflow_runtime = runtime.clone();
    overflow_runtime
        .units
        .get_mut(&system.id)
        .expect("selected unit")
        .calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry
        .transition_count = usize::MAX;
    assert_rejected_unchanged(&mut overflow_runtime, &system, predecessor);

    let mut replay_runtime = runtime;
    assert!(
        advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry(
            &mut replay_runtime,
            &system,
            predecessor,
        )
        .is_ok()
    );
    assert_rejected_unchanged(&mut replay_runtime, &system, predecessor);
}

#[test]
fn cp398_bounded_prefix_rejects_cp397_local_corruption_transactionally() {
    let (runtime, system, predecessor) = completed_cp397_case();

    let mut bad_counter = runtime.clone();
    bad_counter
        .units
        .get_mut(&system.id)
        .expect("selected unit")
        .calc_cooling_post_saturation_capacity_limit_dehumidification_control_none_case_entry
        .inactive_transition_count += 1;
    assert_rejected_unchanged(&mut bad_counter, &system, predecessor);

    let mut bad_route = runtime.clone();
    bad_route
        .units
        .get_mut(&system.id)
        .expect("selected unit")
        .calc_cooling_post_saturation_capacity_limit_dehumidification_control_none_case_entry
        .predecessor_route_counts[0] += 1;
    assert_rejected_unchanged(&mut bad_route, &system, predecessor);

    let mut bad_latest = runtime.clone();
    bad_latest
        .units
        .get_mut(&system.id)
        .expect("selected unit")
        .calc_cooling_post_saturation_capacity_limit_dehumidification_control_none_case_entry
        .latest
        .as_mut()
        .expect("CP397 latest")
        .source = "forged CP397 latest source";
    assert_rejected_unchanged(&mut bad_latest, &system, predecessor);

    let mut bad_witness = runtime.clone();
    let mut forged_witness = predecessor;
    forged_witness.source = "forged CP397 witness source";
    bad_witness.set_cooling_post_saturation_capacity_limit_dehumidification_control_none_case_entry_latest_witness(
        system.id,
        forged_witness,
    );
    assert_rejected_unchanged(&mut bad_witness, &system, predecessor);

    let mut bad_argument_runtime = runtime;
    let mut bad_argument = predecessor;
    bad_argument.source = "forged passed CP397 source";
    assert_rejected_unchanged(&mut bad_argument_runtime, &system, bad_argument);
}

pub(in crate::ideal_loads::calc) fn completed_cp397_case()
-> (PurchasedAirRuntimeState, IdealLoadsAirSystem, Cp397) {
    let (mut runtime, system, _) = completed_cp391_case();
    let unit = runtime.units.get(&system.id).expect("selected unit");
    let ordinal = unit.init_call_count;
    let controlled_zone = unit.controlled_zone.expect("controlled zone");
    let mut cp394 = cp395_fixtures::chain(
        3,
        1,
        true,
        Some(DehumidificationControlType::None),
        ordinal,
        0.7,
        18.0,
        1.0,
    )
    .cp394;
    cp394.system = system.id;
    cp394.parent_call_ordinal = ordinal;
    cp394.controlled_zone = controlled_zone;

    let mut cp395_state = Cp395State::new(system.id);
    let cp395 = advance_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment_state(
        &mut cp395_state,
        cp394,
    )
    .expect("active route-20 CP395");
    let mut cp396_state = Cp396State::new(system.id);
    let cp396 = advance_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_break_state(
        &mut cp396_state,
        cp395,
    )
    .expect("active route-20 CP396");
    let mut cp397_state = Cp397State::new(system.id);
    let cp397 = advance_cooling_post_saturation_capacity_limit_dehumidification_control_none_case_entry_state(
        &mut cp397_state,
        cp396,
    )
    .expect("active route-20 CP397");

    let unit = runtime.units.get_mut(&system.id).expect("selected unit");
    unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment = cp395_state;
    unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_break = cp396_state;
    unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_none_case_entry = cp397_state;
    runtime.set_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment_latest_witness(system.id, cp395);
    runtime.set_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_break_latest_witness(system.id, cp396);
    runtime.set_cooling_post_saturation_capacity_limit_dehumidification_control_none_case_entry_latest_witness(system.id, cp397);
    (runtime, system, cp397)
}

fn assert_rejected_unchanged(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Cp397,
) {
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry(
            runtime,
            system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(*runtime, before);
}
