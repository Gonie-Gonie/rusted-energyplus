use super::super::*;
use crate::ideal_loads::calc::cooling_mixed_air_call::release_tests::{
    release_case, release_case_with_demand,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentError,
    PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentInput, PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_cooling_mixed_air_call,
    advance_direct_no_oa_calc_cooling_positive_supply_cp_air_assignment,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_positive_guard,
};
use crate::psychrometrics::energyplus_psy_cp_air_fn_w;

fn completed_cp330_case() -> (
    PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    crate::ideal_loads::PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,
    crate::heat_balance::state::ZoneHeatBalanceState,
) {
    let (mut runtime, system, predecessor, zone_state) = release_case();
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
    assert!(positive_guard.positive_supply_mass_flow_body_entered);
    (runtime, system, positive_guard, zone_state)
}

fn completed_cp330_case_with_demand(
    cooling_demand_w: f64,
) -> (
    PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    crate::ideal_loads::PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,
    crate::heat_balance::state::ZoneHeatBalanceState,
) {
    let (mut runtime, system, predecessor, zone_state) = release_case_with_demand(cooling_demand_w);
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
    (runtime, system, positive_guard, zone_state)
}

fn completed_cp330_unit_off_case() -> (
    PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    crate::ideal_loads::PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,
    crate::heat_balance::state::ZoneHeatBalanceState,
) {
    let (mut runtime, system, guard) =
        crate::ideal_loads::calc::cooling_economizer_condition_release_tests::
            release_fixture_with_cooling_demand_and_availability(-1_000.0, 0.0);
    let condition = crate::ideal_loads::advance_direct_no_oa_calc_cooling_economizer_condition(
        &mut runtime,
        &system,
        guard,
    )
    .expect("CP316");
    let body = crate::ideal_loads::advance_direct_no_oa_calc_cooling_economizer_body(
        &mut runtime,
        &system,
        condition,
    )
    .expect("CP317");
    let zone_state = crate::ideal_loads::calc::cooling_sensible_flow_release_tests::zone_state(
        body.controlled_zone,
    );
    let sensible = crate::ideal_loads::advance_direct_no_oa_calc_cooling_sensible_flow(
        &mut runtime,
        &system,
        body,
        &zone_state,
    )
    .expect("CP318");
    let dehumidification =
        crate::ideal_loads::advance_direct_no_oa_calc_cooling_dehumidification_flow(
            &mut runtime,
            &system,
            sensible,
        )
        .expect("CP319");
    let humidification = crate::ideal_loads::advance_direct_no_oa_calc_cooling_humidification_flow(
        &mut runtime,
        &system,
        dehumidification,
    )
    .expect("CP320");
    let reset = crate::ideal_loads::advance_direct_no_oa_calc_cooling_capacity_zero_flow_reset(
        &mut runtime,
        &system,
        humidification,
    )
    .expect("CP321");
    let maximum = crate::ideal_loads::advance_direct_no_oa_calc_cooling_supply_mass_flow_maximum(
        &mut runtime,
        &system,
        reset,
    )
    .expect("CP322");
    let ems_guard =
        crate::ideal_loads::advance_direct_no_oa_calc_cooling_supply_mass_flow_ems_override_guard(
            &mut runtime,
            &system,
            maximum,
        )
        .expect("CP323");
    let ems_body =
        crate::ideal_loads::advance_direct_no_oa_calc_cooling_supply_mass_flow_ems_override_body(
            &mut runtime,
            &system,
            ems_guard,
        )
        .expect("CP324");
    let limit_guard =
        crate::ideal_loads::advance_direct_no_oa_calc_cooling_supply_mass_flow_limit_guard(
            &mut runtime,
            &system,
            ems_body,
        )
        .expect("CP325");
    let limit_body =
        crate::ideal_loads::advance_direct_no_oa_calc_cooling_supply_mass_flow_limit_body(
            &mut runtime,
            &system,
            limit_guard,
        )
        .expect("CP326");
    let very_small_guard =
        crate::ideal_loads::advance_direct_no_oa_calc_cooling_supply_mass_flow_very_small_guard(
            &mut runtime,
            &system,
            limit_body,
        )
        .expect("CP327");
    let very_small_body =
        crate::ideal_loads::advance_direct_no_oa_calc_cooling_supply_mass_flow_very_small_guard_body(
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
    assert!(positive_guard.unit_off_skipped);
    (runtime, system, positive_guard, zone_state)
}

fn prospective_cp331_snapshot(
    predecessor: crate::ideal_loads::PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,
    zone_humidity_ratio: f64,
) -> PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentSnapshot {
    let mut state =
        PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentRuntimeState::new(predecessor.system);
    advance_cooling_positive_supply_cp_air_assignment_state(
        &mut state,
        predecessor,
        Some(
            PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentActiveInput {
                zone_humidity_ratio,
            },
        ),
    )
}

#[test]
fn public_release_commits_once_from_live_zone_humidity_and_rejects_replay() {
    let (mut runtime, system, predecessor, zone_state) = completed_cp330_case();
    let snapshot = advance_direct_no_oa_calc_cooling_positive_supply_cp_air_assignment(
        &mut runtime,
        &system,
        predecessor,
        &zone_state,
    )
    .expect("CP331");
    let expected_cp = energyplus_psy_cp_air_fn_w(zone_state.air_humidity_ratio);

    assert!(cooling_positive_supply_cp_air_assignment_snapshot_is_exact_direct_release(snapshot));
    assert_eq!(
        snapshot.zone_humidity_ratio.map(f64::to_bits),
        Some(zone_state.air_humidity_ratio.to_bits())
    );
    assert_eq!(
        snapshot.cp_air_j_per_kg_k.map(f64::to_bits),
        Some(expected_cp.to_bits())
    );
    let state = &runtime
        .units
        .get(&system.id)
        .expect("known unit")
        .calc_cooling_positive_supply_cp_air_assignment;
    assert_eq!(state.transition_count, 1);
    assert_eq!(state.cp_air_assignment_count, 1);
    assert_eq!(state.source_site_execution_count, 3);

    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_positive_supply_cp_air_assignment(
            &mut runtime,
            &system,
            predecessor,
            &zone_state,
        )
        .is_err()
    );
    assert_eq!(runtime, before);
}

