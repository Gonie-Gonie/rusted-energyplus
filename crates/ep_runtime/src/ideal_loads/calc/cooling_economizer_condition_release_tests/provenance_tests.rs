use super::*;

#[test]
fn public_condition_rejects_stale_mixed_route_latest_transactionally() {
    let (mut runtime, system, non_cooling_predecessor) = release_fixture_with_cooling_demand(1.0);
    let stale_non_cooling = advance_direct_no_oa_calc_cooling_economizer_condition(
        &mut runtime,
        &system,
        non_cooling_predecessor,
    )
    .expect("first non-cooling CP316 call");
    assert!(stale_non_cooling.non_cooling_skipped);

    let no_economizer_predecessor = advance_subsequent_fixture_call(&mut runtime, &system, -1.0);
    let latest_no_economizer = advance_direct_no_oa_calc_cooling_economizer_condition(
        &mut runtime,
        &system,
        no_economizer_predecessor,
    )
    .expect("second no-economizer CP316 call");
    assert!(latest_no_economizer.no_economizer_outer_guard_fallthrough_skipped);

    let pending_predecessor = advance_subsequent_fixture_call(&mut runtime, &system, -1.0);
    let mut forged_stale_latest = stale_non_cooling;
    forged_stale_latest.parent_call_ordinal = 2;
    runtime
        .units
        .get_mut(&SYSTEM)
        .expect("selected unit")
        .calc_cooling_economizer_condition
        .latest = Some(forged_stale_latest);

    assert_runtime_invariant_without_mutation(runtime, &system, pending_predecessor);
}

#[test]
fn public_condition_rejects_whole_state_generation_replay_transactionally() {
    let (mut runtime, system, non_cooling_predecessor) = release_fixture_with_cooling_demand(1.0);
    advance_direct_no_oa_calc_cooling_economizer_condition(
        &mut runtime,
        &system,
        non_cooling_predecessor,
    )
    .expect("first non-cooling CP316 call");
    let stale_non_cooling_state = runtime
        .units
        .get(&SYSTEM)
        .expect("selected unit")
        .calc_cooling_economizer_condition
        .clone();

    let no_economizer_predecessor = advance_subsequent_fixture_call(&mut runtime, &system, -1.0);
    advance_direct_no_oa_calc_cooling_economizer_condition(
        &mut runtime,
        &system,
        no_economizer_predecessor,
    )
    .expect("second no-economizer CP316 call");
    let pending_predecessor = advance_subsequent_fixture_call(&mut runtime, &system, -1.0);

    let state = &mut runtime
        .units
        .get_mut(&SYSTEM)
        .expect("selected unit")
        .calc_cooling_economizer_condition;
    *state = stale_non_cooling_state;
    state.transition_count = 2;
    state.non_cooling_skip_count = 1;
    state.no_economizer_outer_guard_fallthrough_skip_count = 1;
    state
        .latest
        .as_mut()
        .expect("restored stale latest")
        .parent_call_ordinal = 2;

    assert_runtime_invariant_without_mutation(runtime, &system, pending_predecessor);
}

#[test]
fn public_condition_rejects_alternate_history_guard_and_condition_splice_transactionally() {
    let (mut target, donor, system, target_pending) = alternate_history_pending_fixture();
    let donor_unit = donor.units.get(&SYSTEM).expect("donor selected unit");
    let target_unit = target.units.get_mut(&SYSTEM).expect("target selected unit");
    target_unit.calc_cooling_economizer_guard = donor_unit.calc_cooling_economizer_guard.clone();
    target_unit.calc_cooling_economizer_condition =
        donor_unit.calc_cooling_economizer_condition.clone();

    assert_runtime_invariant_without_mutation(target, &system, target_pending);
}

#[test]
fn public_condition_rejects_alternate_history_whole_unit_transplant_transactionally() {
    let (mut target, mut donor, system, target_pending) = alternate_history_pending_fixture();
    let donor_unit = donor.units.remove(&SYSTEM).expect("donor selected unit");
    target.units.insert(SYSTEM, donor_unit);

    assert_runtime_invariant_without_mutation(target, &system, target_pending);
}

fn alternate_history_pending_fixture() -> (
    PurchasedAirRuntimeState,
    PurchasedAirRuntimeState,
    IdealLoadsAirSystem,
    PurchasedAirCalcCoolingEconomizerGuardSnapshot,
) {
    let (mut target, system, target_non_cooling) = release_fixture_with_cooling_demand(1.0);
    advance_direct_no_oa_calc_cooling_economizer_condition(
        &mut target,
        &system,
        target_non_cooling,
    )
    .expect("target non-cooling CP316 call");
    let target_no_economizer = advance_subsequent_fixture_call(&mut target, &system, -1.0);
    advance_direct_no_oa_calc_cooling_economizer_condition(
        &mut target,
        &system,
        target_no_economizer,
    )
    .expect("target no-economizer CP316 call");
    let target_pending = advance_subsequent_fixture_call(&mut target, &system, -1.0);

    let (mut donor, donor_system, donor_no_economizer) = release_fixture_with_cooling_demand(-1.0);
    advance_direct_no_oa_calc_cooling_economizer_condition(
        &mut donor,
        &donor_system,
        donor_no_economizer,
    )
    .expect("donor no-economizer CP316 call");
    let donor_non_cooling = advance_subsequent_fixture_call(&mut donor, &donor_system, 1.0);
    advance_direct_no_oa_calc_cooling_economizer_condition(
        &mut donor,
        &donor_system,
        donor_non_cooling,
    )
    .expect("donor non-cooling CP316 call");
    let donor_pending = advance_subsequent_fixture_call(&mut donor, &donor_system, -1.0);
    assert_eq!(
        donor_pending, target_pending,
        "both histories must expose the same third pending NoEconomizer predecessor"
    );
    (target, donor, system, target_pending)
}

fn assert_runtime_invariant_without_mutation(
    mut runtime: PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: PurchasedAirCalcCoolingEconomizerGuardSnapshot,
) {
    let before = runtime.clone();
    assert_eq!(
        advance_direct_no_oa_calc_cooling_economizer_condition(&mut runtime, system, predecessor,),
        Err(
            PurchasedAirCalcCoolingEconomizerConditionError::RuntimeStateInvariantViolation {
                system: SYSTEM,
            },
        ),
    );
    assert_eq!(runtime, before);
}
