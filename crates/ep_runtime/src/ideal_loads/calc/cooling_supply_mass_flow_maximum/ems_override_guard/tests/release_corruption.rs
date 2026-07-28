use super::maximum;
use crate::ideal_loads::{
    advance_direct_no_oa_calc_cooling_supply_mass_flow_ems_override_guard,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_maximum,
    cooling_supply_mass_flow_ems_override_guard_snapshot_is_exact_direct_release,
};

fn release_case(
    cooling_demand_w: f64,
) -> (
    crate::ideal_loads::PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    crate::ideal_loads::PurchasedAirCalcCoolingSupplyMassFlowMaximumSnapshot,
) {
    let (mut runtime, system, reset) = super::super::super::tests::release_case(cooling_demand_w);
    let maximum =
        advance_direct_no_oa_calc_cooling_supply_mass_flow_maximum(&mut runtime, &system, reset)
            .expect("CP322");
    (runtime, system, maximum)
}

#[test]
fn exact_direct_validator_rejects_true_internal_characterization() {
    let predecessor = maximum(-1_000.0);
    let mut state =
        crate::ideal_loads::PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardRuntimeState::new(
            predecessor.system,
        );
    let false_guard = super::super::advance_cooling_supply_mass_flow_ems_override_guard_state(
        &mut state,
        predecessor,
        false,
    );
    assert!(
        cooling_supply_mass_flow_ems_override_guard_snapshot_is_exact_direct_release(false_guard)
    );

    let mut state =
        crate::ideal_loads::PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardRuntimeState::new(
            predecessor.system,
        );
    let true_guard = super::super::advance_cooling_supply_mass_flow_ems_override_guard_state(
        &mut state,
        predecessor,
        true,
    );
    assert!(
        !cooling_supply_mass_flow_ems_override_guard_snapshot_is_exact_direct_release(true_guard)
    );
}

#[test]
fn public_release_uses_false_retained_route_and_replay_is_transactional() {
    let (mut runtime, system, predecessor) = release_case(-1_000.0);
    let snapshot = advance_direct_no_oa_calc_cooling_supply_mass_flow_ems_override_guard(
        &mut runtime,
        &system,
        predecessor,
    )
    .expect("CP323");
    assert!(cooling_supply_mass_flow_ems_override_guard_snapshot_is_exact_direct_release(snapshot));
    assert_eq!(snapshot.ems_supply_mass_flow_override_enabled, Some(false));
    assert!(snapshot.ems_supply_mass_flow_override_guard_false_fallthrough);
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_supply_mass_flow_ems_override_guard(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before);
}

#[test]
fn supplied_cp322_corruption_is_rejected_without_mutation() {
    let (mut runtime, system, mut predecessor) = release_case(-1_000.0);
    predecessor.parent_call_ordinal += 1;
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_supply_mass_flow_ems_override_guard(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before);
}

#[test]
fn retained_cp322_corruption_is_rejected_without_mutation() {
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
        advance_direct_no_oa_calc_cooling_supply_mass_flow_ems_override_guard(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before);
}

#[test]
fn private_cp322_witness_corruption_is_rejected_without_mutation() {
    let (mut runtime, system, predecessor) = release_case(-1_000.0);
    let mut witness = runtime
        .cooling_supply_mass_flow_maximum_latest_witness(system.id)
        .expect("CP322 witness");
    witness.parent_call_ordinal += 1;
    runtime.set_cooling_supply_mass_flow_maximum_latest_witness(system.id, witness);
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_supply_mass_flow_ems_override_guard(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before);
}

#[test]
fn prior_cp321_corruption_is_rejected_without_mutation() {
    let (mut runtime, system, predecessor) = release_case(-1_000.0);
    runtime
        .units
        .get_mut(&system.id)
        .expect("unit")
        .calc_cooling_capacity_zero_flow_reset
        .latest
        .as_mut()
        .expect("CP321")
        .parent_call_ordinal += 1;
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_supply_mass_flow_ems_override_guard(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before);
}

#[test]
fn forged_pending_cp323_counter_is_rejected_without_mutation() {
    let (mut runtime, system, predecessor) = release_case(-1_000.0);
    runtime
        .units
        .get_mut(&system.id)
        .expect("unit")
        .calc_cooling_supply_mass_flow_ems_override_guard
        .ems_supply_mass_flow_override_guard_evaluation_count = 1;
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_supply_mass_flow_ems_override_guard(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before);
}