#[test]
fn private_latest_or_witness_corruption_is_fail_closed_and_transactional() {
    #[derive(Clone, Copy)]
    enum Corruption {
        LatestWithoutWitness,
        WitnessWithoutLatest,
        RetainedRoute,
        RetainedOrdinal,
        WitnessedAssignmentCounter,
    }

    let (runtime, system, predecessor, zone_state) = completed_cp330_case();
    let prospective = prospective_cp331_snapshot(predecessor, zone_state.air_humidity_ratio);
    for corruption in [
        Corruption::LatestWithoutWitness,
        Corruption::WitnessWithoutLatest,
        Corruption::RetainedRoute,
        Corruption::RetainedOrdinal,
        Corruption::WitnessedAssignmentCounter,
    ] {
        let mut case_runtime = runtime.clone();
        match corruption {
            Corruption::LatestWithoutWitness => {
                case_runtime
                    .units
                    .get_mut(&system.id)
                    .expect("known unit")
                    .calc_cooling_positive_supply_cp_air_assignment
                    .latest = Some(prospective);
            }
            Corruption::WitnessWithoutLatest => {
                case_runtime.set_cooling_positive_supply_cp_air_assignment_latest_witness(
                    system.id,
                    prospective,
                );
            }
            Corruption::RetainedRoute => {
                case_runtime
                    .units
                    .get_mut(&system.id)
                    .expect("known unit")
                    .calc_cooling_positive_supply_cp_air_assignment
                    .latest_route = Some(
                    PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentRetainedRoute::
                        CpAirAssigned,
                );
            }
            Corruption::RetainedOrdinal => {
                case_runtime
                    .units
                    .get_mut(&system.id)
                    .expect("known unit")
                    .calc_cooling_positive_supply_cp_air_assignment
                    .latest_transition_ordinal = Some(1);
            }
            Corruption::WitnessedAssignmentCounter => {
                case_runtime
                    .units
                    .get_mut(&system.id)
                    .expect("known unit")
                    .calc_cooling_positive_supply_cp_air_assignment
                    .witnessed_cp_air_assignment_count = 1;
            }
        }

        let before = case_runtime.clone();
        assert_eq!(
            advance_direct_no_oa_calc_cooling_positive_supply_cp_air_assignment(
                &mut case_runtime,
                &system,
                predecessor,
                &zone_state,
            ),
            Err(
                PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentError::
                    RuntimeStateInvariantViolation { system: system.id }
            )
        );
        assert_eq!(case_runtime, before);
    }
}

