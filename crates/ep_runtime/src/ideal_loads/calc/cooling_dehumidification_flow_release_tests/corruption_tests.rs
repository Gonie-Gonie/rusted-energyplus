use ep_model::DehumidificationControlType;

use crate::ideal_loads::{
    PurchasedAirCalcCoolingDehumidificationFlowError,
    advance_direct_no_oa_calc_cooling_dehumidification_flow,
};

use super::release_case;

#[test]
fn forged_predecessor_and_replay_are_rejected_without_mutation() {
    let (runtime, system, predecessor) = release_case(-1_000.0);
    let mut forged = predecessor;
    forged.parent_call_ordinal += 1;
    let mut forged_runtime = runtime.clone();
    let before_forged = forged_runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_dehumidification_flow(
            &mut forged_runtime,
            &system,
            forged,
        )
        .is_err()
    );
    assert_eq!(forged_runtime, before_forged);

    let mut replay_runtime = runtime;
    advance_direct_no_oa_calc_cooling_dehumidification_flow(
        &mut replay_runtime,
        &system,
        predecessor,
    )
    .expect("first CP319 call");
    let before_replay = replay_runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_dehumidification_flow(
            &mut replay_runtime,
            &system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(replay_runtime, before_replay);
}

#[test]
fn humidistat_model_is_rejected_transactionally_without_live_service_input() {
    let (mut runtime, mut system, predecessor) = release_case(-1_000.0);
    system.dehumidification_control_type = DehumidificationControlType::Humidistat;
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_dehumidification_flow(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before);
}

#[test]
fn constant_sensible_heat_ratio_model_is_rejected_transactionally_by_none_guard() {
    let (mut runtime, mut system, predecessor) = release_case(-1_000.0);
    system.dehumidification_control_type = DehumidificationControlType::ConstantSensibleHeatRatio;
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_dehumidification_flow(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before);
}

#[test]
fn retained_count_corruption_fails_before_any_cp319_mutation() {
    let (mut runtime, system, predecessor) = release_case(-1_000.0);
    runtime
        .units
        .get_mut(&system.id)
        .expect("selected unit")
        .calc_cooling_dehumidification_flow
        .transition_count = usize::MAX;
    let before = runtime.clone();
    assert!(matches!(
        advance_direct_no_oa_calc_cooling_dehumidification_flow(&mut runtime, &system, predecessor,),
        Err(PurchasedAirCalcCoolingDehumidificationFlowError::PredecessorCallOrder { .. })
            | Err(
                PurchasedAirCalcCoolingDehumidificationFlowError::RuntimeStateInvariantViolation { .. }
            )
    ));
    assert_eq!(runtime, before);
}

#[test]
fn completed_cp318_source_counter_corruption_fails_transactionally() {
    let (mut runtime, system, predecessor) = release_case(-1_000.0);
    runtime
        .units
        .get_mut(&system.id)
        .expect("selected unit")
        .calc_cooling_sensible_flow
        .cp_air_assignment_count += 1;
    let before = runtime.clone();
    assert!(matches!(
        advance_direct_no_oa_calc_cooling_dehumidification_flow(&mut runtime, &system, predecessor,),
        Err(
            PurchasedAirCalcCoolingDehumidificationFlowError::RuntimeStateInvariantViolation { .. }
        )
    ));
    assert_eq!(runtime, before);
}

#[test]
fn exact_release_validator_rejects_forged_reset_and_provenance() {
    let (mut runtime, system, predecessor) = release_case(-1_000.0);
    let snapshot =
        advance_direct_no_oa_calc_cooling_dehumidification_flow(&mut runtime, &system, predecessor)
            .expect("exact CP319 call");

    let mut forged_reset = snapshot;
    forged_reset.reset_supply_mass_flow_rate_for_dehumidification_kg_per_s = Some(-0.0);
    assert!(
        !super::super::cooling_dehumidification_flow_snapshot_is_exact_direct_release(forged_reset)
    );

    let mut forged_provenance = snapshot;
    forged_provenance.source_order = &[];
    assert!(
        !super::super::cooling_dehumidification_flow_snapshot_is_exact_direct_release(
            forged_provenance
        )
    );
}
