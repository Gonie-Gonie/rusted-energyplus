use ep_model::IdealLoadsAirSystem;

use crate::ideal_loads::{
    PurchasedAirCalcCoolingEconomizerBodyError, PurchasedAirCalcCoolingEconomizerConditionSnapshot,
    PurchasedAirRuntimeState, advance_direct_no_oa_calc_cooling_economizer_body,
    advance_direct_no_oa_calc_cooling_economizer_condition,
};

use super::cooling_economizer_condition_release_tests::{
    advance_subsequent_fixture_call, release_fixture, release_fixture_with_cooling_demand,
};

mod corruption_tests;
mod provenance_tests;

#[test]
fn public_no_oa_body_never_reads_or_mutates_calculation_sites() {
    for cooling_demand_w in [1.0, -1.0] {
        let (mut runtime, system, predecessor) =
            body_release_fixture_with_cooling_demand(cooling_demand_w);
        let snapshot =
            advance_direct_no_oa_calc_cooling_economizer_body(&mut runtime, &system, predecessor)
                .expect("exact CP317 release transition");

        assert!(super::cooling_economizer_body_snapshot_is_exact_direct_release(snapshot));
        assert!(!snapshot.economizer_calculation_body_executed);
        assert!(!snapshot.zone_humidity_ratio_read);
        assert!(!snapshot.psychrometric_cp_air_evaluated);
        assert!(!snapshot.cp_air_assigned);
        assert!(!snapshot.outdoor_air_temperature_read);
        assert!(!snapshot.zone_temperature_read);
        assert!(!snapshot.delta_temperature_calculated);
        assert!(!snapshot.delta_temperature_assigned);
        assert!(!snapshot.delta_temperature_for_gate_read);
        assert!(!snapshot.delta_temperature_body_entered);
        assert!(!snapshot.zone_cooling_setpoint_load_read);
        assert!(!snapshot.supply_mass_flow_rate_calculated);
        assert!(!snapshot.cp_air_for_first_division_read);
        assert!(!snapshot.delta_temperature_for_second_division_read);
        assert!(!snapshot.initial_supply_mass_flow_rate_assigned);
        assert!(!snapshot.maximum_cooling_air_mass_flow_rate_read);
        assert!(!snapshot.maximum_cooling_air_mass_flow_rate_clamp_upper_bound_read);
        assert!(!snapshot.maximum_flow_clamp_body_entered);
        assert!(!snapshot.supply_mass_flow_rate_for_clamp_read);
        assert!(!snapshot.inner_max_evaluated);
        assert!(!snapshot.outer_min_evaluated);
        assert!(!snapshot.clamped_supply_mass_flow_rate_assigned);
        assert!(!snapshot.resulting_supply_mass_flow_rate_read);
        assert!(!snapshot.outdoor_air_mass_flow_rate_read);
        assert!(!snapshot.economizer_on_assigned);
        assert!(!snapshot.economizer_activation_body_entered);
        assert!(!snapshot.supply_mass_flow_rate_for_outdoor_air_assignment_read);
        assert!(!snapshot.system_time_step_read);
        assert!(!snapshot.economizer_active_time_assigned);

        let state = &runtime
            .units
            .get(&system.id)
            .expect("selected unit")
            .calc_cooling_economizer_body;
        assert_eq!(state.transition_count, 1);
        assert_eq!(state.body_execution_count, 0);
        assert_eq!(state.psychrometric_cp_air_evaluation_count, 0);
        assert_eq!(state.economizer_on_assignment_count, 0);
    }
}

#[test]
fn exact_release_snapshot_rejects_impossible_skip_route_predecessor_flags() {
    let (mut runtime, system, predecessor) = body_release_fixture_with_cooling_demand(1.0);
    let snapshot =
        advance_direct_no_oa_calc_cooling_economizer_body(&mut runtime, &system, predecessor)
            .expect("exact non-cooling CP317 release transition");
    assert!(snapshot.non_cooling_skipped);

    for forged in [
        {
            let mut forged = snapshot;
            forged.predecessor_active_guard_false_economizer_fallthrough = true;
            forged
        },
        {
            let mut forged = snapshot;
            forged.predecessor_economizer_guard_evaluated = true;
            forged
        },
        {
            let mut forged = snapshot;
            forged.predecessor_no_economizer_fallthrough = true;
            forged
        },
    ] {
        assert!(!super::cooling_economizer_body_snapshot_is_exact_direct_release(forged));
    }
}