#[test]
fn recursive_cp329_chain_corruption_is_fail_closed_and_transactional() {
    let (mut runtime, system, predecessor, zone_state) = completed_cp330_case();
    let mut forged_cp329_witness = runtime
        .cooling_mixed_air_call_latest_witness(system.id)
        .expect("completed CP329 witness");
    forged_cp329_witness.source = "forged-cp329-private-witness";
    runtime.set_cooling_mixed_air_call_latest_witness(system.id, forged_cp329_witness);
    let before = runtime.clone();

    assert_eq!(
        advance_direct_no_oa_calc_cooling_positive_supply_cp_air_assignment(
            &mut runtime,
            &system,
            predecessor,
            &zone_state,
        ),
        Err(
            PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentError::
                RuntimeStateInvariantViolation { system: system.id }
        )
    );
    assert_eq!(runtime, before);
}

#[test]
fn public_skipped_routes_do_not_validate_zone_humidity() {
    enum SkippedRoute {
        UnitOff,
        NonCooling,
        GuardFalse,
    }

    for (route, poisoned_humidity, expected_unit_off, expected_non_cooling) in [
        (SkippedRoute::UnitOff, f64::NAN, true, false),
        (SkippedRoute::NonCooling, f64::NAN, false, true),
        (SkippedRoute::GuardFalse, f64::NEG_INFINITY, false, false),
    ] {
        let (mut runtime, system, predecessor, mut zone_state) = match route {
            SkippedRoute::UnitOff => completed_cp330_unit_off_case(),
            SkippedRoute::NonCooling => completed_cp330_case_with_demand(1.0),
            SkippedRoute::GuardFalse => completed_cp330_case_with_demand(-1.0e-40),
        };
        assert_eq!(predecessor.unit_off_skipped, expected_unit_off);
        assert_eq!(predecessor.non_cooling_skipped, expected_non_cooling);
        assert_eq!(
            predecessor.active_guard_false_fallthrough,
            !expected_unit_off && !expected_non_cooling
        );
        assert!(!predecessor.positive_supply_mass_flow_body_entered);
        zone_state.air_humidity_ratio = poisoned_humidity;

        let snapshot = advance_direct_no_oa_calc_cooling_positive_supply_cp_air_assignment(
            &mut runtime,
            &system,
            predecessor,
            &zone_state,
        )
        .expect("skipped CP331 release");
        assert!(
            cooling_positive_supply_cp_air_assignment_snapshot_is_exact_direct_release(snapshot)
        );
        assert!(!snapshot.zone_humidity_ratio_read);
        assert!(snapshot.zone_humidity_ratio.is_none());
        assert!(!snapshot.psychrometric_cp_air_evaluated);
        assert!(!snapshot.cp_air_assigned);
        assert_eq!(
            runtime
                .units
                .get(&system.id)
                .expect("known unit")
                .calc_cooling_positive_supply_cp_air_assignment
                .source_site_execution_count,
            0
        );
    }
}

