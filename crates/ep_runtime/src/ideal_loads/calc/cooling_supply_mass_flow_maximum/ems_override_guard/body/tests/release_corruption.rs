use crate::ideal_loads::{
    PurchasedAirCalcCoolingEconomizerGuardSnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot, PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_cooling_capacity_zero_flow_reset,
    advance_direct_no_oa_calc_cooling_dehumidification_flow,
    advance_direct_no_oa_calc_cooling_economizer_body,
    advance_direct_no_oa_calc_cooling_economizer_condition,
    advance_direct_no_oa_calc_cooling_humidification_flow,
    advance_direct_no_oa_calc_cooling_sensible_flow,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_ems_override_body,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_ems_override_guard,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_limit_guard,
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

fn advance_from_economizer_guard_through_cp324(
    runtime: &mut PurchasedAirRuntimeState,
    system: &ep_model::IdealLoadsAirSystem,
    guard: PurchasedAirCalcCoolingEconomizerGuardSnapshot,
) -> PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot {
    let condition = advance_direct_no_oa_calc_cooling_economizer_condition(runtime, system, guard)
        .expect("CP316");
    let economizer_body =
        advance_direct_no_oa_calc_cooling_economizer_body(runtime, system, condition)
            .expect("CP317");
    let zone_state = crate::ideal_loads::calc::cooling_sensible_flow_release_tests::zone_state(
        economizer_body.controlled_zone,
    );
    let sensible = advance_direct_no_oa_calc_cooling_sensible_flow(
        runtime,
        system,
        economizer_body,
        &zone_state,
    )
    .expect("CP318");
    let dehumidification =
        advance_direct_no_oa_calc_cooling_dehumidification_flow(runtime, system, sensible)
            .expect("CP319");
    let humidification =
        advance_direct_no_oa_calc_cooling_humidification_flow(runtime, system, dehumidification)
            .expect("CP320");
    let reset =
        advance_direct_no_oa_calc_cooling_capacity_zero_flow_reset(runtime, system, humidification)
            .expect("CP321");
    let maximum =
        advance_direct_no_oa_calc_cooling_supply_mass_flow_maximum(runtime, system, reset)
            .expect("CP322");
    let ems_guard = advance_direct_no_oa_calc_cooling_supply_mass_flow_ems_override_guard(
        runtime, system, maximum,
    )
    .expect("CP323");
    advance_direct_no_oa_calc_cooling_supply_mass_flow_ems_override_body(runtime, system, ems_guard)
        .expect("CP324")
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
fn coordinated_cp324_and_cp325_historical_counter_corruption_is_rejected() {
    let (mut runtime, system, first_guard) =
        crate::ideal_loads::calc::cooling_economizer_condition_release_tests::
            release_fixture_with_cooling_demand(-1_000.0);
    let first_body =
        advance_from_economizer_guard_through_cp324(&mut runtime, &system, first_guard);
    advance_direct_no_oa_calc_cooling_supply_mass_flow_limit_guard(
        &mut runtime,
        &system,
        first_body,
    )
    .expect("first CP325");

    let second_guard =
        crate::ideal_loads::calc::cooling_economizer_condition_release_tests::
            advance_subsequent_fixture_call(&mut runtime, &system, -1_000.0);
    let second_body =
        advance_from_economizer_guard_through_cp324(&mut runtime, &system, second_guard);

    let unit = runtime.units.get_mut(&system.id).expect("unit");
    let body = &mut unit.calc_cooling_supply_mass_flow_ems_override_body;
    body.cooling_body_entry_count -= 1;
    body.non_cooling_skip_count += 1;
    body.ems_disabled_fallthrough_count -= 1;

    let limit_guard = &mut unit.calc_cooling_supply_mass_flow_limit_guard;
    limit_guard.cooling_body_entry_count -= 1;
    limit_guard.non_cooling_skip_count += 1;
    limit_guard.first_cooling_limit_read_count = 0;
    limit_guard.cooling_limit_flow_rate_comparison_count = 0;
    limit_guard.second_cooling_limit_read_count = 0;
    limit_guard.cooling_limit_flow_rate_and_capacity_comparison_count = 0;
    limit_guard.cooling_limit_rejected_count = 0;

    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_supply_mass_flow_limit_guard(
            &mut runtime,
            &system,
            second_body,
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
