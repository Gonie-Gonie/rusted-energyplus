use super::super::*;
use crate::ideal_loads::calc::cooling_mixed_air_call::release_tests::release_case_with_demand_and_availability;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot,
    PurchasedAirRuntimeState, advance_direct_no_oa_calc_cooling_mixed_air_call,
    advance_direct_no_oa_calc_cooling_positive_supply_cp_air_assignment,
    advance_direct_no_oa_calc_cooling_positive_supply_temperature_assignment,
    advance_direct_no_oa_calc_cooling_positive_supply_temperature_minimum_limit,
    advance_direct_no_oa_calc_cooling_positive_supply_temperature_mixed_air_limit,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_positive_guard,
};

fn completed_cp334_case() -> (
    PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot,
) {
    completed_cp334_case_with_demand_availability_and_humidity(-1_000.0, 1.0, None)
}

fn completed_cp334_case_with_demand(
    cooling_demand_w: f64,
) -> (
    PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot,
) {
    completed_cp334_case_with_demand_availability_and_humidity(cooling_demand_w, 1.0, None)
}

fn completed_cp334_case_with_demand_availability_and_humidity(
    cooling_demand_w: f64,
    overall_availability: f64,
    humidity_ratio: Option<f64>,
) -> (
    PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot,
) {
    let (mut runtime, system, predecessor, mut zone_state) =
        release_case_with_demand_and_availability(cooling_demand_w, overall_availability);
    if let Some(humidity_ratio) = humidity_ratio {
        zone_state.air_humidity_ratio = humidity_ratio;
    }
    let mixed_air = advance_direct_no_oa_calc_cooling_mixed_air_call(
        &mut runtime,
        &system,
        predecessor,
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
    (runtime, system, mixed_air_limit)
}

fn assert_rejected_transactionally(
    runtime: &mut PurchasedAirRuntimeState,
    system: &ep_model::IdealLoadsAirSystem,
    predecessor: PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot,
) {
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment(
            runtime,
            system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(*runtime, before);
}

#[test]
fn public_release_copies_same_call_cp329_humidity_once_and_rejects_replay() {
    let (mut runtime, system, predecessor) = completed_cp334_case();
    assert!(predecessor.supply_temperature_mixed_air_limit_executed);
    let expected = runtime
        .units
        .get(&system.id)
        .and_then(|unit| unit.calc_cooling_mixed_air_call.latest)
        .and_then(|mixed_air| mixed_air.mixed_air_humidity_ratio)
        .expect("CP329 mixed-air humidity ratio");

    let snapshot =
        advance_direct_no_oa_calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment(
            &mut runtime,
            &system,
            predecessor,
        )
        .expect("CP335");
    assert!(
        cooling_positive_supply_humidity_ratio_mixed_air_assignment_snapshot_is_exact_direct_release(
            snapshot
        )
    );
    assert_eq!(
        snapshot.mixed_air_humidity_ratio.map(f64::to_bits),
        Some(expected.to_bits())
    );
    assert_eq!(
        snapshot.assigned_supply_humidity_ratio.map(f64::to_bits),
        Some(expected.to_bits())
    );
    let state = &runtime
        .units
        .get(&system.id)
        .expect("known unit")
        .calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment;
    assert_eq!(state.transition_count, 1);
    assert_eq!(state.supply_humidity_ratio_mixed_air_assignment_count, 1);
    assert_eq!(state.source_site_execution_count, 2);
    assert_eq!(state.mixed_air_humidity_ratio_read_count, 1);
    assert_eq!(state.supply_humidity_ratio_assignment_count, 1);

    assert_rejected_transactionally(&mut runtime, &system, predecessor);
}

#[test]
fn skipped_routes_never_project_cp329_humidity_into_cp335() {
    for (cooling_demand_w, guard_false, cp329_has_humidity) in
        [(1.0, false, false), (-1.0e-40, true, true)]
    {
        let (mut runtime, system, predecessor) = completed_cp334_case_with_demand(cooling_demand_w);
        assert!(!predecessor.supply_temperature_mixed_air_limit_executed);
        assert_eq!(
            runtime
                .units
                .get(&system.id)
                .and_then(|unit| unit.calc_cooling_mixed_air_call.latest)
                .and_then(|mixed_air| mixed_air.mixed_air_humidity_ratio)
                .is_some(),
            cp329_has_humidity
        );

        let snapshot =
            advance_direct_no_oa_calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment(
                &mut runtime,
                &system,
                predecessor,
            )
            .expect("skipped CP335");
        assert_eq!(
            snapshot.positive_guard_false_fallthrough_skipped,
            guard_false
        );
        assert!(!snapshot.mixed_air_humidity_ratio_read);
        assert!(snapshot.mixed_air_humidity_ratio.is_none());
        assert!(!snapshot.supply_humidity_ratio_assignment_performed);
        assert!(snapshot.assigned_supply_humidity_ratio.is_none());
        assert_eq!(
            runtime
                .units
                .get(&system.id)
                .expect("known unit")
                .calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment
                .source_site_execution_count,
            0
        );
    }
}

#[test]
fn active_operand_requires_exact_same_call_cp329_public_private_pair() {
    let (runtime, system, predecessor) = completed_cp334_case();
    let mixed_air = runtime
        .units
        .get(&system.id)
        .expect("known unit")
        .calc_cooling_mixed_air_call
        .latest
        .expect("CP329");
    let mixed_air_witness = runtime
        .cooling_mixed_air_call_latest_witness(system.id)
        .expect("private CP329");
    let humidity_ratio = mixed_air.mixed_air_humidity_ratio;
    assert!(
        super::super::release::active_operand_links_to_retained_prefix_for_test(
            predecessor,
            mixed_air,
            mixed_air_witness,
            humidity_ratio,
        )
    );

    let mut stale_call = mixed_air;
    stale_call.parent_call_ordinal += 1;
    assert!(
        !super::super::release::active_operand_links_to_retained_prefix_for_test(
            predecessor,
            stale_call,
            mixed_air_witness,
            humidity_ratio,
        )
    );
    let mut witness_drift = mixed_air_witness;
    witness_drift.mixed_air_humidity_ratio = witness_drift
        .mixed_air_humidity_ratio
        .map(|value| f64::from_bits(value.to_bits() + 1));
    assert!(
        !super::super::release::active_operand_links_to_retained_prefix_for_test(
            predecessor,
            mixed_air,
            witness_drift,
            humidity_ratio,
        )
    );
    assert!(
        !super::super::release::active_operand_links_to_retained_prefix_for_test(
            predecessor,
            mixed_air,
            mixed_air_witness,
            Some(-0.001),
        )
    );
}

#[test]
fn forged_cp334_argument_or_private_witness_fails_without_mutation() {
    let (runtime, system, predecessor) = completed_cp334_case();
    let mut forged_source = predecessor;
    forged_source.source = "forged";
    let mut forged_value = predecessor;
    forged_value.assigned_supply_temperature_c = forged_value
        .assigned_supply_temperature_c
        .map(|value| f64::from_bits(value.to_bits() + 1));

    for forged in [forged_source, forged_value] {
        let mut case_runtime = runtime.clone();
        assert_rejected_transactionally(&mut case_runtime, &system, forged);
    }

    let mut private_drift_runtime = runtime;
    let mut private_drift = predecessor;
    private_drift.minimum_supply_temperature_c = private_drift
        .minimum_supply_temperature_c
        .map(|value| f64::from_bits(value.to_bits() + 1));
    private_drift_runtime.set_cooling_positive_supply_temperature_mixed_air_limit_latest_witness(
        system.id,
        private_drift,
    );
    assert_rejected_transactionally(&mut private_drift_runtime, &system, predecessor);
}

#[test]
fn cp329_public_or_private_drift_fails_recursively_without_mutation() {
    let (runtime, system, predecessor) = completed_cp334_case();

    let mut public_drift_runtime = runtime.clone();
    let mixed_air = public_drift_runtime
        .units
        .get_mut(&system.id)
        .expect("known unit")
        .calc_cooling_mixed_air_call
        .latest
        .as_mut()
        .expect("CP329");
    mixed_air.mixed_air_humidity_ratio = mixed_air
        .mixed_air_humidity_ratio
        .map(|value| f64::from_bits(value.to_bits() + 1));
    assert_rejected_transactionally(&mut public_drift_runtime, &system, predecessor);

    let mut private_drift_runtime = runtime;
    let mut private_drift = private_drift_runtime
        .cooling_mixed_air_call_latest_witness(system.id)
        .expect("private CP329");
    private_drift.mixed_air_humidity_ratio = private_drift
        .mixed_air_humidity_ratio
        .map(|value| f64::from_bits(value.to_bits() + 1));
    private_drift_runtime.set_cooling_mixed_air_call_latest_witness(system.id, private_drift);
    assert_rejected_transactionally(&mut private_drift_runtime, &system, predecessor);
}

#[test]
fn active_counter_overflow_preflight_is_checked_and_transactional() {
    for counter in 0..6 {
        let (mut runtime, system, predecessor) = completed_cp334_case();
        let unit = runtime.units.get_mut(&system.id).expect("known unit");
        let state = &mut unit.calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment;
        match counter {
            0 => state.transition_count = usize::MAX,
            1 => state.supply_humidity_ratio_mixed_air_assignment_count = usize::MAX,
            2 => state.source_site_execution_count = usize::MAX,
            3 => state.mixed_air_humidity_ratio_read_count = usize::MAX,
            4 => state.supply_humidity_ratio_assignment_count = usize::MAX,
            5 => state.witnessed_supply_humidity_ratio_mixed_air_assignment_count = usize::MAX,
            _ => unreachable!(),
        }
        assert!(
            !super::super::release::
                next_supply_humidity_ratio_mixed_air_assignment_transition_fits_for_test(
                    unit,
                    predecessor,
                )
        );
        assert_rejected_transactionally(&mut runtime, &system, predecessor);
    }
}

#[test]
fn active_source_site_product_overflow_fails_pending_validation_transactionally() {
    let (mut runtime, system, predecessor) = completed_cp334_case();
    let unit = runtime.units.get_mut(&system.id).expect("known unit");
    let state = &mut unit.calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment;
    state.supply_humidity_ratio_mixed_air_assignment_count = usize::MAX / 2 + 1;
    state.source_site_execution_count = 0;
    assert!(
        !super::super::release::
            pending_supply_humidity_ratio_mixed_air_assignment_state_is_consistent_for_test(
                unit,
                predecessor,
                None,
            )
    );
    assert_rejected_transactionally(&mut runtime, &system, predecessor);
}

#[test]
fn every_skipped_route_counter_overflow_fails_without_mutation() {
    for (cooling_demand_w, availability, counter) in [
        (1.0, 1.0, 0),
        (-1.0e-40, 1.0, 1),
        (-1.0e-40, 1.0, 2),
        (-1_000.0, 0.0, 3),
    ] {
        let (mut runtime, system, predecessor) =
            completed_cp334_case_with_demand_availability_and_humidity(
                cooling_demand_w,
                availability,
                None,
            );
        let unit = runtime.units.get_mut(&system.id).expect("known unit");
        let state = &mut unit.calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment;
        match counter {
            0 => state.non_cooling_skip_count = usize::MAX,
            1 => state.positive_guard_false_fallthrough_skip_count = usize::MAX,
            2 => state.witnessed_positive_guard_false_fallthrough_skip_count = usize::MAX,
            3 => state.unit_off_skip_count = usize::MAX,
            _ => unreachable!(),
        }
        assert!(
            !super::super::release::
                next_supply_humidity_ratio_mixed_air_assignment_transition_fits_for_test(
                    unit,
                    predecessor,
                )
        );
        assert_rejected_transactionally(&mut runtime, &system, predecessor);
    }
}

#[test]
fn orphan_public_or_private_cp335_latest_fails_without_mutation() {
    let (runtime, system, predecessor) = completed_cp334_case();
    let mut isolated_state =
        PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentRuntimeState::new(
            system.id,
        );
    let orphan = advance_cooling_positive_supply_humidity_ratio_mixed_air_assignment_state(
        &mut isolated_state,
        predecessor,
        Some(
            PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentActiveInput {
                mixed_air_humidity_ratio: 0.008,
            },
        ),
    );

    let mut public_orphan_runtime = runtime.clone();
    public_orphan_runtime
        .units
        .get_mut(&system.id)
        .expect("known unit")
        .calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment
        .latest = Some(orphan);
    assert_rejected_transactionally(&mut public_orphan_runtime, &system, predecessor);

    let mut private_orphan_runtime = runtime;
    private_orphan_runtime
        .set_cooling_positive_supply_humidity_ratio_mixed_air_assignment_latest_witness(
            system.id, orphan,
        );
    assert_rejected_transactionally(&mut private_orphan_runtime, &system, predecessor);
}

#[test]
fn recursive_cp334_state_corruption_fails_closed_without_mutation() {
    let (mut runtime, system, predecessor) = completed_cp334_case();
    runtime
        .units
        .get_mut(&system.id)
        .expect("known unit")
        .calc_cooling_positive_supply_temperature_mixed_air_limit
        .source_site_execution_count += 1;
    assert_rejected_transactionally(&mut runtime, &system, predecessor);
}
