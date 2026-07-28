use super::super::*;
use crate::ideal_loads::calc::cooling_mixed_air_call::completed_direct_cooling_mixed_air_call_is_consistent;
use crate::ideal_loads::calc::cooling_mixed_air_call::release_tests::{
    install_completed_active_case_at_ordinal, install_completed_case_at_ordinal, release_case,
    release_case_with_demand,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingMixedAirCallSnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardError, PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_cooling_mixed_air_call,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_positive_guard,
    cooling_mixed_air_call_snapshot_is_exact_direct_release,
};

fn completed_cp329_case() -> (
    PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    PurchasedAirCalcCoolingMixedAirCallSnapshot,
) {
    let (mut runtime, system, predecessor, zone_state) = release_case();
    let mixed_air = advance_direct_no_oa_calc_cooling_mixed_air_call(
        &mut runtime,
        &system,
        predecessor,
        &zone_state,
    )
    .expect("CP329");
    (runtime, system, mixed_air)
}

fn completed_cp329_case_with_demand(
    cooling_demand_w: f64,
) -> (
    PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    PurchasedAirCalcCoolingMixedAirCallSnapshot,
) {
    let (mut runtime, system, predecessor, zone_state) = release_case_with_demand(cooling_demand_w);
    let mixed_air = advance_direct_no_oa_calc_cooling_mixed_air_call(
        &mut runtime,
        &system,
        predecessor,
        &zone_state,
    )
    .expect("CP329");
    (runtime, system, mixed_air)
}

fn swap_non_cooling_history_to_unit_off(
    runtime: &mut PurchasedAirRuntimeState,
    system: ep_model::IdealLoadsAirSystemId,
) {
    let state = &mut runtime
        .units
        .get_mut(&system)
        .expect("known unit")
        .calc_cooling_supply_mass_flow_positive_guard;
    assert_eq!(state.non_cooling_skip_count, 1);
    assert_eq!(state.unit_off_skip_count, 0);
    state.non_cooling_skip_count = 0;
    state.unit_off_skip_count = 1;
}

#[test]
fn public_release_commits_once_and_rejects_replay_without_mutation() {
    let (mut runtime, system, predecessor) = completed_cp329_case();
    let snapshot = advance_direct_no_oa_calc_cooling_supply_mass_flow_positive_guard(
        &mut runtime,
        &system,
        predecessor,
    )
    .expect("CP330");

    assert!(cooling_supply_mass_flow_positive_guard_snapshot_is_exact_direct_release(snapshot));
    let state = &runtime
        .units
        .get(&system.id)
        .expect("known unit")
        .calc_cooling_supply_mass_flow_positive_guard;
    assert_eq!(state.transition_count, 1);
    assert_eq!(state.cooling_body_entry_count, 1);
    assert_eq!(
        state.source_site_execution_count,
        2 + usize::from(snapshot.positive_supply_mass_flow_body_entered)
    );

    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_supply_mass_flow_positive_guard(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before);
}

#[test]
fn forged_cp329_snapshot_or_private_witness_fails_before_cp330_mutation() {
    let (runtime, system, predecessor) = completed_cp329_case();
    let mut forged_ordinal = predecessor;
    forged_ordinal.parent_call_ordinal += 1;
    let mut forged_source = predecessor;
    forged_source.source = "forged";
    let mut forged_supply = predecessor;
    forged_supply.supply_mass_flow_rate_kg_per_s = forged_supply
        .supply_mass_flow_rate_kg_per_s
        .map(|value| f64::from_bits(value.to_bits().wrapping_add(1)));

    for forged in [forged_ordinal, forged_source, forged_supply] {
        let mut case_runtime = runtime.clone();
        let before = case_runtime.clone();
        assert!(matches!(
            advance_direct_no_oa_calc_cooling_supply_mass_flow_positive_guard(
                &mut case_runtime,
                &system,
                forged,
            ),
            Err(
                PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardError::
                    CoolingMixedAirCallSnapshotMismatch { system: rejected }
            ) if rejected == system.id
        ));
        assert_eq!(case_runtime, before);
    }

    let mut case_runtime = runtime;
    let mut forged_witness = predecessor;
    forged_witness.source = "forged-private-witness";
    case_runtime.set_cooling_mixed_air_call_latest_witness(system.id, forged_witness);
    let before = case_runtime.clone();
    assert!(matches!(
        advance_direct_no_oa_calc_cooling_supply_mass_flow_positive_guard(
            &mut case_runtime,
            &system,
            predecessor,
        ),
        Err(
            PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardError::
                CoolingMixedAirCallSnapshotMismatch { system: rejected }
        ) if rejected == system.id
    ));
    assert_eq!(case_runtime, before);
}

