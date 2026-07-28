use super::super::*;
use crate::ideal_loads::calc::cooling_economizer_condition_release_tests::cp338_fixture::
    release_fixture_with_cooling_demand_availability_and_capacity_limit;
use crate::ideal_loads::calc::cooling_economizer_condition_release_tests::
    release_fixture_with_cooling_demand_and_availability;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot, PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_cooling_capacity_zero_flow_reset,
    advance_direct_no_oa_calc_cooling_dehumidification_flow,
    advance_direct_no_oa_calc_cooling_economizer_body,
    advance_direct_no_oa_calc_cooling_economizer_condition,
    advance_direct_no_oa_calc_cooling_humidification_flow,
    advance_direct_no_oa_calc_cooling_mixed_air_call,
    advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_cp_air_assignment,
    advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_guard,
    advance_direct_no_oa_calc_cooling_positive_supply_cp_air_assignment,
    advance_direct_no_oa_calc_cooling_positive_supply_enthalpy_assignment,
    advance_direct_no_oa_calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment,
    advance_direct_no_oa_calc_cooling_positive_supply_temperature_assignment,
    advance_direct_no_oa_calc_cooling_positive_supply_temperature_minimum_limit,
    advance_direct_no_oa_calc_cooling_positive_supply_temperature_mixed_air_limit,
    advance_direct_no_oa_calc_cooling_sensible_flow,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_ems_override_body,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_ems_override_guard,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_limit_body,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_limit_guard,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_maximum,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_positive_guard,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_very_small_guard,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_very_small_guard_body,
};
use crate::psychrometrics::energyplus_psy_cp_air_fn_w;

fn completed_cp337_case(
    cooling_demand_w: f64,
    overall_availability: f64,
    capacity_limit: bool,
    humidity_ratio: f64,
) -> (
    PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot,
) {
    let (mut runtime, system, guard) = if capacity_limit {
        release_fixture_with_cooling_demand_availability_and_capacity_limit(
            cooling_demand_w,
            overall_availability,
        )
    } else {
        release_fixture_with_cooling_demand_and_availability(
            cooling_demand_w,
            overall_availability,
        )
    };
    let condition =
        advance_direct_no_oa_calc_cooling_economizer_condition(&mut runtime, &system, guard)
            .expect("CP316");
    let body =
        advance_direct_no_oa_calc_cooling_economizer_body(&mut runtime, &system, condition)
            .expect("CP317");
    let mut zone_state =
        crate::ideal_loads::calc::cooling_sensible_flow_release_tests::zone_state(
            body.controlled_zone,
        );
    zone_state.air_humidity_ratio = humidity_ratio;
    let sensible = advance_direct_no_oa_calc_cooling_sensible_flow(
        &mut runtime,
        &system,
        body,
        &zone_state,
    )
    .expect("CP318");
    let dehumidification = advance_direct_no_oa_calc_cooling_dehumidification_flow(
        &mut runtime,
        &system,
        sensible,
    )
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
    let very_small_guard =
        advance_direct_no_oa_calc_cooling_supply_mass_flow_very_small_guard(
            &mut runtime,
            &system,
            limit_body,
        )
        .expect("CP327");
    let very_small_body =
        advance_direct_no_oa_calc_cooling_supply_mass_flow_very_small_guard_body(
            &mut runtime,
            &system,
            very_small_guard,
        )
        .expect("CP328");
    let mixed_air = advance_direct_no_oa_calc_cooling_mixed_air_call(
        &mut runtime,
        &system,
        very_small_body,
        &zone_state,
    )
    .expect("CP329");
    let positive_guard = advance_direct_no_oa_calc_cooling_supply_mass_flow_positive_guard(
        &mut runtime,
        &system,
        mixed_air,
    )
    .expect("CP330");
    let cp_air_assignment = advance_direct_no_oa_calc_cooling_positive_supply_cp_air_assignment(
        &mut runtime,
        &system,
        positive_guard,
        &zone_state,
    )
    .expect("CP331");
    let temperature_assignment =
        advance_direct_no_oa_calc_cooling_positive_supply_temperature_assignment(
            &mut runtime,
            &system,
            cp_air_assignment,
            &zone_state,
        )
        .expect("CP332");
    let minimum_limit =
        advance_direct_no_oa_calc_cooling_positive_supply_temperature_minimum_limit(
            &mut runtime,
            &system,
            temperature_assignment,
        )
        .expect("CP333");
    let mixed_air_limit =
        advance_direct_no_oa_calc_cooling_positive_supply_temperature_mixed_air_limit(
            &mut runtime,
            &system,
            minimum_limit,
        )
        .expect("CP334");
    let humidity_assignment =
        advance_direct_no_oa_calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment(
            &mut runtime,
            &system,
            mixed_air_limit,
        )
        .expect("CP335");
    let enthalpy_assignment =
        advance_direct_no_oa_calc_cooling_positive_supply_enthalpy_assignment(
            &mut runtime,
            &system,
            humidity_assignment,
        )
        .expect("CP336");
    let capacity_guard =
        advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_guard(
            &mut runtime,
            &system,
            enthalpy_assignment,
        )
        .expect("CP337");
    (runtime, system, capacity_guard)
}

