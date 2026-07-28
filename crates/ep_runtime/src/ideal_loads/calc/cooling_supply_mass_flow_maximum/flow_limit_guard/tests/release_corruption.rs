use ep_model::{AutosizeOrNumber, IdealLoadsAirSystemId, IdealLoadsLimit};

use crate::ideal_loads::{
    PurchasedAirCalcCoolingSupplyMassFlowLimitGuardError,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_ems_override_body,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_ems_override_guard,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_limit_guard,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_maximum,
    cooling_supply_mass_flow_limit_guard_snapshot_is_exact_direct_release,
};

fn release_case(
    cooling_demand_w: f64,
) -> (
    crate::ideal_loads::PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    crate::ideal_loads::PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot,
) {
    let (mut runtime, system, reset) = super::super::super::tests::release_case(cooling_demand_w);
    let maximum =
        advance_direct_no_oa_calc_cooling_supply_mass_flow_maximum(&mut runtime, &system, reset)
            .expect("CP322");
    let guard = advance_direct_no_oa_calc_cooling_supply_mass_flow_ems_override_guard(
        &mut runtime,
        &system,
        maximum,
    )
    .expect("CP323");
    let body = advance_direct_no_oa_calc_cooling_supply_mass_flow_ems_override_body(
        &mut runtime,
        &system,
        guard,
    )
    .expect("CP324");
    (runtime, system, body)
}

