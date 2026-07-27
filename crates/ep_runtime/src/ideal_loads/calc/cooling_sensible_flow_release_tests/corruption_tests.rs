use ep_model::ZoneId;

use crate::ideal_loads::{
    PurchasedAirCalcCoolingSensibleFlowActiveInput, PurchasedAirCalcCoolingSensibleFlowError,
    advance_direct_no_oa_calc_cooling_sensible_flow,
};

use super::release_case;

#[test]
fn forged_predecessor_and_replay_are_rejected_without_mutation() {
    let (runtime, system, predecessor, zone_state) = release_case(-1_000.0);
    let mut forged = predecessor;
    forged.parent_call_ordinal += 1;
    let mut forged_runtime = runtime.clone();
    let before_forged = forged_runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_sensible_flow(
            &mut forged_runtime,
            &system,
            forged,
            &zone_state,
        )
        .is_err()
    );
    assert_eq!(forged_runtime, before_forged);

    let mut replay_runtime = runtime;
    advance_direct_no_oa_calc_cooling_sensible_flow(
        &mut replay_runtime,
        &system,
        predecessor,
        &zone_state,
    )
    .expect("first CP318 call");
    let before_replay = replay_runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_sensible_flow(
            &mut replay_runtime,
            &system,
            predecessor,
            &zone_state,
        )
        .is_err()
    );
    assert_eq!(replay_runtime, before_replay);
}

#[test]
fn zone_identity_and_active_nonfinite_inputs_fail_transactionally() {
    let (runtime, system, predecessor, zone_state) = release_case(-1_000.0);

    let mut wrong_zone = zone_state.clone();
    wrong_zone.zone_id = ZoneId(zone_state.zone_id.0 + 1);
    let mut wrong_zone_runtime = runtime.clone();
    let before_wrong_zone = wrong_zone_runtime.clone();
    assert!(matches!(
        advance_direct_no_oa_calc_cooling_sensible_flow(
            &mut wrong_zone_runtime,
            &system,
            predecessor,
            &wrong_zone,
        ),
        Err(PurchasedAirCalcCoolingSensibleFlowError::ZoneIdentityMismatch { .. })
    ));
    assert_eq!(wrong_zone_runtime, before_wrong_zone);

    let mut nonfinite_zone = zone_state;
    nonfinite_zone.air_humidity_ratio = f64::NAN;
    let mut nonfinite_runtime = runtime;
    let before_nonfinite = nonfinite_runtime.clone();
    assert!(matches!(
        advance_direct_no_oa_calc_cooling_sensible_flow(
            &mut nonfinite_runtime,
            &system,
            predecessor,
            &nonfinite_zone,
        ),
        Err(
            PurchasedAirCalcCoolingSensibleFlowError::NonFiniteActiveInput {
                input: PurchasedAirCalcCoolingSensibleFlowActiveInput::ZoneHumidityRatio,
                ..
            }
        )
    ));
    assert_eq!(nonfinite_runtime, before_nonfinite);
}

#[test]
fn retained_count_corruption_fails_before_any_cp318_mutation() {
    let (mut runtime, system, predecessor, zone_state) = release_case(-1_000.0);
    runtime
        .units
        .get_mut(&system.id)
        .expect("selected unit")
        .calc_cooling_sensible_flow
        .transition_count = usize::MAX;
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_sensible_flow(
            &mut runtime,
            &system,
            predecessor,
            &zone_state,
        )
        .is_err()
    );
    assert_eq!(runtime, before);
}

#[test]
fn exact_release_validator_rejects_forged_arithmetic_and_provenance() {
    let (mut runtime, system, predecessor, zone_state) = release_case(-1_000.0);
    let snapshot = advance_direct_no_oa_calc_cooling_sensible_flow(
        &mut runtime,
        &system,
        predecessor,
        &zone_state,
    )
    .expect("exact CP318 call");

    let mut forged_arithmetic = snapshot;
    forged_arithmetic.assigned_supply_mass_flow_rate_for_cool_kg_per_s = forged_arithmetic
        .assigned_supply_mass_flow_rate_for_cool_kg_per_s
        .map(|value| value + 1.0);
    assert!(
        !super::super::cooling_sensible_flow_snapshot_is_exact_direct_release(forged_arithmetic)
    );

    let mut forged_provenance = snapshot;
    forged_provenance.source_order = &[];
    assert!(
        !super::super::cooling_sensible_flow_snapshot_is_exact_direct_release(forged_provenance)
    );
}
