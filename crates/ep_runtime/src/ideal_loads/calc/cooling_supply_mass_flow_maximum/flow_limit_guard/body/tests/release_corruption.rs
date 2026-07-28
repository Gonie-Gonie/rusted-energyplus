use ep_model::IdealLoadsAirSystemId;

use crate::ideal_loads::{
    PurchasedAirCalcCoolingSupplyMassFlowLimitBodyError,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_ems_override_body,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_ems_override_guard,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_limit_body,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_limit_guard,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_maximum,
    cooling_supply_mass_flow_limit_body_snapshot_is_exact_direct_release,
};

fn release_case(
    cooling_demand_w: f64,
) -> (
    crate::ideal_loads::PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    crate::ideal_loads::PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot,
) {
    let (mut runtime, system, reset) =
        super::super::super::super::tests::release_case(cooling_demand_w);
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
    (runtime, system, limit_guard)
}

#[test]
fn public_release_carries_active_guard_false_supply_and_replay_is_transactional() {
    let (mut runtime, system, predecessor) = release_case(-1_000.0);
    let expected_bits = runtime
        .units
        .get(&system.id)
        .expect("unit")
        .calc_cooling_supply_mass_flow_maximum
        .latest
        .expect("CP322")
        .resulting_supply_mass_flow_rate_kg_per_s
        .map(f64::to_bits);
    let snapshot = advance_direct_no_oa_calc_cooling_supply_mass_flow_limit_body(
        &mut runtime,
        &system,
        predecessor,
    )
    .expect("CP326");
    assert!(cooling_supply_mass_flow_limit_body_snapshot_is_exact_direct_release(snapshot));
    assert!(snapshot.body_skipped);
    assert!(snapshot.active_guard_false_fallthrough);
    assert_eq!(
        snapshot
            .resulting_supply_mass_flow_rate_kg_per_s
            .map(f64::to_bits),
        expected_bits
    );

    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_supply_mass_flow_limit_body(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before);
}

#[test]
fn supplied_or_retained_cp325_corruption_is_rejected_without_mutation() {
    let (mut runtime, system, predecessor) = release_case(-1_000.0);
    let mut supplied = predecessor;
    supplied.parent_call_ordinal += 1;
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_supply_mass_flow_limit_body(
            &mut runtime,
            &system,
            supplied,
        )
        .is_err()
    );
    assert_eq!(runtime, before);

    runtime
        .units
        .get_mut(&system.id)
        .expect("unit")
        .calc_cooling_supply_mass_flow_limit_guard
        .latest
        .as_mut()
        .expect("CP325")
        .source_order = &[];
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_supply_mass_flow_limit_body(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before);
}

#[test]
fn private_cp325_witness_or_cp322_supply_corruption_is_rejected_without_mutation() {
    let (mut runtime, system, predecessor) = release_case(-1_000.0);
    let mut witness = runtime
        .cooling_supply_mass_flow_limit_guard_latest_witness(system.id)
        .expect("CP325 witness");
    witness.parent_call_ordinal += 1;
    runtime.set_cooling_supply_mass_flow_limit_guard_latest_witness(system.id, witness);
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_supply_mass_flow_limit_body(
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
        .calc_cooling_supply_mass_flow_maximum
        .latest
        .as_mut()
        .expect("CP322")
        .resulting_supply_mass_flow_rate_kg_per_s = Some(-0.0);
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_supply_mass_flow_limit_body(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before);
}

#[test]
fn invalid_cached_maximum_and_counter_underflow_fail_closed_without_mutation() {
    let (mut runtime, system, predecessor) = release_case(-1_000.0);
    runtime
        .units
        .get_mut(&system.id)
        .expect("unit")
        .maximum_cooling_air_mass_flow_rate_kg_per_s = -1.0;
    let before = runtime.clone();
    assert_eq!(
        advance_direct_no_oa_calc_cooling_supply_mass_flow_limit_body(
            &mut runtime,
            &system,
            predecessor,
        ),
        Err(
            PurchasedAirCalcCoolingSupplyMassFlowLimitBodyError::
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
        .calc_cooling_supply_mass_flow_limit_guard
        .supply_mass_flow_limit_body_entry_count = usize::MAX;
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_supply_mass_flow_limit_body(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before);
}

#[test]
fn typed_system_identity_mismatch_is_rejected_without_mutation() {
    let (mut runtime, mut system, predecessor) = release_case(-1_000.0);
    system.id = IdealLoadsAirSystemId(system.id.0 + 1);
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_supply_mass_flow_limit_body(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before);
}