#[test]
fn forged_cp330_snapshot_or_private_witness_fails_without_mutation() {
    let (runtime, system, predecessor, zone_state) = completed_cp330_case();
    let mut forged_source = predecessor;
    forged_source.source = "forged";
    let mut forged_supply = predecessor;
    forged_supply.supply_mass_flow_rate_kg_per_s = forged_supply
        .supply_mass_flow_rate_kg_per_s
        .map(|value| f64::from_bits(value.to_bits() + 1));

    for forged in [forged_source, forged_supply] {
        let mut case_runtime = runtime.clone();
        let before = case_runtime.clone();
        assert_eq!(
            advance_direct_no_oa_calc_cooling_positive_supply_cp_air_assignment(
                &mut case_runtime,
                &system,
                forged,
                &zone_state,
            ),
            Err(
                PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentError::
                    CoolingSupplyMassFlowPositiveGuardSnapshotMismatch { system: system.id }
            )
        );
        assert_eq!(case_runtime, before);
    }

    let mut case_runtime = runtime;
    let mut forged_witness = predecessor;
    forged_witness.source = "forged-private-witness";
    case_runtime
        .set_cooling_supply_mass_flow_positive_guard_latest_witness(system.id, forged_witness);
    let before = case_runtime.clone();
    assert_eq!(
        advance_direct_no_oa_calc_cooling_positive_supply_cp_air_assignment(
            &mut case_runtime,
            &system,
            predecessor,
            &zone_state,
        ),
        Err(
            PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentError::
                CoolingSupplyMassFlowPositiveGuardSnapshotMismatch { system: system.id }
        )
    );
    assert_eq!(case_runtime, before);
}

#[test]
fn positive_route_rejects_zone_humidity_drift_negative_and_nonfinite_values() {
    let (runtime, system, predecessor, zone_state) = completed_cp330_case();
    for (humidity_ratio, expected) in [
        (
            f64::from_bits(zone_state.air_humidity_ratio.to_bits() + 1),
            PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentError::
                CoolingMixedAirHumidityLineageMismatch { system: system.id },
        ),
        (
            -0.001,
            PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentError::InvalidActiveInput {
                system: system.id,
                input:
                    PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentInput::ZoneHumidityRatio,
            },
        ),
        (
            f64::INFINITY,
            PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentError::InvalidActiveInput {
                system: system.id,
                input:
                    PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentInput::ZoneHumidityRatio,
            },
        ),
    ] {
        let mut case_runtime = runtime.clone();
        let mut corrupted_zone = zone_state.clone();
        corrupted_zone.air_humidity_ratio = humidity_ratio;
        let before = case_runtime.clone();
        assert_eq!(
            advance_direct_no_oa_calc_cooling_positive_supply_cp_air_assignment(
                &mut case_runtime,
                &system,
                predecessor,
                &corrupted_zone,
            ),
            Err(expected)
        );
        assert_eq!(case_runtime, before);
    }
}

#[test]
fn wrong_zone_identity_fails_before_cp331_mutation() {
    let (mut runtime, system, predecessor, mut zone_state) = completed_cp330_case();
    zone_state.zone_id = ep_model::ZoneId(zone_state.zone_id.0 + 1);
    let before = runtime.clone();

    assert!(matches!(
        advance_direct_no_oa_calc_cooling_positive_supply_cp_air_assignment(
            &mut runtime,
            &system,
            predecessor,
            &zone_state,
        ),
        Err(PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentError::ZoneIdentityMismatch { .. })
    ));
    assert_eq!(runtime, before);
}