#[test]
fn public_release_rejects_default_no_limit_and_replay_is_transactional() {
    for cooling_demand_w in [-1_000.0, 1.0] {
        let (mut runtime, system, predecessor) = release_case(cooling_demand_w);
        let snapshot = advance_direct_no_oa_calc_cooling_supply_mass_flow_limit_guard(
            &mut runtime,
            &system,
            predecessor,
        )
        .expect("CP325");
        assert!(cooling_supply_mass_flow_limit_guard_snapshot_is_exact_direct_release(snapshot));
        assert!(!snapshot.supply_mass_flow_limit_body_entered);
        let before = runtime.clone();
        assert!(
            advance_direct_no_oa_calc_cooling_supply_mass_flow_limit_guard(
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
fn supplied_cp324_corruption_is_rejected_without_mutation() {
    let (mut runtime, system, mut predecessor) = release_case(-1_000.0);
    predecessor.parent_call_ordinal += 1;
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_supply_mass_flow_limit_guard(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before);
}

#[test]
fn retained_cp324_corruption_is_rejected_without_mutation() {
    let (mut runtime, system, predecessor) = release_case(-1_000.0);
    runtime
        .units
        .get_mut(&system.id)
        .expect("unit")
        .calc_cooling_supply_mass_flow_ems_override_body
        .latest
        .as_mut()
        .expect("CP324")
        .source_order = &[];
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_supply_mass_flow_limit_guard(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before);
}

#[test]
fn private_cp324_witness_corruption_is_rejected_without_mutation() {
    let (mut runtime, system, predecessor) = release_case(-1_000.0);
    let mut witness = runtime
        .cooling_supply_mass_flow_ems_override_body_latest_witness(system.id)
        .expect("CP324 witness");
    witness.parent_call_ordinal += 1;
    runtime.set_cooling_supply_mass_flow_ems_override_body_latest_witness(system.id, witness);
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_supply_mass_flow_limit_guard(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before);
}

#[test]
fn prior_cp323_corruption_is_rejected_without_mutation() {
    let (mut runtime, system, predecessor) = release_case(-1_000.0);
    runtime
        .units
        .get_mut(&system.id)
        .expect("unit")
        .calc_cooling_supply_mass_flow_ems_override_guard
        .latest
        .as_mut()
        .expect("CP323")
        .parent_call_ordinal += 1;
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_supply_mass_flow_limit_guard(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before);
}

#[test]
fn forged_pending_cp325_counter_is_rejected_without_mutation() {
    let (mut runtime, system, predecessor) = release_case(-1_000.0);
    runtime
        .units
        .get_mut(&system.id)
        .expect("unit")
        .calc_cooling_supply_mass_flow_limit_guard
        .first_cooling_limit_read_count = 1;
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_supply_mass_flow_limit_guard(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before);
}

#[test]
fn simultaneous_latest_and_witness_input_corruption_is_rejected_bit_exactly() {
    let (mut runtime, mut system, mut predecessor) = release_case(-1_000.0);
    system.cooling_limit = IdealLoadsLimit::LimitFlowRate;
    let unit = runtime.units.get_mut(&system.id).expect("unit");
    let body_state = &mut unit.calc_cooling_supply_mass_flow_ems_override_body;
    body_state.transition_count += 1;
    body_state.cooling_body_entry_count += 1;
    body_state.body_skip_count += 1;
    body_state.ems_disabled_fallthrough_count += 1;

    let forged_positive = super::super::advance_cooling_supply_mass_flow_limit_guard_state(
        &mut unit.calc_cooling_supply_mass_flow_limit_guard,
        predecessor,
        super::super::PurchasedAirCalcCoolingSupplyMassFlowLimitGuardInput {
            cooling_limit: IdealLoadsLimit::LimitFlowRate,
            maximum_cooling_air_mass_flow_rate_kg_per_s: 0.25,
        },
    );
    predecessor.parent_call_ordinal += 1;
    assert!(super::super::release::pending_guard_state_is_consistent(
        unit,
        &system,
        0.25,
        predecessor,
        Some(forged_positive),
    ));
    assert!(
        !super::super::release::pending_guard_state_is_consistent(
            unit,
            &system,
            0.50,
            predecessor,
            Some(forged_positive),
        ),
        "another positive finite maximum must not pass with jointly forged latest+witness"
    );

    unit.calc_cooling_supply_mass_flow_limit_guard =
        super::super::PurchasedAirCalcCoolingSupplyMassFlowLimitGuardRuntimeState::new(system.id);
    let forged_positive_zero = super::super::advance_cooling_supply_mass_flow_limit_guard_state(
        &mut unit.calc_cooling_supply_mass_flow_limit_guard,
        {
            let mut prior = predecessor;
            prior.parent_call_ordinal -= 1;
            prior
        },
        super::super::PurchasedAirCalcCoolingSupplyMassFlowLimitGuardInput {
            cooling_limit: IdealLoadsLimit::LimitFlowRate,
            maximum_cooling_air_mass_flow_rate_kg_per_s: 0.0,
        },
    );
    assert!(super::super::release::pending_guard_state_is_consistent(
        unit,
        &system,
        0.0,
        predecessor,
        Some(forged_positive_zero),
    ));
    assert!(
        !super::super::release::pending_guard_state_is_consistent(
            unit,
            &system,
            -0.0,
            predecessor,
            Some(forged_positive_zero),
        ),
        "observed positive zero and retained negative zero must remain distinct"
    );
}

#[test]
fn post_init_selector_mutation_is_rejected_without_mutation() {
    let (mut runtime, mut system, predecessor) = release_case(-1_000.0);
    system.cooling_limit = IdealLoadsLimit::LimitFlowRate;
    system.maximum_cooling_air_flow_rate_m3_per_s = Some(AutosizeOrNumber::Value(0.25));
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_supply_mass_flow_limit_guard(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before);
}

#[test]
fn invalid_cached_maximum_is_rejected_without_mutation() {
    let (mut runtime, system, predecessor) = release_case(-1_000.0);
    runtime
        .units
        .get_mut(&system.id)
        .expect("unit")
        .maximum_cooling_air_mass_flow_rate_kg_per_s = -1.0;
    let before = runtime.clone();
    assert_eq!(
        advance_direct_no_oa_calc_cooling_supply_mass_flow_limit_guard(
            &mut runtime,
            &system,
            predecessor,
        ),
        Err(
            PurchasedAirCalcCoolingSupplyMassFlowLimitGuardError::
                InvalidMaximumCoolingMassFlowCache {
                    system: system.id,
                    value_kg_per_s: -1.0,
                }
        )
    );
    assert_eq!(runtime, before);
}

#[test]
fn typed_system_identity_mismatch_is_rejected_without_mutation() {
    let (mut runtime, mut system, predecessor) = release_case(-1_000.0);
    system.id = IdealLoadsAirSystemId(system.id.0 + 1);
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_supply_mass_flow_limit_guard(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before);
}