fn active_case() -> (
    PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot,
) {
    completed_cp337_case(-1_000.0, 1.0, true, 0.008)
}

fn assert_rejected_transactionally(
    runtime: &mut PurchasedAirRuntimeState,
    system: &ep_model::IdealLoadsAirSystem,
    predecessor: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot,
) {
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_cp_air_assignment(
            runtime,
            system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(*runtime, before);
}

#[test]
fn public_active_release_uses_retained_cp329_operand_and_rejects_replay() {
    for humidity_ratio in [0.008, 0.0, -0.0] {
        let (mut runtime, system, predecessor) =
            completed_cp337_case(-1_000.0, 1.0, true, humidity_ratio);
        assert!(predecessor.capacity_limit_body_entered);
        let snapshot =
            advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_cp_air_assignment(
                &mut runtime,
                &system,
                predecessor,
            )
            .expect("CP338");
        let expected = energyplus_psy_cp_air_fn_w(humidity_ratio);

        assert!(
            cooling_positive_supply_capacity_limit_cp_air_assignment_snapshot_is_exact_direct_release(
                snapshot,
            )
        );
        assert_eq!(
            snapshot.mixed_air_humidity_ratio.map(f64::to_bits),
            Some(humidity_ratio.to_bits())
        );
        assert_eq!(
            snapshot.cp_air_j_per_kg_k.map(f64::to_bits),
            Some(expected.to_bits())
        );
        let state = &runtime
            .units
            .get(&system.id)
            .expect("known unit")
            .calc_cooling_positive_supply_capacity_limit_cp_air_assignment;
        assert_eq!(state.transition_count, 1);
        assert_eq!(state.capacity_limit_cp_air_assignment_count, 1);
        assert_eq!(state.source_site_execution_count, 3);

        let before = runtime.clone();
        assert!(
            advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_cp_air_assignment(
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
fn public_release_preserves_all_four_zero_site_skip_routes() {
    for (demand, availability, capacity, unit_off, non_cooling, positive_false, capacity_false) in [
        (-1_000.0, 0.0, true, true, false, false, false),
        (1.0, 1.0, true, false, true, false, false),
        (-1.0e-40, 1.0, true, false, false, true, false),
        (-1_000.0, 1.0, false, false, false, false, true),
    ] {
        let (mut runtime, system, predecessor) =
            completed_cp337_case(demand, availability, capacity, 0.008);
        let snapshot =
            advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_cp_air_assignment(
                &mut runtime,
                &system,
                predecessor,
            )
            .expect("skipped CP338");

        assert_eq!(snapshot.unit_off_skipped, unit_off);
        assert_eq!(snapshot.non_cooling_skipped, non_cooling);
        assert_eq!(
            snapshot.positive_guard_false_fallthrough_skipped,
            positive_false
        );
        assert_eq!(
            snapshot.capacity_limit_guard_false_fallthrough_skipped,
            capacity_false
        );
        assert!(!snapshot.mixed_air_humidity_ratio_read);
        assert!(snapshot.mixed_air_humidity_ratio.is_none());
        assert!(!snapshot.psychrometric_cp_air_evaluated);
        assert!(!snapshot.cp_air_assigned);
        assert_eq!(
            runtime
                .units
                .get(&system.id)
                .expect("known unit")
                .calc_cooling_positive_supply_capacity_limit_cp_air_assignment
                .source_site_execution_count,
            0
        );
    }
}

#[test]
fn supplied_public_or_private_cp337_drift_is_transactional() {
    let (runtime, system, predecessor) = active_case();

    let mut forged = predecessor;
    forged.source = "forged";
    let mut supplied = runtime.clone();
    assert_rejected_transactionally(&mut supplied, &system, forged);

    let mut public = runtime.clone();
    public
        .units
        .get_mut(&system.id)
        .expect("known unit")
        .calc_cooling_positive_supply_capacity_limit_guard
        .latest
        .as_mut()
        .expect("CP337 latest")
        .capacity_limit_body_entered = false;
    assert_rejected_transactionally(&mut public, &system, predecessor);

    let mut private = runtime;
    let mut witness = private
        .cooling_positive_supply_capacity_limit_guard_latest_witness(system.id)
        .expect("CP337 witness");
    witness.source = "forged-private";
    private.set_cooling_positive_supply_capacity_limit_guard_latest_witness(
        system.id,
        witness,
    );
    assert_rejected_transactionally(&mut private, &system, predecessor);
}

#[test]
fn active_cp329_public_and_private_operand_lineage_drift_is_transactional() {
    let (runtime, system, predecessor) = active_case();

    let mut public = runtime.clone();
    public
        .units
        .get_mut(&system.id)
        .expect("known unit")
        .calc_cooling_mixed_air_call
        .latest
        .as_mut()
        .expect("CP329 latest")
        .mixed_air_humidity_ratio = Some(0.009);
    assert_rejected_transactionally(&mut public, &system, predecessor);

    let mut private = runtime;
    let mut witness = private
        .cooling_mixed_air_call_latest_witness(system.id)
        .expect("CP329 witness");
    witness.mixed_air_humidity_ratio = Some(0.009);
    private.set_cooling_mixed_air_call_latest_witness(system.id, witness);
    assert_rejected_transactionally(&mut private, &system, predecessor);
}

#[test]
fn orphan_cp338_public_private_and_retained_metadata_are_fail_closed() {
    let (runtime, system, predecessor) = active_case();
    let humidity_ratio = runtime
        .units
        .get(&system.id)
        .expect("known unit")
        .calc_cooling_mixed_air_call
        .latest
        .and_then(|snapshot| snapshot.mixed_air_humidity_ratio)
        .expect("CP329 humidity");
    let mut seed =
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentRuntimeState::new(
            system.id,
        );
    let orphan = advance_cooling_positive_supply_capacity_limit_cp_air_assignment_state(
        &mut seed,
        predecessor,
        Some(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentActiveInput {
                mixed_air_humidity_ratio: humidity_ratio,
            },
        ),
    );

    let mut public = runtime.clone();
    public
        .units
        .get_mut(&system.id)
        .expect("known unit")
        .calc_cooling_positive_supply_capacity_limit_cp_air_assignment
        .latest = Some(orphan);
    assert_rejected_transactionally(&mut public, &system, predecessor);

    let mut private = runtime.clone();
    private
        .set_cooling_positive_supply_capacity_limit_cp_air_assignment_latest_witness(
            system.id,
            orphan,
        );
    assert_rejected_transactionally(&mut private, &system, predecessor);

    let mut retained = runtime;
    retained
        .units
        .get_mut(&system.id)
        .expect("known unit")
        .calc_cooling_positive_supply_capacity_limit_cp_air_assignment
        .latest_transition_ordinal = Some(1);
    assert_rejected_transactionally(&mut retained, &system, predecessor);
}

#[test]
fn active_assignment_counter_overflows_are_preflighted_transactionally() {
    for counter in 0..7 {
        let (mut runtime, system, predecessor) = active_case();
        let state = &mut runtime
            .units
            .get_mut(&system.id)
            .expect("known unit")
            .calc_cooling_positive_supply_capacity_limit_cp_air_assignment;
        match counter {
            0 => state.transition_count = usize::MAX,
            1 => state.capacity_limit_cp_air_assignment_count = usize::MAX,
            2 => state.source_site_execution_count = usize::MAX - 2,
            3 => state.mixed_air_humidity_ratio_read_count = usize::MAX,
            4 => state.psychrometric_cp_air_evaluation_count = usize::MAX,
            5 => state.cp_air_assignment_write_count = usize::MAX,
            6 => state.witnessed_capacity_limit_cp_air_assignment_count = usize::MAX,
            _ => unreachable!(),
        }
        let unit = runtime.units.get(&system.id).expect("known unit");
        assert!(
            !super::super::release::
                next_capacity_limit_cp_air_assignment_transition_fits_for_test(
                    unit,
                    predecessor,
                )
        );
        assert_rejected_transactionally(&mut runtime, &system, predecessor);
    }
}

#[test]
fn capacity_guard_false_counter_overflows_are_preflighted_transactionally() {
    for private_counter in [false, true] {
        let (mut runtime, system, predecessor) =
            completed_cp337_case(-1_000.0, 1.0, false, 0.008);
        assert!(predecessor.active_guard_false_fallthrough);
        let state = &mut runtime
            .units
            .get_mut(&system.id)
            .expect("known unit")
            .calc_cooling_positive_supply_capacity_limit_cp_air_assignment;
        if private_counter {
            state.witnessed_capacity_limit_guard_false_fallthrough_skip_count = usize::MAX;
        } else {
            state.capacity_limit_guard_false_fallthrough_skip_count = usize::MAX;
        }
        let unit = runtime.units.get(&system.id).expect("known unit");
        assert!(
            !super::super::release::
                next_capacity_limit_cp_air_assignment_transition_fits_for_test(
                    unit,
                    predecessor,
                )
        );
        assert_rejected_transactionally(&mut runtime, &system, predecessor);
    }
}

#[test]
fn route_partition_product_corruption_and_post_commit_drift_are_detected() {
    let (mut runtime, system, predecessor) = active_case();
    {
        let state = &mut runtime
            .units
            .get_mut(&system.id)
            .expect("known unit")
            .calc_cooling_positive_supply_capacity_limit_cp_air_assignment;
        state.capacity_limit_cp_air_assignment_count = usize::MAX / 2 + 1;
        state.source_site_execution_count = 0;
    }
    let unit = runtime.units.get(&system.id).expect("known unit");
    assert!(
        !super::super::release::
            pending_capacity_limit_cp_air_assignment_state_is_consistent_for_test(
                unit,
                predecessor,
                None,
            )
    );
    assert_rejected_transactionally(&mut runtime, &system, predecessor);

    let (mut runtime, system, predecessor) = active_case();
    let snapshot =
        advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_cp_air_assignment(
            &mut runtime,
            &system,
            predecessor,
        )
        .expect("CP338");
    let mut public = runtime.clone();
    public
        .units
        .get_mut(&system.id)
        .expect("known unit")
        .calc_cooling_positive_supply_capacity_limit_cp_air_assignment
        .latest
        .as_mut()
        .expect("CP338 latest")
        .cp_air_assigned = false;
    let unit = public.units.get(&system.id).expect("known unit");
    assert!(
        !completed_direct_cooling_positive_supply_capacity_limit_cp_air_assignment_is_consistent(
            &public,
            unit,
            &system,
            snapshot,
            public
                .cooling_positive_supply_capacity_limit_cp_air_assignment_latest_witness(
                    system.id,
                ),
        )
    );

    let lifecycle =
        purchased_air_calc_cooling_positive_supply_capacity_limit_cp_air_assignment_lifecycle_summary(
            &runtime,
            system.id,
        )
        .expect("CP338 lifecycle");
    assert_eq!(lifecycle.state.latest, Some(snapshot));
}
