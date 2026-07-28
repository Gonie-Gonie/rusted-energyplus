use crate::ideal_loads::{
    advance_direct_no_oa_calc_cooling_supply_mass_flow_ems_override_body,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_ems_override_guard,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_maximum,
    cooling_supply_mass_flow_ems_override_body_snapshot_is_exact_direct_release,
};

fn release_case(
    cooling_demand_w: f64,
) -> (
    crate::ideal_loads::PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    crate::ideal_loads::PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardSnapshot,
) {
    let (mut runtime, system, reset) =
        super::super::super::super::tests::release_case(cooling_demand_w);
    let maximum =
        advance_direct_no_oa_calc_cooling_supply_mass_flow_maximum(&mut runtime, &system, reset)
            .expect("CP322");
    let guard = advance_direct_no_oa_calc_cooling_supply_mass_flow_ems_override_guard(
        &mut runtime,
        &system,
        maximum,
    )
    .expect("CP323");
    (runtime, system, guard)
}

#[test]
fn public_release_completely_skips_body_and_replay_is_transactional() {
    for cooling_demand_w in [-1_000.0, 1.0] {
        let (mut runtime, system, predecessor) = release_case(cooling_demand_w);
        let snapshot = advance_direct_no_oa_calc_cooling_supply_mass_flow_ems_override_body(
            &mut runtime,
            &system,
            predecessor,
        )
        .expect("CP324");
        assert!(
            cooling_supply_mass_flow_ems_override_body_snapshot_is_exact_direct_release(snapshot)
        );
        assert!(snapshot.body_skipped);
        assert!(!snapshot.ems_supply_mass_flow_override_value_read);
        let before = runtime.clone();
        assert!(
            advance_direct_no_oa_calc_cooling_supply_mass_flow_ems_override_body(
                &mut runtime,
                &system,
                predecessor,
            )
            .is_err()
        );
        assert_eq!(runtime, before);
    }
}

#[test]
fn supplied_cp323_corruption_is_rejected_without_mutation() {
    let (mut runtime, system, mut predecessor) = release_case(-1_000.0);
    predecessor.parent_call_ordinal += 1;
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_supply_mass_flow_ems_override_body(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before);
}

#[test]
fn retained_cp323_corruption_is_rejected_without_mutation() {
    let (mut runtime, system, predecessor) = release_case(-1_000.0);
    runtime
        .units
        .get_mut(&system.id)
        .expect("unit")
        .calc_cooling_supply_mass_flow_ems_override_guard
        .latest
        .as_mut()
        .expect("CP323")
        .source_order = &[];
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_supply_mass_flow_ems_override_body(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before);
}

#[test]
fn private_cp323_witness_corruption_is_rejected_without_mutation() {
    let (mut runtime, system, predecessor) = release_case(-1_000.0);
    let mut witness = runtime
        .cooling_supply_mass_flow_ems_override_guard_latest_witness(system.id)
        .expect("CP323 witness");
    witness.parent_call_ordinal += 1;
    runtime.set_cooling_supply_mass_flow_ems_override_guard_latest_witness(system.id, witness);
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_supply_mass_flow_ems_override_body(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before);
}

#[test]
fn prior_cp322_corruption_is_rejected_without_mutation() {
    let (mut runtime, system, predecessor) = release_case(-1_000.0);
    runtime
        .units
        .get_mut(&system.id)
        .expect("unit")
        .calc_cooling_supply_mass_flow_maximum
        .latest
        .as_mut()
        .expect("CP322")
        .parent_call_ordinal += 1;
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_supply_mass_flow_ems_override_body(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before);
}

#[test]
fn forged_pending_cp324_counter_is_rejected_without_mutation() {
    let (mut runtime, system, predecessor) = release_case(-1_000.0);
    runtime
        .units
        .get_mut(&system.id)
        .expect("unit")
        .calc_cooling_supply_mass_flow_ems_override_body
        .body_skip_count = 1;
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_supply_mass_flow_ems_override_body(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before);
}

#[test]
fn inactive_route_false_fallthrough_forgery_is_rejected() {
    let (_, _, predecessor) = release_case(1.0);
    let mut state =
        crate::ideal_loads::PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyRuntimeState::new(
            predecessor.system,
        );
    let mut snapshot = super::super::advance_cooling_supply_mass_flow_ems_override_body_state(
        &mut state,
        predecessor,
        None,
    );
    snapshot.predecessor_ems_supply_mass_flow_override_guard_false_fallthrough = true;
    snapshot.ems_disabled_fallthrough = true;
    assert!(!cooling_supply_mass_flow_ems_override_body_snapshot_is_exact_direct_release(snapshot));
}