#[test]
fn corrupted_cp328_private_witness_invalidates_cp329_and_cp330_admission() {
    let (mut runtime, system, predecessor) = completed_cp329_case();
    let mut forged_witness = runtime
        .cooling_supply_mass_flow_very_small_guard_body_latest_witness(system.id)
        .expect("completed CP328 witness");
    forged_witness.source = "forged-cp328-private-witness";
    runtime.set_cooling_supply_mass_flow_very_small_guard_body_latest_witness(
        system.id,
        forged_witness,
    );

    let unit = runtime.units.get(&system.id).expect("known unit");
    assert!(!completed_direct_cooling_mixed_air_call_is_consistent(
        &runtime,
        unit,
        &system,
        predecessor,
        runtime.cooling_mixed_air_call_latest_witness(system.id),
    ));

    let before = runtime.clone();
    assert_eq!(
        advance_direct_no_oa_calc_cooling_supply_mass_flow_positive_guard(
            &mut runtime,
            &system,
            predecessor,
        ),
        Err(
            PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardError::
                RuntimeStateInvariantViolation { system: system.id }
        )
    );
    assert_eq!(runtime, before);
}

#[test]
fn completed_cp329_counter_corruption_fails_before_cp330_mutation() {
    let (mut runtime, system, predecessor) = completed_cp329_case();
    runtime
        .units
        .get_mut(&system.id)
        .expect("known unit")
        .calc_cooling_mixed_air_call
        .supply_mass_flow_rate_read_count += 1;

    let before = runtime.clone();
    assert_eq!(
        advance_direct_no_oa_calc_cooling_supply_mass_flow_positive_guard(
            &mut runtime,
            &system,
            predecessor,
        ),
        Err(
            PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardError::
                RuntimeStateInvariantViolation { system: system.id }
        )
    );
    assert_eq!(runtime, before);
}

#[test]
fn active_source_site_increment_overflow_is_fail_closed_and_non_mutating() {
    for next_supply in [0.0, 1.0] {
        let (mut runtime, system, mut predecessor) = completed_cp329_case();
        predecessor.supply_mass_flow_rate_kg_per_s = Some(next_supply);
        predecessor.child_supply_mass_flow_rate_kg_per_s = Some(next_supply);
        predecessor.resulting_recirculation_mass_flow_rate_kg_per_s = Some(next_supply);
        assert!(cooling_mixed_air_call_snapshot_is_exact_direct_release(
            predecessor
        ));

        let prior_transition_count = (usize::MAX - 1) / 2;
        let next_ordinal = prior_transition_count + 1;
        predecessor =
            install_completed_active_case_at_ordinal(&mut runtime, predecessor, next_ordinal);
        {
            let predecessor_state = &mut runtime
                .units
                .get_mut(&system.id)
                .expect("known unit")
                .calc_cooling_mixed_air_call;
            predecessor_state.unit_off_skip_count = 0;
            predecessor_state.cooling_call_count = next_ordinal;
        }

        let mut prior_predecessor = predecessor;
        prior_predecessor.parent_call_ordinal = prior_transition_count;
        prior_predecessor.supply_mass_flow_rate_kg_per_s = Some(0.0);
        prior_predecessor.child_supply_mass_flow_rate_kg_per_s = Some(0.0);
        prior_predecessor.resulting_recirculation_mass_flow_rate_kg_per_s = Some(0.0);
        let mut seed_state =
            PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardRuntimeState::new(system.id);
        let prior_snapshot = advance_cooling_supply_mass_flow_positive_guard_state(
            &mut seed_state,
            prior_predecessor,
        );

        {
            let state = &mut runtime
                .units
                .get_mut(&system.id)
                .expect("known unit")
                .calc_cooling_supply_mass_flow_positive_guard;
            state.transition_count = prior_transition_count;
            state.cooling_body_entry_count = prior_transition_count;
            state.unit_off_skip_count = 0;
            state.non_cooling_skip_count = 0;
            state.source_site_execution_count = usize::MAX - 1;
            state.supply_mass_flow_rate_read_count = prior_transition_count;
            state.supply_mass_flow_rate_strictly_positive_comparison_count = prior_transition_count;
            state.positive_supply_mass_flow_body_entry_count = 0;
            state.active_guard_false_fallthrough_count = prior_transition_count;
            state.latest = Some(prior_snapshot);
            state.latest_route = Some(
                PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardRetainedRoute::
                    ActiveGuardFalseFallthrough,
            );
            state.latest_transition_ordinal = Some(prior_transition_count);
            state.witnessed_positive_supply_mass_flow_body_entry_count = 0;
            state.witnessed_active_guard_false_fallthrough_count = prior_transition_count;
        }
        runtime
            .set_cooling_supply_mass_flow_positive_guard_latest_witness(system.id, prior_snapshot);

        let unit = runtime.units.get(&system.id).expect("known unit");
        assert!(
            super::super::release::pending_positive_guard_state_is_consistent_for_test(
                unit,
                predecessor,
                runtime.cooling_supply_mass_flow_positive_guard_latest_witness(system.id),
            )
        );
        assert!(
            !super::super::release::next_positive_guard_transition_fits_for_test(unit, predecessor,)
        );

        let before = runtime.clone();
        assert_eq!(
            advance_direct_no_oa_calc_cooling_supply_mass_flow_positive_guard(
                &mut runtime,
                &system,
                predecessor,
            ),
            Err(
                PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardError::
                    RuntimeStateInvariantViolation { system: system.id }
            )
        );
        assert_eq!(runtime, before);
    }
}

