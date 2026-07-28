use ep_model::IdealLoadsAirSystemId;

use crate::ideal_loads::{
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardError,
    advance_direct_no_oa_calc_cooling_capacity_zero_flow_reset,
    advance_direct_no_oa_calc_cooling_dehumidification_flow,
    advance_direct_no_oa_calc_cooling_humidification_flow,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_ems_override_body,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_ems_override_guard,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_limit_body,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_limit_guard,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_maximum,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_very_small_guard,
    cooling_supply_mass_flow_very_small_guard_snapshot_is_exact_direct_release,
};

fn release_case(
    cooling_demand_w: f64,
) -> (
    crate::ideal_loads::PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    crate::ideal_loads::PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot,
) {
    let (mut runtime, system, sensible) =
        crate::ideal_loads::calc::cooling_dehumidification_flow_release_tests::release_case(
            cooling_demand_w,
        );
    let dehumidification =
        advance_direct_no_oa_calc_cooling_dehumidification_flow(&mut runtime, &system, sensible)
            .expect("CP319");
    let humidification = advance_direct_no_oa_calc_cooling_humidification_flow(
        &mut runtime,
        &system,
        dehumidification,
    )
    .expect("CP320");
    let reset = advance_direct_no_oa_calc_cooling_capacity_zero_flow_reset(
        &mut runtime,
        &system,
        humidification,
    )
    .expect("CP321");
    let maximum =
        advance_direct_no_oa_calc_cooling_supply_mass_flow_maximum(&mut runtime, &system, reset)
            .expect("CP322");
    let ems_guard = advance_direct_no_oa_calc_cooling_supply_mass_flow_ems_override_guard(
        &mut runtime,
        &system,
        maximum,
    )
    .expect("CP323");
    let ems_body = advance_direct_no_oa_calc_cooling_supply_mass_flow_ems_override_body(
        &mut runtime,
        &system,
        ems_guard,
    )
    .expect("CP324");
    let limit_guard = advance_direct_no_oa_calc_cooling_supply_mass_flow_limit_guard(
        &mut runtime,
        &system,
        ems_body,
    )
    .expect("CP325");
    let limit_body = advance_direct_no_oa_calc_cooling_supply_mass_flow_limit_body(
        &mut runtime,
        &system,
        limit_guard,
    )
    .expect("CP326");
    (runtime, system, limit_body)
}

#[test]
fn public_release_reads_only_retained_cp326_supply_and_replay_is_transactional() {
    let (mut runtime, system, predecessor) = release_case(-1_000.0);
    let expected_bits = predecessor
        .resulting_supply_mass_flow_rate_kg_per_s
        .map(f64::to_bits);
    let snapshot = advance_direct_no_oa_calc_cooling_supply_mass_flow_very_small_guard(
        &mut runtime,
        &system,
        predecessor,
    )
    .expect("CP327");
    assert!(cooling_supply_mass_flow_very_small_guard_snapshot_is_exact_direct_release(snapshot));
    assert_eq!(
        snapshot.supply_mass_flow_rate_kg_per_s.map(f64::to_bits),
        expected_bits
    );

    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_supply_mass_flow_very_small_guard(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before);
}

#[test]
fn inactive_release_skips_all_line_2166_sites() {
    let (mut runtime, system, predecessor) = release_case(1.0);
    let snapshot = advance_direct_no_oa_calc_cooling_supply_mass_flow_very_small_guard(
        &mut runtime,
        &system,
        predecessor,
    )
    .expect("CP327 UnitOff");
    assert!(snapshot.unit_off_skipped || snapshot.non_cooling_skipped);
    assert!(!snapshot.supply_mass_flow_rate_read);
    assert!(!snapshot.hvac_very_small_mass_flow_read);
    assert!(!snapshot.supply_mass_flow_rate_at_or_below_very_small_mass_flow_comparison_evaluated);
    assert!(!snapshot.zero_flow_reset_body_entered);
}

#[test]
fn supplied_retained_or_private_cp326_corruption_is_rejected_without_mutation() {
    let (mut runtime, system, predecessor) = release_case(-1_000.0);
    let mut supplied = predecessor;
    supplied.parent_call_ordinal += 1;
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_supply_mass_flow_very_small_guard(
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
        .calc_cooling_supply_mass_flow_limit_body
        .latest
        .as_mut()
        .expect("CP326")
        .source_order = &[];
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_supply_mass_flow_very_small_guard(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before);

    let (mut runtime, system, predecessor) = release_case(-1_000.0);
    let mut witness = runtime
        .cooling_supply_mass_flow_limit_body_latest_witness(system.id)
        .expect("CP326 witness");
    witness.resulting_supply_mass_flow_rate_kg_per_s = witness
        .resulting_supply_mass_flow_rate_kg_per_s
        .map(|value| f64::from_bits(value.to_bits() ^ (1_u64 << 63)));
    runtime.set_cooling_supply_mass_flow_limit_body_latest_witness(system.id, witness);
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_supply_mass_flow_very_small_guard(
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
    assert_eq!(
        advance_direct_no_oa_calc_cooling_supply_mass_flow_very_small_guard(
            &mut runtime,
            &system,
            predecessor,
        ),
        Err(
            PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardError::
                InvalidMaximumCoolingMassFlowCache {
                    system: system.id,
                    value_kg_per_s: -1.0,
                }
        )
    );
    assert_eq!(runtime, before);

    let (mut runtime, system, predecessor) = release_case(-1_000.0);
    runtime
        .units
        .get_mut(&system.id)
        .expect("unit")
        .calc_cooling_supply_mass_flow_very_small_guard
        .transition_count = usize::MAX;
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_supply_mass_flow_very_small_guard(
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
    let snapshot = advance_direct_no_oa_calc_cooling_supply_mass_flow_very_small_guard(
        &mut runtime,
        &system,
        predecessor,
    )
    .expect("CP327");
    let state = &mut runtime
        .units
        .get_mut(&system.id)
        .expect("unit")
        .calc_cooling_supply_mass_flow_very_small_guard;
    if snapshot.zero_flow_reset_body_entered {
        state.zero_flow_reset_body_entry_count -= 1;
        state.active_guard_false_fallthrough_count += 1;
    } else {
        state.active_guard_false_fallthrough_count -= 1;
        state.zero_flow_reset_body_entry_count += 1;
    }

    let unit = runtime.units.get(&system.id).expect("unit");
    assert!(
        !super::super::release::
            completed_direct_cooling_supply_mass_flow_very_small_guard_is_consistent(
                &runtime,
                unit,
                &system,
                snapshot,
                runtime.cooling_supply_mass_flow_very_small_guard_latest_witness(system.id),
            )
    );
}

#[test]
fn typed_system_identity_mismatch_is_rejected_without_mutation() {
    let (mut runtime, mut system, predecessor) = release_case(-1_000.0);
    system.id = IdealLoadsAirSystemId(system.id.0 + 1);
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_supply_mass_flow_very_small_guard(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before);
}
