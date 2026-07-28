use ep_model::IdealLoadsAirSystemId;

use crate::ideal_loads::{
    advance_direct_no_oa_calc_cooling_supply_mass_flow_very_small_guard,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_very_small_guard_body,
    cooling_supply_mass_flow_very_small_guard_body_snapshot_is_exact_direct_release,
};

fn release_case(
    cooling_demand_w: f64,
) -> (
    crate::ideal_loads::PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    crate::ideal_loads::PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot,
) {
    let (mut runtime, system, limit_body) =
        super::super::super::tests::release_corruption::release_case(cooling_demand_w);
    let guard = advance_direct_no_oa_calc_cooling_supply_mass_flow_very_small_guard(
        &mut runtime,
        &system,
        limit_body,
    )
    .expect("CP327");
    (runtime, system, guard)
}

#[test]
fn public_release_consumes_only_retained_cp327_and_replay_is_transactional() {
    let (mut runtime, system, predecessor) = release_case(-1_000.0);
    let expected_bits = predecessor.supply_mass_flow_rate_kg_per_s.map(f64::to_bits);
    let snapshot = advance_direct_no_oa_calc_cooling_supply_mass_flow_very_small_guard_body(
        &mut runtime,
        &system,
        predecessor,
    )
    .expect("CP328");
    assert!(
        cooling_supply_mass_flow_very_small_guard_body_snapshot_is_exact_direct_release(snapshot)
    );
    assert_eq!(
        snapshot
            .predecessor_supply_mass_flow_rate_kg_per_s
            .map(f64::to_bits),
        expected_bits
    );

    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_supply_mass_flow_very_small_guard_body(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before);
}

#[test]
fn supplied_retained_or_private_cp327_corruption_is_rejected_without_mutation() {
    let (mut runtime, system, predecessor) = release_case(-1_000.0);
    let mut supplied = predecessor;
    supplied.parent_call_ordinal += 1;
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_supply_mass_flow_very_small_guard_body(
            &mut runtime,
            &system,
            supplied,
        )
        .is_err()
    );
    assert_eq!(runtime, before);

    let (mut runtime, system, predecessor) = release_case(-1_000.0);
    runtime
        .units
        .get_mut(&system.id)
        .expect("unit")
        .calc_cooling_supply_mass_flow_very_small_guard
        .latest
        .as_mut()
        .expect("CP327")
        .source_order = &[];
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_supply_mass_flow_very_small_guard_body(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before);

    let (mut runtime, system, predecessor) = release_case(-1_000.0);
    let mut witness = runtime
        .cooling_supply_mass_flow_very_small_guard_latest_witness(system.id)
        .expect("CP327 witness");
    witness.supply_mass_flow_rate_kg_per_s = witness
        .supply_mass_flow_rate_kg_per_s
        .map(|value| f64::from_bits(value.to_bits() ^ (1_u64 << 63)));
    runtime.set_cooling_supply_mass_flow_very_small_guard_latest_witness(system.id, witness);
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_supply_mass_flow_very_small_guard_body(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before);
}

#[test]
fn invalid_cache_and_counter_overflow_fail_closed_without_mutation() {
    let (mut runtime, system, predecessor) = release_case(-1_000.0);
    runtime
        .units
        .get_mut(&system.id)
        .expect("unit")
        .maximum_cooling_air_mass_flow_rate_kg_per_s = -1.0;
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_supply_mass_flow_very_small_guard_body(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before);

    let (mut runtime, system, predecessor) = release_case(-1_000.0);
    runtime
        .units
        .get_mut(&system.id)
        .expect("unit")
        .calc_cooling_supply_mass_flow_very_small_guard_body
        .transition_count = usize::MAX;
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_supply_mass_flow_very_small_guard_body(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before);
}

#[test]
fn private_cumulative_route_witness_rejects_a_public_counter_partition_forgery() {
    let (mut runtime, system, predecessor) = release_case(-1_000.0);
    let snapshot = advance_direct_no_oa_calc_cooling_supply_mass_flow_very_small_guard_body(
        &mut runtime,
        &system,
        predecessor,
    )
    .expect("CP328");
    let state = &mut runtime
        .units
        .get_mut(&system.id)
        .expect("unit")
        .calc_cooling_supply_mass_flow_very_small_guard_body;
    if snapshot.zero_flow_reset_body_entered {
        state.zero_flow_reset_body_entry_count -= 1;
        state.active_guard_false_fallthrough_count += 1;
        state.supply_mass_flow_rate_positive_zero_assignment_count -= 1;
        state.body_skip_count += 1;
    } else {
        state.active_guard_false_fallthrough_count -= 1;
        state.zero_flow_reset_body_entry_count += 1;
        state.supply_mass_flow_rate_positive_zero_assignment_count += 1;
        state.body_skip_count -= 1;
    }

    let unit = runtime.units.get(&system.id).expect("unit");
    assert!(
        !super::super::release::
            completed_direct_cooling_supply_mass_flow_very_small_guard_body_is_consistent(
                &runtime,
                unit,
                &system,
                snapshot,
                runtime
                    .cooling_supply_mass_flow_very_small_guard_body_latest_witness(system.id),
            )
    );
}

#[test]
fn typed_system_identity_mismatch_is_rejected_without_mutation() {
    let (mut runtime, mut system, predecessor) = release_case(-1_000.0);
    system.id = IdealLoadsAirSystemId(system.id.0 + 1);
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_supply_mass_flow_very_small_guard_body(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before);
}