#[test]
fn public_only_skipped_route_redistribution_fails_completed_and_pending_links_without_mutation() {
    let (mut runtime, system, predecessor) = completed_cp329_case_with_demand(1.0);
    assert!(predecessor.non_cooling_skipped);
    let snapshot = advance_direct_no_oa_calc_cooling_supply_mass_flow_positive_guard(
        &mut runtime,
        &system,
        predecessor,
    )
    .expect("CP330");
    assert!(snapshot.non_cooling_skipped);
    swap_non_cooling_history_to_unit_off(&mut runtime, system.id);

    let unit = runtime.units.get(&system.id).expect("known unit");
    assert!(
        !completed_direct_cooling_supply_mass_flow_positive_guard_is_consistent(
            &runtime,
            unit,
            &system,
            snapshot,
            runtime.cooling_supply_mass_flow_positive_guard_latest_witness(system.id),
        )
    );

    let next_predecessor = install_completed_case_at_ordinal(&mut runtime, predecessor, 2, 0, 2, 0);
    let unit = runtime.units.get(&system.id).expect("known unit");
    assert!(
        !super::super::release::pending_positive_guard_state_is_consistent_for_test(
            unit,
            next_predecessor,
            runtime.cooling_supply_mass_flow_positive_guard_latest_witness(system.id),
        )
    );

    let before = runtime.clone();
    assert_eq!(
        advance_direct_no_oa_calc_cooling_supply_mass_flow_positive_guard(
            &mut runtime,
            &system,
            next_predecessor,
        ),
        Err(
            PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardError::
                RuntimeStateInvariantViolation { system: system.id }
        )
    );
    assert_eq!(runtime, before);
}

#[test]
fn coordinated_public_cp330_and_cp329_corruption_cannot_bypass_cp328_chain() {
    let (mut runtime, system, predecessor) = completed_cp329_case_with_demand(1.0);
    advance_direct_no_oa_calc_cooling_supply_mass_flow_positive_guard(
        &mut runtime,
        &system,
        predecessor,
    )
    .expect("CP330");
    swap_non_cooling_history_to_unit_off(&mut runtime, system.id);

    let (_, _, active_predecessor) = completed_cp329_case();
    let next_predecessor =
        install_completed_case_at_ordinal(&mut runtime, active_predecessor, 2, 1, 0, 1);
    let unit = runtime.units.get(&system.id).expect("known unit");
    assert!(
        super::super::release::pending_positive_guard_state_is_consistent_for_test(
            unit,
            next_predecessor,
            runtime.cooling_supply_mass_flow_positive_guard_latest_witness(system.id),
        )
    );
    assert!(!completed_direct_cooling_mixed_air_call_is_consistent(
        &runtime,
        unit,
        &system,
        next_predecessor,
        runtime.cooling_mixed_air_call_latest_witness(system.id),
    ));

    let before = runtime.clone();
    assert_eq!(
        advance_direct_no_oa_calc_cooling_supply_mass_flow_positive_guard(
            &mut runtime,
            &system,
            next_predecessor,
        ),
        Err(
            PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardError::
                RuntimeStateInvariantViolation { system: system.id }
        )
    );
    assert_eq!(runtime, before);
}