#[test]
fn completed_cp330_counter_corruption_fails_before_cp331_mutation() {
    let (mut runtime, system, predecessor, zone_state) = completed_cp330_case();
    runtime
        .units
        .get_mut(&system.id)
        .expect("known unit")
        .calc_cooling_supply_mass_flow_positive_guard
        .source_site_execution_count += 1;
    let before = runtime.clone();

    assert_eq!(
        advance_direct_no_oa_calc_cooling_positive_supply_cp_air_assignment(
            &mut runtime,
            &system,
            predecessor,
            &zone_state,
        ),
        Err(
            PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentError::
                RuntimeStateInvariantViolation { system: system.id }
        )
    );
    assert_eq!(runtime, before);
}

#[test]
fn active_false_route_preflight_rejects_counter_overflow_transactionally() {
    for corrupt_public_counter in [true, false] {
        let (mut runtime, system, predecessor, zone_state) =
            completed_cp330_case_with_demand(-1.0e-40);
        assert!(predecessor.active_guard_false_fallthrough);
        let state = &mut runtime
            .units
            .get_mut(&system.id)
            .expect("known unit")
            .calc_cooling_positive_supply_cp_air_assignment;
        if corrupt_public_counter {
            state.positive_guard_false_fallthrough_skip_count = usize::MAX;
        } else {
            state.witnessed_positive_guard_false_fallthrough_skip_count = usize::MAX;
        }
        let unit = runtime.units.get(&system.id).expect("known unit");
        assert!(
            !super::super::release::next_cp_air_assignment_transition_fits_for_test(
                unit,
                predecessor,
            )
        );

        let before = runtime.clone();
        assert_eq!(
            advance_direct_no_oa_calc_cooling_positive_supply_cp_air_assignment(
                &mut runtime,
                &system,
                predecessor,
                &zone_state,
            ),
            Err(
                PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentError::
                    RuntimeStateInvariantViolation { system: system.id }
            )
        );
        assert_eq!(runtime, before);
    }
}

#[test]
fn assigned_route_preflight_rejects_each_counter_overflow_transactionally() {
    #[derive(Clone, Copy)]
    enum Counter {
        Transition,
        Assignment,
        SourceSite,
        HumidityRead,
        PsychrometricEvaluation,
        AssignmentWrite,
        WitnessedAssignment,
    }

    for counter in [
        Counter::Transition,
        Counter::Assignment,
        Counter::SourceSite,
        Counter::HumidityRead,
        Counter::PsychrometricEvaluation,
        Counter::AssignmentWrite,
        Counter::WitnessedAssignment,
    ] {
        let (mut runtime, system, predecessor, zone_state) = completed_cp330_case();
        let state = &mut runtime
            .units
            .get_mut(&system.id)
            .expect("known unit")
            .calc_cooling_positive_supply_cp_air_assignment;
        match counter {
            Counter::Transition => state.transition_count = usize::MAX,
            Counter::Assignment => state.cp_air_assignment_count = usize::MAX,
            Counter::SourceSite => state.source_site_execution_count = usize::MAX - 2,
            Counter::HumidityRead => state.zone_humidity_ratio_read_count = usize::MAX,
            Counter::PsychrometricEvaluation => {
                state.psychrometric_cp_air_evaluation_count = usize::MAX;
            }
            Counter::AssignmentWrite => state.cp_air_assignment_write_count = usize::MAX,
            Counter::WitnessedAssignment => state.witnessed_cp_air_assignment_count = usize::MAX,
        }
        let unit = runtime.units.get(&system.id).expect("known unit");
        assert!(
            !super::super::release::next_cp_air_assignment_transition_fits_for_test(
                unit,
                predecessor,
            )
        );

        let before = runtime.clone();
        assert_eq!(
            advance_direct_no_oa_calc_cooling_positive_supply_cp_air_assignment(
                &mut runtime,
                &system,
                predecessor,
                &zone_state,
            ),
            Err(
                PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentError::
                    RuntimeStateInvariantViolation { system: system.id }
            )
        );
        assert_eq!(runtime, before);
    }
}