#[test]
fn public_body_rejects_forgery_replay_overflow_and_prefix_corruption_transactionally() {
    let (runtime, system, predecessor) = body_release_fixture();

    let mut forged = predecessor;
    forged.parent_call_ordinal += 1;
    assert_rejected_without_mutation(runtime.clone(), &system, forged);

    let mut prefix_corruption = runtime.clone();
    prefix_corruption
        .units
        .get_mut(&system.id)
        .expect("selected unit")
        .calc_cooling_oa_max_flow_gate
        .strict_mass_flow_comparison_count = usize::MAX;
    assert_rejected_without_mutation(prefix_corruption, &system, predecessor);

    let mut overflow = runtime.clone();
    overflow
        .units
        .get_mut(&system.id)
        .expect("selected unit")
        .calc_cooling_economizer_body
        .transition_count = usize::MAX;
    assert_rejected_without_mutation(overflow, &system, predecessor);

    let mut replay = runtime;
    advance_direct_no_oa_calc_cooling_economizer_body(&mut replay, &system, predecessor)
        .expect("first CP317 call");
    let before_replay = replay.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_economizer_body(&mut replay, &system, predecessor,)
            .is_err()
    );
    assert_eq!(replay, before_replay);
}

pub(super) fn body_release_fixture() -> (
    PurchasedAirRuntimeState,
    IdealLoadsAirSystem,
    PurchasedAirCalcCoolingEconomizerConditionSnapshot,
) {
    let (mut runtime, system, guard) = release_fixture();
    let condition =
        advance_direct_no_oa_calc_cooling_economizer_condition(&mut runtime, &system, guard)
            .expect("exact CP316 predecessor");
    (runtime, system, condition)
}

pub(super) fn body_release_fixture_with_cooling_demand(
    cooling_demand_w: f64,
) -> (
    PurchasedAirRuntimeState,
    IdealLoadsAirSystem,
    PurchasedAirCalcCoolingEconomizerConditionSnapshot,
) {
    let (mut runtime, system, guard) = release_fixture_with_cooling_demand(cooling_demand_w);
    let condition =
        advance_direct_no_oa_calc_cooling_economizer_condition(&mut runtime, &system, guard)
            .expect("exact CP316 predecessor");
    (runtime, system, condition)
}

pub(super) fn advance_subsequent_body_predecessor(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    cooling_demand_w: f64,
) -> PurchasedAirCalcCoolingEconomizerConditionSnapshot {
    let guard = advance_subsequent_fixture_call(runtime, system, cooling_demand_w);
    advance_direct_no_oa_calc_cooling_economizer_condition(runtime, system, guard)
        .expect("subsequent exact CP316 predecessor")
}

pub(super) fn assert_rejected_without_mutation(
    mut runtime: PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: PurchasedAirCalcCoolingEconomizerConditionSnapshot,
) {
    let before = runtime.clone();
    let result =
        advance_direct_no_oa_calc_cooling_economizer_body(&mut runtime, system, predecessor);
    assert!(
        matches!(
            result,
            Err(
                PurchasedAirCalcCoolingEconomizerBodyError::
                    CoolingEconomizerConditionSnapshotMismatch { .. }
            )
                | Err(
                    PurchasedAirCalcCoolingEconomizerBodyError::PredecessorCallOrder { .. }
                )
                | Err(
                    PurchasedAirCalcCoolingEconomizerBodyError::
                        PredecessorOutsideDirectSubset { .. }
                )
                | Err(
                    PurchasedAirCalcCoolingEconomizerBodyError::
                        RuntimeStateInvariantViolation { .. }
                )
                | Err(
                    PurchasedAirCalcCoolingEconomizerBodyError::InitializationNotReady { .. }
                )
        ),
        "{result:?}"
    );
    assert_eq!(runtime, before);
}
